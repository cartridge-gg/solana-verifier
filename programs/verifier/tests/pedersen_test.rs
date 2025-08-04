use felt::Felt;
use stark::pedersen::PedersenHash;
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn pedersen_hash() {
    let mut stack = BidirectionalStackAccount::default();
    println!("stack.front_index at start: {}", stack.front_index);
    println!("stack.back_index at start: {}", stack.back_index);

    let input = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let proof_verifier = proof.transform_to();

    stack.proof = proof_verifier;

    PedersenHash::push_input(
        Felt::from_hex_unchecked(
            "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        ),
        Felt::from_hex_unchecked(
            "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        ),
        &mut stack,
    );
    stack.push_task(PedersenHash::new());
    while !stack.is_empty_back() {
        stack.execute();
    }
    let result = Felt::from_bytes_be_slice(stack.borrow_front().try_into().unwrap());
    println!("result: {:?}", result);

    let expected = Felt::from_hex_unchecked(
        "030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662",
    );
    stack.pop_front();

    println!("stack.front_index: {}", stack.front_index);
    println!("stack.back_index: {}", stack.back_index);

    assert_eq!(
        result, expected,
        "Pedersen hash result doesn't match expected value"
    );
    assert_eq!(stack.front_index, 0);
    assert_eq!(stack.back_index, 65536);
}
