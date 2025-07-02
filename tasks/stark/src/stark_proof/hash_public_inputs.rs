use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::{felt::Felt, poseidon::PoseidonHashMany};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashPublicInputsStep {
    Init,
    ProgramHash,
    OutputHash,
    Done,
}
#[repr(C)]
pub struct HashPublicInputs {
    pub step: HashPublicInputsStep,
    pub program_input_length: usize,
    pub output_input_length: usize,
    pub program_hash: Felt,
}

impl_type_identifiable!(HashPublicInputs);

impl HashPublicInputs {
    pub fn new(program_input_length: usize, output_input_length: usize) -> Self {
        Self {
            step: HashPublicInputsStep::Init,
            program_input_length,
            output_input_length,
            program_hash: Felt::ZERO,
        }
    }
}

impl Executable for HashPublicInputs {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            HashPublicInputsStep::Init => {
                self.step = HashPublicInputsStep::ProgramHash;
                vec![PoseidonHashMany::new(self.program_input_length).to_vec_with_type_tag()]
            }
            HashPublicInputsStep::ProgramHash => {
                let bytes = stack.borrow_front();
                let program_hash = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();
                self.program_hash = program_hash;
                self.step = HashPublicInputsStep::OutputHash;
                vec![PoseidonHashMany::new(self.output_input_length).to_vec_with_type_tag()]
            }
            HashPublicInputsStep::OutputHash => {
                let bytes = stack.borrow_front();
                let output_hash = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                stack.push_front(&output_hash.to_bytes_be()).unwrap();
                stack.push_front(&self.program_hash.to_bytes_be()).unwrap();

                self.step = HashPublicInputsStep::Done;
                vec![]
            }
            HashPublicInputsStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == HashPublicInputsStep::Done
    }
}
