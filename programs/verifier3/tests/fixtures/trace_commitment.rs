use felt::Felt;
use types::swiftness::air::trace::Commitment as TraceCommitment;
use types::swiftness::commitment::table::config::Config as TableConfig;
use types::swiftness::commitment::table::types::Commitment as TableCommitment;
use types::swiftness::commitment::vector::config::Config as VectorConfig;
use types::swiftness::commitment::vector::types::Commitment as VectorCommitment;
use types::swiftness::global_values::InteractionElements;

pub fn get() -> TraceCommitment<InteractionElements> {
    // Updated data from commitment.traces
    let height = Felt::from_hex("0x20").unwrap(); // 32
    let n_verifier_friendly_layers = Felt::from_hex("0x17").unwrap(); // 23

    let vector_config = VectorConfig {
        height,
        n_verifier_friendly_commitment_layers: n_verifier_friendly_layers,
    };

    let original_commitment_hash =
        Felt::from_hex("0x305f1ee7c0b38a403b2fa7ec86a3d11c8a174891194a2c656147268b59e876d")
            .unwrap();
    let interaction_commitment_hash =
        Felt::from_hex("0x6d41514e4a6e39f5b4e5f18f234525df1d2d92393c11ce11bd885615c88406").unwrap();

    let vector_commitment_original = VectorCommitment::new(vector_config, original_commitment_hash);
    let vector_commitment_interaction =
        VectorCommitment::new(vector_config, interaction_commitment_hash);

    let n_columns_original = Felt::from(6 as u64);
    let n_columns_interaction = Felt::from(2 as u64);

    let table_config_original = TableConfig {
        n_columns: n_columns_original,
        vector: vector_config,
    };
    let table_config_interaction = TableConfig {
        n_columns: n_columns_interaction,
        vector: vector_config,
    };

    // Set up stark commitment with data from the log
    let table_commitment_original =
        TableCommitment::new(table_config_original, vector_commitment_original);
    let table_commitment_interaction =
        TableCommitment::new(table_config_interaction, vector_commitment_interaction);

    let interaction_elements = InteractionElements {
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
    };

    let trace_commitment = TraceCommitment::<InteractionElements>::new(
        table_commitment_original,
        interaction_elements,
        table_commitment_interaction,
    );

    trace_commitment
}

pub fn get_composition_commitment() -> TableCommitment {
    // Data from commitment.composition
    let height = Felt::from_hex("0x20").unwrap(); // 32
    let n_verifier_friendly_layers = Felt::from_hex("0x17").unwrap(); // 23

    let vector_config = VectorConfig {
        height,
        n_verifier_friendly_commitment_layers: n_verifier_friendly_layers,
    };

    let commitment_hash =
        Felt::from_hex("0x112367c6fef0963c09cd918c7d31159ae7effbf9e16ffe7cac15b7bb4074373")
            .unwrap();

    let vector_commitment = VectorCommitment::new(vector_config, commitment_hash);

    let n_columns = Felt::from(2 as u64);

    let table_config = TableConfig {
        n_columns,
        vector: vector_config,
    };

    let table_commitment = TableCommitment::new(table_config, vector_commitment);

    table_commitment
}
