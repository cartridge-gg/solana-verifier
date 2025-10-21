#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct UnsentCommitment {
    pub nonce: u64,
}
