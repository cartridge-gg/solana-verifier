use felt::Felt;
use stark::poseidon::PoseidonHashMany;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_poseidon_hash_1() {
    let a = Felt::from_hex("0x1").unwrap();

    let inputs = vec![a];
    let expected =
        Felt::from_hex("0x579e8877c7755365d5ec1ec7d3a94a457eff5d1f40482bbe9729c064cdead2").unwrap();
    test_hash_with_inputs(&inputs, expected);
}

#[test]
fn test_poseidon_hash_2() {
    let a = Felt::from_hex("0x1").unwrap();
    let b = Felt::from_hex("0x2").unwrap();
    let inputs = vec![a, b];
    let expected =
        Felt::from_hex("0x371cb6995ea5e7effcd2e174de264b5b407027a75a231a70c2c8d196107f0e7")
            .unwrap();
    test_hash_with_inputs(&inputs, expected);
}

#[test]
fn test_poseidon_hash_3() {
    let a = Felt::from_hex("0x1").unwrap();
    let b = Felt::from_hex("0x2").unwrap();
    let c = Felt::from_hex("0x3").unwrap();
    let inputs = vec![a, b, c];
    let expected =
        Felt::from_hex("0x2f0d8840bcf3bc629598d8a6cc80cb7c0d9e52d93dab244bbf9cd0dca0ad082")
            .unwrap();
    test_hash_with_inputs(&inputs, expected);
}

#[test]
fn test_poseidon_hash_4() {
    let a = Felt::from_hex("0x1").unwrap();
    let b = Felt::from_hex("0x2").unwrap();
    let c = Felt::from_hex("0x3").unwrap();
    let d = Felt::from_hex("0x4").unwrap();
    let inputs = vec![a, b, c, d];
    let expected =
        Felt::from_hex("0x26e3ad8b876e02bc8a4fc43dad40a8f81a6384083cabffa190bcf40d512ae1d")
            .unwrap();
    test_hash_with_inputs(&inputs, expected);
}

fn test_hash_with_inputs(inputs: &[Felt], expected: Felt) {
    // Create a stack and push the PoseidonHashMany task
    let mut stack = BidirectionalStackAccount::default();

    // Create the PoseidonHashMany task with the stack reference
    let hash_task = PoseidonHashMany::new(inputs.len());
    PoseidonHashMany::push_input(inputs, &mut stack);
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

    // Verify the result
    assert_eq!(result, expected);
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
}
