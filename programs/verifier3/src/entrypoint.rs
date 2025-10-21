#![allow(unexpected_cfgs)]

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
use crate::processor::process_instruction;

// Declare and export the program's entrypoint
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);
