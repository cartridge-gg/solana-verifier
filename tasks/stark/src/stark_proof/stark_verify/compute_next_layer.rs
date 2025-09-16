use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use super::{ComputeCosetElements, FriFormula};
use crate::swiftness::stark::types::FriVerifyData;

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

                fri_verify_data.working_indices.flush();
                fri_verify_data.working_y_values.flush();
                fri_verify_data.current_coset_index = 0;

                if fri_verify_data.working_queries.is_empty() {
                    self.stage = ComputeNextLayerStep::Done;
                } else {
                    self.stage = ComputeNextLayerStep::ProcessQueries;
                }
                vec![]
            }
            ComputeNextLayerStep::ProcessQueries => {
                let fri_verify_data = stack.borrow_from_cache::<FriVerifyData>();

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

                if let Some(query) = fri_verify_data.working_queries.get(0) {
                    let query_uint = query.index.to_biguint();
                    let coset_index = query_uint / fri_verify_data.coset_size.to_biguint();
                    let coset_index_felt =
                        Felt::from_bytes_be_slice(coset_index.to_bytes_be().as_slice());
                    fri_verify_data.working_indices.push(coset_index_felt);
                    let _coset_start_index = coset_index_felt * fri_verify_data.coset_size;

                    let next_x_inv = query.x_inv_value.pow_felt(&fri_verify_data.coset_size);
                    fri_verify_data.next_x_inv_value = next_x_inv;

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
