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
    let elements_count = 3;

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

    let element0 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element1 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element2 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("element2: {}", element2);
    println!("element1: {}", element1);
    println!("element0: {}", element0);

    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");
    assert!(steps > 0, "Should have executed at least one step");
}
