use felt::Felt;
use stark::verify::Verify;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;

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
