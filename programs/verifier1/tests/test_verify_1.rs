use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier_1::state::BidirectionalStackAccount;
use verify_1::verify::Verify;

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

    let task = Verify::new();
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }
    assert!(
        stack.is_empty_front(),
        " Stack front should be empty after verification"
    );
    assert!(
        stack.is_empty_back(),
        "Stack back should be empty after verification"
    );
}
