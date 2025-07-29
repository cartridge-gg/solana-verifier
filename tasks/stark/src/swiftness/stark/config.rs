use crate::felt::Felt;
use crate::swiftness::air::trace;
use crate::swiftness::commitment;
use crate::swiftness::fri::config::Config as FriConfig;
use crate::swiftness::pow;
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StarkConfig {
    pub traces: trace::config::Config,
    pub composition: commitment::table::config::Config,
    pub fri: FriConfig,
    pub proof_of_work: pow::config::Config,
    // Log2 of the trace domain size.
    pub log_trace_domain_size: Felt,
    // Number of queries to the last component, FRI.
    pub n_queries: Felt,
    // Log2 of the number of cosets composing the evaluation domain, where the coset size is the
    // trace length.
    pub log_n_cosets: Felt,
    // Number of layers that use a verifier friendly hash in each commitment.
    pub n_verifier_friendly_commitment_layers: Felt,
}
