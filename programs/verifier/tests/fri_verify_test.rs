mod fixtures;
use felt::Felt;
use stark::funvec::FunVec;
use stark::swiftness::commitment;
use stark::swiftness::fri;
use stark::swiftness::stark::types::cast_struct_to_slice;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_fri_verify() {
    let mut stack: BidirectionalStackAccount = BidirectionalStackAccount::default();
    let task = stark::stark_proof::stark_verify::fri_verify::FriVerify::new();
    push_data(&mut stack);
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }
    assert_eq!(stack.front_index, 0);
    assert_eq!(stack.back_index, 131072);
}

// Stack layout pre-execution:
// ┌────────────────────────────────────────────────────────────┐
// │ Witness        (stark::swiftness::stark::types::Witness)   │  <- back (stack back)
// │ FRI Decommitment (stark::swiftness::fri::types::Decommitment)
// │ FRI Commitment   (stark::swiftness::fri::types::Commitment)|
// │ Queries         (stark::swiftness::fri::types::Queries)    │
// └────────────────────────────────────────────────────────────┘  <- front (stack front)
fn push_data(stack: &mut BidirectionalStackAccount) {
    let queries = fixtures::queries::get();
    let fri_commitment: stark::swiftness::fri::types::Commitment = fixtures::fri_commitment::get();
    let fri_decommitment: commitment::types::Decommitment = fixtures::fri_decommitment::get();
    let witness: fri::types::Witness = fixtures::witness::get();
    let fri_verify_data = stark::swiftness::stark::types::FriVerifyData {
        queries: FunVec::from_vec(queries.clone()),
        fri_commitment: fri_commitment,
        fri_decommitment: fri_decommitment,
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
        current_coset_index: 0,
    };
    // Użyj nowej metody do przechowania FriVerifyData w cache
    stack.store_in_cache(&fri_verify_data);
    println!("FriVerifyData stored in cache");
}
