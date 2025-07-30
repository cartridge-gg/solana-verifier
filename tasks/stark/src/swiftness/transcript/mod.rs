#[allow(clippy::module_inception)]
pub mod transcript;

use crate::felt::Felt;
use crate::poseidon::{PoseidonHash, PoseidonHashMany};
pub use transcript::Transcript;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

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

    // pub fn push_input<T: BidirectionalStack>(digest: Felt, counter: Felt, stack: &mut T) {
    //     // PoseidonHash expects 5 values on stack: s1, s2, s3, v1, v2
    //     // We need to provide: digest as v1, counter as v2, and zeros for s1, s2, s3
    //     stack.push_front(&counter.to_bytes_be()).unwrap();
    //     stack.push_front(&digest.to_bytes_be()).unwrap();
    //     stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
    //     stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
    //     stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
    // }
}

impl Executable for TranscriptRandomFelt {
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            TranscriptRandomFeltPhase::ComputeHash => {
                self.phase = TranscriptRandomFeltPhase::Finished;
                PoseidonHash::push_input(self.digest, self.counter, _stack);
                vec![PoseidonHash::new().to_vec_with_type_tag()]
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
    phase: TranscriptReadFeltPhase,
    inputs_len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptReadFeltPhase {
    ComputeHash,
    ReadPosiedonResult,
    Finished,
}

impl_type_identifiable!(TranscriptReadFelt);

impl TranscriptReadFelt {
    pub fn new() -> Self {
        Self {
            phase: TranscriptReadFeltPhase::ComputeHash,
            inputs_len: 2,
        }
    }

    pub fn push_input<T: BidirectionalStack>(digest: Felt, val: Felt, stack: &mut T) {
        let inputs = vec![digest + Felt::ONE, val];
        PoseidonHashMany::push_input(&inputs, stack);
    }
}

impl Executable for TranscriptReadFelt {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            TranscriptReadFeltPhase::ComputeHash => {
                self.phase = TranscriptReadFeltPhase::ReadPosiedonResult;
                vec![PoseidonHashMany::new(self.inputs_len).to_vec_with_type_tag()]
            }
            TranscriptReadFeltPhase::ReadPosiedonResult => {
                let result = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                stack.push_front(&result.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                self.phase = TranscriptReadFeltPhase::Finished;
                vec![]
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
    phase: TranscriptReadFeltVectorPhase,
    inputs_len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptReadFeltVectorPhase {
    ComputeHash,
    ReadPosiedonResult,
    Finished,
}

impl_type_identifiable!(TranscriptReadFeltVector);

impl TranscriptReadFeltVector {
    pub fn new(inputs_len: usize) -> Self {
        Self {
            phase: TranscriptReadFeltVectorPhase::ComputeHash,
            inputs_len: inputs_len + 1,
        }
    }

    pub fn push_input<T: BidirectionalStack>(digest: Felt, values: &[Felt], stack: &mut T) {
        let mut inputs = vec![digest + Felt::ONE];
        inputs.extend_from_slice(values);
        PoseidonHashMany::push_input(&inputs, stack);
    }
}

impl Executable for TranscriptReadFeltVector {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            TranscriptReadFeltVectorPhase::ComputeHash => {
                self.phase = TranscriptReadFeltVectorPhase::ReadPosiedonResult;
                vec![PoseidonHashMany::new(self.inputs_len).to_vec_with_type_tag()]
            }
            TranscriptReadFeltVectorPhase::ReadPosiedonResult => {
                let result = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                stack.push_front(&result.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                self.phase = TranscriptReadFeltVectorPhase::Finished;
                vec![]
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
