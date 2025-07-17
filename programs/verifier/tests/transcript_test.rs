use stark::felt::Felt;
use stark::swiftness::transcript::{
    TranscriptRandomFelt, TranscriptReadFelt, TranscriptReadFeltVector,
};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_transcript_random_felt() {
    let mut stack = BidirectionalStackAccount::default();

    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let counter = Felt::from_hex("0x1").unwrap();

    // Push input data for TranscriptRandomFelt
    TranscriptRandomFelt::push_input(digest, counter, &mut stack);

    // Create and push the task
    let task = TranscriptRandomFelt::new(digest, counter);
    stack.push_task(task);

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        if steps > 100 {
            panic!("Too many steps, likely infinite loop");
        }
    }

    // Get the result from the stack
    let result_bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(result_bytes);
    stack.pop_front();
    stack.pop_front();
    stack.pop_front();

    println!("TranscriptRandomFelt result: {:?}", result);
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
}

#[test]
fn test_transcript_read_felt() {
    let mut stack = BidirectionalStackAccount::default();

    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let val = Felt::from_hex("0x42").unwrap();

    // Push input data for TranscriptReadFelt
    TranscriptReadFelt::push_input(digest, val, &mut stack);

    // Create and push the task
    let task = TranscriptReadFelt::new(digest, val);
    stack.push_task(task);

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        if steps > 100 {
            panic!("Too many steps, likely infinite loop");
        }
    }

    // Get the result from the stack
    let result_bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(result_bytes);
    stack.pop_front();
    stack.pop_front();
    stack.pop_front();

    println!("TranscriptReadFelt result: {:?}", result);
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
}

#[test]
fn test_transcript_read_felt_vector() {
    let mut stack = BidirectionalStackAccount::default();

    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let values = vec![
        Felt::from_hex("0x42").unwrap(),
        Felt::from_hex("0x43").unwrap(),
        Felt::from_hex("0x44").unwrap(),
    ];

    // Push input data for TranscriptReadFeltVector
    TranscriptReadFeltVector::push_input(digest, &values, &mut stack);

    // Create and push the task
    let task = TranscriptReadFeltVector::new(digest, values);
    stack.push_task(task);

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        if steps > 100 {
            panic!("Too many steps, likely infinite loop");
        }
    }

    // Get the result from the stack
    let result_bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(result_bytes);
    stack.pop_front();
    stack.pop_front();
    stack.pop_front();

    println!("TranscriptReadFeltVector result: {:?}", result);
    assert!(steps > 0, "Should have executed at least one step");
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
}

#[test]
fn test_transcript_random_felt_consistency() {
    // Test that the same inputs produce the same output
    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let counter = Felt::from_hex("0x1").unwrap();

    let mut stack1 = BidirectionalStackAccount::default();
    let mut stack2 = BidirectionalStackAccount::default();

    // First execution
    TranscriptRandomFelt::push_input(digest, counter, &mut stack1);
    let task1 = TranscriptRandomFelt::new(digest, counter);
    stack1.push_task(task1);

    while !stack1.is_empty_back() {
        stack1.execute();
    }

    let result1_bytes = stack1.borrow_front();
    let result1 = Felt::from_bytes_be_slice(result1_bytes);
    stack1.pop_front();
    stack1.pop_front();
    stack1.pop_front();

    // Second execution
    TranscriptRandomFelt::push_input(digest, counter, &mut stack2);
    let task2 = TranscriptRandomFelt::new(digest, counter);
    stack2.push_task(task2);

    while !stack2.is_empty_back() {
        stack2.execute();
    }

    let result2_bytes = stack2.borrow_front();
    let result2 = Felt::from_bytes_be_slice(result2_bytes);
    stack2.pop_front();
    stack2.pop_front();
    stack2.pop_front();

    assert_eq!(result1, result2, "Same inputs should produce same results");
}

#[test]
fn test_transcript_read_felt_consistency() {
    // Test that the same inputs produce the same output
    let digest = Felt::from_hex("0x1234567890abcdef").unwrap();
    let val = Felt::from_hex("0x42").unwrap();

    let mut stack1 = BidirectionalStackAccount::default();
    let mut stack2 = BidirectionalStackAccount::default();

    // First execution
    TranscriptReadFelt::push_input(digest, val, &mut stack1);
    let task1 = TranscriptReadFelt::new(digest, val);
    stack1.push_task(task1);

    while !stack1.is_empty_back() {
        stack1.execute();
    }

    let result1_bytes = stack1.borrow_front();
    let result1 = Felt::from_bytes_be_slice(result1_bytes);
    stack1.pop_front();
    stack1.pop_front();
    stack1.pop_front();

    // Second execution
    TranscriptReadFelt::push_input(digest, val, &mut stack2);
    let task2 = TranscriptReadFelt::new(digest, val);
    stack2.push_task(task2);

    while !stack2.is_empty_back() {
        stack2.execute();
    }

    let result2_bytes = stack2.borrow_front();
    let result2 = Felt::from_bytes_be_slice(result2_bytes);
    stack2.pop_front();
    stack2.pop_front();
    stack2.pop_front();

    assert_eq!(result1, result2, "Same inputs should produce same results");
}
