use crate::fixtures::{constraint_coefficients, stark_commitment};
use felt::Felt;
use stark::{stark_proof::stark_commit::StarkCommit, swiftness::air::domains::StarkDomains};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;
mod fixtures;

#[test]
fn test_stark_commit_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    let input = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let proof_verifier = proof.transform_to();

    let oods_values = proof_verifier.unsent_commitment.oods_values.clone();

    stack.oods_values = oods_values.as_slice().try_into().unwrap();
    stack.proof = proof_verifier.clone();

    let stark_domains = StarkDomains::new(
        proof_verifier.config.log_trace_domain_size,
        proof_verifier.config.log_n_cosets,
    );

    let trace_generator = stark_domains.trace_generator;
    stack.push_front(&trace_generator.to_bytes_be()).unwrap();

    let trace_domain_size = stark_domains.trace_domain_size;
    stack.push_front(&trace_domain_size.to_bytes_be()).unwrap();

    // Result of GetHash
    let digest =
        Felt::from_hex("0x59496b8e649ff03c8e9f739e141bd82653fccb2fb1b1a51a71760ea3813ea35")
            .unwrap();
    stack.push_front(&digest.to_bytes_be()).unwrap();

    // Push StarkCommit task
    stack.push_task(StarkCommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    let _counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let _digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    // assert_eq!(stack.constraint_coefficients.to_vec(), constraint_coefficients::get_constraint_coefficients_for_interaction_after_oods().to_vec());
    // assert_eq!(stack.constraint_coefficients.to_vec(), constraint_coefficients::get().to_vec());

    assert_eq!(stack.proof, proof_verifier);

    println!("StarkCommit completed in {} steps", steps);

    let stark_commitment = stack.stark_commitment;
    let expected_stark_commitment = stark_commitment::get();
    assert!(expected_stark_commitment == stark_commitment);

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
            stark_commitment
                .fri
                .inner_layers
                .at(i)
                .vector_commitment
                .commitment_hash,
            expected_stark_commitment
                .fri
                .inner_layers
                .at(i)
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
    assert_eq!(stack.back_index, 131072, "Stack should be empty");

    println!("StarkCommit test completed successfully!");
}
