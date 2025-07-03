use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::swiftness::air::domains::StarkDomains;
use crate::{
    felt::Felt,
    poseidon::PoseidonHashMany,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

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
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::Init => {
                println!("______");
                println!("StarkVerifyStep::Init");
                self.step = StarkVerifyStep::Output;
                vec![]
            }
            StarkVerifyStep::Output => {
                println!("StarkVerifyStep::Output");
                self.step = StarkVerifyStep::Program;
                vec![]
            }
            StarkVerifyStep::Program => {
                println!("StarkVerifyStep::Program");
                self.step = StarkVerifyStep::Done;
                vec![]
            }
            StarkVerifyStep::Done => {
                println!("StarkVerifyStep::Done");
                println!("______");
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == StarkVerifyStep::Done
    }
}
