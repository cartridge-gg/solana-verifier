use stark::pedersen::points::PedersenEval;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;
#[test]
fn test_pedersen_eval() {
    let mut stack = BidirectionalStackAccount::default();

    // Pedersen point calculated by swiftness for the example saya proof
    let point = felt::Felt::from_hex_unchecked(
        "0x55883272fbc6be7532b78c2758584f9f15fd43055c0a06eb3471051bf2d0d4a",
    );
    let task = PedersenEval::new(point);
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }

    let expected_pedersen_points_y = felt::Felt::from_hex_unchecked(
        "0x4fe4068e06eefa17eefab622b3c9d9433bc11552fd96bf324893028770e40f4",
    );
    let expected_pedersen_points_x = felt::Felt::from_hex_unchecked(
        "0x598904d65b0434a87c175e65222359d01fff2522cade3bb409c28885b7671e",
    );

    let result_y = felt::Felt::from_bytes_be_slice(stack.borrow_front().try_into().unwrap());
    stack.pop_front();
    let result_x = felt::Felt::from_bytes_be_slice(stack.borrow_front().try_into().unwrap());
    stack.pop_front();
    assert_eq!(result_y, expected_pedersen_points_y);
    assert_eq!(result_x, expected_pedersen_points_x);
    assert!(stack.is_empty_front());
}
  