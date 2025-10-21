use felt::Felt;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use types::funvec::FunVec;
use types::swiftness::commitment::types::Decommitment as FriDecommitment;
use types::swiftness::global_values::InteractionElements;
use types::swiftness::stark::types::{FriVerifyData, StarkCommitment, VerifyVariables};
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier_3::state::BidirectionalStackAccount;
mod fixtures;
use crate::fixtures::constraint_coefficients;
use utils::CacheStorage;

#[test]
pub fn test_proof_verification() {
    let mut stack = BidirectionalStackAccount::default();

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
    let mut stark_commitment: StarkCommitment<InteractionElements> = StarkCommitment::default();
    let oods_point =
        Felt::from_hex("0x49185430497be4bd990699e70b3b91b25c0dd22d5cd436dbf23f364136368bc")
            .unwrap();
    stark_commitment.interaction_after_composition = oods_point;
    let constraint_coeffs =
        constraint_coefficients::get_constraint_coefficients_for_interaction_after_oods();
    stark_commitment.interaction_after_oods = FunVec::from_vec(constraint_coeffs.to_vec());
    stack.stark_commitment = stark_commitment;
    stack.constraint_coefficients =
        constraint_coefficients::get_constraint_coefficients_for_interaction_after_oods()
            .as_slice()
            .try_into()
            .unwrap();
    let queries = fixtures::queries::get();
    let mut verify_variables = types::swiftness::stark::types::VerifyVariables::default();
    for (i, &query) in queries.iter().enumerate() {
        if i < verify_variables.queries_indexes.len() {
            verify_variables.queries_indexes[i] = query;
        }
    }
    stack.verify_variables = verify_variables;

    let stark_verify_data = types::swiftness::stark::types::FriVerifyData {
        queries: FunVec::from_vec(queries.clone()),
        fri_decommitment: FriDecommitment::default(),
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

    while !stack.is_empty_back() {
        stack.execute();
    }
    let stark_verify_data =
        stack.borrow_from_cache::<types::swiftness::stark::types::FriVerifyData>();
    println!(
        "DEBUG Stage 3 - fri_decommitment size: {}",
        stark_verify_data.fri_decommitment.values.len()
    );
    println!(
        "DEBUG Stage 3 - working_elements size: {}",
        stark_verify_data.working_elements.len()
    );
    let mut evaluations = Vec::new();

    let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
    for value in fri_verify_data.fri_decommitment.values.iter() {
        evaluations.push(*value);
    }

    let expected_result = vec![
        "0x56589147f36eee3f7976a1542599dd32be46d202f4ec49dccef821f43ade30f",
        "0x6da23461f6dc6aac5624da021558eaea6f8039c59a3a1596694aaade6ae5aea",
        "0x7c2cb3f9065f1c08480be0521698325689a3346e6fd358e65d98f43ef91848e",
        "0x7272da9be8a83b5007e3b63487265431b894626aabe48070e87412a33f06e21",
        "0x48b12d9655668770fbb57fa2aaa241df1aff1195a68c44ea912563e633c0311",
        "0x5613f5cb362f21af6a28237858c8e25930ee6d1f03d615991862c966b696b07",
        "0x1daf84477265f19fbcbb8fa7b62d85a14221de9add62996cb6a1eba477532c",
        "0x255f150abc9f168bbf353a77445b26a0c4c3243be19985398cef35916b39349",
        "0x3d99e7912b03d046b302ba451fd39d4a2f22173c5d3facd40eaf8e4ca160729",
        "0x3931a734c9e17b5d11721226625ce4d8c2ce416cd05168442c636717b8f2b7c",
        "0x501483805f53ae20ff3317425627bab5a8a31487ce9e62bf09f2ad591d4d636",
        "0x55bf2ccb8e98ecd75c23c941d8201b3ff3cce32f4c2fedeea787307cd42f275",
        "0x2872e8b5f38ac80c1db5cd85801c20696a1480e7a35d532a8d06d51428d7417",
        "0x2217dfcf29dd655b6a85d1769e7cf444ecefa2cd276e1c6de73d5d039c6cf8e",
        "0x1558aa1be37c22f07b2b0422b37a5f67ef6285c8a33a94f7d46347bfc64b9e2",
        "0x43bbcf9a0483a1f8e74570452b870ef248e4d5aa227bf64910c0c92d0afa598",
    ]
    .iter()
    .map(|f| Felt::from_hex(f).unwrap())
    .collect::<Vec<Felt>>();

    println!("Expected result: {:?}", expected_result);
    println!("Actual result:   {:?}", evaluations);

    assert_eq!(
        evaluations, expected_result,
        "Result should match expected value from autogenerated function"
    );

    assert!(stack.is_empty_back(), "Stack should be empty");
    assert!(stack.is_empty_front(), "Stack should be empty");
}
