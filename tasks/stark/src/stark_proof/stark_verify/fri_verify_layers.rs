use felt::Felt;
use utils::global_values::InteractionElements;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use super::table_decommit::TableDecommit;
use super::ComputeNextLayer;
use crate::swiftness::commitment::vector::types::CommitmentTrait;
use crate::swiftness::stark::types::{FriVerifyData, StarkCommitment, StarkProof, VerifyVariables};

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
                // let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                let (stark_commitment, _, fri_verify_data) = stack.get_stark_commitment_proof_and_cache_mut::<
                StarkCommitment<InteractionElements>, 
                StarkProof, 
                FriVerifyData
            >();
                let fri_commitment = &stark_commitment.fri;
                fri_verify_data.current_layer = 0;
                let n_layers_usize: usize = fri_commitment
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
                vec![]
            }

            FriVerifyLayersStep::ProcessLayer => {
                // Use the new extended API to get all data in one call, avoiding borrowing conflicts
                let (stark_commitment, proof, fri_verify_data) = stack.get_stark_commitment_proof_and_cache_mut::<
                    StarkCommitment<InteractionElements>, 
                    StarkProof, 
                    FriVerifyData
                >();
                
                let fri_commitment = &stark_commitment.fri;
                let fri_witness = &proof.witness.fri_witness;
                // println!("fri_witness: {:?}", fri_witness);

                let n_layers_usize: usize = fri_commitment
                    .config
                    .n_layers
                    .to_biguint()
                    .try_into()
                    .unwrap();

                // FriVerifyLayers processes n_layers - 1 layers (like original fri_verify_layers)
                // Last layer is handled by VerifyLastLayer
                if fri_verify_data.current_layer < n_layers_usize - 1 {
                    // Get current layer witness
                    let target_layer_witness = fri_witness
                        .layers
                        .get(fri_verify_data.current_layer)
                        .unwrap();
                    println!("target_layer_witness: {:?}", target_layer_witness);

                    // Prepare parameters for compute_next_layer
                    let step_size = fri_commitment
                        .config
                        .fri_step_sizes
                        .get(fri_verify_data.current_layer + 1)
                        .unwrap();
                    
                    fri_verify_data.coset_size = Felt::TWO.pow_felt(step_size);
                    fri_verify_data.eval_point = *fri_commitment
                        .eval_points
                        .get(fri_verify_data.current_layer)
                        .unwrap();

                    println!("target_layer_witness.leaves_len: {:?}", target_layer_witness.leaves.len());
                    for i in 0..target_layer_witness.leaves.len() {
                        if let Some(value) = target_layer_witness.leaves.get(i) {
                            fri_verify_data.sibling_witness.push(*value);
                        }
                    }
                    self.stage = FriVerifyLayersStep::PushTableData;
                    vec![ComputeNextLayer::new().to_vec_with_type_tag()]
                } else {
                    // All layers processed - copy working_queries to final result
                    self.stage = FriVerifyLayersStep::Done;
                    vec![]
                }
            }

            FriVerifyLayersStep::PushTableData => {
                let (y_values, indices, table_witness, target_commitment) = {
                    // Use the new extended API to get all data in one call
                    let (stark_commitment, proof, fri_verify_data) = stack.get_stark_commitment_proof_and_cache::<
                        StarkCommitment<InteractionElements>, 
                        StarkProof, 
                        FriVerifyData
                    >();
                    let fri_commitment = &stark_commitment.fri;
                    let fri_witness = &proof.witness.fri_witness;
                    let current_layer = fri_verify_data.current_layer;

                    let mut y_values = Vec::new();
                    for i in 0..fri_verify_data.working_y_values.len() {
                        if let Some(value) = fri_verify_data.working_y_values.get(i) {
                            y_values.push(*value);
                        }
                    }

                    let mut indices = Vec::new();
                    for i in 0..fri_verify_data.working_indices.len() {
                        if let Some(index) = fri_verify_data.working_indices.get(i) {
                            indices.push(*index);
                        }
                    }

                    let table_witness = fri_witness
                        .layers
                        .get(current_layer)
                        .unwrap()
                        .table_witness;
                    let target_commitment = fri_commitment
                        .inner_layers
                        .get(current_layer)
                        .unwrap();

                    (y_values, indices, table_witness, *target_commitment)
                };

                for i in (0..table_witness.vector.authentications.len()).rev() {
                    stack
                        .push_front(
                            &table_witness.vector.authentications.as_slice()[i].to_bytes_be(),
                        )
                        .unwrap();
                }
                stack
                    .push_front(
                        &Felt::from(table_witness.vector.authentications.len()).to_bytes_be(),
                    )
                    .unwrap();

                for value in y_values.iter().rev() {
                    stack.push_front(&value.to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(y_values.len()).to_bytes_be())
                    .unwrap();

                for i in (0..indices.len()).rev() {
                    let index = indices[i];
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.temp_queries;
                    queries_slice[i * 2] = index;
                }
                stack
                    .push_front(&Felt::from(indices.len()).to_bytes_be())
                    .unwrap();

                target_commitment.push_to_stack(stack);

                // println!("target_commitment: {:?}", target_commitment);
                // println!("y_values: {:?}", y_values);
                // println!("indices: {:?}", indices);
                // println!("table_witness: {:?}", table_witness);

                self.stage = FriVerifyLayersStep::WaitForTableDecommit;
                println!("Pushing FriVerifyLayers TableDecommit task");
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            FriVerifyLayersStep::WaitForTableDecommit => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                fri_verify_data.current_layer += 1;

                for i in 0..fri_verify_data.next_queries.len() {
                    if let Some(query) = fri_verify_data.next_queries.get(i) {
                        fri_verify_data.working_queries.push(*query);
                    }
                }
                fri_verify_data.next_queries.flush();

                self.stage = FriVerifyLayersStep::ProcessLayer;

                vec![]
            }

            FriVerifyLayersStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == FriVerifyLayersStep::Done
    }
}
