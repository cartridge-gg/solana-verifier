use stark::felt::Felt;
use stark::stark_proof::stark_commit::TracesCommit;
use stark::swiftness::air::trace::UnsentCommitment;
use stark::swiftness::stark::types::{StarkProof, StarkUnsentCommitment};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

//this test is not working yet but its a good starting point
#[test]
fn test_traces_commit_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    // Create a StarkProof with reference trace commitments
    let mut proof = StarkProof::default();

    // Reference values from the test output
    let original_commitment =
        Felt::from_hex("0x2a588e8517b956684162e05e373dc6891146c1853c82d3984fbc707ae937972")
            .unwrap();
    let interaction_commitment =
        Felt::from_hex("0x7171ffc67e24fcbb2a7d1acd6244fa91c54dff15c96ca26d193907b716ce2c5")
            .unwrap();

    proof.unsent_commitment.traces = UnsentCommitment {
        original: original_commitment,
        interaction: interaction_commitment,
    };

    stack.proof = proof;

    // Initial transcript state matching the reference test
    let initial_transcript_digest =
        Felt::from_hex("0x1b9182dce9dc1169fcd00c1f8c0b6acd6baad99ce578370ead5ca230b8fb8c6")
            .unwrap();
    let initial_transcript_counter = Felt::from_hex("0x1").unwrap();

    stack
        .push_front(&initial_transcript_counter.to_bytes_be())
        .unwrap();
    stack
        .push_front(&initial_transcript_digest.to_bytes_be())
        .unwrap();

    stack.push_task(TracesCommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    let final_counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let final_digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    // Verify final transcript values
    assert_eq!(
        final_counter,
        Felt::from_hex("0x7").unwrap(),
        "Final counter mismatch"
    );
    assert_eq!(
        final_digest,
        Felt::from_hex("0x76dd10bb913bf5c08e91dc51f97e0fa0bb18a4ee99adbbb80e1c84c2f67e78a")
            .unwrap(),
        "Final digest mismatch"
    );

    let memory_multi_column_perm_perm_interaction_elm =
        Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let memory_multi_column_perm_hash_interaction_elm0 =
        Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let range_check16_perm_interaction_elm = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let diluted_check_permutation_interaction_elm = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let diluted_check_interaction_z = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let diluted_check_interaction_alpha = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    // Assert all expected interaction elements
    assert_eq!(
        memory_multi_column_perm_perm_interaction_elm,
        expected_memory_multi_column_perm_perm_interaction_elm,
        "Memory multi column perm interaction element mismatch"
    );
    assert_eq!(
        memory_multi_column_perm_hash_interaction_elm0,
        expected_memory_multi_column_perm_hash_interaction_elm0,
        "Memory multi column hash interaction element mismatch"
    );
    assert_eq!(
        range_check16_perm_interaction_elm, expected_range_check16_perm_interaction_elm,
        "Range check 16 perm interaction element mismatch"
    );
    assert_eq!(
        diluted_check_permutation_interaction_elm,
        expected_diluted_check_permutation_interaction_elm,
        "Diluted check permutation interaction element mismatch"
    );
    assert_eq!(
        diluted_check_interaction_z, expected_diluted_check_interaction_z,
        "Diluted check interaction z mismatch"
    );
    assert_eq!(
        diluted_check_interaction_alpha, expected_diluted_check_interaction_alpha,
        "Diluted check interaction alpha mismatch"
    );

    // Reference values from the test output
    let expected_original_commitment =
        Felt::from_hex("0x2a588e8517b956684162e05e373dc6891146c1853c82d3984fbc707ae937972")
            .unwrap();
    let expected_interaction_commitment =
        Felt::from_hex("0x7171ffc67e24fcbb2a7d1acd6244fa91c54dff15c96ca26d193907b716ce2c5")
            .unwrap();

    // Expected interaction elements from the test output
    let expected_memory_multi_column_perm_perm_interaction_elm =
        Felt::from_hex("0x617916729dd4132da40d4c38330a344a4704c284a3c4b36924b4d7603522a62")
            .unwrap();
    let expected_memory_multi_column_perm_hash_interaction_elm0 =
        Felt::from_hex("0x1465794e32fdae09c582f92d227c7764c344f98ee680235459cf3c962a929e3")
            .unwrap();
    let expected_range_check16_perm_interaction_elm =
        Felt::from_hex("0x74ce496ecec64eada712b17dc981af96d402937d655f05a3f608084c876e29b")
            .unwrap();
    let expected_diluted_check_permutation_interaction_elm =
        Felt::from_hex("0x5c3e5df5894a8a28ccd319646fe8867bed69e4c6fbb1a515f7a44ca89a14bbc")
            .unwrap();
    let expected_diluted_check_interaction_z =
        Felt::from_hex("0x19e69143def2003b8c2254413a58b6c573f03448bcb1ad9a4a0c66077683f15")
            .unwrap();
    let expected_diluted_check_interaction_alpha =
        Felt::from_hex("0x7143d36ac29773e3194e4182dea5b4f49459a2c752df09095c0797d499f43b3")
            .unwrap();

    // Verify execution completed successfully
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty");
}
