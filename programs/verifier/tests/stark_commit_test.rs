use felt::Felt;
use stark::stark_proof::stark_commit::StarkCommit;
use stark::swiftness::air::trace::UnsentCommitment;
use stark::swiftness::stark::types::StarkProof;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

mod fixtures;

#[test]
fn test_stark_commit_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    // Create a StarkProof with reference trace commitments
    let mut proof = StarkProof::default();

    let public_input = fixtures::public_input::get();
    let unsent_commitment = fixtures::fri_unsent_commitment::get();
    let config = fixtures::stark_config::get();
    // let stark_domains = fixtures::stark_domains::get();
    let constraint_coefficients = fixtures::constraint_coefficients::get();
    let oods_values = fixtures::oods_values::get();

    proof.unsent_commitment.oods_values = oods_values;
    proof.unsent_commitment.fri = unsent_commitment;
    proof.config = config;
    proof.public_input = public_input;
    // Set proof of work configuration
    // proof.config.proof_of_work.n_bits = 32;
    // proof.unsent_commitment.proof_of_work.nonce = 0xd5bee6b9;

    // Reference values from the test output
    let original_commitment =
        Felt::from_hex("0x305f1ee7c0b38a403b2fa7ec86a3d11c8a174891194a2c656147268b59e876d")
            .unwrap();
    let interaction_commitment =
        Felt::from_hex("0x6d41514e4a6e39f5b4e5f18f234525df1d2d92393c11ce11bd885615c88406")
            .unwrap();

    proof.unsent_commitment.traces = UnsentCommitment {
        original: original_commitment,
        interaction: interaction_commitment,
    };

    stack.constraint_coefficients = constraint_coefficients.as_slice().try_into().unwrap();
    stack.oods_values = oods_values.as_slice().try_into().unwrap();
    stack.proof = proof;

    // Push initial transcript state
    let initial_transcript_digest =
        Felt::from_hex("0x59496b8e649ff03c8e9f739e141bd82653fccb2fb1b1a51a71760ea3813ea35")
            .unwrap();
    let initial_transcript_counter = Felt::from_hex("0x0").unwrap();

    stack.push_front(&initial_transcript_counter.to_bytes_be()).unwrap();
    stack.push_front(&initial_transcript_digest.to_bytes_be()).unwrap();

    // Push the expected interaction elements (in reverse order as they will be popped)
    // diluted_check_interaction_alpha
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x734820597aa2142c285a8ab4990f17ba4241a78de519e3661dafd9453a8e822",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // diluted_check_interaction_z
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x7f01d79f2cdf6aa851c9b2e0fa2e92f64ecd655289f827b14d5e7b483f52b48",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // diluted_check_permutation_interaction_elm
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x1f44508505278264aabe386ad5df3bee4b8147b3d0e20518bfaec709cbc1322",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // range_check16_perm_interaction_elm
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x47256c1d9e69a2c23e0a5b2666fd2e2037ef2987d19b53da2b089c7a79e217c",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // memory_multi_column_perm_hash_interaction_elm0
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x522df1ce46453857bc93d7b48c77fd4968ae6be4de52c9a9ebf3b053fe3f288",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // memory_multi_column_perm_perm_interaction_elm
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x63be95eef090c5ed842139ace99b3dc2e8222f4946d656d2b8ecf9f3a4eaa64",
            )
            .to_bytes_be(),
        )
        .unwrap();

    let point = Felt::from_hex("0x49185430497be4bd990699e70b3b91b25c0dd22d5cd436dbf23f364136368bc")
        .unwrap();
    stack.push_front(&point.to_bytes_be()).unwrap();

    let digest = Felt::from_hex("0x59496b8e649ff03c8e9f739e141bd82653fccb2fb1b1a51a71760ea3813ea35")
        .unwrap();
    stack.push_front(&digest.to_bytes_be()).unwrap();

    let counter = Felt::from_hex("0x0").unwrap();
    stack.push_front(&counter.to_bytes_be()).unwrap();

    // Push StarkCommit task
    stack.push_task(StarkCommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    println!("StarkCommit completed in {} steps", steps);

    // Check that stack is empty
    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");

    println!("StarkCommit test completed successfully!");
} 