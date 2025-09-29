use felt::Felt;
use types::swiftness::commitment::vector::config::Config;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::compute_root_recursive::ComputeRootRecursive;
use types::funvec::{FUNVEC_AUTHENTICATIONS, FUNVEC_QUERIES};
use types::swiftness::commitment::vector::types::Commitment;
use types::swiftness::stark::types::{cast_slice_to_struct, VerifyVariables};

// Main VectorDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorDecommitStep {
    ReadCommitmentAndConfig, // read commitment + queries count
    ProcessQueriesBatch,     // read queries in batches
    InitProcessWitness,      // read n_authentications
    ProcessWitnessBatch,     // read authentications in batches
    PrepareComputeRoot,      // prepare data for ComputeRootRecursive
    VerifyCommitmentHash,
    Done,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VectorDecommit {
    step: VectorDecommitStep,
    reference_commitment_hash: Felt,
    n_authentications: usize,
    queries_count: usize,
    current_query_index: usize,
    current_auth_index: usize,
    vector_commitment: Commitment,
}

impl_type_identifiable!(VectorDecommit);

impl VectorDecommit {
    pub fn new() -> Self {
        Self {
            step: VectorDecommitStep::ReadCommitmentAndConfig,
            reference_commitment_hash: Felt::ZERO,
            n_authentications: 0,
            queries_count: 0,
            current_query_index: 0,
            current_auth_index: 0,
            vector_commitment: Commitment::default(),
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
            VectorDecommitStep::ReadCommitmentAndConfig => {
                // Read vector commitment
                self.vector_commitment = commitment_from_stack(stack);
                self.reference_commitment_hash = self.vector_commitment.commitment_hash;

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.queries_count = queries_len.to_biguint().try_into().unwrap();
                assert!(
                    self.queries_count <= FUNVEC_QUERIES,
                    "Too many queries: {} > {}",
                    self.queries_count,
                    FUNVEC_QUERIES
                );

                self.current_query_index = 0;
                self.step = VectorDecommitStep::ProcessQueriesBatch;
                vec![]
            }

            VectorDecommitStep::ProcessQueriesBatch => {
                const BATCH_SIZE: usize = 50; // Process max 50 queries per transaction

                let batch_end =
                    std::cmp::min(self.current_query_index + BATCH_SIZE, self.queries_count);

                // Process batch of queries
                for i in self.current_query_index..batch_end {
                    let index = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();

                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.temp_queries;
                    queries_slice[i * 2] = index;
                    queries_slice[i * 2 + 1] = value;
                }

                self.current_query_index = batch_end;

                // Check if done with all queries
                if self.current_query_index >= self.queries_count {
                    self.step = VectorDecommitStep::InitProcessWitness;
                } else {
                    // Continue with next batch
                    self.step = VectorDecommitStep::ProcessQueriesBatch;
                }

                vec![]
            }

            VectorDecommitStep::InitProcessWitness => {
                // Read witness authentications count
                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.n_authentications = n_authentications.try_into().unwrap();
                assert!(
                    self.n_authentications <= FUNVEC_AUTHENTICATIONS,
                    "Too many authentications: {} > {}",
                    self.n_authentications,
                    FUNVEC_AUTHENTICATIONS
                );

                println!(
                    "DEBUG VectorWitness::from_stack: n_auth_usize = {}",
                    self.n_authentications
                );

                self.current_auth_index = 0;
                self.step = VectorDecommitStep::ProcessWitnessBatch;
                vec![]
            }

            VectorDecommitStep::ProcessWitnessBatch => {
                const BATCH_SIZE: usize = 50; // Process max 50 authentications per transaction

                let batch_end =
                    std::cmp::min(self.current_auth_index + BATCH_SIZE, self.n_authentications);

                // Process batch of authentications
                for i in self.current_auth_index..batch_end {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();

                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    verify_variables.authentications[i] = auth;
                }

                self.current_auth_index = batch_end;

                // Check if done with all authentications
                if self.current_auth_index >= self.n_authentications {
                    self.step = VectorDecommitStep::PrepareComputeRoot;
                } else {
                    // Continue with next batch
                    self.step = VectorDecommitStep::ProcessWitnessBatch;
                }

                vec![]
            }

            VectorDecommitStep::PrepareComputeRoot => {
                let height = self.vector_commitment.config.height;

                // Push vector config
                push_to_stack(&self.vector_commitment.config, stack);

                let shift = Felt::TWO.pow_felt(&height);

                // Convert from temp_queries (index, value pairs) to queries (QueryWithDepth format)
                {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();

                    // Convert to QueryWithDepth format (index + shift, value, height)
                    for i in 0..self.queries_count {
                        let index = verify_variables.temp_queries[i * 2];
                        let value = verify_variables.temp_queries[i * 2 + 1];
                        verify_variables.queries[i * 3] = index + shift;
                        verify_variables.queries[i * 3 + 1] = value;
                        verify_variables.queries[i * 3 + 2] = height;
                    }
                }

                stack
                    .push_front(&Felt::from(self.n_authentications).to_bytes_be())
                    .unwrap();

                let auth_start = Felt::ZERO;
                let start = Felt::ZERO;

                stack.push_front(&auth_start.to_bytes_be()).unwrap();
                stack.push_front(&start.to_bytes_be()).unwrap();
                stack
                    .push_front(&Felt::from(self.queries_count).to_bytes_be())
                    .unwrap();

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
                println!("DEBUG: VectorDecommitStep::VerifyCommitmentHash done");
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

#[inline(always)]
fn commitment_from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) -> Commitment {
    let data = stack.borrow_front();
    let commitment_ref = cast_slice_to_struct::<Commitment>(data);
    let commitment = *commitment_ref; // Copy only when needed
    stack.pop_front();
    commitment
}

#[inline(always)]
fn push_to_stack<T: BidirectionalStack>(config: &Config, stack: &mut T) {
    stack
        .push_front(&config.n_verifier_friendly_commitment_layers.to_bytes_be())
        .unwrap();
    stack.push_front(&config.height.to_bytes_be()).unwrap();
}
