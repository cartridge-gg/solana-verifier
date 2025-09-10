use felt::Felt;
use sha3::{Digest, Keccak256};
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::poseidon::PoseidonHashMany;
use crate::stark_proof::stark_verify::vector_decommit::VectorDecommit;
use crate::swiftness::commitment::table::types::{Commitment as TableCommitment, Decommitment};
use crate::swiftness::commitment::vector::types::{
    CommitmentTrait, Query, Witness as VectorWitness,
};
use crate::swiftness::stark::types::VerifyVariables;

// TableDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDecommitStep {
    PrepareVectorQueries,
    HashSingleQuery,   // Dla verifier-friendly multi-column
    CollectHashResult, // Zbieranie wyniku z GenerateVectorQueries
    PrepareVectorDecommit,
    ExecuteVectorDecommit,
    Done,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TableDecommit {
    step: TableDecommitStep,
    commitment: TableCommitment,
    n_columns: u32,
    is_bottom_layer_verifier_friendly: bool,
    current_query_index: usize,
    total_queries: usize,
    n_authentications: usize,
    // Tymczasowe przechowywanie vector queries
    vector_queries: Vec<Query>,
}

impl_type_identifiable!(TableDecommit);

impl TableDecommit {
    pub fn new() -> Self {
        Self {
            step: TableDecommitStep::PrepareVectorQueries,
            commitment: TableCommitment::default(),
            n_columns: 0,
            is_bottom_layer_verifier_friendly: false,
            current_query_index: 0,
            total_queries: 0,
            n_authentications: 0,
            vector_queries: Vec::new(),
        }
    }
}

impl Default for TableDecommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for TableDecommit {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            TableDecommitStep::PrepareVectorQueries => {
                // Read table commitment
                let table_commitment = TableCommitment::from_stack(stack);
                println!("Table commitment: {:?}", table_commitment);
                self.commitment = table_commitment;

                // Store commitment config
                self.n_columns = table_commitment
                    .config
                    .n_columns
                    .to_biguint()
                    .try_into()
                    .unwrap();

                // An extra layer is added to the height since the table is considered as a layer
                let bottom_layer_depth = table_commitment.config.vector.height + Felt::ONE;
                self.is_bottom_layer_verifier_friendly = table_commitment
                    .config
                    .vector
                    .n_verifier_friendly_commitment_layers
                    >= bottom_layer_depth;

                // Read queries
                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let queries_count: usize = queries_len.to_biguint().try_into().unwrap();

                // Store queries indices
                let mut query_indices = Vec::new();
                for i in 0..queries_count {
                    // let index = Felt::from_bytes_be_slice(stack.borrow_front());
                    // stack.pop_front();
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.temp_queries;
                    let index = queries_slice[i * 2];
                    println!("DEBUG: index: {:?}", index);
                    query_indices.push(index);
                }

                self.total_queries = queries_count;
                println!("Total queries: {}", self.total_queries);

                // Read decommitment
                Decommitment::from_stack(stack);

                // Validate montgomery values length
                let montgomery_values_len = {
                    let verify_variables: &VerifyVariables = stack.get_verify_variables();
                    verify_variables.montgomery_values.len()
                };

                assert!(
                    self.n_columns as usize * self.total_queries <= montgomery_values_len,
                    "Invalid decommitment length: expected {} values, got {}",
                    self.n_columns as usize * self.total_queries,
                    montgomery_values_len
                );

                // Read witness
                self.n_authentications = VectorWitness::from_stack(stack);

                // Initialize vector queries with indices
                self.vector_queries = query_indices
                    .into_iter()
                    .map(|index| Query {
                        index,
                        value: Felt::ZERO,
                    })
                    .collect();

                self.current_query_index = 0;
                // Decide next step based on configuration
                if self.n_columns > 1 && self.is_bottom_layer_verifier_friendly {
                    // Need to hash each query with Poseidon
                    self.step = TableDecommitStep::HashSingleQuery;
                } else {
                    // Can compute all hashes directly
                    self.compute_all_hashes(stack);
                    self.step = TableDecommitStep::PrepareVectorDecommit;
                }

                vec![]
            }

            TableDecommitStep::HashSingleQuery => {
                if self.current_query_index < self.total_queries {
                    // Push query index for GenerateVectorQueries
                    stack
                        .push_front(&Felt::from(self.current_query_index).to_bytes_be())
                        .unwrap();

                    self.step = TableDecommitStep::CollectHashResult;
                    vec![GenerateVectorQueries::new(
                        self.n_columns,
                        self.is_bottom_layer_verifier_friendly,
                        self.total_queries,
                    )
                    .to_vec_with_type_tag()]
                } else {
                    // All queries hashed, proceed to vector decommit
                    self.step = TableDecommitStep::PrepareVectorDecommit;
                    vec![]
                }
            }

            TableDecommitStep::CollectHashResult => {
                // Get hash result from GenerateVectorQueries
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Store hash in corresponding vector query
                self.vector_queries[self.current_query_index].value = hash;
                self.current_query_index += 1;

                // Move to next query or finish
                self.step = TableDecommitStep::HashSingleQuery;
                vec![]
            }

            TableDecommitStep::PrepareVectorDecommit => {
                VectorWitness::push_to_stack_static(stack, self.n_authentications);
                println!("DEBUG: n_authentications = {}", self.n_authentications);

                // Push all vector queries to stack
                for query in self.vector_queries.iter().rev() {
                    stack.push_front(&query.value.to_bytes_be()).unwrap();
                    stack.push_front(&query.index.to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(self.total_queries).to_bytes_be())
                    .unwrap();
                println!("DEBUG: total_queries = {}", self.total_queries);

                // Push vector commitment
                self.commitment.vector_commitment.push_to_stack(stack);

                self.step = TableDecommitStep::ExecuteVectorDecommit;
                vec![VectorDecommit::new().to_vec_with_type_tag()]
            }

            TableDecommitStep::ExecuteVectorDecommit => {
                // VectorDecommit completed successfully
                self.step = TableDecommitStep::Done;
                vec![]
            }

            TableDecommitStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == TableDecommitStep::Done
    }
}

impl TableDecommit {
    // Helper method to compute all hashes at once (for single column or Keccak cases)
    fn compute_all_hashes<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) {
        let verify_variables: &VerifyVariables = stack.get_verify_variables();
        let montgomery_values = &verify_variables.montgomery_values;

        for i in 0..self.total_queries {
            let hash = if self.n_columns == 1 {
                // Single column: use value directly
                montgomery_values[i]
            } else {
                // Multiple columns with Keccak256
                let slice = &montgomery_values
                    [(i * self.n_columns as usize)..((i + 1) * self.n_columns as usize)];

                let mut hasher = Keccak256::new();
                for &value in slice {
                    hasher.update(value.to_bytes_be());
                }
                Felt::from_bytes_be_slice(&hasher.finalize().as_slice()[12..32])
            };

            self.vector_queries[i].value = hash;
        }
    }
}

// GenerateVectorQueries task for hashing multiple values
#[derive(Debug, Clone)]
#[repr(C)]
pub struct GenerateVectorQueries {
    step: GenerateVectorQueriesStep,
    pub n_columns: u32,
    pub is_verifier_friendly: bool,
    pub queries_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateVectorQueriesStep {
    Init,
    WaitForPoseidonHash,
    Done,
}

impl GenerateVectorQueries {
    pub fn new(n_columns: u32, is_verifier_friendly: bool, queries_count: usize) -> Self {
        Self {
            step: GenerateVectorQueriesStep::Init,
            n_columns,
            is_verifier_friendly,
            queries_count,
        }
    }
}

impl Default for GenerateVectorQueries {
    fn default() -> Self {
        Self::new(0, false, 0)
    }
}

impl_type_identifiable!(GenerateVectorQueries);

impl Executable for GenerateVectorQueries {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            GenerateVectorQueriesStep::Init => {
                // Get current query index
                let current_query_index = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let current_query_index: usize =
                    current_query_index.to_biguint().try_into().unwrap();

                if self.is_verifier_friendly {
                    // Use Poseidon for verifier-friendly hashing
                    let inputs: Vec<Felt> = {
                        let verify_variables: &VerifyVariables = stack.get_verify_variables();
                        verify_variables.montgomery_values[(current_query_index
                            * self.n_columns as usize)
                            ..((current_query_index + 1) * self.n_columns as usize)]
                            .to_vec()
                    };

                    PoseidonHashMany::push_input(&inputs, stack);

                    self.step = GenerateVectorQueriesStep::WaitForPoseidonHash;
                    vec![PoseidonHashMany::new(inputs.len()).to_vec_with_type_tag()]
                } else {
                    // Use Keccak256 for non-verifier-friendly hashing
                    let result = {
                        let verify_variables: &VerifyVariables = stack.get_verify_variables();
                        let values = &verify_variables.montgomery_values;
                        let slice = &values[(current_query_index * self.n_columns as usize)
                            ..((current_query_index + 1) * self.n_columns as usize)];

                        let mut hasher = Keccak256::new();
                        for &value in slice {
                            hasher.update(value.to_bytes_be());
                        }

                        Felt::from_bytes_be_slice(&hasher.finalize().as_slice()[12..32])
                    };

                    // Push result to stack
                    stack.push_front(&result.to_bytes_be()).unwrap();

                    self.step = GenerateVectorQueriesStep::Done;
                    vec![]
                }
            }

            GenerateVectorQueriesStep::WaitForPoseidonHash => {
                // Get result from PoseidonHashMany
                let result = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                // Push result to stack for TableDecommit to collect
                stack.push_front(&result.to_bytes_be()).unwrap();

                self.step = GenerateVectorQueriesStep::Done;
                vec![]
            }

            GenerateVectorQueriesStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == GenerateVectorQueriesStep::Done
    }
}
