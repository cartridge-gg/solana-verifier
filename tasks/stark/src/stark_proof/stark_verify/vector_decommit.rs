use felt::Felt;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::funvec::FUNVEC_QUERIES;
use crate::stark_proof::stark_verify::compute_root_recursive::ComputeRootRecursive;
use crate::swiftness::commitment::vector::config::ConfigTrait;
use crate::swiftness::commitment::vector::types::{
    Commitment as VectorCommitment, CommitmentTrait, Query, QueryWithDepth,
    Witness as VectorWitness,
};
use crate::swiftness::stark::types::VerifyVariables;
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
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            VectorDecommitStep::VectorCommitmentDecommit => {
                // Read vector commitment using trait method
                let vector_commitment = VectorCommitment::from_stack(stack);
                println!("DEBUG: vector_commitment: {:?}", vector_commitment);

                self.reference_commitment_hash = vector_commitment.commitment_hash;
                let height = vector_commitment.config.height;

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let queries_count: usize = queries_len.to_biguint().try_into().unwrap();
                assert!(
                    queries_count <= FUNVEC_QUERIES,
                    "Too many queries: {} > {}",
                    queries_count,
                    FUNVEC_QUERIES
                );

                // Read queries into pre-allocated array
                let mut count = queries_count;
                Query::read_queries_from_stack(stack, &mut count);

                // Read witness (authentications) into pre-allocated array
                println!(
                    "DEBUG: About to call VectorWitness::from_stack(), stack front: {:?}",
                    Felt::from_bytes_be_slice(stack.borrow_front())
                );
                VectorWitness::from_stack(stack);

                // Push vector config using trait method
                vector_commitment.config.push_to_stack(stack);

                let shift = Felt::TWO.pow_felt(&height);

                // Convert from temp_queries (index, value pairs) to queries (QueryWithDepth format)
                {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();

                    // Convert to QueryWithDepth format (index + shift, value, height)
                    for i in 0..queries_count {
                        let index = verify_variables.temp_queries[i * 2];
                        let value = verify_variables.temp_queries[i * 2 + 1];
                        verify_variables.queries[i * 3] = index + shift;
                        verify_variables.queries[i * 3 + 1] = value;
                        verify_variables.queries[i * 3 + 2] = height;
                    }
                }

                // Push authentications using trait method
                let (real_count, auth_bytes) = {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let authentications_slice = &verify_variables.authentications;
                    // Znajdź rzeczywistą liczbę authentications (nie-zero elementów)
                    let mut real_count = 0;
                    #[allow(clippy::needless_range_loop)]
                    for i in 0..authentications_slice.len() {
                        if authentications_slice[i] != Felt::ZERO {
                            real_count = i + 1;
                        }
                    }

                    println!("DEBUG vector_decommit: real_count = {}", real_count);

                    // Przygotuj bytes do pushowania
                    let mut auth_bytes = Vec::new();
                    for i in (0..real_count).rev() {
                        auth_bytes.push(authentications_slice[i].to_bytes_be());
                    }

                    (real_count, auth_bytes)
                };

                // Push authentications w odwrotnej kolejności
                for auth_bytes in auth_bytes {
                    stack.push_front(&auth_bytes).unwrap();
                }
                // Push liczbę authentications
                stack
                    .push_front(&Felt::from(real_count).to_bytes_be())
                    .unwrap();

                let auth_start = Felt::ZERO;
                let start = Felt::ZERO;
                stack.push_front(&auth_start.to_bytes_be()).unwrap();
                stack.push_front(&start.to_bytes_be()).unwrap();

                // Push queries with depth using trait method
                QueryWithDepth::push_queries_with_depth_to_stack(queries_count, stack);

                let computed_hash = Felt::ZERO;
                stack.push_front(&computed_hash.to_bytes_be()).unwrap();

                self.step = VectorDecommitStep::VerifyCommitmentHash;
                vec![ComputeRootRecursive::new().to_vec_with_type_tag()]
            }

            VectorDecommitStep::VerifyCommitmentHash => {
                let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                println!("commitment_hash: {:?}", commitment_hash);

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
