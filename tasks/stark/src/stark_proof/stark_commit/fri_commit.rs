use crate::felt::Felt;
use crate::stark_proof::stark_commit::{TableCommit, TranscriptReadFeltVector};
use crate::swiftness::stark::types::StarkProof;
use crate::swiftness::transcript::TranscriptRandomFelt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FriCommitStep {
    Init,
    ProcessInnerLayer(usize), // TableCommit for inner layer
    GenerateEvalPoint(usize), // TranscriptRandomFelt for eval_point
    CollectEvalPoint(usize),  // Collect eval_point and continue
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

                self.n_layers = fri_config.n_layers.to_biguint().try_into().unwrap();

                // Validate n_layers
                assert!(self.n_layers > 0, "Invalid n_layers value");

                // Get transcript state from stack (pushed by StarkCommit or test)
                // Test pushes: counter first, then digest
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                println!("transcript_counter: {:?}", transcript_counter);
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                println!("transcript_digest: {:?}", transcript_digest);

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
                let proof: &StarkProof = stack.get_proof_reference();
                println!(
                    "ProcessInnerLayer: layer_idx={}, n_layers={}",
                    layer_idx, self.n_layers
                );

                // Check if we've processed all inner layers (n_layers - 1 total)
                if layer_idx >= self.n_layers as usize - 1 {
                    println!(
                        "Finished processing inner layers, going to ReadLastLayerCoefficients"
                    );
                    self.step = FriCommitStep::ReadLastLayerCoefficients;
                    vec![]
                } else {
                    //push unsent commitment for commitments vector
                    //commitments are int fact just unsent_commitment.fri.inner_layers
                    stack
                        .push_front(
                            &proof
                                .unsent_commitment
                                .fri
                                .inner_layers
                                .get(layer_idx)
                                .unwrap()
                                .to_bytes_be(),
                        )
                        .unwrap();

                    // Push transcript state for TableCommit
                    stack
                        .push_front(&self.current_transcript_digest.to_bytes_be())
                        .unwrap();

                    self.step = FriCommitStep::GenerateEvalPoint(layer_idx);

                    // First: TableCommit (like original table_commit call)
                    vec![TableCommit::new().to_vec_with_type_tag()]
                }
            }

            FriCommitStep::GenerateEvalPoint(layer_idx) => {
                let layer_idx = *layer_idx;
                println!("GenerateEvalPoint: layer_idx={}", layer_idx);

                // TableCommit finished, get results: [counter, digest]
                let table_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let table_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript state with TableCommit results
                self.current_transcript_digest = table_digest;
                self.current_transcript_counter = table_counter;

                self.step = FriCommitStep::CollectEvalPoint(layer_idx);

                // Second: TranscriptRandomFelt using transcript state AFTER TableCommit
                vec![TranscriptRandomFelt::new(table_digest, table_counter).to_vec_with_type_tag()]
            }

            FriCommitStep::CollectEvalPoint(layer_idx) => {
                let layer_idx = *layer_idx;
                println!("CollectEvalPoint: layer_idx={}", layer_idx);

                // TranscriptRandomFelt finished, get results: [counter, eval_point]
                let updated_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let eval_point = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript state from TranscriptRandomFelt result
                self.current_transcript_counter = updated_counter;

                // Push eval_point back to stack for final collection
                println!(
                    "Pushing eval_point for layer {}: {:?}",
                    layer_idx, eval_point
                );
                stack.push_front(&eval_point.to_bytes_be()).unwrap();

                // Continue with next layer
                self.step = FriCommitStep::ProcessInnerLayer(layer_idx + 1);
                vec![]
            }

            FriCommitStep::ReadLastLayerCoefficients => {
                // Transcript state is already updated by the last CollectEvalPoint
                // No need to read from stack - eval_points and commitments are on stack for final collection

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

                // Note: We need to use the exact length since TranscriptReadFeltVector::new()
                // automatically handles the +1 for digest internally
                vec![
                    TranscriptReadFeltVector::new(last_layer_coefficients.as_slice().len())
                        .to_vec_with_type_tag(),
                ]
            }

            FriCommitStep::Done => {
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

    fn is_finished(&mut self) -> bool {
        self.step == FriCommitStep::Done
    }
}
