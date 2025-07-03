use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::swiftness::air::domains::StarkDomains;
use crate::{
    felt::Felt,
    poseidon::PoseidonHashMany,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

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
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            StarkCommitStep::Init => {
                println!("______");
                println!("StarkCommitStep::Init");
                self.step = StarkCommitStep::Output;
                vec![]
            }
            StarkCommitStep::Output => {
                println!("StarkCommitStep::Output");
                self.step = StarkCommitStep::Program;
                vec![]
            }
            StarkCommitStep::Program => {
                println!("StarkCommitStep::Program");
                self.step = StarkCommitStep::Done;
                vec![]
            }
            StarkCommitStep::Done => {
                println!("StarkCommitStep::Done");
                println!("______");
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == StarkCommitStep::Done
    }
}
