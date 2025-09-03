use crate::swiftness::commitment::vector::{
    self,
    config::{ConfigTrait, VectorConfigBytes},
};
use felt::Felt;
use utils::BidirectionalStack;
use vector::config::Config as VectorConfig;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Config {
    pub n_columns: Felt,
    pub vector: VectorConfig,
}

impl Config {
    fn new(n_columns: Felt, vector: vector::config::Config) -> Self {
        Self { n_columns, vector }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct TableConfigBytes {
    pub n_columns: [u8; 32],
    pub vector: VectorConfigBytes,
}

impl ConfigTrait<TableConfigBytes> for Config {
    fn from_stack<T: BidirectionalStack>(stack: &mut T) -> Self {
        let n_columns = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        let vector = vector::config::Config::from_stack(stack);
        Self::new(n_columns, vector)
    }

    fn push_to_stack<T: BidirectionalStack>(&self, stack: &mut T) {
        stack
            .push_front(
                &self
                    .vector
                    .to_bytes_be()
                    .n_verifier_friendly_commitment_layers,
            )
            .unwrap();
        stack.push_front(&self.vector.to_bytes_be().height).unwrap();
        stack.push_front(&self.n_columns.to_bytes_be()).unwrap();
    }

    fn to_bytes_be(&self) -> TableConfigBytes {
        TableConfigBytes {
            n_columns: self.n_columns.to_bytes_be(),
            vector: self.vector.to_bytes_be(),
        }
    }
}
