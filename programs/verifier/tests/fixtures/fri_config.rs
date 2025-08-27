use felt::Felt;
use stark::funvec::FunVec;
use stark::swiftness::commitment::{table, vector};
use stark::swiftness::fri::config::Config as FriConfig;

pub fn get() -> FriConfig {
    FriConfig {
        log_input_size: Felt::from_hex_unchecked("0x20"),
        n_layers: Felt::from_hex_unchecked("0x9"),
        inner_layers: FunVec::from_vec(vec![
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x8"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x1d"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x8"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x1a"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x8"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x17"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x8"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x8"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0x11"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x8"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0xe"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x4"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0xc"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
            table::config::Config {
                n_columns: Felt::from_hex_unchecked("0x4"),
                vector: vector::config::Config {
                    height: Felt::from_hex_unchecked("0xa"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                },
            },
        ]),
        fri_step_sizes: FunVec::from_vec(vec![
            Felt::from_hex_unchecked("0x0"),
            Felt::from_hex_unchecked("0x3"),
            Felt::from_hex_unchecked("0x3"),
            Felt::from_hex_unchecked("0x3"),
            Felt::from_hex_unchecked("0x3"),
            Felt::from_hex_unchecked("0x3"),
            Felt::from_hex_unchecked("0x3"),
            Felt::from_hex_unchecked("0x2"),
            Felt::from_hex_unchecked("0x2"),
        ]),
        log_last_layer_degree_bound: Felt::from_hex_unchecked("0x6"),
    }
}
