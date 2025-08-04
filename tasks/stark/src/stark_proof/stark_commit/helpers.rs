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

// Helper task for computing public memory product
#[repr(C)]
pub struct ComputePublicMemoryProduct {
    processed: bool,
}

impl_type_identifiable!(ComputePublicMemoryProduct);

impl ComputePublicMemoryProduct {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for ComputePublicMemoryProduct {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ComputePublicMemoryProduct {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.processed {
            return vec![];
        }

        let proof: &StarkProof = stack.get_proof_reference();
        let public_input = &proof.public_input;

        // Get public_memory_column_size from stack
        let public_memory_column_size = Felt::from_bytes_be_slice(stack.borrow_front());

        // Get interaction elements from stack (they should be there from GenerateInteractionElements)
        // We need memory_z and memory_alpha
        let memory_alpha = Felt::from_bytes_be_slice(stack.borrow_front());
        let memory_z = Felt::from_bytes_be_slice(stack.borrow_front());

        // Call public_input.get_public_memory_product_ratio
        let public_memory_prod_ratio = public_input.get_public_memory_product_ratio(
            memory_z,
            memory_alpha,
            public_memory_column_size,
        );

        stack.pop_front();
        stack
            .push_front(&public_memory_prod_ratio.to_bytes_be())
            .unwrap();

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}

// Helper task for computing diluted product
#[repr(C)]
pub struct ComputeDilutedProduct {
    processed: bool,
}

impl_type_identifiable!(ComputeDilutedProduct);

impl ComputeDilutedProduct {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for ComputeDilutedProduct {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ComputeDilutedProduct {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.processed {
            return vec![];
        }

        // Get diluted interaction elements from stack
        let diluted_alpha = Felt::from_bytes_be_slice(stack.borrow_front());
        let diluted_z = Felt::from_bytes_be_slice(stack.borrow_front());

        // Calculate diluted product
        let diluted_prod = get_diluted_product(
            DILUTED_N_BITS.into(),
            DILUTED_SPACING.into(),
            diluted_z,
            diluted_alpha,
        );

        // Push result to stack
        stack.push_front(&diluted_prod.to_bytes_be()).unwrap();

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}

// Helper task for computing periodic columns
#[repr(C)]
pub struct ComputePeriodicColumns {
    step: PeriodicColumnsStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeriodicColumnsStep {
    ComputePedersenPoints,
    ComputePoseidonPoints,
    Done,
}

impl_type_identifiable!(ComputePeriodicColumns);

impl ComputePeriodicColumns {
    pub fn new() -> Self {
        Self {
            step: PeriodicColumnsStep::ComputePedersenPoints,
        }
    }
}

impl Default for ComputePeriodicColumns {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ComputePeriodicColumns {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            PeriodicColumnsStep::ComputePedersenPoints => {
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;

                // Get point from stack (oods_point)
                let point = Felt::from_bytes_be_slice(stack.borrow_front());

                // Calculate n_steps
                let n_steps = FELT_2.pow_felt(&public_input.log_n_steps);

                // Calculate n_pedersen_hash_copies
                let n_pedersen_hash_copies = n_steps.field_div(
                    &NonZeroFelt::try_from(
                        Felt::from(PEDERSEN_BUILTIN_RATIO)
                            * Felt::from(PEDERSEN_BUILTIN_REPETITIONS),
                    )
                    .unwrap(),
                );

                // Calculate pedersen_point
                let pedersen_point = point.pow_felt(&n_pedersen_hash_copies);

                // Evaluate pedersen points
                let pedersen_points_x = eval_pedersen_x(pedersen_point);
                let pedersen_points_y = eval_pedersen_y(pedersen_point);

                // Push to stack
                stack.push_front(&pedersen_points_y.to_bytes_be()).unwrap();
                stack.push_front(&pedersen_points_x.to_bytes_be()).unwrap();

                self.step = PeriodicColumnsStep::ComputePoseidonPoints;
                vec![]
            }

            PeriodicColumnsStep::ComputePoseidonPoints => {
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;

                // Get point from stack
                let point = Felt::from_bytes_be_slice(stack.borrow_front());

                // Calculate n_steps
                let n_steps = FELT_2.pow_felt(&public_input.log_n_steps);

                // Calculate n_poseidon_copies
                let n_poseidon_copies =
                    n_steps.field_div(&NonZeroFelt::try_from(Felt::from(POSEIDON_RATIO)).unwrap());

                // Calculate poseidon_point
                let poseidon_point = point.pow_felt(&n_poseidon_copies);

                // Evaluate poseidon points
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

                // Push to stack
                stack
                    .push_front(&poseidon_partial_round_key1.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&poseidon_partial_round_key0.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&poseidon_full_round_key2.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&poseidon_full_round_key1.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&poseidon_full_round_key0.to_bytes_be())
                    .unwrap();

                self.step = PeriodicColumnsStep::Done;
                vec![]
            }

            PeriodicColumnsStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == PeriodicColumnsStep::Done
    }
}

// Constants (adjust values based on your implementation)
pub const DILUTED_N_BITS: u32 = 16;
pub const DILUTED_SPACING: u32 = 4;
pub const FELT_2: Felt = Felt::from_hex_unchecked("0x2");
// pub const PUBLIC_MEMORY_STEP: u32 = 8;
pub const PEDERSEN_BUILTIN_RATIO: u32 = 8;
pub const PEDERSEN_BUILTIN_REPETITIONS: u32 = 4;
pub const POSEIDON_RATIO: u32 = 8;
