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
    current_query_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum ComputeNextLayerStep {
    Init,
    ProcessQueries,
    ComputeCosetElements,
    WaitForCosetElements,
    ApplyFriFormula,
    Done,
}

impl_type_identifiable!(ComputeNextLayer);

impl ComputeNextLayer {
    pub fn new() -> Self {
        Self {
            stage: ComputeNextLayerStep::Init,
            current_query_index: 0,
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

                // Initialize computation
                self.current_query_index = 0;
                fri_verify_data.working_indices.flush();
                fri_verify_data.working_y_values.flush();

                if fri_verify_data.working_queries.is_empty() {
                    self.stage = ComputeNextLayerStep::Done;
                } else {
                    self.stage = ComputeNextLayerStep::ProcessQueries;
                }

                // Dane są już zmodyfikowane bezpośrednio na stosie!
                vec![]
            }
            ComputeNextLayerStep::ProcessQueries => {
                let fri_verify_data = stack.borrow_from_cache::<FriVerifyData>();

                if self.current_query_index < fri_verify_data.working_queries.len() {
                    self.stage = ComputeNextLayerStep::ComputeCosetElements;
                } else {
                    // All queries processed
                    self.stage = ComputeNextLayerStep::Done;
                }
                vec![]
            }
            ComputeNextLayerStep::ComputeCosetElements => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // Get current query and compute coset information
                if let Some(query) = fri_verify_data
                    .working_queries
                    .get(self.current_query_index)
                {
                    let query_uint = query.index.to_biguint();
                    let coset_index = query_uint / fri_verify_data.coset_size.to_biguint();
                    let coset_index_felt =
                        Felt::from_bytes_be_slice(coset_index.to_bytes_be().as_slice());
                    let coset_start_index = coset_index_felt * fri_verify_data.coset_size;

                    // Store coset information for ComputeCosetElements
                    // ComputeCosetElements will work directly on FriVerifyData

                    // Dane są już zmodyfikowane bezpośrednio na stosie!

                    // Create and launch ComputeCosetElements task
                    self.stage = ComputeNextLayerStep::WaitForCosetElements;
                    vec![ComputeCosetElements::new().to_vec_with_type_tag()]
                } else {
                    self.stage = ComputeNextLayerStep::Done;
                    vec![]
                }
            }
            ComputeNextLayerStep::WaitForCosetElements => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // ComputeCosetElements finished - results are in working_elements
                // Copy elements to working_y_values for verification
                for i in 0..fri_verify_data.working_elements.len() {
                    if let Some(element) = fri_verify_data.working_elements.get(i) {
                        fri_verify_data.working_y_values.push(*element);
                    }
                }

                // Dane są już zmodyfikowane bezpośrednio na stosie!

                // Create FriFormula sub-task
                self.stage = ComputeNextLayerStep::ApplyFriFormula;
                vec![FriFormula::new().to_vec_with_type_tag()]
            }
            ComputeNextLayerStep::ApplyFriFormula => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // // FriFormula completed - get result from stack
                // let fri_formula_result = Felt::from_bytes_be_slice(stack.borrow_front());
                // stack.pop_front();

                self.current_query_index += 1;

                // Get current query and compute next query
                if let Some(query) = fri_verify_data
                    .working_queries
                    .get(self.current_query_index - 1)
                {
                    let query_uint = query.index.to_biguint();
                    let coset_index = query_uint / fri_verify_data.coset_size.to_biguint();
                    let coset_index_felt =
                        Felt::from_bytes_be_slice(coset_index.to_bytes_be().as_slice());

                    // Calculate next x_inv
                    let next_x_inv = query.x_inv_value.pow_felt(&fri_verify_data.coset_size);

                    // Update working_queries with next layer query
                    let next_query = FriLayerQuery {
                        index: coset_index_felt,
                        y_value: *fri_verify_data
                            .working_y_values
                            .get(self.current_query_index - 1)
                            .unwrap(),
                        x_inv_value: next_x_inv,
                    };

                    // Replace current query with next layer query
                    *fri_verify_data
                        .working_queries
                        .at_mut(self.current_query_index - 1) = next_query;
                }

                self.stage = ComputeNextLayerStep::ProcessQueries;
                vec![]
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
