use stark::felt::Felt;
use stark::stark_proof::stark_commit::FriCommit;
use stark::swiftness::stark::types::StarkProof;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

mod fixtures;
use fixtures::{fri_commitment, fri_config, fri_unsent_commitment, stark_config};

#[test]
fn test_fri_commit_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    // Create a StarkProof with reference FRI config and unsent commitment
    let mut proof = StarkProof::default();

    // Set FRI config
    proof.config.fri = fri_config::get();

    // Set FRI unsent commitment
    proof.unsent_commitment.fri = fri_unsent_commitment::get();

    // Set other necessary config values
    proof.config = stark_config::get();

    stack.proof = proof;

    // Initial transcript state from the reference test
    // This should match the state after OODS coefficients are generated
    let initial_transcript_digest =
        Felt::from_hex("0x3612d68f9f68b263d83b0854b812018fd1b7ba0359d7514fffdabd44f0696e6")
            .unwrap();
    let initial_transcript_counter = Felt::from_hex("0x1").unwrap();

    // Push initial transcript state to stack
    stack
        .push_front(&initial_transcript_counter.to_bytes_be())
        .unwrap();
    stack
        .push_front(&initial_transcript_digest.to_bytes_be())
        .unwrap();

    // Execute FriCommit
    stack.push_task(FriCommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    println!("steps: {:?}", steps);

    // Collect results from stack
    let transcript_counter_final = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    println!("transcript_counter_final: {:?}", transcript_counter_final);

    let transcript_digest_final = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    println!("transcript_digest_final: {:?}", transcript_digest_final);

    // Collect eval_points (should be 4 for n_layers=5)
    let mut eval_points = Vec::new();
    for i in 0..4 {
        let eval_point = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        eval_points.push(eval_point);
        println!("eval_point[{}]: {:?}", i, eval_point);
    }

    // Verify against expected values
    let expected_commitment = fri_commitment::get();

    // Check eval_points match
    assert_eq!(
        eval_points.len(),
        expected_commitment.eval_points.len(),
        "Number of eval_points mismatch"
    );

    for (i, (actual, expected)) in eval_points
        .iter()
        .zip(expected_commitment.eval_points.iter())
        .enumerate()
    {
        assert_eq!(actual, expected, "eval_point[{}] mismatch", i);
    }

    // Verify the transcript was updated correctly (counter should be 0 after read_felt_vector)
    assert_eq!(
        transcript_counter_final,
        Felt::ZERO,
        "Transcript counter should be reset after reading last layer coefficients"
    );

    // Verify execution completed successfully
    assert!(steps > 0, "Should have executed at least one step");

    // The stack should contain the inner layer commitments and other data
    // but we're mainly testing the eval_points generation and transcript updates

    println!("FriCommit test completed successfully in {} steps", steps);
}

// #[test]
// fn test_fri_commit_single_layer() {
//     let mut stack = BidirectionalStackAccount::default();

//     // Create a StarkProof with n_layers = 1 (edge case)
//     let mut proof = StarkProof::default();

//     // Set FRI config with single layer
//     let mut single_layer_config = fri_config::get();
//     single_layer_config.n_layers = Felt::from(1);
//     single_layer_config.inner_layers = vec![];
//     proof.config.fri = single_layer_config;

//     // Set minimal FRI unsent commitment
//     proof.unsent_commitment.fri = FriUnsentCommitment {
//         inner_layers: vec![],
//         last_layer_coefficients: vec![Felt::ZERO; 128], // 2^7 coefficients
//     };

//     stack.proof = proof;

//     // Initial transcript state
//     let initial_transcript_digest = Felt::from_hex("0x1").unwrap();
//     let initial_transcript_counter = Felt::from_hex("0x0").unwrap();

//     stack.push_front(&initial_transcript_counter.to_bytes_be()).unwrap();
//     stack.push_front(&initial_transcript_digest.to_bytes_be()).unwrap();

//     // Execute FriCommit
//     stack.push_task(FriCommit::new());

//     while !stack.is_empty_back() {
//         stack.execute();
//     }

//     // For single layer, we should have no eval_points
//     let transcript_counter_final = Felt::from_bytes_be_slice(stack.borrow_front());
//     stack.pop_front();
//     let transcript_digest_final = Felt::from_bytes_be_slice(stack.borrow_front());
//     stack.pop_front();

//     // Verify transcript was updated by read_felt_vector
//     assert_eq!(transcript_counter_final, Felt::ZERO,
//         "Transcript counter should be reset after reading coefficients");
//     assert_ne!(transcript_digest_final, Felt::from_hex("0x1").unwrap(),
//         "Transcript digest should be updated");

//     println!("Single layer FriCommit test completed successfully");
// }
