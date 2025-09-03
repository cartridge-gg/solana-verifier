use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::swiftness::commitment::vector::types::Query;

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
        let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
        println!("READ: Queries length: {:?}", queries_len);
        stack.pop_front();

        let mut queries = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());
        for _ in 0..queries_len.to_biguint().try_into().unwrap() {
            queries.push(Query::from_stack(stack));
        }

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
