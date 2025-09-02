use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

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

impl Executable for ComputeQueryPoints {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Compute query points based on original:
        // let points = queries_to_points(queries, stark_domains);

        // TODO: Implement queries_to_points logic:
        // 1. Read queries from previous task
        // 2. Get stark_domains (trace_generator, etc.)
        // 3. Convert queries to evaluation points
        // 4. Push points to stack for OODS evaluation

        // For now, just placeholder
        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
