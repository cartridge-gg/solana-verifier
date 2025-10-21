use crate::constants::{POINTS_P1, POINTS_P2, POINTS_P3, POINTS_P4, SHIFT_POINT};
use felt::Felt;
use lambdaworks_math::elliptic_curve::short_weierstrass::{
    curves::stark_curve::StarkCurve, point::ShortWeierstrassProjectivePoint,
};
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

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

// Helper function to perform point addition with fresh stack frame
#[inline(never)]
fn operate_with_affine_inline_never(
    px: &Felt,
    py: &Felt,
    pz: &Felt,
    qx: &Felt,
    qy: &Felt,
) -> (Felt, Felt, Felt) {
    // Compute u = qy * pz and v = qx * pz
    let u = qy * pz;
    let v = qx * pz;

    // Check edge cases
    if u == *py {
        if v != *px || *py == Felt::ZERO {
            // Return point at infinity (0, 1, 0)
            return (Felt::ZERO, Felt::ONE, Felt::ZERO);
        } else {
            // Point doubling case - use lambdaworks double() directly
            let p = ShortWeierstrassProjectivePoint::<StarkCurve>::new([
                *px.inner(),
                *py.inner(),
                *pz.inner(),
            ])
            .unwrap();
            let result = p.double();
            return (
                Felt::from_bytes_be_slice(&result.x().to_bytes_be()),
                Felt::from_bytes_be_slice(&result.y().to_bytes_be()),
                Felt::from_bytes_be_slice(&result.z().to_bytes_be()),
            );
        }
    }

    // Compute differences
    let u = u - py;
    let v = v - px;

    // Compute intermediate values
    let vv = v * v;
    let uu = u * u;
    let vvv = v * vv;
    let r = vv * px;
    let a = (uu * pz - vvv) - (r + r);

    // Compute final coordinates
    let x = v * a;
    let y = u * (r - a) - vvv * py;
    let z = vvv * pz;

    (x, y, z)
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
    bits: [bool; 248],
    bits_len: usize,
    table_index: u8,
    chunk_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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
            phase: LookupAndAccumulatePhase::Accumulate,
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
                // Stack already has accumulator point (x, y, z) from previous task
                // Just transition to Accumulate phase
                self.phase = LookupAndAccumulatePhase::Accumulate;
                self.chunk_index = 0;
                vec![]
            }
            LookupAndAccumulatePhase::Accumulate => {
                const CHUNK_SIZE: usize = 10;

                let bits = &self.bits[..self.bits_len];
                let total_chunks = bits.len().div_ceil(PedersenHash::CURVE_CONST_BITS);

                if self.chunk_index >= total_chunks {
                    // Done - accumulator is on stack, mark as finished
                    self.phase = LookupAndAccumulatePhase::Finished;
                    vec![]
                } else {
                    // Read current accumulator from stack
                    let z = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                    stack.pop_front();
                    let y = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                    stack.pop_front();
                    let x = Felt::from_bytes_be(stack.borrow_front().try_into().unwrap());
                    stack.pop_front();

                    let mut acc_x = x;
                    let mut acc_y = y;
                    let mut acc_z = z;

                    // Process multiple chunks (up to CHUNK_SIZE)
                    let start_chunk = self.chunk_index;
                    let end_chunk = (start_chunk + CHUNK_SIZE).min(total_chunks);

                    for i in start_chunk..end_chunk {
                        let chunk_start = i * PedersenHash::CURVE_CONST_BITS;
                        let chunk_end =
                            (chunk_start + PedersenHash::CURVE_CONST_BITS).min(bits.len());
                        let chunk = &bits[chunk_start..chunk_end];
                        let offset = bools_to_usize_le(chunk);

                        if offset > 0 {
                            let point_index = i * PedersenHash::TABLE_SIZE + offset - 1;

                            // Get the point from the table
                            let point = match self.table_index {
                                1 => &POINTS_P1[point_index],
                                2 => &POINTS_P2[point_index],
                                3 => &POINTS_P3[point_index],
                                4 => &POINTS_P4[point_index],
                                _ => panic!("Invalid table index"),
                            };

                            // Perform point addition with fresh stack frame
                            let (new_x, new_y, new_z) = operate_with_affine_inline_never(
                                &acc_x,
                                &acc_y,
                                &acc_z,
                                &Felt::from_bytes_be_slice(&point.x().to_bytes_be()),
                                &Felt::from_bytes_be_slice(&point.y().to_bytes_be()),
                            );

                            acc_x = new_x;
                            acc_y = new_y;
                            acc_z = new_z;
                        }
                    }

                    // Push updated accumulator back to stack
                    stack.push_front(&acc_x.to_bytes_be()).unwrap();
                    stack.push_front(&acc_y.to_bytes_be()).unwrap();
                    stack.push_front(&acc_z.to_bytes_be()).unwrap();

                    self.chunk_index = end_chunk;
                    vec![]
                }
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
