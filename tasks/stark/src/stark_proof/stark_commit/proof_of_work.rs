use crate::swiftness::stark::types::StarkProof;
use crate::swiftness::transcript::TranscriptReadFelt;
use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

// Constants
pub const MAGIC: u64 = 0x0123456789abcded;

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
    digest: Felt,
}

impl_type_identifiable!(ProofOfWork);

impl ProofOfWork {
    pub fn new() -> Self {
        Self {
            step: ProofOfWorkStep::PrepareInitialHash,
            n_bits: 0,
            nonce: 0,
            digest: Felt::ZERO,
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
                let digest_bytes: [u8; 32] = stack.borrow_front().try_into().unwrap();
                stack.pop_front();
                self.digest = Felt::from_bytes_be_slice(&digest_bytes);

                stack.push_front(&[self.n_bits]).unwrap();
                stack.push_front(&digest_bytes).unwrap();
                stack.push_front(&MAGIC.to_be_bytes()).unwrap();

                self.step = ProofOfWorkStep::ComputeInitialHash;

                vec![ComputeHash::new(41).to_vec_with_type_tag()]
            }

            ProofOfWorkStep::ComputeInitialHash => {
                // Prepare data for final hash: init_hash || nonce
                let init_hash: [u8; 32] = stack.borrow_front().try_into().unwrap();
                stack.pop_front();
                println!("init_hash: {:?}", init_hash);
                let nonce_bytes = self.nonce.to_be_bytes();
                println!("nonce_bytes: {:?}", nonce_bytes);

                stack.push_front(&self.nonce.to_be_bytes()).unwrap();
                stack.push_front(&init_hash).unwrap();

                self.step = ProofOfWorkStep::ComputeFinalHash;

                vec![ComputeHash::new(40).to_vec_with_type_tag()]
            }

            ProofOfWorkStep::ComputeFinalHash => {
                // Final hash is now on stack
                self.step = ProofOfWorkStep::VerifyWork;
                vec![]
            }

            ProofOfWorkStep::VerifyWork => {
                let final_hash: [u8; 32] = stack.borrow_front().try_into().unwrap();
                stack.pop_front();
                println!("final_hash: {:?}", final_hash);

                // Check first 16 bytes (128 bits)
                let work_value = Felt::from_bytes_be_slice(&final_hash[0..16]);
                println!("work_value: {:?}", work_value);
                let threshold = Felt::TWO.pow(128 - self.n_bits);
                println!("n_bits: {}, threshold: {}", self.n_bits, threshold);
                println!("work_value < threshold: {}", work_value < threshold);

                assert!(work_value < threshold, "Proof of work verification failed");
                self.step = ProofOfWorkStep::UpdateTranscript;
                vec![]
            }

            ProofOfWorkStep::UpdateTranscript => {
                stack.push_front(&self.digest.to_bytes_be()).unwrap();
                // Update transcript with nonce
                stack.push_front(&self.nonce.to_be_bytes()).unwrap();
                self.step = ProofOfWorkStep::Done;

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
        // Collect all input bytes directly from stack
        let mut input_data = Vec::new();

        // Read all data from stack until we have the required length
        while input_data.len() < self.input_length {
            let chunk = stack.borrow_front();
            let remaining = self.input_length - input_data.len();
            let to_read = remaining.min(chunk.len());
            input_data.extend_from_slice(&chunk[0..to_read]);
            stack.pop_front();
        }

        println!(
            "ComputeHash: input_length={}, actual_read={}",
            self.input_length,
            input_data.len()
        );
        println!(
            "ComputeHash: input_data: {:?}",
            &input_data[0..input_data.len().min(16)]
        );

        println!("input_data: {:?}", input_data);
        let hash_result = {
            use sha3::{Digest, Keccak256};
            let mut hasher = Keccak256::new();
            hasher.update(&input_data);
            hasher.finalize().to_vec()
        };
        println!("hash_result: {:?}", &hash_result);

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
        // Get nonce bytes from stack
        let nonce_bytes: [u8; 8] = stack.borrow_front().try_into().unwrap();
        let nonce = u64::from_be_bytes(nonce_bytes);
        stack.pop_front();

        // Get transcript digest
        let digest = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();

        // Convert u64 to Felt and update transcript
        let nonce_felt = Felt::from(nonce);

        TranscriptReadFelt::push_input(digest, nonce_felt, stack);

        self.processed = true;
        vec![TranscriptReadFelt::new().to_vec_with_type_tag()]
    }

    fn is_finished(&mut self) -> bool {
        self.processed
    }
}
