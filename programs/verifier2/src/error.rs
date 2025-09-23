use std::io;
use std::num::TryFromIntError;

use thiserror::Error;

/// Custom errors for the verifier program
#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Account not owned by program")]
    InvalidOwner,

    #[error("Error deserializing scheduler")]
    SchedulerDeserializationError,

    #[error("Error pushing task to scheduler")]
    SchedulerTaskPushError,

    #[error("Error executing scheduler task")]
    SchedulerExecutionError,

    #[error("Error popping data from scheduler")]
    SchedulerDataPopError,

    #[error("Error serializing scheduler")]
    SchedulerSerializationError,

    #[error(transparent)]
    TryFromInt(#[from] TryFromIntError),

    /// The stack is empty and cannot be popped from.
    #[error("Empty stack - attempted to read from an empty stack")]
    EmptyStack,

    /// Error propagated from the stack module.
    #[error("Stack capacity exceeded")]
    StackCapacity,

    /// The task data length is invalid.
    #[error("Invalid task length - task data exceeds maximum allowed size")]
    InvalidTaskLength,

    /// Error during task execution.
    #[error("Execution error: {0}")]
    Execution(String),

    /// Error in task implementation.
    #[error("Task error: {0}")]
    Task(String),

    /// Error for invalid data.
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// General IO error.
    #[error(transparent)]
    Io(#[from] io::Error),
}
