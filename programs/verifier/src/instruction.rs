use borsh::{BorshDeserialize, BorshSerialize};

/// Instructions supported by the verifier program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VerifierInstruction {
    /// Sets the data at the given offset in the verifier account
    SetAccountData(usize, Vec<u8>),

    /// Pushes a task to the verifier account's bidirectional stack
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    PushTask(Vec<u8>),

    /// Pushes data to the verifier account's bidirectional stack
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    PushData(Vec<u8>),

    /// Pushes a chunk of data to the verifier account's bidirectional stack without appending the length.
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    ///
    /// This instruction allows pushing part of a large data buffer. Call multiple times for each chunk.
    PushDataChunk(Vec<u8>),

    /// Completes the chunked data push by appending the total length of the data.
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    ///
    /// Call this after all chunks have been pushed, passing the total size of the data.
    PushDataChunkComplete(u32),

    /// Executes the next task in the verifier account's bidirectional stack
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    Execute(u32),

    /// Closes the verifier account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    Close,
}
