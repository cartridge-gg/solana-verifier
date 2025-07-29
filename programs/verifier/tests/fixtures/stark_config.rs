use stark::felt::Felt;
use stark::swiftness::stark::config::StarkConfig;

pub fn get() -> StarkConfig {
    StarkConfig {
        traces: Default::default(), // You can fill with actual config if needed
        composition: Default::default(),
        fri: super::fri_config::get(),
        proof_of_work: Default::default(),
        log_trace_domain_size: Felt::from_hex_unchecked("0x14"),
        n_queries: Felt::from_hex_unchecked("0x12"),
        log_n_cosets: Felt::from_hex_unchecked("0x4"),
        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
    }
}
