use crate::felt::Felt;
use crate::stark_proof::stark_commit::{PoseidonHash, TableCommit, TranscriptReadFeltVector};
use crate::swiftness::stark::types::StarkProof;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone)]
pub enum FriCommitStep {
    Init,
    ProcessInnerLayer(usize), // Track which layer we're processing
    GenerateEvalPoint(usize), // Generate eval point after TableCommit
    CollectEvalPoint(usize),  // Collect eval point from PoseidonHash
    ReadLastLayerCoefficients,
    Done,
}

impl_type_identifiable!(FriCommit);

#[repr(C)]
pub struct FriCommit {
    step: FriCommitStep,
    n_layers: u32,
    current_transcript_digest: Felt,
    current_transcript_counter: Felt,
}

impl FriCommit {
    pub fn new() -> Self {
        Self {
            step: FriCommitStep::Init,
            n_layers: 0,
            current_transcript_digest: Felt::ZERO,
            current_transcript_counter: Felt::ZERO,
        }
    }
}

impl Executable for FriCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match &self.step {
            FriCommitStep::Init => {
                // Get FRI config from proof
                let proof: &StarkProof = stack.get_proof_reference();
                let fri_config = &proof.config.fri;

                // Validate n_layers
                assert!(
                    fri_config.n_layers > Felt::from(0),
                    "Invalid n_layers value"
                );

                self.n_layers = fri_config.n_layers.to_biguint().try_into().unwrap();

                // Get transcript state from stack (pushed by StarkCommit)
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.current_transcript_digest = transcript_digest;
                self.current_transcript_counter = transcript_counter;

                // If n_layers == 1, go directly to reading last layer coefficients
                if self.n_layers == 1 {
                    self.step = FriCommitStep::ReadLastLayerCoefficients;
                } else {
                    self.step = FriCommitStep::ProcessInnerLayer(0);
                }

                vec![]
            }

            FriCommitStep::ProcessInnerLayer(layer_idx) => {
                let layer_idx = *layer_idx;

                // Check if we've processed all inner layers
                if layer_idx >= self.n_layers as usize - 1 {
                    self.step = FriCommitStep::ReadLastLayerCoefficients;
                    vec![]
                } else {
                    // Push transcript state for TableCommit
                    // stack
                    //     .push_front(&self.current_transcript_counter.to_bytes_be())
                    //     .unwrap();
                    stack
                        .push_front(&self.current_transcript_digest.to_bytes_be())
                        .unwrap();

                    // After TableCommit, we'll generate eval_point (except for last inner layer)
                    if layer_idx < self.n_layers as usize - 2 {
                        self.step = FriCommitStep::GenerateEvalPoint(layer_idx);
                    } else {
                        // Last inner layer, no eval_point needed
                        self.step = FriCommitStep::ProcessInnerLayer(layer_idx + 1);
                    }

                    vec![TableCommit::new().to_vec_with_type_tag()]
                }
            }

            FriCommitStep::GenerateEvalPoint(layer_idx) => {
                let layer_idx = *layer_idx;

                // TableCommit finished, get updated transcript state
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // TableCommit already pushed its commitment hash to stack
                // We'll leave it there for StarkCommit to collect later

                self.current_transcript_digest = transcript_digest;
                self.current_transcript_counter = transcript_counter;

                // Generate eval_point using PoseidonHash
                PoseidonHash::push_input(transcript_digest, transcript_counter, stack);

                self.step = FriCommitStep::CollectEvalPoint(layer_idx);

                vec![PoseidonHash::new().to_vec_with_type_tag()]
            }

            FriCommitStep::CollectEvalPoint(layer_idx) => {
                let layer_idx = *layer_idx;

                // PoseidonHash result is on stack
                let eval_point = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // Pop remaining PoseidonHash results
                stack.pop_front();

                // Increment transcript counter (random_felt_to_prover behavior)
                self.current_transcript_counter += Felt::ONE;

                // Push eval_point back to stack for StarkCommit to collect
                stack.push_front(&eval_point.to_bytes_be()).unwrap();

                // Continue with next layer
                self.step = FriCommitStep::ProcessInnerLayer(layer_idx + 1);
                vec![]
            }

            FriCommitStep::ReadLastLayerCoefficients => {
                // If we had inner layers and just finished the last one
                if self.n_layers > 1 {
                    // Check if we're coming from last TableCommit
                    let counter_bytes = stack.borrow_front();
                    if !counter_bytes.is_empty() {
                        // Get updated transcript state from last TableCommit
                        let transcript_counter = Felt::from_bytes_be_slice(counter_bytes);
                        stack.pop_front();
                        let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                        stack.pop_front();

                        self.current_transcript_digest = transcript_digest;
                        self.current_transcript_counter = transcript_counter;
                    }
                }

                // Read last layer coefficients from proof
                let proof: &StarkProof = stack.get_proof_reference();
                let last_layer_coefficients =
                    proof.unsent_commitment.fri.last_layer_coefficients.clone();

                // Validate coefficients length
                let expected_len =
                    Felt::TWO.pow_felt(&proof.config.fri.log_last_layer_degree_bound);
                assert!(
                    expected_len == last_layer_coefficients.len().into(),
                    "Invalid last layer coefficients length"
                );

                // Use TranscriptReadFeltVector to read coefficients
                TranscriptReadFeltVector::push_input(
                    self.current_transcript_digest,
                    last_layer_coefficients.as_slice(),
                    stack,
                );

                self.step = FriCommitStep::Done;

                vec![
                    TranscriptReadFeltVector::new(last_layer_coefficients.as_slice().len())
                        .to_vec_with_type_tag(),
                ]
            }

            FriCommitStep::Done => {
                // TranscriptReadFeltVector finished, get updated digest
                let new_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push final transcript state to stack
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap(); // Reset counter after read_felt_vector
                stack.push_front(&new_digest.to_bytes_be()).unwrap();

                // At this point, the stack contains (from front to back):
                // - transcript_counter (0)
                // - transcript_digest (updated)
                // - inner_layer_commitments[0] hash (from TableCommit)
                // - eval_points[0] (from PoseidonHash)
                // - inner_layer_commitments[1] hash (from TableCommit)
                // - eval_points[1] (from PoseidonHash)
                // - ...
                // - inner_layer_commitments[n-2] hash (from TableCommit)
                //
                // StarkCommit will collect these in order

                vec![]
            }
        }
    }
}
