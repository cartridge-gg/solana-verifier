use crate::eval_oods_polynomial::EvalOodsPolynomial;
use felt::{Felt, NonZeroFelt};
use types::funvec::FUNVEC_QUERY_INDICES;
use types::swiftness::air::domains::STARK_PRIME_MINUS_ONE;
use types::swiftness::air::recursive_with_poseidon::{Layout, StaticLayoutTrait};
use types::swiftness::global_values::InteractionElements;
use types::swiftness::stark::types::{FriVerifyData, StarkCommitment, StarkProof};
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkVerifyTrait,
    TypeIdentifiable, CONSTRAINT_DEGREE,
};

const MAX_DOMAIN_SIZE: Felt = Felt::from_hex_unchecked("0x40");
const FIELD_GENERATOR: Felt = Felt::from_hex_unchecked("0x3");

// EvalOodsBoundaryPolyAtPoints task - using fixed-size arrays for Solana BPF
#[derive(Debug, Clone)]
#[repr(C)]
pub struct EvalOodsBoundaryPolyAtPoints {
    n_original_columns: u32,
    n_interaction_columns: u32,
    step: EvalOodsBoundaryStep,
    points_count: usize,
    current_point_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalOodsBoundaryStep {
    ReadPoints,
    PreparePoint,
    CollectResult,
    Done,
}

impl_type_identifiable!(EvalOodsBoundaryPolyAtPoints);

impl EvalOodsBoundaryPolyAtPoints {
    pub fn new() -> Self {
        Self {
            n_original_columns: 0,
            n_interaction_columns: 0,
            step: EvalOodsBoundaryStep::ReadPoints,
            points_count: 0,
            current_point_index: 0,
        }
    }
}

impl Default for EvalOodsBoundaryPolyAtPoints {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for EvalOodsBoundaryPolyAtPoints {
    fn execute<T: BidirectionalStack + ProofData + StarkVerifyTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        match self.step {
            EvalOodsBoundaryStep::ReadPoints => {
                self.n_original_columns = Layout::NUM_COLUMNS_FIRST;
                self.n_interaction_columns = Layout::NUM_COLUMNS_SECOND;

                // Read queries count and points from stack
                let queries_len = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.points_count = queries_len.to_biguint().try_into().unwrap();

                assert!(
                    self.points_count <= FUNVEC_QUERY_INDICES,
                    "Too many query points: {} > {}",
                    self.points_count,
                    FUNVEC_QUERY_INDICES
                );

                // assert!(
                //     decommitment.original.values.len() as u32 == points.len() as u32 * n_original_columns,
                //     "Invalid value"
                // );
                // assert!(
                //     decommitment.interaction.values.len() as u32 == points.len() as u32 * n_interaction_columns,
                //     "Invalid value"
                // );
                // assert!(
                //     composition_decommitment.values.len() == points.len() * Layout::CONSTRAINT_DEGREE,
                //     "Invalid value"
                // );

                // Initialize evaluations array (already zeroed in constructor)
                self.current_point_index = 0;

                self.step = EvalOodsBoundaryStep::PreparePoint;
                vec![]
            }

            EvalOodsBoundaryStep::PreparePoint => {
                if self.current_point_index >= self.points_count {
                    self.step = EvalOodsBoundaryStep::Done;
                    return vec![];
                }

                let current_point = {
                    let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                    let points = &fri_verify_data.fri_decommitment.points;
                    let current_point = points.at(self.current_point_index);
                    *current_point
                };

                let (stark_commitment, proof) = stack.get_stark_commitment_and_proof::<StarkCommitment<InteractionElements>, StarkProof>();

                // Extract OODS evaluation info from commitment and proof
                let oods_point = stark_commitment.interaction_after_composition;

                // Get trace generator from global values
                let log_trace_domain_size = proof.config.log_trace_domain_size;
                let trace_domain_size = Felt::TWO.pow_felt(&log_trace_domain_size);
                let trace_generator = FIELD_GENERATOR.pow_felt(
                    &STARK_PRIME_MINUS_ONE
                        .field_div(&NonZeroFelt::try_from(trace_domain_size).unwrap()),
                );

                // Get decommitment data
                let traces_decommitment = &proof.witness.traces_decommitment;
                let composition_decommitment = &proof.witness.composition_decommitment;

                // Collect column values for this point (following the original algorithm)
                let mut column_values = Vec::with_capacity(
                    self.n_original_columns as usize
                        + self.n_interaction_columns as usize
                        + CONSTRAINT_DEGREE,
                );

                let i = self.current_point_index;

                // Add original trace columns
                let original_start = i * self.n_original_columns as usize;
                let original_end = (i + 1) * self.n_original_columns as usize;
                column_values.extend(
                    &traces_decommitment.original.values.as_slice()[original_start..original_end],
                );

                // Add interaction trace columns
                let interaction_start = i * self.n_interaction_columns as usize;
                let interaction_end = (i + 1) * self.n_interaction_columns as usize;
                column_values.extend(
                    &traces_decommitment.interaction.values.as_slice()
                        [interaction_start..interaction_end],
                );

                // Add composition columns
                let composition_start = i * CONSTRAINT_DEGREE;
                let composition_end = (i + 1) * CONSTRAINT_DEGREE;
                column_values.extend(
                    &composition_decommitment.values.as_slice()[composition_start..composition_end],
                );

                // Store column values in the preallocated column_values array
                let column_values_array = stack.get_proof_data_references::<StarkProof>().6;
                for (j, &value) in column_values.iter().enumerate() {
                    if j < column_values_array.len() {
                        column_values_array[j] = value;
                    }
                }

                // Push evaluation parameters for EvalOodsPolynomial
                stack.push_front(&trace_generator.to_bytes_be()).unwrap();
                stack.push_front(&oods_point.to_bytes_be()).unwrap();
                stack.push_front(&current_point.to_bytes_be()).unwrap();

                self.step = EvalOodsBoundaryStep::CollectResult;
                vec![EvalOodsPolynomial::new().to_vec_with_type_tag()]
            }

            EvalOodsBoundaryStep::CollectResult => {
                let evaluation = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
                fri_verify_data.fri_decommitment.values.push(evaluation);

                self.current_point_index += 1;
                self.step = EvalOodsBoundaryStep::PreparePoint;
                vec![]
            }

            EvalOodsBoundaryStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == EvalOodsBoundaryStep::Done
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
