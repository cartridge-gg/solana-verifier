mod fixtures;
use felt::Felt;
use stark::stark_proof::get_hash::GetHash;
use starknet_crypto::pedersen_hash;
use starknet_crypto::{poseidon_hash_many, Felt as StarkFelt};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn get_hash() {
    let mut stack = BidirectionalStackAccount::default();

    let input = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let proof_verifier = proof.transform_to();
    stack.proof = proof_verifier.clone();
    stack.push_task(GetHash::new(
        proof_verifier.config.n_verifier_friendly_commitment_layers,
    ));
    while !stack.is_empty_back() {
        stack.execute();
    }

    let expected = calculate_expected_get_hash(
        &stack.proof.public_input,
        stack.proof.config.n_verifier_friendly_commitment_layers,
    );

    let result = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    assert_eq!(expected, result);
    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");
}

// This implements the original get_hash logic synchronously for comparison
fn calculate_expected_get_hash(
    public_input: &stark::swiftness::air::public_memory::PublicInput,
    n_verifier_friendly_commitment_layers: Felt,
) -> Felt {
    let mut main_page_hash = StarkFelt::ZERO;
    for memory in public_input.main_page.0.iter() {
        let address_bytes = memory.address.to_bytes_be();
        let value_bytes = memory.value.to_bytes_be();
        let address_starknet = StarkFelt::from_bytes_be(&address_bytes);
        let value_starknet = StarkFelt::from_bytes_be(&value_bytes);
        main_page_hash = pedersen_hash(&main_page_hash, &address_starknet);
        main_page_hash = pedersen_hash(&main_page_hash, &value_starknet);
    }
    let length_multiplier_bytes =
        (Felt::TWO * Felt::from(public_input.main_page.0.len())).to_bytes_be();
    let length_multiplier = StarkFelt::from_bytes_be(&length_multiplier_bytes);
    main_page_hash = pedersen_hash(&main_page_hash, &length_multiplier);

    let mut hash_data = vec![
        StarkFelt::from_bytes_be(&n_verifier_friendly_commitment_layers.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.log_n_steps.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.range_check_min.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.range_check_max.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.layout.to_bytes_be()),
    ];

    if let Some(dynamic_params) = &public_input.dynamic_params {
        let dynamic_params_vec: Vec<u32> = (*dynamic_params).into();
        hash_data.extend(dynamic_params_vec.into_iter().map(|x| StarkFelt::from(x)));
    }

    hash_data.extend(public_input.segments.iter().flat_map(|s| {
        let begin_addr_bytes = s.begin_addr.to_bytes_be();
        let stop_ptr_bytes = s.stop_ptr.to_bytes_be();
        vec![
            StarkFelt::from_bytes_be(&begin_addr_bytes),
            StarkFelt::from_bytes_be(&stop_ptr_bytes),
        ]
    }));

    let padding_addr_bytes = public_input.padding_addr.to_bytes_be();
    let padding_value_bytes = public_input.padding_value.to_bytes_be();
    hash_data.push(StarkFelt::from_bytes_be(&padding_addr_bytes));
    hash_data.push(StarkFelt::from_bytes_be(&padding_value_bytes));

    hash_data.push(StarkFelt::from(
        public_input.continuous_page_headers.len() + 1,
    ));

    hash_data.push(StarkFelt::from(public_input.main_page.0.len()));
    hash_data.push(main_page_hash);

    hash_data.extend(public_input.continuous_page_headers.iter().flat_map(|h| {
        let start_address_bytes = h.start_address.to_bytes_be();
        let size_bytes = h.size.to_bytes_be();
        let hash_bytes = h.hash.to_bytes_be();
        vec![
            StarkFelt::from_bytes_be(&start_address_bytes),
            StarkFelt::from_bytes_be(&size_bytes),
            StarkFelt::from_bytes_be(&hash_bytes),
        ]
    }));

    let result_starknet = poseidon_hash_many(&hash_data[..]);
    Felt::from_bytes_be(&result_starknet.to_bytes_be())
}
