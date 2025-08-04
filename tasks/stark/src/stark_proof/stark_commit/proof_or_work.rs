use crate::stark_proof::PoseidonHashMany;
use crate::swiftness::stark::types::StarkProof;
use lambdaworks_math::traits::ByteConversion;
// use sha3::{Digest, Keccak256};
use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofOfWorkStep {
    PrepareInitialHash,
    ComputeInitialHash,
    ComputeFinalHash,
    VerifyWork,
    UpdateTranscript,
    Done,
}

#[repr(C)]
pub struct ProofOfWork {
    step: ProofOfWorkStep,
    n_bits: u8,
    nonce: u64,
}

impl_type_identifiable!(ProofOfWork);

impl ProofOfWork {
    pub fn new() -> Self {
        Self {
            step: ProofOfWorkStep::PrepareInitialHash,
            n_bits: 0,
            nonce: 0,
        }
    }
}

impl Default for ProofOfWork {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ProofOfWork {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            ProofOfWorkStep::PrepareInitialHash => {
                let proof: &StarkProof = stack.get_proof_reference();
                let config = &proof.config.proof_of_work;

                self.n_bits = config.n_bits;
                self.nonce = proof.unsent_commitment.proof_of_work.nonce;

                // Get transcript digest
                let digest = Felt::from_bytes_be_slice(stack.borrow_front());
                let digest_bytes = digest.to_bytes_be();

                // Prepare data for initial hash: MAGIC || digest || n_bits
                // Total 41 bytes

                // Push n_bits (1 byte)
                stack.push_front(&[self.n_bits]).unwrap();

                // Push digest (32 bytes)
                stack.push_front(&digest_bytes).unwrap();

                // Push MAGIC (8 bytes)
                let magic_bytes = MAGIC.to_be_bytes();
                stack.push_front(&magic_bytes).unwrap();

                // Push total length for hash
                stack.push_front(&41u32.to_bytes_be()).unwrap();

                self.step = ProofOfWorkStep::ComputeInitialHash;

                // Call hash function (Blake2s or Keccak depending on feature)
                vec![ComputeHash::new(41).to_vec_with_type_tag()]
            }

            ProofOfWorkStep::ComputeInitialHash => {
                // Initial hash is now on stack
                // Prepare data for final hash: init_hash || nonce
                // Total 40 bytes

                // Push nonce (8 bytes)
                let nonce_bytes = self.nonce.to_be_bytes();
                stack.push_front(&nonce_bytes).unwrap();

                // init_hash is already on stack (32 bytes)

                // Push total length
                stack.push_front(&40u32.to_bytes_be()).unwrap();

                self.step = ProofOfWorkStep::ComputeFinalHash;

                // Call hash function again
                vec![ComputeHash::new(40).to_vec_with_type_tag()]
            }

            ProofOfWorkStep::ComputeFinalHash => {
                // Final hash is now on stack
                self.step = ProofOfWorkStep::VerifyWork;
                vec![]
            }

            ProofOfWorkStep::VerifyWork => {
                // Get final hash from stack
                let final_hash_bytes = stack.borrow_front();

                // Check first 16 bytes (128 bits)
                let work_value = Felt::from_bytes_be_slice(&final_hash_bytes[0..16]);
                let threshold = Felt::TWO.pow(128 - self.n_bits);

                assert!(work_value < threshold, "Proof of work verification failed");

                // Pop the hash
                stack.pop_front();

                self.step = ProofOfWorkStep::UpdateTranscript;
                vec![]
            }

            ProofOfWorkStep::UpdateTranscript => {
                // Update transcript with nonce
                // Push nonce as u64 (8 bytes)
                let nonce_bytes = self.nonce.to_be_bytes();
                stack.push_front(&nonce_bytes).unwrap();

                self.step = ProofOfWorkStep::Done;

                // Call UpdateTranscriptU64
                vec![UpdateTranscriptU64::new().to_vec_with_type_tag()]
            }

            ProofOfWorkStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == ProofOfWorkStep::Done
    }
}

// Task for computing hash (Blake2s or Keccak)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeHash {
    input_length: usize,
    processed: bool,
}

impl_type_identifiable!(ComputeHash);

impl ComputeHash {
    pub fn new(input_length: usize) -> Self {
        Self {
            input_length,
            processed: false,
        }
    }
}

impl Executable for ComputeHash {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.processed {
            return vec![];
        }

        // Pop length indicator
        stack.pop_front();

        // Collect input bytes
        let mut input_data = Vec::with_capacity(self.input_length);
        let bytes_to_read = self.input_length.div_ceil(32);

        for i in 0..bytes_to_read {
            let felt_bytes = stack.borrow_front();
            if i == bytes_to_read - 1 {
                // Last chunk might be partial
                let remaining = self.input_length - (i * 32);
                input_data.extend_from_slice(&felt_bytes[0..remaining.min(32)]);
            } else {
                input_data.extend_from_slice(felt_bytes);
            }
            stack.pop_front();
        }

        let hash_result = {
            use sha3::{Digest, Keccak256};
            let mut hasher = Keccak256::new();
            hasher.update(&input_data);
            hasher.finalize().to_vec()
        };

        // Push hash result (32 bytes)
        stack.push_front(&hash_result).unwrap();

        self.processed = true;
        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}

// Task for updating transcript with u64
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UpdateTranscriptU64 {
    processed: bool,
}

impl_type_identifiable!(UpdateTranscriptU64);

impl UpdateTranscriptU64 {
    pub fn new() -> Self {
        Self { processed: false }
    }
}

impl Default for UpdateTranscriptU64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for UpdateTranscriptU64 {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        if self.processed {
            return vec![];
        }

        // Get nonce bytes from stack
        let nonce_bytes = stack.borrow_front();
        let nonce = u64::from_be_bytes(nonce_bytes[0..8].try_into().unwrap());
        stack.pop_front();

        // Get transcript digest
        let digest = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        // Convert u64 to Felt and update transcript
        let nonce_felt = Felt::from(nonce);

        // Update digest: hash(digest + 1, nonce_felt)
        let digest_plus_one = digest + Felt::ONE;

        // Push values for poseidon_hash_many
        stack.push_front(&nonce_felt.to_bytes_be()).unwrap();
        stack.push_front(&digest_plus_one.to_bytes_be()).unwrap();
        stack.push_front(&2u32.to_bytes_be()).unwrap();

        self.processed = true;

        // Call PoseidonHashMany
        vec![PoseidonHashMany::new(2).to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}

// Constants
pub const MAGIC: u64 = 0x0123456789abcded;
