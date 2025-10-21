use felt::Felt;
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, Executable, ProofData,
    TypeIdentifiable,
};

use crate::compute_coset_elements::ComputeCosetElements;
use crate::fri_formula::FriFormula;
use types::swiftness::stark::types::FriVerifyData;

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
    fn execute<T: BidirectionalStack + ProofData + CacheStorage>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.stage {
            ComputeNextLayerStep::Init => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                fri_verify_data.working_indices.flush();
                fri_verify_data.working_elements.flush();
                fri_verify_data.working_y_values.flush();
                fri_verify_data.current_coset_index = 0;

                if fri_verify_data.active_query_count == 0 {
                    self.stage = ComputeNextLayerStep::Done;
                } else {
                    self.stage = ComputeNextLayerStep::ProcessQueries;
                }
                vec![]
            }

            ComputeNextLayerStep::ProcessQueries => {
                let fri_verify_data = stack.borrow_from_cache::<FriVerifyData>();

                if fri_verify_data.active_query_count > 0 {
                    self.stage = ComputeNextLayerStep::ComputeCosetElements;
                } else {
                    self.stage = ComputeNextLayerStep::Done;
                }
                vec![]
            }

            ComputeNextLayerStep::ComputeCosetElements => {
                let query = {
                    let fri_verify_data = stack.borrow_from_cache::<FriVerifyData>();
                    fri_verify_data.get_first_active_query().copied()
                };

                let Some(query) = query else {
                    self.stage = ComputeNextLayerStep::Done;
                    return vec![];
                };

                let coset_start_index = {
                    let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                    let coset_index_felt = Felt::from_bytes_be_slice(
                        (query.index.to_biguint() / fri_verify_data.coset_size.to_biguint())
                            .to_bytes_be()
                            .as_slice(),
                    );

                    fri_verify_data.working_indices.push(coset_index_felt);
                    fri_verify_data.next_x_inv_value =
                        query.x_inv_value.pow_felt(&fri_verify_data.coset_size);
                    coset_index_felt * fri_verify_data.coset_size
                };

                self.stage = ComputeNextLayerStep::WaitForCosetElements;
                vec![ComputeCosetElements::new(coset_start_index).to_vec_with_type_tag()]
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
