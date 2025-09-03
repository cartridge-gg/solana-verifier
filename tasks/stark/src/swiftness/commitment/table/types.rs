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

impl CommitmentTrait<Decommitment> for Decommitment {
    fn from_stack<T: BidirectionalStack>(stack: &mut T) -> Self {
        // Read values length
        let values_len = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        println!("values_len: {:?}", values_len);
        let len = values_len.to_biguint().try_into().unwrap();

        // Read values
        let mut values = FunVec::default();
        for _ in 0..len {
            let value = Felt::from_bytes_be_slice(stack.borrow_front());
            stack.pop_front();
            values.push(value);
        }

        // Read montgomery_values length
        let montgomery_len = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        println!("montgomery_len: {:?}", montgomery_len);
        let montgomery_len_usize = montgomery_len.to_biguint().try_into().unwrap();

        // Read montgomery_values
        let mut montgomery_values = FunVec::default();
        for _ in 0..montgomery_len_usize {
            let value = Felt::from_bytes_be_slice(stack.borrow_front());
            stack.pop_front();
            montgomery_values.push(value);
        }

        Self {
            values,
            montgomery_values,
        }
    }

    fn push_to_stack<T: BidirectionalStack>(&self, stack: &mut T) {
        // Push montgomery_values in reverse order
        for value in self.montgomery_values.as_slice().iter().rev() {
            stack.push_front(&value.to_bytes_be()).unwrap();
        }
        stack
            .push_front(&Felt::from(self.montgomery_values.len()).to_bytes_be())
            .unwrap();

        // Push values in reverse order
        for value in self.values.as_slice().iter().rev() {
            stack.push_front(&value.to_bytes_be()).unwrap();
        }
        stack
            .push_front(&Felt::from(self.values.len()).to_bytes_be())
            .unwrap();
    }

    fn to_bytes_be(&self) -> Decommitment {
        *self
    }
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
