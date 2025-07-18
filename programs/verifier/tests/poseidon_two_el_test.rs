use stark::felt::Felt;
use stark::poseidon::PoseidonHash;
use stark::poseidon::PoseidonHashMany;
use starknet_crypto::poseidon_hash;
use starknet_crypto::Felt as StarkFelt;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_poseidon_hash_2() {
    let a = Felt::from_hex("0x1").unwrap();
    let b = Felt::from_hex("0x2").unwrap();
    let a_stark = StarkFelt::from_hex("0x1").unwrap();
    let b_stark = StarkFelt::from_hex("0x2").unwrap();
    let inputs = vec![a, b];
    let expected = poseidon_hash(a_stark, b_stark);
    test_hash_with_inputs(&inputs, expected);
}

fn test_hash_with_inputs(inputs: &[Felt], expected: StarkFelt) {
    // Create a stack and push the PoseidonHashMany task
    let mut stack = BidirectionalStackAccount::default();

    // Create the PoseidonHashMany task with the stack reference
    let hash_task = PoseidonHash::new();
    PoseidonHash::push_input(inputs[0], inputs[1], &mut stack);
    stack.push_task(hash_task);

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    // Get the result from the stack
    let bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(bytes);
    stack.pop_front();
    stack.pop_front();
    stack.pop_front();

    let result_bytes = result.to_bytes_be();
    let result_stark = StarkFelt::from_bytes_be_slice(&result_bytes);
    // Verify the result
    assert_eq!(result_stark, expected);
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
}
