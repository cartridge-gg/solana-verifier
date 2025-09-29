use std::vec;

use felt::{Felt, NonZeroFelt};
use types::swiftness::{
    air::domains::{FIELD_GENERATOR, STARK_PRIME_MINUS_ONE},
    global_values::InteractionElements,
    stark::types::{FriVerifyData, StarkCommitment, StarkProof},
};
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, Executable, FullProofDataVerifier3,
    ProofData, StarkVerifyTrait, TypeIdentifiable,
};

use crate::eval_oods_boundary_poly_at_points::{ComputeQueryPoints, EvalOodsBoundaryPolyAtPoints};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkVerifyStep {
    ComputeQueryPoints,
    EvalOodsBoundaryPoly,
    FriVerify,
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
        T: BidirectionalStack + ProofData + StarkVerifyTrait + FullProofDataVerifier3 + CacheStorage,
    >(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::ComputeQueryPoints => {
                let queries_len = {
                    let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                    fri_verify_data.queries.len()
                };

                for i in (0..queries_len).rev() {
                    let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                    stack
                        .push_front(&fri_verify_data.queries.at(i).to_bytes_be())
                        .unwrap();
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
                println!("Pushing ComputeQueryPoints task");
                vec![ComputeQueryPoints::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::EvalOodsBoundaryPoly => {
                let points_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // for _ in 0..points_len.to_biguint().try_into().unwrap() {
                //     let point = Felt::from_bytes_be_slice(stack.borrow_front());
                //     stack.pop_front();
                //     let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                //     fri_verify_data.fri_decommitment.points.push(point);
                // }
                stack.push_front(&points_len.to_bytes_be()).unwrap();

                {
                    let (stark_commitment, _) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                    //unsafe
                    stack.set_constraint_coefficients(
                        stark_commitment.interaction_after_oods.as_slice(),
                    );
                }

                self.step = StarkVerifyStep::FriVerify;
                println!("Pushing EvalOodsBoundaryPoly task");
                vec![EvalOodsBoundaryPolyAtPoints::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::FriVerify => {
                self.step = StarkVerifyStep::Done;
                println!("Pushing FriVerify task");
                // vec![FriVerify::new().to_vec_with_type_tag()]
                vec![]
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

// #[inline(always)]
// fn commitment_push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(
//     commitment: &TableCommitment,
//     stack: &mut T,
// ) {
//     let commitment_bytes = cast_struct_to_slice(commitment);
//     stack.push_front(commitment_bytes).unwrap();
// }
