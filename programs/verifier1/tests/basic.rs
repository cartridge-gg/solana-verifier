use utils_1::{BidirectionalStack, Scheduler};
use validate_public_input::validate::ValidatePublicInput;
use verifier_1::state::BidirectionalStackAccount;

#[test]
fn test_hades_permutation() {
    // Create a stack and push the Hades permutation task
    let mut stack = BidirectionalStackAccount::default();
    stack.push_task(ValidatePublicInput::new());

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    println!("Completed in {steps} steps");
}
