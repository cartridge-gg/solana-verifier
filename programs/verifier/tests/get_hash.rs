use stark::felt::Felt;
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

    let proof_verifier = proof.transform_to();

    // Replace the public_input with the one from get()
    // proof_verifier.public_input = get();

    stack.proof = proof_verifier;

    stack.push_task(GetHash::new(Felt::ZERO));
    while !stack.is_empty_back() {
        stack.execute();
    }

    // let expected = Felt::from_hex_unchecked(
    //     "0x2175311c4b644202c7f37b3bd7f19ef26cd8d7085f71cd03cf20ec82897f0e2",
    // );
    // assert_eq!(Felt::from_bytes_be_slice(stack.borrow_front()), expected);
}
