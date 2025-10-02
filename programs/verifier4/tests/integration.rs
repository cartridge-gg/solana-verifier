use felt::Felt;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use types::funvec::FunVec;
use types::swiftness::commitment::types::Decommitment;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;
use verifier_2::state::BidirectionalStackAccount as Verifier2StackAccount;
use verifier_3::state::BidirectionalStackAccount as Verifier3StackAccount;
use verifier_4::state::BidirectionalStackAccount as Verifier4StackAccount;
mod fixtures;
use utils::CacheStorage;

#[test]
pub fn test_proof_verification() {
    println!("Starting Verifier 1");
    let mut stack = Verifier1StackAccount::default();

    let proof_str = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();
    let proof_verifier = proof.transform_to();

    stack.proof = proof_verifier.clone();
    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();

    let task = verify_1::verify::Verify::new();
    stack.push_task(task);

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    let counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    println!("counter: {:?}", counter);
    let digest = Felt::from_bytes_be_slice(stack.borrow_front());
    println!("digest: {:?}", digest);
    stack.pop_front();
    let commitment = stack.stark_commitment;

    println!("Starting Verifier 2");
    let mut stack = Verifier2StackAccount::default();

    stack.proof = proof_verifier.clone();
    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();
    stack.stark_commitment = commitment.clone();

    println!("digest: {:?}", digest);
    println!("counter: {:?}", counter);

    let task = verify_2::verify::Verify::new(digest, counter);
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    let verify_variables = stack.verify_variables;
    let queries = verify_variables.queries_indexes;

    println!("Starting Verifier 3");
    let mut stack = Verifier3StackAccount::default();

    stack.proof = proof_verifier.clone();
    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();
    stack.stark_commitment = commitment.clone();
    let stark_verify_data = types::swiftness::stark::types::FriVerifyData {
        queries: FunVec::from_vec(queries.to_vec()),
        fri_decommitment: Decommitment::default(),
        current_layer: 0,
        layer_queries: FunVec::default(),
        active_query_count: 0,
        working_elements: FunVec::default(),
        working_indices: FunVec::default(),
        working_y_values: FunVec::default(),
        coset_size: Felt::ZERO,
        eval_point: Felt::ZERO,
        sibling_witness: FunVec::default(),
        next_x_inv_value: Felt::ZERO,
        coset_x_inv: Felt::ZERO,
        current_coset_index: 0,
    };
    stack.store_in_cache(&stark_verify_data);

    let task = verify_3::verify::Verify::new();
    stack.push_task(task);

    let mut steps_stage_3 = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        steps_stage_3 += 1;
    }
    println!("steps stage 3: {:?}", steps_stage_3);

    let stark_verify_data =
        stack.borrow_from_cache::<types::swiftness::stark::types::FriVerifyData>();

    println!("Starting Verifier 4");
    let mut stack = Verifier4StackAccount::default();

    stack.proof = proof_verifier.clone();

    stack.stark_commitment = commitment;
    stack.store_in_cache(stark_verify_data);

    let task = verify_4::verify::Verify::new();
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    println!("steps: {:?}", steps);
    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("Result program hash: {result_program_hash:?}");
    println!("Result output hash: {result_output_hash:?}");
    assert_eq!(
        result_program_hash,
        Felt::from_hex("0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07").unwrap()
    );
    assert_eq!(
        result_output_hash,
        Felt::from_hex("0x3233b5615a8de5563f7d3ba086b8f260189ac47753a1c131d063ed3f6c24400")
            .unwrap()
    );

    assert!(
        stack.is_empty_front(),
        " Stack front should be empty after verification"
    );
    assert!(
        stack.is_empty_back(),
        "Stack back should be empty after verification"
    );
}
