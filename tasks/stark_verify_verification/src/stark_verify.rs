use std::vec;

use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkVerifyStep {
    ComputeQueryPoints,
    EvalOodsBoundaryPoly,
    FriVerify,
    Done,
}

#[repr(C)]
pub struct StarkVerify {
    step: StarkVerifyStep,
    n_original_columns: u32,
    n_interaction_columns: u32,
    queries_len: u128,
}

impl_type_identifiable!(StarkVerify);

impl StarkVerify {
    pub fn new(n_original_columns: u32, n_interaction_columns: u32) -> Self {
        Self {
            step: StarkVerifyStep::ComputeQueryPoints,
            n_original_columns,
            n_interaction_columns,
            queries_len: 0,
        }
    }
}

impl Default for StarkVerify {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Executable for StarkVerify {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        _stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            StarkVerifyStep::ComputeQueryPoints => {
                // Query points computed, evaluate OODS boundary poly
                self.step = StarkVerifyStep::EvalOodsBoundaryPoly;
                // vec![EvalOodsBoundaryPolyAtPoints::new(
                //     self.n_original_columns,
                //     self.n_interaction_columns,
                // )
                // .to_vec_with_type_tag()]
                vec![]
            }

            StarkVerifyStep::EvalOodsBoundaryPoly => {
                // OODS evaluation finished, start FRI verification
                self.step = StarkVerifyStep::FriVerify;
                // vec![FriVerify::new().to_vec_with_type_tag()]
                vec![]
            }

            StarkVerifyStep::FriVerify => {
                // FRI verification finished, read result
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
