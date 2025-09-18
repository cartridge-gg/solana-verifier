use std::vec;

use crate::swiftness::{
    air::domains::{FIELD_GENERATOR, STARK_PRIME_MINUS_ONE},
    commitment::vector::types::CommitmentTrait,
    stark::types::VerifyVariables,
};
use felt::{Felt, NonZeroFelt};
use utils::{
    global_values::InteractionElements, impl_type_identifiable, BidirectionalStack, Executable,
    ProofData, StarkVerifyTrait, TypeIdentifiable,
};
pub mod compute_root_recursive;
pub mod eval_oods_boundary_poly_at_points;
pub mod eval_oods_polynomial;
pub mod fri_verify;
pub mod fri_verify_layers;
pub mod hash_computation;
pub mod table_decommit;
pub mod traces_decommit;
pub mod vector_decommit;
pub use fri_verify_layers::FriVerifyLayers;
pub mod compute_coset_elements;
pub mod compute_next_layer;
pub mod fri_formula;
pub mod group;

// Re-export the new task types
pub use compute_coset_elements::ComputeCosetElements;
pub use compute_next_layer::ComputeNextLayer;
pub use fri_formula::FriFormula;
pub use vector_decommit::VectorDecommit;

use crate::{
    stark_proof::stark_verify::{
        eval_oods_boundary_poly_at_points::{ComputeQueryPoints, EvalOodsBoundaryPolyAtPoints},
        fri_verify::FriVerify,
        table_decommit::TableDecommit,
        traces_decommit::TracesDecommit,
    },
    swiftness::stark::types::{FriVerifyData, StarkCommitment, StarkProof},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkVerifyStep {
    TracesDecommit,
    TableDecommit,
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
            step: StarkVerifyStep::TracesDecommit,
        }
    }
}

impl Default for StarkVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for StarkVerify {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::TracesDecommit => {
                let queries_len = {
                    let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                    fri_verify_data.queries.len()
                };

                for i in (0..queries_len).rev() {
                    let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                    let query = fri_verify_data.queries.at(i);
                    stack.push_front(&query.to_bytes_be()).unwrap();
                }

                stack
                    .push_front(&Felt::from(queries_len).to_bytes_be())
                    .unwrap();

                println!("Pushing TracesDecommit task");
                self.step = StarkVerifyStep::TableDecommit;
                vec![TracesDecommit::new().to_vec_with_type_tag()]
            }
            StarkVerifyStep::TableDecommit => {
                let (authentications_len, decommitment_values_len) = {
                    let (_, proof) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                    let authentications = &proof.witness.composition_witness.vector.authentications;
                    let decommitment_values = &proof.witness.composition_decommitment.values;
                    (authentications.len(), decommitment_values.len())
                };
                // println!("authentications_len: {}", authentications_len);
                // println!("decommitment_values_len: {}", decommitment_values_len);

                {
                    for i in (0..authentications_len).rev() {
                        let (_, proof) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                        let authentications =
                            &proof.witness.composition_witness.vector.authentications;
                        stack
                            .push_front(&authentications.at(i).to_bytes_be())
                            .unwrap();
                    }

                    stack
                        .push_front(&Felt::from(authentications_len as u64).to_bytes_be())
                        .unwrap();
                }

                {
                    for i in (0..decommitment_values_len).rev() {
                        let (_, proof) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                        let decommitment_values = &proof.witness.composition_decommitment.values;
                        stack
                            .push_front(&decommitment_values.at(i).to_bytes_be())
                            .unwrap();
                    }

                    let decommitment_length = Felt::from(decommitment_values_len as u64);
                    stack
                        .push_front(&decommitment_length.to_bytes_be())
                        .unwrap();
                }

                let queries_len = {
                    let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                    fri_verify_data.queries.len()
                };
                // println!("queries_len: {:?}", queries_len);

                for i in (0..queries_len).rev() {
                    let index = {
                        let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                        *fri_verify_data.queries.at(i)
                    };

                    {
                        let verify_variables: &mut VerifyVariables =
                            stack.get_verify_variables_mut();
                        let queries_slice = &mut verify_variables.temp_queries;
                        queries_slice[i * 2] = index;
                    }
                }

                stack
                    .push_front(&Felt::from(queries_len).to_bytes_be())
                    .unwrap();

                {
                    let (stark_commitment, _) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                    let table_commitment = stark_commitment.composition;
                    //here we clone the table_commitment to avoid borrowing issues
                    table_commitment.push_to_stack(stack);
                }

                self.step = StarkVerifyStep::ComputeQueryPoints;
                println!("Pushing TableDecommit task");
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

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
                for _ in 0..points_len.to_biguint().try_into().unwrap() {
                    let point = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                    fri_verify_data.fri_decommitment.points.push(point);
                }
                stack.push_front(&points_len.to_bytes_be()).unwrap();

                self.step = StarkVerifyStep::FriVerify;
                println!("Pushing EvalOodsBoundaryPoly task");
                vec![EvalOodsBoundaryPolyAtPoints::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::FriVerify => {
                self.step = StarkVerifyStep::Done;
                println!("Pushing FriVerify task");
                vec![FriVerify::new().to_vec_with_type_tag()]
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
