use stark::felt::Felt;
use stark::swiftness::transcript::TranscriptRandomFelt;
use starknet_crypto::poseidon_hash;
use starknet_crypto::Felt as StarknetFelt;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_transcript_random_felt() {
    let mut stack = BidirectionalStackAccount::default();

    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let counter = Felt::from_hex("0x2").unwrap();

    // Expected result using reference function
    let expected_result = random_felt_to_prover_reference(digest, counter);

    stack.push_task(TranscriptRandomFelt::new(digest, counter));

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    // Get result from stack - should be the hash result
    let result = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("Expected: {:?}", expected_result);
    println!("Actual:   {:?}", result);

    assert_eq!(result, expected_result, "Random felt should match expected");
    assert!(steps > 0, "Should have executed at least one step");
}

// Reference implementation for verification
fn random_felt_to_prover_reference(digest: Felt, counter: Felt) -> Felt {
    // Convert to StarknetFelt for hash calculation
    let digest_starknet = StarknetFelt::from_bytes_be(&digest.to_bytes_be());
    let counter_starknet = StarknetFelt::from_bytes_be(&counter.to_bytes_be());

    let hash_result = poseidon_hash(digest_starknet, counter_starknet);

    // Convert back to our Felt
    Felt::from_bytes_be(&hash_result.to_bytes_be())
}
