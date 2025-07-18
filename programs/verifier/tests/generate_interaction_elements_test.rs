use stark::felt::Felt;
use stark::stark_proof::stark_commit::GenerateInteractionElements;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_generate_interaction_elements() {
    let mut stack = BidirectionalStackAccount::default();

    // Setup transcript state
    let digest = Felt::from_hex("0x123").unwrap();
    let counter = Felt::ZERO;
    let elements_count = 2;

    // Push transcript state (counter, digest)
    stack.push_front(&counter.to_bytes_be()).unwrap();
    stack.push_front(&digest.to_bytes_be()).unwrap();

    let interaction_task = GenerateInteractionElements::new(elements_count);
    stack.push_task(interaction_task);

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    // Should have: [final_counter, final_digest, element1, element0]
    let final_counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let final_digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    // Should have generated 2 elements
    let _element1 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let _element0 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    assert_eq!(
        final_counter,
        Felt::from(elements_count),
        "Counter should be incremented by element count"
    );
    assert_eq!(final_digest, digest, "Digest should be preserved");
    assert!(steps > 0, "Should have executed at least one step");
}
