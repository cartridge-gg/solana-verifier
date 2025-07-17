use stark::felt::Felt;
use stark::stark_proof::stark_commit::TableCommit;
use stark::swiftness::stark::types::StarkProof;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_table_commitment() {
    let mut stack = BidirectionalStackAccount::default();

    // Same data as in the example
    let unsent_commitment = Felt::from_hex_unchecked(
        "0x1e9b0fa29ebe52b9c9a43a1d44e555ce42da3199370134d758735bfe9f40269",
    );
    let transcript_digest = Felt::from_hex_unchecked(
        "0x1b9182dce9dc1169fcd00c1f8c0b6acd6baad99ce578370ead5ca230b8fb8c6",
    );
    let transcript_counter = Felt::from_hex_unchecked("0x1");

    // Expected results from the example
    let expected_digest = Felt::from_hex_unchecked(
        "0x1abd607dab09dede570ed131d9df0a1997e33735b11933c45dc84353df84259",
    );
    let expected_counter = Felt::from_hex_unchecked("0x0");

    // Create a StarkProof with the composition commitment
    let mut proof = StarkProof::default();
    proof.unsent_commitment.composition = unsent_commitment;
    stack.proof = proof;

    // Push transcript state (counter, digest) - same as in VectorCommit
    stack.push_front(&transcript_counter.to_bytes_be()).unwrap();
    stack.push_front(&transcript_digest.to_bytes_be()).unwrap();

    stack.push_task(TableCommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    println!("Executed {} steps", steps);
    println!("Final stack size: {}", stack.back_index - stack.front_index);

    // Get final transcript state (counter, digest)
    let final_counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let final_digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("Expected digest: {:?}", expected_digest);
    println!("Actual digest:   {:?}", final_digest);
    println!("Expected counter: {:?}", expected_counter);
    println!("Actual counter:   {:?}", final_counter);

    assert_eq!(
        final_digest, expected_digest,
        "Digest should match expected"
    );
    assert_eq!(
        final_counter, expected_counter,
        "Counter should be reset to 0"
    );
    assert!(steps > 0, "Should have executed at least one step");
}
