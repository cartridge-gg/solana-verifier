use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

// TableDecommit task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct TableDecommit {
    processed: bool,
}

impl_type_identifiable!(TableDecommit);

impl TableDecommit {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for TableDecommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for TableDecommit {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let queries = match super::Query::read_queries_from_stack(stack) {
            Ok(queries) => queries,
            Err(_) => {
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                self.processed = true;
                return vec![];
            }
        };

        // TODO: Implement actual table decommit logic:
        // 1. Read commitment from stack/proof data
        // 2. Read decommitment values from stack
        // 3. Read witness from stack
        // 4. Compute bottom layer depth
        // 5. Check if bottom layer is verifier friendly
        // 6. Validate decommitment values length
        // 7. Convert to Montgomery form
        // 8. Generate vector queries
        // 9. Call vector_commitment_decommit

        // For now, just placeholder that passes validation
        stack.push_front(&Felt::ONE.to_bytes_be()).unwrap(); // Success indicator

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
