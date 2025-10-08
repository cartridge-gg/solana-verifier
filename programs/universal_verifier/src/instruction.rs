use borsh::{BorshDeserialize, BorshSerialize};
use utils::VerifierMode;

/// Verifier instruction enum (same as verifier programs)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VerifierInstruction {
    SetAccountData(usize, Vec<u8>),
    PushTask(Vec<u8>),
    PushData(Vec<u8>),
    Execute(u32),
    TestExecute,
    Close,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum UniversalVerifierInstruction {
    /// Switch the verifier mode
    /// Accounts expected:
    /// 0. `[writable]` Universal stack account
    SwitchMode(VerifierMode),

    /// Set account data at offset
    /// Accounts expected:
    /// 0. `[writable]` Universal stack account
    SetAccountData(usize, Vec<u8>),

    /// Push a task to the back of the stack
    /// Accounts expected:
    /// 0. `[writable]` Universal stack account
    PushTask(Vec<u8>),

    /// Push data to the front of the stack
    /// Accounts expected:
    /// 0. `[writable]` Universal stack account
    PushData(Vec<u8>),

    /// Execute the next task in the stack
    /// Accounts expected:
    /// 0. `[writable]` Universal stack account
    Execute(u32), // nonce for tracking

    /// Execute with specific verifier program ID
    /// Accounts expected:
    /// 0. `[writable]` Universal stack account
    /// 1. `[]` Verifier program account
    ExecuteWithProgramId(u32, [u8; 32]), // nonce and program ID

    /// Test CPI by calling verifier1's TestExecute
    /// Accounts expected:
    /// 0. `[writable]` Universal stack account
    /// 1. `[]` Verifier1 program account
    TestCPI([u8; 32]), // verifier1 program ID

    /// Close the account and return lamports
    /// Accounts expected:
    /// 0. `[writable]` Target account to close
    /// 1. `[writable]` Payer account (receives lamports)
    /// 2. `[]` System program
    Close,
}
