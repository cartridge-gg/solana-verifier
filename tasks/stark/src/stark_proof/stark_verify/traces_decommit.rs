// use felt::Felt;
// use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

// use crate::stark_proof::stark_verify::table_decommit::TableDecommit;
// use crate::swiftness::commitment::table::config::Config as TableConfig;
// use crate::swiftness::commitment::table::types::Commitment;
// use crate::swiftness::commitment::vector::config::Config as VectorConfig;
// use crate::swiftness::commitment::vector::types::{
//     Commitment as VectorCommitment, CommitmentTrait, Witness as VectorWitness,
// };

// // TracesDecommit task phases
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum TracesDecommitStep {
//     PrepareOriginalTable,
//     ExecuteOriginalTable,
//     PrepareInteractionTable,
//     ExecuteInteractionTable,
//     Done,
// }

// #[derive(Debug, Clone)]
// #[repr(C)]
// pub struct TracesDecommit {
//     step: TracesDecommitStep,
//     queries_count: usize,
// }

// impl_type_identifiable!(TracesDecommit);

// impl TracesDecommit {
//     pub fn new() -> Self {
//         Self {
//             step: TracesDecommitStep::PrepareOriginalTable,
//             queries_count: 0,
//         }
//     }
// }

// impl Default for TracesDecommit {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Executable for TracesDecommit {
//     fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
//         match self.step {
//             TracesDecommitStep::PrepareOriginalTable => {
//                 // Read queries
//                 let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 self.queries_count = queries_len.to_biguint().try_into().unwrap();

//                 let mut queries = Vec::with_capacity(self.queries_count);
//                 for _ in 0..self.queries_count {
//                     let query = Felt::from_bytes_be_slice(stack.borrow_front());
//                     stack.pop_front();
//                     queries.push(query);
//                 }

//                 // Read original commitment
//                 let original_commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let original_height = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let original_n_verifier_friendly = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let original_n_columns = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();

//                 // Read original decommitment values
//                 let original_values_len = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let mut original_values =
//                     Vec::with_capacity(original_values_len.to_biguint().try_into().unwrap());

//                 for _ in 0..original_values_len.to_biguint().try_into().unwrap() {
//                     let value = Felt::from_bytes_be_slice(stack.borrow_front());
//                     stack.pop_front();
//                     original_values.push(value);
//                 }

//                 // Read original witness authentications using trait method
//                 let original_witness = VectorWitness::from_stack(stack);

//                 // Store interaction data for later (read but keep on stack for later stages)
//                 // We'll need to preserve this data across the original table decommit

//                 // Create table commitment and push using trait method
//                 let vector_config = VectorConfig {
//                     height: original_height,
//                     n_verifier_friendly_commitment_layers: original_n_verifier_friendly,
//                 };
//                 let vector_commitment =
//                     VectorCommitment::new(vector_config, original_commitment_hash);
//                 let table_config = TableConfig {
//                     n_columns: original_n_columns,
//                     vector: vector_config,
//                 };
//                 let table_commitment = Commitment::new(table_config, vector_commitment);

//                 // Push table commitment using trait method
//                 table_commitment.push_to_stack(stack);

//                 stack.push_front(&queries_len.to_bytes_be()).unwrap();
//                 for query in queries.iter() {
//                     stack.push_front(&query.to_bytes_be()).unwrap();
//                 }

//                 stack
//                     .push_front(&original_values_len.to_bytes_be())
//                     .unwrap();
//                 for value in original_values.iter() {
//                     stack.push_front(&value.to_bytes_be()).unwrap();
//                 }

//                 // Push original witness using trait method
//                 original_witness.push_to_stack(stack);

//                 self.step = TracesDecommitStep::ExecuteOriginalTable;
//                 vec![TableDecommit::new().to_vec_with_type_tag()]
//             }

//             TracesDecommitStep::ExecuteOriginalTable => {
//                 // Original table decommit completed
//                 // Now prepare interaction table decommit
//                 self.step = TracesDecommitStep::PrepareInteractionTable;
//                 vec![]
//             }

//             TracesDecommitStep::PrepareInteractionTable => {
//                 // Read queries again (they should still be available)
//                 let queries_len = Felt::from(self.queries_count);
//                 let mut queries = Vec::with_capacity(self.queries_count);

//                 // Note: In real implementation, queries would be preserved from earlier
//                 // or re-read from appropriate position in stack

//                 // Read interaction commitment
//                 let interaction_commitment_hash = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let interaction_height = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let interaction_n_verifier_friendly =
//                     Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let interaction_n_columns = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();

//                 // Read interaction decommitment values
//                 let interaction_values_len = Felt::from_bytes_be_slice(stack.borrow_front());
//                 stack.pop_front();
//                 let mut interaction_values =
//                     Vec::with_capacity(interaction_values_len.to_biguint().try_into().unwrap());

//                 for _ in 0..interaction_values_len.to_biguint().try_into().unwrap() {
//                     let value = Felt::from_bytes_be_slice(stack.borrow_front());
//                     stack.pop_front();
//                     interaction_values.push(value);
//                 }

//                 // Read interaction witness authentications using trait method
//                 let interaction_witness = VectorWitness::from_stack(stack);

//                 // Re-read queries for interaction table
//                 for _ in 0..self.queries_count {
//                     let query = Felt::from_bytes_be_slice(stack.borrow_front());
//                     stack.pop_front();
//                     queries.push(query);
//                 }

//                 // Create interaction table commitment and push using trait method
//                 let interaction_vector_config = VectorConfig {
//                     height: interaction_height,
//                     n_verifier_friendly_commitment_layers: interaction_n_verifier_friendly,
//                 };
//                 let interaction_vector_commitment =
//                     VectorCommitment::new(interaction_vector_config, interaction_commitment_hash);
//                 let interaction_table_config = TableConfig {
//                     n_columns: interaction_n_columns,
//                     vector: interaction_vector_config,
//                 };
//                 let interaction_table_commitment =
//                     Commitment::new(interaction_table_config, interaction_vector_commitment);

//                 // Push interaction table commitment using trait method
//                 interaction_table_commitment.push_to_stack(stack);

//                 stack.push_front(&queries_len.to_bytes_be()).unwrap();
//                 for query in queries.iter() {
//                     stack.push_front(&query.to_bytes_be()).unwrap();
//                 }

//                 stack
//                     .push_front(&interaction_values_len.to_bytes_be())
//                     .unwrap();
//                 for value in interaction_values.iter() {
//                     stack.push_front(&value.to_bytes_be()).unwrap();
//                 }

//                 // Push interaction witness using trait method
//                 interaction_witness.push_to_stack(stack);

//                 self.step = TracesDecommitStep::ExecuteInteractionTable;
//                 vec![TableDecommit::new().to_vec_with_type_tag()]
//             }

//             TracesDecommitStep::ExecuteInteractionTable => {
//                 // Both table decommits completed successfully
//                 self.step = TracesDecommitStep::Done;
//                 vec![]
//             }

//             TracesDecommitStep::Done => {
//                 vec![]
//             }
//         }
//     }

//     fn is_finished(&mut self) -> bool {
//         self.step == TracesDecommitStep::Done
//     }
// }
