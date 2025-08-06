use felt::Felt;
use stark::stark_proof::stark_commit::eval_oods_polynomial_inner::EvalOodsPolynomialInner;
use stark::swiftness::air::recursive_with_poseidon::GlobalValues;
use stark::swiftness::stark::types::StarkProof;
use utils::global_values::EcPoint;
use utils::{BidirectionalStack, Scheduler, OODS_VALUES_SIZE};
use verifier::state::BidirectionalStackAccount;
mod fixtures;
use fixtures::{fri_config, fri_unsent_commitment, oods_values, stark_config};
use stark::swiftness::air::recursive_with_poseidon::Layout;

use crate::fixtures::constraint_coefficients;

#[test]
fn test_eval_oods_polynomial_inner() {
    let mut stack = BidirectionalStackAccount::default();
    let mut proof = StarkProof::default();

    proof.config.fri = fri_config::get();
    proof.unsent_commitment.fri = fri_unsent_commitment::get();
    proof.config = stark_config::get();
    stack.proof = proof;

    let column_values = vec![
        Felt::from_hex("0x4259480c2e22c881ee19f1a33a3ba4741be9d482d60a1e101caa1880a60da4a")
            .unwrap(),
        Felt::from_hex("0x30e9bd02ed5c2b63c21aea250864b92954874321b8e435b4094c4dc0eee66ed")
            .unwrap(),
        Felt::from_hex("0x10d0a289021b4237e35246efcadec0bc8ba9639d3a4fe8ee0fabcb9b1debd87")
            .unwrap(),
        Felt::from_hex("0x1a3f12b77f078a954ab0679e396b9bb4f983926be28f8e51aa3ed96071a200d")
            .unwrap(),
        Felt::from_hex("0x495d4e5eabd1299ab4e710d293df100d2cb92c3e660d16c1a5c2f9ce8b0ec6a")
            .unwrap(),
        Felt::from_hex("0x7c790c2b3715747359636cc557168400f6dfa53f69d39f6566cf849333034f2")
            .unwrap(),
        Felt::from_hex("0x5396ee71fe13db8760e752eae2c345b478bb742d474e06a026d19020355a514")
            .unwrap(),
        Felt::from_hex("0x2e6bfcb0a0dc9bc65f65a09947db8c653d138a0fefe55275c9c1d42b4be1b97")
            .unwrap(),
        Felt::from_hex("0x1a0cebd05a8079567be06d342813a5726b0094dfe5294ebb221bdfc5ff5006a")
            .unwrap(),
        Felt::from_hex("0x6c6059de8def374197a382cb19880e30f1aa80a009bf4c3db6dfa5656f308").unwrap(),
    ];

    let oods_values = oods_values::get();
    let oods_slice = &oods_values.as_slice()[0..OODS_VALUES_SIZE];

    stack.column_values = column_values.try_into().unwrap();
    stack.oods_values = oods_slice.try_into().unwrap();
    stack.constraint_coefficients = constraint_coefficients::get()
        .as_slice()
        .try_into()
        .unwrap();

    let global_values = GlobalValues {
        trace_length: Felt::from_hex("0x10000000").unwrap(),
        initial_pc: Felt::from_hex("0x1").unwrap(),
        final_pc: Felt::from_hex("0x5").unwrap(),
        initial_ap: Felt::from_hex("0x1c6").unwrap(),
        final_ap: Felt::from_hex("0x1c43b3").unwrap(),
        initial_pedersen_addr: Felt::from_hex("0x1c43b8").unwrap(),
        initial_range_check_addr: Felt::from_hex("0x1f43b8").unwrap(),
        initial_bitwise_addr: Felt::from_hex("0x2f43b8").unwrap(),
        initial_poseidon_addr: Felt::from_hex("0x7f43b8").unwrap(),
        range_check_min: Felt::from_hex("0x0").unwrap(),
        range_check_max: Felt::from_hex("0xffff").unwrap(),
        offset_size: Felt::from_hex("0x10000").unwrap(),
        half_offset_size: Felt::from_hex("0x8000").unwrap(),
        pedersen_shift_point: EcPoint {
            x: Felt::from_hex("0x49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804")
                .unwrap(),
            y: Felt::from_hex("0x3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a")
                .unwrap(),
        },
        pedersen_points_x: Felt::from_hex(
            "0x598904d65b0434a87c175e65222359d01fff2522cade3bb409c28885b7671e",
        )
        .unwrap(),
        pedersen_points_y: Felt::from_hex(
            "0x4fe4068e06eefa17eefab622b3c9d9433bc11552fd96bf324893028770e40f4",
        )
        .unwrap(),
        poseidon_poseidon_full_round_key0: Felt::from_hex(
            "0x4f7c465fb34210b739758542eb985867c6ba4ec77b078ccb61b8e4288cbbae8",
        )
        .unwrap(),
        poseidon_poseidon_full_round_key1: Felt::from_hex(
            "0x2f96e26e8a7034b6317c2483e935e6bd1d5ea8efa42dc84ebba571760a1527d",
        )
        .unwrap(),
        poseidon_poseidon_full_round_key2: Felt::from_hex(
            "0x79e52af7b64407d08c6b7b54d92ea2477b7120da296f986f0d52705a850043d",
        )
        .unwrap(),
        poseidon_poseidon_partial_round_key0: Felt::from_hex(
            "0x17d8c8dc5aaa6ac1879e160be09a2012f52e1d6df8e3528255e00fa01f13020",
        )
        .unwrap(),
        poseidon_poseidon_partial_round_key1: Felt::from_hex(
            "0x786dda7880b1250660bec5c62a9c1a255f95c69b9d050d5bc4a89b4accdd89d",
        )
        .unwrap(),
        memory_multi_column_perm_perm_interaction_elm: Felt::from_hex(
            "0x63be95eef090c5ed842139ace99b3dc2e8222f4946d656d2b8ecf9f3a4eaa64",
        )
        .unwrap(),
        memory_multi_column_perm_hash_interaction_elm0: Felt::from_hex(
            "0x522df1ce46453857bc93d7b48c77fd4968ae6be4de52c9a9ebf3b053fe3f288",
        )
        .unwrap(),
        range_check16_perm_interaction_elm: Felt::from_hex(
            "0x47256c1d9e69a2c23e0a5b2666fd2e2037ef2987d19b53da2b089c7a79e217c",
        )
        .unwrap(),
        diluted_check_permutation_interaction_elm: Felt::from_hex(
            "0x1f44508505278264aabe386ad5df3bee4b8147b3d0e20518bfaec709cbc1322",
        )
        .unwrap(),
        diluted_check_interaction_z: Felt::from_hex(
            "0x7f01d79f2cdf6aa851c9b2e0fa2e92f64ecd655289f827b14d5e7b483f52b48",
        )
        .unwrap(),
        diluted_check_interaction_alpha: Felt::from_hex(
            "0x734820597aa2142c285a8ab4990f17ba4241a78de519e3661dafd9453a8e822",
        )
        .unwrap(),
        memory_multi_column_perm_perm_public_memory_prod: Felt::from_hex(
            "0x5593c3e7c28433d4bed879adb1cb8081b0a46decda462e76da45b0d7244cbf0",
        )
        .unwrap(),
        range_check16_perm_public_memory_prod: Felt::from_hex("0x1").unwrap(),
        diluted_check_first_elm: Felt::from_hex("0x0").unwrap(),
        diluted_check_permutation_public_memory_prod: Felt::from_hex("0x1").unwrap(),
        diluted_check_final_cum_val: Felt::from_hex(
            "0x5f16ce646fe7bef242b9158006cb52930937bf075c6e8bc638bba2b8244dfa",
        )
        .unwrap(),
    };

    stack.global_values = global_values;

    let point = Felt::from_hex("0x7f90255cc310f54635400a0fc3ad5d4dcd9afb685485297d828f04cb9c29fcb")
        .unwrap();
    let oods_point =
        Felt::from_hex("0x49185430497be4bd990699e70b3b91b25c0dd22d5cd436dbf23f364136368bc")
            .unwrap();
    let trace_generator =
        Felt::from_hex("0x57a797181c06d8427145cb66056f032751615d8617c5468258e96d2bb6422f9")
            .unwrap();

    stack.push_front(&trace_generator.to_bytes_be()).unwrap();
    stack.push_front(&oods_point.to_bytes_be()).unwrap();
    stack.push_front(&point.to_bytes_be()).unwrap();

    stack.push_task(EvalOodsPolynomialInner::new());

    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    println!("Executed {} steps", steps);

    let result = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    use stark::swiftness::air::recursive_with_poseidon::autogenerated::autogenerated_oods::eval_oods_polynomial_inner;
    let expected_result = eval_oods_polynomial_inner::<Layout>(
        &stack.column_values,
        &stack.oods_values,
        &stack.constraint_coefficients,
        &point,
        &oods_point,
        &trace_generator,
    );

    println!("Expected result: {:?}", expected_result);
    println!("Actual result:   {:?}", result);

    assert!(steps > 0, "Should have executed at least one step");
    assert!(stack.is_empty_back(), "Stack should be empty");
    assert!(stack.is_empty_front(), "Stack should be empty");
    assert_eq!(result, expected_result);
}
