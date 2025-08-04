use felt::Felt;
use stark::stark_proof::stark_commit::VectorCommit;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_vector_commit() {
    let mut stack = BidirectionalStackAccount::default();

    // Reference test data from working implementation
    let commitment =
        Felt::from_hex("0x1e9b0fa29ebe52b9c9a43a1d44e555ce42da3199370134d758735bfe9f40269")
            .unwrap();
    let transcript_digest =
        Felt::from_hex("0x1b9182dce9dc1169fcd00c1f8c0b6acd6baad99ce578370ead5ca230b8fb8c6")
            .unwrap();
    // let transcript_counter = Felt::from_hex("0x1").unwrap();

    // Expected result
    let expected_digest =
        Felt::from_hex("0x1abd607dab09dede570ed131d9df0a1997e33735b11933c45dc84353df84259")
            .unwrap();
    let expected_counter = Felt::from_hex("0x0").unwrap(); // Counter should not change in this case

    // stack.push_front(&transcript_counter.to_bytes_be()).unwrap();
    stack.push_front(&transcript_digest.to_bytes_be()).unwrap();
    stack.push_front(&commitment.to_bytes_be()).unwrap();

    stack.push_task(VectorCommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    println!("Executed {} steps", steps);
    println!("Final stack size: {}", stack.back_index - stack.front_index);

    let final_counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let final_digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("Expected digest: {:?}", expected_digest);
    println!("Actual digest:   {:?}", final_digest);
    println!("Expected counter: {:?}", expected_counter);
    println!("Actual counter:   {:?}", final_counter);
    assert_eq!(
        final_counter, expected_counter,
        "Counter should match expected"
    );
    assert_eq!(
        final_digest, expected_digest,
        "Digest should match expected"
    );
    assert!(steps > 0, "Should have executed at least one step");
    assert!(stack.is_empty_back(), "Stack should be empty");
    assert!(stack.is_empty_front(), "Stack should be empty");
}
