use felt::Felt;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Config {
    pub height: Felt,
    pub n_verifier_friendly_commitment_layers: Felt,
}
