use felt::{Felt, NonZeroFelt};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::stark_proof::stark_verify::hash_computation::{
    HashComputation, HashComputationWithQueries,
};
use crate::swiftness::commitment::vector::config::{Config as VectorConfig, ConfigTrait};
use crate::swiftness::commitment::vector::types::QueryWithDepth;

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
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            ComputeRootRecursiveStep::ProcessCurrent => {
                let _computed_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut queue = Vec::new();
                let n_queries_usize: usize = n_queries.try_into().unwrap();

                for _ in 0..n_queries_usize {
                    let index = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let depth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    queue.push(QueryWithDepth {
                        index,
                        value,
                        depth,
                    });
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
                let mut authentications = Vec::new();

                let n_auth_usize: usize = n_authentications.try_into().unwrap();

                for _ in 0..n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    authentications.push(auth);
                }

                // Read vector config using trait method
                let vector_config = VectorConfig::from_stack(stack);
                let n_verifier_friendly_layers =
                    vector_config.n_verifier_friendly_commitment_layers;

                let current = &queue[start];
                self.current = current.clone();
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
                        if start + 1 < queue.len() {
                            let next = &queue[start + 1];

                            if self.current.index + Felt::ONE == next.index {
                                // Push vector config using trait method
                                vector_config.push_to_stack(stack);

                                for auth in authentications.iter().rev() {
                                    stack.push_front(&auth.to_bytes_be()).unwrap();
                                }
                                stack
                                    .push_front(&Felt::from(authentications.len()).to_bytes_be())
                                    .unwrap();

                                stack
                                    .push_front(&Felt::from(auth_start).to_bytes_be())
                                    .unwrap();
                                stack
                                    .push_front(&Felt::from(start + 2).to_bytes_be())
                                    .unwrap();

                                for query in queue.iter().rev() {
                                    stack.push_front(&query.depth.to_bytes_be()).unwrap();
                                    stack.push_front(&query.value.to_bytes_be()).unwrap();
                                    stack.push_front(&query.index.to_bytes_be()).unwrap();
                                }
                                stack
                                    .push_front(&Felt::from(queue.len()).to_bytes_be())
                                    .unwrap();

                                self.step = ComputeRootRecursiveStep::ProcessCurrent;

                                return vec![HashComputationWithQueries::new(
                                    self.current.value,
                                    next.value,
                                    is_verifier_friendly,
                                    self.parent,
                                    self.current.depth - Felt::ONE,
                                )
                                .to_vec_with_type_tag()];
                            }
                        }

                        // Push vector config using trait method
                        vector_config.push_to_stack(stack);

                        for auth in authentications.iter().rev() {
                            stack.push_front(&auth.to_bytes_be()).unwrap();
                        }
                        stack
                            .push_front(&Felt::from(authentications.len()).to_bytes_be())
                            .unwrap();

                        stack
                            .push_front(&Felt::from(auth_start + 1).to_bytes_be())
                            .unwrap();
                        stack
                            .push_front(&Felt::from(start + 1).to_bytes_be())
                            .unwrap();

                        for query in queue.iter().rev() {
                            stack.push_front(&query.depth.to_bytes_be()).unwrap();
                            stack.push_front(&query.value.to_bytes_be()).unwrap();
                            stack.push_front(&query.index.to_bytes_be()).unwrap();
                        }
                        stack
                            .push_front(&Felt::from(queue.len()).to_bytes_be())
                            .unwrap();

                        self.step = ComputeRootRecursiveStep::ReadHash;
                        vec![HashComputation::new(
                            self.current.value,
                            authentications[auth_start],
                            is_verifier_friendly,
                        )
                        .to_vec_with_type_tag()]
                    } else {
                        // Push vector config using trait method
                        vector_config.push_to_stack(stack);

                        for auth in authentications.iter().rev() {
                            stack.push_front(&auth.to_bytes_be()).unwrap();
                        }
                        stack
                            .push_front(&Felt::from(authentications.len()).to_bytes_be())
                            .unwrap();
                        stack
                            .push_front(&Felt::from(auth_start + 1).to_bytes_be())
                            .unwrap();
                        stack
                            .push_front(&Felt::from(start + 1).to_bytes_be())
                            .unwrap();

                        for query in queue.iter().rev() {
                            stack.push_front(&query.depth.to_bytes_be()).unwrap();
                            stack.push_front(&query.value.to_bytes_be()).unwrap();
                            stack.push_front(&query.index.to_bytes_be()).unwrap();
                        }
                        stack
                            .push_front(&Felt::from(queue.len()).to_bytes_be())
                            .unwrap();

                        self.step = ComputeRootRecursiveStep::ReadHash;
                        // Create hash computation task
                        vec![HashComputation::new(
                            authentications[auth_start],
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

                let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut queue = Vec::new();
                let n_queries_usize: usize = n_queries.try_into().unwrap();

                for _ in 0..n_queries_usize {
                    let index = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let depth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    queue.push(QueryWithDepth {
                        index,
                        value,
                        depth,
                    });
                }

                queue.push(QueryWithDepth {
                    index: self.parent,
                    value: hash,
                    depth: self.current.depth - Felt::ONE,
                });

                // Push queue using trait method
                QueryWithDepth::push_queries_with_depth_to_stack(&queue, stack);

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
