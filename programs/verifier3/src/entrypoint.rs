#![allow(unexpected_cfgs)]

use solana_program::entrypoint;

use crate::processor::process_instruction;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
