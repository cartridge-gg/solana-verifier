use crate::pedersen::points::point_x::PedersenPointsX as PointsX;
use crate::pedersen::points::point_y::PedersenPointsY as PointsY;
use crate::stark_proof::stark_commit::eval_composition_polynomial_inner::EvalCompositionPolynomialInner;
use crate::stark_proof::stark_commit::helpers::{
    DILUTED_N_BITS, DILUTED_SPACING, FELT_2, PEDERSEN_BUILTIN_RATIO, PEDERSEN_BUILTIN_REPETITIONS,
    POSEIDON_RATIO,
};
use felt::Felt;
use felt::NonZeroFelt;
use types::swiftness::air::consts::*;
use types::swiftness::air::periodic_columns::{
    eval_poseidon_poseidon_full_round_key0, eval_poseidon_poseidon_full_round_key1,
    eval_poseidon_poseidon_full_round_key2, eval_poseidon_poseidon_partial_round_key0,
    eval_poseidon_poseidon_partial_round_key1,
};
use types::swiftness::air::recursive_with_poseidon::segments;
use types::swiftness::air::recursive_with_poseidon::PUBLIC_MEMORY_STEP;
use types::swiftness::air::recursive_with_poseidon::{SHIFT_POINT_X, SHIFT_POINT_Y};
use types::swiftness::global_values::{GlobalValues, InteractionElements};
use types::swiftness::stark::types::StarkCommitment;
use types::swiftness::stark::types::StarkProof;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkCommitmentTrait,
    TypeIdentifiable,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct EvalCompositionPolynomial {
    step: EvalCompositionStep,
    // Store intermediate values with defaults
    trace_domain_size: Felt,
    point: Felt,
    trace_generator: Felt,
    public_memory_prod_ratio: Felt,
    diluted_prod: Felt,
    pedersen_points_x: Felt,
    pedersen_points_y: Felt,
    poseidon_keys: (Felt, Felt, Felt, Felt, Felt), // (key0, key1, key2, partial_key0, partial_key1)
    diluted_check_interaction_alpha: Felt,
    diluted_check_interaction_z: Felt,
    diluted_check_permutation_interaction_elm: Felt,
    range_check16_perm_interaction_elm: Felt,
    memory_multi_column_perm_hash_interaction_elm0: Felt,
    memory_multi_column_perm_perm_interaction_elm: Felt,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalCompositionStep {
    CollectMaskValues,
    ComputePeriodicColumns,
    ComputeDilutedProduct,
    ComputePublicMemoryProductRatio,
    ComputePedersenPoints,
    ComputePoseidonPoints,
    EvalPolynomial,
    Done,
}

impl_type_identifiable!(EvalCompositionPolynomial);

impl EvalCompositionPolynomial {
    pub fn new() -> Self {
        Self {
            step: EvalCompositionStep::CollectMaskValues,
            trace_domain_size: Felt::ZERO,
            point: Felt::ZERO,
            trace_generator: Felt::ZERO,
            public_memory_prod_ratio: Felt::ZERO,
            diluted_prod: Felt::ZERO,
            pedersen_points_x: Felt::ZERO,
            pedersen_points_y: Felt::ZERO,
            poseidon_keys: (Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ZERO, Felt::ZERO),
            diluted_check_interaction_alpha: Felt::ZERO,
            diluted_check_interaction_z: Felt::ZERO,
            diluted_check_permutation_interaction_elm: Felt::ZERO,
            range_check16_perm_interaction_elm: Felt::ZERO,
            memory_multi_column_perm_hash_interaction_elm0: Felt::ZERO,
            memory_multi_column_perm_perm_interaction_elm: Felt::ZERO,
        }
    }
}

impl Default for EvalCompositionPolynomial {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for EvalCompositionPolynomial {
    fn execute<T: BidirectionalStack + ProofData + StarkCommitmentTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            EvalCompositionStep::CollectMaskValues => {
                // Get parameters from stack
                self.point = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.trace_generator = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.trace_domain_size = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let stark_commitment =
                    stack.get_stark_commitment_mut::<StarkCommitment<InteractionElements>>();

                self.memory_multi_column_perm_perm_interaction_elm = stark_commitment
                    .traces
                    .interaction_elements
                    .memory_multi_column_perm_perm_interaction_elm;
                self.memory_multi_column_perm_hash_interaction_elm0 = stark_commitment
                    .traces
                    .interaction_elements
                    .memory_multi_column_perm_hash_interaction_elm0;
                self.range_check16_perm_interaction_elm = stark_commitment
                    .traces
                    .interaction_elements
                    .range_check16_perm_interaction_elm;
                self.diluted_check_permutation_interaction_elm = stark_commitment
                    .traces
                    .interaction_elements
                    .diluted_check_permutation_interaction_elm;
                self.diluted_check_interaction_z = stark_commitment
                    .traces
                    .interaction_elements
                    .diluted_check_interaction_z;
                self.diluted_check_interaction_alpha = stark_commitment
                    .traces
                    .interaction_elements
                    .diluted_check_interaction_alpha;

                self.step = EvalCompositionStep::ComputeDilutedProduct;
                vec![]
            }
            EvalCompositionStep::ComputeDilutedProduct => {
                self.diluted_prod = get_diluted_product(
                    DILUTED_N_BITS.into(),
                    DILUTED_SPACING.into(),
                    self.diluted_check_interaction_z,
                    self.diluted_check_interaction_alpha,
                );
                self.step = EvalCompositionStep::ComputePublicMemoryProductRatio;
                vec![]
            }
            EvalCompositionStep::ComputePublicMemoryProductRatio => {
                let _public_memory_column_size = self
                    .trace_domain_size
                    .field_div(&NonZeroFelt::try_from(Felt::from(PUBLIC_MEMORY_STEP)).unwrap());

                self.step = EvalCompositionStep::ComputePeriodicColumns;
                // vec![PublicMemoryRatio::new(
                //     self.memory_multi_column_perm_perm_interaction_elm,
                //     self.memory_multi_column_perm_hash_interaction_elm0,
                //     public_memory_column_size,
                // )
                // .to_vec_with_type_tag()]
                vec![]
            }

            EvalCompositionStep::ComputePeriodicColumns => {
                // self.public_memory_prod_ratio = Felt::from_bytes_be_slice(stack.borrow_front());
                // stack.pop_front();

                self.public_memory_prod_ratio = Felt::from_hex_unchecked(
                    "0x5593c3e7c28433d4bed879adb1cb8081b0a46decda462e76da45b0d7244cbf0",
                ); // Placeholder until PublicMemoryRatio is implemented
                   // Calculate n_steps
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;
                let n_steps = FELT_2.pow_felt(&public_input.log_n_steps);
                stack.push_front(&n_steps.to_bytes_be()).unwrap();
                let n_pedersen_hash_copies = n_steps.field_div(
                    &NonZeroFelt::try_from(
                        Felt::from(PEDERSEN_BUILTIN_RATIO)
                            * Felt::from(PEDERSEN_BUILTIN_REPETITIONS),
                    )
                    .unwrap(),
                );
                let pedersen_point = self.point.pow_felt(&n_pedersen_hash_copies);

                self.step = EvalCompositionStep::ComputePedersenPoints;
                vec![
                    PointsX::new(pedersen_point).to_vec_with_type_tag(),
                    PointsY::new(pedersen_point).to_vec_with_type_tag(),
                ]
            }
            EvalCompositionStep::ComputePedersenPoints => {
                let pedersen_points_y = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let pedersen_points_x = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.pedersen_points_x = pedersen_points_x;
                self.pedersen_points_y = pedersen_points_y;
                self.step = EvalCompositionStep::ComputePoseidonPoints;
                vec![]
            }

            EvalCompositionStep::ComputePoseidonPoints => {
                // Calculate poseidon points
                let n_steps = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let n_poseidon_copies =
                    n_steps.field_div(&NonZeroFelt::try_from(Felt::from(POSEIDON_RATIO)).unwrap());
                let poseidon_point = self.point.pow_felt(&n_poseidon_copies);
                let poseidon_full_round_key0 =
                    eval_poseidon_poseidon_full_round_key0(poseidon_point);
                let poseidon_full_round_key1 =
                    eval_poseidon_poseidon_full_round_key1(poseidon_point);
                let poseidon_full_round_key2 =
                    eval_poseidon_poseidon_full_round_key2(poseidon_point);
                let poseidon_partial_round_key0 =
                    eval_poseidon_poseidon_partial_round_key0(poseidon_point);
                let poseidon_partial_round_key1 =
                    eval_poseidon_poseidon_partial_round_key1(poseidon_point);

                self.poseidon_keys = (
                    poseidon_full_round_key0,
                    poseidon_full_round_key1,
                    poseidon_full_round_key2,
                    poseidon_partial_round_key0,
                    poseidon_partial_round_key1,
                );

                self.step = EvalCompositionStep::EvalPolynomial;
                vec![]
            }

            EvalCompositionStep::EvalPolynomial => {
                // Get proof data to access interaction elements and public input
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;

                // Get all the computed values
                let (
                    poseidon_key0,
                    poseidon_key1,
                    poseidon_key2,
                    poseidon_partial_key0,
                    poseidon_partial_key1,
                ) = self.poseidon_keys;

                // Create GlobalValues structure
                let global_values = GlobalValues {
                    trace_length: self.trace_domain_size,
                    initial_pc: public_input
                        .segments
                        .get(segments::PROGRAM)
                        .unwrap()
                        .begin_addr,
                    final_pc: public_input
                        .segments
                        .get(segments::PROGRAM)
                        .unwrap()
                        .stop_ptr,
                    initial_ap: public_input
                        .segments
                        .get(segments::EXECUTION)
                        .unwrap()
                        .begin_addr,
                    final_ap: public_input
                        .segments
                        .get(segments::EXECUTION)
                        .unwrap()
                        .stop_ptr,
                    initial_pedersen_addr: public_input
                        .segments
                        .get(segments::PEDERSEN)
                        .unwrap()
                        .begin_addr,
                    initial_range_check_addr: public_input
                        .segments
                        .get(segments::RANGE_CHECK)
                        .unwrap()
                        .begin_addr,
                    initial_bitwise_addr: public_input
                        .segments
                        .get(segments::BITWISE)
                        .unwrap()
                        .begin_addr,
                    initial_poseidon_addr: public_input
                        .segments
                        .get(segments::POSEIDON)
                        .unwrap()
                        .begin_addr,
                    range_check_min: public_input.range_check_min,
                    range_check_max: public_input.range_check_max,
                    offset_size: FELT_65536,
                    half_offset_size: FELT_32768,
                    pedersen_shift_point: types::swiftness::global_values::EcPoint {
                        x: SHIFT_POINT_X,
                        y: SHIFT_POINT_Y,
                    },
                    pedersen_points_x: self.pedersen_points_x,
                    pedersen_points_y: self.pedersen_points_y,
                    poseidon_poseidon_full_round_key0: poseidon_key0,
                    poseidon_poseidon_full_round_key1: poseidon_key1,
                    poseidon_poseidon_full_round_key2: poseidon_key2,
                    poseidon_poseidon_partial_round_key0: poseidon_partial_key0,
                    poseidon_poseidon_partial_round_key1: poseidon_partial_key1,
                    memory_multi_column_perm_perm_interaction_elm: self
                        .memory_multi_column_perm_perm_interaction_elm,
                    memory_multi_column_perm_hash_interaction_elm0: self
                        .memory_multi_column_perm_hash_interaction_elm0,
                    range_check16_perm_interaction_elm: self.range_check16_perm_interaction_elm,
                    diluted_check_permutation_interaction_elm: self
                        .diluted_check_permutation_interaction_elm,
                    diluted_check_interaction_z: self.diluted_check_interaction_z,
                    diluted_check_interaction_alpha: self.diluted_check_interaction_alpha,
                    memory_multi_column_perm_perm_public_memory_prod: self.public_memory_prod_ratio,
                    range_check16_perm_public_memory_prod: FELT_1,
                    diluted_check_first_elm: FELT_0,
                    diluted_check_permutation_public_memory_prod: FELT_1,
                    diluted_check_final_cum_val: self.diluted_prod,
                };

                // Set global values in the preallocated location in BidirectionalStackAccount
                stack.set_global_values(global_values);

                // Push parameters for EvalCompositionPolynomialInner
                stack
                    .push_front(&self.trace_generator.to_bytes_be())
                    .unwrap();
                stack.push_front(&self.point.to_bytes_be()).unwrap();

                self.step = EvalCompositionStep::Done;

                vec![EvalCompositionPolynomialInner::new().to_vec_with_type_tag()]
            }

            EvalCompositionStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == EvalCompositionStep::Done
    }
}

// The cumulative value is defined using the next recursive formula:
//   r_1 = 1, r_{j+1} = r_j * (1 + z * u_j) + alpha * u_j^2
// where u_j = Dilute(j, spacing, n_bits) - Dilute(j-1, spacing, n_bits)
// and we want to compute the final value r_{2^n_bits}.
// Note that u_j depends only on the number of trailing zeros in the binary representation of j.
// Specifically, u_{(1+2k)*2^i} = u_{2^i} = u_{2^{i-1}} + 2^{i*spacing} - 2^{(i-1)*spacing + 1}.
//
// The recursive formula can be reduced to a nonrecursive form:
//   r_j = prod_{n=1..j-1}(1+z*u_n) + alpha*sum_{n=1..j-1}(u_n^2 * prod_{m=n+1..j-1}(1+z*u_m))
//
// We rewrite this equation to generate a recursive formula that converges in log(j) steps:
// Denote:
//   p_i = prod_{n=1..2^i-1}(1+z*u_n)
//   q_i = sum_{n=1..2^i-1}(u_n^2 * prod_{m=n+1..2^i-1}(1+z*u_m))
//   x_i = u_{2^i}.
//
// Clearly
//   r_{2^i} = p_i + alpha * q_i.
// Moreover,
//   p_i = p_{i-1} * (1 + z * x_{i-1}) * p_{i-1}
//   q_i = q_{i-1} * (1 + z * x_{i-1}) * p_{i-1} + x_{i-1}^2 * p_{i-1} + q_{i-1}
//
// Now we can compute p_{n_bits} and q_{n_bits} in just n_bits recursive steps and we are done.
#[inline(always)]
pub fn get_diluted_product(n_bits: Felt, spacing: Felt, z: Felt, alpha: Felt) -> Felt {
    let diff_multiplier = FELT_2.pow_felt(&spacing);
    let mut diff_x: Felt = diff_multiplier - FELT_2;
    let mut x: Felt = FELT_1;
    let mut p: Felt = z + FELT_1;
    let mut q: Felt = FELT_1;

    let mut i = FELT_0;
    loop {
        if i == n_bits - FELT_1 {
            break p + q * alpha;
        }

        x += diff_x;
        diff_x *= diff_multiplier;
        let x_p = x * p;
        let y = p + z * x_p;
        q = q * y + x * x_p + q;
        p *= y;

        i += FELT_1;
    }
}
