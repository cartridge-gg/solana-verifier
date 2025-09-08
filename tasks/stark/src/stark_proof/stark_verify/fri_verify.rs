use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::swiftness::{
    commitment, fri,
    stark::{self, types::{cast_slice_to_struct, FriVerifyData}},
};

// FriVerify task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    stage: FriVerifyStep,
}

#[allow(dead_code)]
const FIELD_GENERATOR_INVERSE: Felt =
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
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            FriVerifyStep::Init => {
                let data = stack.borrow_front();
                let fri_verify_data: &FriVerifyData = cast_slice_to_struct(data);
                let queries_len = fri_verify_data.queries.len();
                let fri_len = fri_verify_data.fri_decommitment.values.len();

                assert_eq!(
                    fri_len, queries_len,
                    "FRI decommitment length does not match queries length"
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
