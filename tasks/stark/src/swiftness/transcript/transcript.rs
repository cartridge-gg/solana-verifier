use crate::felt::Felt;

// This is NOT a task - it's a regular struct used for transcript operations
pub struct Transcript {
    digest: Felt,
    counter: Felt,
}

impl Transcript {
    pub fn new(digest: Felt) -> Self {
        Self {
            digest,
            counter: Felt::from(0),
        }
    }

    pub fn new_with_counter(digest: Felt, counter: Felt) -> Self {
        Self { digest, counter }
    }

    pub fn digest(&self) -> &Felt {
        &self.digest
    }

    pub fn counter(&self) -> &Felt {
        &self.counter
    }

    pub fn random_felt_to_prover(&mut self) -> Felt {
        // This function should be replaced with TranscriptRandomFelt task
        // For now, this is a placeholder implementation
        todo!("Use TranscriptRandomFelt task instead")
    }

    pub fn random_felts_to_prover(&mut self, mut len: Felt) -> Vec<Felt> {
        let mut res = Vec::new();
        while len > Felt::ZERO {
            res.push(self.random_felt_to_prover());
            len -= Felt::ONE
        }
        res
    }

    pub fn read_felt_from_prover(&mut self, _val: &Felt) {
        // This function should be replaced with TranscriptReadFelt task
        // For now, this is a placeholder implementation
        todo!("Use TranscriptReadFelt task instead")
    }

    pub fn read_felt_vector_from_prover(&mut self, _val: &[Felt]) {
        // This function should be replaced with TranscriptReadFeltVector task
        // For now, this is a placeholder implementation
        todo!("Use TranscriptReadFeltVector task instead")
    }

    pub fn read_uint64_from_prover(&mut self, val: u64) {
        self.read_felt_from_prover(&Felt::from(val))
    }
}
