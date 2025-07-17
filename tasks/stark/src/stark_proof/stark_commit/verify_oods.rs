use crate::felt::NonZeroFelt;
use crate::stark_proof::stark_commit::helpers::{
    ComputeDilutedProduct, ComputePeriodicColumns, ComputePublicMemoryProduct,
};
use crate::swiftness::air::recursive_with_poseidon::PUBLIC_MEMORY_STEP;
use crate::{felt::Felt, swiftness::stark::types::StarkProof};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

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

impl Executable for VerifyOods {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyOodsStep::PrepareEvaluation => {
                let proof: &StarkProof = stack.get_proof_reference();
                // Get OODS values count
                self.oods_values_count = proof.unsent_commitment.oods_values.len();

                // Push OODS values to stack (excluding last 2 which are composition values)
                for i in 0..(self.oods_values_count - 2) {
                    let proof: &StarkProof = stack.get_proof_reference();
                    let value = proof.unsent_commitment.oods_values.as_slice()[i];
                    stack.push_front(&value.to_bytes_be()).unwrap();
                }

                let proof: &StarkProof = stack.get_proof_reference();
                // Push the two composition values separately
                let comp_value_0 =
                    proof.unsent_commitment.oods_values.as_slice()[self.oods_values_count - 2];
                let comp_value_1 =
                    proof.unsent_commitment.oods_values.as_slice()[self.oods_values_count - 1];
                stack.push_front(&comp_value_1.to_bytes_be()).unwrap();
                stack.push_front(&comp_value_0.to_bytes_be()).unwrap();

                self.step = VerifyOodsStep::EvalCompositionPolynomial;

                // Call EvalCompositionPolynomial task
                vec![EvalCompositionPolynomial::new(self.oods_values_count - 2)
                    .to_vec_with_type_tag()]
            }

            VerifyOodsStep::EvalCompositionPolynomial => {
                // At this point, composition_from_trace is on the stack
                let composition_from_trace = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Get the two composition values
                let comp_value_0 = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let comp_value_1 = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

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
                unreachable!()
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

// Task for evaluating composition polynomial
#[derive(Debug, Clone)]
#[repr(C)]
pub struct EvalCompositionPolynomial {
    mask_values_count: usize,
    step: EvalCompositionStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalCompositionStep {
    CollectMaskValues,
    ComputePublicMemoryProduct,
    ComputeDilutedProduct,
    ComputePeriodicColumns,
    PrepareGlobalValues,
    EvalPolynomial,
    Done,
}

impl_type_identifiable!(EvalCompositionPolynomial);

impl EvalCompositionPolynomial {
    pub fn new(mask_values_count: usize) -> Self {
        Self {
            mask_values_count,
            step: EvalCompositionStep::CollectMaskValues,
        }
    }
}

impl Executable for EvalCompositionPolynomial {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            EvalCompositionStep::CollectMaskValues => {
                // Mask values are already on stack from VerifyOods
                self.step = EvalCompositionStep::ComputePublicMemoryProduct;
                vec![]
            }

            EvalCompositionStep::ComputePublicMemoryProduct => {
                let proof: &StarkProof = stack.get_proof_reference();

                // Get trace_domain_size from stack (should be there from validate_public_input)
                let trace_domain_size = Felt::from_bytes_be_slice(stack.borrow_front());

                // Calculate public memory column size
                let public_memory_column_size = trace_domain_size
                    .field_div(&NonZeroFelt::try_from(Felt::from(PUBLIC_MEMORY_STEP)).unwrap());

                // Push for public memory product calculation
                stack
                    .push_front(&public_memory_column_size.to_bytes_be())
                    .unwrap();

                self.step = EvalCompositionStep::ComputeDilutedProduct;

                // Call task to compute public memory product
                vec![ComputePublicMemoryProduct::new().to_vec_with_type_tag()]
            }

            EvalCompositionStep::ComputeDilutedProduct => {
                // Public memory product ratio is now on stack

                self.step = EvalCompositionStep::ComputePeriodicColumns;

                // Call task to compute diluted product
                vec![ComputeDilutedProduct::new().to_vec_with_type_tag()]
            }

            EvalCompositionStep::ComputePeriodicColumns => {
                // Diluted product is on stack

                self.step = EvalCompositionStep::PrepareGlobalValues;

                // Call task to compute periodic columns
                vec![ComputePeriodicColumns::new().to_vec_with_type_tag()]
            }

            EvalCompositionStep::PrepareGlobalValues => {
                // All computed values are on stack

                self.step = EvalCompositionStep::EvalPolynomial;

                // Prepare global values structure
                // vec![PrepareGlobalValues::new().to_vec_with_type_tag()]
                vec![]
            }

            EvalCompositionStep::EvalPolynomial => {
                // Call autogenerated eval_composition_polynomial_inner
                self.step = EvalCompositionStep::Done;

                // vec![EvalCompositionPolynomialInner::new().to_vec_with_type_tag()]
                vec![]
            }

            EvalCompositionStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == EvalCompositionStep::Done
    }
}
