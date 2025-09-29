use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

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
    /// data we need atp: queries: &[Felt], commitment: FriCommitment,    decommitment: FriDecommitment, witness: Witness.
    fn execute<T: BidirectionalStack + ProofData>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        // FRI verify logic based on original:
        // fri_verify(
        //     queries,
        //     commitment.fri,
        //     fri_decommitment,
        //     witness.fri_witness.to_owned()
        // )?

        //         // TODO: Implement actual FRI verification logic:
        //         // 1. Read queries from stack
        //         // 2. Get FRI commitment from StarkCommitment
        //         // 3. Get FRI decommitment data (values, points)
        //         // 4. Get FRI witness from witness
        //         // 5. Perform FRI verification protocol
        //         // 6. This will likely involve multiple sub-tasks for:
        //         //    - Inner layer verification
        //         //    - Last layer verification
        //         //    - Vector commitment decommitments

        // For now, just return success

        // stack.push_front(&Felt::ONE.to_bytes_be()).unwrap(); // Success indicator

        match self.stage {
            FriVerifyStep::Init => {
                self.stage = FriVerifyStep::ComputeFirstLayer;
                vec![]
            }
            FriVerifyStep::ComputeFirstLayer => {
                self.stage = FriVerifyStep::ComputeFriGroup;
                vec![]
            }
            FriVerifyStep::ComputeFriGroup => {
                self.stage = FriVerifyStep::VerifyInnerLayers;
                vec![]
            }
            FriVerifyStep::VerifyInnerLayers => {
                self.stage = FriVerifyStep::VerifyLastLayer;
                vec![]
            }
            FriVerifyStep::VerifyLastLayer => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == FriVerifyStep::VerifyLastLayer
    }
}
