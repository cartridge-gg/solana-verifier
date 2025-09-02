use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

// FriVerify task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriVerify {
    processed: bool,
}

impl_type_identifiable!(FriVerify);

impl FriVerify {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for FriVerify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriVerify {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // FRI verify logic based on original:
        // fri_verify(
        //     queries,
        //     commitment.fri,
        //     fri_decommitment,
        //     witness.fri_witness.to_owned()
        // )?

        // TODO: Implement actual FRI verification logic:
        // 1. Read queries from stack
        // 2. Get FRI commitment from StarkCommitment
        // 3. Get FRI decommitment data (values, points)
        // 4. Get FRI witness from witness
        // 5. Perform FRI verification protocol
        // 6. This will likely involve multiple sub-tasks for:
        //    - Inner layer verification
        //    - Last layer verification
        //    - Vector commitment decommitments

        // For now, just return success
        stack.push_front(&Felt::ONE.to_bytes_be()).unwrap(); // Success indicator

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
