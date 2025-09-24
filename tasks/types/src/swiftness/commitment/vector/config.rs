use felt::Felt;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
#[repr(C)]
pub struct Config {
    pub height: Felt,
    pub n_verifier_friendly_commitment_layers: Felt,
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
#[repr(C)]
pub struct VectorConfigBytes {
    pub height: [u8; 32],
    pub n_verifier_friendly_commitment_layers: [u8; 32],
}

impl Config {
    pub fn new(height: Felt, n_verifier_friendly_commitment_layers: Felt) -> Self {
        Self {
            height,
            n_verifier_friendly_commitment_layers,
        }
    }
}
