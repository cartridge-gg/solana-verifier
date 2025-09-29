use felt::{Felt, NonZeroFelt};
use stark_verify_decommitments::stark_verify::StarkVerify;
use transcript::transcript::TranscriptRandomFelt;
use types::{
    funvec::FunVec,
    swiftness::stark::types::{FriVerifyData, StarkProof},
};
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, Executable, ProofData,
    TypeIdentifiable,
};

const DIVISOR: Felt = Felt::from_hex_unchecked("0x100000000000000000000000000000000");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStep {
    GenerateQueries,
    GenerateQueriesLoop,
    StarkVerify,
    Done,
}

#[repr(C)]
pub struct Verify {
    step: VerifyStep,
    // Fields for query generation
    samples: FunVec<Felt, 20>,
    current_index: usize,
    total_queries: usize,
    query_upper_bound: Felt,
    digest: Felt,
    counter: Felt,
}

impl_type_identifiable!(Verify);

impl Verify {
    pub fn new(digest: Felt, counter: Felt) -> Self {
        Self {
            step: VerifyStep::GenerateQueries,
            samples: FunVec::default(),
            current_index: 0,
            total_queries: 0,
            query_upper_bound: Felt::ZERO,
            digest,
            counter,
        }
    }
}

impl Default for Verify {
    fn default() -> Self {
        Self::new(Felt::ZERO, Felt::ZERO)
    }
}

impl Executable for Verify {
    fn execute<T: BidirectionalStack + ProofData + CacheStorage>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            VerifyStep::GenerateQueries => {
                let proof = stack.get_proof_reference::<StarkProof>();
                self.total_queries = proof.config.n_queries.to_biguint().try_into().unwrap();

                let (log_trace_domain_size, log_n_cosets) = {
                    let proof: &StarkProof = stack.get_proof_reference();
                    (
                        proof.config.log_trace_domain_size,
                        proof.config.log_n_cosets,
                    )
                };
                let log_eval_domain_size = log_trace_domain_size + log_n_cosets;
                let eval_domain_size = Felt::TWO.pow_felt(&log_eval_domain_size);

                self.query_upper_bound = eval_domain_size;
                self.current_index = 0;
                self.step = VerifyStep::GenerateQueriesLoop;
                vec![TranscriptRandomFelt::new(self.digest, self.counter).to_vec_with_type_tag()]
            }
            VerifyStep::GenerateQueriesLoop => {
                // Get the random felt result from stack
                self.counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let random_felt = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Process the random felt to get a sample
                let (_, low) = random_felt.div_rem(&NonZeroFelt::from_felt_unchecked(DIVISOR));
                let (_, sample) =
                    low.div_rem(&NonZeroFelt::try_from(self.query_upper_bound).unwrap());
                self.samples.push(sample);
                self.current_index += 1;

                if self.current_index < self.total_queries {
                    // Generate next random felt - stay in the same step
                    vec![TranscriptRandomFelt::new(self.digest, self.counter).to_vec_with_type_tag()]
                } else {
                    // Sort the samples directly
                    let mut sorted_samples = self.samples.to_vec();
                    sorted_samples.sort();

                    // let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                    // fri_verify_data.queries = FunVec::from_vec(sorted_samples);
                    for sample in sorted_samples.iter().rev() {
                        stack.push_front(&sample.to_bytes_be()).unwrap();
                    }

                    stack
                        .push_front(&Felt::from(sorted_samples.len()).to_bytes_be())
                        .unwrap();

                    self.step = VerifyStep::StarkVerify;
                    vec![]
                }
            }
            VerifyStep::StarkVerify => {
                // assert!(
                //     stack.is_empty_front(),
                //     "Stack should be empty before StarkVerify"
                // );

                self.step = VerifyStep::Done;
                vec![StarkVerify::new().to_vec_with_type_tag()]
            }
            VerifyStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VerifyStep::Done
    }
}
