use felt::Felt;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

use crate::table_decommit::TableDecommit;
use types::swiftness::global_values::InteractionElements;
use types::swiftness::stark::types::{
    cast_struct_to_slice, StarkCommitment, StarkProof, VerifyVariables,
};

// TracesDecommit task phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracesDecommitStep {
    PrepareOriginalTable,
    PrepareOriginalWitnessAuth,
    PrepareOriginalValues,
    PrepareOriginalCommitment,
    PrepareInteractionTable,
    PrepareInteractionWitnessAuth,
    PrepareInteractionValues,
    PrepareInteractionCommitment,
    Done,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TracesDecommit {
    step: TracesDecommitStep,
    queries_count: usize,
    decommitment_values_len: Felt,
    witness_count: usize,
}

impl_type_identifiable!(TracesDecommit);

impl TracesDecommit {
    pub fn new() -> Self {
        Self {
            step: TracesDecommitStep::PrepareOriginalTable,
            queries_count: 0,
            decommitment_values_len: Felt::ZERO,
            witness_count: 0,
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
                }

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

                println!(
                    "DEBUG: decommitment_values_len: {:?}",
                    self.decommitment_values_len
                );
                println!("DEBUG: witness_count: {:?}", self.witness_count);

                self.step = TracesDecommitStep::PrepareOriginalWitnessAuth;
                vec![]
            }

            TracesDecommitStep::PrepareOriginalWitnessAuth => {
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

                self.step = TracesDecommitStep::PrepareOriginalValues;
                vec![]
            }

            TracesDecommitStep::PrepareOriginalValues => {
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

                stack
                    .push_front(&Felt::from(self.queries_count).to_bytes_be())
                    .unwrap();

                self.step = TracesDecommitStep::PrepareOriginalCommitment;
                vec![]
            }

            TracesDecommitStep::PrepareOriginalCommitment => {
                let (stark_commitment, _) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                let original_table_commitment = stark_commitment.traces.original;
                let commitment_bytes = cast_struct_to_slice(&original_table_commitment);
                stack.push_front(commitment_bytes).unwrap();

                self.step = TracesDecommitStep::PrepareInteractionTable;
                vec![TableDecommit::new().to_vec_with_type_tag()]
            }

            TracesDecommitStep::PrepareInteractionTable => {
                // Get interaction metadata
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

                self.step = TracesDecommitStep::PrepareInteractionWitnessAuth;
                vec![]
            }

            TracesDecommitStep::PrepareInteractionWitnessAuth => {
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

                self.step = TracesDecommitStep::PrepareInteractionValues;
                vec![]
            }

            TracesDecommitStep::PrepareInteractionValues => {
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

                stack
                    .push_front(&Felt::from(self.queries_count).to_bytes_be())
                    .unwrap();

                self.step = TracesDecommitStep::PrepareInteractionCommitment;
                vec![]
            }

            TracesDecommitStep::PrepareInteractionCommitment => {
                let (stark_commitment, _) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();
                let interaction_table_commitment = stark_commitment.traces.interaction;
                let commitment_bytes = cast_struct_to_slice(&interaction_table_commitment);
                stack.push_front(commitment_bytes).unwrap();

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
