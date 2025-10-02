use std::vec;
use utils::{
    impl_type_identifiable, BidirectionalStack, CacheStorage, Executable, FullProofDataVerifier3,
    ProofData, StarkVerifyTrait, TypeIdentifiable,
};

use crate::fri_verify::FriVerify;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkVerifyStep {
    FriVerify,
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
            step: StarkVerifyStep::FriVerify,
        }
    }
}

impl Default for StarkVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for StarkVerify {
    fn execute<
        T: BidirectionalStack + ProofData + StarkVerifyTrait + FullProofDataVerifier3 + CacheStorage,
    >(
        &mut self,
        _stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::FriVerify => {
                self.step = StarkVerifyStep::Done;
                println!("Pushing FriVerify task");
                vec![FriVerify::new().to_vec_with_type_tag()]
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
