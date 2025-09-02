use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::stark_proof::get_hash::GetHash;
use crate::stark_proof::stark_commit::StarkCommit;
use crate::stark_proof::stark_verify::StarkVerify;
use crate::stark_proof::validate_public_input::ValidatePublicInput;
use crate::stark_proof::VerifyPublicInput;
use felt::Felt;

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
    fn execute<T: BidirectionalStack + ProofData>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyStep::ValidatePublicInput => {
                self.step = VerifyStep::GetHash;
                vec![ValidatePublicInput::new().to_vec_with_type_tag()]
            }
            VerifyStep::GetHash => {
                self.step = VerifyStep::StarkCommit;
                vec![GetHash::new(Felt::ZERO).to_vec_with_type_tag()]
            }
            VerifyStep::StarkCommit => {
                self.step = VerifyStep::StarkVerify;
                vec![StarkCommit::new().to_vec_with_type_tag()]
            }
            VerifyStep::StarkVerify => {
                self.step = VerifyStep::VerifyPublicInput;
                vec![StarkVerify::new(0, 0).to_vec_with_type_tag()]
            }
            VerifyStep::VerifyPublicInput => {
                self.step = VerifyStep::Done;
                vec![VerifyPublicInput::new().to_vec_with_type_tag()]
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
