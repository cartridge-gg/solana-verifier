use felt::Felt;
use stark::swiftness::commitment::table::config::Config as TableConfig;
use stark::swiftness::commitment::table::types::Commitment as TableCommitment;
use stark::swiftness::commitment::vector::config::Config as VectorConfig;
use stark::swiftness::commitment::vector::types::Commitment as VectorCommitment;
use stark::{
    stark_proof::stark_verify::table_decommit::TableDecommit,
    swiftness::commitment::vector::types::CommitmentTrait,
};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

use crate::fixtures::{authentications, commitment, decommitment, queries};
mod fixtures;

#[test]
fn test_table_decommit() {
    let mut stack = BidirectionalStackAccount::default();

    let decommitment_values = decommitment::get_original_decommitment();
    let authentications = authentications::get_original_authentications();

    // Push authentications in reverse order (for stack)
    for auth in authentications.as_slice().iter().rev() {
        stack.push_front(&auth.to_bytes_be()).unwrap();
    }

    stack
        .push_front(&Felt::from(authentications.len() as u64).to_bytes_be())
        .unwrap();

    for value in decommitment_values.as_slice().iter().rev() {
        stack.push_front(&value.to_bytes_be()).unwrap();
    }
    let decommitment_length = Felt::from(decommitment_values.len() as u64);
    stack
        .push_front(&decommitment_length.to_bytes_be())
        .unwrap();

    let queries = queries::get();
    for i in (0..queries.len()).rev() {
        let index = queries[i];
        stack.verify_variables.temp_queries[i * 2] = index;
    }
    let queries_length = Felt::from(queries.len() as u64);
    stack.push_front(&queries_length.to_bytes_be()).unwrap();

    let table_commitment = commitment::get().original;

    // Push vector commitment using trait method
    table_commitment.push_to_stack(&mut stack);

    // Push the VectorDecommit task
    stack.push_task(TableDecommit::new());

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
