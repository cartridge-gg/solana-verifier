use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark_verify_fri::fri_verify::FriVerify;
use std::{mem::size_of, path::Path};
use types::swiftness::stark::types::cast_struct_to_slice;
use utils::{AccountCast, BidirectionalStack, Executable};
use verifier_4::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 1000;

/// Main entry point for the Solana program client
#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> client::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("client", log::LevelFilter::Trace)
        .init();

    let config = Config::parse_args();

    let client = initialize_client(&config).await?;

    let payer = setup_payer(&client, &config).await?;

    let program_path = Path::new("target/deploy/verifier_4.so");

    let program_id = setup_program(&client, &payer, &config, program_path).await?;

    println!("Using program ID: {program_id}");

    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<BidirectionalStackAccount>();
    println!("Account space: {space} bytes");

    let create_account_ix = create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        &program_id,
    );

    let create_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &stack_account],
        client.get_latest_blockhash().await?,
    );

    let signature = client
        .send_and_confirm_transaction(&create_account_tx)
        .await?;
    println!("Account created successfully: {signature}");

    println!("\nEvalOodsBoundaryPolyAtPoints Task on Solana");
    println!("=============================================");

    // Set up the account with minimal data like in unit test
    let stack_bytes = prepare_input::get_bytes();
    let mut instructions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        instructions.push(set_data_ix);
    }
    // Send transactions
    let mut transactions = Vec::new();
    for instruction in instructions.iter() {
        let set_proof_tx = Transaction::new_signed_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(set_proof_tx.clone());
    }
    send_and_confirm_transactions(&client, &transactions).await?;
    println!("Account data set successfully");
    // Push the task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(FriVerify::new().to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_task_ix],
    )
    .await?;
    println!("\nTask pushed: {signature}");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_200_000);

    // Execute all steps until task is complete - split into chunks of max 5000
    const MAX_CHUNK_SIZE: usize = 5000;

    let simulation_steps_usize = simulation_steps as usize;

    for chunk_start in (0..simulation_steps_usize).step_by(MAX_CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + MAX_CHUNK_SIZE, simulation_steps_usize);
        let chunk_size = chunk_end - chunk_start;

        println!(
            "Processing steps {}-{} ({} steps)",
            chunk_start,
            chunk_end - 1,
            chunk_size
        );

        let mut transactions = Vec::new();
        for i in chunk_start..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::Execute(i as u32),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            );
            let execute_tx = Transaction::new_signed_with_payer(
                &[limit_instructions.clone(), execute_ix],
                Some(&payer.pubkey()),
                &[&payer],
                client.get_latest_blockhash().await?,
            );
            transactions.push(execute_tx.clone());
        }

        send_and_confirm_transactions(&client, &transactions).await?;
        println!("Chunk {}-{} completed", chunk_start, chunk_end - 1);
    }

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);

    assert!(stack.is_empty_front(), "Stack should be empty");
    assert!(stack.is_empty_back(), "Stack should be empty");
    println!("All execution steps completed");
    println!("\nFriVerify successfully executed on Solana!");
    Ok(())
}

mod prepare_input {
    use felt::Felt;
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::swiftness::global_values::InteractionElements;
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use types::swiftness::stark::types::FriVerifyData;
    use types::swiftness::stark::types::StarkCommitment;
    use verifier_4::state::BidirectionalStackAccount;

    use types::funvec::FunVec;
    use types::swiftness::commitment::table::config::Config as TableCommitmentConfig;
    use types::swiftness::commitment::table::types::Commitment as TableCommitment;
    use types::swiftness::commitment::vector::config::Config as VectorCommitmentConfig;
    use types::swiftness::commitment::vector::types::Commitment as VectorCommitment;
    use types::swiftness::fri::config::Config as FriConfig;
    use types::swiftness::fri::types::Commitment as FriCommitment;
    use utils::CacheStorage;

    pub fn get_bytes() -> Vec<u8> {
        let mut stack = BidirectionalStackAccount::default();

        // Load proof like in unit test
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();
        stack.proof = proof_verifier.clone();

        let mut stark_commitment: StarkCommitment<InteractionElements> = StarkCommitment::default();

        stark_commitment.fri = get();
        stack.stark_commitment = stark_commitment;

        // Użyj nowej metody do przechowania FriVerifyData w cache
        let mut fri_verify_data = FriVerifyData::default();
        let fri_decommitment = get_decommitment();
        fri_verify_data.fri_decommitment = fri_decommitment;
        let queries = vec![
            "0xd20990",
            "0x1702a2dc",
            "0x233bfb24",
            "0x2fc8f32e",
            "0x367bcdcb",
            "0x44445cc6",
            "0x4bf4ed93",
            "0x8df252ca",
            "0x97a48b5b",
            "0xafea6443",
            "0xc62f63b8",
            "0xd76e5257",
            "0xecca885b",
            "0xedc42f8b",
            "0xf6821efe",
            "0xf7769c26",
        ];
        let queries = queries
            .iter()
            .map(|f| Felt::from_hex_unchecked(f))
            .collect::<Vec<Felt>>();

        fri_verify_data.queries = FunVec::from_vec(queries);
        stack.store_in_cache(&fri_verify_data);
        println!("FriVerifyData stored in cache");

        let bytes = cast_struct_to_slice_mut(&mut stack).to_vec();
        bytes
    }

    pub fn get() -> FriCommitment {
        FriCommitment {
            config: FriConfig {
                log_input_size: Felt::from_hex_unchecked("0x20"),
                n_layers: Felt::from_hex_unchecked("0x9"),
                inner_layers: FunVec::from_vec(vec![
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x1d"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x1a"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x17"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x14"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x11"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xe"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x4"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xc"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x4"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xa"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                ]),
                fri_step_sizes: FunVec::from_vec(vec![
                    Felt::from_hex_unchecked("0x0"),
                    Felt::from_hex_unchecked("0x3"),
                    Felt::from_hex_unchecked("0x3"),
                    Felt::from_hex_unchecked("0x3"),
                    Felt::from_hex_unchecked("0x3"),
                    Felt::from_hex_unchecked("0x3"),
                    Felt::from_hex_unchecked("0x3"),
                    Felt::from_hex_unchecked("0x2"),
                    Felt::from_hex_unchecked("0x2"),
                ]),
                log_last_layer_degree_bound: Felt::from_hex_unchecked("0x6"),
            },
            inner_layers: FunVec::from_vec(vec![
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x1d"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x1d"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x31bff9de415b246e26441df6d7ededb680bfedd63ab962377cd678d848c45ba",
                        ),
                    },
                },
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x1a"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x1a"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x60a16437f82f2551257c0c6b8b4a0bc47195df671010b4c7a953a97e5ff057",
                        ),
                    },
                },
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x17"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x17"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x792506d84d73eaa2cb70f6dcd2bdac7d7e369874396487a1438e2afc10bd1a2",
                        ),
                    },
                },
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x14"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x14"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x5166a06143bb6b6b38f6bb042b958a8e730cca90b0f2a62eeec13da9d7e5130",
                        ),
                    },
                },
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x11"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0x11"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x7a8ca0a6032fafd9634eaae474451268e01d232352f11bb08e5a98d110503b0",
                        ),
                    },
                },
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x8"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xe"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xe"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x57f01089ae4521585eb840bd8fa8a2dbdc4ae90e633f2f345c54cb52357676b",
                        ),
                    },
                },
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x4"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xc"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xc"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x7f566ad90570ed8e70b122f4ecf4c0a26b73d9e7cfcaa31f34f21bc4d4d8473",
                        ),
                    },
                },
                TableCommitment {
                    config: TableCommitmentConfig {
                        n_columns: Felt::from_hex_unchecked("0x4"),
                        vector: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xa"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                    },
                    vector_commitment: VectorCommitment {
                        config: VectorCommitmentConfig {
                            height: Felt::from_hex_unchecked("0xa"),
                            n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x17"),
                        },
                        commitment_hash: Felt::from_hex_unchecked(
                            "0x66cf0196b784195c9036dfafc33cbf1a3140781d1b141d4af4e6e1037e1e378",
                        ),
                    },
                },
            ]),
            eval_points: FunVec::from_vec(vec![
                "0x496c968f1dac9c4e65716021ecbad75f43995e95232b76f9511f1c2bdb125e1",
                "0x21405f4b938b8e96735126fe129009b6875fa3836b43caeae53f1c2868fe2ea",
                "0x6beae26510027e130e3e2eaddf855ce622cbfee5f4fda948ac8d7994f82a5fe",
                "0x5c0bf9884530fdbc71613efffae4e91c5c1d8d5ea611ef467e04ee209fe4319",
                "0x5cef02aa4aab094f92cc250b7a1463cd43fa7c242657a205fb169a19c099805",
                "0x468cb430f752e4abad5504b5ecaee3b5e6ddbcea288dea763c7bd24cf15b245",
                "0x163dce928440e6d53639e3f5361d28fa1141da0939a32a3f4891655fc4658df",
                "0x63bf61faefd29520a1c37bfa4fefbf615a8e54f48fdda77bfed480a5a5a6eff",
            ]
            .iter()
            .map(|f| Felt::from_hex_unchecked(f))
            .collect()),
            last_layer_coefficients: FunVec::from_vec(vec![
                "0x66c796d3d02b79f1651070cb45f0bf66555e52586bde97db07d3587acebcb1e",
                "0x5a65f0a67b296d6fde75095e7bf9bb15147cdf46dac056a3515f3211c755a84",
                "0x2a86628c832b25e8f7c66db9cd8e75acb17c032f73184e794b314d5b6768f16",
                "0x18f8e25f5b8a67d4a815542cfc1af6798f1e6ceca9476fb1e116e88c3e44d90",
                "0x78b507b81e1b5c348589e3d1df85c2dd43a522949fd5aef31e37365e230c234",
                "0x24fa994cd55659e2d7f0112b1cee5c7321d0a47443422da51b4e607eba4b36c",
                "0x4f8df7df167ee1dec27c283cb96c4c9ff85c62416f61e197fa517dfa78cde51",
                "0x3a061e33f6af1045e247b390d5b71b8c3cd74d7936d85e1dab7ec1079b1f723",
                "0x1fd53dff2886b9d6dd8020d63f8e07063fa051a1fead92975ac95887f62296f",
                "0x6c2172d15248be94f9b5fb457a164ec6db6ed63fb84c0ab154f05a45ae1a6b0",
                "0x3e18d61a269faa393ccd03bb2b64364065b7af2f529d83bbe13bad4d819fe3",
                "0x4741100133289117afd8d7f75191b8cdb60674b5e46fe337c590c1fe9d589e1",
                "0x4ce865332e2742b7294844916d0a5592adac0f2e574f7c726dd9ba0d16166",
                "0x521b6743607208f1e573da338e0528176563d60b458262ebd3bea3493140ef7",
                "0x24f45c1ebcfbf13c772f32b5aa48dbb0ca8abdaeb8338ac5e302b5674f4f7b3",
                "0x19ff856bb023150b68993d3c59598f209084c2b05d1a49b9bf0ffdd58319612",
                "0x5837d69ead25595f8729bd17a0723bd410878ecb7b0d16ac0a2f96019761a41",
                "0x1ca31d4d9b77c07de0be1cab20657388978365f24ff58254928f319419be580",
                "0x668e682b2116ff3e6ced4e554c0787781113a49a249876a00396bd06c356f88",
                "0x617863095a0f5af592ea321e1e3d26739aebef5ad2fda50cc0e5fc3e3a399bb",
                "0x2924f13b449fec4aa26b53e38ee1b422c17cfb1de4fe8f875f696d8e313e142",
                "0x6a685173638b6009f043aee321a4d71c39a6b77793edc1c08e48b672254d8ad",
                "0x6118f485eb29bbe3d5ae3e4834295f35a3cabbbee8e70ae3ff245f8ef9a5598",
                "0xaef070e0b90f861871b1f3f499439b517718fe8d43bace6d55a90e69f154f7",
                "0x57aadf081d14036422f6112b638b6a62218d75940542424624a3ed87b8c8d70",
                "0x43817969bea69628097471048d3342b2ce2df90e83cd0705a4bbbdc4ad9a26f",
                "0x19fadb1a38f3bf87a67758ce80ca095ed279bbf13627e173c395a52e4a0beec",
                "0x26717f7a71785c0442416461ecf9b8692d04b2c762e2bbde8761606e5088231",
                "0x652c1e32e09ba9fddbb602bf2c89d0a09aecb2aa6153b55b1ea73da7434d43e",
                "0x7cfde088e2b31a36da73203d84afb491729d25075e10e4a43f1eea3cfddff95",
                "0x2cd6701557574e1609c88e1a897851afd384cad0562d506422b89229fa2571b",
                "0x1ef21bfa917b63aad98765bb9eab0b62d893c7da9e930a558ce2803c30a6965",
                "0x737175bfaad9bec33e9b180d1e2f378d895b521e76dc0f2f2df9d274ad8278c",
                "0x4fc88bea560a9c57e9dd0acd39f1454b38b768019f053d6e34a5c9c29f7154a",
                "0x5a0a6e245e1bef2345106ca153caa41cc45569a2c2e763a3b714ca0e0fe948f",
                "0x660d7d653c5f457ad83402f824d5ed801c9bf31ce19ea7c1fce003bacdbbbb9",
                "0x46c77c3d74a2c3053f7cf212eebb5267414d9b79d13cbdc4bae750d1ef18855",
                "0x716128901707f48b66a45f6486e820bf1d8197febe9e54d510c740786317da0",
                "0x5813d095b76f9696142adce747f1092da9d7f4b4c78ec4f80f4677acbe0895f",
                "0x23805037d788d418797cc464397b2fde00842c21eb5766fcaab215e212432f0",
                "0x12525b04e5a921bee8888521e61a9560be03195655074e9022b09a25e543d1f",
                "0x4c1c2994d16395a519f7477bde0052b8f2dcfa521fc8ba6de837a8f23711b47",
                "0x2cf2d39ee47800e9d7f4fb662a8ec9ee4510dcb114a4ab6b5fcd9188f0bb0d9",
                "0x25601c3d21d3768ed8e588f83428196799e75b6ebbb85ca8886549ad4258963",
                "0x5d594bf05fe3020e30c58b949bfed2f14e946bb6564a6f7f7003f91fcbfe1c9",
                "0x12736ff70283026b7a4e279f492ffa1f0b6433209e96b439d1728fed4429c26",
                "0x9e4ef6f319e6d61c5ada1c0c01b85f705da3251e9c0038791995a1b4a9672b",
                "0x34d0796fed079fe2eee157f30bda10630d34158bb45aa56ac88427fe70706b9",
                "0x44e64282f5f87a93472b1ebf9d2a63e389708640ec1c2480c643c6aba386fd5",
                "0x3f1fb9576bf9060f5c3c197018e4b4229a5b1427da821b1d23b509e16d28376",
                "0x6fe2b5886bdfd06eb1a2a33e99dda8229d6d11d9df2815d7dcfe53230ea42aa",
                "0x6de15d80bb2106afdff4e63f268e38cf0c75c7a188ea249987eac7da7cf9e75",
                "0xce3f50c606621b881811f32242dd76e6f855601e7fa7a307d3bdae78fc7709",
                "0x7a8ca41b50fce78b56de444fb90ad2a1c5b021a5e65f1535ad3e2bfd82ac35e",
                "0x1812ea75c5b6bd574a0bf536f6bed6edaa6148e785b9615c6bfa9ce105c2996",
                "0x17e9f49792461fb9185124566661cd1977d8ac8715b468840e09dd4aec18994",
                "0x113229d548d7a1169f3a863d39f5f49b0d62268c57eb22de14c0fec227e22d9",
                "0x3a7cbb5ffbdfda6ec423c778beb40e092a8158713a7fbc1349edd835d7205f9",
                "0x37429f0c3c16caa393a37cf022d8026563e550682a70bd7cfb74f0eb0fbe641",
                "0x779f6680f64e3a5d2ab847b788b28bc29da9dbc90d2fd9a779e8712b07cc153",
                "0x2f6fc641bb2fda367785f91ce33398d61b804b5454aba1cf1d0a74121b84c15",
                "0x3e7fcd7510327a6e70fc72020c1214cbe30a9331a44c7ddddded98cca785708",
                "0x3157be835d92a4a5a0b4b46d6f11bf800c0fd1920454d58402612417bee11a8",
                "0x2077c8e77e96c8db5212cf46c32546f1bd9a3e97c63aebccacc1438ffcc9aa7",
            ]
            .iter()
            .map(|f| Felt::from_hex_unchecked(f))
            .collect()),
        }
    }
    use types::swiftness::commitment::types::Decommitment;

    pub fn get_decommitment() -> Decommitment {
        Decommitment {
            values: FunVec::from_vec(get_values()),
            points: FunVec::from_vec(get_points()),
        }
    }

    pub fn get_values() -> Vec<Felt> {
        vec![
            "0x56589147f36eee3f7976a1542599dd32be46d202f4ec49dccef821f43ade30f",
            "0x6da23461f6dc6aac5624da021558eaea6f8039c59a3a1596694aaade6ae5aea",
            "0x7c2cb3f9065f1c08480be0521698325689a3346e6fd358e65d98f43ef91848e",
            "0x7272da9be8a83b5007e3b63487265431b894626aabe48070e87412a33f06e21",
            "0x48b12d9655668770fbb57fa2aaa241df1aff1195a68c44ea912563e633c0311",
            "0x5613f5cb362f21af6a28237858c8e25930ee6d1f03d615991862c966b696b07",
            "0x1daf84477265f19fbcbb8fa7b62d85a14221de9add62996cb6a1eba477532c",
            "0x255f150abc9f168bbf353a77445b26a0c4c3243be19985398cef35916b39349",
            "0x3d99e7912b03d046b302ba451fd39d4a2f22173c5d3facd40eaf8e4ca160729",
            "0x3931a734c9e17b5d11721226625ce4d8c2ce416cd05168442c636717b8f2b7c",
            "0x501483805f53ae20ff3317425627bab5a8a31487ce9e62bf09f2ad591d4d636",
            "0x55bf2ccb8e98ecd75c23c941d8201b3ff3cce32f4c2fedeea787307cd42f275",
            "0x2872e8b5f38ac80c1db5cd85801c20696a1480e7a35d532a8d06d51428d7417",
            "0x2217dfcf29dd655b6a85d1769e7cf444ecefa2cd276e1c6de73d5d039c6cf8e",
            "0x1558aa1be37c22f07b2b0422b37a5f67ef6285c8a33a94f7d46347bfc64b9e2",
            "0x43bbcf9a0483a1f8e74570452b870ef248e4d5aa227bf64910c0c92d0afa598",
        ]
        .iter()
        .map(|s| Felt::from_hex_unchecked(s))
        .collect()
    }

    pub fn get_points() -> Vec<Felt> {
        vec![
            "0x19def6309c27c3fa7844c5dcf97482dfb990623fffa356c0b6aa93a84840728",
            "0x492280f95460c8f9db2fecc27ee0a783fcf1deab4f327511844f9bb42425cf6",
            "0x71563605a5b60d9422cadbcfec42ad8e9c0852480122970c88133a7cbd8f56b",
            "0x3af83aef91f27a7940b894ae7ca082a482078c31a322a39b76b4f5b1c44b6e1",
            "0x5e4dfa204eab845ffa6b00b011a3745fd71106364d948a4fb048752c7bf954d",
            "0x5c0bdca0f6180c2b3cfca224a853cb9504c16b0a16f1025be8746e54335cf01",
            "0x7c1fbdcf0da9f44c6ee49a7cc2da7bfb5aae7fe8405a3fd42105c0a9d864a36",
            "0x587e32ddf511d3dd04193d0af898e18e80cae410ba411400e6185c162635419",
            "0xe1314b65854a3e4a87ffd44299dfa1fd5ec35c83cedad436204e7a12c8bd13",
            "0x22bd9975e69ab780c1bd874c99fb102d337e90d3a905eac19ce54c5d1b6bbd1",
            "0x67c3e65dd1624c47dce264322e2e6b2797d096fa76248f11e2182fe9a99f5f2",
            "0x38958ba48451e0157ffab3225716567beac30b44df4db2a251e743cbb93af49",
            "0x3bc1a9f0df58b8c03d1535e3b02c4b4a646ef22b21ef6d47241e7f781e57ce0",
            "0x77e2e9cca0a2415553be66e6ebd9393570c3ecf3426546e6944e74774010e03",
            "0x3561aa6ed23bb17fac27de9a4e314d768f5ea05a033bbcb1de2cff9ae90ab6",
            "0x7f90255cc310f54635400a0fc3ad5d4dcd9afb685485297d828f04cb9c29fcb",
        ]
        .iter()
        .map(|s| Felt::from_hex_unchecked(s))
        .collect()
    }
}
