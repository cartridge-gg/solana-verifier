use crate::swiftness::commitment::vector;
use felt::Felt;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Config {
    pub n_columns: Felt,
    pub vector: vector::config::Config,
}
