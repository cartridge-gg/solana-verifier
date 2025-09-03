use felt::Felt;
use sha3::{Digest, Keccak256};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::poseidon::PoseidonHash;
use crate::swiftness::commitment::vector::types::QueryWithDepth;

// New tasks to replace method calls
#[derive(Debug, Clone)]
#[repr(C)]
pub struct HashComputation {
    step: HashComputationStep,
    pub x: Felt,
    pub y: Felt,
    pub is_verifier_friendly: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashComputationStep {
    Init,
    WaitForPoseidonHash,
    Done,
}

impl HashComputation {
    pub fn new(x: Felt, y: Felt, is_verifier_friendly: bool) -> Self {
        Self {
            step: HashComputationStep::Init,
            x,
            y,
            is_verifier_friendly,
        }
    }
}

impl Default for HashComputation {
    fn default() -> Self {
        Self::new(Felt::ZERO, Felt::ZERO, false)
    }
}

impl_type_identifiable!(HashComputation);

impl Executable for HashComputation {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            HashComputationStep::Init => {
                if self.is_verifier_friendly {
                    PoseidonHash::push_input(self.x, self.y, stack);
                    self.step = HashComputationStep::WaitForPoseidonHash;
                    vec![PoseidonHash::new().to_vec_with_type_tag()]
                } else {
                    let hash = keccak_hash(self.x, self.y);
                    stack.push_front(&hash.to_bytes_be()).unwrap();

                    self.step = HashComputationStep::Done;
                    vec![]
                }
            }
            HashComputationStep::WaitForPoseidonHash => {
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                stack.push_front(&hash.to_bytes_be()).unwrap();

                self.step = HashComputationStep::Done;
                vec![]
            }
            HashComputationStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == HashComputationStep::Done
    }
}

#[inline(always)]
fn keccak_hash(x: Felt, y: Felt) -> Felt {
    let mut hash_data = Vec::with_capacity(64);
    hash_data.extend(&x.to_bytes_be());
    hash_data.extend(&y.to_bytes_be());

    let mut hasher = Keccak256::new();
    hasher.update(&hash_data);
    Felt::from_bytes_be_slice(&hasher.finalize().as_slice()[12..32])
}

// New tasks to replace method calls
#[derive(Debug, Clone)]
#[repr(C)]
pub struct HashComputationWithQueries {
    step: HashComputationWithQueriesStep,
    pub x: Felt,
    pub y: Felt,
    pub is_verifier_friendly: bool,
    pub parent_index: Felt,
    pub parent_depth: Felt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashComputationWithQueriesStep {
    Init,
    WaitForPoseidonHash,
    Done,
}

impl HashComputationWithQueries {
    pub fn new(
        x: Felt,
        y: Felt,
        is_verifier_friendly: bool,
        parent_index: Felt,
        parent_depth: Felt,
    ) -> Self {
        Self {
            step: HashComputationWithQueriesStep::Init,
            x,
            y,
            is_verifier_friendly,
            parent_index,
            parent_depth,
        }
    }
}

impl Default for HashComputationWithQueries {
    fn default() -> Self {
        Self::new(Felt::ZERO, Felt::ZERO, false, Felt::ZERO, Felt::ZERO)
    }
}

impl_type_identifiable!(HashComputationWithQueries);

impl Executable for HashComputationWithQueries {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            HashComputationWithQueriesStep::Init => {
                if self.is_verifier_friendly {
                    PoseidonHash::push_input(self.x, self.y, stack);
                    self.step = HashComputationWithQueriesStep::WaitForPoseidonHash;
                    vec![PoseidonHash::new().to_vec_with_type_tag()]
                } else {
                    let hash = keccak_hash(self.x, self.y);

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
                        index: self.parent_index,
                        value: hash,
                        depth: self.parent_depth,
                    });

                    for query in queue.iter().rev() {
                        stack.push_front(&query.depth.to_bytes_be()).unwrap();
                        stack.push_front(&query.value.to_bytes_be()).unwrap();
                        stack.push_front(&query.index.to_bytes_be()).unwrap();
                    }

                    stack
                        .push_front(&Felt::from(queue.len()).to_bytes_be())
                        .unwrap();

                    stack.push_front(&hash.to_bytes_be()).unwrap();

                    self.step = HashComputationWithQueriesStep::Done;
                    vec![]
                }
            }
            HashComputationWithQueriesStep::WaitForPoseidonHash => {
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                // Read queue data from stack
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
                    index: self.parent_index,
                    value: hash,
                    depth: self.parent_depth,
                });

                for query in queue.iter().rev() {
                    stack.push_front(&query.depth.to_bytes_be()).unwrap();
                    stack.push_front(&query.value.to_bytes_be()).unwrap();
                    stack.push_front(&query.index.to_bytes_be()).unwrap();
                }

                stack
                    .push_front(&Felt::from(queue.len()).to_bytes_be())
                    .unwrap();

                stack.push_front(&hash.to_bytes_be()).unwrap();

                self.step = HashComputationWithQueriesStep::Done;
                vec![]
            }
            HashComputationWithQueriesStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == HashComputationWithQueriesStep::Done
    }
}
