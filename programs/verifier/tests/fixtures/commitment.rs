use super::{fri_config, fri_unsent_commitment, interaction_elements};
use crate::fixtures::config;
use crate::fixtures::unsent_commitment;
use felt::Felt;
use stark::swiftness::air::trace::Commitment as TraceCommitment;
use stark::swiftness::commitment::types::Commitment as FriCommitment;
use stark::swiftness::commitment::{
    table::{config::Config as TableCommitmentConfig, types::Commitment as TableCommitment},
    vector::{config::Config as VectorCommitmentConfig, types::Commitment as VectorCommitment},
};
use stark::swiftness::fri::types::Commitment;
use utils::global_values::InteractionElements;

pub fn get() -> TraceCommitment<InteractionElements> {
    let unsent_commitment = unsent_commitment::get();
    let traces_config = config::get();

    TraceCommitment {
        original: TableCommitment {
            config: traces_config.original,
            vector_commitment: VectorCommitment {
                config: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: unsent_commitment.original,
            },
        },
        interaction_elements: interaction_elements::get(),
        interaction: TableCommitment {
            config: traces_config.interaction,
            vector_commitment: VectorCommitment {
                config: VectorCommitmentConfig {
                    height: Felt::from_hex_unchecked("0x14"),
                    n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                },
                commitment_hash: unsent_commitment.interaction,
            },
        },
    }
}
