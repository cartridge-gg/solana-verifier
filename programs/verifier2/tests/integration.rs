use felt::Felt;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier_2::state::BidirectionalStackAccount;
mod fixtures;
use crate::fixtures::stark_commitment;
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
    stack.stark_commitment = stark_commitment::get();

    let task = verify_2::verify::Verify::default();
    stack.push_task(task);
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x781658415a62f749fdd7abb778c210fac73bd47ce05470d227cb455aec6055e",
            )
            .to_bytes_be(),
        )
        .unwrap();
    stack
        .push_front(&Felt::from_hex_unchecked("0x0").to_bytes_be())
        .unwrap();

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    println!("steps: {}", steps);
    assert!(
        stack.is_empty_front(),
        " Stack front should be empty after verification"
    );
    assert!(
        stack.is_empty_back(),
        "Stack back should be empty after verification"
    );
}
