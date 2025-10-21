use crate::funvec::{FunVec, FUNVEC_DECOMMITMENT_VALUES};
use crate::swiftness::commitment::table::config::{Config, TableConfigBytes};
use crate::swiftness::commitment::vector::types::VectorCommitmentBytes;
use crate::swiftness::commitment::vector::{self};
use felt::Felt;
// Commitment for a table (n_rows x n_columns) of field elements in montgomery form.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Commitment {
    pub config: Config,
    pub vector_commitment: vector::types::Commitment,
}

impl Commitment {
    pub fn new(config: Config, vector_commitment: vector::types::Commitment) -> Self {
        Self {
            config,
            vector_commitment,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct TableCommitmentBytes {
    pub config: TableConfigBytes,
    pub vector_commitment: VectorCommitmentBytes,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct Decommitment {
    pub values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
    pub montgomery_values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
}

impl Decommitment {
    pub fn new(
        values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
        montgomery_values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
    ) -> Self {
        Self {
            values,
            montgomery_values,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Witness {
    pub vector: vector::types::Witness,
}
