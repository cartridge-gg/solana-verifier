use crate::write_keypair_file;
use crate::{initialize_client, send_and_confirm_with_limit, setup_payer};
use crate::{read_keypair_file, Config, Result};
use felt::Felt;
use log::info;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
};
use utils::AccountCast;
use utils::BidirectionalStack;
use utils::Executable;
use verifier_1::instruction::VerifierInstruction as Verifier1Instruction;
use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;
// use verifier_2::instruction::VerifierInstruction as Verifier2Instruction;
use verifier_2::state::BidirectionalStackAccount as Verifier2StackAccount;
use verifier_3::state::BidirectionalStackAccount as Verifier3StackAccount;
use verifier_4::state::BidirectionalStackAccount as Verifier4StackAccount;

use verify_1::verify::Verify as Verify_Stage_One;
use verify_2::verify::Verify as Verify_Stage_Two;
use verify_3::verify::Verify as Verify_Stage_Three;
use verify_4::verify::Verify as Verify_Stage_Four;

pub const CHUNK_SIZE: usize = 900;

#[allow(clippy::unnecessary_mut_passed)]
pub async fn verify(config: &Config) -> Result<()> {
    let client = initialize_client(config).await?;
    let payer = if let Some(ref payer_keypair) = config.payer_keypair {
        Keypair::from_base58_string(payer_keypair)
    } else {
        setup_payer(&client, config).await?
    };
    info!(public_key:% = payer.pubkey(); "Using payer");

    write_keypair_file(&payer, "keypairs/payer-keypair.json").unwrap();

    // let time = std::time::Instant::now();

    info!("Using proof file: {}", config.proof);

    info!("\n========== STAGE 1: Verifier1 on Account1 ==========");

    let program_keypair = read_keypair_file("keypairs/verifier_1-keypair.json").unwrap();
    let verifier1_program_id = program_keypair.pubkey();
    let verifier2_program_id = read_keypair_file("keypairs/verifier_2-keypair.json")
        .unwrap()
        .pubkey();
    let verifier3_program_id = read_keypair_file("keypairs/verifier_3-keypair.json")
        .unwrap()
        .pubkey();
    let verifier4_program_id = read_keypair_file("keypairs/verifier_4-keypair.json")
        .unwrap()
        .pubkey();
    let account1 = read_keypair_file("keypairs/verifier-1-account-keypair.json").unwrap();
    let account2 = read_keypair_file("keypairs/verifier-2-account-keypair.json").unwrap();

    // Ensure accounts are owned by verifier1 and verifier2 before starting verification
    info!("  Ensuring proper ownership before verification...");
    ensure_ownership(&client, &payer, &verifier1_program_id, &account1).await?;
    ensure_ownership(&client, &payer, &verifier2_program_id, &account2).await?;

    // Clear account1 before starting verification (ensures clean state)
    // info!("  Clearing account1 before verification...");
    // clear_account(&client, &payer, &account1).await?;

    // Prepare initial data for Verifier1 (only needed for Stage 1)
    let stack_bytes = prepare_input::get_bytes_stage1(&config.proof);
    set_account_data_chunked(
        &client,
        &payer,
        &verifier1_program_id,
        &account1,
        &stack_bytes,
    )
    .await?;

    // Push Verify task
    let verify_task = Verify_Stage_One::new();
    push_task(
        &client,
        &payer,
        &verifier1_program_id,
        &account1,
        verify_task.to_vec_with_type_tag(),
    )
    .await?;

    // Calculate exact number of steps needed via simulation
    let mut account_data = client.get_account_data(&account1.pubkey()).await?;
    let stack = Verifier1StackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    info!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier1_program_id,
        &account1,
        simulation_steps as u32,
    )
    .await?;

    info!("✓ Stage 1 completed on account1");

    // ========== STAGE 2: Verifier2 on Account2 (PING-PONG: copy account1 → account2) ==========
    info!("\n========== STAGE 2: Verifier2 on Account2 ==========");

    // Clear account2 before starting verification (ensures clean state)
    info!("  Clearing account2 before verification...");
    // clear_account(&client, &payer, &account2).await?;

    copy_from_account(&client, &payer, &verifier2_program_id, &account1, &account2).await?;

    // Push Verify task (digest and counter are on stack, copied from account1)
    // Verify_Stage_Two will read them from stack in execute()
    let verify_task = Verify_Stage_Two::default();
    push_task(
        &client,
        &payer,
        &verifier2_program_id,
        &account2,
        verify_task.to_vec_with_type_tag(),
    )
    .await?;

    info!("Going to fetch account2 data");
    // Calculate exact number of steps needed via simulation
    let mut account_data = client.get_account_data(&account2.pubkey()).await?;

    info!("Account2 data fetched successfully");
    let stack = Verifier2StackAccount::cast_mut(&mut account_data);
    info!("Account2 data casted successfully");
    info!("Going to simulate");
    let simulation_steps = stack.simulate();
    info!("Simulation completed successfully");
    info!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier2_program_id,
        &account2,
        simulation_steps as u32,
    )
    .await?;

    info!("✓ Stage 2 completed on account2");

    // ========== STAGE 3: Verifier3 on Account1 (TRANSFER ownership account1: verifier1 → verifier3) ==========
    info!("\n========== STAGE 3: Verifier3 on Account1 ==========");

    // Transfer ownership of account1 from verifier1 to verifier3
    info!("  Transferring ownership account1: verifier1 → verifier3...");
    transfer_ownership(
        &client,
        &payer,
        &verifier1_program_id,
        &account1,
        &verifier3_program_id,
    )
    .await?;

    // PING-PONG: Copy all data from account2 to account1
    println!("  Copying account2 → account1...");
    copy_from_account(&client, &payer, &verifier3_program_id, &account2, &account1).await?;

    // Push Verify task (Verifier3 uses copied data from account2)
    let verify_task = Verify_Stage_Three::new();
    push_task(
        &client,
        &payer,
        &verifier3_program_id,
        &account1,
        verify_task.to_vec_with_type_tag(),
    )
    .await?;

    // Calculate exact number of steps needed via simulation
    let mut account_data = client.get_account_data(&account1.pubkey()).await?;
    let stack = Verifier3StackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    info!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier3_program_id,
        &account1,
        simulation_steps as u32,
    )
    .await?;

    info!("✓ Stage 3 completed on account1 (owner: verifier3)");

    // ========== STAGE 4: Verifier4 on Account2 (TRANSFER ownership account2: verifier2 → verifier4) ==========
    info!("\n========== STAGE 4: Verifier4 on Account2 ==========");

    // Transfer ownership of account2 from verifier2 to verifier4
    info!("  Transferring ownership account2: verifier2 → verifier4...");
    transfer_ownership(
        &client,
        &payer,
        &verifier2_program_id,
        &account2,
        &verifier4_program_id,
    )
    .await?;

    copy_from_account(&client, &payer, &verifier4_program_id, &account1, &account2).await?;

    // Push Verify task (Verifier4 uses copied data from account1)
    let verify_task = Verify_Stage_Four::new();
    push_task(
        &client,
        &payer,
        &verifier4_program_id,
        &account2,
        verify_task.to_vec_with_type_tag(),
    )
    .await?;

    // Calculate exact number of steps needed via simulation
    let mut account_data = client.get_account_data(&account2.pubkey()).await?;
    let stack = Verifier4StackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    info!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier4_program_id,
        &account2,
        simulation_steps as u32,
    )
    .await?;

    // Read final results
    let mut account_data = client.get_account_data(&account2.pubkey()).await?;
    let stack = Verifier1StackAccount::cast_mut(&mut account_data);

    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    info!("\n========== VERIFICATION RESULTS ==========");
    info!("  Program Hash: {:?}", result_program_hash);
    info!("  Output Hash:  {:?}", result_output_hash);

    // For saya proof
    // // Verify expected values
    // assert_eq!(
    //     result_program_hash,
    //     Felt::from_hex("0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07").unwrap(),
    //     "Program hash mismatch"
    // );
    // assert_eq!(
    //     result_output_hash,
    //     Felt::from_hex("0x3233b5615a8de5563f7d3ba086b8f260189ac47753a1c131d063ed3f6c24400")
    //         .unwrap(),
    //     "Output hash mismatch"
    // );
    assert!(stack.is_empty_back(), "Stack should be empty");
    assert!(stack.is_empty_front(), "Stack should be empty");

    Ok(())
}

async fn copy_from_account(
    client: &RpcClient,
    payer: &Keypair,
    dest_program_id: &solana_sdk::pubkey::Pubkey,
    source_account: &Keypair,
    dest_account: &Keypair,
) -> Result<()> {
    let copy_ix = Instruction::new_with_borsh(
        *dest_program_id,
        &Verifier1Instruction::CopyFromAccount, // All verifiers have the same instruction
        vec![
            AccountMeta::new_readonly(source_account.pubkey(), false),
            AccountMeta::new(dest_account.pubkey(), false),
        ],
    );

    send_and_confirm_with_limit(client, &[copy_ix], payer, 1_200_000, 100).await?;
    println!("  ✓ Copied successfully");
    Ok(())
}

async fn transfer_ownership(
    client: &RpcClient,
    payer: &Keypair,
    current_owner_program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    new_owner_program_id: &solana_sdk::pubkey::Pubkey,
) -> Result<()> {
    let transfer_ix = Instruction::new_with_borsh(
        *current_owner_program_id,
        &Verifier1Instruction::TransferOwnership, // All verifiers have the same instruction
        vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(*new_owner_program_id, false),
        ],
    );

    send_and_confirm_with_limit(client, &[transfer_ix], payer, 1_400_000, 100).await?;
    println!("  ✓ Ownership transferred successfully");
    Ok(())
}

async fn set_account_data_chunked(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    stack_bytes: &[u8],
) -> Result<()> {
    let mut instructions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            *program_id,
            &Verifier1Instruction::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(account.pubkey(), false)],
        );
        instructions.push(set_data_ix);
    }
    send_and_confirm_with_limit(client, &instructions, payer, 1_400_000, 100).await?;
    info!("Data set successfully");
    Ok(())
}

async fn push_task(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    task_data: Vec<u8>,
) -> Result<()> {
    let push_task_ix = Instruction::new_with_borsh(
        *program_id,
        &Verifier1Instruction::PushTask(task_data),
        vec![AccountMeta::new(account.pubkey(), false)],
    );

    send_and_confirm_with_limit(client, &[push_task_ix], payer, 1_200_000, 100).await?;
    println!("  Task pushed successfully");
    Ok(())
}

async fn execute_verifier(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    simulation_steps: u32,
) -> Result<()> {
    let simulation_steps_usize = simulation_steps as usize;

    let mut step = 0;
    while step < simulation_steps_usize {
        // let chunk_size = if step >= 24500 { 1 } else { CHUNK_SIZE };
        let chunk_size = CHUNK_SIZE;
        let chunk_end = std::cmp::min(step + chunk_size, simulation_steps_usize);

        info!("Processing steps {}-{}", step, chunk_end - 1);

        let mut instructions = Vec::new();
        for i in step..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &Verifier1Instruction::Execute(i as u32),
                vec![AccountMeta::new(account.pubkey(), false)],
            );
            instructions.push(execute_ix);
        }

        send_and_confirm_with_limit(client, &instructions, payer, 1_200_000, 100).await?;
        info!("Chunk {}-{} completed", step, chunk_end - 1);

        step = chunk_end;
    }
    Ok(())
}

/// Ensure account is owned by the specified program, transfer ownership if needed
async fn ensure_ownership(
    client: &RpcClient,
    payer: &Keypair,
    target_program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
) -> Result<()> {
    // Get account info to find current owner
    let account_info = client.get_account(&account.pubkey()).await?;
    let current_owner = account_info.owner;

    // if current_owner == *target_program_id {
    //     info!("  Account already owned by target program: {}", target_program_id);
    //     return Ok(());
    // }

    info!(
        "  Transferring ownership from {} to {}",
        current_owner, target_program_id
    );

    let transfer_ix = Instruction::new_with_borsh(
        current_owner,
        &Verifier1Instruction::TransferOwnership,
        vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(*target_program_id, false),
        ],
    );

    send_and_confirm_with_limit(client, &[transfer_ix], payer, 1_400_000, 100).await?;
    info!("  Ownership transferred successfully");
    Ok(())
}

mod prepare_input {
    use std::fs;
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;

    pub fn get_bytes_stage1(proof_path: &str) -> Vec<u8> {
        let proof_str = fs::read_to_string(proof_path)
            .unwrap_or_else(|e| panic!("Failed to read proof file '{}': {}", proof_path, e));
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(&proof_str)
            .unwrap_or_else(|e| panic!("Failed to parse proof JSON from '{}': {}", proof_path, e));
        let proof = StarkProofParser::try_from(proof_json)
            .unwrap_or_else(|e| panic!("Failed to transform proof from '{}': {:?}", proof_path, e));
        let proof_verifier = proof.transform_to();

        let mut stack = Verifier1StackAccount {
            proof: proof_verifier.clone(),
            oods_values: proof_verifier
                .unsent_commitment
                .oods_values
                .as_slice()
                .try_into()
                .unwrap(),
            ..Default::default()
        };

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }
}
