mod tests {
    use types::swiftness::{
        air::public_memory::PublicInput,
        stark::{
            config::StarkConfig,
            types::{StarkProof, StarkUnsentCommitment, StarkWitness},
        },
    };

    #[test]
    fn test_init_proof() {
        let config = StarkConfig::default();
        println!("config: {config:?}");
        let public_input = PublicInput::default();
        println!("public_input: {public_input:?}");
        let unsent_commitment = StarkUnsentCommitment::default();
        println!("unsent_commitment: {unsent_commitment:?}");
        let witness = StarkWitness::default();
        println!("witness: {witness:?}");
        let _proof = StarkProof {
            config,
            public_input,
            unsent_commitment,
            witness,
        };
    }
}
