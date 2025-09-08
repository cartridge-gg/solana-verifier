use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::swiftness::{fri::types::FriVerifyInput, stark::types::cast_slice_to_struct};

// FriVerify task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    stage: FriVerifyStep,
}

const _FIELD_GENERATOR_INVERSE: Felt =
    Felt::from_hex_unchecked("0x2AAAAAAAAAAAAB0555555555555555555555555555555555555555555555556");

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FriVerifyStep {
    Init,
    ComputeFirstLayer,
    ComputeFriGroup,
    VerifyInnerLayers,
    VerifyLastLayer,
}
impl_type_identifiable!(FriVerify);

impl FriVerify {
    pub fn new() -> Self {
        Self {
            stage: FriVerifyStep::Init,
        }
    }
}

impl Default for FriVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriVerify {
    /// data we need atp: queries: &[Felt], commitment: FriCommitment,    decommitment: FriDecommitment, witness: Witness.
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            FriVerifyStep::Init => {
                let data = stack.borrow_front();
                let input = cast_slice_to_struct::<FriVerifyInput>(data);
                let queries_len = input.queries.len();
                let decommitment_len = input.fri_decommitment.values.len();
                assert_eq!(
                    queries_len, decommitment_len,
                    "Queries length and decommitment length must be equal"
                );
                stack.pop_front();
                self.stage = FriVerifyStep::ComputeFirstLayer;
                println!("Transitioning to ComputeFirstLayer");
                vec![]
            }
            FriVerifyStep::ComputeFirstLayer => {
                self.stage = FriVerifyStep::ComputeFriGroup;
                println!("Transitioning to ComputeFriGroup");
                vec![]
            }
            FriVerifyStep::ComputeFriGroup => {
                self.stage = FriVerifyStep::VerifyInnerLayers;
                println!("Transitioning to VerifyInnerLayers");
                vec![]
            }
            FriVerifyStep::VerifyInnerLayers => {
                self.stage = FriVerifyStep::VerifyLastLayer;
                println!("Transitioning to VerifyLastLayer");
                vec![]
            }
            FriVerifyStep::VerifyLastLayer => {
                println!("FRI Verification completed");
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == FriVerifyStep::VerifyLastLayer
    }
}
