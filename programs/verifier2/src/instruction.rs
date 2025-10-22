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

    /// Copies data from another account (owned by different program)
    ///
    /// Accounts expected:
    /// 0. `[]` Source account (readonly, can be owned by any program)
    /// 1. `[writable]` Destination account (must be owned by this program)
    CopyFromAccount,

    /// Transfers ownership of account to a new program
    ///
    /// Accounts expected:
    /// 0. `[writable]` The account to transfer
    /// 1. `[]` The new owner program ID
    TransferOwnership,

    /// Closes the verifier account
    ///
    /// Accounts expected:
    /// 0. `[writable]` The verifier account
    Close,
}
