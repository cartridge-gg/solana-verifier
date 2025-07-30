use stark::felt::Felt;
use stark::swiftness::transcript::TranscriptReadFelt;
use starknet_crypto::{poseidon_hash_many, Felt as StarknetFelt};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_transcript_read_felt() {
    let mut stack = BidirectionalStackAccount::default();

    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let val = Felt::from_hex("0x987654321").unwrap();

    // Expected result using reference function
    let expected_digest = read_felt_from_prover_reference(digest, val);
    let expected_counter = Felt::ZERO;

    TranscriptReadFelt::push_input(digest, val, &mut stack);

    stack.push_task(TranscriptReadFelt::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    // Get results from stack - should be counter (ZERO) and new digest
    let result_counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("Expected digest: {:?}", expected_digest);
    println!("Actual digest:   {:?}", result_digest);
    println!("Expected counter: {:?}", expected_counter);
    println!("Actual counter:   {:?}", result_counter);

    assert_eq!(
        result_digest, expected_digest,
        "Digest should match expected"
    );
    assert_eq!(result_counter, expected_counter, "Counter should be zero");
    assert!(steps > 0, "Should have executed at least one step");
}

// Reference implementation for verification
fn read_felt_from_prover_reference(digest: Felt, val: Felt) -> Felt {
    // Convert to StarknetFelt for hash calculation
    let digest_plus_one_starknet = StarknetFelt::from_bytes_be(&(digest + Felt::ONE).to_bytes_be());
    let val_starknet = StarknetFelt::from_bytes_be(&val.to_bytes_be());

    let hash_result = poseidon_hash_many(&[digest_plus_one_starknet, val_starknet]);

    // Convert back to our Felt
    Felt::from_bytes_be(&hash_result.to_bytes_be())
}
