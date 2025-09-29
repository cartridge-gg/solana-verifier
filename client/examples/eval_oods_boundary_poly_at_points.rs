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
use stark_verify_verification::eval_oods_boundary_poly_at_points::EvalOodsBoundaryPolyAtPoints;
use std::{mem::size_of, path::Path};
use types::swiftness::stark::types::FriVerifyData;
use utils::BidirectionalStack;
use utils::CacheStorage;
use utils::{AccountCast, Executable};
use verifier_3::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

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

    let program_path = Path::new("target/deploy/verifier_3.so");

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

    // Use the same points as in the unit test (from fixtures::queries::result())
    let points_hex = [
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
    ];

    let points = points_hex
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<_>>();

    // Push points in reverse order (like in unit test)
    for point in points.iter().rev() {
        let push_point_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::PushData(point.to_bytes_be().to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );

        let _signature = interact_with_program_instructions(
            &client,
            &payer,
            &program_id,
            &stack_account,
            &[push_point_ix],
        )
        .await?;
    }

    // Push points length
    let points_length = Felt::from(points.len() as u64);
    let push_points_length_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(points_length.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_points_length_ix],
    )
    .await?;

    // Push the EvalOodsBoundaryPolyAtPoints task to the stack
    let eval_oods_boundary_poly_at_points_task = EvalOodsBoundaryPolyAtPoints::new();

    println!(
        "Using EvalOodsBoundaryPolyAtPoints with TYPE_TAG: {}",
        EvalOodsBoundaryPolyAtPoints::TYPE_TAG
    );

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(
            eval_oods_boundary_poly_at_points_task.to_vec_with_type_tag(),
        ),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let _signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_task_ix],
    )
    .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_200_000);

    // Execute all steps until task is complete - split into chunks of max 5000
    const MAX_CHUNK_SIZE: usize = 1000;

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

    println!("All execution steps completed");

    // Read results like in unit test
    let mut evaluations = Vec::new();

    let fri_verify_data: &mut FriVerifyData = stack.borrow_from_cache_mut();
    for value in fri_verify_data.fri_decommitment.values.iter() {
        evaluations.push(*value);
    }

    // Expected results from unit test
    let expected_result = vec![
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
    .map(|f| Felt::from_hex(f).unwrap())
    .collect::<Vec<Felt>>();

    println!("Expected result: {:?}", expected_result);
    println!("Actual result:   {:?}", evaluations);

    assert_eq!(
        evaluations, expected_result,
        "Result should match expected value from unit test"
    );

    // Check that stack is empty (task completed successfully)
    assert!(stack.is_empty_back(), "Stack should be empty");
    assert!(stack.is_empty_front(), "Stack should be empty");

    println!("✓ All verifications passed! Results match expected values from unit test");
    println!("✓ Stack is empty - task completed successfully");
    println!("✓ EvalOodsBoundaryPolyAtPoints test completed successfully on Solana!");

    Ok(())
}

mod prepare_input {
    use felt::Felt;
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::swiftness::global_values::InteractionElements;
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use types::swiftness::stark::types::StarkCommitment;
    use verifier_3::state::BidirectionalStackAccount;

    use crate::constraint_coefficients;

    pub fn get_bytes() -> Vec<u8> {
        let mut stack = BidirectionalStackAccount::default();

        // Load proof like in unit test
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();
        stack.proof = proof_verifier.clone();

        // Set constraint coefficients and oods values like in unit test
        stack.constraint_coefficients =
            constraint_coefficients::get_constraint_coefficients_for_interaction_after_oods()
                .as_slice()
                .try_into()
                .unwrap();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();

        // Set up stark commitment with OODS point like in unit test
        let oods_point =
            Felt::from_hex("0x49185430497be4bd990699e70b3b91b25c0dd22d5cd436dbf23f364136368bc")
                .unwrap();

        let mut stark_commitment: StarkCommitment<InteractionElements> = StarkCommitment::default();
        stark_commitment.interaction_after_composition = oods_point;
        stack.stark_commitment = stark_commitment;

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
    pub fn get_constraint_coefficients_for_interaction_after_oods() -> FunVec<Felt, 194> {
        FunVec::from_vec(
            vec![
                "0x1",
                "0x1b63b11b6dbde42a3c1889ed7a8a7fa8b341c3e478db9d2364cd7b0f98ba234",
                "0x1540bf531b68f071978f5f2bfafe5a316ad3f3f2d4a1d0301118c3462a9a85f",
                "0x4c590f823dd204e1c2576968a1e5abce136f69bf52de6cef4e486c895f8ef15",
                "0x15a60926c43c071bfaec4eba798d842000a3ad4faf7fbe4942d8f5c4c8c823c",
                "0x1cce7414b3015704bb129075520177f1d91391d64b27687702054162ff0ebb9",
                "0x636718275ac6fb788c3fdecb5bc12cf7c0460b75a4f837b91cfa5c1aba28085",
                "0x7d6e1959906ed0e60c8c716445b29126736f5f441790bf677e206f064fd6347",
                "0x1ec592e5a96ac46830f2a73a24872514f5ae2e0605295feda5c29bdcbab0f2b",
                "0x79f182e12eeaa6f787d449582cf498bf35b7fe635a91ea4ede86d7d1dc2b787",
                "0x58a9309fbad4f54610b70d2bab7c18ea40a03792211189ba1ed5e07b96c6632",
                "0xc09732081f5afec7ae0113c68cce87f11309cc2b8e6aabe9a497f245a8217f",
                "0x361bbfd1225b5b90e7a48f09f04a7a9c566511015885314942b7b25610e4be7",
                "0x75bc16f7c3b462fb42b8b5adaf75d8ee40d9ce4810e6894db9771d897abf9af",
                "0x3215de0659269e0e31202ec4478a5701826bb570e99c768107d88dc829d4603",
                "0x7ea169d9072974f778b6860cfa53ad577a81754e4f0e32dd7be690913cb9e6c",
                "0x5709a5b9d67e5b2fc30f065e9ac4315e2d4558e1c848bc154d0db6b1817e2e5",
                "0x1872e58c6886e9dd7852b808a14405d7c6b9b4173366eb5d11459fada72992b",
                "0x690bfdd6d6b4731bbdbe83238ab879069d1e2dd79f1964c447bb975182a409b",
                "0x1af437997a6b8194353c516bd26f3b8d615026eefc39261e08e73dcd0e8f300",
                "0x658b1909fcfb2a55ab5ed9346a3f37f790774679f5658c98c30f83d5cd8d3aa",
                "0x729fc4caebc09d35b1083df4cb8e98f8392e39f15b44e08b3664391d1bc9b85",
                "0x3cf804e2c2f93805d35a16509db119e9306f00bace0a1d84195cdfaf933270",
                "0x2e7a7656df4024d3b43e827b9fb907968fe5e79f6b8fce9d564b93793ad88b3",
                "0x7c1979f53cd24a3aaaec2950be392c43f213d27038f55ba172d964ea7429625",
                "0x7138f9e39f62c453db7adc56e506d5c5fbcc8c677a1e5a7dd702ad347abf0e5",
                "0x441e9c67345f7d947282025d3b17f135aeadb2fffa110c741578d06d906f2a0",
                "0x3f2a7c265985dcf9ae78b903191e11102b6539e9855eeeb3da2081005eb5b1",
                "0x731186586f2addfb4cd991475f4eebea3e470779dd7d645aa79988eb9c5c1cf",
                "0x4b53b0ec48ed4443cea2155e48b292f00aebec6b07f37f9c2bd13d155ced9a4",
                "0x2351b3b2ca2d1ff1e9262340efa77b1efa9566a0a2fa0e2f240df34dd8693ec",
                "0x6a16c246ae4da2e2189bd80ee3f3b711ac72c7292f6b97429e602bcd8e3557c",
                "0xe2dc8df7c16230ab4be29a8a9960ee72956142af2a9b595143bcec37271897",
                "0x437c423e9482eba774eb4a55b9b706fb020e1f4aa399f4fb9d549782da5dcc9",
                "0x6d31a762149082022873cfb2b22c835d4538c675806b47a32508ab6d9b98440",
                "0x131433950a91d0a0351ec7ed43cd6f3ca92e6aae40929d77257b4a339129185",
                "0x7b6070e8bae8440e595dd53b96c6e9a8b5a2bccd25cc75b4856ed0b7be0fb04",
                "0x41290faa3d56a317295479bee01172d071ce27a07f37219ee11ede6a651f41",
                "0x57d4313be0d04063669db86f0bbab67d4ae07bf8ff742d293eb9492c3b5c744",
                "0x16e8fc9bf9fdee3cfcc13913eda1c796bc1692da2e844898b4e43c8512519b3",
                "0x7519dacc964a4420f19aa25e92379ebb01079e95f3c486e7228000e4cf8d60a",
                "0x7166679002a7b681b2ecd62c36decdfd4755265c9d681d2b36bcc6f707063a4",
                "0x4b640b14f292471d7154ef618c65fe5e39f466dd6483c50fc455e5974debca",
                "0x4da6026e85025aeaf42e38d1e3a1aea98abb2d9a1115f46f810e4085e0b50ee",
                "0x4ecc80349de6b7804da60b27205801aefc6ea4cf0d864e47356c8ce784a5a80",
                "0x4f373d9b427a84f5d08150b68b110d249fde101943cad49942824f0cbd630c1",
                "0x1e8638340e9f61321a7190be952aa833d3bdcbd2f23e945fcd770f5dadf81ce",
                "0x7cdae99167d70f6193e357c2ffdcd2957bdfb0bb4f7a4bf2cbad35e09006f2f",
                "0x4699ca7563be7b7ca3a86f578bd8ee2f58b056576ff99e8df20ca9ba1bccc52",
                "0x305bb255adf3abd121321bfdae16bf36689ba68ad34ba26d1aefbac188897a5",
                "0x4a96dc8540435486a36f75d704abdb5083d4a5842899837ba72acf7abe85ec0",
                "0x333a82914de82858baeaf3525f103d50684c6dffd6dfa9ebedc36b1d8e8917f",
                "0x153eb906e68eccf32b38223c55208ca8099d2d374faddb2a8db88692dd967d",
                "0x5d53b00ca863cf2b89a4a603ead71fa1495d91193f163702962dc310bf92951",
                "0xb4bc07860b5cfaecc6b41e135ff9b2649cf55b02e90eb0e5fe299e3dd31e04",
                "0x3c91b7308a1f5958601782a343bee956bdd728d1d7b5df8a2198e6392daa2bb",
                "0x2df9eaab6e0156593c6d44c2ebee2df038ccd1715ca2470d302e29b07260176",
                "0x2595852af47f1bac984e14f0b2e704b0e9a9807a13bdeed850051e138a90b7d",
                "0x4b53e06d5e3aa7afe9d47d1292e97636329d19bae457f87a9c454ae540337a6",
                "0x235056aac3afb2341a5cab57eaa67eb8e1e78c2f7bee7a3655bf2e06f95156f",
                "0x11ec46329e45a959956bf0d8cfeb6519cdec453e711b571f7c428574bb2f6e0",
                "0x122efcb50c70b3dcbe72c68280ed4ecfb6de3f8bf6ab622c1a75870c0fdd599",
                "0x11091ab6ce693928f731d2995661b55148268cebceb21ab152264349e03982b",
                "0x53eba1db2c7502b5c52c1e1a57102540c45ad2faf46fe5023a5fc42ceffc1c0",
                "0x16d2b1849a29cb565cf743c834834ac4d5888c6eece38c2637889311b1885d6",
                "0x1a963195c459de1b5b6fb1242cb18f49460f84c877de10ee0733482323b703e",
                "0x3b6d28895d980125d30401a4001f3969d269c834af6bd4575d678454d43fe77",
                "0x4faddedccc48ccf22b430f21a65011b5758b65e4324ea2268bc0a8718c03659",
                "0x7ef6c96e1dd2a296d320cb0246786834c17a4a35a704441371183b55b49b5db",
                "0x37afeaf2ddad2737a666736d9529ac1ec655e00782b861a6e3691a9bc218b61",
                "0x7b567b0d38db69169f11de0693807d38a4d7fe706bdcbee56ff34864f3509d",
                "0x29d52301316e47ff6e2869a67aa98863f09beeea0f3586184ddc4c7bfc84cc1",
                "0x3fb9d5a5e68f16ebefdb710db92f55025fbd0e7d34bc9df7796afac6b6c5793",
                "0x3a73c6c1d0159841808ad5bc3b5229ee8bba13c85c6b71a55785202a08ab341",
                "0xb0629e1d5986766e17eea7de625085625e7d3b031a161713612952887ae906",
                "0x3360b868a7225363f41e2240573b2b6475152b5f0d24b8584e6d7b21dba052e",
                "0x18a0bb8a790afbe63f5b7c32fdd34732c401d27f2ad5ac2d88459bbc4b775e1",
                "0x3ac87f6ee65d9075129b49a7ef96fa2be06c522df2457d711ca26cf3098bd95",
                "0x1af79fae42067105aa20e71b0259039e2b356ec132d7b4f817872f8d2358605",
                "0x18600c3923a67466d77a035336aa4a3361332665ecaa1d5d803ee02162c950b",
                "0x4ac5e3493995fe70fc1081af1e54ea4bb47b32a607a2851b36e17eed9d4e7a1",
                "0x3e8050fd86f211fac87000970ea67f1e5077a3450bb7cd602d704da8124995e",
                "0x4679767a59aa8bfbc865a2aafe5a27ca249b5667d942703b16ee13b45ad1592",
                "0xda528262d3cc3f341c9e49662fb23adfbcb0070d217c6b5d93dfb710fafc0c",
                "0x34e759efb080862196b82054de3bdba7f787ff644d033797ce561fdcf962ca3",
                "0x7abb8faa7cfba157dae9b97a72c1411180b5ad0168a33b981779a4be0d1f179",
                "0x5cec019c5b800f7326c4ce6b7381c7c686b817cf528a7283a6a3d01a1d29a41",
                "0x2e22ad1ff142156ac8be3d985388e5a100f252572c3046347e908a9fd2201dd",
                "0x68049a51ef5977bf0324dd763de6590e468064240bdd05ef4ca7ef89382926c",
                "0x5419df57acda5de881158d2a82d767b7fe2764591c662b6571d82fb759c3e25",
                "0x18fe3382fdb36f7498f067ee91003a4e06e9cf1acdc0f600d50c0351a1ea019",
                "0x7d14b02bd67662ebd47fdfab77a09c5592874f1706a37dba45f6e846cb949c0",
                "0x7eb30ed00230133494e5f5813f966b20fde40cce9a8fb7caa72bfcd4eb33233",
                "0x5b69e4bf5b31b0ba9c4df82ccfb9d82e0ba16d07512db91e5b855455163f1db",
                "0x5ad78452a2db2fdb29505046075ee1fad94ce9dac56c89e6bcbd609b02cbe98",
                "0x2047594d9f85165bfe2c11a6c7cec744b1dfa85b07b9464ad6d91c3bdf2d8dd",
                "0x6144467207b6bd12b3439e43e81a09d9ce2054372f4766d6860022da6f5a9ba",
                "0x40965c9fe4a7e9bc7e1d63be68db157759bc2bf402f312aae31b1b6abaa25d5",
                "0x6dfd773d3d03319fb9cb9a72a7b2e6d701515c38c999a03a91370837cb92987",
                "0x2eb44bd16bc008432add7d7d7f1fb2afe1d8f471adc02da558bc4bbcd45b39f",
                "0xfb1511de0b7e718f4c737b6e70d093fa55d3e6cd182d620249fe99a4f49707",
                "0x34aef5d75bea4bab33c48493638cf8fc1b52732d0cf53278e4f319f98cb6a5",
                "0x52cc74c26995dc90683f0f97e767177b2ceb30ccb5bc0fd36cb06f889342719",
                "0xcc0ce9ff5f1f2583ab22ebdb3a970cd4a62dcae45429d31d8d342d532d60c1",
                "0x114cd1615857484fc5403c586c050b3be3a54260a20a70935749be81a78ccc3",
                "0x182c9fa7eac92f96e262043e37ddccd62641455b0f72be53c8b92c3a09a7742",
                "0x45d197019141bbd3cdb2e534b3929116c77f9963482932d3a44ddfcfc990c64",
                "0x28fdecf482722907fb995e3f626e8ce1053f3c480c3fec50be959063cb88dbf",
                "0x221d1b34f7f32cbae5abfd26701b408d73617e25b438d8aa5aec241323911d2",
                "0x59bee037ca5613934d9b51cc9296407a639960dbb6507e40e1575dd6b148e0",
                "0x3ebf2a6663191b8eacc33c302e43dfb47db9ffa82741aec4b795ba17aa714d",
                "0x790f8ad0d61fdbfbb5abae53e45d6f1167acef054b4636198a2a5714e356f64",
                "0x334380a9b387c79a1bb673bcfb503f11a310c54804d93f52527b1ca6bb3261a",
                "0x7b2f94ca34696bfffd71a9cd5adf47ecce22af7025738eb1512391df8948a91",
                "0x657af76e2a5d2fc4c08829b88a8d2ddceba6b8107e7cc8f2c9012494671fa90",
                "0x470507cf36fa5416e7a46430958be0ea15d654d821292f8932228e177edd0cb",
                "0x116c38e8787004837660dff1fefd9e9feb8cae822348bec11b84a5dbaf67c0c",
                "0x1626ab0daad9d2693b4786d3f4e4048e6f33641b0de2ea41048ac2fd0edb7c7",
                "0x3d9f9c7ad535de55d579094ade4669d3eec8ae469b2e0ee8d20f204161f0bed",
                "0x60d711f4986fe6c6575770ca8282a4848350b9d7d86bb27750c02415b64ffc2",
                "0x1a157d254b0957f79547f8886deafdf7c0d4d3b93a81a7e3444d9c1a53411c1",
                "0x634eb9709e787a1989515eb6bb881de37df0517237223ecd39f3cf30bf1d3b2",
                "0x4a9a238c388534e1079f3b0a4d72043505cb9f916d3a371835e49424440738e",
                "0x4369143d15c9336b35fbf8bc83ebc2eadfc157b1f762402c4218edee7eac942",
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
