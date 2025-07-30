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
        .push_front(&initial_transcript_digest.to_bytes_be())
        .unwrap();
    stack
        .push_front(&initial_transcript_counter.to_bytes_be())
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
    // Collect from back (LIFO order) since eval_points are pushed to front
    let mut temp_points = Vec::new();
    for _ in 0..4 {
        let eval_point = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        temp_points.push(eval_point);
    }

    // Reverse to get correct order (first generated should be eval_points[0])
    for point in temp_points.into_iter().rev() {
        eval_points.push(point);
    }

    for (i, point) in eval_points.iter().enumerate() {
        println!("eval_point[{}]: {:?}", i, point);
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
    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");
    assert_eq!(stack.is_empty_front(), true, "Stack should be empty");

    // The stack should contain the inner layer commitments and other data
    // but we're mainly testing the eval_points generation and transcript updates

    println!("FriCommit test completed successfully in {} steps", steps);
}
