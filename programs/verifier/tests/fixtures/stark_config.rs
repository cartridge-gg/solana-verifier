use felt::Felt;
use stark::funvec::FunVec;
use stark::swiftness::air::trace::config::Config as TraceConfig;
use stark::swiftness::commitment::table::config::Config as TableConfig;
use stark::swiftness::commitment::vector::config::Config as VectorConfig;
use stark::swiftness::fri::config::Config as FriConfig;
use stark::swiftness::pow::config::Config as PowConfig;
use stark::swiftness::stark::config::StarkConfig;

pub fn get() -> StarkConfig {
    // Vector config for common settings
    let vector_config = VectorConfig {
        height: Felt::from_hex("0x20").unwrap(), // 32
        n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
    };

    // Traces config
    let traces = TraceConfig {
        original: TableConfig {
            n_columns: Felt::from_hex("0x6").unwrap(), // 6
            vector: vector_config,
        },
        interaction: TableConfig {
            n_columns: Felt::from_hex("0x2").unwrap(), // 2
            vector: vector_config,
        },
    };

    // Composition config
    let composition = TableConfig {
        n_columns: Felt::from_hex("0x2").unwrap(), // 2
        vector: vector_config,
    };

    // FRI config
    let fri = FriConfig {
        log_input_size: Felt::from_hex("0x20").unwrap(), // 32
        n_layers: Felt::from_hex("0x9").unwrap(),        // 9
        inner_layers: FunVec::from_vec(vec![
            TableConfig {
                n_columns: Felt::from_hex("0x8").unwrap(), // 8
                vector: VectorConfig {
                    height: Felt::from_hex("0x1d").unwrap(), // 29
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
            TableConfig {
                n_columns: Felt::from_hex("0x8").unwrap(), // 8
                vector: VectorConfig {
                    height: Felt::from_hex("0x1a").unwrap(), // 26
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
            TableConfig {
                n_columns: Felt::from_hex("0x8").unwrap(), // 8
                vector: VectorConfig {
                    height: Felt::from_hex("0x17").unwrap(), // 23
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
            TableConfig {
                n_columns: Felt::from_hex("0x8").unwrap(), // 8
                vector: VectorConfig {
                    height: Felt::from_hex("0x14").unwrap(), // 20
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
            TableConfig {
                n_columns: Felt::from_hex("0x8").unwrap(), // 8
                vector: VectorConfig {
                    height: Felt::from_hex("0x11").unwrap(), // 17
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
            TableConfig {
                n_columns: Felt::from_hex("0x8").unwrap(), // 8
                vector: VectorConfig {
                    height: Felt::from_hex("0xe").unwrap(), // 14
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
            TableConfig {
                n_columns: Felt::from_hex("0x4").unwrap(), // 4
                vector: VectorConfig {
                    height: Felt::from_hex("0xc").unwrap(), // 12
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
            TableConfig {
                n_columns: Felt::from_hex("0x4").unwrap(), // 4
                vector: VectorConfig {
                    height: Felt::from_hex("0xa").unwrap(), // 10
                    n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
                },
            },
        ]),
        fri_step_sizes: FunVec::from_vec(vec![
            Felt::from_hex("0x0").unwrap(), // 0
            Felt::from_hex("0x3").unwrap(), // 3
            Felt::from_hex("0x3").unwrap(), // 3
            Felt::from_hex("0x3").unwrap(), // 3
            Felt::from_hex("0x3").unwrap(), // 3
            Felt::from_hex("0x3").unwrap(), // 3
            Felt::from_hex("0x3").unwrap(), // 3
            Felt::from_hex("0x2").unwrap(), // 2
            Felt::from_hex("0x2").unwrap(), // 2
        ]),
        log_last_layer_degree_bound: Felt::from_hex("0x6").unwrap(), // 6
    };

    // Proof of work config
    let proof_of_work = PowConfig { n_bits: 32 };

    StarkConfig {
        traces,
        composition,
        fri,
        proof_of_work,
        log_trace_domain_size: Felt::from_hex("0x1c").unwrap(), // 28
        n_queries: Felt::from_hex("0x10").unwrap(),             // 16
        log_n_cosets: Felt::from_hex("0x4").unwrap(),           // 4
        n_verifier_friendly_commitment_layers: Felt::from_hex("0x17").unwrap(), // 23
    }
}
