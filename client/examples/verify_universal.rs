/// Universal Stack Account Verification Example - ON-CHAIN (PING-PONG)
///
/// This example demonstrates the PING-PONG architecture with 2 accounts alternating
/// ownership between 4 verifier programs. Data is copied between accounts as ownership transfers.
use client::{
    initialize_client, send_and_confirm_transactions, setup_payer, setup_program, Config,
};
use felt::Felt;
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

fn main() {
    let result = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .thread_stack_size(32 * 1024 * 1024)
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async_main())
        })
        .unwrap()
        .join()
        .unwrap();

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

#[allow(clippy::result_large_err)]
async fn async_main() -> client::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("client", log::LevelFilter::Info)
        .init();

    let config = Config::parse_args();
    let client = initialize_client(&config).await?;
    let payer = setup_payer(&client, &config).await?;

    println!("\n========================================");
    println!("  PING-PONG Verifier - 2 Accounts");
    println!("========================================\n");

    // Deploy ALL 4 verifier programs
    println!("Deploying programs...");

    let verifier1_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_1.so"),
    )
    .await?;
    println!("✓ Verifier 1: {}", verifier1_program_id);

    let verifier2_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_2.so"),
    )
    .await?;
    println!("✓ Verifier 2: {}", verifier2_program_id);

    let verifier3_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_3.so"),
    )
    .await?;
    println!("✓ Verifier 3: {}", verifier3_program_id);

    let verifier4_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_4.so"),
    )
    .await?;
    println!("✓ Verifier 4: {}", verifier4_program_id);

    // Create TWO accounts for ping-pong
    let account1 = Keypair::new();
    let account2 = Keypair::new();

    let space = size_of::<Verifier1StackAccount>();
    println!(
        "\n✓ Creating account1: {} (owner: verifier1)",
        account1.pubkey()
    );
    println!("  Account size: {} bytes", space);
    create_account_tx(&client, &payer, &account1, space, &verifier1_program_id).await?;

    println!(
        "\n✓ Creating account2: {} (owner: verifier2)",
        account2.pubkey()
    );
    create_account_tx(&client, &payer, &account2, space, &verifier2_program_id).await?;

    // ========== STAGE 1: Verifier1 on Account1 ==========
    println!("\n========== STAGE 1: Verifier1 on Account1 ==========");

    // Prepare initial data for Verifier1 (only needed for Stage 1)
    let stack_bytes = prepare_input::get_bytes_stage1();
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
    println!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier1_program_id,
        &account1,
        simulation_steps as u32,
    )
    .await?;

    println!("✓ Stage 1 completed on account1");

    // ========== STAGE 2: Verifier2 on Account2 (PING-PONG: copy account1 → account2) ==========
    println!("\n========== STAGE 2: Verifier2 on Account2 ==========");

    // PING-PONG: Copy all data from account1 to account2
    println!("  Copying account1 → account2...");
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
    println!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier2_program_id,
        &account2,
        simulation_steps as u32,
    )
    .await?;

    println!("✓ Stage 2 completed on account2");

    // ========== STAGE 3: Verifier3 on Account1 (TRANSFER ownership account1: verifier1 → verifier3) ==========
    println!("\n========== STAGE 3: Verifier3 on Account1 ==========");

    // Transfer ownership of account1 from verifier1 to verifier3
    println!("  Transferring ownership account1: verifier1 → verifier3...");
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
    println!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier3_program_id,
        &account1,
        simulation_steps as u32,
    )
    .await?;

    println!("✓ Stage 3 completed on account1 (owner: verifier3)");

    // ========== STAGE 4: Verifier4 on Account2 (TRANSFER ownership account2: verifier2 → verifier4) ==========
    println!("\n========== STAGE 4: Verifier4 on Account2 ==========");

    // Transfer ownership of account2 from verifier2 to verifier4
    println!("  Transferring ownership account2: verifier2 → verifier4...");
    transfer_ownership(
        &client,
        &payer,
        &verifier2_program_id,
        &account2,
        &verifier4_program_id,
    )
    .await?;

    // PING-PONG: Copy all data from account1 to account2
    println!("  Copying account1 → account2...");
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
    println!("  Steps in simulation: {}", simulation_steps);

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

    println!("\n========== VERIFICATION RESULTS ==========");
    println!("  Program Hash: {:?}", result_program_hash);
    println!("  Output Hash:  {:?}", result_output_hash);

    // Verify expected values
    assert_eq!(
        result_program_hash,
        Felt::from_hex("0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07").unwrap(),
        "Program hash mismatch"
    );
    assert_eq!(
        result_output_hash,
        Felt::from_hex("0x3233b5615a8de5563f7d3ba086b8f260189ac47753a1c131d063ed3f6c24400")
            .unwrap(),
        "Output hash mismatch"
    );

    println!("\n✓ All 4 stages completed successfully using PING-PONG architecture!");
    println!("✓ All verifications passed!");
    println!("✓ PING-PONG verifier test completed on Solana!");
    println!("\nFinal owners:");
    println!("  Account1: {} (owner: verifier3)", account1.pubkey());
    println!("  Account2: {} (owner: verifier4)", account2.pubkey());

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
    println!("  Account created successfully");
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
    println!("  ✓ Copied successfully");
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
    println!("  ✓ Ownership transferred successfully");
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
    println!("  Account data set successfully");
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
    println!("  Task pushed successfully");
    Ok(())
}

async fn execute_verifier(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    simulation_steps: u32,
) -> client::Result<()> {
    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
    let simulation_steps_usize = simulation_steps as usize;

    let mut step = 0;
    while step < simulation_steps_usize {
        let mut chunk_size = MAX_CHUNK_SIZE;
            if step >= 24000 { chunk_size = 100; }
            if step >= 25900 { chunk_size = 1; }
        let chunk_end = std::cmp::min(step + chunk_size, simulation_steps_usize);

        println!("Processing steps {}-{}", step, chunk_end - 1);

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
        println!("Chunk {}-{} completed", step, chunk_end - 1);

        step = chunk_end;
    }
    Ok(())
}

mod prepare_input {
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;

    /// Prepare initial data for Stage 1
    /// Only Stage 1 needs initial data setup - all other stages use ping-pong copying
    pub fn get_bytes_stage1() -> Vec<u8> {
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();

        println!("DEBUG: last_layer_coefficients.len() = {}", proof_verifier.unsent_commitment.fri.last_layer_coefficients.len());
        println!("DEBUG: log_last_layer_degree_bound = {}", proof_verifier.config.fri.log_last_layer_degree_bound);

        let mut stack = Verifier1StackAccount::default();
        stack.proof = proof_verifier.clone();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();

        println!("DEBUG: After assignment - stack.proof.unsent_commitment.fri.last_layer_coefficients.len() = {}",
            stack.proof.unsent_commitment.fri.last_layer_coefficients.len());

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }
}
