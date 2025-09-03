use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::swiftness::commitment::vector::types::Query;

// TracesDecommit task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct TracesDecommit {
    processed: bool,
}

impl_type_identifiable!(TracesDecommit);

impl TracesDecommit {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for TracesDecommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for TracesDecommit {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
        println!("READ: Queries length: {:?}", queries_len);
        stack.pop_front();

        let mut queries = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());
        for _ in 0..queries_len.to_biguint().try_into().unwrap() {
            queries.push(Query::from_stack(stack));
        }
        // TODO: Implement actual traces decommit logic:
        // 1. Get traces commitment from StarkCommitment
        // 2. Get traces decommitment from witness
        // 3. Get traces witness from witness
        // 4. Call Layout::traces_decommit with queries slice
        // 5. This should call table_decommit for both original and interaction traces

        // Push queries back for next task using helper method
        super::Query::push_queries_to_stack(&queries, stack);

        // For now, just pass through
        stack.push_front(&Felt::ONE.to_bytes_be()).unwrap(); // Success indicator

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
