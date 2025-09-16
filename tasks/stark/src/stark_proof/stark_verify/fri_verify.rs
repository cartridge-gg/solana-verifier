use felt::{Felt, NonZeroFelt};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::funvec::FunVec;

use crate::{
    stark_proof::stark_verify::FriVerifyLayers,
    swiftness::{fri::types::FriLayerQuery, stark::types::FriVerifyData},
};

// FriVerify task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    stage: FriVerifyStep,
    fri_queries: FunVec<FriLayerQuery, 256>,
}

#[allow(dead_code)]
const FIELD_GENERATOR_INVERSE: Felt =
    Felt::from_hex_unchecked("0x2AAAAAAAAAAAAB0555555555555555555555555555555555555555555555556");

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FriVerifyStep {
    Init,
    VerifyInnerLayers,
    VerifyLastLayer,
    Done,
}
impl_type_identifiable!(FriVerify);

impl FriVerify {
    pub fn new() -> Self {
        Self {
            stage: FriVerifyStep::Init,
            fri_queries: FunVec::default(),
        }
    }
}

impl Default for FriVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriVerify {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            FriVerifyStep::Init => {
                let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                println!(
                    "DEBUG: eval_points[0]: {:?}",
                    fri_verify_data.fri_commitment.eval_points.get(0).unwrap()
                );

                let queries_len = fri_verify_data.queries.len();
                let fri_len = fri_verify_data.fri_decommitment.values.len();

                assert_eq!(
                    fri_len, queries_len,
                    "FRI decommitment length does not match queries length"
                );

                self.fri_queries.flush();
                for (index, query) in fri_verify_data.queries.iter().enumerate() {
                    if index < fri_verify_data.fri_decommitment.values.len()
                        && index < fri_verify_data.fri_decommitment.points.len()
                    {
                        // Translate the coset to the homogenous group to have simple FRI equations.
                        let shifted_x_value = fri_verify_data.fri_decommitment.points.at(index)
                            * FIELD_GENERATOR_INVERSE;

                        self.fri_queries.push(FriLayerQuery {
                            index: *query,
                            y_value: *fri_verify_data.fri_decommitment.values.at(index),
                            x_inv_value: Felt::ONE
                                .field_div(&NonZeroFelt::from_felt_unchecked(shifted_x_value)),
                        });
                    }
                }
                self.stage = FriVerifyStep::VerifyInnerLayers;
                println!("Transitioning to ComputeFirstLayer");
                vec![]
            }

            FriVerifyStep::VerifyInnerLayers => {
                // Initialize FriVerifyData with data needed for ComputeNextLayer
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // Initialize working_queries with fri_queries from Init step
                fri_verify_data.working_queries.flush();
                for query in &self.fri_queries {
                    fri_verify_data.working_queries.push(*query);
                }
                self.stage = FriVerifyStep::VerifyLastLayer;
                vec![FriVerifyLayers::new().to_vec_with_type_tag()]
            }

            FriVerifyStep::VerifyLastLayer => {
                // Verify last layer using Horner evaluation
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                for i in 0..fri_verify_data.working_queries.len() {
                    let query = fri_verify_data.working_queries.get(i).unwrap();
                    let horner_result = self.horner_eval(
                        fri_verify_data
                            .fri_commitment
                            .last_layer_coefficients
                            .as_slice(),
                        Felt::ONE.field_div(&NonZeroFelt::from_felt_unchecked(query.x_inv_value)),
                    );

                    if horner_result != query.y_value {
                        panic!(
                            "Last layer verification failed: expected {}, got {}",
                            query.y_value, horner_result
                        );
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
