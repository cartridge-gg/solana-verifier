pub mod points_x;
pub mod points_y;

use felt::Felt;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkCommitmentTrait,
    TypeIdentifiable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PedersenEvalPhase {
    PointsX,
    PointsY,
    Done,
}

impl_type_identifiable!(PedersenEval);
#[repr(C)]
pub struct PedersenEval {
    step: PedersenEvalPhase,
    pedersen_point: Felt,
}

impl PedersenEval {
    pub fn new(pedersen_point: Felt) -> Self {
        Self {
            step: PedersenEvalPhase::PointsX,
            pedersen_point,
        }
    }
}

impl Default for PedersenEval {
    fn default() -> Self {
        Self::new(Felt::ZERO)
    }
}

impl Executable for PedersenEval {
    fn execute<T: BidirectionalStack + ProofData + StarkCommitmentTrait>(
        &mut self,
        _stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match &self.step {
            PedersenEvalPhase::PointsX => {
                self.step = PedersenEvalPhase::PointsY;
                vec![points_x::PedersenPointsX::new(self.pedersen_point).to_vec_with_type_tag()]
            }
            PedersenEvalPhase::PointsY => {
                self.step = PedersenEvalPhase::Done;
                vec![points_y::PedersenPointsY::new(self.pedersen_point).to_vec_with_type_tag()]
            }
            PedersenEvalPhase::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == PedersenEvalPhase::Done
    }
}
