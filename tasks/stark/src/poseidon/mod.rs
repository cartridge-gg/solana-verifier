pub mod constants;
pub mod hades;

use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::{felt::Felt, poseidon::hades::HadesPermutation};

#[repr(C)]
pub struct PoseidonHashMany {
    state: [Felt; 3],
    input_length: usize,
    counter: usize,
}

impl_type_identifiable!(PoseidonHashMany);

impl PoseidonHashMany {
    pub fn new(inputs_len: usize) -> Self {
        Self {
            state: [Felt::ZERO; 3],
            input_length: (inputs_len + 1).div_ceil(2) * 2,
            counter: 0,
        }
    }

    pub fn push_input<T: BidirectionalStack>(inputs: &[Felt], stack: &mut T) {
        let inputs_len = inputs.len() + 1;
        let zero_count = inputs_len.div_ceil(2) * 2 - inputs_len;
        for _ in 0..zero_count {
            stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
        }
        stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();

        inputs.iter().rev().for_each(|value| {
            stack.push_front(&value.to_bytes_be()).unwrap();
        });
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
        stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
    }
}

impl Executable for PoseidonHashMany {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let s1 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let s2 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let s3 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let v1 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let v2 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        self.state[0] = s1 + v1;
        self.state[1] = s2 + v2;
        self.state[2] = s3;

        self.counter += 2;

        vec![HadesPermutation::new(self.state).to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.counter >= self.input_length
    }
}

#[repr(C)]
pub struct PoseidonHash {
    state: [Felt; 3],
    phase: PoseidonPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoseidonPhase {
    Init,
    Done,
}

impl_type_identifiable!(PoseidonHash);

impl PoseidonHash {
    pub fn new(x: Felt, y: Felt) -> Self {
        Self {
            state: [x, y, Felt::TWO],
            phase: PoseidonPhase::Init,
        }
    }
}

impl Executable for PoseidonHash {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let s1 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let s2 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let s3 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let v1 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        let v2 = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();

        self.state[0] = s1 + v1;
        self.state[1] = s2 + v2;
        self.state[2] = s3;

        self.phase = PoseidonPhase::Done;

        vec![HadesPermutation::new(self.state).to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.phase == PoseidonPhase::Done
    }
}
