use felt::Felt;
use types::swiftness::commitment::vector::config::Config;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::stark_verify::compute_root_recursive::ComputeRootRecursive;
use types::funvec::{FUNVEC_AUTHENTICATIONS, FUNVEC_QUERIES};
use types::swiftness::commitment::vector::types::Commitment;
use types::swiftness::stark::types::{cast_slice_to_struct, VerifyVariables};
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
    n_authentications: usize,
}

impl_type_identifiable!(VectorDecommit);

impl VectorDecommit {
    pub fn new() -> Self {
        Self {
            step: VectorDecommitStep::VectorCommitmentDecommit,
            reference_commitment_hash: Felt::ZERO,
            n_authentications: 0,
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
                let vector_commitment = commitment_from_stack(stack);
                println!("DEBUG: vector_commitment: {:?}", vector_commitment);

                self.reference_commitment_hash = vector_commitment.commitment_hash;
                let height = vector_commitment.config.height;

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                println!("DEBUG: queries_len: {:?}", queries_len);

                let queries_count: usize = queries_len.to_biguint().try_into().unwrap();
                println!("DEBUG: queries_count: {:?}", queries_count);
                assert!(
                    queries_count <= FUNVEC_QUERIES,
                    "Too many queries: {} > {}",
                    queries_count,
                    FUNVEC_QUERIES
                );

                // Read queries into pre-allocated array
                let mut count = queries_count;
                read_queries_from_stack(stack, &mut count);
                self.n_authentications = witness_from_stack(stack);

                // Push vector config using trait method
                push_to_stack(&vector_commitment.config, stack);

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
                let auth_bytes = {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let authentications_slice = &verify_variables.authentications;

                    let mut auth_bytes = Vec::new();
                    for i in (0..self.n_authentications).rev() {
                        auth_bytes.push(authentications_slice[i].to_bytes_be());
                    }

                    auth_bytes
                };

                for auth_bytes in auth_bytes {
                    stack.push_front(&auth_bytes).unwrap();
                }
                stack
                    .push_front(&Felt::from(self.n_authentications).to_bytes_be())
                    .unwrap();

                let auth_start = Felt::ZERO;
                let start = Felt::ZERO;
                stack.push_front(&auth_start.to_bytes_be()).unwrap();
                stack.push_front(&start.to_bytes_be()).unwrap();

                // Push queries with depth using trait method
                push_queries_with_depth_to_stack(queries_count, stack);

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

/// Read queries from stack and store them in a mutable slice (no allocation)
pub fn read_queries_from_stack<T: BidirectionalStack + StarkVerifyTrait>(
    stack: &mut T,
    count: &mut usize,
) {
    // Read queries directly into the slice
    for i in 0..*count {
        let index = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        let value = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
        let queries_slice = &mut verify_variables.temp_queries;
        queries_slice[i * 2] = index;
        queries_slice[i * 2 + 1] = value;
    }
}

/// Push queries with depth from a slice to stack (no allocation)
pub fn push_queries_with_depth_to_stack<T: BidirectionalStack + StarkVerifyTrait>(
    count: usize,
    stack: &mut T,
) {
    // Push queries in reverse order - no allocation
    for i in (0..count).rev() {
        let (depth_bytes, value_bytes, index_bytes) = {
            let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
            let queries_slice = &mut verify_variables.queries;
            let depth = queries_slice[i * 3 + 2];
            let value = queries_slice[i * 3 + 1];
            let index = queries_slice[i * 3];
            (
                depth.to_bytes_be(),
                value.to_bytes_be(),
                index.to_bytes_be(),
            )
        };

        stack.push_front(&depth_bytes).unwrap();
        stack.push_front(&value_bytes).unwrap();
        stack.push_front(&index_bytes).unwrap();
    }
    // Push length
    stack.push_front(&Felt::from(count).to_bytes_be()).unwrap();
}

fn commitment_from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) -> Commitment {
    let data = stack.borrow_front();
    let commitment_ref = cast_slice_to_struct::<Commitment>(data);
    let commitment = *commitment_ref; // Copy only when needed
    stack.pop_front();
    commitment
}

fn witness_from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) -> usize {
    let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    let n_auth_usize: usize = n_authentications.try_into().unwrap();
    assert!(
        n_auth_usize <= FUNVEC_AUTHENTICATIONS,
        "Too many authentications: {} > {}",
        n_auth_usize,
        FUNVEC_AUTHENTICATIONS
    );
    println!(
        "DEBUG VectorWitness::from_stack: n_auth_usize = {}",
        n_auth_usize
    );

    for i in 0..n_auth_usize {
        let auth = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
        verify_variables.authentications[i] = auth;
    }
    n_auth_usize
}

fn push_to_stack<T: BidirectionalStack>(config: &Config, stack: &mut T) {
    stack
        .push_front(&config.n_verifier_friendly_commitment_layers.to_bytes_be())
        .unwrap();
    stack.push_front(&config.height.to_bytes_be()).unwrap();
}
