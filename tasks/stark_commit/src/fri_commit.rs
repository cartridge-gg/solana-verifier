use crate::table_commit::TableCommit;
use felt::Felt;
use solana_program::msg;
use transcript::transcript::TranscriptRandomFelt;
use transcript::transcript::TranscriptReadFeltVector;
use types::swiftness::global_values::InteractionElements;
use types::swiftness::stark::types::{StarkCommitment, StarkProof};
use utils::ProofDataDecommitment;
use utils::{
    impl_type_identifiable, BidirectionalStack, Executable, ProofData, StarkCommitmentTrait,
    TypeIdentifiable,
};

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

impl Default for FriCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriCommit {
    fn execute<T: BidirectionalStack + ProofData + StarkCommitmentTrait + ProofDataDecommitment>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
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
                // Check if we've processed all inner layers (n_layers - 1 total)
                if layer_idx >= self.n_layers as usize - 1 {
                    self.step = FriCommitStep::ReadLastLayerCoefficients;
                    vec![]
                } else {
                    let (stark_commitment, proof) = stack.get_stark_commitment_and_proof_mut::<StarkCommitment<InteractionElements>, StarkProof>();

                    let inner_layer = proof
                        .unsent_commitment
                        .fri
                        .inner_layers
                        .get(layer_idx)
                        .unwrap();

                    // Instead of pushing, assign to existing element
                    let target_layer = &mut stark_commitment.fri.inner_layers.at_mut(layer_idx);
                    target_layer.vector_commitment.commitment_hash = *inner_layer;

                    let proof: &StarkProof = stack.get_proof_reference();
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

                let stark_commitment =
                    stack.get_stark_commitment_mut::<StarkCommitment<InteractionElements>>();
                stark_commitment.fri.eval_points.push(eval_point);

                self.current_transcript_counter = updated_counter;
                // stack.push_front(&eval_point.to_bytes_be()).unwrap();

                self.step = FriCommitStep::ProcessInnerLayer(layer_idx + 1);
                vec![]
            }

            FriCommitStep::ReadLastLayerCoefficients => {
                let (coef_len, chunk_size) = {
                    let (stark_commitment, proof) = stack.get_stark_commitment_and_proof_mut::<StarkCommitment<InteractionElements>, StarkProof>();
                    let last_layer_coefficients =
                        &proof.unsent_commitment.fri.last_layer_coefficients;

                    let expected_len =
                        Felt::TWO.pow_felt(&proof.config.fri.log_last_layer_degree_bound);
                    msg!(
                        "FriCommit: log_last_layer_degree_bound={}, expected_len={}, actual_len={}",
                        proof.config.fri.log_last_layer_degree_bound,
                        expected_len,
                        last_layer_coefficients.len()
                    );
                    assert!(
                        expected_len == last_layer_coefficients.len().into(),
                        "Invalid last layer coefficients length: expected {}, got {}",
                        expected_len,
                        last_layer_coefficients.len()
                    );

                    stark_commitment
                        .fri
                        .last_layer_coefficients
                        .extend(last_layer_coefficients.as_slice());

                    (
                        last_layer_coefficients.len(),
                        10.min(last_layer_coefficients.len()),
                    )
                };

                // Manually push inputs to avoid Vec allocation
                // Push zeros for padding
                let inputs_len = coef_len; // +1 for digest
                let zero_count = inputs_len.div_ceil(2) * 2 - inputs_len;
                for _ in 0..zero_count {
                    stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                }
                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();

                // Push coefficients in chunks to limit stack allocation
                let total_chunks = coef_len.div_ceil(chunk_size);
                for chunk_idx in (0..total_chunks).rev() {
                    let start = chunk_idx * chunk_size;
                    let end = ((chunk_idx + 1) * chunk_size).min(coef_len);

                    let chunk_bytes: Vec<[u8; 32]> = {
                        let stark_commitment =
                            stack.get_stark_commitment::<StarkCommitment<InteractionElements>>();
                        stark_commitment.fri.last_layer_coefficients.as_slice()[start..end]
                            .iter()
                            .map(|f| f.to_bytes_be())
                            .collect()
                    };

                    for byte_arr in chunk_bytes.iter().rev() {
                        stack.push_front(byte_arr).unwrap();
                    }
                }

                // Push digest + 1
                let digest_plus_one = self.current_transcript_digest + Felt::ONE;
                stack.push_front(&digest_plus_one.to_bytes_be()).unwrap();

                // Push initial state
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                self.step = FriCommitStep::Done;
                vec![TranscriptReadFeltVector::new(coef_len).to_vec_with_type_tag()]
            }

            FriCommitStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == FriCommitStep::Done
    }
}
