use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::stark_proof::stark_verify::compute_root_recursive::ComputeRootRecursive;
use crate::swiftness::commitment::vector::types::{Query, QueryWithDepth};
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
                self.reference_commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_verifier_friendly_layers = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let height = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let mut queries = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());
                for _ in 0..queries_len.to_biguint().try_into().unwrap() {
                    queries.push(Query::from_stack(stack));
                }

                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut authentications = Vec::new();

                let n_auth_usize: usize = n_authentications.try_into().unwrap();

                for _ in 0..n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    authentications.push(auth);
                }

                stack
                    .push_front(&n_verifier_friendly_layers.to_bytes_be())
                    .unwrap();

                let shift = Felt::TWO.pow_felt(&height);
                let mut shifted_queries = Vec::with_capacity(queries.len());

                for query in &queries {
                    shifted_queries
                        .push(QueryWithDepth::from_query_with_shift(query, height, shift));
                }

                for auth in authentications.iter().rev() {
                    stack.push_front(&auth.to_bytes_be()).unwrap();
                }
                stack.push_front(&n_authentications.to_bytes_be()).unwrap();

                let auth_start = Felt::ZERO;
                let start = Felt::ZERO;
                stack.push_front(&auth_start.to_bytes_be()).unwrap();
                stack.push_front(&start.to_bytes_be()).unwrap();

                for query in shifted_queries.iter().rev() {
                    stack.push_front(&query.depth.to_bytes_be()).unwrap();
                    stack.push_front(&query.value.to_bytes_be()).unwrap();
                    stack.push_front(&query.index.to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(shifted_queries.len()).to_bytes_be())
                    .unwrap();

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
