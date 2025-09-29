use stark_verify_decommitments::stark_verify::StarkVerify;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStep {
    GenerateQueries,
    GenerateQueriesLoop,
    StarkVerify,
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
            step: VerifyStep::GenerateQueries,
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
            VerifyStep::GenerateQueries => {
                self.step = VerifyStep::GenerateQueriesLoop;
                vec![]
            }

            VerifyStep::GenerateQueriesLoop => {
                self.step = VerifyStep::StarkVerify;
                vec![]
            }
            VerifyStep::StarkVerify => {
                assert!(
                    stack.is_empty_front(),
                    "Stack should be empty before StarkVerify"
                );

                self.step = VerifyStep::Done;
                vec![StarkVerify::new(0, 0).to_vec_with_type_tag()]
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
