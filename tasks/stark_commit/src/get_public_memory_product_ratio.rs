use felt::Felt;
use types::swiftness::stark::types::StarkProof;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct GetPublicMemoryProductRatio {
    step: GetPublicMemoryProductRatioStep,
    z: Felt,
    alpha: Felt,
    public_memory_column_size: Felt,
    current_page_index: usize,
    pages_product: Felt,
    total_length: Felt,
    result: Felt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GetPublicMemoryProductRatioStep {
    Initialize,
    ProcessMainPage,
    ProcessMainPageBatch,
    ProcessContinuousPages,
    ComputeFinalRatio,
    Done,
}

impl_type_identifiable!(GetPublicMemoryProductRatio);

impl GetPublicMemoryProductRatio {
    pub fn new() -> Self {
        Self {
            step: GetPublicMemoryProductRatioStep::Initialize,
            z: Felt::ZERO,
            alpha: Felt::ZERO,
            public_memory_column_size: Felt::ZERO,
            current_page_index: 0,
            pages_product: Felt::ONE,
            total_length: Felt::ZERO,
            result: Felt::ZERO,
        }
    }
}

impl Default for GetPublicMemoryProductRatio {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for GetPublicMemoryProductRatio {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GetPublicMemoryProductRatioStep::Initialize => {
                self.z = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.alpha = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.public_memory_column_size = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.current_page_index = 0;
                self.pages_product = Felt::ONE;
                self.total_length = Felt::ZERO;

                self.step = GetPublicMemoryProductRatioStep::ProcessMainPage;
                vec![]
            }

            GetPublicMemoryProductRatioStep::ProcessMainPage => {
                // Initialize batching for main page
                self.current_page_index = 0;
                self.pages_product = Felt::ONE;
                self.total_length = Felt::ZERO;

                self.step = GetPublicMemoryProductRatioStep::ProcessMainPageBatch;
                vec![]
            }

            GetPublicMemoryProductRatioStep::ProcessMainPageBatch => {
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;
                let main_page = &public_input.main_page.0;

                const BATCH_SIZE: usize = 10; // Process 10 elements at a time
                let start_idx = self.current_page_index;
                let end_idx = (start_idx + BATCH_SIZE).min(main_page.len());

                // Process current batch
                for i in start_idx..end_idx {
                    if let Some(current) = main_page.get(i) {
                        let product = self.z - (current.address + self.alpha * current.value);
                        self.pages_product *= product;
                        self.total_length += Felt::ONE;
                    }
                }

                self.current_page_index = end_idx;

                // Check if all main page elements processed
                if self.current_page_index >= main_page.len() {
                    self.step = GetPublicMemoryProductRatioStep::ProcessContinuousPages;
                }

                vec![]
            }

            GetPublicMemoryProductRatioStep::ProcessContinuousPages => {
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;

                const BATCH_SIZE: usize = 10;
                let continuous_pages = &public_input.continuous_page_headers;
                let start_idx = self.current_page_index;
                let end_idx = (start_idx + BATCH_SIZE).min(continuous_pages.len());

                for i in start_idx..end_idx {
                    if let Some(header) = continuous_pages.get(i) {
                        self.pages_product *= header.prod;
                        self.total_length += header.size;
                    }
                }

                self.current_page_index = end_idx;

                if self.current_page_index >= continuous_pages.len() {
                    self.step = GetPublicMemoryProductRatioStep::ComputeFinalRatio;
                }

                vec![]
            }

            GetPublicMemoryProductRatioStep::ComputeFinalRatio => {
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;

                let numerator = self.z.pow_felt(&self.public_memory_column_size);
                let padded =
                    self.z - (public_input.padding_addr + self.alpha * public_input.padding_value);

                assert!(self.total_length <= self.public_memory_column_size);
                let denominator_pad =
                    padded.pow_felt(&(self.public_memory_column_size - self.total_length));

                self.result = numerator
                    .field_div(&felt::NonZeroFelt::from_felt_unchecked(self.pages_product))
                    .field_div(&felt::NonZeroFelt::from_felt_unchecked(denominator_pad));

                stack.push_front(&self.result.to_bytes_be()).unwrap();

                self.step = GetPublicMemoryProductRatioStep::Done;
                vec![]
            }

            GetPublicMemoryProductRatioStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == GetPublicMemoryProductRatioStep::Done
    }
}
