use crate::felt::Felt;

pub mod get_hash;
pub mod hash_public_inputs;
pub mod stark_commit;
pub mod stark_verify;
pub mod validate_public_input;
pub mod verify;
pub mod verify_public_input;

// Constants for validation
pub const MAX_LOG_N_STEPS: Felt = Felt::from_hex_unchecked("0x50");
pub const MAX_RANGE_CHECK: Felt = Felt::from_hex_unchecked("0xffff");
pub const MAX_ADDRESS: usize = 0xffffffffffffffff;
pub const INITIAL_PC: usize = 1;

pub mod segments {
    pub const BITWISE: usize = 5;
    pub const EXECUTION: usize = 1;
    pub const N_SEGMENTS: usize = 7;
    pub const OUTPUT: usize = 2;
    pub const PEDERSEN: usize = 3;
    pub const POSEIDON: usize = 6;
    pub const PROGRAM: usize = 0;
    pub const RANGE_CHECK: usize = 4;
}
