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
    n_verifier_friendly_commitment_layers: Felt,
    next_value: Felt,
    is_verifier_friendly: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeRootRecursiveStep {
    Init,
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
            step: ComputeRootRecursiveStep::Init,
            n_auth_usize: 0,
            parent: Felt::ZERO,
            current: QueryWithDepth::default(),
            start: 0,
            auth_start: 0,
            n_queries_usize: 0,
            n_verifier_friendly_commitment_layers: Felt::ZERO,
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
            ComputeRootRecursiveStep::Init => {
                println!("compute root recursive step Init");
                
                // Read all read-only data once at the beginning
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

                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.n_auth_usize = n_authentications.try_into().unwrap();
                assert!(
                    self.n_auth_usize <= FUNVEC_AUTHENTICATIONS,
                    "Too many authentications: {} > {}",
                    self.n_auth_usize,
                    FUNVEC_AUTHENTICATIONS
                );
                
                // Read authentications once
                for i in 0..self.n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let authentications = &mut verify_variables.authentications;
                    authentications[i] = auth;
                }
                
                let _height = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.n_verifier_friendly_commitment_layers = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.step = ComputeRootRecursiveStep::Continue;
                vec![]
            }
            ComputeRootRecursiveStep::ProcessCurrent => {
                println!("compute root recursive step ProcessCurrent");
                 // Read all read-only data once at the beginning
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
 
                //  for i in 0..self.n_queries_usize {
                //      let index = Felt::from_bytes_be_slice(stack.borrow_front());
                //      stack.pop_front();
                //      let value = Felt::from_bytes_be_slice(stack.borrow_front());
                //      stack.pop_front();
                //      let depth = Felt::from_bytes_be_slice(stack.borrow_front());
                //      stack.pop_front();
 
                //      let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                //      let queries_slice = &mut verify_variables.queries;
                //      queries_slice[i * 3] = index;
                //      queries_slice[i * 3 + 1] = value;
                //      queries_slice[i * 3 + 2] = depth;
                //  }
 
                 self.start = Felt::from_bytes_be_slice(stack.borrow_front())
                     .try_into()
                     .unwrap();
                 stack.pop_front();
 
                 self.auth_start = Felt::from_bytes_be_slice(stack.borrow_front())
                     .try_into()
                     .unwrap();
                 stack.pop_front();

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
                
                if self.current.index == Felt::ONE {
                    stack.push_front(&self.current.value.to_bytes_be()).unwrap();
                    self.step = ComputeRootRecursiveStep::Done;
                    vec![]
                } else {
                    self.step = ComputeRootRecursiveStep::Continue;
                    vec![]
                }
            }
            ComputeRootRecursiveStep::Continue => {
                println!("compute root recursive step Continue");
                let (parent, bit) = self.current.index.div_rem(&NonZeroFelt::TWO);
                self.is_verifier_friendly = self.n_verifier_friendly_commitment_layers >= self.current.depth;
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

                    stack
                        .push_front(&Felt::from(self.auth_start + 1).to_bytes_be())
                        .unwrap();
                    stack
                        .push_front(&Felt::from(self.start + 1).to_bytes_be())
                        .unwrap();

                    stack.push_front(&Felt::from(self.n_queries_usize).to_bytes_be()).unwrap();
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
                println!("compute root recursive step HashComputation");
                stack
                    .push_front(&Felt::from(self.auth_start + 1).to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&Felt::from(self.start + 1).to_bytes_be())
                    .unwrap();

                stack.push_front(&Felt::from(self.n_queries_usize).to_bytes_be()).unwrap();

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
                println!("compute root recursive step HashComputationWithQueries");
                  // Push vector config using trait method
                  println!("hash computation with queries step Poseidon");

                  stack
                      .push_front(&Felt::from(self.auth_start).to_bytes_be())
                      .unwrap();
                  stack
                      .push_front(&Felt::from(self.start + 2).to_bytes_be())
                      .unwrap();
                  stack.push_front(&Felt::from(self.n_queries_usize).to_bytes_be()).unwrap();

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
                println!("compute root recursive step ReadHash");
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let queries_len:usize = Felt::from_bytes_be_slice(stack.borrow_front()).try_into().unwrap();
                stack.pop_front();
                // stack.push_front(&Felt::from(queries_len).to_bytes_be()).unwrap();
                // read_queries_with_depth_from_stack(stack);

                // Add new query to pre-allocated array
                let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                let queries_slice = &mut verify_variables.queries;

                // Add new query with bounds checking
                assert!(
                    queries_len < FUNVEC_QUERIES / 3,
                    "Queries array full: next_slot={}, max_slots={}",
                    queries_len,
                    FUNVEC_QUERIES / 3
                );
                queries_slice[queries_len * 3] = self.parent;
                queries_slice[queries_len * 3 + 1] = hash;
                queries_slice[queries_len * 3 + 2] = self.current.depth - Felt::ONE;

                stack.push_front(&Felt::from(queries_len+1).to_bytes_be()).unwrap();
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