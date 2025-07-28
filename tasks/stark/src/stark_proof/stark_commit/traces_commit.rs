use crate::poseidon::PoseidonHash;
use crate::stark_proof::PoseidonHashMany;
use crate::{felt::Felt, swiftness::stark::types::StarkProof};
// use lambdaworks_math::traits::ByteConversion;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracesCommitStep {
    ReadOriginalCommitment,
    GenerateInteractionElements,
    ReadInteractionCommitment,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transcript {
    digest: Felt,
    counter: Felt,
}

impl Transcript {
    pub fn new() -> Self {
        Self {
            digest: Felt::ZERO,
            counter: Felt::ZERO,
        }
    }
}

#[repr(C)]
pub struct TracesCommit {
    step: TracesCommitStep,
    interaction_elements_count: u32,
    current_element: u32,
    transcript: Transcript,
    interaction_commitment: Felt,
    original_commitment: Felt,
}

impl_type_identifiable!(TracesCommit);

impl TracesCommit {
    pub fn new(digest: Felt) -> Self {
        Self {
            step: TracesCommitStep::ReadOriginalCommitment,
            interaction_elements_count: 6, // recursive_with_poseidon has 6 interaction elements
            current_element: 0,
            transcript: Transcript {
                digest: digest,
                counter: Felt::ZERO,
            },
            interaction_commitment: Felt::ZERO,
            original_commitment: Felt::ZERO,
        }
    }
}

impl Default for TracesCommit {
    fn default() -> Self {
        Self::new(Felt::ZERO)
    }
}

impl Executable for TracesCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            TracesCommitStep::ReadOriginalCommitment => {
                let proof: &StarkProof = stack.get_proof_reference();

                let unsent_commitment = proof.unsent_commitment.traces;

                stack
                    .push_front(&self.transcript.digest.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&unsent_commitment.original.to_bytes_be())
                    .unwrap();

                self.step = TracesCommitStep::GenerateInteractionElements;

                vec![VectorCommit::new().to_vec_with_type_tag()]
            }

            TracesCommitStep::GenerateInteractionElements => {
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                self.original_commitment = transcript_digest;
                println!("original_commitment: {:?}", self.original_commitment);

                stack.push_front(&transcript_counter.to_bytes_be()).unwrap();
                stack.push_front(&transcript_digest.to_bytes_be()).unwrap();

                let transcript = Transcript {
                    digest: transcript_digest,
                    counter: transcript_counter,
                };
                self.transcript = transcript;

                self.step = TracesCommitStep::ReadInteractionCommitment;
                vec![
                    GenerateInteractionElements::new(self.interaction_elements_count)
                        .to_vec_with_type_tag(),
                ]
            }

            TracesCommitStep::ReadInteractionCommitment => {
                let proof: &StarkProof = stack.get_proof_reference();
                let interaction_commitment = proof.unsent_commitment.traces.interaction;

                stack
                    .push_front(&self.transcript.digest.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&interaction_commitment.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&self.original_commitment.to_bytes_be())
                    .unwrap();

                self.step = TracesCommitStep::Done;
                vec![VectorCommit::new().to_vec_with_type_tag()]
            }

            TracesCommitStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == TracesCommitStep::Done
    }
}

// Task for generating interaction elements
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GenerateInteractionElements {
    total_elements: u32,
    current_element: u32,
    step: GenerateInteractionStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateInteractionStep {
    GenerateHash,
    ReadResult,
}

impl_type_identifiable!(GenerateInteractionElements);

impl GenerateInteractionElements {
    pub fn new(total_elements: u32) -> Self {
        Self {
            total_elements,
            current_element: 0,
            step: GenerateInteractionStep::GenerateHash,
        }
    }
}

impl Executable for GenerateInteractionElements {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GenerateInteractionStep::GenerateHash => {
                // Get transcript digest and counter from stack
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // println!("transcript_digest in generate hash: {:?}", transcript_digest);
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // println!("transcript_counter in generate hash: {:?}", transcript_counter);

                // Store transcript state for later restoration
                stack.push_front(&transcript_counter.to_bytes_be()).unwrap();
                stack.push_front(&transcript_digest.to_bytes_be()).unwrap();

                self.step = GenerateInteractionStep::ReadResult;

                // Call PoseidonHash to generate random element
                PoseidonHash::push_input(transcript_digest, transcript_counter, stack);
                vec![PoseidonHash::new().to_vec_with_type_tag()]
            }

            GenerateInteractionStep::ReadResult => {
                let hash_result = Felt::from_bytes_be_slice(stack.borrow_front());
                println!("hash_result: {:?}", hash_result);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // println!("transcript_digest in read result: {:?}", transcript_digest);
                let transcript_counter = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                // println!("transcript_counter in read result: {:?}", transcript_counter);

                // Push generated element to result stack
                stack.push_front(&hash_result.to_bytes_be()).unwrap();

                // Update transcript counter for next iteration
                let new_counter = transcript_counter + Felt::ONE;

                self.current_element += 1;

                if self.current_element < self.total_elements {
                    stack.push_front(&new_counter.to_bytes_be()).unwrap();
                    stack.push_front(&transcript_digest.to_bytes_be()).unwrap();
                    self.step = GenerateInteractionStep::GenerateHash;
                }

                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.current_element >= self.total_elements
    }
}

// VectorCommit task
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VectorCommit {
    committed: bool,
    phase: VectorCommitPhase,
}

impl_type_identifiable!(VectorCommit);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorCommitPhase {
    CallPoseidonHashMany,
    RestoreTranscriptState,
    Done,
}

impl VectorCommit {
    pub fn new() -> Self {
        Self {
            committed: false,
            phase: VectorCommitPhase::CallPoseidonHashMany,
        }
    }
}

impl Default for VectorCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for VectorCommit {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            VectorCommitPhase::CallPoseidonHashMany => {
                let commitment = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                let transcript_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();

                // Update transcript digest: hash(digest + 1, commitment)
                let new_digest_input = transcript_digest + Felt::ONE;

                PoseidonHashMany::push_input(&[new_digest_input, commitment], stack);

                self.phase = VectorCommitPhase::RestoreTranscriptState;

                vec![PoseidonHashMany::new(2).to_vec_with_type_tag()]
            }
            VectorCommitPhase::RestoreTranscriptState => {
                let new_digest = Felt::from_bytes_be_slice(stack.borrow_front());
                stack.pop_front();
                stack.pop_front();

                let transcript_counter = Felt::ZERO;
                stack.pop_front();

                stack.push_front(&new_digest.to_bytes_be()).unwrap();
                stack.push_front(&transcript_counter.to_bytes_be()).unwrap();
                self.phase = VectorCommitPhase::Done;
                vec![]
            }
            VectorCommitPhase::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.phase == VectorCommitPhase::Done
    }
}
