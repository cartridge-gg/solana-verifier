pub mod eval_composition_polynomial;
pub mod eval_composition_polynomial_inner;
pub mod eval_oods_polynomial_inner;
pub mod fri_commit;
pub mod helpers;
pub mod proof_of_work;
pub mod table_commit;
pub mod traces_commit;
pub mod verify_oods;

use crate::swiftness::air::recursive_with_poseidon::Layout;
use crate::swiftness::air::recursive_with_poseidon::LayoutTrait;
use crate::swiftness::air::trace::config::Config as ConfigTrace;
use crate::swiftness::air::trace::Commitment as CommitmentTrace;
use crate::swiftness::commitment::table::config::Config as ConfigTable;
use crate::swiftness::commitment::table::types::Commitment as CommitmentTable;
use crate::swiftness::commitment::vector::config::Config as ConfigVector;
use crate::swiftness::commitment::vector::types::Commitment as CommitmentVector;
use crate::swiftness::stark::types::StarkCommitment;
use crate::swiftness::stark::types::StarkProof;
use crate::swiftness::transcript::{TranscriptRandomFelt, TranscriptReadFeltVector};
use felt::Felt;
use utils::global_values::InteractionElements;
use utils::ProofData;
use utils::StarkCommitmentTrait;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

// Import and re-export actual tasks from their modules
pub use self::eval_composition_polynomial::EvalCompositionPolynomial;
pub use self::fri_commit::FriCommit;
pub use self::helpers::PowersArray;
pub use self::proof_of_work::{ComputeHash, ProofOfWork, UpdateTranscriptU64};
pub use self::table_commit::TableCommit;
pub use self::traces_commit::{GenerateInteractionElements, TracesCommit, VectorCommit};
pub use self::verify_oods::VerifyOods;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarkCommitStep {
    Init,
    TracesCommit,
    GenerateCompositionAlpha,
    GenerateTracesCoefficients,
    CompositionCommit,
    GenerateInteractionAfterComposition,
    ReadOodsValues,
    VerifyOods,
    GenerateOodsAlpha,
    GenerateOodsCoefficients,
    FriCommit,
    ProofOfWork,
    Output,
    Done,
}

#[repr(C)]
pub struct StarkCommit {
    step: StarkCommitStep,
    traces_coefficients_count: u32,
    oods_coefficients_count: u32,
    current_transcript_digest: Felt,
    current_transcript_counter: Felt,
    oods_point: Felt,
    trace_domain_size: Felt,
    trace_generator: Felt,
    memory_multi_column_perm_perm_interaction_elm: Felt,
    memory_multi_column_perm_hash_interaction_elm0: Felt,
    range_check16_perm_interaction_elm: Felt,
    diluted_check_permutation_interaction_elm: Felt,
    diluted_check_interaction_z: Felt,
    diluted_check_interaction_alpha: Felt,
}

impl_type_identifiable!(StarkCommit);

impl StarkCommit {
    pub fn new() -> Self {
        Self {
            step: StarkCommitStep::Init,
            traces_coefficients_count: 0,
            oods_coefficients_count: 0,
            current_transcript_digest: Felt::ZERO,
            current_transcript_counter: Felt::ZERO,
            oods_point: Felt::ZERO,
            trace_domain_size: Felt::ZERO,
            trace_generator: Felt::ZERO,
            memory_multi_column_perm_perm_interaction_elm: Felt::ZERO,
            memory_multi_column_perm_hash_interaction_elm0: Felt::ZERO,
            range_check16_perm_interaction_elm: Felt::ZERO,
            diluted_check_permutation_interaction_elm: Felt::ZERO,
            diluted_check_interaction_z: Felt::ZERO,
            diluted_check_interaction_alpha: Felt::ZERO,
        }
    }
}

impl Default for StarkCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for StarkCommit {
    fn execute<T: BidirectionalStack + ProofData + StarkCommitmentTrait>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
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
                // Get initial transcript state from stack (should be set by caller)
                let initial_transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let initial_transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.oods_point = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.trace_domain_size = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.trace_generator = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push initial transcript state back to stack for TracesCommit
                stack
                    .push_front(&initial_transcript_digest.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&initial_transcript_counter.to_bytes_be())
                    .unwrap();

                self.step = StarkCommitStep::GenerateCompositionAlpha;

                // Return TracesCommit task which will update transcript with trace commitments
                vec![TracesCommit::new(initial_transcript_digest).to_vec_with_type_tag()]
            }

            StarkCommitStep::GenerateCompositionAlpha => {
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let stark_commitment =
                    stack.get_stark_commitment_mut::<StarkCommitment<InteractionElements>>();
                stark_commitment
                    .traces
                    .interaction
                    .vector_commitment
                    .commitment_hash = transcript_digest;

                self.current_transcript_digest = transcript_digest;
                self.current_transcript_counter = transcript_counter;

                self.step = StarkCommitStep::GenerateTracesCoefficients;

                // Use TranscriptRandomFelt to generate composition_alpha
                vec![
                    TranscriptRandomFelt::new(transcript_digest, transcript_counter)
                        .to_vec_with_type_tag(),
                ]
            }

            StarkCommitStep::GenerateTracesCoefficients => {
                // TranscriptRandomFelt finished, get updated transcript state and random value
                let updated_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let composition_alpha = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript state from TranscriptRandomFelt result
                self.current_transcript_counter = updated_counter;

                // Store values for PowersArray: (initial=ONE, alpha=composition_alpha)
                stack.push_front(&composition_alpha.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();

                self.step = StarkCommitStep::CompositionCommit;

                // Return PowersArray task to generate coefficients
                vec![PowersArray::new(self.traces_coefficients_count).to_vec_with_type_tag()]
            }

            StarkCommitStep::CompositionCommit => {
                // At this point, traces_coefficients are on the stack from PowersArray
                // TableCommit will update transcript with composition commitment

                // Use updated transcript state with incremented counter
                stack
                    .push_front(&self.current_transcript_counter.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&self.current_transcript_digest.to_bytes_be())
                    .unwrap();
                let proof: &StarkProof = stack.get_proof_reference();
                stack
                    .push_front(&proof.unsent_commitment.composition.to_bytes_be())
                    .unwrap();

                self.step = StarkCommitStep::GenerateInteractionAfterComposition;
                vec![TableCommit::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::GenerateInteractionAfterComposition => {
                // TableCommit finished, get updated transcript state from stack
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                let stark_commitment =
                    stack.get_stark_commitment_mut::<StarkCommitment<InteractionElements>>();
                stark_commitment
                    .composition
                    .vector_commitment
                    .commitment_hash = transcript_digest;

                // Store current transcript state
                self.current_transcript_digest = transcript_digest;
                self.current_transcript_counter = transcript_counter;

                self.step = StarkCommitStep::ReadOodsValues;

                // Use TranscriptRandomFelt to generate interaction_after_composition
                vec![
                    TranscriptRandomFelt::new(transcript_digest, transcript_counter)
                        .to_vec_with_type_tag(),
                ]
            }

            StarkCommitStep::ReadOodsValues => {
                // TranscriptRandomFelt finished, get updated transcript state and random value
                let updated_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let interaction_after_composition = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript state from TranscriptRandomFelt result
                self.current_transcript_counter = updated_counter;

                // Store interaction_after_composition for later use
                stack
                    .push_front(&interaction_after_composition.to_bytes_be())
                    .unwrap();

                let proof: &StarkProof = stack.get_proof_reference();
                let oods_values = proof.unsent_commitment.oods_values.as_slice().to_vec();

                // Use TranscriptReadFeltVector to implement read_felt_vector_from_prover
                // This will: hash(digest + 1, oods_values), update digest, reset counter
                TranscriptReadFeltVector::push_input(
                    self.current_transcript_digest,
                    &oods_values,
                    stack,
                );

                self.step = StarkCommitStep::VerifyOods;
                vec![TranscriptReadFeltVector::new(oods_values.len()).to_vec_with_type_tag()]
            }

            StarkCommitStep::VerifyOods => {
                // TranscriptReadFeltVector finished, get updated transcript state
                let reseted_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let updated_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript state from TranscriptReadFeltVector result
                self.current_transcript_digest = updated_digest;
                self.current_transcript_counter = reseted_counter;

                stack
                    .push_front(&self.trace_domain_size.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&self.trace_generator.to_bytes_be())
                    .unwrap();
                stack.push_front(&self.oods_point.to_bytes_be()).unwrap();

                self.step = StarkCommitStep::GenerateOodsAlpha;

                // Return VerifyOods task
                vec![VerifyOods::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::GenerateOodsAlpha => {
                // VerifyOods finished, use current transcript state
                self.step = StarkCommitStep::GenerateOodsCoefficients;

                // Use TranscriptRandomFelt to generate oods_alpha
                vec![TranscriptRandomFelt::new(
                    self.current_transcript_digest,
                    self.current_transcript_counter,
                )
                .to_vec_with_type_tag()]
            }

            StarkCommitStep::GenerateOodsCoefficients => {
                // TranscriptRandomFelt finished, get updated transcript state and random value
                let updated_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let oods_alpha = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript state from TranscriptRandomFelt result
                self.current_transcript_counter = updated_counter;

                // Store values for PowersArray: (initial=ONE, alpha=oods_alpha)
                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();
                stack.push_front(&oods_alpha.to_bytes_be()).unwrap();

                self.step = StarkCommitStep::FriCommit;

                // Return PowersArray task for oods_coefficients
                vec![PowersArray::new(self.oods_coefficients_count).to_vec_with_type_tag()]
            }

            StarkCommitStep::FriCommit => {
                for _ in 0..self.oods_coefficients_count {
                    let oods_coefficient = Felt::from_bytes_be_slice(stack.borrow_front());
                    stack.pop_front();
                    let stark_commitment =
                        stack.get_stark_commitment_mut::<StarkCommitment<InteractionElements>>();
                    stark_commitment
                        .interaction_after_oods
                        .push(oods_coefficient);
                }

                self.step = StarkCommitStep::ProofOfWork;

                vec![FriCommit::new().to_vec_with_type_tag()]
            }

            StarkCommitStep::ProofOfWork => {
                self.step = StarkCommitStep::Output;
                vec![ProofOfWork::new().to_vec_with_type_tag()]
            }
            StarkCommitStep::Output => {
                let (stark_commitment, proof) = stack.get_stark_commitment_and_proof_mut::<StarkCommitment<InteractionElements>, StarkProof>();
                stark_commitment.oods_values =
                    proof.unsent_commitment.oods_values.as_slice().to_vec();

                self.step = StarkCommitStep::Done;
                vec![]
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
