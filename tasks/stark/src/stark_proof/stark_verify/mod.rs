use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

pub mod vector_decommit;

// Re-export the new task types
// pub use vector_decommit::{VectorDecommit, VectorCommitmentDecommit, ComputeRootFromQueries};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkVerifyStep {
    Init,
    Output,
    Program,
    Done,
}
#[repr(C)]
pub struct StarkVerify {
    step: StarkVerifyStep,
}

impl_type_identifiable!(StarkVerify);

impl StarkVerify {
    pub fn new() -> Self {
        Self {
            step: StarkVerifyStep::Init,
        }
    }
}

impl Default for StarkVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for StarkVerify {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::Init => {
                self.step = StarkVerifyStep::Output;
                vec![]
            }
            StarkVerifyStep::Output => {
                self.step = StarkVerifyStep::Program;
                vec![]
            }
            StarkVerifyStep::Program => {
                self.step = StarkVerifyStep::Done;
                vec![]
            }
            StarkVerifyStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == StarkVerifyStep::Done
    }
}
