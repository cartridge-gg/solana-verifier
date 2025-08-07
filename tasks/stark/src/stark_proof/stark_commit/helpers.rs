use crate::swiftness::air::diluted::get_diluted_product;
use crate::swiftness::air::periodic_columns::{
    eval_pedersen_x, eval_pedersen_y, eval_poseidon_poseidon_full_round_key0,
    eval_poseidon_poseidon_full_round_key1, eval_poseidon_poseidon_full_round_key2,
    eval_poseidon_poseidon_partial_round_key0, eval_poseidon_poseidon_partial_round_key1,
};
use crate::swiftness::stark::types::StarkProof;
use felt::Felt;
use felt::NonZeroFelt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

pub const DILUTED_N_BITS: u32 = 16;
pub const DILUTED_SPACING: u32 = 4;
pub const FELT_2: Felt = Felt::from_hex_unchecked("0x2");
pub const PEDERSEN_BUILTIN_RATIO: u32 = 8;
pub const PEDERSEN_BUILTIN_REPETITIONS: u32 = 4;
pub const POSEIDON_RATIO: u32 = 8;

// PowersArray task - generates array of powers [1, alpha, alpha^2, ..., alpha^(n-1)]
#[repr(C)]
pub struct PowersArray {
    count: u32,
    current: u32,
}

impl_type_identifiable!(PowersArray);

impl PowersArray {
    pub fn new(count: u32) -> Self {
        Self { count, current: 0 }
    }
}

impl Executable for PowersArray {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.current == 0 {
            // First iteration - get initial value and alpha
            let initial = Felt::from_bytes_be_slice(stack.borrow_front());
            stack.pop_front();
            let alpha = Felt::from_bytes_be_slice(stack.borrow_front());

            stack.pop_front();
            stack.push_front(&initial.to_bytes_be()).unwrap();

            let next_value = initial * alpha;

            // Keep alpha on stack for next iteration
            stack.push_front(&alpha.to_bytes_be()).unwrap();
            stack.push_front(&next_value.to_bytes_be()).unwrap();

            self.current = 1;
        } else if self.current < self.count {
            // Get current value and alpha
            let current_value = Felt::from_bytes_be_slice(stack.borrow_front());
            stack.pop_front();
            let alpha = Felt::from_bytes_be_slice(stack.borrow_front());
            stack.pop_front();
            // Push current value to results
            stack.push_front(&current_value.to_bytes_be()).unwrap();

            // Calculate next value for next iteration
            let next_value = current_value * alpha;
            if self.current + 1 < self.count {
                stack.push_front(&alpha.to_bytes_be()).unwrap();
                stack.push_front(&next_value.to_bytes_be()).unwrap();
            }

            self.current += 1;
        }

        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.current >= self.count
    }
}
