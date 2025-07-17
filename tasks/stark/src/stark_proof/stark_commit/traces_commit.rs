use crate::poseidon::PoseidonHash;
use crate::stark_proof::PoseidonHashMany;
use crate::{felt::Felt, swiftness::stark::types::StarkProof};
use lambdaworks_math::traits::ByteConversion;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracesCommitStep {
    ReadOriginalCommitment,
    GenerateInteractionElements,
    ReadInteractionCommitment,
    Done,
}

#[repr(C)]
pub struct TracesCommit {
    step: TracesCommitStep,
    interaction_elements_count: u32,
    current_element: u32,
}

impl_type_identifiable!(TracesCommit);

impl TracesCommit {
    pub fn new() -> Self {
        Self {
            step: TracesCommitStep::ReadOriginalCommitment,
            interaction_elements_count: 6, // Layout has 6 interaction elements
            current_element: 0,
        }
    }
}

impl Executable for TracesCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            TracesCommitStep::ReadOriginalCommitment => {
                let proof: &StarkProof = stack.get_proof_reference();

                // Get original commitment from unsent_commitment
                let original_commitment = proof.unsent_commitment.traces.original;

                // Push to stack for vector_commit processing
                stack
                    .push_front(&original_commitment.to_bytes_be())
                    .unwrap();

                self.step = TracesCommitStep::GenerateInteractionElements;

                // Call VectorCommit to process the commitment
                vec![VectorCommit::new().to_vec_with_type_tag()]
            }

            TracesCommitStep::GenerateInteractionElements => {
                // At this point, original commitment is processed
                // Generate interaction elements using transcript

                self.step = TracesCommitStep::ReadInteractionCommitment;

                // Generate interaction elements one by one
                vec![
                    GenerateInteractionElements::new(self.interaction_elements_count)
                        .to_vec_with_type_tag(),
                ]
            }

            TracesCommitStep::ReadInteractionCommitment => {
                let proof: &StarkProof = stack.get_proof_reference();

                // Get interaction commitment
                let interaction_commitment = proof.unsent_commitment.traces.interaction;

                // Push to stack for vector_commit processing
                stack
                    .push_front(&interaction_commitment.to_bytes_be())
                    .unwrap();

                self.step = TracesCommitStep::Done;

                // Call VectorCommit again for interaction commitment
                vec![VectorCommit::new().to_vec_with_type_tag()]
            }

            TracesCommitStep::Done => {
                // All trace commitments are now on the stack
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == TracesCommitStep::Done
    }
}

// Task for generating interaction elements
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GenerateInteractionElements {
    total_elements: u32,
    current_element: u32,
    step: GenerateInteractionStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateInteractionStep {
    GenerateHash,
    ReadResult,
}

impl_type_identifiable!(GenerateInteractionElements);

impl GenerateInteractionElements {
    pub fn new(total_elements: u32) -> Self {
        Self {
            total_elements,
            current_element: 0,
            step: GenerateInteractionStep::GenerateHash,
        }
    }
}

impl Executable for GenerateInteractionElements {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GenerateInteractionStep::GenerateHash => {
                if self.current_element >= self.total_elements {
                    return vec![];
                }

                // Get transcript digest and counter from stack
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Store transcript state for later restoration
                stack.push_front(&transcript_digest.to_bytes_be()).unwrap();
                stack.push_front(&transcript_counter.to_bytes_be()).unwrap();

                // Push inputs for PoseidonHash
                stack.push_front(&transcript_counter.to_bytes_be()).unwrap();
                stack.push_front(&transcript_digest.to_bytes_be()).unwrap();

                self.step = GenerateInteractionStep::ReadResult;

                // Call PoseidonHash to generate random element
                vec![PoseidonHash::new(transcript_digest, transcript_counter).to_vec_with_type_tag()]
            }

            GenerateInteractionStep::ReadResult => {
                // PoseidonHash puts 3 values on stack: [hash_result, input2, input1]
                let hash_result = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // Pop the two input values
                stack.pop_front(); // counter
                stack.pop_front(); // digest

                // Get transcript state that we stored
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push generated element to result stack
                stack.push_front(&hash_result.to_bytes_be()).unwrap();

                // Update transcript counter for next iteration
                let new_counter = transcript_counter + Felt::ONE;

                self.current_element += 1;

                if self.current_element < self.total_elements {
                    // More elements to generate - push transcript state for next iteration
                    stack.push_front(&new_counter.to_bytes_be()).unwrap();
                    stack.push_front(&transcript_digest.to_bytes_be()).unwrap();
                    self.step = GenerateInteractionStep::GenerateHash;
                } else {
                    // All elements generated - push final transcript state
                    stack.push_front(&new_counter.to_bytes_be()).unwrap();
                    stack.push_front(&transcript_digest.to_bytes_be()).unwrap();
                }

                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.current_element >= self.total_elements
    }
}

// VectorCommit task
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VectorCommit {
    committed: bool,
    phase: VectorCommitPhase,
}

impl_type_identifiable!(VectorCommit);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorCommitPhase {
    CallPoseidonHashMany,
    RestoreTranscriptState,
    Done,
}

impl VectorCommit {
    pub fn new() -> Self {
        Self {
            committed: false,
            phase: VectorCommitPhase::CallPoseidonHashMany,
        }
    }
}

impl Executable for VectorCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            VectorCommitPhase::CallPoseidonHashMany => {
                let commitment = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript digest: hash(digest + 1, commitment)
                let new_digest_input = transcript_digest + Felt::ONE;

                PoseidonHashMany::push_input(&[new_digest_input, commitment], stack);

                self.phase = VectorCommitPhase::RestoreTranscriptState;

                vec![PoseidonHashMany::new(2).to_vec_with_type_tag()]
            }
            VectorCommitPhase::RestoreTranscriptState => {
                let new_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                let transcript_counter = Felt::ZERO;
                stack.pop_front();

                stack.push_front(&new_digest.to_bytes_be()).unwrap();
                stack.push_front(&transcript_counter.to_bytes_be()).unwrap();
                self.phase = VectorCommitPhase::Done;
                vec![]
            }
            VectorCommitPhase::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == VectorCommitPhase::Done
    }
}
