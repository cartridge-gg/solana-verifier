use crate::poseidon::PoseidonHash;
use crate::stark_proof::stark_commit::traces_commit::VectorCommit;
use crate::stark_proof::PoseidonHashMany;
use crate::{felt::Felt, swiftness::stark::types::StarkProof};
use lambdaworks_math::traits::ByteConversion;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriCommitStep {
    ValidateConfig,
    CommitRounds,
    ReadLastLayerCoefficients,
    ValidateCoefficients,
    Done,
}

#[repr(C)]
pub struct FriCommit {
    step: FriCommitStep,
    n_layers: u32,
    current_layer: u32,
}

impl_type_identifiable!(FriCommit);

impl FriCommit {
    pub fn new() -> Self {
        Self {
            step: FriCommitStep::ValidateConfig,
            n_layers: 0,
            current_layer: 0,
        }
    }
}

impl Executable for FriCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            FriCommitStep::ValidateConfig => {
                let proof: &StarkProof = stack.get_proof_reference();
                let config = &proof.config.fri;

                // Validate n_layers > 0
                assert!(
                    config.n_layers > Felt::ZERO,
                    "FRI config n_layers must be greater than 0"
                );

                // Convert Felt to u32 using try_into()
                self.n_layers = config.n_layers.try_into().unwrap();
                self.current_layer = 0;

                self.step = FriCommitStep::CommitRounds;
                vec![]
            }

            FriCommitStep::CommitRounds => {
                if self.current_layer >= self.n_layers - 1 {
                    self.step = FriCommitStep::ReadLastLayerCoefficients;
                    return vec![];
                }

                // Process one layer at a time
                self.current_layer += 1;

                // Call FriCommitRound for current layer
                vec![FriCommitRound::new(self.current_layer - 1).to_vec_with_type_tag()]
            }

            FriCommitStep::ReadLastLayerCoefficients => {
                let proof: &StarkProof = stack.get_proof_reference();
                let coefficients = &proof.unsent_commitment.fri.last_layer_coefficients;

                let coeff_vec: Vec<Felt> = coefficients.iter().map(|f| f.clone()).collect();
                let coefficients_len = coefficients.len();

                // Push coefficients count
                stack
                    .push_front(&(coefficients_len as u32).to_be_bytes().to_vec())
                    .unwrap();

                // Push all coefficients to stack (convert to Vec first for rev())
                for coeff in coeff_vec.iter().rev() {
                    stack.push_front(&coeff.to_bytes_be()).unwrap();
                }

                self.step = FriCommitStep::ValidateCoefficients;

                // Update transcript with coefficients
                vec![UpdateTranscriptWithVector::new(coefficients_len).to_vec_with_type_tag()]
            }

            FriCommitStep::ValidateCoefficients => {
                let proof: &StarkProof = stack.get_proof_reference();
                let config = &proof.config.fri;
                let coefficients_len = proof.unsent_commitment.fri.last_layer_coefficients.len();

                // Validate that 2^log_last_layer_degree_bound == coefficients.len()
                let expected_len = Felt::TWO.pow_felt(&config.log_last_layer_degree_bound);
                assert!(
                    expected_len == Felt::from(coefficients_len),
                    "Invalid last layer coefficients length"
                );

                self.step = FriCommitStep::Done;
                vec![]
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

// Task for committing a single FRI round
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FriCommitRound {
    layer_index: u32,
    step: FriCommitRoundStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriCommitRoundStep {
    ReadCommitment,
    GenerateEvalPoint,
    Done,
}

impl_type_identifiable!(FriCommitRound);

impl FriCommitRound {
    pub fn new(layer_index: u32) -> Self {
        Self {
            layer_index,
            step: FriCommitRoundStep::ReadCommitment,
        }
    }
}

impl Executable for FriCommitRound {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            FriCommitRoundStep::ReadCommitment => {
                let proof: &StarkProof = stack.get_proof_reference();
                let layer_commitment =
                    proof.unsent_commitment.fri.inner_layers.as_slice()[self.layer_index as usize];

                // Push commitment to stack
                stack.push_front(&layer_commitment.to_bytes_be()).unwrap();

                self.step = FriCommitRoundStep::GenerateEvalPoint;

                // Call VectorCommit to process commitment
                vec![VectorCommit::new().to_vec_with_type_tag()]
            }

            FriCommitRoundStep::GenerateEvalPoint => {
                // Generate evaluation point from transcript
                // This would be transcript.random_felt_to_prover()

                self.step = FriCommitRoundStep::Done;

                // Generate random eval point
                vec![GenerateRandomFelt::new().to_vec_with_type_tag()]
            }

            FriCommitRoundStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == FriCommitRoundStep::Done
    }
}

// Task for updating transcript with vector of felts
#[derive(Debug, Clone)]
#[repr(C)]
pub struct UpdateTranscriptWithVector {
    count: usize,
    processed: bool,
}

impl_type_identifiable!(UpdateTranscriptWithVector);

impl UpdateTranscriptWithVector {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            processed: false,
        }
    }
}

impl Executable for UpdateTranscriptWithVector {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.processed {
            return vec![];
        }

        // Get transcript digest from stack
        let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        // Prepare values for poseidon_hash_many
        // First value is digest + 1
        let digest_plus_one = transcript_digest + Felt::ONE;
        stack.push_front(&digest_plus_one.to_bytes_be()).unwrap();

        // Count is already on stack from ReadLastLayerCoefficients
        // Values are already on stack

        // Total count for hash_many is 1 + coefficients count
        let total_count = 1 + self.count;
        stack
            .push_front(&(total_count as u32).to_be_bytes().to_vec())
            .unwrap();

        self.processed = true;

        // Call PoseidonHashMany
        vec![PoseidonHashMany::new(total_count).to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}

// Task for generating random felt from transcript
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GenerateRandomFelt {
    step: GenerateRandomFeltStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateRandomFeltStep {
    GenerateHash,
    ReadResult,
}

impl_type_identifiable!(GenerateRandomFelt);

impl GenerateRandomFelt {
    pub fn new() -> Self {
        Self {
            step: GenerateRandomFeltStep::GenerateHash,
        }
    }
}

impl Executable for GenerateRandomFelt {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GenerateRandomFeltStep::GenerateHash => {
                // Get transcript state from stack
                let digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Store values for next phase
                stack.push_front(&counter.to_bytes_be()).unwrap();
                stack.push_front(&digest.to_bytes_be()).unwrap();

                // Generate random value using PoseidonHash
                self.step = GenerateRandomFeltStep::ReadResult;
                vec![PoseidonHash::new(digest, counter).to_vec_with_type_tag()]
            }

            GenerateRandomFeltStep::ReadResult => {
                // Get the hash result from stack
                let hash_result = stack.borrow_front().clone();
                let hash_felt = Felt::from_bytes_be_slice(&hash_result);
                stack.pop_front();

                // Get stored transcript state
                let digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Push generated value
                stack.push_front(&hash_felt.to_bytes_be()).unwrap();

                // Update transcript state
                let new_counter = counter + Felt::ONE;
                stack.push_front(&new_counter.to_bytes_be()).unwrap();
                stack.push_front(&digest.to_bytes_be()).unwrap();

                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == GenerateRandomFeltStep::ReadResult
    }
}
