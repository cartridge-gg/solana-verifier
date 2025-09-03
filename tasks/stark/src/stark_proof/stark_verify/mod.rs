use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

pub mod compute_root_recursive;
pub mod eval_oods_boundary_poly_at_points;
pub mod eval_oods_polynomial;
pub mod fri_verify;
pub mod hash_computation;
pub mod table_decommit;
pub mod traces_decommit;
pub mod vector_decommit;

// Re-export the new task types
pub use eval_oods_boundary_poly_at_points::{ComputeQueryPoints, EvalOodsBoundaryPolyAtPoints};
pub use eval_oods_polynomial::EvalOodsPolynomial;
pub use fri_verify::FriVerify;
pub use table_decommit::TableDecommit;
pub use traces_decommit::TracesDecommit;
pub use vector_decommit::VectorDecommit;

use crate::swiftness::commitment::vector::types::Query;
use crate::swiftness::stark::types::StarkProof;

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
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::Init => {
                // Read queries from stack (should be pushed by caller)
                // Expected stack format: [query_n, query_n-1, ..., query_1, query_0, queries_len]
                // Each query is a Query struct: [index, value] (64 bytes total)
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

                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                println!("READ: Queries length: {:?}", queries_len);
                stack.pop_front();

                let mut queries = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());
                for _ in 0..queries_len.to_biguint().try_into().unwrap() {
                    queries.push(Query::from_stack(stack));
                }
                // Push queries back onto stack using helper method
                Query::push_queries_to_stack(&queries, stack);

                self.step = StarkVerifyStep::TracesDecommit;
                vec![TracesDecommit::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::TracesDecommit => {
                // TracesDecommit finished, continue with table decommit
                // Pass through queries for table_decommit
                // Queries should already be on stack in correct format from previous task

                self.step = StarkVerifyStep::TableDecommit;
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::TableDecommit => {
                // TableDecommit finished, compute query points
                self.step = StarkVerifyStep::ComputeQueryPoints;
                vec![ComputeQueryPoints::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::ComputeQueryPoints => {
                // Query points computed, evaluate OODS boundary poly
                self.step = StarkVerifyStep::EvalOodsBoundaryPoly;
                vec![EvalOodsBoundaryPolyAtPoints::new(
                    self.n_original_columns,
                    self.n_interaction_columns,
                )
                .to_vec_with_type_tag()]
            }

            StarkVerifyStep::EvalOodsBoundaryPoly => {
                // OODS evaluation finished, start FRI verification
                self.step = StarkVerifyStep::FriVerify;
                vec![FriVerify::new().to_vec_with_type_tag()]
            }

            StarkVerifyStep::FriVerify => {
                // FRI verification finished, read result
                let result = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push final verification result
                stack.push_front(&result.to_bytes_be()).unwrap();

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
