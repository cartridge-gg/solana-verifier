// use crate::swiftness::commitment::vector::types::{Commitment, Witness};
// use crate::swiftness::commitment::vector::config::Config;
// use crate::funvec::FunVec;
use felt::{Felt, NonZeroFelt};
use sha3::{Digest, Keccak256};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::poseidon::PoseidonHash;
use crate::swiftness::commitment::vector::types::{Query, QueryWithDepth};
// Error types for vector decommit operations
#[derive(Debug, Clone)]
pub enum VectorDecommitError {
    MisMatch { value: Felt, expected: Felt },
    AuthenticationInvalid,
    RootInvalid,
    IndexInvalid,
    ConversionError,
}

// Main VectorDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorDecommitStep {
    ReadAuthentications,
    VectorCommitmentDecommit,
    PrepareRootComputation,
    Done,
}

// Main VectorDecommit task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VectorDecommit {
    step: VectorDecommitStep,
}

impl_type_identifiable!(VectorDecommit);

impl VectorDecommit {
    pub fn new() -> Self {
        Self {
            step: VectorDecommitStep::ReadAuthentications,
        }
    }
}

impl Default for VectorDecommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for VectorDecommit {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VectorDecommitStep::ReadAuthentications => {
                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut authentications = Vec::new();

                let n_auth_usize: usize = match n_authentications.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        // Push error indicator and return
                        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                        self.step = VectorDecommitStep::Done;
                        return vec![];
                    }
                };

                for _ in 0..n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    authentications.push(auth);
                }

                // Store authentications for next task
                for auth in authentications.iter().rev() {
                    stack.push_front(&auth.to_bytes_be()).unwrap();
                }
                stack.push_front(&n_authentications.to_bytes_be()).unwrap();

                self.step = VectorDecommitStep::PrepareRootComputation;
                vec![]
            }

            VectorDecommitStep::VectorCommitmentDecommit => {
                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut authentications = Vec::new();

                let n_auth_usize: usize = match n_authentications.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                        self.step = VectorDecommitStep::Done;
                        return vec![];
                    }
                };

                for _ in 0..n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    authentications.push(auth);
                }

                // Read queries using helper method
                let queries = match Query::read_queries_from_stack(stack) {
                    Ok(queries) => queries,
                    Err(_) => {
                        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                        self.step = VectorDecommitStep::Done;
                        return vec![];
                    }
                };

                // Read commitment data
                let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let height = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_verifier_friendly_layers = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push data for ComputeRootFromQueries task
                stack
                    .push_front(&n_verifier_friendly_layers.to_bytes_be())
                    .unwrap();
                stack.push_front(&height.to_bytes_be()).unwrap();
                stack.push_front(&commitment_hash.to_bytes_be()).unwrap();

                // Apply shift logic from original vector_commitment_decommit:
                // let shift = Felt::TWO.pow_felt(&commitment.config.height);
                // Shifts the query indices by shift=2**height, to convert index representation to heap-like.
                let shift = Felt::TWO.pow_felt(&height);
                let mut shifted_queries = Vec::with_capacity(queries.len());

                for query in &queries {
                    shifted_queries
                        .push(QueryWithDepth::from_query_with_shift(query, height, shift));
                }

                // Push shifted queries with depth for ComputeRootFromQueries task
                for query in shifted_queries.iter().rev() {
                    stack.push_front(&query.depth.to_bytes_be()).unwrap();
                    stack.push_front(&query.value.to_bytes_be()).unwrap();
                    stack.push_front(&query.index.to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(shifted_queries.len()).to_bytes_be())
                    .unwrap();

                // Push authentications
                for auth in authentications.iter().rev() {
                    stack.push_front(&auth.to_bytes_be()).unwrap();
                }
                stack.push_front(&n_authentications.to_bytes_be()).unwrap();

                self.step = VectorDecommitStep::PrepareRootComputation;
                vec![]
            }

            VectorDecommitStep::PrepareRootComputation => {
                // Read data from stack
                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut authentications = Vec::new();

                let n_auth_usize: usize = match n_authentications.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                        self.step = VectorDecommitStep::Done;
                        return vec![];
                    }
                };

                for _ in 0..n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    authentications.push(auth);
                }

                let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut queries = Vec::new();
                let n_queries = n_queries.to_biguint();

                let n_queries_usize: usize = match n_queries.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                        self.step = VectorDecommitStep::Done;
                        return vec![];
                    }
                };

                for _ in 0..n_queries_usize {
                    let shifted_index = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let depth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    queries.push(QueryWithDepth {
                        index: shifted_index,
                        value,
                        depth,
                    });
                }

                let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let height = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_verifier_friendly_layers = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Store all data back for processing step
                stack
                    .push_front(&n_verifier_friendly_layers.to_bytes_be())
                    .unwrap();
                stack.push_front(&height.to_bytes_be()).unwrap();
                stack.push_front(&commitment_hash.to_bytes_be()).unwrap();

                for query in queries.iter().rev() {
                    stack.push_front(&query.depth.to_bytes_be()).unwrap();
                    stack.push_front(&query.value.to_bytes_be()).unwrap();
                    stack.push_front(&query.index.to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(n_queries_usize).to_bytes_be())
                    .unwrap();

                for auth in authentications.iter().rev() {
                    stack.push_front(&auth.to_bytes_be()).unwrap();
                }
                stack.push_front(&n_authentications.to_bytes_be()).unwrap();

                // Push data for ComputeRootRecursive task
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap(); // auth_start
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap(); // start

                for auth in authentications.iter().rev() {
                    stack.push_front(&auth.to_bytes_be()).unwrap();
                }
                stack.push_front(&n_authentications.to_bytes_be()).unwrap();

                stack
                    .push_front(&n_verifier_friendly_layers.to_bytes_be())
                    .unwrap();
                stack.push_front(&commitment_hash.to_bytes_be()).unwrap();

                for query in queries.iter().rev() {
                    stack.push_front(&query.depth.to_bytes_be()).unwrap();
                    stack.push_front(&query.value.to_bytes_be()).unwrap();
                    stack.push_front(&query.index.to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(n_queries_usize).to_bytes_be())
                    .unwrap();

                self.step = VectorDecommitStep::Done;
                vec![ComputeRootRecursive::new().to_vec_with_type_tag()]
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
// ComputeRootRecursive task - handles one step of the recursive root computation
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeRootRecursive {
    step: ComputeRootRecursiveStep,
    pending_hash_computation: Option<PendingHashComputation>,
}

#[derive(Debug, Clone)]
struct PendingHashComputation {
    parent_index: Felt,
    parent_depth: Felt,
    next_start: usize,
    next_auth_start: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeRootRecursiveStep {
    ProcessCurrent,
    WaitForHashResult,
    CheckResult,
    Done,
}

impl_type_identifiable!(ComputeRootRecursive);

impl ComputeRootRecursive {
    pub fn new() -> Self {
        Self {
            step: ComputeRootRecursiveStep::ProcessCurrent,
            pending_hash_computation: None,
        }
    }

    /// Helper method to compute hash and continue with next iteration
    fn compute_hash_and_continue<T: BidirectionalStack + ProofData>(
        &mut self,
        stack: &mut T,
        x: Felt,
        y: Felt,
        is_verifier_friendly: bool,
        parent_index: Felt,
        parent_depth: Felt,
        queue: &mut Vec<QueryWithDepth>,
        start: usize,
        n_verifier_friendly_layers: Felt,
        commitment_hash: Felt,
        authentications: &[Felt],
        auth_start: usize,
    ) -> Vec<Vec<u8>> {
        if is_verifier_friendly {
            // Use Poseidon hash - push task and wait for result
            PoseidonHash::push_input(x, y, stack);

            self.pending_hash_computation = Some(PendingHashComputation {
                parent_index,
                parent_depth,
                next_start: start,
                next_auth_start: auth_start,
            });

            self.step = ComputeRootRecursiveStep::WaitForHashResult;
            vec![PoseidonHash::new().to_vec_with_type_tag()]
        } else {
            // Use Keccak hash - compute directly
            let hash = keccak_hash(x, y);
            queue.push(QueryWithDepth {
                index: parent_index,
                value: hash,
                depth: parent_depth,
            });

            // Continue with next iteration
            self.prepare_next_iteration(
                stack,
                queue,
                start,
                n_verifier_friendly_layers,
                commitment_hash,
                authentications,
                auth_start,
            );
            vec![ComputeRootRecursive::new().to_vec_with_type_tag()]
        }
    }

    /// Helper method to handle authentication errors
    fn handle_auth_error<T: BidirectionalStack + ProofData>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        self.step = ComputeRootRecursiveStep::Done;
        vec![]
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
                // Check if we have a pending hash result
                if let Some(pending) = &self.pending_hash_computation {
                    if !stack.is_empty_front() {
                        let hash_result = Felt::from_bytes_be_slice(stack.borrow_front());
                        stack.pop_front();

                        // Continue with the pending computation
                        // We need to read the current state from stack and continue
                        self.pending_hash_computation = None;
                        self.step = ComputeRootRecursiveStep::ProcessCurrent;
                        return vec![ComputeRootRecursive::new().to_vec_with_type_tag()];
                    }
                }

                // Read queue data
                let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut queue = Vec::new();

                let n_queries_usize: usize = match n_queries.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        self.step = ComputeRootRecursiveStep::Done;
                        return vec![];
                    }
                };

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

                let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_verifier_friendly_layers = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut authentications = Vec::new();

                let n_auth_usize: usize = match n_authentications.try_into() {
                    Ok(val) => val,
                    Err(_) => {
                        self.step = ComputeRootRecursiveStep::Done;
                        return vec![];
                    }
                };

                for _ in 0..n_auth_usize {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    authentications.push(auth);
                }

                let start: usize = match Felt::from_bytes_be_slice(stack.borrow_front()).try_into()
                {
                    Ok(val) => val,
                    Err(_) => {
                        self.step = ComputeRootRecursiveStep::Done;
                        return vec![];
                    }
                };
                stack.pop_front();

                let auth_start: usize =
                    match Felt::from_bytes_be_slice(stack.borrow_front()).try_into() {
                        Ok(val) => val,
                        Err(_) => {
                            self.step = ComputeRootRecursiveStep::Done;
                            return vec![];
                        }
                    };
                stack.pop_front();

                // Process one step of the algorithm
                if start >= queue.len() {
                    self.step = ComputeRootRecursiveStep::Done;
                    return vec![];
                }

                let current = &queue[start];

                if current.index == Felt::ONE {
                    // We've reached the root - compare with expected commitment
                    if commitment_hash == current.value {
                        stack.push_front(&Felt::ONE.to_bytes_be()).unwrap(); // Success
                    } else {
                        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap(); // Mismatch
                    }
                    self.step = ComputeRootRecursiveStep::Done;
                    return vec![];
                }

                let parent = current
                    .index
                    .div_rem(&NonZeroFelt::from_felt_unchecked(Felt::TWO))
                    .0;
                let bit = current
                    .index
                    .div_rem(&NonZeroFelt::from_felt_unchecked(Felt::TWO))
                    .1;
                let is_verifier_friendly = n_verifier_friendly_layers >= current.depth;

                if bit == Felt::ZERO {
                    if start + 1 < queue.len() && current.index + 1 == queue[start + 1].index {
                        // next is a sibling of current
                        self.compute_hash_and_continue(
                            stack,
                            current.value,
                            queue[start + 1].value,
                            is_verifier_friendly,
                            parent,
                            current.depth - 1,
                            &mut queue,
                            start + 2,
                            n_verifier_friendly_layers,
                            commitment_hash,
                            &authentications,
                            auth_start,
                        )
                    } else {
                        if auth_start >= authentications.len() {
                            return self.handle_auth_error(stack);
                        }

                        self.compute_hash_and_continue(
                            stack,
                            current.value,
                            authentications[auth_start],
                            is_verifier_friendly,
                            parent,
                            current.depth - 1,
                            &mut queue,
                            start + 1,
                            n_verifier_friendly_layers,
                            commitment_hash,
                            &authentications,
                            auth_start + 1,
                        )
                    }
                } else {
                    if auth_start >= authentications.len() {
                        return self.handle_auth_error(stack);
                    }

                    self.compute_hash_and_continue(
                        stack,
                        authentications[auth_start],
                        current.value,
                        is_verifier_friendly,
                        parent,
                        current.depth - 1,
                        &mut queue,
                        start + 1,
                        n_verifier_friendly_layers,
                        commitment_hash,
                        &authentications,
                        auth_start + 1,
                    )
                }
            }

            ComputeRootRecursiveStep::WaitForHashResult => {
                // Wait for hash result from PoseidonHash task
                if !stack.is_empty_front() {
                    let hash_result = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();

                    if let Some(pending) = &self.pending_hash_computation {
                        // Continue with next iteration using the hash result
                        self.pending_hash_computation = None;
                        self.step = ComputeRootRecursiveStep::ProcessCurrent;
                        return vec![ComputeRootRecursive::new().to_vec_with_type_tag()];
                    }
                }

                // Still waiting for result
                vec![]
            }

            ComputeRootRecursiveStep::CheckResult => {
                self.step = ComputeRootRecursiveStep::Done;
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

impl ComputeRootRecursive {
    fn prepare_next_iteration<T: BidirectionalStack + ProofData>(
        &mut self,
        stack: &mut T,
        queue: &mut Vec<QueryWithDepth>,
        start: usize,
        n_verifier_friendly_layers: Felt,
        commitment_hash: Felt,
        authentications: &[Felt],
        auth_start: usize,
    ) {
        // Push auth_start and start
        stack
            .push_front(&Felt::from(auth_start).to_bytes_be())
            .unwrap();
        stack.push_front(&Felt::from(start).to_bytes_be()).unwrap();

        // Push authentications
        for auth in authentications.iter().rev() {
            stack.push_front(&auth.to_bytes_be()).unwrap();
        }
        stack
            .push_front(&Felt::from(authentications.len()).to_bytes_be())
            .unwrap();

        // Push commitment data
        stack
            .push_front(&n_verifier_friendly_layers.to_bytes_be())
            .unwrap();
        stack.push_front(&commitment_hash.to_bytes_be()).unwrap();

        // Push queue
        for query in queue.iter().rev() {
            stack.push_front(&query.depth.to_bytes_be()).unwrap();
            stack.push_front(&query.value.to_bytes_be()).unwrap();
            stack.push_front(&query.index.to_bytes_be()).unwrap();
        }
        stack
            .push_front(&Felt::from(queue.len()).to_bytes_be())
            .unwrap();

        self.step = ComputeRootRecursiveStep::ProcessCurrent;
    }
}

/// Keccak hash function for non-verifier-friendly layers
fn keccak_hash(x: Felt, y: Felt) -> Felt {
    let mut hash_data = Vec::with_capacity(64);
    hash_data.extend(&x.to_bytes_be());
    hash_data.extend(&y.to_bytes_be());

    let mut hasher = Keccak256::new();
    hasher.update(&hash_data);
    Felt::from_bytes_be_slice(&hasher.finalize().as_slice()[12..32])
}
