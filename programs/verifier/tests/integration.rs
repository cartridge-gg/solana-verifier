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

    //    proof_verifier.witness.fri_witness.layers.at_mut(0).leaves = fixtures::witness::get_layers()[0].leaves.clone();

    stack.proof = proof_verifier.clone();
    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();

    let task = Verify::new();
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }
    // let executed_task = &stack.executed_tasks;
    // println!("Executed tasks and their counts: {:#?}", executed_task);
}
