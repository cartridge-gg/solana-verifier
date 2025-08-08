use felt::Felt;
use stark::stark_proof::stark_commit::VerifyOods;
use stark::swiftness::stark::types::StarkProof;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;
mod fixtures;
use crate::fixtures::constraint_coefficients;
use fixtures::{fri_unsent_commitment, oods_values, public_input, stark_config, stark_domains};
use utils::transcript::Transcript;

#[test]
fn test_verify_oods_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    let transcript = Transcript::new_with_counter(
        Felt::from_hex_unchecked(
            "0xaf91f2c71f4a594b1575d258ce82464475c82d8fb244142d0db450491c1b52",
        ),
        Felt::from_hex_unchecked("0x0"),
    );

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

    //Push the interaction elements to stack for testing (in reverse order as they will be popped)
    // diluted_check_interaction_alpha
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x734820597aa2142c285a8ab4990f17ba4241a78de519e3661dafd9453a8e822",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // diluted_check_interaction_z
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x7f01d79f2cdf6aa851c9b2e0fa2e92f64ecd655289f827b14d5e7b483f52b48",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // diluted_check_permutation_interaction_elm
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x1f44508505278264aabe386ad5df3bee4b8147b3d0e20518bfaec709cbc1322",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // range_check16_perm_interaction_elm
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x47256c1d9e69a2c23e0a5b2666fd2e2037ef2987d19b53da2b089c7a79e217c",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // memory_multi_column_perm_hash_interaction_elm0
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x522df1ce46453857bc93d7b48c77fd4968ae6be4de52c9a9ebf3b053fe3f288",
            )
            .to_bytes_be(),
        )
        .unwrap();
    // memory_multi_column_perm_perm_interaction_elm
    stack
        .push_front(
            &Felt::from_hex_unchecked(
                "0x63be95eef090c5ed842139ace99b3dc2e8222f4946d656d2b8ecf9f3a4eaa64",
            )
            .to_bytes_be(),
        )
        .unwrap();

    let point = Felt::from_hex("0x7f90255cc310f54635400a0fc3ad5d4dcd9afb685485297d828f04cb9c29fcb")
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

    println!("VerifyOods test completed successfully in {} steps", steps);
}
