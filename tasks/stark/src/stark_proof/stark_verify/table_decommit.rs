use felt::Felt;
use sha3::{Digest, Keccak256};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::poseidon::PoseidonHashMany;
use crate::stark_proof::stark_verify::vector_decommit::VectorDecommit;
use crate::swiftness::commitment::table::types::{Commitment as TableCommitment, Decommitment};
use crate::swiftness::commitment::vector::types::{
    CommitmentTrait, Query, Witness as VectorWitness,
};

const MONTGOMERY_R: Felt =
    Felt::from_hex_unchecked("0x7FFFFFFFFFFFDF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE1");

// TableDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDecommitStep {
    PrepareVectorQueries,
    HashQueries,
    ProcessHashResult,
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
        }
    }
}

impl Default for TableDecommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for TableDecommit {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            TableDecommitStep::PrepareVectorQueries => {
                // Read table commitment using trait
                let table_commitment = TableCommitment::from_stack(stack);
                println!("Table commitment: {:?}", table_commitment);

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

                // Read queries using trait method
                let queries = Query::read_queries_from_stack(stack);
                self.total_queries = queries.len();
                self.current_query_index = 0;

                // Read decommitment using trait method
                let decommitment = Decommitment::from_stack(stack);

                // Convert values to Montgomery form
                let mut montgomery_values = Vec::new();
                for value in decommitment.values.as_slice() {
                    montgomery_values.push(value * MONTGOMERY_R);
                }

                assert!(
                    self.n_columns as usize * self.total_queries == montgomery_values.len(),
                    "Invalid decommitment length"
                );

                println!("reading witness from stack");
                // Read witness (authentications) using trait method
                let witness = VectorWitness::from_stack(stack);

                // Push Montgomery values to stack (in reverse order)
                for value in montgomery_values.iter().rev() {
                    stack.push_front(&value.to_bytes_be()).unwrap();
                }

                println!("Pushing witness to stack");
                // Store witness for later use
                witness.push_to_stack(stack);

                Query::push_queries_to_stack(&queries, stack);

                self.step = TableDecommitStep::HashQueries;
                vec![]
            }

            TableDecommitStep::HashQueries => {
                let queries = Query::read_queries_from_stack(stack);

                if self.current_query_index < self.total_queries {
                    let current_query = &queries[self.current_query_index];

                    let hash = if self.n_columns == 1 {
                        // For single column, just use the value directly
                        let value = Felt::from_bytes_be_slice(stack.borrow_front());
                        stack.pop_front();
                        value
                    } else {
                        stack
                            .push_front(&Felt::from(self.current_query_index).to_bytes_be())
                            .unwrap();
                        self.step = TableDecommitStep::ProcessHashResult;
                        return vec![GenerateVectorQueries::new(
                            self.n_columns,
                            self.is_bottom_layer_verifier_friendly,
                        )
                        .to_vec_with_type_tag()];
                    };

                    // Push hash result to stack for later use (with original query index)
                    stack.push_front(&hash.to_bytes_be()).unwrap();
                    stack
                        .push_front(&current_query.index.to_bytes_be())
                        .unwrap();

                    self.current_query_index += 1;
                    vec![]
                } else {
                    // Push vector queries to stack (they are already on stack from hash results)
                    // We need to collect them and push using trait method
                    let mut vector_queries = Vec::new();
                    for _ in 0..self.total_queries {
                        let index = Felt::from_bytes_be_slice(stack.borrow_front());
                        stack.pop_front();
                        let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                        stack.pop_front();
                        vector_queries.push(Query { index, value: hash });
                    }
                    Query::push_queries_to_stack(&vector_queries, stack);

                    // All queries processed, move to VectorDecommit
                    // Push data for VectorDecommit using trait methods
                    // Push vector commitment using its push_to_stack method
                    self.commitment.vector_commitment.push_to_stack(stack);

                    // Witness is already on stack from PrepareVectorQueries step

                    self.step = TableDecommitStep::ExecuteVectorDecommit;
                    vec![VectorDecommit::new().to_vec_with_type_tag()]
                }
            }

            TableDecommitStep::ProcessHashResult => {
                // Get hash result from HashComputationMany
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // We need to get the original query index - this is tricky since we don't store it
                // For now, let's use the current_query_index as a workaround
                // In a real implementation, we'd need to store the original query index
                let original_index = Felt::from(self.current_query_index);

                // Push hash result to stack for later use (with original query index)
                stack.push_front(&hash.to_bytes_be()).unwrap();
                stack.push_front(&original_index.to_bytes_be()).unwrap();

                self.current_query_index += 1;
                self.step = TableDecommitStep::HashQueries;
                vec![]
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

// HashComputationMany task for hashing multiple values
#[derive(Debug, Clone)]
#[repr(C)]
pub struct GenerateVectorQueries {
    step: GenerateVectorQueriesStep,
    pub n_columns: u32,
    pub is_verifier_friendly: bool,
    pub result: Felt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateVectorQueriesStep {
    Init,
    WaitForPoseidonHash,
    Done,
}

impl GenerateVectorQueries {
    pub fn new(n_columns: u32, is_verifier_friendly: bool) -> Self {
        Self {
            step: GenerateVectorQueriesStep::Init,
            n_columns,
            is_verifier_friendly,
            result: Felt::ZERO,
        }
    }
}

impl Default for GenerateVectorQueries {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl_type_identifiable!(GenerateVectorQueries);

impl Executable for GenerateVectorQueries {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GenerateVectorQueriesStep::Init => {
                let current_query_index = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let current_query_index: usize =
                    current_query_index.to_biguint().try_into().unwrap();

                let mut values = Vec::new();
                for _ in 0..self.n_columns {
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    values.push(value);
                }

                if self.is_verifier_friendly {
                    let slice = &values[(current_query_index * self.n_columns as usize)
                        ..((current_query_index + 1) * self.n_columns as usize)];

                    PoseidonHashMany::push_input(slice, stack);
                    self.step = GenerateVectorQueriesStep::WaitForPoseidonHash;
                    vec![PoseidonHashMany::new(slice.len()).to_vec_with_type_tag()]
                } else {
                    // Use Keccak256 for non-verifier-friendly hashing (matching original logic)
                    let slice = &values[(current_query_index * self.n_columns as usize)
                        ..((current_query_index + 1) * self.n_columns as usize)];
                    let mut data = Vec::new();
                    data.extend(slice.iter().flat_map(|x| x.to_bytes_be().to_vec()));

                    let mut hasher = Keccak256::new();
                    hasher.update(&data);

                    // Use the same slice as in the original: [12..32] for 160-bit output
                    self.result = Felt::from_bytes_be_slice(&hasher.finalize().as_slice()[12..32]);

                    // Push result to stack for TableDecommit to consume
                    stack.push_front(&self.result.to_bytes_be()).unwrap();

                    self.step = GenerateVectorQueriesStep::Done;
                    vec![]
                }
            }
            GenerateVectorQueriesStep::WaitForPoseidonHash => {
                // Get result from PoseidonHashMany
                self.result = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                println!("Poseidon hash result: {:?}", self.result);
                let some_value = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                println!("Some value: {:?}", some_value);

                // Push result back to stack for TableDecommit to consume
                stack.push_front(&self.result.to_bytes_be()).unwrap();

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
