use crate::fixtures::authentications;
use crate::fixtures::constraint_coefficients;
use crate::fixtures::decommitment;
use crate::fixtures::stark_config;
use crate::fixtures::trace_decommitment;
use felt::Felt;
use fixtures::trace_commitment;
use stark::funvec::FunVec;
use stark::stark_proof::stark_verify::StarkVerify;
use stark::swiftness::commitment::types::Decommitment;
use stark::swiftness::fri;
use stark::swiftness::stark::types::StarkCommitment;
use stark::swiftness::stark::types::StarkProof;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use utils::global_values::InteractionElements;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;
mod fixtures;

#[test]
fn test_stark_verify() {
    let mut stack = BidirectionalStackAccount::default();
    push_data(&mut stack);

    let proof_str = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
    let proof_verifier = StarkProofParser::try_from(proof_json).unwrap();
    let proof_verifier_transformed = proof_verifier.transform_to();

    let mut proof = StarkProof::default();

    stack.constraint_coefficients =
        constraint_coefficients::get_constraint_coefficients_for_interaction_after_oods()
            .as_slice()
            .try_into()
            .unwrap();
    stack.oods_values = proof_verifier_transformed
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();

    let mut stark_commitment = StarkCommitment::<InteractionElements>::default();

    let trace_decommitment = trace_decommitment::get();
    proof.witness.traces_decommitment = trace_decommitment;

    let original_authentications = authentications::get_original_authentications();
    let interaction_authentications = authentications::get_interaction_authentications();
    let composition_authentications = authentications::get_composition_authentications();

    proof.witness.traces_witness.original.vector.authentications = original_authentications;

    proof
        .witness
        .traces_witness
        .interaction
        .vector
        .authentications = interaction_authentications;

    proof.witness.composition_witness.vector.authentications = composition_authentications;
    proof.witness.composition_decommitment.values = decommitment::get_composition_decommitment();
    stark_commitment.composition = trace_commitment::get_composition_commitment();

    let trace_commitment = trace_commitment::get();
    stark_commitment.traces = trace_commitment;

    proof.config = stark_config::get();

    stack.proof = proof;
    stark_commitment.interaction_after_composition =
        Felt::from_hex("0x49185430497be4bd990699e70b3b91b25c0dd22d5cd436dbf23f364136368bc")
            .unwrap();
    stack.stark_commitment = stark_commitment;

    stack.push_task(StarkVerify::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    println!("Executed {} steps", steps);

    println!("Final stack size: {}", stack.back_index - stack.front_index);
    assert!(steps > 0, "Should have executed at least one step");
    assert!(stack.is_empty_front(), "Front stack should be empty");
    assert!(stack.is_empty_back(), "Back stack should be empty");
    println!("Test completed successfully");
}

fn push_data(stack: &mut BidirectionalStackAccount) {
    let queries = fixtures::queries::get();
    let fri_commitment: stark::swiftness::fri::types::Commitment = fixtures::fri_commitment::get();
    // let fri_decommitment: commitment::types::Decommitment = fixtures::fri_decommitment::get();
    let witness: fri::types::Witness = fixtures::witness::get();

    let fri_verify_data = stark::swiftness::stark::types::FriVerifyData {
        queries: FunVec::from_vec(queries.clone()),
        fri_commitment: fri_commitment,
        fri_decommitment: Decommitment::default(),
        witness: witness.clone(),
        current_layer: 0,
        working_queries: FunVec::default(),
        working_elements: FunVec::default(),
        working_indices: FunVec::default(),
        working_y_values: FunVec::default(),
        coset_size: Felt::ZERO,
        eval_point: Felt::ZERO,
        next_queries: FunVec::default(),
        sibling_witness: FunVec::default(),
        next_x_inv_value: Felt::ZERO,
        coset_x_inv: Felt::ZERO,
        coset_elements: FunVec::default(),
        current_coset_index: 0,
    };
    // Użyj nowej metody do przechowania FriVerifyData w cache
    stack.store_in_cache(&fri_verify_data);
    println!("FriVerifyData stored in cache");
}
