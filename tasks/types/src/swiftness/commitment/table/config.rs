use crate::swiftness::commitment::vector::{self, config::VectorConfigBytes};
use felt::Felt;
use vector::config::Config as VectorConfig;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Config {
    pub n_columns: Felt,
    pub vector: VectorConfig,
}
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct TableConfigBytes {
    pub n_columns: [u8; 32],
    pub vector: VectorConfigBytes,
}
