use crate::stark_proof::segments;
use crate::stark_proof::{MAX_LOG_N_STEPS, MAX_RANGE_CHECK};
use crate::swiftness::stark::types::StarkProof;
use felt::Felt;
use felt::NonZeroFelt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatePublicInputStep {
    Validate,
    Done,
}

#[repr(C)]
pub struct ValidatePublicInput {
    step: ValidatePublicInputStep,
}

impl_type_identifiable!(ValidatePublicInput);

impl ValidatePublicInput {
    pub fn new() -> Self {
        Self {
            step: ValidatePublicInputStep::Validate,
        }
    }
}

impl Default for ValidatePublicInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ValidatePublicInput {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            ValidatePublicInputStep::Validate => {
                let proof: &StarkProof = stack.get_proof_reference();
                let public_input = &proof.public_input;

                let log_trace_domain_size = proof.config.log_trace_domain_size;
                let trace_domain_size = Felt::TWO.pow_felt(&log_trace_domain_size);

                // 1. Validate log_n_steps
                assert!(
                    public_input.log_n_steps < MAX_LOG_N_STEPS,
                    "log_n_steps exceeds maximum"
                );

                // 2. Validate trace length
                let n_steps = FELT_2.pow_felt(&public_input.log_n_steps);
                let expected_trace_length =
                    n_steps * Felt::from(CPU_COMPONENT_HEIGHT) * Felt::from(CPU_COMPONENT_STEP);

                assert!(
                    expected_trace_length == trace_domain_size,
                    "Trace length is invalid"
                );

                // 3. Validate segments count
                assert!(
                    public_input.segments.len() == segments::N_SEGMENTS,
                    "Invalid number of segments"
                );

                // 4. Validate range check bounds
                assert!(
                    FELT_0 <= public_input.range_check_min,
                    "Range check min is invalid"
                );
                assert!(
                    public_input.range_check_min < public_input.range_check_max,
                    "Range check min must be less than max"
                );
                assert!(
                    public_input.range_check_max <= MAX_RANGE_CHECK,
                    "Range check max exceeds maximum"
                );

                // 5. Validate layout
                assert!(public_input.layout == LAYOUT_CODE, "Invalid layout code");

                // 6. Validate output uses
                let output_segment = &public_input.segments.as_slice()[segments::OUTPUT];
                let output_uses = output_segment.stop_ptr - output_segment.begin_addr;
                assert!(
                    output_uses <= u128::MAX.into(),
                    "Output uses exceed maximum"
                );

                // 7. Validate pedersen uses
                let pedersen_copies = trace_domain_size.field_div(
                    &NonZeroFelt::try_from(Felt::from(PEDERSEN_BUILTIN_ROW_RATIO))
                        .expect("PEDERSEN_BUILTIN_ROW_RATIO should be non-zero"),
                );
                let pedersen_segment = &public_input.segments.as_slice()[segments::PEDERSEN];
                let pedersen_uses = (pedersen_segment.stop_ptr - pedersen_segment.begin_addr)
                    .field_div(&NonZeroFelt::from_felt_unchecked(FELT_3));
                assert!(
                    pedersen_uses <= pedersen_copies,
                    "Pedersen uses exceed copies"
                );

                // 8. Validate range check uses
                let range_check_copies = trace_domain_size.field_div(
                    &NonZeroFelt::try_from(Felt::from(RANGE_CHECK_BUILTIN_ROW_RATIO))
                        .expect("RANGE_CHECK_BUILTIN_ROW_RATIO should be non-zero"),
                );
                let range_check_segment = &public_input.segments.as_slice()[segments::RANGE_CHECK];
                let range_check_uses =
                    range_check_segment.stop_ptr - range_check_segment.begin_addr;
                assert!(
                    range_check_uses <= range_check_copies,
                    "Range check uses exceed copies"
                );

                // 9. Validate bitwise uses
                let bitwise_copies = trace_domain_size.field_div(
                    &NonZeroFelt::try_from(Felt::from(BITWISE_ROW_RATIO))
                        .expect("BITWISE_ROW_RATIO should be non-zero"),
                );
                let bitwise_segment = &public_input.segments.as_slice()[segments::BITWISE];
                let bitwise_uses = (bitwise_segment.stop_ptr - bitwise_segment.begin_addr)
                    .field_div(&NonZeroFelt::from_felt_unchecked(FELT_5));
                assert!(bitwise_uses <= bitwise_copies, "Bitwise uses exceed copies");

                // 10. Validate poseidon uses
                let poseidon_copies = trace_domain_size.field_div(
                    &NonZeroFelt::try_from(Felt::from(POSEIDON_ROW_RATIO))
                        .expect("POSEIDON_ROW_RATIO should be non-zero"),
                );
                let poseidon_segment = &public_input.segments.as_slice()[segments::POSEIDON];
                let poseidon_uses = (poseidon_segment.stop_ptr - poseidon_segment.begin_addr)
                    .field_div(&NonZeroFelt::from_felt_unchecked(FELT_6));
                assert!(
                    poseidon_uses <= poseidon_copies,
                    "Poseidon uses exceed copies"
                );

                self.step = ValidatePublicInputStep::Done;
                vec![]
            }

            ValidatePublicInputStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == ValidatePublicInputStep::Done
    }
}

// Constants
pub const FELT_0: Felt = Felt::ZERO;
pub const FELT_2: Felt = Felt::from_hex_unchecked("0x2");
pub const FELT_3: Felt = Felt::from_hex_unchecked("0x3");
pub const FELT_5: Felt = Felt::from_hex_unchecked("0x5");
pub const FELT_6: Felt = Felt::from_hex_unchecked("0x6");

pub const CPU_COMPONENT_HEIGHT: u32 = 16;
pub const CPU_COMPONENT_STEP: u32 = 1;
pub const LAYOUT_CODE: Felt =
    Felt::from_hex_unchecked("0x7265637572736976655f776974685f706f736569646f6e");

pub const PEDERSEN_BUILTIN_ROW_RATIO: u32 = 4096;
pub const RANGE_CHECK_BUILTIN_ROW_RATIO: u32 = 256;
pub const BITWISE_ROW_RATIO: u32 = 256;
pub const POSEIDON_ROW_RATIO: u32 = 1024;
