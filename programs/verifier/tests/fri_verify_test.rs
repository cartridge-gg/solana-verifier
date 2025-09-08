mod fixtures;

use std::fs;

use stark::swiftness::stark::types::cast_struct_to_slice;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;
#[test]
fn test_fri_verify() {
    let mut stack = BidirectionalStackAccount::default();
    let task = stark::stark_proof::stark_verify::FriVerify::new();

    stack.push_task(task);
    push_data(&mut stack);
    while !stack.is_empty_back() {
        stack.execute();
    }
}

fn push_data(stack: &mut BidirectionalStackAccount) {
    let queries = fixtures::queries::get();
    let fri_commitment: stark::swiftness::fri::types::Commitment = fixtures::fri_commitment::get();
    let fri_decommitment: stark::swiftness::commitment::types::Decommitment =
        fixtures::fri_decommitment::get();
    let witness: stark::swiftness::commitment::types::Witness = fixtures::witness::get();
    let mut input = stark::swiftness::fri::types::FriVerifyInput {
        queries,
        fri_commitment,
        fri_decommitment,
        witness,
    };
    let bytes = cast_struct_to_slice(&mut input);
    stack
        .push_front(bytes)
        .expect("Failed to push data onto the stack")
}
