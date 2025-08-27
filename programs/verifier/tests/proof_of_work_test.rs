use felt::Felt;
use stark::stark_proof::stark_commit::ProofOfWork;
use stark::swiftness::stark::types::StarkProof;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

mod fixtures;

#[test]
fn test_proof_of_work_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    let mut proof = StarkProof::default();

    let digest: [u8; 32] = [
        4, 95, 162, 177, 183, 248, 159, 156, 78, 134, 182, 48, 118, 191, 221, 127, 64, 53, 5, 81,
        127, 23, 239, 148, 25, 227, 176, 237, 25, 158, 163, 230,
    ];
    let nonce: u64 = 0xd5bee6b9;
    let n_bits: u8 = 32;

    proof.unsent_commitment.proof_of_work.nonce = nonce;
    // proof.config.proof_of_work.n_bits = n_bits;

    stack.proof = proof;

    stack.push_front(&digest).unwrap();

    stack.push_task(ProofOfWork::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    let reseted_counter = Felt::from_bytes_be_slice(stack.borrow_front());
    println!("reseted_counter: {:?}", reseted_counter);
    stack.pop_front();
    let digest = Felt::from_bytes_be_slice(stack.borrow_front());
    println!("digest: {:?}", digest);
    stack.pop_front();

    let expected_digest = Felt::from_hex_unchecked("0x781658415a62f749fdd7abb778c210fac73bd47ce05470d227cb455aec6055e");
    assert_eq!(digest, expected_digest);
    assert_eq!(reseted_counter, Felt::from_hex_unchecked("0x0"));

    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");
    println!("ProofOfWork test completed successfully in {} steps", steps);
}
