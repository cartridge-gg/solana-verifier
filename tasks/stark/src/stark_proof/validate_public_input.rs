use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::swiftness::air::domains::StarkDomains;
use crate::{
    felt::Felt,
    poseidon::PoseidonHashMany,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatePublicInputStep {
    Init,
    Output,
    Program,
    Done,
}
#[repr(C)]
pub struct ValidatePublicInput {
    step: ValidatePublicInputStep,
}

impl_type_identifiable!(ValidatePublicInput);

impl ValidatePublicInput {
    pub fn new() -> Self {
        Self {
            step: ValidatePublicInputStep::Init,
        }
    }
}

impl Default for ValidatePublicInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ValidatePublicInput {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            ValidatePublicInputStep::Init => {
                // let proof_reference: &mut [u8] = stack.get_proof_reference();
                // let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                // let public_input = &proof.public_input;
                // let stark_domains = StarkDomains::new(
                //     proof.config.log_trace_domain_size,
                //     proof.config.log_n_cosets,
                // );

                println!("______");
                println!("ValidatePublicInputStep::Init");
                self.step = ValidatePublicInputStep::Output;
                vec![]
            }
            ValidatePublicInputStep::Output => {
                println!("ValidatePublicInputStep::Output");
                self.step = ValidatePublicInputStep::Program;
                vec![]
            }
            ValidatePublicInputStep::Program => {
                println!("ValidatePublicInputStep::Program");
                self.step = ValidatePublicInputStep::Done;

                vec![]
            }
            ValidatePublicInputStep::Done => {
                println!("ValidatePublicInputStep::Done");
                println!("______");
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == ValidatePublicInputStep::Done
    }
}
