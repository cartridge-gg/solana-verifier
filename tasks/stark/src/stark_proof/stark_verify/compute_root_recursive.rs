use felt::{Felt, NonZeroFelt};
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::funvec::{FUNVEC_AUTHENTICATIONS, FUNVEC_QUERIES};
use crate::stark_proof::stark_verify::hash_computation::{
    HashComputation, HashComputationWithQueries,
};
use crate::swiftness::commitment::vector::config::{Config as VectorConfig, ConfigTrait};
use crate::swiftness::commitment::vector::types::QueryWithDepth;
use crate::swiftness::stark::types::VerifyVariables;

// ComputeRootRecursive task - handles one step of the recursive root computation
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeRootRecursive {
    step: ComputeRootRecursiveStep,
    pub parent: Felt,
    pub current: QueryWithDepth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeRootRecursiveStep {
    ProcessCurrent,
    ReadHash,
    Done,
}

impl_type_identifiable!(ComputeRootRecursive);

impl ComputeRootRecursive {
    pub fn new() -> Self {
        Self {
            step: ComputeRootRecursiveStep::ProcessCurrent,
            parent: Felt::ZERO,
            current: QueryWithDepth::default(),
        }
    }
}

impl Default for ComputeRootRecursive {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ComputeRootRecursive {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            ComputeRootRecursiveStep::ProcessCurrent => {
                let _computed_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_queries_usize: usize = n_queries.try_into().unwrap();
                println!("DEBUG: n_queries_usize = {}", n_queries_usize);
                assert!(
                    n_queries_usize <= FUNVEC_QUERIES,
                    "Too many queries: {} > {}",
                    n_queries_usize,
                    FUNVEC_QUERIES
                );
                // Read queries into pre-allocated array
                for i in 0..n_queries_usize {
                    let index = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let depth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();

                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.queries;
                    queries_slice[i * 3] = index;
                    queries_slice[i * 3 + 1] = value;
                    queries_slice[i * 3 + 2] = depth;
                }

                let start: usize = Felt::from_bytes_be_slice(stack.borrow_front())
                    .try_into()
                    .unwrap();
                stack.pop_front();

                let auth_start: usize = Felt::from_bytes_be_slice(stack.borrow_front())
                    .try_into()
                    .unwrap();
                stack.pop_front();

                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let n_auth_usize: usize = n_authentications.try_into().unwrap();
                assert!(
                    n_auth_usize <= FUNVEC_AUTHENTICATIONS,
                    "Too many authentications: {} > {}",
                    n_auth_usize,
                    FUNVEC_AUTHENTICATIONS
                );
                for i in 0..n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let authentications = &mut verify_variables.authentications;
                    authentications[i] = auth;
                }

                // Read vector config using trait method
                let vector_config = VectorConfig::from_stack(stack);
                let n_verifier_friendly_layers =
                    vector_config.n_verifier_friendly_commitment_layers;

                // Get current query from array
                let (current_index, current_value, current_depth) = {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.queries;
                    (
                        queries_slice[start * 3],
                        queries_slice[start * 3 + 1],
                        queries_slice[start * 3 + 2],
                    )
                };
                self.current = QueryWithDepth {
                    index: current_index,
                    value: current_value,
                    depth: current_depth,
                };
                // Check if we reached the root
                if self.current.index == Felt::ONE {
                    // We found the root - push it to stack and finish
                    stack.push_front(&self.current.value.to_bytes_be()).unwrap();
                    self.step = ComputeRootRecursiveStep::Done;
                    vec![]
                } else {
                    let (parent, bit) = self.current.index.div_rem(&NonZeroFelt::TWO);
                    let is_verifier_friendly = n_verifier_friendly_layers >= self.current.depth;
                    self.parent = parent;

                    if bit == Felt::ZERO {
                        if start + 1 < n_queries_usize {
                            let (next_index, next_value, _next_depth) = {
                                let verify_variables: &mut VerifyVariables =
                                    stack.get_verify_variables_mut();
                                let queries_slice = &mut verify_variables.queries;
                                (
                                    queries_slice[(start + 1) * 3],
                                    queries_slice[(start + 1) * 3 + 1],
                                    queries_slice[(start + 1) * 3 + 2],
                                )
                            };

                            if self.current.index + Felt::ONE == next_index {
                                // Push vector config using trait method
                                vector_config.push_to_stack(stack);

                                for i in (0..n_auth_usize).rev() {
                                    let auth = {
                                        let verify_variables: &mut VerifyVariables =
                                            stack.get_verify_variables_mut();
                                        let authentications = &mut verify_variables.authentications;
                                        authentications[i]
                                    };
                                    stack.push_front(&auth.to_bytes_be()).unwrap();
                                }
                                stack
                                    .push_front(&Felt::from(n_auth_usize).to_bytes_be())
                                    .unwrap();

                                stack
                                    .push_front(&Felt::from(auth_start).to_bytes_be())
                                    .unwrap();
                                stack
                                    .push_front(&Felt::from(start + 2).to_bytes_be())
                                    .unwrap();

                                // Push queries using trait method
                                QueryWithDepth::push_queries_with_depth_to_stack(
                                    n_queries_usize,
                                    stack,
                                );

                                self.step = ComputeRootRecursiveStep::ProcessCurrent;

                                return vec![HashComputationWithQueries::new(
                                    self.current.value,
                                    next_value,
                                    is_verifier_friendly,
                                    self.parent,
                                    self.current.depth - Felt::ONE,
                                )
                                .to_vec_with_type_tag()];
                            }
                        }

                        // Push vector config using trait method
                        vector_config.push_to_stack(stack);

                        for i in (0..n_auth_usize).rev() {
                            let auth = {
                                let verify_variables: &mut VerifyVariables =
                                    stack.get_verify_variables_mut();
                                let authentications = &mut verify_variables.authentications;
                                authentications[i]
                            };
                            stack.push_front(&auth.to_bytes_be()).unwrap();
                        }
                        stack
                            .push_front(&Felt::from(n_auth_usize).to_bytes_be())
                            .unwrap();

                        stack
                            .push_front(&Felt::from(auth_start + 1).to_bytes_be())
                            .unwrap();
                        stack
                            .push_front(&Felt::from(start + 1).to_bytes_be())
                            .unwrap();

                        // Push queries using trait method
                        QueryWithDepth::push_queries_with_depth_to_stack(n_queries_usize, stack);

                        self.step = ComputeRootRecursiveStep::ReadHash;
                        vec![HashComputation::new(
                            self.current.value,
                            {
                                let verify_variables: &mut VerifyVariables =
                                    stack.get_verify_variables_mut();
                                let authentications = &mut verify_variables.authentications;
                                authentications[auth_start]
                            },
                            is_verifier_friendly,
                        )
                        .to_vec_with_type_tag()]
                    } else {
                        // Push vector config using trait method
                        vector_config.push_to_stack(stack);

                        for i in (0..n_auth_usize).rev() {
                            let auth = {
                                let verify_variables: &mut VerifyVariables =
                                    stack.get_verify_variables_mut();
                                let authentications = &mut verify_variables.authentications;
                                authentications[i]
                            };
                            stack.push_front(&auth.to_bytes_be()).unwrap();
                        }
                        stack
                            .push_front(&Felt::from(n_auth_usize).to_bytes_be())
                            .unwrap();
                        stack
                            .push_front(&Felt::from(auth_start + 1).to_bytes_be())
                            .unwrap();
                        stack
                            .push_front(&Felt::from(start + 1).to_bytes_be())
                            .unwrap();

                        // Push queries using trait method
                        QueryWithDepth::push_queries_with_depth_to_stack(n_queries_usize, stack);

                        self.step = ComputeRootRecursiveStep::ReadHash;
                        // Create hash computation task
                        vec![HashComputation::new(
                            {
                                let verify_variables: &mut VerifyVariables =
                                    stack.get_verify_variables_mut();
                                let authentications = &mut verify_variables.authentications;
                                authentications[auth_start]
                            },
                            self.current.value,
                            is_verifier_friendly,
                        )
                        .to_vec_with_type_tag()]
                    }
                }
            }
            ComputeRootRecursiveStep::ReadHash => {
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Read queries into pre-allocated array
                QueryWithDepth::read_queries_with_depth_from_stack(stack);

                // Add new query to pre-allocated array
                let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                let queries_slice = &mut verify_variables.queries;

                // Find next available slot
                let mut next_slot = 0;
                let max_slots = queries_slice.len() / 3;
                while next_slot < max_slots && queries_slice[next_slot * 3] != Felt::ZERO {
                    next_slot += 1;
                }

                // Add new query with bounds checking
                assert!(
                    next_slot < max_slots,
                    "Queries array full: next_slot={}, max_slots={}",
                    next_slot,
                    max_slots
                );
                queries_slice[next_slot * 3] = self.parent;
                queries_slice[next_slot * 3 + 1] = hash;
                queries_slice[next_slot * 3 + 2] = self.current.depth - Felt::ONE;

                // Push queries using trait method - calculate actual count
                let actual_count = {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.queries;
                    let mut count = 0;
                    for i in 0..(queries_slice.len() / 3) {
                        if queries_slice[i * 3] != Felt::ZERO {
                            count = i + 1;
                        }
                    }
                    count
                };
                QueryWithDepth::push_queries_with_depth_to_stack(actual_count, stack);

                stack.push_front(&hash.to_bytes_be()).unwrap();

                self.step = ComputeRootRecursiveStep::ProcessCurrent;
                vec![]
            }

            ComputeRootRecursiveStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == ComputeRootRecursiveStep::Done
    }
}
