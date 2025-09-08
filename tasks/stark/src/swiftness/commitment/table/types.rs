use crate::funvec::{FunVec, FUNVEC_DECOMMITMENT_VALUES};
use crate::swiftness::commitment::table::config::{Config, TableConfigBytes};
use crate::swiftness::commitment::vector::config::ConfigTrait;
use crate::swiftness::commitment::vector::types::{CommitmentTrait, VectorCommitmentBytes};
use crate::swiftness::commitment::vector::{self};
use crate::swiftness::stark::types::{cast_slice_to_struct, cast_struct_to_slice, VerifyVariables};
use felt::Felt;
use utils::{BidirectionalStack, StarkVerifyTrait};

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
    fn from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) -> Self {
        let data = stack.borrow_front();
        let commitment_ref = cast_slice_to_struct::<Self>(data);
        let commitment = *commitment_ref; // Copy only when needed
        stack.pop_front();
        commitment
    }

    fn from_stack_ref<T: BidirectionalStack + StarkVerifyTrait>(stack: &T) -> &Self {
        let data = stack.borrow_front();
        cast_slice_to_struct::<Self>(data)
    }

    fn push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(&mut self, stack: &mut T) {
        let commitment_bytes = cast_struct_to_slice(self);
        stack.push_front(commitment_bytes).unwrap();
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
    pub values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
    pub montgomery_values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
}

impl CommitmentTrait<Decommitment, ()> for Decommitment {
    fn from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) {
        // Read values length
        let values_len = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        let count = values_len.to_biguint().try_into().unwrap();
        println!("count: {:?}", count);

        // Read decommitment_values
        for i in 0..count {
            let value = Felt::from_bytes_be_slice(stack.borrow_front());
            stack.pop_front();
            let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
            verify_variables.decommitment_values[i] = value;
        }

        // Read montgomery_values
        for i in 0..count {
            let value = Felt::from_bytes_be_slice(stack.borrow_front());
            stack.pop_front();
            let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
            verify_variables.montgomery_values[i] = value;
        }
    }

    fn push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(&mut self, stack: &mut T) {
        // Get count first
        let count = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        let count_usize: usize = count.to_biguint().try_into().unwrap();

        // Push montgomery_values in reverse order - no allocation
        for i in (0..count_usize).rev() {
            let value_bytes = {
                let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                verify_variables.montgomery_values[i].to_bytes_be()
            };
            stack.push_front(&value_bytes).unwrap();
        }
        stack
            .push_front(&Felt::from(count_usize).to_bytes_be())
            .unwrap();

        // Push decommitment_values in reverse order - no allocation
        for i in (0..count_usize).rev() {
            let value_bytes = {
                let verify_variables: &mut VerifyVariables = stack.get_verify_variables_mut();
                verify_variables.decommitment_values[i].to_bytes_be()
            };
            stack.push_front(&value_bytes).unwrap();
        }
        stack
            .push_front(&Felt::from(count_usize).to_bytes_be())
            .unwrap();
    }

    fn from_stack_ref<T: BidirectionalStack + StarkVerifyTrait>(_stack: &T) -> &Self {
        // For Decommitment, data is stored in VerifyVariables, use from_stack instead
        unimplemented!("Decommitment data is stored in VerifyVariables, use from_stack instead")
    }

    fn to_bytes_be(&self) -> Decommitment {
        *self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Witness {
    pub vector: vector::types::Witness,
}

impl CommitmentTrait<Witness, ()> for Witness {
    fn from_stack<T: BidirectionalStack + StarkVerifyTrait>(stack: &mut T) {
        vector::types::Witness::from_stack(stack);
    }

    fn from_stack_ref<T: BidirectionalStack + StarkVerifyTrait>(_stack: &T) -> &Self {
        // For Witness, data is stored in VerifyVariables, use from_stack instead
        unimplemented!("Witness data is stored in VerifyVariables, use from_stack instead")
    }

    fn push_to_stack<T: BidirectionalStack + StarkVerifyTrait>(&mut self, stack: &mut T) {
        self.vector.push_to_stack(stack);
    }

    fn to_bytes_be(&self) -> Witness {
        *self
    }
}
