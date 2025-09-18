mod fixtures;
use felt::Felt;
use stark::funvec::FunVec;
use stark::swiftness::commitment;
use stark::swiftness::fri;
use stark::swiftness::stark::types::StarkCommitment;
use stark::swiftness::stark::types::StarkProof;
use utils::global_values::InteractionElements;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_fri_verify() {
    let mut stack: BidirectionalStackAccount = BidirectionalStackAccount::default();
    let task = stark::stark_proof::stark_verify::fri_verify::FriVerify::new();
    push_data(&mut stack);
    stack.push_task(task);

    let mut proof = StarkProof::default();
    proof.witness.fri_witness = fixtures::witness::get();
    stack.proof = proof;

    let mut stark_commitment = StarkCommitment::<InteractionElements>::default();
    stark_commitment.fri = fixtures::fri_commitment::get();
    stack.stark_commitment = stark_commitment;

    while !stack.is_empty_back() {
        stack.execute();
    }

    assert_eq!(stack.front_index, 0);
    assert_eq!(stack.back_index, 131072);
    println!("SUCCESSFULLY EXECUTED FRI VERIFY");
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
    let fri_decommitment: commitment::types::Decommitment = fixtures::fri_decommitment::get();
    let fri_verify_data = stark::swiftness::stark::types::FriVerifyData {
        queries: FunVec::from_vec(queries.clone()),
        fri_decommitment: fri_decommitment,
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
