use crate::pedersen::constants::{POINTS_P1, POINTS_P2, POINTS_P3, POINTS_P4, SHIFT_POINT};
use felt::Felt;
use lambdaworks_math::elliptic_curve::short_weierstrass::{
    curves::stark_curve::StarkCurve, point::ShortWeierstrassProjectivePoint,
};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

pub mod constants;

#[repr(C)]
pub struct PedersenHash {
    phase: PerdersenPhase,
    acc: ShortWeierstrassProjectivePoint<StarkCurve>,
    x: [bool; 256],
    y: [bool; 256],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerdersenPhase {
    LookupP1,
    LookupP2,
    LookupP3,
    LookupP4,
    Results,
    Finished,
}

impl_type_identifiable!(PedersenHash);

impl Default for PedersenHash {
    fn default() -> Self {
        Self::new()
    }
}

impl PedersenHash {
    pub fn new() -> Self {
        Self {
            phase: PerdersenPhase::LookupP1,
            acc: SHIFT_POINT,
            x: [false; 256],
            y: [false; 256],
        }
    }

    pub fn push_input<T: BidirectionalStack>(x: Felt, y: Felt, stack: &mut T) {
        stack.push_front(&x.to_bytes_be()).unwrap();
        stack.push_front(&y.to_bytes_be()).unwrap();
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
        match self.phase {
            PerdersenPhase::LookupP1 => {
                let y = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();
                let x = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();
                let x = x.to_bits_le();
                let y = y.to_bits_le();
                self.x = x;
                self.y = y;

                stack.push_front(&self.acc.x().to_bytes_be()).unwrap();
                stack.push_front(&self.acc.y().to_bytes_be()).unwrap();
                stack.push_front(&self.acc.z().to_bytes_be()).unwrap();

                self.phase = PerdersenPhase::LookupP2;
                vec![LookupAndAccumulate::new(&self.x[..248], 1).to_vec_with_type_tag()]
            }
            PerdersenPhase::LookupP2 => {
                self.phase = PerdersenPhase::LookupP3;
                vec![LookupAndAccumulate::new(&self.x[248..252], 2).to_vec_with_type_tag()]
            }
            PerdersenPhase::LookupP3 => {
                self.phase = PerdersenPhase::LookupP4;
                vec![LookupAndAccumulate::new(&self.y[..248], 3).to_vec_with_type_tag()]
            }
            PerdersenPhase::LookupP4 => {
                self.phase = PerdersenPhase::Results;
                vec![LookupAndAccumulate::new(&self.y[248..252], 4).to_vec_with_type_tag()]
            }
            PerdersenPhase::Results => {
                let z = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();
                let y = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();
                let x = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();

                self.acc = ShortWeierstrassProjectivePoint::<StarkCurve>::new([
                    *x.inner(),
                    *y.inner(),
                    *z.inner(),
                ])
                .unwrap();

                let result = *self.acc.to_affine().x();
                stack.push_front(&result.to_bytes_be()).unwrap();

                self.phase = PerdersenPhase::Finished;
                vec![]
            }
            PerdersenPhase::Finished => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == PerdersenPhase::Finished
    }
}

#[repr(C)]
pub struct LookupAndAccumulate {
    phase: LookupAndAccumulatePhase,
    acc: ShortWeierstrassProjectivePoint<StarkCurve>,
    bits: [bool; 248],
    bits_len: usize,
    table_index: u8,
    chunk_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupAndAccumulatePhase {
    Lookup,
    Accumulate,
    Finished,
}

impl_type_identifiable!(LookupAndAccumulate);

impl LookupAndAccumulate {
    pub fn new(bits: &[bool], table_index: u8) -> Self {
        let mut bits_array = [false; 248];
        let len = bits.len();
        bits_array[..len].copy_from_slice(&bits[..len]);

        Self {
            phase: LookupAndAccumulatePhase::Lookup,
            acc: SHIFT_POINT,
            bits: bits_array,
            bits_len: len,
            table_index,
            chunk_index: 0,
        }
    }
}

impl Executable for LookupAndAccumulate {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            LookupAndAccumulatePhase::Lookup => {
                let z = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();
                let y = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();
                let x = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                stack.pop_front();

                self.acc = ShortWeierstrassProjectivePoint::<StarkCurve>::new([
                    *x.inner(),
                    *y.inner(),
                    *z.inner(),
                ])
                .unwrap();
                self.phase = LookupAndAccumulatePhase::Accumulate;
                vec![]
            }
            LookupAndAccumulatePhase::Accumulate => {
                const CHUNK_SIZE: usize = 10;

                let prep: &[ShortWeierstrassProjectivePoint<StarkCurve>] = match self.table_index {
                    1 => &POINTS_P1,
                    2 => &POINTS_P2,
                    3 => &POINTS_P3,
                    4 => &POINTS_P4,
                    _ => panic!("Invalid table index"),
                };

                let bits = &self.bits[..self.bits_len];
                let start_chunk = self.chunk_index;

                #[allow(clippy::double_ended_iterator_last)]
                let processed = bits
                    .chunks(PedersenHash::CURVE_CONST_BITS)
                    .enumerate()
                    .skip(start_chunk)
                    .take(CHUNK_SIZE)
                    .map(|(i, chunk)| {
                        let offset = bools_to_usize_le(chunk);
                        if offset > 0 {
                            self.acc = self.acc.operate_with_affine(
                                &prep[i * PedersenHash::TABLE_SIZE + offset - 1],
                            );
                        }
                        i + 1
                    })
                    .last()
                    .unwrap_or(start_chunk);

                self.chunk_index = processed;

                let total_chunks = bits.len().div_ceil(PedersenHash::CURVE_CONST_BITS);
                if self.chunk_index >= total_chunks {
                    stack.push_front(&self.acc.x().to_bytes_be()).unwrap();
                    stack.push_front(&self.acc.y().to_bytes_be()).unwrap();
                    stack.push_front(&self.acc.z().to_bytes_be()).unwrap();
                    self.phase = LookupAndAccumulatePhase::Finished;
                }

                vec![]
            }
            LookupAndAccumulatePhase::Finished => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == LookupAndAccumulatePhase::Finished
    }
}
