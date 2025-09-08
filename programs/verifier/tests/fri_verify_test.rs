mod fixtures;
use stark::swiftness::commitment;
use stark::swiftness::fri;
use stark::swiftness::stark::types::cast_struct_to_slice;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_fri_verify() {
    let mut stack: BidirectionalStackAccount = BidirectionalStackAccount::default();
    let task = stark::stark_proof::stark_verify::FriVerify::new();
    push_data(&mut stack);
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }
    assert_eq!(stack.front_index, 0);
    assert_eq!(stack.back_index, 65536);
}

// Stack layout pre-execution:
// ┌────────────────────────────────────────────────────────────┐
// │ Witness        (stark::swiftness::stark::types::Witness)   │  <- back (stack back)
// │ FRI Decommitment (stark::swiftness::fri::types::Decommitment)
// │ FRI Commitment   (stark::swiftness::fri::types::Commitment)|
// │ Queries         (stark::swiftness::fri::types::Queries)    │
// └────────────────────────────────────────────────────────────┘  <- front (stack front)
fn push_data(stack: &mut BidirectionalStackAccount) {
    let queries = fixtures::queries::get();
    let fri_commitment: stark::swiftness::fri::types::Commitment = fixtures::fri_commitment::get();
    let fri_decommitment: commitment::types::Decommitment = fixtures::fri_decommitment::get();
    let witness: commitment::types::Witness = fixtures::witness::get();
    let mut fri_verify_data = stark::swiftness::stark::types::FriVerifyData {
        queries: queries.clone(),
        fri_commitment: fri_commitment,
        fri_decommitment: fri_decommitment,
        witness: witness.clone(),
    };
    let fri_verify_data_bytes = cast_struct_to_slice(&mut fri_verify_data);
    stack.push_front(fri_verify_data_bytes).unwrap();
}
