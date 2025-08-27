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
use stark::swiftness::stark::types::cast_struct_to_slice;
use stark::{stark_proof::stark_commit::StarkCommit, swiftness::stark::types::StarkCommitment};
use std::{mem::size_of, path::Path};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::{
    global_values::{EcPoint, GlobalValues, InteractionElements},
    AccountCast, Executable,
};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 1000;

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> client::Result<()> {
    let config = Config::parse_args();

    let client = initialize_client(&config).await?;

    let payer = setup_payer(&client, &config).await?;

    let program_path = Path::new("target/deploy/verifier.so");

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

    println!("\nStarkCommit Task on Solana");
    println!("========================");

    let input = include_str!("../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let mut proof_verifier = proof.transform_to();

    // Calculate GlobalValues size (approximately 35 * 32 = 1120 bytes)
    let global_values = GlobalValues {
        trace_length: Felt::from_hex("0x10000000").unwrap(),
        initial_pc: Felt::from_hex("0x1").unwrap(),
        final_pc: Felt::from_hex("0x5").unwrap(),
        initial_ap: Felt::from_hex("0x1c6").unwrap(),
        final_ap: Felt::from_hex("0x1c43b3").unwrap(),
        initial_pedersen_addr: Felt::from_hex("0x1c43b8").unwrap(),
        initial_range_check_addr: Felt::from_hex("0x1f43b8").unwrap(),
        initial_bitwise_addr: Felt::from_hex("0x2f43b8").unwrap(),
        initial_poseidon_addr: Felt::from_hex("0x7f43b8").unwrap(),
        range_check_min: Felt::from_hex("0x0").unwrap(),
        range_check_max: Felt::from_hex("0xffff").unwrap(),
        offset_size: Felt::from_hex("0x10000").unwrap(),
        half_offset_size: Felt::from_hex("0x8000").unwrap(),
        pedersen_shift_point: EcPoint {
            x: Felt::from_hex("0x49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804")
                .unwrap(),
            y: Felt::from_hex("0x3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a")
                .unwrap(),
        },
        pedersen_points_x: Felt::from_hex(
            "0x598904d65b0434a87c175e65222359d01fff2522cade3bb409c28885b7671e",
        )
        .unwrap(),
        pedersen_points_y: Felt::from_hex(
            "0x4fe4068e06eefa17eefab622b3c9d9433bc11552fd96bf324893028770e40f4",
        )
        .unwrap(),
        poseidon_poseidon_full_round_key0: Felt::from_hex(
            "0x4f7c465fb34210b739758542eb985867c6ba4ec77b078ccb61b8e4288cbbae8",
        )
        .unwrap(),
        poseidon_poseidon_full_round_key1: Felt::from_hex(
            "0x2f96e26e8a7034b6317c2483e935e6bd1d5ea8efa42dc84ebba571760a1527d",
        )
        .unwrap(),
        poseidon_poseidon_full_round_key2: Felt::from_hex(
            "0x79e52af7b64407d08c6b7b54d92ea2477b7120da296f986f0d52705a850043d",
        )
        .unwrap(),
        poseidon_poseidon_partial_round_key0: Felt::from_hex(
            "0x17d8c8dc5aaa6ac1879e160be09a2012f52e1d6df8e3528255e00fa01f13020",
        )
        .unwrap(),
        poseidon_poseidon_partial_round_key1: Felt::from_hex(
            "0x786dda7880b1250660bec5c62a9c1a255f95c69b9d050d5bc4a89b4accdd89d",
        )
        .unwrap(),
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
        memory_multi_column_perm_perm_public_memory_prod: Felt::from_hex(
            "0x5593c3e7c28433d4bed879adb1cb8081b0a46decda462e76da45b0d7244cbf0",
        )
        .unwrap(),
        range_check16_perm_public_memory_prod: Felt::from_hex("0x1").unwrap(),
        diluted_check_first_elm: Felt::from_hex("0x0").unwrap(),
        diluted_check_permutation_public_memory_prod: Felt::from_hex("0x1").unwrap(),
        diluted_check_final_cum_val: Felt::from_hex(
            "0x5f16ce646fe7bef242b9158006cb52930937bf075c6e8bc638bba2b8244dfa",
        )
        .unwrap(),
    };

    // Calculate offsets
    let proof_bytes = cast_struct_to_slice(&mut proof_verifier).to_vec();
    let proof_size = proof_bytes.len();
    let mut global_values_mut = global_values;
    let global_values_bytes = cast_struct_to_slice(&mut global_values_mut).to_vec();
    let global_values_size = global_values_bytes.len();
    let stark_commitment_size = std::mem::size_of::<StarkCommitment<InteractionElements>>();

    println!("Proof size: {} bytes", proof_size);
    println!("Global values size: {} bytes", global_values_size);
    println!("Stark commitment size: {} bytes", stark_commitment_size);

    let stack_offset = 0;
    let proof_offset = 16; // front_index + back_index
    let buffer_offset = proof_offset + proof_size;
    let autogenerated_pows_offset = buffer_offset + 65536; // CAPACITY
    let oods_values_offset = autogenerated_pows_offset + 4288; // POWS_SIZE * 32
    let domains_offset = oods_values_offset + 6208; // OODS_VALUES_SIZE * 32
    let global_values_offset = domains_offset + 992; // DOMAINS_SIZE * 32
    let constraint_coefficients_offset = global_values_offset + global_values_size;
    let column_values_offset = constraint_coefficients_offset + 6208; // N_CONSTRAINTS * 32
    let stark_commitment_offset = column_values_offset + stark_commitment_size;

    // Start with stack initialization
    let mut stack_init_input: [u64; 2] = [0, 65536];
    let stack_init_bytes = cast_struct_to_slice(&mut stack_init_input);

    // Create instructions for each section
    let mut instructions = Vec::new();

    // 1. Set stack initialization
    let stack_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::SetAccountData(stack_offset, stack_init_bytes.to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );
    instructions.push(stack_ix);

    // 2. Set proof data
    let proof_chunks: Vec<_> = proof_bytes
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetAccountData(
                    proof_offset + (i * CHUNK_SIZE),
                    chunk.to_vec(),
                ),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect();
    instructions.extend(proof_chunks);

    // 3. Set global values
    let global_chunks: Vec<_> = global_values_bytes
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetAccountData(
                    global_values_offset + (i * CHUNK_SIZE),
                    chunk.to_vec(),
                ),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect();
    instructions.extend(global_chunks);

    // 4. Set constraint coefficients (empty)
    let constraint_coefficients_size = 6208; // N_CONSTRAINTS * 32
    let empty_constraint_coefficients = vec![0u8; constraint_coefficients_size];
    let constraint_coefficients_chunks: Vec<_> = empty_constraint_coefficients
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetAccountData(
                    constraint_coefficients_offset + (i * CHUNK_SIZE),
                    chunk.to_vec(),
                ),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect();
    instructions.extend(constraint_coefficients_chunks);

    // 5. Set column values (empty)
    let column_values_size = stark_commitment_size;
    let empty_column_values = vec![0u8; column_values_size];
    let column_values_chunks: Vec<_> = empty_column_values
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetAccountData(
                    column_values_offset + (i * CHUNK_SIZE),
                    chunk.to_vec(),
                ),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect();
    instructions.extend(column_values_chunks);

    // 6. Set stark commitment (empty)
    let empty_stark_commitment = vec![0u8; stark_commitment_size];
    let stark_commitment_chunks: Vec<_> = empty_stark_commitment
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetAccountData(
                    stark_commitment_offset + (i * CHUNK_SIZE),
                    chunk.to_vec(),
                ),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect();
    instructions.extend(stark_commitment_chunks);

    println!("Total instructions: {}", instructions.len());

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

    // Push the StarkCommit task to the stack
    let stark_commit_task = StarkCommit::new();

    println!("Using StarkCommit with TYPE_TAG: {}", StarkCommit::TYPE_TAG);

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(stark_commit_task.to_vec_with_type_tag()),
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
    println!("StarkCommit task pushed: {signature}");

    // Push digest to the stack
    let trace_generator =
        Felt::from_hex("0x57a797181c06d8427145cb66056f032751615d8617c5468258e96d2bb6422f9")
            .unwrap();
    let trace_domain_size = Felt::from_hex("0x10000000").unwrap();
    let digest =
        Felt::from_hex("0x59496b8e649ff03c8e9f739e141bd82653fccb2fb1b1a51a71760ea3813ea35")
            .unwrap();
    let counter = Felt::from_hex("0x0").unwrap();

    let push_digest_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(digest.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_digest_ix],
    )
    .await?;
    println!("Digest pushed to stack: {signature}");

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

    println!("All execution steps completed");
    println!("\nStarkCommit successfully executed on Solana!");

    Ok(())
}
