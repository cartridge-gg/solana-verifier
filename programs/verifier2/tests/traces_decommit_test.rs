use felt::Felt;
use stark_verify_decommitments::traces_decommit::TracesDecommit;
use types::swiftness::global_values::InteractionElements;
use types::swiftness::stark::types::StarkCommitment;
use utils::{BidirectionalStack, Scheduler};
use verifier_2::state::BidirectionalStackAccount;
mod fixtures;
use types::swiftness::stark::types::StarkProof;

use crate::fixtures::{authentications, queries, trace_commitment, trace_decommitment};

#[test]
fn test_traces_decommit() {
    let mut stack = BidirectionalStackAccount::default();
    let queries = queries::get();
    for index in queries.iter().rev() {
        stack.push_front(&index.to_bytes_be()).unwrap();
    }
    let queries_length = Felt::from(queries.len() as u64);
    stack.push_front(&queries_length.to_bytes_be()).unwrap();

    let mut proof = StarkProof::default();
    let mut stark_commitment = StarkCommitment::<InteractionElements>::default();

    let trace_decommitment = trace_decommitment::get();
    proof.witness.traces_decommitment = trace_decommitment;

    let original_authentications = authentications::get_original_authentications();
    let interaction_authentications = authentications::get_interaction_authentications();

    proof.witness.traces_witness.original.vector.authentications = original_authentications;

    proof
        .witness
        .traces_witness
        .interaction
        .vector
        .authentications = interaction_authentications;

    let trace_commitment = trace_commitment::get();
    stark_commitment.traces = trace_commitment;

    stack.proof = proof;
    stack.stark_commitment = stark_commitment;

    // Push the VectorDecommit task
    stack.push_task(TracesDecommit::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        println!("Step: {}", steps);
        // if steps == 35 {
        //     break;
        // }
        steps += 1;
    }

    println!("Executed {} steps", steps);
    println!("Final stack size: {}", stack.back_index - stack.front_index);
    assert!(steps > 0, "Should have executed at least one step");
    assert!(stack.is_empty_front(), "Front stack should be empty");
    assert!(stack.is_empty_back(), "Back stack should be empty");
    println!("Test completed successfully");
}
