/// Universal Stack Account Verification Example - ON-CHAIN (PING-PONG)
///
/// This example demonstrates the PING-PONG architecture with 2 accounts alternating
/// ownership between 4 verifier programs. Data is copied between accounts as ownership transfers.
use clap::Parser;
use client::{
    initialize_client, send_and_confirm_transactions, setup_payer, setup_program, Config,
};
use felt::Felt;
use log::{debug, error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use std::{mem::size_of, path::Path};
use utils::{AccountCast, BidirectionalStack, Executable};
use verifier_1::instruction::VerifierInstruction as Verifier1Instruction;
use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;
use verifier_2::instruction::VerifierInstruction as Verifier2Instruction;
use verifier_2::state::BidirectionalStackAccount as Verifier2StackAccount;
use verifier_3::state::BidirectionalStackAccount as Verifier3StackAccount;
use verifier_4::state::BidirectionalStackAccount as Verifier4StackAccount;

use verify_1::verify::Verify as Verify_Stage_One;
use verify_2::verify::Verify as Verify_Stage_Two;
use verify_3::verify::Verify as Verify_Stage_Three;
use verify_4::verify::Verify as Verify_Stage_Four;

pub const CHUNK_SIZE: usize = 1000;
const MAX_CHUNK_SIZE: usize = 3000;

#[derive(Parser, Debug)]
#[command(name = "verify")]
#[command(about = "Verify a STARK proof on Solana")]
struct Args {
    /// Path to the proof JSON file
    #[arg(short, long, default_value = "example_proof/saya.json")]
    proof: String,

    #[command(flatten)]
    config: Config,
}

fn main() {
    let args = Args::parse();

    let result = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .thread_stack_size(32 * 1024 * 1024)
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async_main(args.proof, args.config))
        })
        .unwrap()
        .join()
        .unwrap();

    if let Err(e) = result {
        error!("Error: {:?}", e);
        std::process::exit(1);
    }
}

#[allow(clippy::result_large_err)]
async fn async_main(proof_path: String, config: Config) -> client::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("client", log::LevelFilter::Info)
        .init();

    // let config = Config::parse_args();
    let client = initialize_client(&config).await?;
    let payer = setup_payer(&client, &config).await?;

    info!("========================================");
    info!("  PING-PONG Verifier - 2 Accounts");
    info!("========================================");
    info!("Using proof file: {}", proof_path);

    // Deploy ALL 4 verifier programs
    info!("Deploying programs...");

    let verifier1_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_1.so"),
    )
    .await?;
    info!("✓ Verifier 1: {}", verifier1_program_id);

    let verifier2_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_2.so"),
    )
    .await?;
    info!("✓ Verifier 2: {}", verifier2_program_id);

    let verifier3_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_3.so"),
    )
    .await?;
    info!("✓ Verifier 3: {}", verifier3_program_id);

    let verifier4_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_4.so"),
    )
    .await?;
    info!("✓ Verifier 4: {}", verifier4_program_id);

    // Create TWO accounts for ping-pong
    let account1 = Keypair::new();
    let account2 = Keypair::new();

    let space = size_of::<Verifier1StackAccount>();
    info!(
        "✓ Creating account1: {} (owner: verifier1)",
        account1.pubkey()
    );
    debug!("  Account size: {} bytes", space);
    create_account_tx(&client, &payer, &account1, space, &verifier1_program_id).await?;

    info!(
        "✓ Creating account2: {} (owner: verifier2)",
        account2.pubkey()
    );
    create_account_tx(&client, &payer, &account2, space, &verifier2_program_id).await?;

    // ========== STAGE 1: Verifier1 on Account1 ==========
    info!("========== STAGE 1: Verifier1 on Account1 ==========");

    // Prepare initial data for Verifier1 (only needed for Stage 1)
    let stack_bytes = prepare_input::get_bytes_stage1(&proof_path);
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
    debug!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier1_program_id,
        &account1,
        simulation_steps as u32,
        1, // Stage 1
    )
    .await?;

    info!("✓ Stage 1 completed on account1");

    // ========== STAGE 2: Verifier2 on Account2 (PING-PONG: copy account1 → account2) ==========
    info!("========== STAGE 2: Verifier2 on Account2 ==========");

    // PING-PONG: Copy all data from account1 to account2
    debug!("  Copying account1 → account2...");
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

    // Calculate exact number of steps needed via simulation
    let mut account_data = client.get_account_data(&account2.pubkey()).await?;
    let stack = Verifier2StackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    debug!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier2_program_id,
        &account2,
        simulation_steps as u32,
        2, // Stage 2
    )
    .await?;

    info!("✓ Stage 2 completed on account2");

    // ========== STAGE 3: Verifier3 on Account1 (TRANSFER ownership account1: verifier1 → verifier3) ==========
    info!("========== STAGE 3: Verifier3 on Account1 ==========");

    // Transfer ownership of account1 from verifier1 to verifier3
    debug!("  Transferring ownership account1: verifier1 → verifier3...");
    transfer_ownership(
        &client,
        &payer,
        &verifier1_program_id,
        &account1,
        &verifier3_program_id,
    )
    .await?;

    // PING-PONG: Copy all data from account2 to account1
    debug!("  Copying account2 → account1...");
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
    debug!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier3_program_id,
        &account1,
        simulation_steps as u32,
        3, // Stage 3
    )
    .await?;

    info!("✓ Stage 3 completed on account1 (owner: verifier3)");

    // ========== STAGE 4: Verifier4 on Account2 (TRANSFER ownership account2: verifier2 → verifier4) ==========
    info!("========== STAGE 4: Verifier4 on Account2 ==========");

    // Transfer ownership of account2 from verifier2 to verifier4
    debug!("  Transferring ownership account2: verifier2 → verifier4...");
    transfer_ownership(
        &client,
        &payer,
        &verifier2_program_id,
        &account2,
        &verifier4_program_id,
    )
    .await?;

    // PING-PONG: Copy all data from account1 to account2
    debug!("  Copying account1 → account2...");
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
    debug!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier4_program_id,
        &account2,
        simulation_steps as u32,
        4, // Stage 4
    )
    .await?;

    // Read final results
    let mut account_data = client.get_account_data(&account2.pubkey()).await?;
    let stack = Verifier1StackAccount::cast_mut(&mut account_data);

    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    info!("========== VERIFICATION RESULTS ==========");
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

    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");
    assert_eq!(stack.is_empty_front(), true, "Stack should be empty");
    info!("✓ All 4 stages completed successfully using PING-PONG architecture!");
    info!("✓ All verifications passed!");
    info!("✓ PING-PONG verifier test completed on Solana!");
    info!("Final owners:");
    info!("  Account1: {} (owner: verifier3)", account1.pubkey());
    info!("  Account2: {} (owner: verifier4)", account2.pubkey());

    Ok(())
}

// Helper functions
async fn create_account_tx(
    client: &RpcClient,
    payer: &Keypair,
    stack_account: &Keypair,
    space: usize,
    program_id: &solana_sdk::pubkey::Pubkey,
) -> client::Result<()> {
    let create_account_ix = create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        program_id,
    );

    let create_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[payer, stack_account],
        client.get_latest_blockhash().await?,
    );

    client
        .send_and_confirm_transaction(&create_account_tx)
        .await?;
    debug!("  Account created successfully");
    Ok(())
}

async fn copy_from_account(
    client: &RpcClient,
    payer: &Keypair,
    dest_program_id: &solana_sdk::pubkey::Pubkey,
    source_account: &Keypair,
    dest_account: &Keypair,
) -> client::Result<()> {
    let copy_ix = Instruction::new_with_borsh(
        *dest_program_id,
        &Verifier2Instruction::CopyFromAccount, // All verifiers have the same instruction
        vec![
            AccountMeta::new_readonly(source_account.pubkey(), false),
            AccountMeta::new(dest_account.pubkey(), false),
        ],
    );

    let tx = Transaction::new_signed_with_payer(
        &[copy_ix],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&tx).await?;
    debug!("  ✓ Copied successfully");
    Ok(())
}

async fn transfer_ownership(
    client: &RpcClient,
    payer: &Keypair,
    current_owner_program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    new_owner_program_id: &solana_sdk::pubkey::Pubkey,
) -> client::Result<()> {
    let transfer_ix = Instruction::new_with_borsh(
        *current_owner_program_id,
        &Verifier1Instruction::TransferOwnership, // All verifiers have the same instruction
        vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(*new_owner_program_id, false),
        ],
    );

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&tx).await?;
    debug!("  ✓ Ownership transferred successfully");
    Ok(())
}

async fn set_account_data_chunked(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    stack_bytes: &[u8],
) -> client::Result<()> {
    let mut transactions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            *program_id,
            &Verifier1Instruction::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(account.pubkey(), false)],
        );
        let tx = Transaction::new_signed_with_payer(
            &[set_data_ix],
            Some(&payer.pubkey()),
            &[payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(tx);
    }
    send_and_confirm_transactions(client, &transactions).await?;
    debug!("  Account data set successfully");
    Ok(())
}

async fn push_task(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    task_data: Vec<u8>,
) -> client::Result<()> {
    let push_task_ix = Instruction::new_with_borsh(
        *program_id,
        &Verifier1Instruction::PushTask(task_data),
        vec![AccountMeta::new(account.pubkey(), false)],
    );

    let tx = Transaction::new_signed_with_payer(
        &[push_task_ix],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&tx).await?;
    debug!("  Task pushed successfully");
    Ok(())
}

async fn execute_verifier(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    simulation_steps: u32,
    _stage_number: u8,
) -> client::Result<()> {
    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
    let simulation_steps_usize = simulation_steps as usize;

    let mut step = 0;
    while step < simulation_steps_usize {
        let chunk_size = MAX_CHUNK_SIZE;
        let chunk_end = std::cmp::min(step + chunk_size, simulation_steps_usize);

        info!("Processing steps {}-{}", step, chunk_end - 1);

        let mut transactions = Vec::new();
        for i in step..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &Verifier1Instruction::Execute(i as u32),
                vec![AccountMeta::new(account.pubkey(), false)],
            );
            let tx = Transaction::new_signed_with_payer(
                &[limit_instructions.clone(), execute_ix],
                Some(&payer.pubkey()),
                &[payer],
                client.get_latest_blockhash().await?,
            );
            transactions.push(tx);
        }

        send_and_confirm_transactions(client, &transactions).await?;
        info!("Chunk {}-{} completed", step, chunk_end - 1);

        step = chunk_end;
    }
    Ok(())
}

mod prepare_input {
    use std::fs;
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;

    /// Prepare initial data for Stage 1
    /// Only Stage 1 needs initial data setup - all other stages use ping-pong copying
    pub fn get_bytes_stage1(proof_path: &str) -> Vec<u8> {
        let proof_str = fs::read_to_string(proof_path)
            .unwrap_or_else(|e| panic!("Failed to read proof file '{}': {}", proof_path, e));
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(&proof_str)
            .unwrap_or_else(|e| panic!("Failed to parse proof JSON from '{}': {}", proof_path, e));
        let proof = StarkProofParser::try_from(proof_json)
            .unwrap_or_else(|e| panic!("Failed to transform proof from '{}': {:?}", proof_path, e));
        let proof_verifier = proof.transform_to();

        let mut stack = Verifier1StackAccount::default();
        stack.proof = proof_verifier.clone();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }
}
