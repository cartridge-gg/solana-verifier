use felt::Felt;
use utils::global_values::InteractionElements;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::funvec::{FunVec, FUNVEC_QUERY_INDICES};
use crate::stark_proof::stark_verify::table_decommit::TableDecommit;
use crate::swiftness::stark::types::{
    cast_struct_to_slice, StarkCommitment, StarkProof, VerifyVariables,
};

// TracesDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracesDecommitStep {
    PrepareOriginalTable,
    PrepareInteractionTable,
    Done,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TracesDecommit {
    step: TracesDecommitStep,
    queries_count: usize,
    decommitment_values_len: Felt,
    witness_count: usize,
    indexes: FunVec<Felt, FUNVEC_QUERY_INDICES>,
}

impl_type_identifiable!(TracesDecommit);

impl TracesDecommit {
    pub fn new() -> Self {
        Self {
            step: TracesDecommitStep::PrepareOriginalTable,
            queries_count: 0,
            decommitment_values_len: Felt::ZERO,
            witness_count: 0,
            indexes: FunVec::from_vec(vec![Felt::ZERO; FUNVEC_QUERY_INDICES]),
        }
    }
}

impl Default for TracesDecommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for TracesDecommit {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            TracesDecommitStep::PrepareOriginalTable => {
                //collect query indices
                self.queries_count = Felt::from_bytes_be_slice(stack.borrow_front())
                    .to_biguint()
                    .try_into()
                    .unwrap();
                stack.pop_front();

                for i in 0..self.queries_count {
                    let index = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();

                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.temp_queries;
                    queries_slice[i * 2] = index;
                    *self.indexes.at_mut(i) = index;
                }

                {
                    let proof = stack.get_proof_reference::<StarkProof>();
                    self.decommitment_values_len =
                        Felt::from(proof.witness.traces_decommitment.original.values.len());
                    self.witness_count = proof
                        .witness
                        .traces_witness
                        .original
                        .vector
                        .authentications
                        .len();
                }

                {
                    // Push witness authentications first
                    for i in (0..self.witness_count).rev() {
                        let proof = stack.get_proof_reference::<StarkProof>();
                        stack
                            .push_front(
                                &proof
                                    .witness
                                    .traces_witness
                                    .original
                                    .vector
                                    .authentications
                                    .as_slice()[i]
                                    .to_bytes_be(),
                            )
                            .unwrap();
                    }
                    stack
                        .push_front(&Felt::from(self.witness_count).to_bytes_be())
                        .unwrap();
                }

                {
                    // Push decommitment values
                    for i in (0..self
                        .decommitment_values_len
                        .to_biguint()
                        .try_into()
                        .unwrap())
                        .rev()
                    {
                        let proof = stack.get_proof_reference::<StarkProof>();
                        let decommitment_bytes =
                            proof.witness.traces_decommitment.original.values.as_slice()[i]
                                .to_bytes_be();
                        stack.push_front(&decommitment_bytes).unwrap();
                    }
                    stack
                        .push_front(&self.decommitment_values_len.to_bytes_be())
                        .unwrap();
                }
                stack
                    .push_front(&Felt::from(self.queries_count).to_bytes_be())
                    .unwrap();

                {
                    // Push Original Table Commitment last
                    let (stark_commitment, _) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                    let original_table_commitment = stark_commitment.traces.original;
                    let commitment_bytes = cast_struct_to_slice(&original_table_commitment);
                    stack.push_front(commitment_bytes).unwrap();
                }
                self.step = TracesDecommitStep::PrepareInteractionTable;
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            TracesDecommitStep::PrepareInteractionTable => {
                for i in (0..self.queries_count).rev() {
                    let index = self.indexes.at(i);
                    let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                    let queries_slice = &mut verify_variables.temp_queries;
                    queries_slice[i * 2] = *index;
                }

                {
                    let proof = stack.get_proof_reference::<StarkProof>();
                    self.decommitment_values_len =
                        Felt::from(proof.witness.traces_decommitment.interaction.values.len());
                    self.witness_count = proof
                        .witness
                        .traces_witness
                        .interaction
                        .vector
                        .authentications
                        .len();
                }

                {
                    // Push witness authentications first
                    for i in (0..self.witness_count).rev() {
                        let proof = stack.get_proof_reference::<StarkProof>();
                        stack
                            .push_front(
                                &proof
                                    .witness
                                    .traces_witness
                                    .interaction
                                    .vector
                                    .authentications
                                    .as_slice()[i]
                                    .to_bytes_be(),
                            )
                            .unwrap();
                    }
                    stack
                        .push_front(&Felt::from(self.witness_count).to_bytes_be())
                        .unwrap();
                }

                {
                    // Push decommitment values
                    for i in (0..self
                        .decommitment_values_len
                        .to_biguint()
                        .try_into()
                        .unwrap())
                        .rev()
                    {
                        let proof = stack.get_proof_reference::<StarkProof>();
                        let decommitment_bytes = proof
                            .witness
                            .traces_decommitment
                            .interaction
                            .values
                            .as_slice()[i]
                            .to_bytes_be();
                        stack.push_front(&decommitment_bytes).unwrap();
                    }
                    stack
                        .push_front(&self.decommitment_values_len.to_bytes_be())
                        .unwrap();
                }
                stack
                    .push_front(&Felt::from(self.queries_count).to_bytes_be())
                    .unwrap();

                {
                    // Push Interaction Table Commitment last
                    let (stark_commitment, _) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                    let interaction_table_commitment = stark_commitment.traces.interaction;
                    let commitment_bytes = cast_struct_to_slice(&interaction_table_commitment);
                    stack.push_front(commitment_bytes).unwrap();
                }

                self.step = TracesDecommitStep::Done;
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            TracesDecommitStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == TracesDecommitStep::Done
    }
}
