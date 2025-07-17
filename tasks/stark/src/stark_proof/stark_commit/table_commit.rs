use crate::stark_proof::stark_commit::traces_commit::VectorCommit;
use crate::{felt::Felt, swiftness::stark::types::StarkProof};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone, Copy)]
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

impl Executable for TableCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let proof: &StarkProof = stack.get_proof_reference();

        let composition_commitment = proof.unsent_commitment.composition;
        stack
            .push_front(&composition_commitment.to_bytes_be())
            .unwrap();

        self.processed = true;
        vec![VectorCommit::new().to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
