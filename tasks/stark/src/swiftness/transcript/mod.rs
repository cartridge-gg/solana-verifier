pub mod transcript;

use crate::felt::Felt;
use crate::poseidon::{PoseidonHash, PoseidonHashMany};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

pub use transcript::Transcript;

#[repr(C)]
pub struct TranscriptRandomFelt {
    digest: Felt,
    counter: Felt,
    phase: TranscriptRandomFeltPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptRandomFeltPhase {
    ComputeHash,
    Finished,
}

impl_type_identifiable!(TranscriptRandomFelt);

impl TranscriptRandomFelt {
    pub fn new(digest: Felt, counter: Felt) -> Self {
        Self {
            digest,
            counter,
            phase: TranscriptRandomFeltPhase::ComputeHash,
        }
    }

    pub fn push_input<T: BidirectionalStack>(digest: Felt, counter: Felt, stack: &mut T) {
        // PoseidonHash expects 5 values on stack: s1, s2, s3, v1, v2
        // We need to provide: digest as v1, counter as v2, and zeros for s1, s2, s3
        stack.push_front(&counter.to_bytes_be()).unwrap();
        stack.push_front(&digest.to_bytes_be()).unwrap();
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
    }
}

impl Executable for TranscriptRandomFelt {
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            TranscriptRandomFeltPhase::ComputeHash => {
                self.phase = TranscriptRandomFeltPhase::Finished;
                vec![PoseidonHash::new(self.digest, self.counter).to_vec_with_type_tag()]
            }
            TranscriptRandomFeltPhase::Finished => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == TranscriptRandomFeltPhase::Finished
    }
}

#[repr(C)]
pub struct TranscriptReadFelt {
    digest: Felt,
    val: Felt,
    phase: TranscriptReadFeltPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptReadFeltPhase {
    ComputeHash,
    Finished,
}

impl_type_identifiable!(TranscriptReadFelt);

impl TranscriptReadFelt {
    pub fn new(digest: Felt, val: Felt) -> Self {
        Self {
            digest,
            val,
            phase: TranscriptReadFeltPhase::ComputeHash,
        }
    }

    pub fn push_input<T: BidirectionalStack>(digest: Felt, val: Felt, stack: &mut T) {
        let inputs = vec![digest + Felt::ONE, val];
        PoseidonHashMany::push_input(&inputs, stack);
    }
}

impl Executable for TranscriptReadFelt {
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            TranscriptReadFeltPhase::ComputeHash => {
                self.phase = TranscriptReadFeltPhase::Finished;
                let inputs = vec![self.digest + Felt::ONE, self.val];
                vec![PoseidonHashMany::new(inputs.len()).to_vec_with_type_tag()]
            }
            TranscriptReadFeltPhase::Finished => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == TranscriptReadFeltPhase::Finished
    }
}

#[repr(C)]
pub struct TranscriptReadFeltVector {
    digest: Felt,
    values: Vec<Felt>,
    phase: TranscriptReadFeltVectorPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptReadFeltVectorPhase {
    ComputeHash,
    Finished,
}

impl_type_identifiable!(TranscriptReadFeltVector);

impl TranscriptReadFeltVector {
    pub fn new(digest: Felt, values: Vec<Felt>) -> Self {
        Self {
            digest,
            values,
            phase: TranscriptReadFeltVectorPhase::ComputeHash,
        }
    }

    pub fn push_input<T: BidirectionalStack>(digest: Felt, values: &[Felt], stack: &mut T) {
        let mut inputs = vec![digest + Felt::ONE];
        inputs.extend_from_slice(values);
        PoseidonHashMany::push_input(&inputs, stack);
    }
}

impl Executable for TranscriptReadFeltVector {
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            TranscriptReadFeltVectorPhase::ComputeHash => {
                self.phase = TranscriptReadFeltVectorPhase::Finished;
                let mut inputs = vec![self.digest + Felt::ONE];
                inputs.extend_from_slice(&self.values);
                vec![PoseidonHashMany::new(inputs.len()).to_vec_with_type_tag()]
            }
            TranscriptReadFeltVectorPhase::Finished => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == TranscriptReadFeltVectorPhase::Finished
    }
}
