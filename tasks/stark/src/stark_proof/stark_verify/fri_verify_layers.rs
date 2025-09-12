use felt::Felt;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use super::table_decommit::TableDecommit;
use super::ComputeNextLayer;
use crate::swiftness::commitment::vector::types::CommitmentTrait;
use crate::swiftness::fri::types::FriLayerQuery;
use crate::swiftness::stark::types::{
    cast_slice_to_struct, cast_slice_to_struct_mut, FriVerifyData,
};

// Task for verifying FRI layers
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerifyLayers {
    stage: FriVerifyLayersStep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FriVerifyLayersStep {
    Init,
    ProcessLayer,
    PushTableData,
    WaitForTableDecommit,
    NextLayer,
    Done,
}

impl_type_identifiable!(FriVerifyLayers);

impl FriVerifyLayers {
    pub fn new() -> Self {
        Self {
            stage: FriVerifyLayersStep::Init,
        }
    }
}

impl Default for FriVerifyLayers {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriVerifyLayers {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.stage {
            FriVerifyLayersStep::Init => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // Initialize working fields
                fri_verify_data.current_layer = 0;
                fri_verify_data.working_queries.flush();
                fri_verify_data.working_indices.flush();
                fri_verify_data.working_y_values.flush();
                fri_verify_data.working_elements.flush();

                // Copy initial queries to working_queries
                for i in 0..fri_verify_data.queries.len() {
                    if let Some(query_felt) = fri_verify_data.queries.get(i) {
                        // Convert Felt queries to FriLayerQuery (simplified for now)
                        let fri_query = FriLayerQuery {
                            index: *query_felt,
                            y_value: Felt::ZERO,    // Will be set later
                            x_inv_value: Felt::ONE, // Will be set later
                        };
                        fri_verify_data.working_queries.push(fri_query);
                    }
                }

                let n_layers_usize: usize = fri_verify_data
                    .fri_commitment
                    .config
                    .n_layers
                    .to_biguint()
                    .try_into()
                    .unwrap();
                if n_layers_usize == 0 {
                    self.stage = FriVerifyLayersStep::Done;
                } else {
                    self.stage = FriVerifyLayersStep::ProcessLayer;
                }

                // Dane są już zmodyfikowane bezpośrednio na stosie!
                vec![]
            }

            FriVerifyLayersStep::ProcessLayer => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                let n_layers_usize: usize = fri_verify_data
                    .fri_commitment
                    .config
                    .n_layers
                    .to_biguint()
                    .try_into()
                    .unwrap();

                if fri_verify_data.current_layer < n_layers_usize {
                    // Get current layer witness
                    let target_layer_witness = fri_verify_data
                        .witness
                        .layers
                        .get(fri_verify_data.current_layer)
                        .unwrap();

                    // Prepare parameters for compute_next_layer
                    let step_size = fri_verify_data
                        .fri_commitment
                        .config
                        .fri_step_sizes
                        .get(fri_verify_data.current_layer + 1)
                        .unwrap_or(&Felt::ONE);
                    fri_verify_data.coset_size = Felt::TWO.pow_felt(step_size);
                    fri_verify_data.eval_point = *fri_verify_data
                        .fri_commitment
                        .eval_points
                        .get(fri_verify_data.current_layer)
                        .unwrap();

                    // Copy sibling witness to working_elements
                    fri_verify_data.working_elements.flush();
                    for i in 0..target_layer_witness.leaves.len() {
                        if let Some(value) = target_layer_witness.leaves.get(i) {
                            fri_verify_data.working_elements.push(*value);
                        }
                    }

                    // Dane są już zmodyfikowane bezpośrednio na stosie!
                    self.stage = FriVerifyLayersStep::PushTableData;
                    vec![ComputeNextLayer::new().to_vec_with_type_tag()]
                } else {
                    // All layers processed - copy working_queries to final result
                    self.stage = FriVerifyLayersStep::Done;
                    vec![]
                }
            }

            FriVerifyLayersStep::PushTableData => {
                // Pobierz wszystkie potrzebne dane w jednym kroku
                let (current_layer, y_values, indices, table_witness, target_commitment) = {
                    let fri_verify_data = stack.borrow_from_cache::<FriVerifyData>();

                    let current_layer = fri_verify_data.current_layer;

                    // Zbierz y_values
                    let mut y_values = Vec::new();
                    for i in 0..fri_verify_data.working_y_values.len() {
                        if let Some(value) = fri_verify_data.working_y_values.get(i) {
                            y_values.push(*value);
                        }
                    }

                    // Zbierz indices
                    let mut indices = Vec::new();
                    for i in 0..fri_verify_data.working_indices.len() {
                        if let Some(index) = fri_verify_data.working_indices.get(i) {
                            indices.push(*index);
                        }
                    }

                    // Pobierz referencje do struktur (bez klonowania jeszcze)
                    let table_witness = fri_verify_data
                        .witness
                        .layers
                        .get(current_layer)
                        .unwrap()
                        .table_witness;
                    let target_commitment = fri_verify_data
                        .fri_commitment
                        .inner_layers
                        .get(current_layer)
                        .unwrap();

                    (
                        current_layer,
                        y_values,
                        indices,
                        table_witness,
                        *target_commitment,
                    )
                }; // Koniec immutable borrow

                // Teraz pushuj wszystko na stack
                table_witness.push_to_stack(stack);

                stack
                    .push_front(&Felt::from(y_values.len()).to_bytes_be())
                    .unwrap();
                for value in y_values.iter().rev() {
                    stack.push_front(&value.to_bytes_be()).unwrap();
                }

                stack
                    .push_front(&Felt::from(indices.len()).to_bytes_be())
                    .unwrap();
                for index in indices.iter().rev() {
                    stack.push_front(&index.to_bytes_be()).unwrap();
                }

                target_commitment.push_to_stack(stack);

                // Create TableDecommit task
                self.stage = FriVerifyLayersStep::WaitForTableDecommit;
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            FriVerifyLayersStep::WaitForTableDecommit => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // TableDecommit completed successfully - move to next layer
                fri_verify_data.current_layer += 1;
                self.stage = FriVerifyLayersStep::ProcessLayer;

                // Dane są już zmodyfikowane bezpośrednio na stosie!
                vec![]
            }

            FriVerifyLayersStep::NextLayer => {
                // This step is merged with WaitForTableDecommit
                self.stage = FriVerifyLayersStep::ProcessLayer;
                vec![]
            }

            FriVerifyLayersStep::Done => {
                let fri_verify_data = stack.borrow_from_cache::<FriVerifyData>();

                // // Push final queries to stack
                // stack.push_front(&Felt::from(fri_verify_data.working_queries.len()).to_bytes_be()).unwrap();
                // for i in (0..fri_verify_data.working_queries.len()).rev() {
                //     if let Some(query) = fri_verify_data.working_queries.get(i) {
                //         stack.push_front(&query.x_inv_value.to_bytes_be()).unwrap();
                //         stack.push_front(&query.y_value.to_bytes_be()).unwrap();
                //         stack.push_front(&query.index.to_bytes_be()).unwrap();
                //     }
                // }
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == FriVerifyLayersStep::Done
    }
}

impl FriVerifyLayers {
    // Parse final queries from FriVerifyData
    pub fn parse_final_queries_from_data(fri_verify_data: &FriVerifyData) -> Vec<FriLayerQuery> {
        let mut final_queries = Vec::new();
        for i in 0..fri_verify_data.working_queries.len() {
            if let Some(query) = fri_verify_data.working_queries.get(i) {
                final_queries.push(*query);
            }
        }
        final_queries
    }
}
