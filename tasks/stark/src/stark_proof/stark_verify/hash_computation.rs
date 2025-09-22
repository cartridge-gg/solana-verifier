use felt::Felt;
use sha3::{Digest, Keccak256};
use types::funvec::FUNVEC_QUERIES;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::poseidon::PoseidonHash;
use types::swiftness::stark::types::VerifyVariables;

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
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            HashComputationWithQueriesStep::Init => {
                if self.is_verifier_friendly {
                    PoseidonHash::push_input(self.x, self.y, stack);
                    self.step = HashComputationWithQueriesStep::WaitForPoseidonHash;
                    vec![PoseidonHash::new().to_vec_with_type_tag()]
                } else {
                    let hash = keccak_hash(self.x, self.y);

                    // Read queue using trait method
                    read_queries_with_depth_from_stack(stack);

                    // Add new query to pre-allocated array
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.queries;

                    // Find next available slot
                    let mut next_slot = 0;
                    while next_slot < queries_slice.len() / 3
                        && queries_slice[next_slot * 3] != Felt::ZERO
                    {
                        next_slot += 1;
                    }

                    // Check if we found a free slot
                    assert!(
                        next_slot < queries_slice.len() / 3,
                        "No free slot for query, next_slot: {}, max: {}",
                        next_slot,
                        queries_slice.len() / 3
                    );

                    // Add new query
                    queries_slice[next_slot * 3] = self.parent_index;
                    queries_slice[next_slot * 3 + 1] = hash;
                    queries_slice[next_slot * 3 + 2] = self.parent_depth;

                    // Push queue using trait method - calculate actual count
                    let actual_count = {
                        let verify_variables: &mut VerifyVariables =
                            stack.get_verify_variables_mut();
                        let queries_slice = &mut verify_variables.queries;
                        let mut count = 0;
                        for i in 0..(queries_slice.len() / 3) {
                            if queries_slice[i * 3] != Felt::ZERO {
                                count = i + 1;
                            }
                        }
                        count
                    };
                    push_queries_with_depth_to_stack(actual_count, stack);

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

                // Read queue using trait method
                read_queries_with_depth_from_stack(stack);

                // Add new query to pre-allocated array
                let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                let queries_slice = &mut verify_variables.queries;

                // Find next available slot
                let mut next_slot = 0;
                while next_slot < queries_slice.len() / 3
                    && queries_slice[next_slot * 3] != Felt::ZERO
                {
                    next_slot += 1;
                }

                // Check if we found a free slot
                assert!(
                    next_slot < queries_slice.len() / 3,
                    "No free slot for query, next_slot: {}, max: {}",
                    next_slot,
                    queries_slice.len() / 3
                );

                // Add new query
                queries_slice[next_slot * 3] = self.parent_index;
                queries_slice[next_slot * 3 + 1] = hash;
                queries_slice[next_slot * 3 + 2] = self.parent_depth;

                // Push queue using trait method - calculate actual count
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
                push_queries_with_depth_to_stack(actual_count, stack);

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
