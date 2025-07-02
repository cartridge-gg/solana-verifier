use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::stark_proof::validate_public_input::ValidatePublicInput;
use crate::swiftness::air::domains::StarkDomains;
use crate::{
    felt::Felt,
    poseidon::PoseidonHashMany,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStep {
    ValidatePublicInput,
    StarkCommit,
    StarkVerify,
    GetHash,
    VerifyPublicInput,
    Done,
}

#[repr(C)]
pub struct Verify {
    step: VerifyStep,
}

impl_type_identifiable!(Verify);

impl Verify {
    pub fn new() -> Self {
        Self {
            step: VerifyStep::ValidatePublicInput,
        }
    }
}

impl Default for Verify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for Verify {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyStep::ValidatePublicInput => {
                self.step = VerifyStep::StarkCommit;
                vec![ValidatePublicInput::new().to_vec_with_type_tag()]
            }
            VerifyStep::StarkCommit => {
                self.step = VerifyStep::StarkVerify;
                vec![]
            }
            VerifyStep::StarkVerify => {
                self.step = VerifyStep::GetHash;
                vec![]
            }
            VerifyStep::GetHash => {
                self.step = VerifyStep::VerifyPublicInput;
                vec![]
            }
            VerifyStep::VerifyPublicInput => {
                self.step = VerifyStep::Done;
                // vec![VerifyPublicInput::new().to_vec_with_type_tag()]
                vec![]
            }
            VerifyStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VerifyStep::Done
    }
}
