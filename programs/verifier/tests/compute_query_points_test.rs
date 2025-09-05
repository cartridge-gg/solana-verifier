mod fixtures;

use felt::Felt;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;
#[test]
fn test_compute_query_points() {
    let mut stack = BidirectionalStackAccount::default();
    let task = stark::stark_proof::stark_verify::ComputeQueryPoints::new();

    push_data(&mut stack);
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }

    let result = fixtures::queries::result();
    let mut computed_points_len = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    for result_point in result.iter() {
        let computed_point = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        assert_eq!(&computed_point, result_point, "Point should match expected");
        computed_points_len -= Felt::ONE;
    }
    assert_eq!(
        computed_points_len,
        Felt::ZERO,
        "All points should be consumed"
    );
    assert_eq!(stack.front_index, 0, "Stack should be empty after test");
    assert_eq!(stack.back_index, 65536, "Stack should be empty after test");
}

// Stack layout post-execution:
// ┌──────────────────────────────┐
// │ point_n                      │
// │ point_n-1                    │
// │   ...                        │
// │ point_1                      │
// │ point_0                      │
// │ point_len                    │
// └──────────────────────────────┘  <- front (stack front)

// Stack layout pre-execution:
// ┌──────────────────────────────┐
// │ query_n                      │
// │ query_n-1                    │
// │   ...                        │
// │ query_1                      │
// │ query_0                      │
// │ queries_len                  │
// │ eval_generator               │
// │ log_eval_domain_size         │
// └──────────────────────────────┘  <- front (stack front)

fn push_data(stack: &mut BidirectionalStackAccount) {
    let queries = fixtures::queries::get();
    let queries_len = Felt::from(queries.len());
    let log_eval_domain_size = Felt::from_hex_unchecked("0x20");
    let eval_generator = Felt::from_hex_unchecked(
        "0x50732ed0be8ced2fea566de48221e1a719252eb81c43de5c129d0f1d3ce8992",
    );
    for query in queries.iter().rev() {
        stack.push_front(&query.to_bytes_be()).unwrap();
    }
    stack.push_front(&queries_len.to_bytes_be()).unwrap();
    stack.push_front(&eval_generator.to_bytes_be()).unwrap();
    stack
        .push_front(&log_eval_domain_size.to_bytes_be())
        .unwrap();
}
