use crate::fixtures::stark_commitment;
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
    let oods_values = fixtures::oods_values::get();
    // let stark_domains = fixtures::stark_domains::get();

    proof.unsent_commitment.oods_values = oods_values;
    proof.unsent_commitment.fri = unsent_commitment;
    proof.config = config;
    proof.public_input = public_input;

    // Reference values from the test output
    let original_commitment =
        Felt::from_hex("0x305f1ee7c0b38a403b2fa7ec86a3d11c8a174891194a2c656147268b59e876d")
            .unwrap();
    let interaction_commitment =
        Felt::from_hex("0x6d41514e4a6e39f5b4e5f18f234525df1d2d92393c11ce11bd885615c88406").unwrap();

    proof.unsent_commitment.traces = UnsentCommitment {
        original: original_commitment,
        interaction: interaction_commitment,
    };
    proof.unsent_commitment.composition =
        Felt::from_hex("0x112367c6fef0963c09cd918c7d31159ae7effbf9e16ffe7cac15b7bb4074373")
            .unwrap();

    stack.oods_values = oods_values.as_slice().try_into().unwrap();
    stack.proof = proof;

    let trace_generator =
        Felt::from_hex("0x57a797181c06d8427145cb66056f032751615d8617c5468258e96d2bb6422f9")
            .unwrap();
    stack.push_front(&trace_generator.to_bytes_be()).unwrap();

    let trace_domain_size = Felt::from_hex("0x10000000").unwrap();
    stack.push_front(&trace_domain_size.to_bytes_be()).unwrap();

    let digest =
        Felt::from_hex("0x59496b8e649ff03c8e9f739e141bd82653fccb2fb1b1a51a71760ea3813ea35")
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

    let stark_commitment = stack.stark_commitment;
    let expected_stark_commitment = stark_commitment::get();

    assert_eq!(
        stark_commitment
            .traces
            .original
            .vector_commitment
            .commitment_hash,
        expected_stark_commitment
            .traces
            .original
            .vector_commitment
            .commitment_hash
    );
    assert_eq!(
        stark_commitment
            .traces
            .interaction
            .vector_commitment
            .commitment_hash,
        expected_stark_commitment
            .traces
            .interaction
            .vector_commitment
            .commitment_hash
    );
    assert_eq!(
        stark_commitment.traces.interaction_elements,
        expected_stark_commitment.traces.interaction_elements
    );
    assert_eq!(
        stark_commitment
            .composition
            .vector_commitment
            .commitment_hash,
        expected_stark_commitment
            .composition
            .vector_commitment
            .commitment_hash
    );
    assert_eq!(
        stark_commitment.interaction_after_composition,
        expected_stark_commitment.interaction_after_composition
    );
    assert_eq!(
        stark_commitment.oods_values,
        expected_stark_commitment.oods_values
    );
    assert_eq!(
        stark_commitment.interaction_after_oods,
        expected_stark_commitment.interaction_after_oods
    );
    for i in 0..expected_stark_commitment.fri.inner_layers.len() {
        assert_eq!(
            stark_commitment.fri.inner_layers[i]
                .vector_commitment
                .commitment_hash,
            expected_stark_commitment.fri.inner_layers[i]
                .vector_commitment
                .commitment_hash
        );
    }
    assert_eq!(
        stark_commitment.fri.eval_points,
        expected_stark_commitment.fri.eval_points
    );
    assert_eq!(
        stark_commitment.fri.last_layer_coefficients,
        expected_stark_commitment.fri.last_layer_coefficients
    );

    // Check that stack is empty
    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");

    println!("StarkCommit test completed successfully!");
}
