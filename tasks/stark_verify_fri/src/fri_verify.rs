use crate::fri_verify_layers::FriVerifyLayers;
use felt::{Felt, NonZeroFelt};
use types::swiftness::global_values::InteractionElements;
use types::swiftness::{
    fri::types::FriLayerQuery,
    stark::types::{FriVerifyData, StarkCommitment, StarkProof},
};
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, CachedProofData, Executable,
    ProofData, TypeIdentifiable,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    stage: FriVerifyStep,
    current_query_index: usize,
}

#[allow(dead_code)]
const FIELD_GENERATOR_INVERSE: Felt =
    Felt::from_hex_unchecked("0x2AAAAAAAAAAAAB0555555555555555555555555555555555555555555555556");

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FriVerifyStep {
    Init,
    InitBatch(usize), // Process queries in batches
    VerifyLastLayer(usize),
    Done,
}
impl_type_identifiable!(FriVerify);

impl FriVerify {
    pub fn new() -> Self {
        Self {
            stage: FriVerifyStep::Init,
            current_query_index: 0,
        }
    }
}

impl Default for FriVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriVerify {
    fn execute<T: BidirectionalStack + ProofData + CacheStorage + CachedProofData>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.stage {
            FriVerifyStep::Init => {
                let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();

                assert_eq!(
                    fri_verify_data.fri_decommitment.values.len(),
                    fri_verify_data.queries.len(),
                    "FRI decommitment length does not match queries length"
                );

                self.current_query_index = 0;
                self.stage = FriVerifyStep::InitBatch(0);
                vec![]
            }

            FriVerifyStep::InitBatch(chunk_index) => {
                let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();

                const BATCH_SIZE: usize = 10;
                let query_count = fri_verify_data.queries.len();
                let start = chunk_index * BATCH_SIZE;
                let end = (start + BATCH_SIZE).min(query_count);

                for index in start..end {
                    let query = *fri_verify_data.queries.at(index);
                    let point = *fri_verify_data.fri_decommitment.points.at(index);
                    let value = *fri_verify_data.fri_decommitment.values.at(index);

                    fri_verify_data.layer_queries.push(FriLayerQuery {
                        index: query,
                        y_value: value,
                        x_inv_value: Felt::ONE.field_div(&NonZeroFelt::from_felt_unchecked(
                            point * FIELD_GENERATOR_INVERSE,
                        )),
                    });
                }

                if end >= query_count {
                    fri_verify_data.init_active_queries();
                    self.stage = FriVerifyStep::VerifyLastLayer(0);
                    vec![FriVerifyLayers::new().to_vec_with_type_tag()]
                } else {
                    self.stage = FriVerifyStep::InitBatch(chunk_index + 1);
                    vec![]
                }
            }

            FriVerifyStep::VerifyLastLayer(chunk_index) => {
                let (stark_commitment, _, fri_verify_data) = stack.get_stark_commitment_proof_and_cache::<
                    StarkCommitment<InteractionElements>,
                    StarkProof,
                    FriVerifyData
                >();

                const CHUNK_SIZE: usize = 4;
                let start = chunk_index * CHUNK_SIZE;
                let end = (start + CHUNK_SIZE).min(fri_verify_data.active_query_count);

                for i in start..end {
                    let query = fri_verify_data.layer_queries.get(i).unwrap();

                    // Horner eval inline
                    let mut result = Felt::ZERO;
                    let coefficients = stark_commitment.fri.last_layer_coefficients.as_slice();
                    let point =
                        Felt::ONE.field_div(&NonZeroFelt::from_felt_unchecked(query.x_inv_value));

                    for coef in coefficients.iter().rev() {
                        result = result * point + coef;
                    }

                    assert_eq!(result, query.y_value, "Last layer verification failed");
                }

                self.stage = if end >= fri_verify_data.active_query_count {
                    FriVerifyStep::Done
                } else {
                    FriVerifyStep::VerifyLastLayer(chunk_index + 1)
                };

                vec![]
            }

            FriVerifyStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == FriVerifyStep::Done
    }
}
