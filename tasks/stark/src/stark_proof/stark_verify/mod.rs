use std::vec;

use felt::Felt;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
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

use crate::{stark_proof::stark_verify::{eval_oods_boundary_poly_at_points::{ComputeQueryPoints, EvalOodsBoundaryPolyAtPoints}, fri_verify::FriVerify, table_decommit::TableDecommit, traces_decommit::TracesDecommit}, swiftness::{commitment::vector::types::Query, stark::types::StarkProof}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkVerifyStep {
    Init,
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
    n_original_columns: u32,
    n_interaction_columns: u32,
    queries_len: u128,
}

impl_type_identifiable!(StarkVerify);

impl StarkVerify {
    pub fn new(n_original_columns: u32, n_interaction_columns: u32) -> Self {
        Self {
            step: StarkVerifyStep::Init,
            n_original_columns,
            n_interaction_columns,
            queries_len: 0,
        }
    }
}

impl Default for StarkVerify {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Executable for StarkVerify {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::Init => {
                let proof: &StarkProof = stack.get_proof_reference();

                self.queries_len = match proof.config.n_queries.to_biguint().try_into() {
                    Ok(len) => len,
                    Err(_) => {
                        // Push error and finish
                        println!("Error: Queries len could not be converted to u128");
                        self.step = StarkVerifyStep::Done;
                        return vec![];
                    }
                };
                // Is this sanity check? why do we take n_queries from proof and then read from stack?
                // Should we just read from stack? or assert they are equal?

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                println!("READ: Queries length: {:?}", queries_len);
                stack.pop_front();

                let mut queries = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());
                for _ in 0..queries_len.to_biguint().try_into().unwrap() {
                    queries.push(Query::from_stack(stack));
                }
                // Push queries back onto stack using helper method
                Query::push_queries_to_stack(queries.len(), stack);

                self.step = StarkVerifyStep::TracesDecommit;
                vec![TracesDecommit::new().to_vec_with_type_tag()]
            }
            StarkVerifyStep::TracesDecommit => {
                self.step = StarkVerifyStep::TableDecommit;
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::TableDecommit => {
                self.step = StarkVerifyStep::ComputeQueryPoints;
                vec![ComputeQueryPoints::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::ComputeQueryPoints => {
                self.step = StarkVerifyStep::EvalOodsBoundaryPoly;
                vec![EvalOodsBoundaryPolyAtPoints::new()
                .to_vec_with_type_tag()]
            }

            StarkVerifyStep::EvalOodsBoundaryPoly => {
                self.step = StarkVerifyStep::FriVerify;
                vec![FriVerify::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::FriVerify => {
                self.step = StarkVerifyStep::Done;
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
