use crate::stark_proof::stark_commit::eval_composition_polynomial::EvalCompositionPolynomial;
use crate::swiftness::stark::types::StarkProof;
use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyOodsStep {
    PrepareEvaluation,
    EvalCompositionPolynomial,
    VerifyComposition,
    Done,
}

#[repr(C)]
pub struct VerifyOods {
    step: VerifyOodsStep,
    oods_values_count: usize,
}

impl_type_identifiable!(VerifyOods);

impl VerifyOods {
    pub fn new() -> Self {
        Self {
            step: VerifyOodsStep::PrepareEvaluation,
            oods_values_count: 0,
        }
    }
}

impl Default for VerifyOods {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for VerifyOods {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyOodsStep::PrepareEvaluation => {
                self.step = VerifyOodsStep::EvalCompositionPolynomial;

                vec![EvalCompositionPolynomial::new().to_vec_with_type_tag()]
            }

            VerifyOodsStep::EvalCompositionPolynomial => {
                // At this point, composition_from_trace is on the stack
                let composition_from_trace = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let proof: &StarkProof = stack.get_proof_reference();
                // Push the two composition values separately
                let comp_value_0 =
                    proof.unsent_commitment.oods_values.as_slice()[self.oods_values_count - 2];
                let comp_value_1 =
                    proof.unsent_commitment.oods_values.as_slice()[self.oods_values_count - 1];

                // Get oods_point (interaction_after_composition from stack)
                let oods_point = Felt::from_bytes_be_slice(stack.borrow_front());

                // Calculate claimed_composition = comp_value_0 + comp_value_1 * oods_point
                let claimed_composition = comp_value_0 + comp_value_1 * oods_point;

                // Verify they match
                assert!(
                    composition_from_trace == claimed_composition,
                    "OODS evaluation invalid: expected {:?}, got {:?}",
                    claimed_composition,
                    composition_from_trace
                );

                self.step = VerifyOodsStep::Done;
                vec![]
            }

            VerifyOodsStep::VerifyComposition => {
                // This step is merged into EvalCompositionPolynomial for efficiency
                vec![]
            }

            VerifyOodsStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VerifyOodsStep::Done
    }
}
