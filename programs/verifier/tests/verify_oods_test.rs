use felt::Felt;
use stark::stark_proof::stark_commit::VerifyOods;
use stark::swiftness::stark::types::StarkProof;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;
mod fixtures;
use crate::fixtures::constraint_coefficients;
use crate::fixtures::stark_commitment;
use fixtures::{fri_unsent_commitment, oods_values, public_input, stark_config, stark_domains};

#[test]
fn test_verify_oods_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    let public_input = public_input::get();
    let unsent_commitment = fri_unsent_commitment::get();
    let config = stark_config::get();
    let stark_domains = stark_domains::get();
    let constraint_coefficients = constraint_coefficients::get();

    stack.constraint_coefficients = constraint_coefficients.as_slice().try_into().unwrap();

    // Create a StarkProof with reference oods_values
    let mut proof = StarkProof::default();

    let oods_values = oods_values::get();

    proof.unsent_commitment.oods_values = oods_values;
    stack.oods_values = oods_values.as_slice().try_into().unwrap();

    proof.unsent_commitment.fri = unsent_commitment;
    proof.config = config;
    proof.public_input = public_input;
    stack.proof = proof;

    let mut stark_commitment = stark_commitment::get();

    stark_commitment
        .traces
        .interaction_elements
        .memory_multi_column_perm_perm_interaction_elm = Felt::from_hex_unchecked(
        "0x63be95eef090c5ed842139ace99b3dc2e8222f4946d656d2b8ecf9f3a4eaa64",
    );
    stark_commitment
        .traces
        .interaction_elements
        .memory_multi_column_perm_hash_interaction_elm0 = Felt::from_hex_unchecked(
        "0x522df1ce46453857bc93d7b48c77fd4968ae6be4de52c9a9ebf3b053fe3f288",
    );
    stark_commitment
        .traces
        .interaction_elements
        .range_check16_perm_interaction_elm = Felt::from_hex_unchecked(
        "0x47256c1d9e69a2c23e0a5b2666fd2e2037ef2987d19b53da2b089c7a79e217c",
    );
    stark_commitment
        .traces
        .interaction_elements
        .diluted_check_permutation_interaction_elm = Felt::from_hex_unchecked(
        "0x1f44508505278264aabe386ad5df3bee4b8147b3d0e20518bfaec709cbc1322",
    );
    stark_commitment
        .traces
        .interaction_elements
        .diluted_check_interaction_z = Felt::from_hex_unchecked(
        "0x7f01d79f2cdf6aa851c9b2e0fa2e92f64ecd655289f827b14d5e7b483f52b48",
    );
    stark_commitment
        .traces
        .interaction_elements
        .diluted_check_interaction_alpha = Felt::from_hex_unchecked(
        "0x734820597aa2142c285a8ab4990f17ba4241a78de519e3661dafd9453a8e822",
    );

    stack.stark_commitment = stark_commitment;

    let point = Felt::from_hex("0x49185430497be4bd990699e70b3b91b25c0dd22d5cd436dbf23f364136368bc")
        .unwrap();

    stack
        .push_front(&stark_domains.trace_domain_size.to_bytes_be())
        .unwrap();
    stack
        .push_front(&stark_domains.trace_generator.to_bytes_be())
        .unwrap();
    stack.push_front(&point.to_bytes_be()).unwrap();

    // Push the VerifyOods task
    stack.push_task(VerifyOods::new());

    // Execute tasks until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }
    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");
    println!("VerifyOods test completed successfully in {} steps", steps);
}
