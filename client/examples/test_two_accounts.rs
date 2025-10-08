use client::{initialize_client, setup_payer, setup_program, Config};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use std::{mem::size_of, path::Path};
use utils::UniversalStackAccount;
use verifier_1::instruction::VerifierInstruction as Verifier1Instruction;
use verifier_2::instruction::VerifierInstruction as Verifier2Instruction;

#[tokio::main]
async fn main() -> client::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("client", log::LevelFilter::Trace)
        .init();

    println!("\n=== Two Accounts Test: Verifier1 → Verifier2 ===\n");

    let config = Config::parse_args();
    let client = initialize_client(&config).await?;
    let payer = setup_payer(&client, &config).await?;

    // Setup programs
    let verifier1_path = Path::new("target/deploy/verifier_1.so");
    let verifier1_program_id = setup_program(&client, &payer, &config, verifier1_path).await?;
    println!("Verifier1 program ID: {}", verifier1_program_id);

    let verifier2_path = Path::new("target/deploy/verifier_2.so");
    let verifier2_program_id = setup_program(&client, &payer, &config, verifier2_path).await?;
    println!("Verifier2 program ID: {}", verifier2_program_id);

    let space = size_of::<UniversalStackAccount>();
    println!("\nAccount space: {} bytes", space);

    // Create account1 (owned by verifier1)
    let account1 = Keypair::new();
    println!("\nCreating account1: {} (owner: verifier1)", account1.pubkey());

    let create_account1_ix = create_account(
        &payer.pubkey(),
        &account1.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        &verifier1_program_id,
    );

    let create_tx1 = Transaction::new_signed_with_payer(
        &[create_account1_ix],
        Some(&payer.pubkey()),
        &[&payer, &account1],
        client.get_latest_blockhash().await?,
    );
    client.send_and_confirm_transaction(&create_tx1).await?;
    println!("✓ Account1 created (owner: verifier1)");

    // Create account2 (owned by verifier2)
    let account2 = Keypair::new();
    println!("\nCreating account2: {} (owner: verifier2)", account2.pubkey());

    let create_account2_ix = create_account(
        &payer.pubkey(),
        &account2.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        &verifier2_program_id,
    );

    let create_tx2 = Transaction::new_signed_with_payer(
        &[create_account2_ix],
        Some(&payer.pubkey()),
        &[&payer, &account2],
        client.get_latest_blockhash().await?,
    );
    client.send_and_confirm_transaction(&create_tx2).await?;
    println!("✓ Account2 created (owner: verifier2)");

    // Initialize account1 with test data
    let init_data: [u64; 2] = [0, 65536];
    let init_bytes = unsafe {
        std::slice::from_raw_parts(
            init_data.as_ptr() as *const u8,
            std::mem::size_of_val(&init_data)
        )
    };

    let init_ix = Instruction::new_with_borsh(
        verifier1_program_id,
        &Verifier1Instruction::SetAccountData(0, init_bytes.to_vec()),
        vec![AccountMeta::new(account1.pubkey(), false)],
    );

    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );
    client.send_and_confirm_transaction(&init_tx).await?;
    println!("✓ Account1 initialized with test data");

    // STEP 1: Verifier1 modifies account1 (owns it)
    println!("\n=== STEP 1: Verifier1 modifies account1 ===");

    let test_ix1 = Instruction::new_with_borsh(
        verifier1_program_id,
        &Verifier1Instruction::TestExecute,
        vec![AccountMeta::new(account1.pubkey(), false)],
    );

    let test_tx1 = Transaction::new_signed_with_payer(
        &[test_ix1],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    match client.send_and_confirm_transaction(&test_tx1).await {
        Ok(sig) => {
            println!("✓ Verifier1 successfully modified account1!");
            println!("  Transaction: {}", sig);
        }
        Err(e) => {
            println!("✗ Verifier1 failed: {:?}", e);
            return Err(e.into());
        }
    }

    // Read account1 data
    let account1_data = client.get_account_data(&account1.pubkey()).await?;
    let front_index = u64::from_le_bytes(account1_data[0..8].try_into().unwrap());
    println!("  Account1 front_index after verifier1: {}", front_index);

    // STEP 2: Verifier2 copies from account1 to account2
    println!("\n=== STEP 2: Verifier2 copies account1 → account2 ===");

    let copy_ix = Instruction::new_with_borsh(
        verifier2_program_id,
        &Verifier2Instruction::CopyFromAccount,
        vec![
            AccountMeta::new_readonly(account1.pubkey(), false), // Source (owned by verifier1)
            AccountMeta::new(account2.pubkey(), false),          // Dest (owned by verifier2)
        ],
    );

    let copy_tx = Transaction::new_signed_with_payer(
        &[copy_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    match client.send_and_confirm_transaction(&copy_tx).await {
        Ok(sig) => {
            println!("✓ Verifier2 successfully copied account1 → account2!");
            println!("  Transaction: {}", sig);
        }
        Err(e) => {
            println!("✗ Copy failed: {:?}", e);
            return Err(e.into());
        }
    }

    // Verify data was copied
    let account2_data = client.get_account_data(&account2.pubkey()).await?;
    let front_index2 = u64::from_le_bytes(account2_data[0..8].try_into().unwrap());
    println!("  Account2 front_index after copy: {}", front_index2);

    if front_index == front_index2 {
        println!("\n✓✓✓ TWO ACCOUNTS TEST PASSED! ✓✓✓");
        println!("Verifier2 successfully read from account1 (owned by verifier1)");
        println!("and copied to account2 (owned by verifier2)!");
    } else {
        println!("\n✗✗✗ TEST FAILED ✗✗✗");
        println!("Data mismatch: {} != {}", front_index, front_index2);
    }

    println!("\nAccount1 (verifier1 owner): {}", account1.pubkey());
    println!("Account2 (verifier2 owner): {}", account2.pubkey());

    Ok(())
}
