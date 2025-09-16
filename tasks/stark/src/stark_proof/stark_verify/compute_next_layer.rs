use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use super::{ComputeCosetElements, FriFormula};
use crate::swiftness::fri::types::FriLayerQuery;
use crate::swiftness::stark::types::{
    cast_slice_to_struct, cast_slice_to_struct_mut, FriVerifyData,
};

// Note: FriLayerComputationParams removed - data now stored in FriVerifyData

// Task for computing FRI next layer
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeNextLayer {
    stage: ComputeNextLayerStep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum ComputeNextLayerStep {
    Init,
    ProcessQueries,
    ComputeCosetElements,
    WaitForCosetElements,
    Done,
}

impl_type_identifiable!(ComputeNextLayer);

impl ComputeNextLayer {
    pub fn new() -> Self {
        Self {
            stage: ComputeNextLayerStep::Init,
        }
    }
}

impl Default for ComputeNextLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ComputeNextLayer {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            ComputeNextLayerStep::Init => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // Initialize computation (like original function start)
                fri_verify_data.working_indices.flush();
                fri_verify_data.working_y_values.flush(); // Clear at start, then extend for each coset
                // fri_verify_data.next_queries.flush(); // Clear next_queries for fresh start
                fri_verify_data.current_coset_index = 0; // Start with first coset

                if fri_verify_data.working_queries.is_empty() {
                    self.stage = ComputeNextLayerStep::Done;
                } else {
                    self.stage = ComputeNextLayerStep::ProcessQueries;
                }
                vec![]
            }
            ComputeNextLayerStep::ProcessQueries => {
                let fri_verify_data = stack.borrow_from_cache::<FriVerifyData>();

                // Check if there are still queries to process (like original: while !queries.is_empty())
                if !fri_verify_data.working_queries.is_empty() {
                    self.stage = ComputeNextLayerStep::ComputeCosetElements;
                } else {
                    // All queries processed
                    self.stage = ComputeNextLayerStep::Done;
                }
                vec![]
            }
            ComputeNextLayerStep::ComputeCosetElements => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // Get first query and compute coset information (like original: queries.first().unwrap())
                if let Some(query) = fri_verify_data.working_queries.get(0) {
                    let query_uint = query.index.to_biguint();
                    let coset_index = query_uint / fri_verify_data.coset_size.to_biguint();
                    let coset_index_felt =
                        Felt::from_bytes_be_slice(coset_index.to_bytes_be().as_slice());
                    fri_verify_data.working_indices.push(coset_index_felt);
                    let _coset_start_index = coset_index_felt * fri_verify_data.coset_size;

                    // Store x_inv_value for next_queries calculation
                    let next_x_inv = query.x_inv_value.pow_felt(&fri_verify_data.coset_size);
                    fri_verify_data.next_x_inv_value = next_x_inv;

                    // Create and launch ComputeCosetElements task with coset_start_index
                    let coset_start_index = coset_index_felt * fri_verify_data.coset_size;
                    self.stage = ComputeNextLayerStep::WaitForCosetElements;
                    vec![
                        ComputeCosetElements::with_coset_start_index(coset_start_index)
                            .to_vec_with_type_tag(),
                    ]
                } else {
                    self.stage = ComputeNextLayerStep::Done;
                    vec![]
                }
            }
            ComputeNextLayerStep::WaitForCosetElements => {
                // ComputeCosetElements finished - results are already in working_y_values and working_elements
                // next_x_inv_value was already calculated in ComputeCosetElements step

                // Create FriFormula sub-task
                self.stage = ComputeNextLayerStep::ProcessQueries;
                vec![FriFormula::new().to_vec_with_type_tag()]
            }
            ComputeNextLayerStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == ComputeNextLayerStep::Done
    }
}
