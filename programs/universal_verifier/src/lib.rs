// Export modules
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod scheduler;

// Re-export commonly used items
pub use instruction::UniversalVerifierInstruction;
pub use processor::process_instruction;
