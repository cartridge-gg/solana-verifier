use felt::{Felt, NonZeroFelt};
use types::swiftness::{fri::types::FriLayerQuery, stark::types::{FriVerifyData, StarkCommitment, StarkProof}};
use utils::{impl_type_identifiable, BidirectionalStack, CacheStorage, CachedProofData, Executable, ProofData, TypeIdentifiable};
use types::swiftness::global_values::InteractionElements;

use crate::fri_verify_layers::FriVerifyLayers;
// use crate::fri_verify_layers::FriVerifyLayers;
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    stage: FriVerifyStep,
}

#[allow(dead_code)]
const FIELD_GENERATOR_INVERSE: Felt =
    Felt::from_hex_unchecked("0x2AAAAAAAAAAAAB0555555555555555555555555555555555555555555555556");

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FriVerifyStep {
    Init,
    VerifyLastLayer,
    Done,
}
impl_type_identifiable!(FriVerify);

impl FriVerify {
    pub fn new() -> Self {
        Self {
            stage: FriVerifyStep::Init,
        }
    }
}

impl Default for FriVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriVerify {
    fn execute<T: BidirectionalStack + ProofData + CacheStorage + CachedProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            FriVerifyStep::Init => {
                let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                let queries_len = fri_verify_data.queries.len();
                let fri_len = fri_verify_data.fri_decommitment.values.len();
                
                assert_eq!(
                    fri_len, queries_len,
                    "FRI decommitment length does not match queries length"
                );

                for (index, query) in fri_verify_data.queries.iter().enumerate() {
                    if index < fri_verify_data.fri_decommitment.values.len()
                        && index < fri_verify_data.fri_decommitment.points.len()
                    {
                        let shifted_x_value = fri_verify_data.fri_decommitment.points.at(index)
                            * FIELD_GENERATOR_INVERSE;

                        fri_verify_data.layer_queries.push(FriLayerQuery {
                            index: *query,
                            y_value: *fri_verify_data.fri_decommitment.values.at(index),
                            x_inv_value: Felt::ONE
                                .field_div(&NonZeroFelt::from_felt_unchecked(shifted_x_value)),
                        });
                    }
                }
                
                fri_verify_data.init_active_queries();
                
                self.stage = FriVerifyStep::VerifyLastLayer;
                vec![FriVerifyLayers::new().to_vec_with_type_tag()]
            }


            FriVerifyStep::VerifyLastLayer => {
                let (stark_commitment, _, fri_verify_data) = stack.get_stark_commitment_proof_and_cache::<
                StarkCommitment<InteractionElements>,
                StarkProof,
                FriVerifyData
                >();

                let fri_commitment = &stark_commitment.fri;
                
                for i in 0..fri_verify_data.active_query_count {
                    let query = fri_verify_data.layer_queries.get(i).unwrap();

                    let horner_result = self.horner_eval(
                        fri_commitment.last_layer_coefficients.as_slice(),
                        Felt::ONE.field_div(&NonZeroFelt::from_felt_unchecked(query.x_inv_value)),
                    );

                    if horner_result != query.y_value {
                        panic!("Last layer verification failed");
                    }
                }

                self.stage = FriVerifyStep::Done;
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

impl FriVerify {
    #[inline(always)]
    fn horner_eval(&self, coefficients: &[Felt], point: Felt) -> Felt {
        let mut result = Felt::ZERO;
        for coef in coefficients.iter().rev() {
            result = result * point + coef;
        }
        result
    }
}
