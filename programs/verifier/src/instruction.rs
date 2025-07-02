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
