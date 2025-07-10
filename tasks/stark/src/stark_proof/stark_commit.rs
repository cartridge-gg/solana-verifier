use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkCommitStep {
    Init,
    Output,
    Program,
    Done,
}
#[repr(C)]
pub struct StarkCommit {
    step: StarkCommitStep,
}

impl_type_identifiable!(StarkCommit);

impl StarkCommit {
    pub fn new() -> Self {
        Self {
            step: StarkCommitStep::Init,
        }
    }
}

impl Default for StarkCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for StarkCommit {
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            StarkCommitStep::Init => {
                self.step = StarkCommitStep::Output;
                vec![]
            }
            StarkCommitStep::Output => {
                self.step = StarkCommitStep::Program;
                vec![]
            }
            StarkCommitStep::Program => {
                self.step = StarkCommitStep::Done;
                vec![]
            }
            StarkCommitStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == StarkCommitStep::Done
    }
}
