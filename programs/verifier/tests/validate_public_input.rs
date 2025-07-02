use stark::felt::Felt;
use stark::stark_proof::validate_public_input::ValidatePublicInput;
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn validate_public_input() {
    let mut stack = BidirectionalStackAccount::default();

    let input = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let proof_verifier = proof.transform_to();

    stack.proof = proof_verifier;

    stack.push_task(ValidatePublicInput::new());
    while !stack.is_empty_back() {
        stack.execute();
    }
}
