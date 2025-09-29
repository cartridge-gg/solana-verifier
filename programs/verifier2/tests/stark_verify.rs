// use crate::fixtures::constraint_coefficients;
// use crate::fixtures::stark_commitment;
// use felt::Felt;
// use stark_verify_decommitments::stark_verify::StarkVerify;
// use types::swiftness::commitment::types::Decommitment;
// use swiftness_proof_parser::json_parser;
// use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
// use types::funvec::FunVec;
// use utils::BidirectionalStack;
// use utils::Scheduler;
// use verifier_2::state::BidirectionalStackAccount;
// use utils::CacheStorage;
// mod fixtures;

// #[test]
// fn test_stark_verify() {
//     let mut stack = BidirectionalStackAccount::default();
//     push_data(&mut stack);

//     let proof_str = include_str!("../../../example_proof/saya.json");
//     let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
//     let proof_verifier = StarkProofParser::try_from(proof_json).unwrap();
//     let proof_verifier_transformed = proof_verifier.transform_to();

//     stack.constraint_coefficients =
//         constraint_coefficients::get_constraint_coefficients_for_interaction_after_oods()
//             .as_slice()
//             .try_into()
//             .unwrap();
//     stack.oods_values = proof_verifier_transformed
//         .unsent_commitment
//         .oods_values
//         .as_slice()
//         .try_into()
//         .unwrap();

//     stack.proof = proof_verifier_transformed.clone();
//     stack.stark_commitment = stark_commitment::get();

//     stack.push_task(StarkVerify::new());

//     let mut steps = 0;
//     while !stack.is_empty_back() {
//         stack.execute();
//         steps += 1;
//     }

//     assert_eq!(stack.proof, proof_verifier_transformed);
//     assert_eq!(stack.stark_commitment, stark_commitment::get());

//     println!("Executed {} steps", steps);

//     println!("Final stack size: {}", stack.back_index - stack.front_index);
//     assert!(steps > 0, "Should have executed at least one step");
//     assert!(stack.is_empty_front(), "Front stack should be empty");
//     assert!(stack.is_empty_back(), "Back stack should be empty");
//     println!("Test completed successfully");
// }

// fn push_data(stack: &mut BidirectionalStackAccount) {
//     let queries = fixtures::queries::get();

//     let fri_verify_data = types::swiftness::stark::types::FriVerifyData {
//         queries: FunVec::from_vec(queries.clone()),
//         fri_decommitment: Decommitment::default(),
//         current_layer: 0,
//         working_queries: FunVec::default(),
//         working_elements: FunVec::default(),
//         working_indices: FunVec::default(),
//         working_y_values: FunVec::default(),
//         coset_size: Felt::ZERO,
//         eval_point: Felt::ZERO,
//         next_queries: FunVec::default(),
//         sibling_witness: FunVec::default(),
//         next_x_inv_value: Felt::ZERO,
//         coset_x_inv: Felt::ZERO,
//         coset_elements: FunVec::default(),
//         current_coset_index: 0,
//     };
//     stack.store_in_cache(&fri_verify_data);
//     println!("FriVerifyData stored in cache");
// }
