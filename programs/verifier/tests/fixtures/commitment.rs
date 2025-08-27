use felt::Felt;
use stark::swiftness::air::trace::Commitment;
use stark::swiftness::commitment::table::config::Config as TableCommitmentConfig;
use stark::swiftness::commitment::table::types::Commitment as TableCommitment;
use stark::swiftness::commitment::vector::config::Config as VectorCommitmentConfig;
use stark::swiftness::commitment::vector::types::Commitment as VectorCommitment;
use utils::global_values::InteractionElements;

pub fn get() -> Commitment<InteractionElements> {
    Commitment {
        original: TableCommitment {
            config: TableCommitmentConfig {
                n_columns: Felt::from_hex_unchecked("0x6"),
                vector: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x20"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            vector_commitment: VectorCommitment {
                config: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x20"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
                commitment_hash: Felt::from_hex_unchecked(
                    "0x305f1ee7c0b38a403b2fa7ec86a3d11c8a174891194a2c656147268b59e876d",
                ),
            },
        },
        interaction_elements: InteractionElements {
            memory_multi_column_perm_perm_interaction_elm: Felt::from_hex_unchecked(
                "0x63be95eef090c5ed842139ace99b3dc2e8222f4946d656d2b8ecf9f3a4eaa64",
            ),
            memory_multi_column_perm_hash_interaction_elm0: Felt::from_hex_unchecked(
                "0x522df1ce46453857bc93d7b48c77fd4968ae6be4de52c9a9ebf3b053fe3f288",
            ),
            range_check16_perm_interaction_elm: Felt::from_hex_unchecked(
                "0x47256c1d9e69a2c23e0a5b2666fd2e2037ef2987d19b53da2b089c7a79e217c",
            ),
            diluted_check_permutation_interaction_elm: Felt::from_hex_unchecked(
                "0x1f44508505278264aabe386ad5df3bee4b8147b3d0e20518bfaec709cbc1322",
            ),
            diluted_check_interaction_z: Felt::from_hex_unchecked(
                "0x7f01d79f2cdf6aa851c9b2e0fa2e92f64ecd655289f827b14d5e7b483f52b48",
            ),
            diluted_check_interaction_alpha: Felt::from_hex_unchecked(
                "0x734820597aa2142c285a8ab4990f17ba4241a78de519e3661dafd9453a8e822",
            ),
        },
        interaction: TableCommitment {
            config: TableCommitmentConfig {
                n_columns: Felt::from_hex_unchecked("0x2"),
                vector: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x20"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            vector_commitment: VectorCommitment {
                config: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x20"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
                commitment_hash: Felt::from_hex_unchecked(
                    "0x6d41514e4a6e39f5b4e5f18f234525df1d2d92393c11ce11bd885615c88406",
                ),
            },
        },
    }
}
