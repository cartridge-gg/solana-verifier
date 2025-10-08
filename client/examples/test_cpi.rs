use client::{initialize_client, setup_payer, setup_program, Config};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use std::{mem::size_of, path::Path};
use utils::UniversalStackAccount;
use universal_verifier::UniversalVerifierInstruction;

#[tokio::main]
async fn main() -> client::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("client", log::LevelFilter::Trace)
        .init();

    println!("\n=== CPI Test: Can verifier1 modify account owned by universal_verifier? ===\n");

    let config = Config::parse_args();
    let client = initialize_client(&config).await?;
    let payer = setup_payer(&client, &config).await?;

    // Setup universal_verifier program
    let universal_program_path = Path::new("target/deploy/universal_verifier.so");
    let universal_program_id = setup_program(&client, &payer, &config, universal_program_path).await?;
    println!("Universal verifier program ID: {}", universal_program_id);

    // Setup verifier1 program
    let verifier1_program_path = Path::new("target/deploy/verifier_1.so");
    let verifier1_program_id = setup_program(&client, &payer, &config, verifier1_program_path).await?;
    println!("Verifier1 program ID: {}", verifier1_program_id);

    // Create stack account owned by universal_verifier
    let stack_account = Keypair::new();
    println!("\nCreating stack account: {}", stack_account.pubkey());

    let space = size_of::<UniversalStackAccount>();
    println!("Account space: {} bytes", space);

    let create_account_ix = create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        &universal_program_id, // Owned by universal_verifier!
    );

    let create_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &stack_account],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&create_account_tx).await?;
    println!("✓ Stack account created (owner: universal_verifier)");

    // Initialize the account with front_index = 0
    let init_data: [u64; 2] = [0, 65536]; // front_index=0, back_index=65536
    let init_bytes = unsafe {
        std::slice::from_raw_parts(
            init_data.as_ptr() as *const u8,
            std::mem::size_of_val(&init_data)
        )
    };

    let init_ix = Instruction::new_with_borsh(
        universal_program_id,
        &UniversalVerifierInstruction::SetAccountData(0, init_bytes.to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&init_tx).await?;
    println!("✓ Stack account initialized (front_index = 0)");

    // Now the TEST: Call universal_verifier::TestCPI which will call verifier1::TestExecute
    println!("\n=== Running CPI Test ===");
    println!("universal_verifier will call verifier1 via CPI");
    println!("verifier1 will try to increment front_index");
    println!("Account owner: universal_verifier");
    println!("Account modifier: verifier1 (via CPI)\n");

    let test_cpi_ix = Instruction::new_with_borsh(
        universal_program_id,
        &UniversalVerifierInstruction::TestCPI(verifier1_program_id.to_bytes()),
        vec![
            AccountMeta::new(stack_account.pubkey(), true),  // Changed to TRUE - stack_account is signer!
            AccountMeta::new_readonly(verifier1_program_id, false),
        ],
    );

    let test_tx = Transaction::new_signed_with_payer(
        &[test_cpi_ix],
        Some(&payer.pubkey()),
        &[&payer, &stack_account],  // Added stack_account as signer!
        client.get_latest_blockhash().await?,
    );

    match client.send_and_confirm_transaction(&test_tx).await {
        Ok(sig) => {
            println!("✓✓✓ CPI TEST PASSED! ✓✓✓");
            println!("Transaction: {}", sig);
            println!("\nResult: verifier1 CAN modify account owned by universal_verifier via CPI!");
            println!("The Solana documentation was correct.");
        }
        Err(e) => {
            println!("✗✗✗ CPI TEST FAILED ✗✗✗");
            println!("Error: {:?}", e);
            println!("\nResult: verifier1 CANNOT modify account owned by universal_verifier via CPI");
            println!("This contradicts the Solana documentation.");
        }
    }

    Ok(())
}
