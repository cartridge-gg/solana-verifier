use felt::Felt;
use sha3::{Digest, Keccak256};
use types::funvec::{FunVec, FUNVEC_QUERIES};
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use poseidon::poseidon::PoseidonHash;
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
                    stack.push_front(&self.y.to_bytes_be()).unwrap();
                    stack.push_front(&self.x.to_bytes_be()).unwrap();
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

// #[inline(always)]
fn keccak_hash(x: Felt, y: Felt) -> Felt {
    let mut hash_data = FunVec::<u8, 64>::default();
    hash_data.extend(x.to_bytes_be().as_slice());
    hash_data.extend(y.to_bytes_be().as_slice());

    let mut hasher = Keccak256::new();
    hasher.update(hash_data.as_slice());
    Felt::from_bytes_be_slice(&hasher.finalize()[12..32])
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
                    stack.push_front(&self.y.to_bytes_be()).unwrap();
                    stack.push_front(&self.x.to_bytes_be()).unwrap();
                    self.step = HashComputationWithQueriesStep::WaitForPoseidonHash;
                    vec![PoseidonHash::new().to_vec_with_type_tag()]
                } else {
                    let hash = keccak_hash(self.x, self.y);

                    let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();

                    // Add new query to pre-allocated array
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.queries;

                    let next_slot: usize = (queries_len + Felt::ONE).try_into().unwrap();

                    // Check if we found a free slot
                    assert!(
                        next_slot < FUNVEC_QUERIES / 3,
                        "No free slot for query, next_slot: {}, max: {}",
                        next_slot,
                        FUNVEC_QUERIES / 3
                    );

                    // Add new query
                    queries_slice[next_slot * 3] = self.parent_index;
                    queries_slice[next_slot * 3 + 1] = hash;
                    queries_slice[next_slot * 3 + 2] = self.parent_depth;

                    stack
                        .push_front(&Felt::from(next_slot + 1).to_bytes_be())
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

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Add new query to pre-allocated array
                let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                let queries_slice = &mut verify_variables.queries;

                let next_slot: usize = (queries_len).try_into().unwrap();

                // Check if we found a free slot
                assert!(
                    next_slot < FUNVEC_QUERIES / 3,
                    "No free slot for query, next_slot: {}, max: {}",
                    next_slot,
                    FUNVEC_QUERIES / 3
                );

                // Add new query
                queries_slice[next_slot * 3] = self.parent_index;
                queries_slice[next_slot * 3 + 1] = hash;
                queries_slice[next_slot * 3 + 2] = self.parent_depth;

                stack
                    .push_front(&Felt::from(next_slot + 1).to_bytes_be())
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
