use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::stark_proof::stark_verify::compute_root_recursive::ComputeRootRecursive;
use crate::swiftness::commitment::vector::config::ConfigTrait;
use crate::swiftness::commitment::vector::types::{
    Commitment as VectorCommitment, CommitmentTrait, Query, QueryWithDepth,
    Witness as VectorWitness,
};
// Main VectorDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorDecommitStep {
    VectorCommitmentDecommit,
    VerifyCommitmentHash,
    Done,
}
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VectorDecommit {
    step: VectorDecommitStep,
    reference_commitment_hash: Felt,
}

impl_type_identifiable!(VectorDecommit);

impl VectorDecommit {
    pub fn new() -> Self {
        Self {
            step: VectorDecommitStep::VectorCommitmentDecommit,
            reference_commitment_hash: Felt::ZERO,
        }
    }
}

impl Default for VectorDecommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for VectorDecommit {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VectorDecommitStep::VectorCommitmentDecommit => {
                // Read vector commitment using trait method
                let vector_commitment = VectorCommitment::from_stack(stack);

                self.reference_commitment_hash = vector_commitment.commitment_hash;
                let height = vector_commitment.config.height;

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let mut queries = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());
                for _ in 0..queries_len.to_biguint().try_into().unwrap() {
                    queries.push(Query::from_stack(stack));
                }

                // Read authentications using trait method
                let witness = VectorWitness::from_stack(stack);

                // Push vector config using trait method
                vector_commitment.config.push_to_stack(stack);

                let shift = Felt::TWO.pow_felt(&height);
                let mut shifted_queries = Vec::with_capacity(queries.len());

                for query in &queries {
                    shifted_queries
                        .push(QueryWithDepth::from_query_with_shift(query, height, shift));
                }

                // Push authentications using trait method
                witness.push_to_stack(stack);

                let auth_start = Felt::ZERO;
                let start = Felt::ZERO;
                stack.push_front(&auth_start.to_bytes_be()).unwrap();
                stack.push_front(&start.to_bytes_be()).unwrap();

                // Use QueryWithDepth trait method to push queries
                QueryWithDepth::push_to_stack(&shifted_queries, stack);

                let computed_hash = Felt::ZERO;
                stack.push_front(&computed_hash.to_bytes_be()).unwrap();

                self.step = VectorDecommitStep::VerifyCommitmentHash;
                vec![ComputeRootRecursive::new().to_vec_with_type_tag()]
            }

            VectorDecommitStep::VerifyCommitmentHash => {
                let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                assert!(
                    commitment_hash == self.reference_commitment_hash,
                    "Commitment hash verification failed"
                );
                self.step = VectorDecommitStep::Done;
                vec![]
            }

            VectorDecommitStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VectorDecommitStep::Done
    }
}
