use std::fs;

use felt::NonZeroFelt;
use utils::global_values::InteractionElements;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::funvec::FunVec;
use crate::stark_proof::get_hash::GetHash;
use crate::stark_proof::stark_commit::StarkCommit;
use crate::stark_proof::stark_verify::StarkVerify;
use crate::stark_proof::validate_public_input::ValidatePublicInput;
use crate::stark_proof::VerifyPublicInput;
use crate::swiftness::air::domains::StarkDomains;
use crate::swiftness::stark::types::StarkProof;
use crate::swiftness::stark::types::{cast_struct_to_slice, FriVerifyData, StarkCommitment};
use crate::swiftness::transcript::TranscriptRandomFelt;
use felt::Felt;

const DIVISOR: Felt = Felt::from_hex_unchecked("0x100000000000000000000000000000000");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStep {
    ValidatePublicInput,
    GetHash,
    StarkCommit,
    GenerateQueries,
    GenerateQueriesLoop,
    StarkVerify,
    VerifyPublicInput,
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
    pub fn new() -> Self {
        Self {
            step: VerifyStep::ValidatePublicInput,
            samples: FunVec::default(),
            current_index: 0,
            total_queries: 0,
            query_upper_bound: Felt::ZERO,
            digest: Felt::ZERO,
            counter: Felt::ZERO,
        }
    }
}

impl Default for Verify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for Verify {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyStep::ValidatePublicInput => {
                self.step = VerifyStep::GetHash;
                vec![ValidatePublicInput::new().to_vec_with_type_tag()]
            }
            VerifyStep::GetHash => {
                let proof =
                    stack.get_proof_reference::<crate::swiftness::stark::types::StarkProof>();
                let n_verifier_friendly_commitment_layers =
                    proof.config.n_verifier_friendly_commitment_layers;
                assert!(
                    stack.is_empty_front(),
                    "Stack back should be empty before GetHash"
                );
                self.step = VerifyStep::StarkCommit;
                vec![GetHash::new(n_verifier_friendly_commitment_layers).to_vec_with_type_tag()]
            }
            VerifyStep::StarkCommit => {
                let result = stack.borrow_front().to_owned();
                stack.pop_front();

                assert!(
                    stack.is_empty_front(),
                    "Stack back should be empty after GetHash"
                );

                let proof =
                    stack.get_proof_reference::<crate::swiftness::stark::types::StarkProof>();

                let stark_domain = StarkDomains::new(
                    proof.config.log_trace_domain_size,
                    proof.config.log_n_cosets,
                );

                stack
                    .push_front(stark_domain.trace_generator.to_bytes_be().as_slice())
                    .unwrap();

                stack
                    .push_front(stark_domain.trace_domain_size.to_bytes_be().as_slice())
                    .unwrap();

                stack.push_front(&result).unwrap();

                self.step = VerifyStep::GenerateQueries;
                vec![StarkCommit::new().to_vec_with_type_tag()]
            }
            VerifyStep::GenerateQueries => {
                self.counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

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

                    let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                    fri_verify_data.queries = FunVec::from_vec(sorted_samples);

                    self.step = VerifyStep::StarkVerify;
                    vec![]
                }
            }
            VerifyStep::StarkVerify => {
                assert!(
                    stack.is_empty_front(),
                    "Stack should be empty before StarkVerify"
                );

                self.step = VerifyStep::VerifyPublicInput;
                vec![StarkVerify::new().to_vec_with_type_tag()]
            }
            VerifyStep::VerifyPublicInput => {
                assert!(
                    stack.is_empty_front(),
                    "Stack should be empty before verifying public input"
                );
                self.step = VerifyStep::Done;
                vec![VerifyPublicInput::new().to_vec_with_type_tag()]
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
