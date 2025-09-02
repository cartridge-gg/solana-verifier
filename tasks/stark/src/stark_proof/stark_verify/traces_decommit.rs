use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

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
        let queries = match super::Query::read_queries_from_stack(stack) {
            Ok(queries) => queries,
            Err(_) => {
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                self.processed = true;
                return vec![];
            }
        };

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
