use felt::{Felt, NonZeroFelt};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::{
    stark_proof::stark_verify::{group::get_fri_group, FriVerifyLayers},
    swiftness::{
        commitment,
        fri::{self, types::FriLayerQuery},
        stark::{
            self,
            types::{cast_slice_to_struct, FriVerifyData},
        },
    },
};

// FriVerify task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    stage: FriVerifyStep,
    fri_queries: Vec<FriLayerQuery>,
}

#[allow(dead_code)]
const FIELD_GENERATOR_INVERSE: Felt =
    Felt::from_hex_unchecked("0x2AAAAAAAAAAAAB0555555555555555555555555555555555555555555555556");

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FriVerifyStep {
    Init,
    ComputeFirstLayer,
    ComputeFriGroup,
    VerifyInnerLayers,
    VerifyLastLayer,
    Done,
}
impl_type_identifiable!(FriVerify);

impl FriVerify {
    pub fn new() -> Self {
        Self {
            stage: FriVerifyStep::Init,
            fri_queries: Vec::new(),
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

                let queries_len = fri_verify_data.queries.len();
                let fri_len = fri_verify_data.fri_decommitment.values.len();

                assert_eq!(
                    fri_len, queries_len,
                    "FRI decommitment length does not match queries length"
                );

                self.fri_queries.clear();
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
                // stack.pop_front();
                self.stage = FriVerifyStep::ComputeFirstLayer;
                println!("Transitioning to ComputeFirstLayer");
                vec![]
            }
            FriVerifyStep::ComputeFirstLayer => {
                self.stage = FriVerifyStep::ComputeFriGroup;
                println!("Transitioning to ComputeFriGroup");
                vec![]
            }
            FriVerifyStep::ComputeFriGroup => {
                self.stage = FriVerifyStep::VerifyInnerLayers;
                println!("Transitioning to VerifyInnerLayers");
                vec![]
            }
            FriVerifyStep::VerifyInnerLayers => {
                let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                let n_layers = fri_verify_data.fri_commitment.config.n_layers;
                let total_layers = n_layers
                    .to_biguint()
                    .try_into()
                    .unwrap_or(0usize)
                    .saturating_sub(1); // n_layers - 1 inner layers

                self.stage = FriVerifyStep::VerifyLastLayer;
                vec![FriVerifyLayers::new(
                    // get_fri_group(),
                    // n_layers,
                    // fri_verify_data.fri_commitment.inner_layers.to_vec(),
                    // fri_verify_data.witness.layers,
                    // fri_verify_data.fri_commitment.eval_points.clone(),
                    // fri_verify_data.fri_commitment.config.fri_step_sizes.as_slice()[1..].to_vec(),
                    // self.fri_queries.clone(),
                )
                .to_vec_with_type_tag()]
            }
            FriVerifyStep::VerifyLastLayer => {
                // Verify last layer using Horner evaluation
                let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();

                // Get the last queries from FriVerifyLayers (should be on stack or in fri_queries)
                // For each query, evaluate the polynomial using Horner's method
                for query in &self.fri_queries {
                    let horner_result = self.horner_eval(
                        &fri_verify_data.fri_commitment.last_layer_coefficients,
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
