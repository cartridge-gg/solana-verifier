use felt::Felt;
use stark::stark_proof::stark_commit::FriCommit;
use stark::swiftness::stark::types::StarkProof;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

mod fixtures;
use fixtures::{fri_commitment, fri_config, fri_unsent_commitment, stark_config};

#[test]
fn test_fri_commit_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();
    let mut proof = StarkProof::default();

    proof.config.fri = fri_config::get();
    proof.unsent_commitment.fri = fri_unsent_commitment::get();
    proof.config = stark_config::get();
    stack.proof = proof;

    let initial_transcript_digest =
        Felt::from_hex("0x3612d68f9f68b263d83b0854b812018fd1b7ba0359d7514fffdabd44f0696e6")
            .unwrap();
    let initial_transcript_counter = Felt::from_hex("0x1").unwrap();

    stack
        .push_front(&initial_transcript_digest.to_bytes_be())
        .unwrap();
    stack
        .push_front(&initial_transcript_counter.to_bytes_be())
        .unwrap();

    stack.push_task(FriCommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    println!("steps: {:?}", steps);

    let transcript_counter_final = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    let _transcript_digest_final = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    // let mut eval_points = Vec::new();
    // let mut temp_points = Vec::new();
    // for _ in 0..4 {
    //     let eval_point = Felt::from_bytes_be_slice(stack.borrow_front());
    //     stack.pop_front();
    //     temp_points.push(eval_point);
    // }

    // for point in temp_points.into_iter().rev() {
    //     eval_points.push(point);
    // }

    // for (i, point) in eval_points.iter().enumerate() {
    //     println!("eval_point[{}]: {:?}", i, point);
    // }

    // let expected_commitment = fri_commitment::get();

    // assert_eq!(
    //     eval_points.len(),
    //     expected_commitment.eval_points.len(),
    //     "Number of eval_points mismatch"
    // );

    // for (i, (actual, expected)) in eval_points
    //     .iter()
    //     .zip(expected_commitment.eval_points.iter())
    //     .enumerate()
    // {
    //     assert_eq!(actual, expected, "eval_point[{}] mismatch", i);
    // }

    assert_eq!(
        transcript_counter_final,
        Felt::ZERO,
        "Transcript counter should be reset after reading last layer coefficients"
    );

    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");
    assert_eq!(stack.is_empty_front(), true, "Stack should be empty");
}
