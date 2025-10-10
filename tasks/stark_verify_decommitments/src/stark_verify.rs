use std::vec;

use felt::Felt;
use types::{
    funvec::FunVec,
    swiftness::{
        commitment::table::types::Commitment as TableCommitment,
        global_values::InteractionElements,
        stark::types::{
            cast_struct_to_slice, FriVerifyData, StarkCommitment, StarkProof, VerifyVariables,
        },
    },
};
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, Executable, ProofData,
    ProofDataDecommitment, StarkVerifyTrait, TypeIdentifiable,
};

pub use crate::vector_decommit::VectorDecommit;
use crate::{table_decommit::TableDecommit, traces_decommit::TracesDecommit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkVerifyStep {
    TracesDecommit,
    TableDecommit,
    Done,
}

#[repr(C)]
pub struct StarkVerify {
    step: StarkVerifyStep,
    queries: FunVec<Felt, 16>,
}

impl_type_identifiable!(StarkVerify);

impl StarkVerify {
    pub fn new() -> Self {
        Self {
            step: StarkVerifyStep::TracesDecommit,
            queries: FunVec::default(),
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
        T: BidirectionalStack + ProofData + StarkVerifyTrait + ProofDataDecommitment + CacheStorage,
    >(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::TracesDecommit => {
                // let queries_len = {
                //     let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                //     fri_verify_data.queries.len()
                // };
                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                for _ in 0..queries_len.to_biguint().try_into().unwrap() {
                    self.queries
                        .push(Felt::from_bytes_be_slice(stack.borrow_front()));
                    stack.pop_front();
                }
                {
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    verify_variables.queries_indexes = self.queries.as_slice().try_into().unwrap();
                }
                // {
                //     let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                //     fri_verify_data.queries.flush();
                //     let queries = self.queries.as_slice();
                //     for query in queries {
                //         fri_verify_data.queries.push(*query);
                //     }
                // };

                for query in self.queries.as_slice().iter().rev() {
                    stack.push_front(&query.to_bytes_be()).unwrap();
                }

                stack
                    .push_front(&Felt::from(self.queries.len()).to_bytes_be())
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

                let queries_len = self.queries.len();

                for i in (0..queries_len).rev() {
                    // let index = {
                    //     let fri_verify_data: &FriVerifyData = stack.borrow_from_cache();
                    //     *fri_verify_data.queries.at(i)
                    // };
                    let index = self.queries.at(i);

                    {
                        let verify_variables: &mut VerifyVariables =
                            stack.get_verify_variables_mut();
                        let queries_slice = &mut verify_variables.temp_queries;
                        queries_slice[i * 2] = *index;
                    }
                }

                stack
                    .push_front(&Felt::from(queries_len).to_bytes_be())
                    .unwrap();

                {
                    let (stark_commitment, _) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                    let table_commitment = stark_commitment.composition;
                    //here we clone the table_commitment to avoid borrowing issues
                    commitment_push_to_stack(&table_commitment, stack);
                }

                self.step = StarkVerifyStep::Done;
                println!("Pushing TableDecommit task");
                vec![TableDecommit::new().to_vec_with_type_tag()]
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

#[inline(always)]
fn commitment_push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(
    commitment: &TableCommitment,
    stack: &mut T,
) {
    let commitment_bytes = cast_struct_to_slice(commitment);
    stack.push_front(commitment_bytes).unwrap();
}
