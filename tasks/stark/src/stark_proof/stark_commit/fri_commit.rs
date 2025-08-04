use crate::stark_proof::stark_commit::{TableCommit, TranscriptReadFeltVector};
use crate::swiftness::stark::types::StarkProof;
use crate::swiftness::transcript::TranscriptRandomFelt;
use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FriCommitStep {
    Init,
    ProcessInnerLayer(usize),
    GenerateEvalPoint(usize),
    CollectEvalPoint(usize),
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
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match &self.step {
            FriCommitStep::Init => {
                let proof: &StarkProof = stack.get_proof_reference();
                let fri_config = &proof.config.fri;

                self.n_layers = fri_config.n_layers.to_biguint().try_into().unwrap();
                assert!(self.n_layers > 0, "Invalid n_layers value");

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
                let proof: &StarkProof = stack.get_proof_reference();
                println!(
                    "ProcessInnerLayer: layer_idx={}, n_layers={}",
                    layer_idx, self.n_layers
                );

                // Check if we've processed all inner layers (n_layers - 1 total)
                if layer_idx >= self.n_layers as usize - 1 {
                    self.step = FriCommitStep::ReadLastLayerCoefficients;
                    vec![]
                } else {
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

                    stack
                        .push_front(&self.current_transcript_digest.to_bytes_be())
                        .unwrap();

                    self.step = FriCommitStep::GenerateEvalPoint(layer_idx);

                    vec![TableCommit::new().to_vec_with_type_tag()]
                }
            }

            FriCommitStep::GenerateEvalPoint(layer_idx) => {
                let layer_idx = *layer_idx;

                let table_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let table_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.current_transcript_digest = table_digest;
                self.current_transcript_counter = table_counter;

                self.step = FriCommitStep::CollectEvalPoint(layer_idx);

                vec![TranscriptRandomFelt::new(table_digest, table_counter).to_vec_with_type_tag()]
            }

            FriCommitStep::CollectEvalPoint(layer_idx) => {
                let layer_idx = *layer_idx;

                let updated_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let eval_point = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                self.current_transcript_counter = updated_counter;
                stack.push_front(&eval_point.to_bytes_be()).unwrap();

                self.step = FriCommitStep::ProcessInnerLayer(layer_idx + 1);
                vec![]
            }

            FriCommitStep::ReadLastLayerCoefficients => {
                let proof: &StarkProof = stack.get_proof_reference();
                let last_layer_coefficients =
                    proof.unsent_commitment.fri.last_layer_coefficients.clone();

                let expected_len =
                    Felt::TWO.pow_felt(&proof.config.fri.log_last_layer_degree_bound);
                assert!(
                    expected_len == last_layer_coefficients.len().into(),
                    "Invalid last layer coefficients length"
                );

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
