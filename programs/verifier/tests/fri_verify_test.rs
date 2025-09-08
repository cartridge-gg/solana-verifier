mod fixtures;

use felt::Felt;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;
#[test]
fn test_fri_verify() {
    let mut stack = BidirectionalStackAccount::default();
    let task = stark::stark_proof::stark_verify::FriVerify::new();

    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }
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
    let fri_commitment = fixtures::fri_commitment::get();
    let fri_decommitment = fixtures::fri_decommitment::get();
    let witness = fixtures::witness::get();
}
