use felt::Felt;
use types::swiftness::{global_values::InteractionElements, stark::types::cast_struct_to_slice};
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, CachedProofData, Executable,
    ProofData, StarkVerifyTrait, TypeIdentifiable,
};

use crate::compute_next_layer::ComputeNextLayer;
use stark_verify_decommitments::table_decommit::TableDecommit;
use types::swiftness::stark::types::{FriVerifyData, StarkCommitment, StarkProof, VerifyVariables};

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
    fn execute<
        T: BidirectionalStack + ProofData + StarkVerifyTrait + CachedProofData + CacheStorage,
    >(
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
                fri_verify_data.current_layer = 0;
                self.stage = if stark_commitment
                    .fri
                    .config
                    .n_layers
                    .to_biguint()
                    .try_into()
                    .ok()
                    == Some(0usize)
                {
                    FriVerifyLayersStep::Done
                } else {
                    FriVerifyLayersStep::ProcessLayer
                };
                vec![]
            }

            FriVerifyLayersStep::ProcessLayer => {
                let (stark_commitment, proof, fri_verify_data) = stack
                .get_stark_commitment_proof_and_cache_mut::<
                    StarkCommitment<InteractionElements>,
                    StarkProof,
                    FriVerifyData,
                >();

                let n_layers: usize = stark_commitment
                    .fri
                    .config
                    .n_layers
                    .to_biguint()
                    .try_into()
                    .unwrap();

                if fri_verify_data.current_layer < n_layers - 1 {
                    let current = fri_verify_data.current_layer;

                    let step_size = *stark_commitment
                        .fri
                        .config
                        .fri_step_sizes
                        .get(current + 1)
                        .unwrap();

                    fri_verify_data.coset_size = Felt::TWO.pow_felt(&step_size);
                    fri_verify_data.eval_point =
                        *stark_commitment.fri.eval_points.get(current).unwrap();

                    let leaves = &proof
                        .witness
                        .fri_witness
                        .layers
                        .get(current)
                        .unwrap()
                        .leaves;
                    for i in 0..leaves.len() {
                        if let Some(value) = leaves.get(i) {
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
                {
                    let (auth_len, current_layer) = {
                        let (_, proof, fri_verify_data) = stack.get_stark_commitment_proof_and_cache::<
                            StarkCommitment<InteractionElements>,
                            StarkProof,
                            FriVerifyData,
                        >();
                        (
                            proof
                                .witness
                                .fri_witness
                                .layers
                                .get(fri_verify_data.current_layer)
                                .unwrap()
                                .table_witness
                                .vector
                                .authentications
                                .len(),
                            fri_verify_data.current_layer,
                        )
                    };

                    const CHUNK_SIZE: usize = 16;
                    let num_chunks = auth_len.div_ceil(CHUNK_SIZE);

                    for chunk_idx in (0..num_chunks).rev() {
                        let start = chunk_idx * CHUNK_SIZE;
                        let end = (start + CHUNK_SIZE).min(auth_len);
                        let chunk_len = end - start;

                        let auth_chunk = {
                            let (_, proof, _) = stack.get_stark_commitment_proof_and_cache::<
                                StarkCommitment<InteractionElements>,
                                StarkProof,
                                FriVerifyData,
                            >();
                            let auth_slice = proof
                                .witness
                                .fri_witness
                                .layers
                                .get(current_layer)
                                .unwrap()
                                .table_witness
                                .vector
                                .authentications
                                .as_slice();

                            let mut buffer = [[0u8; 32]; CHUNK_SIZE];
                            for i in 0..chunk_len {
                                buffer[i] = auth_slice[start + i].to_bytes_be();
                            }
                            buffer
                        };

                        for i in (0..chunk_len).rev() {
                            stack.push_front(&auth_chunk[i]).unwrap();
                        }
                    }

                    stack
                        .push_front(&Felt::from(auth_len).to_bytes_be())
                        .unwrap();
                }

                let (y_len, indices_len, commitment) = {
                    let (stark_commitment, _, fri_verify_data) = stack
                        .get_stark_commitment_proof_and_cache::<
                            StarkCommitment<InteractionElements>,
                            StarkProof,
                            FriVerifyData,
                        >();

                    let current = fri_verify_data.current_layer;

                    (
                        fri_verify_data.working_y_values.len(),
                        fri_verify_data.working_indices.len(),
                        *stark_commitment.fri.inner_layers.get(current).unwrap(),
                    )
                };

                for i in (0..y_len).rev() {
                    let y_bytes = {
                        let (_, _, fri_verify_data) = stack.get_stark_commitment_proof_and_cache::<
                            StarkCommitment<InteractionElements>,
                            StarkProof,
                            FriVerifyData,
                        >();
                        fri_verify_data
                            .working_y_values
                            .get(i)
                            .unwrap()
                            .to_bytes_be()
                    };
                    stack.push_front(&y_bytes).unwrap();
                }
                stack.push_front(&Felt::from(y_len).to_bytes_be()).unwrap();

                // Aktualizacja temp_queries
                for i in (0..indices_len).rev() {
                    let index_value = {
                        let (_, _, fri_verify_data) = stack.get_stark_commitment_proof_and_cache::<
                            StarkCommitment<InteractionElements>,
                            StarkProof,
                            FriVerifyData,
                        >();
                        *fri_verify_data.working_indices.get(i).unwrap()
                    };
                    let verify_vars: &mut VerifyVariables = stack.get_verify_variables_mut();
                    verify_vars.temp_queries[i * 2] = index_value;
                }
                stack
                    .push_front(&Felt::from(indices_len).to_bytes_be())
                    .unwrap();

                stack.push_front(cast_struct_to_slice(&commitment)).unwrap();

                self.stage = FriVerifyLayersStep::WaitForTableDecommit;
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            FriVerifyLayersStep::WaitForTableDecommit => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();
                fri_verify_data.current_layer += 1;
                fri_verify_data.advance_layer();

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
