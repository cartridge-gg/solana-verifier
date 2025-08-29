// use crate::swiftness::commitment::vector::types::{Commitment, Witness};
// use crate::swiftness::commitment::vector::config::Config;
// use crate::funvec::FunVec;
// use felt::Felt;
// use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

// // Query represents a single query to the vector commitment
// #[derive(Debug, Clone)]
// pub struct Query {
//     pub index: Felt,
//     pub value: Felt,
// }

// // QueryWithDepth extends Query with depth information for tree traversal
// #[derive(Debug, Clone)]
// pub struct QueryWithDepth {
//     pub index: Felt,
//     pub value: Felt,
//     pub depth: Felt,
// }

// // Error types for vector decommit operations
// #[derive(Debug, Clone)]
// pub enum VectorDecommitError {
//     MisMatch { value: Felt, expected: Felt },
//     AuthenticationInvalid,
//     RootInvalid,
//     IndexInvalid,
// }

// impl std::fmt::Display for VectorDecommitError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             VectorDecommitError::MisMatch { value, expected } => {
//                 write!(f, "mismatch value {} expected {}", value, expected)
//             }
//             VectorDecommitError::AuthenticationInvalid => {
//                 write!(f, "authentications length is invalid")
//             }
//             VectorDecommitError::RootInvalid => {
//                 write!(f, "root tree-node error")
//             }
//             VectorDecommitError::IndexInvalid => {
//                 write!(f, "index invalid")
//             }
//         }
//     }
// }

// // Main VectorDecommit task
// #[derive(Debug, Clone)]
// #[repr(C)]
// pub struct VectorDecommit {
//     processed: bool,
// }

// impl_type_identifiable!(VectorDecommit);

// impl VectorDecommit {
//     pub fn new() -> Self {
//         Self { processed: false }
//     }
// }

// impl Default for VectorDecommit {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Executable for VectorDecommit {
//     fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
//         // Read commitment from stack
//         let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let height = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let n_verifier_friendly_layers = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();

//         let commitment = Commitment {
//             config: Config {
//                 height,
//                 n_verifier_friendly_commitment_layers: n_verifier_friendly_layers,
//             },
//             commitment_hash,
//         };

//         // Read queries from stack
//         let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let mut queries = Vec::new();
//         for _ in 0..n_queries.to_u64().unwrap() {
//             let value = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             let index = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             queries.push(Query { index, value });
//         }

//         // Read witness from stack
//         let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let mut authentications = Vec::new();
//         for _ in 0..n_authentications.to_u64().unwrap() {
//             let auth = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             authentications.push(auth);
//         }

//         let witness = Witness {
//             authentications: FunVec::from_vec(authentications),
//         };

//         // Push data for VectorCommitmentDecommit task
//         stack.push_front(&n_verifier_friendly_layers.to_bytes_be()).unwrap();
//         stack.push_front(&height.to_bytes_be()).unwrap();
//         stack.push_front(&commitment_hash.to_bytes_be()).unwrap();

//         // Push queries data
//         for query in queries.iter().rev() {
//             stack.push_front(&query.value.to_bytes_be()).unwrap();
//             stack.push_front(&query.index.to_bytes_be()).unwrap();
//         }
//         stack.push_front(&n_queries.to_bytes_be()).unwrap();

//         // Push authentications data
//         for auth in authentications.iter().rev() {
//             stack.push_front(&auth.to_bytes_be()).unwrap();
//         }
//         stack.push_front(&n_authentications.to_bytes_be()).unwrap();

//         self.processed = true;
//         vec![VectorCommitmentDecommit::new().to_vec_with_type_tag()]
//     }

//     fn is_finished(&mut self) -> bool {
//         self.processed
//     }
// }

// // VectorCommitmentDecommit task - handles the actual decommitment logic
// #[derive(Debug, Clone)]
// #[repr(C)]
// pub struct VectorCommitmentDecommit {
//     processed: bool,
// }

// impl_type_identifiable!(VectorCommitmentDecommit);

// impl VectorCommitmentDecommit {
//     pub fn new() -> Self {
//         Self { processed: false }
//     }
// }

// impl Default for VectorCommitmentDecommit {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Executable for VectorCommitmentDecommit {
//     fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
//         // Read data from stack
//         let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let mut authentications = Vec::new();
//         for _ in 0..n_authentications.to_u64().unwrap() {
//             let auth = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             authentications.push(auth);
//         }

//         let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let mut queries = Vec::new();
//         for _ in 0..n_queries.to_u64().unwrap() {
//             let index = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             let value = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             queries.push(Query { index, value });
//         }

//         let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let height = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let n_verifier_friendly_layers = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();

//         let commitment = Commitment {
//             config: Config {
//                 height,
//                 n_verifier_friendly_commitment_layers: n_verifier_friendly_layers,
//             },
//             commitment_hash,
//         };

//         let witness = Witness {
//             authentications: FunVec::from_vec(authentications),
//         };

//         // Push data for ComputeRootFromQueries task
//         stack.push_front(&n_verifier_friendly_layers.to_bytes_be()).unwrap();
//         stack.push_front(&height.to_bytes_be()).unwrap();
//         stack.push_front(&commitment_hash.to_bytes_be()).unwrap();

//         // Push queries with depth
//         let shift = Felt::TWO.pow_felt(&height);
//         for query in queries.iter().rev() {
//             let depth = height;
//             let shifted_index = query.index + shift;
//             stack.push_front(&depth.to_bytes_be()).unwrap();
//             stack.push_front(&query.value.to_bytes_be()).unwrap();
//             stack.push_front(&shifted_index.to_bytes_be()).unwrap();
//         }
//         stack.push_front(&n_queries.to_bytes_be()).unwrap();

//         // Push authentications
//         for auth in authentications.iter().rev() {
//             stack.push_front(&auth.to_bytes_be()).unwrap();
//         }
//         stack.push_front(&n_authentications.to_bytes_be()).unwrap();

//         self.processed = true;
//         vec![ComputeRootFromQueries::new().to_vec_with_type_tag()]
//     }

//     fn is_finished(&mut self) -> bool {
//         self.processed
//     }
// }

// // ComputeRootFromQueries task - handles the recursive root computation
// #[derive(Debug, Clone)]
// #[repr(C)]
// pub struct ComputeRootFromQueries {
//     processed: bool,
// }

// impl_type_identifiable!(ComputeRootFromQueries);

// impl ComputeRootFromQueries {
//     pub fn new() -> Self {
//         Self { processed: false }
//     }
// }

// impl Default for ComputeRootFromQueries {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Executable for ComputeRootFromQueries {
//     fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
//         // Read data from stack
//         let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let mut authentications = Vec::new();
//         for _ in 0..n_authentications.to_u64().unwrap() {
//             let auth = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             authentications.push(auth);
//         }

//         let n_queries = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let mut queries = Vec::new();

//         for _ in 0..n_queries.to_biguint() {
//             let shifted_index = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             let value = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             let depth = Felt::from_bytes_be_slice(stack.borrow_front());
//             stack.pop_front();
//             queries.push(QueryWithDepth { index: shifted_index, value, depth });
//         }

//         let commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let height = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();
//         let n_verifier_friendly_layers = Felt::from_bytes_be_slice(stack.borrow_front());
//         stack.pop_front();

//         // Compute root using the recursive algorithm
//         let result = compute_root_from_queries(
//             queries,
//             0,
//             n_verifier_friendly_layers,
//             authentications,
//             0,
//         );

//         match result {
//             Ok(expected_commitment) => {
//                 if commitment_hash == expected_commitment {
//                     // Success - push success indicator (1)
//                     stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();
//                 } else {
//                     // Mismatch - push error indicator (0)
//                     stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
//                 }
//             }
//             Err(_) => {
//                 // Error - push error indicator (0)
//                 stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
//             }
//         }

//         self.processed = true;
//         vec![] // No new tasks to push
//     }

//     fn is_finished(&mut self) -> bool {
//         self.processed
//     }
// }

// /// Recursively compute the root hash from queries and authentication paths
// fn compute_root_from_queries(
//     mut queue: Vec<QueryWithDepth>,
//     start: usize,
//     n_verifier_friendly_layers: Felt,
//     authentications: Vec<Felt>,
//     auth_start: usize,
// ) -> Result<Felt, VectorDecommitError> {
//     let current = queue.get(start).ok_or(VectorDecommitError::IndexInvalid)?;

//     if current.index == Felt::ONE {
//         // root
//         Ok(current.value)
//     } else {
//         let parent = current.index / Felt::TWO;
//         let bit = current.index % Felt::TWO;
//         let is_verifier_friendly = n_verifier_friendly_layers >= current.depth;

//         let hash = if bit == Felt::ZERO {
//             if start + 1 != queue.len() {
//                 let next = queue.get(start + 1).ok_or(VectorDecommitError::IndexInvalid)?;
//                 if current.index + 1 == next.index {
//                     // next is a sibling of current
//                     let hash = hash_friendly_unfriendly(current.value, next.value, is_verifier_friendly);
//                     queue.push(QueryWithDepth {
//                         index: parent,
//                         value: hash,
//                         depth: current.depth - 1,
//                     });
//                     return compute_root_from_queries(
//                         queue,
//                         start + 2,
//                         n_verifier_friendly_layers,
//                         authentications,
//                         auth_start,
//                     );
//                 }
//             }
//             hash_friendly_unfriendly(
//                 current.value,
//                 *authentications.get(auth_start).ok_or(VectorDecommitError::IndexInvalid)?,
//                 is_verifier_friendly,
//             )
//         } else {
//             hash_friendly_unfriendly(
//                 *authentications.get(auth_start).ok_or(VectorDecommitError::IndexInvalid)?,
//                 current.value,
//                 is_verifier_friendly,
//             )
//         };

//         queue.push(QueryWithDepth {
//             index: parent,
//             value: hash,
//             depth: current.depth - 1
//         });

//         compute_root_from_queries(
//             queue,
//             start + 1,
//             n_verifier_friendly_layers,
//             authentications,
//             auth_start + 1,
//         )
//     }
// }

// /// Hash function that switches between Poseidon (verifier-friendly) and Keccak/Blake2s
// fn hash_friendly_unfriendly(x: Felt, y: Felt, is_verifier_friendly: bool) -> Felt {
//     if is_verifier_friendly {
//         // Use Poseidon hash for verifier-friendly layers
//         // Note: This would need to be implemented or imported from a crypto library
//         // For now, using a simple placeholder
//         poseidon_hash_placeholder(x, y)
//     } else {
//         // Use Keccak hash for non-verifier-friendly layers
//         // Note: This would need to be implemented or imported from a crypto library
//         // For now, using a simple placeholder
//         keccak_hash_placeholder(x, y)
//     }
// }

// /// Placeholder for Poseidon hash - needs to be replaced with actual implementation
// fn poseidon_hash_placeholder(x: Felt, y: Felt) -> Felt {
//     // This is a placeholder - in real implementation would use actual Poseidon hash
//     // For now, using a simple combination
//     x + y + Felt::from(1u64)
// }

// /// Placeholder for Keccak hash - needs to be replaced with actual implementation
// fn keccak_hash_placeholder(x: Felt, y: Felt) -> Felt {
//     // This is a placeholder - in real implementation would use actual Keccak hash
//     // For now, using a simple combination
//     x + y + Felt::from(2u64)
// }
