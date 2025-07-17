pub mod fri_commit;
pub mod helpers;
pub mod proof_or_work;
pub mod table_commit;
pub mod traces_commit;
pub mod verify_oods;

use crate::swiftness::air::recursive_with_poseidon::Layout;
use crate::swiftness::air::recursive_with_poseidon::LayoutTrait;
use crate::swiftness::transcript::TranscriptRandomFelt;
use crate::{felt::Felt, swiftness::stark::types::StarkProof};
use lambdaworks_math::traits::ByteConversion;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};
use crate::swiftness::transcript::Transcript;

// Import and re-export actual tasks from their modules
pub use self::fri_commit::{
    FriCommit, FriCommitRound, GenerateRandomFelt, UpdateTranscriptWithVector,
};
pub use self::helpers::{
    ComputeDilutedProduct, ComputePeriodicColumns, ComputePublicMemoryProduct, PowersArray,
};
pub use self::proof_or_work::{ComputeHash, ProofOfWork, UpdateTranscriptU64};
pub use self::table_commit::TableCommit;
pub use self::traces_commit::{GenerateInteractionElements, TracesCommit, VectorCommit};
pub use self::verify_oods::{EvalCompositionPolynomial, VerifyOods};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkCommitStep {
    Init,
    TracesCommit,
    GenerateCompositionAlpha,
    ProcessCompositionAlpha,    // Odbiera wynik TranscriptRandomFelt
    GenerateTracesCoefficients, // Wywołuje PowersArray z composition_alpha
    CompositionCommit,
    GenerateInteractionAfterComposition,
    ProcessInteractionAfterComposition, // Odbiera wynik TranscriptRandomFelt
    ReadOodsValues,
    VerifyOods,
    GenerateOodsAlpha,
    ProcessOodsAlpha,         // Odbiera wynik TranscriptRandomFelt
    GenerateOodsCoefficients, // Wywołuje PowersArray z oods_alpha
    FriCommit,
    ProofOfWork,
    Done,
}

#[repr(C)]
pub struct StarkCommit {
    step: StarkCommitStep,
    // Store minimal state needed across steps
    traces_coefficients_count: u32,
    oods_coefficients_count: u32,
    transcript: Transcript,
}


impl_type_identifiable!(StarkCommit);

impl StarkCommit {
    pub fn new() -> Self {
        Self {
            step: StarkCommitStep::Init,
            traces_coefficients_count: 0,
            oods_coefficients_count: 0,
            transcript: Transcript::new(Felt::ZERO),
        }
    }
}

impl Default for StarkCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for StarkCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            StarkCommitStep::Init => {
                // Get Layout::N_CONSTRAINTS and calculate counts
                // Assuming these are available through the proof or as constants
                self.traces_coefficients_count = Layout::N_CONSTRAINTS as u32;
                self.oods_coefficients_count =
                    (Layout::MASK_SIZE + Layout::CONSTRAINT_DEGREE) as u32;

                self.step = StarkCommitStep::TracesCommit;
                vec![]
            }

            StarkCommitStep::TracesCommit => {
                // Initialize transcript state - should come from outside, but for now use placeholder
                let initial_transcript_digest = Felt::from_hex("0x1").unwrap(); // Should come from caller
                let initial_transcript_counter = Felt::ZERO;

                // Push initial transcript state to stack
                stack
                    .push_front(&initial_transcript_counter.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&initial_transcript_digest.to_bytes_be())
                    .unwrap();

                self.step = StarkCommitStep::GenerateCompositionAlpha;

                // Return TracesCommit task which will update transcript with trace commitments
                vec![TracesCommit::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::GenerateCompositionAlpha => {
                // At this point TracesCommit finished, transcript state should be updated on stack
                // Get current transcript state from stack
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push transcript input for TranscriptRandomFelt task
                TranscriptRandomFelt::push_input(transcript_digest, transcript_counter, stack);

                self.step = StarkCommitStep::ProcessCompositionAlpha;

                // Return TranscriptRandomFelt task to generate composition_alpha
                vec![
                    TranscriptRandomFelt::new(transcript_digest, transcript_counter)
                        .to_vec_with_type_tag(),
                ]
            }

            StarkCommitStep::ProcessCompositionAlpha => {
                // TranscriptRandomFelt finished, composition_alpha is on stack
                let composition_alpha = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // Also pop the remaining PoseidonHash results (3 values total)
                stack.pop_front();
                stack.pop_front();

                // Store values for PowersArray: (initial=ONE, alpha=composition_alpha)
                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();
                stack.push_front(&composition_alpha.to_bytes_be()).unwrap();

                self.step = StarkCommitStep::GenerateTracesCoefficients;
                vec![]
            }

            StarkCommitStep::GenerateTracesCoefficients => {
                // Generate traces_coefficients using PowersArray with composition_alpha
                self.step = StarkCommitStep::CompositionCommit;

                // Return PowersArray task to generate coefficients
                vec![PowersArray::new(self.traces_coefficients_count).to_vec_with_type_tag()]
            }

            StarkCommitStep::CompositionCommit => {
                // At this point, traces_coefficients are on the stack from PowersArray
                // TableCommit will update transcript with composition commitment

                stack.push_front(&self.transcript.counter().to_bytes_be()).unwrap();
                stack.push_front(&self.transcript.digest().to_bytes_be()).unwrap();

                self.step = StarkCommitStep::GenerateInteractionAfterComposition;
                vec![TableCommit::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::GenerateInteractionAfterComposition => {
                // TableCommit finished, get updated transcript state from stack
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push transcript input for TranscriptRandomFelt task
                TranscriptRandomFelt::push_input(transcript_digest, transcript_counter, stack);

                self.step = StarkCommitStep::ProcessInteractionAfterComposition;

                // Return TranscriptRandomFelt task to generate interaction_after_composition
                vec![
                    TranscriptRandomFelt::new(transcript_digest, transcript_counter)
                        .to_vec_with_type_tag(),
                ]
            }

            StarkCommitStep::ProcessInteractionAfterComposition => {
                // TranscriptRandomFelt finished, interaction_after_composition is on stack
                let interaction_after_composition = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // Pop remaining PoseidonHash results
                stack.pop_front();
                stack.pop_front();

                // Store interaction_after_composition for later use
                stack
                    .push_front(&interaction_after_composition.to_bytes_be())
                    .unwrap();

                self.step = StarkCommitStep::ReadOodsValues;
                vec![]
            }

            StarkCommitStep::ReadOodsValues => {
                let proof: &StarkProof = stack.get_proof_reference();

                // Read OODS values from unsent_commitment and process with transcript
                // For now, we'll push the count of oods_values to stack
                let oods_values_count = proof.unsent_commitment.oods_values.len() as u32;
                stack.push_front(&oods_values_count.to_bytes_be()).unwrap();

                self.step = StarkCommitStep::VerifyOods;
                vec![]
            }

            StarkCommitStep::VerifyOods => {
                // Get necessary values from stack for verify_oods
                // trace_domain_size should already be on stack from validate_public_input

                self.step = StarkCommitStep::GenerateOodsAlpha;

                // Return VerifyOods task
                vec![VerifyOods::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::GenerateOodsAlpha => {
                // VerifyOods finished, get updated transcript state from stack
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push transcript input for TranscriptRandomFelt task
                TranscriptRandomFelt::push_input(transcript_digest, transcript_counter, stack);

                self.step = StarkCommitStep::ProcessOodsAlpha;

                // Return TranscriptRandomFelt task to generate oods_alpha
                vec![
                    TranscriptRandomFelt::new(transcript_digest, transcript_counter)
                        .to_vec_with_type_tag(),
                ]
            }

            StarkCommitStep::ProcessOodsAlpha => {
                // TranscriptRandomFelt finished, oods_alpha is on stack
                let oods_alpha = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // Pop remaining PoseidonHash results
                stack.pop_front();
                stack.pop_front();

                // Store values for PowersArray: (initial=ONE, alpha=oods_alpha)
                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();
                stack.push_front(&oods_alpha.to_bytes_be()).unwrap();

                self.step = StarkCommitStep::GenerateOodsCoefficients;
                vec![]
            }

            StarkCommitStep::GenerateOodsCoefficients => {
                // Generate oods_coefficients using PowersArray with oods_alpha
                self.step = StarkCommitStep::FriCommit;

                // Return PowersArray task for oods_coefficients
                vec![PowersArray::new(self.oods_coefficients_count).to_vec_with_type_tag()]
            }

            StarkCommitStep::FriCommit => {
                // At this point, oods_coefficients are on the stack

                self.step = StarkCommitStep::ProofOfWork;

                // Return FriCommit task
                vec![FriCommit::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::ProofOfWork => {
                self.step = StarkCommitStep::Done;

                // Return ProofOfWork task
                vec![ProofOfWork::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::Done => {
                // All commitment data should now be on the stack
                // The calling function will construct StarkCommitment from stack data
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == StarkCommitStep::Done
    }
}
