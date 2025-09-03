use crate::swiftness::commitment::table::config::{Config, TableConfigBytes};
use crate::swiftness::commitment::vector::config::ConfigTrait;
use crate::swiftness::commitment::vector::types::{CommitmentTrait, VectorCommitmentBytes};
use crate::{
    funvec::{FunVec, FUNVEC_DECOMMITMENT_VALUES},
    swiftness::commitment::vector::{self, types::Commitment as VectorCommitment},
};
use felt::Felt;
use utils::BidirectionalStack;

// Commitment for a table (n_rows x n_columns) of field elements in montgomery form.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

pub struct TableCommitmentBytes {
    pub config: TableConfigBytes,
    pub vector_commitment: VectorCommitmentBytes,
}

impl CommitmentTrait<TableCommitmentBytes> for Commitment {
    fn from_stack<T: BidirectionalStack>(stack: &mut T) -> Self {
        let config = Config::from_stack(stack);
        let value = VectorCommitment::from_stack(stack);
        Self::new(config, value)
    }

    fn push_to_stack<T: BidirectionalStack>(&self, stack: &mut T) {
        self.vector_commitment.push_to_stack(stack);
        self.config.push_to_stack(stack);
    }

    fn to_bytes_be(&self) -> TableCommitmentBytes {
        TableCommitmentBytes {
            vector_commitment: self.vector_commitment.to_bytes_be(),
            config: self.config.to_bytes_be(),
        }
    }
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

impl CommitmentTrait<Witness> for Witness {
    fn from_stack<T: BidirectionalStack>(stack: &mut T) -> Self {
        let vector = vector::types::Witness::from_stack(stack);
        Self { vector }
    }

    fn push_to_stack<T: BidirectionalStack>(&self, stack: &mut T) {
        self.vector.push_to_stack(stack);
    }

    fn to_bytes_be(&self) -> Witness {
        *self
    }
}
