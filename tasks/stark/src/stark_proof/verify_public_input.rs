use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::stark_proof::hash_public_inputs::HashPublicInputs;
use crate::stark_proof::segments;
use crate::stark_proof::INITIAL_PC;
use crate::stark_proof::MAX_ADDRESS;
use crate::{
    felt::Felt,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyPublicInputStep {
    Init,
    Output,
    Program,
    Done,
}
#[repr(C)]
pub struct VerifyPublicInput {
    step: VerifyPublicInputStep,
    program_start: usize,
    program_end: usize,
    program_len: usize,
    output_start: usize,
    output_end: usize,
    output_len: usize,
}

impl_type_identifiable!(VerifyPublicInput);

impl VerifyPublicInput {
    pub fn new() -> Self {
        Self {
            step: VerifyPublicInputStep::Init,
            program_start: 0,
            program_end: 0,
            output_start: 0,
            output_end: 0,
            program_len: 0,
            output_len: 0,
        }
    }
}

impl Default for VerifyPublicInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for VerifyPublicInput {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyPublicInputStep::Init => {
                let proof_reference: &mut [u8] = stack.get_proof_reference();
                let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                let public_segments = &proof.public_input.segments;

                let initial_pc: usize = public_segments
                    .get(segments::PROGRAM)
                    .unwrap()
                    .begin_addr
                    .try_into()
                    .unwrap();
                let initial_fp: usize = public_segments
                    .get(segments::EXECUTION)
                    .unwrap()
                    .begin_addr
                    .try_into()
                    .unwrap();
                let final_ap: usize = public_segments
                    .get(segments::EXECUTION)
                    .unwrap()
                    .stop_ptr
                    .try_into()
                    .unwrap();

                assert!(
                    initial_fp < MAX_ADDRESS,
                    "Initial AP exceeds maximum address"
                );
                assert!(final_ap < MAX_ADDRESS, "Final AP exceeds maximum address");
                assert!(
                    proof.public_input.continuous_page_headers.is_empty(),
                    "Continuous page headers are not empty"
                );
                assert!(initial_pc == INITIAL_PC, "Wrong initial PC");

                //1. Program segment
                let program_end_pc: usize = initial_fp - 2;
                let program_len = program_end_pc - initial_pc;

                let output_start: usize = public_segments
                    .get(segments::OUTPUT)
                    .unwrap()
                    .begin_addr
                    .try_into()
                    .unwrap();
                let output_end: usize = public_segments
                    .get(segments::OUTPUT)
                    .unwrap()
                    .stop_ptr
                    .try_into()
                    .unwrap();
                let output_len = output_end - output_start;
                let output_start = proof.public_input.main_page.0.len() - output_len;

                self.output_start = output_start;
                self.output_end = proof.public_input.main_page.0.len();
                self.output_len = output_len;

                self.program_end = program_len;
                self.program_len = program_len;

                self.step = VerifyPublicInputStep::Output;
                vec![]
            }
            VerifyPublicInputStep::Output => {
                let inputs_len = self.output_len + 1;
                let zero_count = inputs_len.div_ceil(2) * 2 - inputs_len;
                for _ in 0..zero_count {
                    stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                }
                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();

                for i in (self.output_start..self.output_end).rev() {
                    let proof_reference: &mut [u8] = stack.get_proof_reference();
                    let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                    let memory = proof.public_input.main_page.0.as_slice();
                    let item = memory[i].value;
                    stack.push_front(&item.to_bytes_be()).unwrap();
                }

                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                self.step = VerifyPublicInputStep::Program;
                vec![]
            }
            VerifyPublicInputStep::Program => {
                let inputs_len = self.program_len + 1;
                let zero_count = inputs_len.div_ceil(2) * 2 - inputs_len;
                for _ in 0..zero_count {
                    stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                }

                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();
                for i in (self.program_start..self.program_end).rev() {
                    let proof_reference: &mut [u8] = stack.get_proof_reference();
                    let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                    let memory = proof.public_input.main_page.0.as_slice();
                    let item = memory[i].value;
                    stack.push_front(&item.to_bytes_be()).unwrap();
                }
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                self.step = VerifyPublicInputStep::Done;

                vec![HashPublicInputs::new(self.program_len, self.output_len).to_vec_with_type_tag()]
            }
            VerifyPublicInputStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VerifyPublicInputStep::Done
    }
}
