use std::vec;

use felt::{Felt, NonZeroFelt};
use types::{
    funvec::{FunVec, FUNVEC_QUERY_INDICES},
    swiftness::{
        air::domains::{FIELD_GENERATOR, STARK_PRIME_MINUS_ONE},
        global_values::InteractionElements,
        stark::types::{FriVerifyData, StarkCommitment, StarkProof, VerifyVariables},
    },
};
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, CachedProofData, Executable,
    ProofData, ProofDataVerification, StarkVerifyTrait, TypeIdentifiable,
};

use crate::eval_oods_boundary_poly_at_points::{ComputeQueryPoints, EvalOodsBoundaryPolyAtPoints};
use types::swiftness::commitment::types::Decommitment as FriDecommitment;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StarkVerifyStep {
    ComputeQueryPoints,
    EvalOodsBoundaryPoly,
    Done,
}

#[repr(C)]
pub struct StarkVerify {
    step: StarkVerifyStep,
}

impl_type_identifiable!(StarkVerify);

impl StarkVerify {
    pub fn new() -> Self {
        Self {
            step: StarkVerifyStep::ComputeQueryPoints,
        }
    }
}

impl Default for StarkVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for StarkVerify {
    fn execute<
        T: BidirectionalStack
            + ProofData
            + StarkVerifyTrait
            + ProofDataVerification
            + CacheStorage
            + CachedProofData,
    >(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::ComputeQueryPoints => {
                solana_program::msg!("StarkVerifyStep::ComputeQueryPoints");
                // Use queries_indexes from VerifyVariables instead of cache to avoid access violations
                let queries_indexes = {
                    let verify_variables: &VerifyVariables = stack.get_verify_variables();
                    verify_variables.queries_indexes
                };
                let queries_len = queries_indexes.len();
                assert!(queries_len != 0, "Queries length is equal to 0");

                for i in (0..queries_len).rev() {
                    stack.push_front(&queries_indexes[i].to_bytes_be()).unwrap();
                }
                stack
                    .push_front(&Felt::from(queries_len).to_bytes_be())
                    .unwrap();

                let (log_trace_domain_size, log_n_cosets) = {
                    let proof: &StarkProof = stack.get_proof_reference();
                    (
                        proof.config.log_trace_domain_size,
                        proof.config.log_n_cosets,
                    )
                };
                let log_eval_domain_size = log_trace_domain_size + log_n_cosets;
                let eval_domain_size = Felt::TWO.pow_felt(&log_eval_domain_size);
                let eval_generator = FIELD_GENERATOR.pow_felt(
                    &STARK_PRIME_MINUS_ONE
                        .field_div(&NonZeroFelt::try_from(eval_domain_size).unwrap()),
                );

                stack.push_front(&eval_generator.to_bytes_be()).unwrap();
                stack
                    .push_front(&log_eval_domain_size.to_bytes_be())
                    .unwrap();

                self.step = StarkVerifyStep::EvalOodsBoundaryPoly;
                vec![ComputeQueryPoints::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::EvalOodsBoundaryPoly => {
                let points_len = Felt::from_bytes_be_slice(stack.borrow_front());
                assert!(points_len != Felt::ZERO, "Points length is equal to 0");
                stack.pop_front();
                let mut points_vec = FunVec::<Felt, FUNVEC_QUERY_INDICES>::default();

                // Collect all points first to avoid repeated cache access in loop
                for _ in 0..points_len.to_biguint().try_into().unwrap() {
                    let point = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    points_vec.push(point);
                }

                // Store points and queries in cache once after collecting all points
                let queries_indexes = {
                    let verify_variables: &VerifyVariables = stack.get_verify_variables();
                    verify_variables.queries_indexes
                };
                {
                    let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();

                    // Store queries from verify_variables
                    fri_verify_data.queries.flush();
                    for query in queries_indexes.iter() {
                        fri_verify_data.queries.push(*query);
                    }

                    // Store points
                    let fri_decommitment: &mut FriDecommitment =
                        &mut fri_verify_data.fri_decommitment;
                    fri_decommitment.points.flush();
                    for point in points_vec.as_slice() {
                        fri_decommitment.points.push(*point);
                    }
                }

                for point in points_vec.as_slice().iter().rev() {
                    stack.push_front(&point.to_bytes_be()).unwrap();
                }

                stack.push_front(&points_len.to_bytes_be()).unwrap();

                {
                    let (stark_commitment, coeffs) =
                        ProofDataVerification::get_stark_commitment_and_coefficients_mut::<
                            StarkCommitment<InteractionElements>,
                        >(stack);

                    coeffs.fill(Felt::ZERO);
                    for (i, coeff) in coeffs
                        .iter_mut()
                        .enumerate()
                        .take(stark_commitment.interaction_after_oods.len())
                    {
                        *coeff = *stark_commitment.interaction_after_oods.at(i);
                    }
                }

                self.step = StarkVerifyStep::Done;
                vec![EvalOodsBoundaryPolyAtPoints::new().to_vec_with_type_tag()]
            }
            StarkVerifyStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == StarkVerifyStep::Done
    }
}
