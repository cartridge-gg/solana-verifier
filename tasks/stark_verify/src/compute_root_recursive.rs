use felt::{Felt, NonZeroFelt};
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::hash_computation::{HashComputation, HashComputationWithQueries};
use types::funvec::{FUNVEC_AUTHENTICATIONS, FUNVEC_QUERIES};
use types::swiftness::commitment::vector::config::Config as VectorConfig;
use types::swiftness::commitment::vector::types::QueryWithDepth;
use types::swiftness::stark::types::VerifyVariables;

// ComputeRootRecursive task - handles one step of the recursive root computation
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeRootRecursive {
    step: ComputeRootRecursiveStep,
    n_auth_usize: usize,
    parent: Felt,
    current: QueryWithDepth,
    start: usize,
    auth_start: usize,
    n_queries_usize: usize,
    vector_config: VectorConfig,
    next_value: Felt,
    is_verifier_friendly: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeRootRecursiveStep {
    ProcessCurrent,
    Continue,
    HashComputation,
    HashComputationWithQueries,
    ReadHash,
    Done,
}

impl_type_identifiable!(ComputeRootRecursive);

impl ComputeRootRecursive {
    pub fn new() -> Self {
        Self {
            step: ComputeRootRecursiveStep::ProcessCurrent,
            n_auth_usize: 0,
            parent: Felt::ZERO,
            current: QueryWithDepth::default(),
            start: 0,
            auth_start: 0,
            n_queries_usize: 0,
            vector_config: VectorConfig::new(Felt::ZERO, Felt::ZERO),
            next_value: Felt::ZERO,
            is_verifier_friendly: false,
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
                self.n_queries_usize = n_queries.try_into().unwrap();
                assert!(
                    self.n_queries_usize <= FUNVEC_QUERIES,
                    "Too many queries: {} > {}",
                    self.n_queries_usize,
                    FUNVEC_QUERIES
                );
                // Clear the entire queries array first (new instance starts fresh)
                {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.queries;
                    let temp_queries_slice = &mut verify_variables.temp_queries;
                    queries_slice.fill(Felt::ZERO);
                    temp_queries_slice.fill(Felt::ZERO);
                }

                // Read queries into pre-allocated array
                for i in 0..self.n_queries_usize {
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

                self.start = Felt::from_bytes_be_slice(stack.borrow_front())
                    .try_into()
                    .unwrap();
                stack.pop_front();

                self.auth_start = Felt::from_bytes_be_slice(stack.borrow_front())
                    .try_into()
                    .unwrap();
                stack.pop_front();

                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.n_auth_usize = n_authentications.try_into().unwrap();
                assert!(
                    self.n_auth_usize <= FUNVEC_AUTHENTICATIONS,
                    "Too many authentications: {} > {}",
                    self.n_auth_usize,
                    FUNVEC_AUTHENTICATIONS
                );
                for i in 0..self.n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let authentications = &mut verify_variables.authentications;
                    authentications[i] = auth;
                }
                
                // Read vector config using trait method
                self.vector_config = from_stack(stack);


                // Get current query from array
                let (current_index, current_value, current_depth) = {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.queries;
                    (
                        queries_slice[self.start * 3],
                        queries_slice[self.start * 3 + 1],
                        queries_slice[self.start * 3 + 2],
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
                    self.step = ComputeRootRecursiveStep::Continue;
                    vec![]
                }
            }
            ComputeRootRecursiveStep::Continue => {
                let (parent, bit) = self.current.index.div_rem(&NonZeroFelt::TWO);
                self.is_verifier_friendly = self.vector_config.n_verifier_friendly_commitment_layers >= self.current.depth;
                self.parent = parent;

                if bit == Felt::ZERO {
                    if self.start + 1 < self.n_queries_usize {
                        let (next_index, next_value, _next_depth) = {
                            let verify_variables: &mut VerifyVariables =
                                stack.get_verify_variables_mut();
                            let queries_slice = &mut verify_variables.queries;
                            (
                                queries_slice[(self.start + 1) * 3],
                                queries_slice[(self.start + 1) * 3 + 1],
                                queries_slice[(self.start + 1) * 3 + 2],
                            )
                        };
                        if self.current.index + Felt::ONE == next_index {
                            self.next_value = next_value;
                            self.step = ComputeRootRecursiveStep::HashComputationWithQueries;
                            return vec![];
                        }
                    }

                    // Push vector config using trait method
                    push_to_stack(&self.vector_config, stack);

                    for i in (0..self.n_auth_usize).rev() {
                        let auth = {
                            let verify_variables: &mut VerifyVariables =
                                stack.get_verify_variables_mut();
                            let authentications = &mut verify_variables.authentications;
                            authentications[i]
                        };
                        stack.push_front(&auth.to_bytes_be()).unwrap();
                    }
                    stack
                        .push_front(&Felt::from(self.n_auth_usize).to_bytes_be())
                        .unwrap();

                    stack
                        .push_front(&Felt::from(self.auth_start + 1).to_bytes_be())
                        .unwrap();
                    stack
                        .push_front(&Felt::from(self.start + 1).to_bytes_be())
                        .unwrap();

                    // Push queries using trait method
                    push_queries_with_depth_to_stack(self.n_queries_usize, stack);

                    self.step = ComputeRootRecursiveStep::ReadHash;
                    vec![HashComputation::new(
                        self.current.value,
                        {
                            let verify_variables: &mut VerifyVariables =
                                stack.get_verify_variables_mut();
                            let authentications = &mut verify_variables.authentications;
                            authentications[self.auth_start]
                        },
                        self.is_verifier_friendly,
                    )
                    .to_vec_with_type_tag()]
                } else {
                    self.step = ComputeRootRecursiveStep::HashComputation;
                    vec![]
                }
            }

            ComputeRootRecursiveStep::HashComputation => {
                push_to_stack(&self.vector_config, stack);

                for i in (0..self.n_auth_usize).rev() {
                    let auth = {
                        let verify_variables: &mut VerifyVariables =
                            stack.get_verify_variables_mut();
                        let authentications = &mut verify_variables.authentications;
                        authentications[i]
                    };
                    stack.push_front(&auth.to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(self.n_auth_usize).to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&Felt::from(self.auth_start + 1).to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&Felt::from(self.start + 1).to_bytes_be())
                    .unwrap();

                // Push queries using trait method
                push_queries_with_depth_to_stack(self.n_queries_usize, stack);

                self.step = ComputeRootRecursiveStep::ReadHash;
                // Create hash computation task
                vec![HashComputation::new(
                    {
                        let verify_variables: &mut VerifyVariables =
                            stack.get_verify_variables_mut();
                        let authentications = &mut verify_variables.authentications;
                        authentications[self.auth_start]
                    },
                    self.current.value,
                    self.is_verifier_friendly,
                )
                .to_vec_with_type_tag()]
            }

            ComputeRootRecursiveStep::HashComputationWithQueries => {
                  // Push vector config using trait method
                  println!("hash computation with queries step Poseidon");
                  push_to_stack(&self.vector_config, stack);

                  for i in (0..self.n_auth_usize).rev() {
                      let auth = {
                          let verify_variables: &mut VerifyVariables =
                              stack.get_verify_variables_mut();
                          let authentications = &mut verify_variables.authentications;
                          authentications[i]
                      };
                      stack.push_front(&auth.to_bytes_be()).unwrap();
                  }
                  stack
                      .push_front(&Felt::from(self.n_auth_usize).to_bytes_be())
                      .unwrap();

                  stack
                      .push_front(&Felt::from(self.auth_start).to_bytes_be())
                      .unwrap();
                  stack
                      .push_front(&Felt::from(self.start + 2).to_bytes_be())
                      .unwrap();

                  // Push queries using trait method
                  push_queries_with_depth_to_stack(self.n_queries_usize, stack);

                  self.step = ComputeRootRecursiveStep::ProcessCurrent;

                  vec![HashComputationWithQueries::new(
                      self.current.value,
                      self.next_value,
                      self.is_verifier_friendly,
                      self.parent,
                      self.current.depth - Felt::ONE,
                  )
                  .to_vec_with_type_tag()]
            }
            ComputeRootRecursiveStep::ReadHash => {
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.push_front(&queries_len.to_bytes_be()).unwrap();
                read_queries_with_depth_from_stack(stack);

                // Add new query to pre-allocated array
                let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                let queries_slice = &mut verify_variables.queries;

                let next_slot: usize = (queries_len).try_into().unwrap();

                // Add new query with bounds checking
                assert!(
                    next_slot < FUNVEC_QUERIES / 3,
                    "Queries array full: next_slot={}, max_slots={}",
                    next_slot,
                    FUNVEC_QUERIES / 3
                );
                queries_slice[next_slot * 3] = self.parent;
                queries_slice[next_slot * 3 + 1] = hash;
                queries_slice[next_slot * 3 + 2] = self.current.depth - Felt::ONE;

                push_queries_with_depth_to_stack(next_slot+1, stack);

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

/// Read queries with depth from stack and store them in a mutable slice (no allocation)
pub fn read_queries_with_depth_from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) {
    let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    let n_queries_usize: usize = n_queries.try_into().unwrap();
    assert!(
        n_queries_usize <= FUNVEC_QUERIES,
        "Too many queries: {} > {}",
        n_queries_usize,
        FUNVEC_QUERIES
    );

    // Clear the entire queries array first
    {
        let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
        let queries_slice = &mut verify_variables.queries;
        queries_slice.fill(Felt::ZERO);
    }

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
}

fn from_stack<T: BidirectionalStack>(stack: &mut T) -> VectorConfig {
    let height = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let n_verifier_friendly_commitment_layers = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    VectorConfig::new(height, n_verifier_friendly_commitment_layers)
}

fn push_to_stack<T: BidirectionalStack>(config: &VectorConfig, stack: &mut T) {
    stack
        .push_front(&config.n_verifier_friendly_commitment_layers.to_bytes_be())
        .unwrap();
    stack.push_front(&config.height.to_bytes_be()).unwrap();
}
