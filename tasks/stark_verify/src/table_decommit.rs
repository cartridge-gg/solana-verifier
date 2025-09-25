use felt::Felt;
use sha3::{Digest, Keccak256};
use types::funvec::FUNVEC_AUTHENTICATIONS;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::vector_decommit::VectorDecommit;
use poseidon::poseidon::PoseidonHashMany;
use types::swiftness::commitment::table::types::Commitment as TableCommitment;
use types::swiftness::commitment::vector::types::Commitment as VectorCommitment;
use types::swiftness::stark::types::{cast_slice_to_struct, cast_struct_to_slice, VerifyVariables};
pub const MONTGOMERY_R: Felt =
    Felt::from_hex_unchecked("0x7FFFFFFFFFFFDF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE1");

// Batch size for processing authentications to avoid transaction size limits
const BATCH_SIZE: usize = 50;

// TableDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDecommitStep {
    ReadCommitmentAndQueries, // commitment + queries count
    ProcessDecommitment,      // decommitment_from_stack()
    InitProcessWitness,       // read n_authentications and prepare batching
    ProcessWitnessBatch,      // process authentications in batches
    CopyQueriesToVerifyVars,  // pętla kopiowania queries
    ComputeHashes,            // compute_all_hashes() lub przygotowanie do HashSingleQuery
    HashSingleQuery,
    CollectHashResult,
    PrepareVectorDecommit,
    ExecuteVectorDecommit,
    Done,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TableDecommit {
    step: TableDecommitStep,
    commitment: VectorCommitment,
    n_columns: u32,
    is_bottom_layer_verifier_friendly: bool,
    current_query_index: usize,
    total_queries: usize,
    n_authentications: usize,
    decommitment_values_count: usize,
    current_auth_index: usize, // Track current authentication being processed
}

impl_type_identifiable!(TableDecommit);

impl TableDecommit {
    pub fn new() -> Self {
        Self {
            step: TableDecommitStep::ReadCommitmentAndQueries,
            commitment: VectorCommitment::default(),
            n_columns: 0,
            is_bottom_layer_verifier_friendly: false,
            current_query_index: 0,
            total_queries: 0,
            n_authentications: 0,
            decommitment_values_count: 0,
            current_auth_index: 0,
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
            TableDecommitStep::ReadCommitmentAndQueries => {
                // Read table commitment
                let table_commitment = commitment_from_stack(stack);
                println!("Table commitment: {:?}", table_commitment);
                self.commitment = table_commitment.vector_commitment;

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

                // Read queries count
                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let queries_count: usize = queries_len.to_biguint().try_into().unwrap();

                self.total_queries = queries_count;
                println!("Total queries: {}", self.total_queries);

                self.step = TableDecommitStep::ProcessDecommitment;
                vec![]
            }

            TableDecommitStep::ProcessDecommitment => {
                // Read decommitment values
                let values_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.decommitment_values_count = values_len.to_biguint().try_into().unwrap();

                // Process decommitment values and convert to Montgomery form
                for i in 0..self.decommitment_values_count {
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    verify_variables.decommitment_values[i] = value;
                    // This convertion is not necessary as we do it in transform.rs and we have montommery values in proof field
                    verify_variables.montgomery_values[i] = value * MONTGOMERY_R;
                }

                self.step = TableDecommitStep::InitProcessWitness;
                vec![]
            }

            TableDecommitStep::InitProcessWitness => {
                let n_authentications = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.n_authentications = n_authentications.try_into().unwrap();
                assert!(
                    self.n_authentications <= FUNVEC_AUTHENTICATIONS,
                    "Too many authentications: {} > {}",
                    self.n_authentications,
                    FUNVEC_AUTHENTICATIONS
                );

                // Initialize batch processing
                self.current_auth_index = 0;
                self.step = TableDecommitStep::ProcessWitnessBatch;
                vec![]
            }

            TableDecommitStep::ProcessWitnessBatch => {
                let remaining_auths = self.n_authentications - self.current_auth_index;
                let batch_size = std::cmp::min(BATCH_SIZE, remaining_auths);

                for i in 0..batch_size {
                    let auth = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();

                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    verify_variables.authentications[self.current_auth_index + i] = auth;
                }

                self.current_auth_index += batch_size;

                // Check if we've processed all authentications
                if self.current_auth_index >= self.n_authentications {
                    self.step = TableDecommitStep::CopyQueriesToVerifyVars;
                }
                // If not, stay in ProcessWitnessBatch for next transaction

                vec![]
            }

            TableDecommitStep::CopyQueriesToVerifyVars => {
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

                // Copy queries to verify variables
                {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &verify_variables.temp_queries;

                    for i in 0..self.total_queries {
                        let index = queries_slice[i * 2];
                        println!("DEBUG: index: {:?}", index);

                        verify_variables.queries[i * 2] = index;
                        verify_variables.queries[i * 2 + 1] = Felt::ZERO;
                    }
                }

                self.step = TableDecommitStep::ComputeHashes;
                vec![]
            }

            TableDecommitStep::ComputeHashes => {
                self.current_query_index = 0;
                println!("TableDecommitStep::ComputeHashes step");

                // Decide next step based on configuration
                if self.n_columns > 1 && self.is_bottom_layer_verifier_friendly {
                    // Need to hash each query with Poseidon (one by one)
                    self.step = TableDecommitStep::HashSingleQuery;
                } else {
                    // Can compute all hashes directly in this step
                    self.compute_all_hashes(stack);
                    self.step = TableDecommitStep::PrepareVectorDecommit;
                }

                vec![]
            }

            TableDecommitStep::HashSingleQuery => {
                println!("TableDecommitStep::HashSingleQuery step");
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
                println!("TableDecommitStep::CollectHashResult step");
                // Get hash result from GenerateVectorQueries
                let hash = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    verify_variables.queries[self.current_query_index * 2 + 1] = hash;
                }
                self.current_query_index += 1;

                // Move to next query or finish
                self.step = TableDecommitStep::HashSingleQuery;
                vec![]
            }

            TableDecommitStep::PrepareVectorDecommit => {
                println!("TableDecommitStep::PrepareVectorDecommit step");
                witness_push_to_stack_static(stack, self.n_authentications);
                {
                    for i in (0..self.total_queries).rev() {
                        let (query_index, query_value) = {
                            let verify_variables: &VerifyVariables = stack.get_verify_variables();
                            (
                                verify_variables.queries[i * 2],
                                verify_variables.queries[i * 2 + 1],
                            )
                        };
                        stack.push_front(&query_value.to_bytes_be()).unwrap();
                        stack.push_front(&query_index.to_bytes_be()).unwrap();
                    }
                }

                stack
                    .push_front(&Felt::from(self.total_queries).to_bytes_be())
                    .unwrap();
                println!("DEBUG: total_queries = {}", self.total_queries);

                // Push vector commitment
                commitment_push_to_stack(&self.commitment, stack);
                self.step = TableDecommitStep::ExecuteVectorDecommit;
                vec![VectorDecommit::new().to_vec_with_type_tag()]
            }

            TableDecommitStep::ExecuteVectorDecommit => {
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
    #[inline(always)]
    fn compute_all_hashes<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) {
        let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
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

            // self.vector_queries[i].value = hash;
            verify_variables.queries[i * 2 + 1] = hash;
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
                println!("GenerateVectorQueriesStep::Init step");
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
                    println!("GenerateVectorQueriesStep::Init step if verifier_friendly");
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
                println!("GenerateVectorQueriesStep::WaitForPoseidonHash step");
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

#[inline(always)]
pub fn witness_push_to_stack_static<T: BidirectionalStack + StarkVerifyTrait>(
    stack: &mut T,
    count: usize,
) {
    // Push authentications in reverse order (for stack) - no allocation
    for i in (0..count).rev() {
        let auth_bytes = {
            let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
            verify_variables.authentications[i].to_bytes_be()
        };
        stack.push_front(&auth_bytes).unwrap();
    }
    stack.push_front(&Felt::from(count).to_bytes_be()).unwrap();
}

#[inline(always)]
fn commitment_push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(
    commitment: &VectorCommitment,
    stack: &mut T,
) {
    let commitment_bytes = cast_struct_to_slice(commitment);
    stack.push_front(commitment_bytes).unwrap();
}

#[inline(always)]
fn commitment_from_stack<T: BidirectionalStack + StarkVerifyTrait>(
    stack: &mut T,
) -> TableCommitment {
    let data = stack.borrow_front();
    let commitment_ref = cast_slice_to_struct::<TableCommitment>(data);
    let commitment = *commitment_ref; // Copy only when needed
    stack.pop_front();
    commitment
}
