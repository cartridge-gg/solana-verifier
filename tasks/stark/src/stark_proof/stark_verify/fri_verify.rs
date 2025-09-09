use felt::{Felt, NonZeroFelt};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::{
    funvec::FunVec,
    swiftness::{
        fri::{self, layer::FriLayerQuery, types::FriVerifyInput},
        stark::types::{cast_slice_to_struct, cast_struct_to_slice},
    },
};

// FriVerify task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    stage: FriVerifyStep,
}

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
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            FriVerifyStep::Init => {
                let data = stack.borrow_front();
                let input: &FriVerifyInput = cast_slice_to_struct::<FriVerifyInput>(data);
                let queries_len = input.queries.len();
                let decommitment_len = input.fri_decommitment.values.len();
                assert_eq!(
                    queries_len, decommitment_len,
                    "Queries length and decommitment length must be equal"
                );
                self.stage = FriVerifyStep::ComputeFirstLayer;
                println!("Transitioning to ComputeFirstLayer");
                vec![]
            }
            FriVerifyStep::ComputeFirstLayer => {
                let data = stack.borrow_front();
                let input: &FriVerifyInput = cast_slice_to_struct::<FriVerifyInput>(data);
                let queries = &input.queries;
                let values = &input.fri_decommitment.values;
                let points = &input.fri_decommitment.points;
                let mut result = Vec::new();
                for (i, query) in queries.iter().enumerate() {
                    let shifted_x_value = points[i].to_owned() * FIELD_GENERATOR_INVERSE;
                    result.push(FriLayerQuery {
                        index: *query,
                        y_value: values[i].to_owned(),
                        x_inv_value: Felt::ONE
                            .field_div(&NonZeroFelt::from_felt_unchecked(shifted_x_value)),
                    });
                }
                // Take out the input data from the stack and push the result
                stack.pop_front();
                stack.push_front(cast_struct_to_slice(&mut result)).unwrap();
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

// Returns the elements of the multiplicative subgroup of order 16, in bit-reversed order for the
// cairo prime field. Note that the first 2^k elements correspond to the group of size 2^k.
pub fn get_fri_group() -> FunVec<Felt, 16> {
    FunVec::from_vec(vec![
        Felt::from_hex_unchecked("0x1"),
        Felt::from_hex_unchecked(
            "0x800000000000011000000000000000000000000000000000000000000000000",
        ),
        Felt::from_hex_unchecked(
            "0x625023929a2995b533120664329f8c7c5268e56ac8320da2a616626f41337e3",
        ),
        Felt::from_hex_unchecked(
            "0x1dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc81e",
        ),
        Felt::from_hex_unchecked(
            "0x63365fe0de874d9c90adb1e2f9c676e98c62155e4412e873ada5e1dee6feebb",
        ),
        Felt::from_hex_unchecked(
            "0x1cc9a01f2178b3736f524e1d06398916739deaa1bbed178c525a1e211901146",
        ),
        Felt::from_hex_unchecked(
            "0x3b912c31d6a226e4a15988c6b7ec1915474043aac68553537192090b43635cd",
        ),
        Felt::from_hex_unchecked(
            "0x446ed3ce295dda2b5ea677394813e6eab8bfbc55397aacac8e6df6f4bc9ca34",
        ),
        Felt::from_hex_unchecked(
            "0x5ec467b88826aba4537602d514425f3b0bdf467bbf302458337c45f6021e539",
        ),
        Felt::from_hex_unchecked(
            "0x213b984777d9556bac89fd2aebbda0c4f420b98440cfdba7cc83ba09fde1ac8",
        ),
        Felt::from_hex_unchecked(
            "0x5ce3fa16c35cb4da537753675ca3276ead24059dddea2ca47c36587e5a538d1",
        ),
        Felt::from_hex_unchecked(
            "0x231c05e93ca34c35ac88ac98a35cd89152dbfa622215d35b83c9a781a5ac730",
        ),
        Felt::from_hex_unchecked(
            "0x00b54759e8c46e1258dc80f091e6f3be387888015452ce5f0ca09ce9e571f52",
        ),
        Felt::from_hex_unchecked(
            "0x7f4ab8a6173b92fda7237f0f6e190c41c78777feabad31a0f35f63161a8e0af",
        ),
        Felt::from_hex_unchecked(
            "0x23c12f3909539339b83645c1b8de3e14ebfee15c2e8b3ad2867e3a47eba558c",
        ),
        Felt::from_hex_unchecked(
            "0x5c3ed0c6f6ac6dd647c9ba3e4721c1eb14011ea3d174c52d7981c5b8145aa75",
        ),
    ])
}
