use crate::felt::Felt;
use crate::pedersen::constants::{points_p1, points_p2, points_p3, points_p4, shift_point};
use lambdaworks_math::elliptic_curve::short_weierstrass::{
    curves::stark_curve::StarkCurve, point::ShortWeierstrassProjectivePoint,
};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

pub mod constants;

#[repr(C)]
pub struct PedersenHash {
    phase: PerdersenPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerdersenPhase {
    Hash,
    Finished,
}

impl_type_identifiable!(PedersenHash);

impl PedersenHash {
    pub fn new() -> Self {
        Self {
            phase: PerdersenPhase::Hash,
        }
    }

    /// Helper function to push inputs to stack and create task
    pub fn push_input<T: BidirectionalStack>(x: Felt, y: Felt, stack: &mut T) -> () {
        // Push the inputs to the stack
        stack.push_front(&x.to_bytes_be()).unwrap();
        stack.push_front(&y.to_bytes_be()).unwrap();
    }

    #[inline(always)]
    fn lookup_and_accumulate(
        acc: &mut ShortWeierstrassProjectivePoint<StarkCurve>,
        bits: &[bool],
        prep: &[ShortWeierstrassProjectivePoint<StarkCurve>],
    ) {
        bits.chunks(PedersenHash::CURVE_CONST_BITS)
            .enumerate()
            .for_each(|(i, v)| {
                let offset = bools_to_usize_le(v);
                if offset > 0 {
                    // Table lookup at 'offset-1' in table for chunk 'i'
                    *acc =
                        acc.operate_with_affine(&prep[i * PedersenHash::TABLE_SIZE + offset - 1]);
                }
            })
    }
}

#[inline(always)]
fn bools_to_usize_le(bools: &[bool]) -> usize {
    let mut result: usize = 0;
    for (ind, bit) in bools.iter().enumerate() {
        if *bit {
            result += 1 << ind;
        }
    }
    result
}

impl Executable for PedersenHash {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Get inputs from stack
        let y = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();
        let x = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
        stack.pop_front();
        println!("x: {:?}", x);
        println!("y: {:?}", y);

        let x = x.to_bits_le();
        let y = y.to_bits_le();
        let mut acc = shift_point();

        Self::lookup_and_accumulate(&mut acc, &x[..248], &points_p1()); // Add a_low * P1
        Self::lookup_and_accumulate(&mut acc, &x[248..252], &points_p2()); // Add a_high * P2
        Self::lookup_and_accumulate(&mut acc, &y[..248], &points_p3()); // Add b_low * P3
        Self::lookup_and_accumulate(&mut acc, &y[248..252], &points_p4()); // Add b_high * P4

        let result = *acc.to_affine().x();

        // Push result back to stack
        stack.push_front(&result.to_bytes_be()).unwrap();

        self.phase = PerdersenPhase::Finished;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.phase == PerdersenPhase::Finished
    }
}
