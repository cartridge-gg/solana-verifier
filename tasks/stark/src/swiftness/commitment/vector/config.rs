use felt::Felt;
use utils::BidirectionalStack;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Config {
    pub height: Felt,
    pub n_verifier_friendly_commitment_layers: Felt,
}

pub trait ConfigTrait<C>: Sized {
    fn from_stack<T: BidirectionalStack>(stack: &mut T) -> Self;
    fn push_to_stack<T: BidirectionalStack>(&self, stack: &mut T);
    fn to_bytes_be(&self) -> C;
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct VectorConfigBytes {
    pub height: [u8; 32],
    pub n_verifier_friendly_commitment_layers: [u8; 32],
}

impl Config {
    fn new(height: Felt, n_verifier_friendly_commitment_layers: Felt) -> Self {
        Self {
            height,
            n_verifier_friendly_commitment_layers,
        }
    }
}

impl ConfigTrait<VectorConfigBytes> for Config {
    fn from_stack<T: BidirectionalStack>(stack: &mut T) -> Self {
        let height = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        let n_verifier_friendly_commitment_layers = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        Self::new(height, n_verifier_friendly_commitment_layers)
    }

    fn push_to_stack<T: BidirectionalStack>(&self, stack: &mut T) {
        stack
            .push_front(&self.n_verifier_friendly_commitment_layers.to_bytes_be())
            .unwrap();
        stack.push_front(&self.height.to_bytes_be()).unwrap();
    }

    fn to_bytes_be(&self) -> VectorConfigBytes {
        VectorConfigBytes {
            height: self.height.to_bytes_be(),
            n_verifier_friendly_commitment_layers: self
                .n_verifier_friendly_commitment_layers
                .to_bytes_be(),
        }
    }
}
