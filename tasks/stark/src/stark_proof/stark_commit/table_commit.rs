use crate::stark_proof::stark_commit::traces_commit::VectorCommit;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};
#[derive(Debug, Clone)]
#[repr(C)]
pub struct TableCommit {
    processed: bool,
}

impl_type_identifiable!(TableCommit);

impl TableCommit {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for TableCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for TableCommit {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        self.processed = true;
        vec![VectorCommit::new().to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
