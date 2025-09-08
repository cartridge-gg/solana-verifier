use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

const MAX_DOMAIN_SIZE: Felt = Felt::from_hex_unchecked("0x40");
const FIELD_GENERATOR: Felt = Felt::from_hex_unchecked("0x3");

// EvalOodsBoundaryPolyAtPoints task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct EvalOodsBoundaryPolyAtPoints {
    n_original_columns: u32,
    n_interaction_columns: u32,
    processed: bool,
}

impl_type_identifiable!(EvalOodsBoundaryPolyAtPoints);

impl EvalOodsBoundaryPolyAtPoints {
    pub fn new(n_original_columns: u32, n_interaction_columns: u32) -> Self {
        Self {
            n_original_columns,
            n_interaction_columns,
            processed: false,
        }
    }
}

impl Default for EvalOodsBoundaryPolyAtPoints {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Executable for EvalOodsBoundaryPolyAtPoints {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // OODS boundary polynomial evaluation based on original:
        // let oods_poly_evals = eval_oods_boundary_poly_at_points::<Layout>(
        //     n_original_columns,
        //     n_interaction_columns,
        //     public_input,
        //     &eval_info,
        //     &points,
        //     &witness.traces_decommitment,
        //     &witness.composition_decommitment,
        // );

        // TODO: Implement actual OODS boundary polynomial evaluation:
        // 1. Read query points from stack (computed from queries_to_points)
        // 2. Get OodsEvaluationInfo:
        //    - oods_values from commitment
        //    - oods_point from commitment.interaction_after_composition
        //    - trace_generator from stark_domains
        //    - constraint_coefficients from commitment.interaction_after_oods
        // 3. Get public_input from proof data
        // 4. Get traces_decommitment and composition_decommitment from witness
        // 5. Evaluate OODS boundary polynomial at the given points
        // 6. Return evaluations for FRI decommitment

        // For now, just placeholder
        // Push dummy oods_poly_evals for FRI
        stack.push_front(&Felt::ONE.to_bytes_be()).unwrap(); // Placeholder evaluation

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}

// ComputeQueryPoints task
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeQueryPoints {
    processed: bool,
}

impl_type_identifiable!(ComputeQueryPoints);

impl ComputeQueryPoints {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for ComputeQueryPoints {
    fn default() -> Self {
        Self::new()
    }
}

// Stack layout pre-execution:
// ┌──────────────────────────────┐
// │ query_n                      │
// │ query_n-1                    │
// │   ...                        │
// │ query_1                      │
// │ query_0                      │
// │ queries_len                  │
// │ eval_generator               │
// │ log_eval_domain_size         │
// └──────────────────────────────┘  <- front (stack front)

impl Executable for ComputeQueryPoints {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let log_eval_domain_size = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        let eval_generator = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        // Evaluation domains of size greater than 2**64 are not supported
        assert!(log_eval_domain_size <= MAX_DOMAIN_SIZE);

        let shift = Felt::TWO.pow_felt(&(MAX_DOMAIN_SIZE - log_eval_domain_size));

        let mut points = Vec::with_capacity(queries_len.to_biguint().try_into().unwrap());

        for _ in 0..queries_len.to_biguint().try_into().unwrap() {
            let query = Felt::from_bytes_be_slice(stack.borrow_front());
            let index: u64 = (query * shift).to_biguint().try_into().unwrap();
            let point = FIELD_GENERATOR * eval_generator.pow(index.reverse_bits());
            points.push(point);
            stack.pop_front();
        }

        for point in points.iter().rev() {
            stack.push_front(&point.to_bytes_be()).unwrap();
        }
        stack.push_front(&queries_len.to_bytes_be()).unwrap();

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
