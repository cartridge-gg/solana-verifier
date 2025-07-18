use stark::felt::Felt;
use stark::stark_proof::stark_commit::helpers::PowersArray;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_powers_array() {
    let mut stack = BidirectionalStackAccount::default();

    // Test PowersArray generating [1, alpha, alpha^2] where alpha = 2
    let alpha = Felt::from(2);
    let initial = Felt::ONE;
    let count = 3;

    // Push inputs: (initial, alpha)
    stack.push_front(&alpha.to_bytes_be()).unwrap();
    stack.push_front(&initial.to_bytes_be()).unwrap();

    let powers_task = PowersArray::new(count);
    stack.push_task(powers_task);

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    println!("Final stack size: {}", stack.back_index - stack.front_index);

    // Verify results: read all values and print them
    let mut results = Vec::new();
    while stack.front_index != 0_usize {
        let value = Felt::from_bytes_be_slice(stack.borrow_front());
        println!("Stack value: {:?}", value);
        results.push(value);
        stack.pop_front();
    }

    assert_eq!(results.len(), 3, "Should have 3 results");
    let results = results.into_iter().rev().collect::<Vec<_>>();
    let (result0, result1, result2) = (results[0], results[1], results[2]);

    assert_eq!(result0, Felt::ONE, "Powers[0] should be 1");
    assert_eq!(result1, Felt::from(2), "Powers[1] should be 2");
    assert_eq!(result2, Felt::from(4), "Powers[2] should be 4");
    assert!(steps > 0, "Should have executed at least one step");
}
