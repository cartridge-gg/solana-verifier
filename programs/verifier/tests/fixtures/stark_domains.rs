use felt::Felt;
use stark::swiftness::air::domains::StarkDomains;

pub fn get() -> StarkDomains {
    StarkDomains {
        log_eval_domain_size: Felt::from_hex_unchecked("0x20"),
        eval_domain_size: Felt::from_hex_unchecked("0x100000000"),
        eval_generator: Felt::from_hex_unchecked(
            "0x50732ed0be8ced2fea566de48221e1a719252eb81c43de5c129d0f1d3ce8992",
        ),
        log_trace_domain_size: Felt::from_hex_unchecked("0x1c"),
        trace_domain_size: Felt::from_hex_unchecked("0x10000000"),
        trace_generator: Felt::from_hex_unchecked(
            "0x57a797181c06d8427145cb66056f032751615d8617c5468258e96d2bb6422f9",
        ),
    }
}
