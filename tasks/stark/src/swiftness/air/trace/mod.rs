pub mod config;

use crate::swiftness::commitment::table;
use felt::Felt;
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct UnsentCommitment {
    pub original: Felt,
    pub interaction: Felt,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Decommitment {
    // Responses for queries to the original trace.
    pub original: table::types::Decommitment,
    // Responses for queries to the interaction trace.
    pub interaction: table::types::Decommitment,
}

// A witness for a decommitment of the AIR traces over queries.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Witness {
    pub original: table::types::Witness,
    pub interaction: table::types::Witness,
}
