use felt::Felt;
use stark::stark_proof::verify::Verify;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;
mod fixtures;

#[test]
pub fn test_proof_verification() {
    let mut stack = BidirectionalStackAccount::default();

    let proof_str = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();
    let proof_verifier = proof.transform_to();

    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();

    stack.proof = proof_verifier.clone();

    let task = Verify::new();
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }
    let mut program_output_vec = Vec::new();
    for _ in 0..5 {
        let program_output = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        program_output_vec.push(program_output);
    }
    println!("Program Output: {:?}", program_output_vec);
    let program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    println!("Program Hash: {:?}", program_hash);

    // let executed_task = &stack.executed_tasks;
    // println!("Executed tasks and their counts: {:#?}", executed_task);
    assert!(stack.is_empty_front());
    assert!(stack.is_empty_back());
    assert_eq!(stack.proof, proof_verifier);
}
