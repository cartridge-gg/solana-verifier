use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::stark_proof::stark_verify::vector_decommit::VectorDecommit;
use crate::swiftness::commitment::table::types::Commitment as TableCommitment;
use crate::swiftness::commitment::vector::types::{
    CommitmentTrait, Query, Witness as VectorWitness,
};

const MONTGOMERY_R: Felt =
    Felt::from_hex_unchecked("0x7FFFFFFFFFFFDF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE1");

// TableDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableDecommitStep {
    PrepareVectorQueries,
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
}

impl_type_identifiable!(TableDecommit);

impl TableDecommit {
    pub fn new() -> Self {
        Self {
            step: TableDecommitStep::PrepareVectorQueries,
            commitment: TableCommitment::default(),
            n_columns: 0,
            is_bottom_layer_verifier_friendly: false,
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
                let mut queries = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());
                for _ in 0..queries_len.to_biguint().try_into().unwrap() {
                    let query = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    queries.push(query);
                }

                // Read decommitment values
                let values_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let mut montgomery_values =
                    Vec::with_capacity(values_len.to_biguint().try_into().unwrap());

                for _ in 0..values_len.to_biguint().try_into().unwrap() {
                    let value = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    montgomery_values.push(value * MONTGOMERY_R);
                }

                // Validate decommitment length
                assert!(
                    self.n_columns as usize * queries.len() == montgomery_values.len(),
                    "Invalid decommitment length"
                );

                // Generate vector queries
                let vector_queries = self.generate_vector_queries(&queries, &montgomery_values);

                // Read witness (authentications) using trait method
                let witness = VectorWitness::from_stack(stack);

                // Push data for VectorDecommit using trait methods
                // Push vector commitment using its push_to_stack method
                table_commitment.vector_commitment.push_to_stack(stack);

                // Use Query trait method to push queries
                Query::push_queries_to_stack(&vector_queries, stack);

                // Push witness using trait method
                witness.push_to_stack(stack);

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
    fn generate_vector_queries(&self, queries: &[Felt], values: &[Felt]) -> Vec<Query> {
        let mut vector_queries = Vec::new();

        for i in 0..queries.len() {
            let hash = if self.n_columns == 1 {
                values[i]
            } else if self.is_bottom_layer_verifier_friendly {
                // Use Poseidon hash for verifier-friendly
                let slice =
                    &values[(i * self.n_columns as usize)..((i + 1) * self.n_columns as usize)];
                self.poseidon_hash_many(slice)
            } else {
                // Use Keccak/Blake2s for non-verifier-friendly
                let slice =
                    &values[(i * self.n_columns as usize)..((i + 1) * self.n_columns as usize)];
                self.hash_non_verifier_friendly(slice)
            };

            vector_queries.push(Query {
                index: queries[i],
                value: hash,
            });
        }

        vector_queries
    }

    fn poseidon_hash_many(&self, values: &[Felt]) -> Felt {
        // Simplified Poseidon hash implementation
        // In real implementation, this would call the actual Poseidon hash function
        let mut result = Felt::ZERO;
        for value in values {
            result = result + value;
        }
        result
    }

    fn hash_non_verifier_friendly(&self, values: &[Felt]) -> Felt {
        // Simplified hash implementation for Keccak/Blake2s
        // In real implementation, this would use the actual hash function based on feature flags
        let mut result = Felt::ZERO;
        for value in values {
            result = result + value;
        }
        result
    }
}
