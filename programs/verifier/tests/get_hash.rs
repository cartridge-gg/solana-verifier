mod fixtures;
use felt::Felt;
use fixtures::public_input::get;
use stark::stark_proof::get_hash::GetHash;
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn get_hash() {
    let mut stack = BidirectionalStackAccount::default();

    let input = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let mut proof_verifier = proof.transform_to();
    proof_verifier.public_input = get();

    stack.proof = proof_verifier;

    stack.push_task(GetHash::new(Felt::ZERO));
    while !stack.is_empty_back() {
        stack.execute();
    }

    let expected = Felt::from_hex_unchecked(
        "0x648acb805304a6d1280c406beb7b5cc946052ab1968fba40488fb4f5e67adaf",
    );
    assert_eq!(Felt::from_bytes_be_slice(stack.borrow_front()), expected);
    println!(
        "Result: {:?}",
        Felt::from_bytes_be_slice(stack.borrow_front())
    );
}
