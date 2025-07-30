use stark::felt::Felt;
use stark::swiftness::transcript::TranscriptReadFeltVector;
use starknet_crypto::{poseidon_hash_many, Felt as StarknetFelt};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_transcript_read_felt_vector() {
    let mut stack = BidirectionalStackAccount::default();

    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let values = vec![
        Felt::from_hex("0x111").unwrap(),
        Felt::from_hex("0x222").unwrap(),
        Felt::from_hex("0x333").unwrap(),
    ];

    // Expected result using reference function
    let expected_digest = read_felt_vector_from_prover_reference(digest, &values);
    let expected_counter = Felt::ZERO;

    TranscriptReadFeltVector::push_input(digest, &values, &mut stack);

    stack.push_task(TranscriptReadFeltVector::new(values.len()));

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
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
    assert_eq!(stack.back_index, 65536, "Stack should be empty after test");
}

// Reference implementation for verification
fn read_felt_vector_from_prover_reference(digest: Felt, values: &[Felt]) -> Felt {
    // Convert to StarknetFelt for hash calculation
    let mut starknet_values = Vec::new();

    // Add digest + 1 as first element
    let digest_plus_one = digest + Felt::ONE;
    starknet_values.push(StarknetFelt::from_bytes_be(&digest_plus_one.to_bytes_be()));

    // Add all values
    for val in values {
        starknet_values.push(StarknetFelt::from_bytes_be(&val.to_bytes_be()));
    }

    let hash_result = poseidon_hash_many(&starknet_values);

    // Convert back to our Felt
    Felt::from_bytes_be(&hash_result.to_bytes_be())
}
