use felt::Felt;
use stark::poseidon::hades::HadesPermutation;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_hades_permutation() {
    // Initialize a state to test
    let state = [
        Felt::from_hex("0x9").unwrap(),
        Felt::from_hex("0xb").unwrap(),
        Felt::from_hex("0x2").unwrap(),
    ];

    // Create a stack and push the Hades permutation task
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(HadesPermutation::new(state));

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    // Get the result from the stack
    let bytes = stack.borrow_front();
    let result1 = Felt::from_bytes_be_slice(bytes);
    stack.pop_front();
    let bytes = stack.borrow_front();
    let result2 = Felt::from_bytes_be_slice(bytes);
    stack.pop_front();
    let bytes = stack.borrow_front();
    let result3 = Felt::from_bytes_be_slice(bytes);
    stack.pop_front();

    // The expected output should match the result we got
    let expected_result1 =
        Felt::from_hex("0x510f3a3faf4084e3b1e95fd44c30746271b48723f7ea9c8be6a9b6b5408e7e6")
            .unwrap();
    let expected_result2 =
        Felt::from_hex("0x4f511749bd4101266904288021211333fb0a514cb15381af087462fa46e6bd9")
            .unwrap();
    let expected_result3 =
        Felt::from_hex("0x186f6dd1a6e79cb1b66d505574c349272cd35c07c223351a0990410798bb9d8")
            .unwrap();

    assert_eq!(result1, expected_result1);
    assert_eq!(result2, expected_result2);
    assert_eq!(result3, expected_result3);
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
}
