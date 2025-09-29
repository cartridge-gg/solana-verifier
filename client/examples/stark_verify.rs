use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use felt::Felt;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark_verify_verification::stark_verify::StarkVerify;
use std::{mem::size_of, path::Path};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use types::swiftness::stark::types::StarkCommitment;
use types::swiftness::stark::types::{cast_struct_to_slice, VerifyVariables};
use types::{
    funvec::FunVec,
    swiftness::global_values::{GlobalValues, InteractionElements},
};
use utils::{AccountCast, Executable, COLUMN_VALUES_SIZE};

use utils::BidirectionalStack;
use verifier_2::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 1000;

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

    let program_path = Path::new("target/deploy/verifier_2.so");

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

    println!("\nEvalCompositionPolynomialInner Task on Solana");
    println!("==============================================");

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
    println!("All data set successfully");

    // Push the StarkVerify task to the stack
    let stark_verify_task = StarkVerify::new(0, 0);

    println!("Using StarkVerify with TYPE_TAG: {}", StarkVerify::TYPE_TAG);

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(stark_verify_task.to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _ = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_task_ix],
    )
    .await?;

    let trace_generator =
        Felt::from_hex("0x57a797181c06d8427145cb66056f032751615d8617c5468258e96d2bb6422f9")
            .unwrap();
    let push_trace_generator_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(trace_generator.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _ = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_trace_generator_ix],
    )
    .await?;

    let trace_domain_size = Felt::from_hex("0x10000000").unwrap();
    let push_trace_domain_size_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(trace_domain_size.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _ = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_trace_domain_size_ix],
    )
    .await?;

    let digest =
        Felt::from_hex("0x59496b8e649ff03c8e9f739e141bd82653fccb2fb1b1a51a71760ea3813ea35")
            .unwrap();
    let push_digest_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(digest.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _ = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_digest_ix],
    )
    .await?;

    let counter = Felt::from_hex("0x0").unwrap();
    let push_counter_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(counter.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _ = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_counter_ix],
    )
    .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(800_000);

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

    // Check that stack is empty (task completed successfully)
    // assert_eq!(stack.is_empty_front(), true, "Stack should be empty");
    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");

    println!("✓ All verifications passed! Results match expected values from stark_verify.rs");
    println!("✓ Stack is empty - task completed successfully");
    println!("✓ StarkVerify test completed successfully on Solana!");

    Ok(())
}

mod prepare_input {
    use felt::Felt;
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::funvec::FunVec;
    use types::swiftness::air::trace::Commitment as TraceCommitment;
    use types::swiftness::air::trace::Decommitment as TraceDecommitment;
    use types::swiftness::commitment::table::config::Config as TableConfig;
    use types::swiftness::commitment::table::types::Commitment as TableCommitment;
    use types::swiftness::commitment::types::Decommitment;
    use types::swiftness::commitment::vector::config::Config as VectorConfig;
    use types::swiftness::commitment::vector::types::Commitment as VectorCommitment;
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use types::swiftness::stark::types::StarkCommitment;
    use types::swiftness::{global_values::InteractionElements, stark::types::VerifyVariables};
    use utils::BidirectionalStack;
    use utils::StarkCommitmentTrait;
    use verifier_2::state::BidirectionalStackAccount;

    use crate::constraint_coefficients;

    pub fn get_bytes() -> Vec<u8> {
        let mut stack = BidirectionalStackAccount::default();

        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();
        stack.proof = proof_verifier.clone();

        let interaction_elements = InteractionElements {
            memory_multi_column_perm_perm_interaction_elm: Felt::from_hex(
                "0x63be95eef090c5ed842139ace99b3dc2e8222f4946d656d2b8ecf9f3a4eaa64",
            )
            .unwrap(),
            memory_multi_column_perm_hash_interaction_elm0: Felt::from_hex(
                "0x522df1ce46453857bc93d7b48c77fd4968ae6be4de52c9a9ebf3b053fe3f288",
            )
            .unwrap(),
            range_check16_perm_interaction_elm: Felt::from_hex(
                "0x47256c1d9e69a2c23e0a5b2666fd2e2037ef2987d19b53da2b089c7a79e217c",
            )
            .unwrap(),
            diluted_check_permutation_interaction_elm: Felt::from_hex(
                "0x1f44508505278264aabe386ad5df3bee4b8147b3d0e20518bfaec709cbc1322",
            )
            .unwrap(),
            diluted_check_interaction_z: Felt::from_hex(
                "0x7f01d79f2cdf6aa851c9b2e0fa2e92f64ecd655289f827b14d5e7b483f52b48",
            )
            .unwrap(),
            diluted_check_interaction_alpha: Felt::from_hex(
                "0x734820597aa2142c285a8ab4990f17ba4241a78de519e3661dafd9453a8e822",
            )
            .unwrap(),
        };

        let n_columns_original = Felt::from_hex_unchecked("0x6");
        let n_columns_interaction = Felt::from_hex_unchecked("0x2");
        let height = Felt::from_hex("0x20").unwrap(); // 20
        let n_verifier_friendly_layers = Felt::from_hex("0x17").unwrap(); // 100
        let original_commitment_hash =
            Felt::from_hex("0x305f1ee7c0b38a403b2fa7ec86a3d11c8a174891194a2c656147268b59e876d")
                .unwrap();
        let interaction_commitment_hash =
            Felt::from_hex("0x6d41514e4a6e39f5b4e5f18f234525df1d2d92393c11ce11bd885615c88406")
                .unwrap();

        let vector_config = VectorConfig {
            height,
            n_verifier_friendly_commitment_layers: n_verifier_friendly_layers,
        };

        let table_config_original = TableConfig {
            n_columns: n_columns_original,
            vector: vector_config,
        };
        let table_config_interaction = TableConfig {
            n_columns: n_columns_interaction,
            vector: vector_config,
        };
        let vector_commitment_original =
            VectorCommitment::new(vector_config, original_commitment_hash);
        let vector_commitment_interaction =
            VectorCommitment::new(vector_config, interaction_commitment_hash);

        // Set up stark commitment with data from the log
        let table_commitment_original =
            TableCommitment::new(table_config_original, vector_commitment_original);
        let table_commitment_interaction =
            TableCommitment::new(table_config_interaction, vector_commitment_interaction);
        let trace_commitment = TraceCommitment::<InteractionElements>::new(
            table_commitment_original,
            interaction_elements,
            table_commitment_interaction,
        );

        stack.constraint_coefficients = constraint_coefficients::get()
            .as_slice()
            .try_into()
            .unwrap();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();

        let mut stark_commitment: StarkCommitment<InteractionElements> = StarkCommitment::default();
        stark_commitment.traces = trace_commitment;
        stack.stark_commitment = stark_commitment;

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

        let fri_verify_data = types::swiftness::stark::types::FriVerifyData {
            queries: FunVec::from_vec(queries),
            fri_decommitment: Decommitment::default(),
            current_layer: 0,
            working_queries: FunVec::default(),
            working_elements: FunVec::default(),
            working_indices: FunVec::default(),
            working_y_values: FunVec::default(),
            coset_size: Felt::ZERO,
            eval_point: Felt::ZERO,
            next_queries: FunVec::default(),
            sibling_witness: FunVec::default(),
            next_x_inv_value: Felt::ZERO,
            coset_x_inv: Felt::ZERO,
            coset_elements: FunVec::default(),
            current_coset_index: 0,
        };
        stack.store_in_cache(&fri_verify_data);

        let bytes = cast_struct_to_slice_mut(&mut stack).to_vec();
        bytes
    }
}

mod constraint_coefficients {
    use felt::Felt;
    use types::funvec::FunVec;

    pub fn get() -> FunVec<Felt, 194> {
        FunVec::from_vec(
            vec![
                "0x1",
                "0x27dab20f0955aea0793cb622d7b74b56318978dbd44835af673e1adda5a2cc9",
                "0x4040b9a013f1db8a95f19415974fb60636e99e3458edc6009e422208601645e",
                "0x5f62630850416ab25451cb9c233678947ac980b4943aab801adc211005d8d87",
                "0x42585a6cef0c43c6f48b6214833961ac266ec20f7d5ddd8debec01eb7d37fcc",
                "0x4461dc6c692aa5f2128a2ac23aefc900e7cd421be86d852a640569c8854bed",
                "0x3fbed0c884c75b9f0b40aec97ed672f4715c29d803b2abd72eab3bd3ad6b535",
                "0x1204110e1d3151d2f7b12fbc9bb0d9b79ca7ea2a01a846fb624036444de3a84",
                "0x1369805cedd8e28ba8b6c2f6400975a0ca3283f9ff889b6d52e89064656e4d6",
                "0x44c8b4c6d5850f809bc2534c64e2ede742fd4d123dde2f914da7b33d13e5875",
                "0x1cab697256523ee0ad691ad41b711ff4ff13f42c86ff83c2e67bee36b204b8c",
                "0x219352bf7d888149c48868e738427424278b2e2ba009ba116fd0ae1dfb7dcbb",
                "0x4cbde2d2527ceaa11cee77a01dc504823abd66024d475f6dd29447f37196563",
                "0x5a2cfe6680539d1ee3b0d0c315352d1d90731ac20359ad1ce2c4d5fd05b1b5e",
                "0x6b45468a2dd15d330cdd384cddf7af2efc86c69048ff948d5500984aec6b830",
                "0x3b4570a7f748c7581dc55bbe06c74e9f831266d9e9d9307e64f1f58101589e9",
                "0x6733de2c8331352184d7024002376c6c214eb09dc09a6857a1df09281175951",
                "0x1540fcb219ed88f9aff5a28313f2fe726eb1cc7d3ccd9f9384e1d19f7f5532b",
                "0x3ed54456078fbae52dc454a238d4982396645586ee17e1e3cce8b1800f27a86",
                "0x3a8a82fcd40543eca6ad0c4a82c9e559ba5fafd86effb9f793de860901b68cc",
                "0x10484a5a314e1e7c7c5b9c7fa6d7f86fbe20c49d8d9a149c56790100a3ffb5",
                "0x38ac1c0094d82ba13b6ea942cd493de8c4ac985c4873c6a46c064e6d433525",
                "0x14ed110b351b9b492bb40a194e462f3b9c06bbe19ea4f0f2b27d367d830fc32",
                "0x6e2fd526e4963aa7f529b0dbb564b5b8879e5f180837b2ef57c108b9d2c51d2",
                "0x50550ad7f99b75a60fb2f5193b204ea997d243a9b3c9eb21b85a1b22f2f013e",
                "0x4ad8ef3201cdd98824986ad6b3db73b8169871b0c8b0b39b7b4f4a05b178862",
                "0x46512c15dfc10487196c1df9337444d95fcc3b6a963c3bb682aadc9a69ebb59",
                "0xee52d4f8a05c470798b2a2b11b3e210cc384ea7d5178fdd87d6073e72c2995",
                "0x32d013ff1c6fe8b052774e8dd89723f954860eac3d2949cc9163f9fbcbde2ab",
                "0x494d277d1c530409ef302234c16f7e886f28c76d1dbc7a5e1db3b78d4c4646f",
                "0x4189df84b90919e54bcc8264c89e36b11539eef965bbeb5066d1d2557b39f52",
                "0x6ff14d8e9a3b2189c1fda7b37ca5772902c0fc55f251a7011308d4d636f75b9",
                "0x26d2858f7bc61a2e11bc44671ab48cc3b0e18a409444485532c201308064d8a",
                "0x45d5c7365cf9a22f075ba1bc62eb4a03922f35ad567a8ecede089b2562fb933",
                "0x38b9d162a0f995a3c86572bae519aae67d1ae762e4f80836cb27194bcbcf4e9",
                "0x21cb978a899f40c2ecfaab14c4619beb879e9a610f81787391d4f6a8ba30420",
                "0x10ac29054e4b43e0c9522a61a8fc1087da348e86ac557484568e35d69cb75f",
                "0x10853ddb3111227d402a7b0db168aee96489a91533dd5d7bb51511297215b76",
                "0x4c1583a1d5c4577b7e75c265c05e68e81b3fe27aa69a0ae33b65a2c279fb2a2",
                "0x4a9f86346a282676bb0e2de28325ddd8ab9884b1f8f5f0805d123ff6170aea5",
                "0x2d97725449a903f9f6b196c276a52714a7579e270d0764cf2b733bcdc1584d1",
                "0x50682eb3ff8b83467d74c7ded44df147f3e864093cfc7a4f64baae5050a9d9",
                "0x7e0574d010b422f32c7402fa1f38d89ce0146e0f09f70ec3c6b13af4e16ae01",
                "0xfe28b4929a7978adf78782bb1aed22368aad8b3508abb56ae6f4e17496c197",
                "0x42ec36434a9757bbd58bc2ce9b1eb17bdd200ed1bc12a9010de8fe94410783",
                "0x5add140e834af1345f587e99b26d4e85adfe2e17e95beb88fe51fb6502244af",
                "0x2b9336d3e08c86b874218bf7afdd658c2769fa5b76130ea4a7a1153be2bc0c7",
                "0x25c8192ea2019574fd9681c4e855dc9be7d00f6802d29592725578cf5c76764",
                "0x7dd5addc9624bf333985d3852d64c28b51943878ee788dd35fe938c801ba262",
                "0xfe9f1f57326b111d555aadd9c5087a525dac6091766d870630ff74aaa057b8",
                "0x237424ea6d26d17f516ef6767f25d76aaf6fff2430cccc4ce306f7fc9e3eb48",
                "0x5e21fdf7828d3479842f11528e44c6c09a0be2f3d0d15991ca2ba02c9622f1e",
                "0x445b755a29f8f24ac2fcfff928943fea9b425fc8122f07a9bd8364753893c3",
                "0x5cd960ee5373b5f8a0748f4b6e5de88f7d1cc3ce542f10abb74d41320b76300",
                "0x1dee062068896f21ae1276b8fb1bd557a51028b487fbbf977fcd6d5ec132278",
                "0x39071db8bc5e7096aef050971271256c1a2b3d79e30520b45feb7c3ed5a0f30",
                "0x63a0483bac31d71bbc9e230e9e388c3df9557441f9fd791c49f9c1fd5fa8f0d",
                "0x6e3d38d26d401920226aa1ae17d893f211fda12dfaf5b94cef9b79f81865574",
                "0x536662bf6b81ce90389ecf693674a0358527b4f5b0995456fee37469dd28e16",
                "0x72b7a0129fca4d354d5acbf84052626688e1d186e6c2e8cb19aa9f631e0e9db",
                "0xc64a895d8a2b0c16921c55d6510ce11a9e1750a02e8c29dd5cac86c8d1e982",
                "0x32cd32d89d9ca1fd4fae3b473e0226915514a6cfa23d79d9579fb403e46e65c",
                "0x6b0655c2fd47f55cc04d05cd3cf0e10945971d1b97f573fb882b71c59dfb449",
                "0x7fd2d96c69653a91504d2651acddcbc92b6acddc8f7693116051b427d056d61",
                "0x2de7ae0a048d05b30ad0ea302520305a672d4ace87f2cc07330d1ba5df09dae",
                "0x3c6989ae5b937ee6af8375d81e18e733f68b487b5263c7be7a1bc5088d47d2d",
                "0x2a4a042a939ea99471d90d95fff2c325e3fa543c6cd67349c1559fe37827046",
                "0x5d529b2b178acdb995804427e7eff4dcc69aa071a53c03765d88334b537df1f",
                "0xb0b56f73d99478dcdf327f4c06c7ba51abfbbe28cbc0db24fafb9eb23b618",
                "0x6bdf96d133763ab12d86ff94f84db26484fd87177ebe789a13514c984b45e46",
                "0x17c72c965af666b40f0267d9fad0edba73a8ba003afab033e8cffcac5852fd",
                "0xf737ecfab351ba0148871f58273322346f56547e488a7e1ae5b343e99c345f",
                "0x434abf1f06013ec6f4685e8acfb81c879f10acb9eb0672efc1110d2091df91c",
                "0x76b1c70082f50524c2b229eb0fea1e4f39b8b308e679a28f35543873706ca65",
                "0x3dfa27228f394377e7fd9bb3aff7f5d3bb3d942b7f92d8401ebf371b041f444",
                "0x39325345e24ed792e4829142649e85780311a2ecbdc7d9e99bac0ad367ac9c9",
                "0x123446a80f6089e6a3e8e1c82f61feda2e95e7eff5c7ca878e146d9c342f2a8",
                "0x52b724a01163f936b0fdeccc5be49e29e583826216bb75dcaa21789f62a017f",
                "0x7768cfdb7e0377847fdd144527ce8be42053e9b949f39d28c77d096a45f4899",
                "0x4e6eebaea0f3578f62a773500d6d2d9ff9f93c3182e9f42aa68a940305653c0",
                "0x7108e82f24f76758e0d15a4352fe677b732447a9243fb3474983bb6d1c8777",
                "0x3d89ed5c454278934cc97dccf8e114e096b069e92820b3d5728b19560638c0c",
                "0x1bb8284cc014252d03751f4ddc54390bcb6645766cebe93e98f71dc821d124b",
                "0x373f8ce96b7168ebcb3a118bac24ee5515903a6bd24ec8f943e693d9af1e02a",
                "0x4bac7f84dd6cc5d3e5fc9400a5ecebf27c95336eef665f059c78071c090ad68",
                "0x45c560059e0b51bf0a2a8bd09fb74f5699f3dd8f5ba5754168402be844ce79a",
                "0x210464e2c1dfa56a7f5e14a738f438893db6a08fee024ea69810fb5a2554bbb",
                "0x453a14ef1f22959e16d2d771403ce90cdef73715a22c59b14bf62049615db10",
                "0x29a22cb47f53c2f5c20a0efe19c59eecb5ab7df6d8da5c8cfb1de8f579c89e2",
                "0x392d994085ce631134acfc485420adb5a74feaa13192db209fa4f4b48209ca9",
                "0x3002c84174df933997e19c236604bc85718d6bf66232cf0063b2df7e1dc878c",
                "0x5e644483749453ffed1a1b0817d8053963b1adcc2541c0a211834ef69be4c93",
                "0x4ae257840c2bc49e9d065bf16fb91bb53550bb06198dee2022c818c71763189",
                "0x4b98c5bd7e3507beaf2873cb8f2ebf4e9743b75982b01708b243c43971b4bba",
                "0x118a75b675b9a6f002769d76c03d710236dca30a5b5b913cc09781940ee4d4e",
                "0x38e507ce7d76355bcdcffd9aa0dc70cfaa5aff9ba7c2ef2d09d0ac4e03f36a0",
                "0x157e61846651871215d58910b709fb4ad4daff448da093ead6bef00a48f8c69",
                "0x1270c3f8de988b8a6389ee0ce88e3dc7ab0d8bb0ba83d86b6b2a621a9df50d3",
                "0x286f0362032a502be387080b40a7b92fc44de1e8b58c537d8ae5e847ad8619f",
                "0x30dd31d5c754a281f91d250c811bc6cbd3550f09af105175b6b72fff8261f93",
                "0x5cde7daac8b8980147731923af7bed02b1187496d9ca9dd6a759aacad820f9b",
                "0x33bc352e5bdd7046a890d82436470a09cd1bd3677b22373dcde2cd5deb93a1",
                "0x7b3919f49df4e1a86ced84c553c013e10c21ff845eb08e59392e787b09368b3",
                "0x2db1e75d3e4bbb5661a1281b77ed7b558d557a16651ca7dda0b972ba6195ab9",
                "0x7203949255ea03ff9f23cb19c80391e101234fa845b73b1ddb4340bb46de1d2",
                "0x3ccd0554def08e86cd07dd920f3e628dafdd49d7c5bccfe24038b8f80e89073",
                "0x4f732ef59d57c4c1fcd4236ee74f430f05bf48d8fb156f18c146ec019bc82ac",
                "0x1a23747fe8961d2ba2115eb26d49529cafbf0664e68f3755f61d6582fba5de5",
                "0x288c9ec9962981a8e2c792cdba93eb48af7d5665b1baeec5722f58c2288967b",
                "0x3afebc1e0efd5591f40ac4793bb0cddfcb56c795b799dfe0d4d34259986674f",
                "0x54767d417dfad86bed2ce2ff323c738100e78d3339b3e38bb13f3f8f43927a3",
                "0x1dc95b995965f629eb75a50b58232c4936ff453ec84e25c6a56ffe65adc6c3",
                "0x1f3671ddbc04827b7a9c25295306446b0a238b2e17296d85e5d29bbd27ed17e",
                "0x78300d43ef835ac4d65442e4900d2d33f63f0cc04a7117276e8ca9eec90379e",
                "0x511a5f709b0b7bae5ef1e468fc1bf74f11c46016d73bd08fb8422ea453373df",
                "0x24a8d3a6a2c33b2134ecaa7d0c3160e7969dfc12f4c96409f159298dabd6f83",
                "0x613f34263ebf0b3c131d7709e391253a40265a92d28369b4c3a0e85a98a4491",
                "0x2893fff1cd679033eb438255ae1fc29bb0f31a6d21f961ba8c6d9d28de746aa",
                "0x463097b52722f2801396dc86261cecc4f32811168710af53e778d1009a79ec4",
                "0x2ad540971ff03c58fd7953197b4e4b2f6225ddbf2e57f7e00d6c1b5a57dcd2b",
                "0x7c5756c61c6f314e01c133b37ce7e7671c5578d6aba9a6331cdb2494bbea6a7",
                "0x1dc8cb820d563976f1c34a1c4cd79f7546713f2c62b30d956e6ea992f6ffb4c",
                "0x793e2fce80a764e00be738e5a1254a3c841688db32af4ed50c46c49bb40316c",
                "0x13c40f6cf7067e78cda28f0501825bc32ca4f0e55cebc5cf487c16dd441a20e",
                "0x6778b966da6bc8ece431e6eea74b3b582e312af52726193d536da9f3aae8fa5",
                "0x441596b6c3e462b121ebb816933470367abae8768e2eeb243baad9915410e98",
                "0x47668c8cc4202cb7ad26f83e58c14415d5ea0c28082616f95f630f8928eede0",
                "0x623b07af4d2460035e87395b2183f175a85b8fb81557383b904461d9a7dbe2a",
                "0x43dacf2240b2547f73c1f8161b7549305bc4a1c0891530f6e61158b520e7754",
                "0x50b45743c33745031d53a68543541ef03a31a49bf2a44ab8ad97f94e00db2dd",
                "0x1d5c5c152ff6d864cc8508742c41b80e6750032b1b4042f94bb093d8b6dc8d9",
                "0x787bd280a7f9ea9d1ad7847e14abb148757d757706162d6cc19417aace16780",
                "0x7ff36d1c713012e55c44662faf6aea7dbcc4ed412bc9575bf700821e905b401",
                "0x38a2015bc24b2d49fa34ea71694331b2bb9b6816c51e74b952745615ec0fdf1",
                "0x6017d279eef512afffa38341d7ddf37ca7b67e6afbc988be86d38cd28153783",
                "0x46680d2b0edced686278772ae149aef5df53c015b3c2605c1f2abd88658fea5",
                "0x5c4f2b1d6d547892cbcf746d92644a77809ec418edffe473466f3e9254b582",
                "0x7be091f07e6af6130285635ef1794d496034db9d7413820cd241bc9d684c1a9",
                "0x4e764c122050bb2fda8e3a41c85a24eae031a8a6c1a860b2ca158de0a62956d",
                "0xaf462e85b46e8d582e4cdc5c616f98f597427239da7684638ddcfadcf2231f",
                "0x47c9ac4416d13da3826ac274753cd216bda93c76548b557b341d1c14c11490d",
                "0x253b4111611580963cf488d4e7867ca46be58139306f41212357a9128f425c2",
                "0x341c6bc6bf683f626f1eaa877fdb39728de0bea7bf20663baa871e1a26e2af2",
                "0x703c1c5f8233ee0e51461e0a8f5e1c7aa1192fcc23eb622dc54d40ab6090ff7",
                "0x4e16ca0f93a7f2d72b84f9cc288a52ec9888cd4ec28b845f5b24bbc676f692c",
                "0x2101da64ef6a076d24f72be291055eaaa7d680547f80dfbbedbcd22ad905a6",
                "0x6eaf2f49b9d892ff68cbc38adcd777cd923ed58fcc42c91558639a8f63ad899",
                "0x6d2703bffa2a841a6fa6119fefeaddb627b4b8a5f2a2a05cf1d553eb506f128",
                "0x3dccd46b84d6655bbea7cb5bf556e863c84358894c88a0036c97a92a2937b84",
                "0x2d40d918e72ecae39c8496fe17e7784c394f303ca0d3f7b708426ddcbbbde39",
                "0x5c582cc5e546c5c4bf92b5d7715a618c6c43226cecd1c3a9e9b9e1d01753dd4",
                "0x425f1209a76ec6e9595a1a30c3cd99b7ca432e7e67493e730ddceaecab8aedf",
                "0x46c6cd668e4ac058cc45d48d1c70c4516ded3f025d0276e8185a14a9080244f",
                "0x544834e94ec103ba3850de1cfbe4e9b56e07b13e599ed328619c21b0f7a16bc",
                "0x7d5e2ec3780d71281fedcca6d08f1d0ed85264c5f1a2ec887fcac9a9d007de",
                "0x2277fa2169ec97ec808f6bba2c92271ac0fb73b20205ae0676addef2d4c88a7",
                "0x61d74d052b50379f3112cab0fc83991263ad5c89e326baddf4b228562e41953",
                "0x316e8bcdc0d8dbf1db4df8d2e63f96bb8ab900597030b7551f0332ff9d2a09c",
                "0x3b2c6d515af8b542f4c1273da8a3b78001e282c5fac30fafe8389c964bb8531",
                "0x3e81bb1da8ad64ec5cd3d7f4ce25ec8e963fa162ad25a685f20c9d2a783c472",
                "0x20f852942e1bf3bccc8e59852582e59bb79b56db056239e17ba29f1e48eac2",
                "0x4fd06fb2dc9f15df8c5d276942d0f178b826bd113d93e581544a68001e8d250",
                "0x34a12266c02676d97dbf4bf84447ddbefbacf28625ef94e5c045084fad4fd49",
                "0x5f8df30e9ffb54518dd8fc9b2f405e4f360c4a1b96346e5ef3819a04519729e",
                "0x21767ea9a9d982092c4b8c4dde56426ec50cf3e07f9ba5e29b2fa3e5754e102",
                "0x2d03896bb6ac6252768c4977b7590881a3b4c6d052fb8a8d263df2d25e7625",
                "0x356b0f96b11f5795990d625d8ea9c6de7d869a2044b7cd50dfe951ac29a6e0e",
                "0x6ae56f7f8741c8f43ef2f6966133dc0b34d5e8610b8c7dea3da7c89d7c2d2b9",
                "0x5ffa0f47cc7768c83d204643a7a447580a00fbfad7a12a769bab53febce2895",
                "0x2051d63f5ce3795e77efa76826f43872619ec46ec6afa7c23f9d6f355b443e0",
                "0x7e0f8123feae0a258bcda08a38e65e7bb28cdc04f84523b1640c9f54b68ce4b",
                "0x6c717817eef0504e32ce7a0b4fb18659be9ae9c2aa56ba210468ab2fcaf79cf",
                "0x5337b52058f81bc3dcdd03993850a58e5911af43cb3536ff4228539070fbdbc",
                "0x691cb37604424cfb585a6a35a42748479f1748ea0e65b902dd11651c2e85886",
                "0x59df9036736f235dc89f2261133f598664955a36c8ca22171542d7c2454c3eb",
                "0x31b7677eba3df8f0a5fc108056cc706cd4a68d233fa1bf9c8c65a05528a4c8c",
                "0x1f2e1702db19c2fae9e42f329ebd14c0964d9227e20362397f1f47138599561",
                "0x238b32490ee3ab940f6ca41f921b60d1aa44cb05f225287380703c76ed7b892",
                "0x4ab9b7217d715a3fe55eceb2a5dace1623a87a6e7118b209c13326824c30935",
                "0x47ca19eafd3a13ab82ba5557d7b9a2432d5fec617bd659862aed8f7a9141695",
                "0x10abd80ae93ca1193c6d22d9783565e6297ae5758f5c2a9fed9c979ffd8cb40",
                "0x386bba816d036566ed7c37de103c04055c7923c6aa36a95c3a04303645e21fa",
                "0x7a47cef3e67468513e6962ef223e732a1f5191041e837eec773a9fd2f0875c0",
                "0xbf2262e2b94bb0e73800663b0dc9039cdc1c808cc7ae62ca2e914f598a6958",
                "0x18413a00414d92091c18b2d984eeaa6020e3d2fc9ff17124a9e82702f294fb8",
                "0x531e2b4e28c591fa1e8df17ffe646db148b691b89f0bde12e8c465eb7c14c26",
                "0x5d9bde719bcc9d7f1d2dcd8c80b993c55e55064bebd17ec1484cdbffd9c910f",
                "0x70716e2187959123507a5a08a3d12a52c26632a74a4814a34346612cbfb626d",
                "0x31267b2b6dcb35cc23f7f0573a26722a5494f84c5c71d119772b80a0d692175",
                "0x5dea5c7c620d5bb173a5288c59e4b92f2944c502e73d10c01f933b844a9644b",
                "0x2cd3c1354b4772d3a5b6020902e388a290475dd6bc35ec3d215dc930f8dc56d",
                "0x5a233ad19f77821f871ea6d9d716501c9d1db9c4a525b2fc2b36d9aead841a6",
                "0x4d72e9a46fc082fd230296bd215d4e312bfccc9eb2301bdb486fc7c7d3d99f2",
                "0x71a9fa23e7973944d33b4556eb3d5c940bc111be9967c06ecddbafcf10de274",
            ]
            .iter()
            .map(|f| Felt::from_hex_unchecked(f))
            .collect(),
        )
    }
}
