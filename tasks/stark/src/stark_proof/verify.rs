use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::stark_proof::get_hash::GetHash;
use crate::stark_proof::stark_commit::StarkCommit;
use crate::stark_proof::stark_verify::StarkVerify;
use crate::stark_proof::validate_public_input::ValidatePublicInput;
use crate::stark_proof::verify_public_input::VerifyPublicInput;
use crate::swiftness::air::domains::StarkDomains;
use crate::{
    felt::Felt,
    poseidon::PoseidonHashMany,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStep {
    ValidatePublicInput,
    GetHash,
    StarkCommit,
    StarkVerify,
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
                self.step = VerifyStep::GetHash;
                println!("___ValidatePublicInput___");
                vec![ValidatePublicInput::new().to_vec_with_type_tag()]
            }
            VerifyStep::GetHash => {
                self.step = VerifyStep::StarkCommit;
                println!("___GetHash___");
                vec![GetHash::new(Felt::ZERO).to_vec_with_type_tag()]
            }
            VerifyStep::StarkCommit => {
                self.step = VerifyStep::StarkVerify;
                println!("___StarkCommit___");
                // vec![StarkCommit::new().to_vec_with_type_tag()]
                vec![]
            }
            VerifyStep::StarkVerify => {
                self.step = VerifyStep::VerifyPublicInput;
                println!("___StarkVerify___");
                // vec![StarkVerify::new().to_vec_with_type_tag()]
                vec![]
            }
            VerifyStep::VerifyPublicInput => {
                self.step = VerifyStep::Done;
                println!("___VerifyPublicInput___");
                // vec![VerifyPublicInput::new().to_vec_with_type_tag()]
                vec![]
            }
            VerifyStep::Done => {
                println!("___Done___");
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VerifyStep::Done
    }
}
