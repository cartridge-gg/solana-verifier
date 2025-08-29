use crate::swiftness::commitment::table::config::Config;
use crate::{
    funvec::{FunVec, FUNVEC_DECOMMITMENT_VALUES},
    swiftness::commitment::vector,
};
use felt::Felt;

// Commitment for a table (n_rows x n_columns) of field elements in montgomery form.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Commitment {
    pub config: Config,
    pub vector_commitment: vector::types::Commitment,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Decommitment {
    // n_columns * n_queries values to decommit.
    pub values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
    pub montgomery_values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Witness {
    pub vector: vector::types::Witness,
}
