use stark_verify_fri::stark_verify::StarkVerify;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};
use verify_public_input::hash_public_input::VerifyPublicInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStep {
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
            step: VerifyStep::StarkVerify,
        }
    }
}

impl Default for Verify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for Verify {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyStep::StarkVerify => {
                assert!(
                    stack.is_empty_front(),
                    "Stack should be empty before StarkVerify"
                );

                self.step = VerifyStep::VerifyPublicInput;
                vec![StarkVerify::new().to_vec_with_type_tag()]
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
