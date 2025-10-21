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
    #[inline(never)]
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
                // Get length first to avoid borrowing conflicts
                let queries_len = {
                    let verify_variables: &VerifyVariables = stack.get_verify_variables();
                    verify_variables.queries_indexes.len()
                };
                assert!(queries_len != 0, "Queries length is equal to 0");

                // Push each element individually to avoid stack allocation
                for i in (0..queries_len).rev() {
                    let query_value = {
                        let verify_variables: &VerifyVariables = stack.get_verify_variables();
                        verify_variables.queries_indexes[i]
                    }; // Release immutable borrow
                    stack.push_front(&query_value.to_bytes_be()).unwrap();
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
                // let mut points_vec = FunVec::<Felt, FUNVEC_QUERY_INDICES>::default();

                // Collect all points first to avoid repeated cache access in loop
                for i in 0..points_len.to_biguint().try_into().unwrap() {
                    let point = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    verify_variables.points[i] = point;
                }

                // Store points and queries in cache once after collecting all points
                {
                    // let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                    let (fri_verify_data, verify_variables) = stack.get_fri_verify_data_and_verify_variables_mut::<FriVerifyData, VerifyVariables>();

                    // Store queries from verify_variables - element by element to avoid stack allocation
                    fri_verify_data.queries.flush();
                    for i in 0..verify_variables.queries_indexes.len() {
                        fri_verify_data.queries.push(verify_variables.queries_indexes[i]);
                    }

                    // Store points
                    let fri_decommitment: &mut FriDecommitment =
                        &mut fri_verify_data.fri_decommitment;
                    fri_decommitment.points.flush();
                    for point in verify_variables.points.as_slice() {
                        fri_decommitment.points.push(*point);
                    }
                }

                let points_len = {
                    let verify_variables: &VerifyVariables = stack.get_verify_variables();
                    verify_variables.points.iter().filter(|&&p| p != Felt::ZERO).count()
                };

                for i in (0..points_len).rev() {
                    let point = {
                        let verify_variables: &VerifyVariables = stack.get_verify_variables();
                        verify_variables.points[i]
                    };
                    stack.push_front(&point.to_bytes_be()).unwrap();
                }
                stack.push_front(&Felt::from(points_len).to_bytes_be()).unwrap();
            
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
