use client::{initialize_client, setup_payer, setup_program, Config};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use std::{mem::size_of, path::Path};
use verifier_1::instruction::VerifierInstruction as Verifier1Instruction;
use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;
use verifier_2::instruction::VerifierInstruction as Verifier2Instruction;
use verifier_2::state::BidirectionalStackAccount as Verifier2StackAccount;

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

    let space = size_of::<Verifier1StackAccount>();
    println!("\nAccount space: {} bytes", space);

    // Create account1 (owned by verifier1)
    let account1 = Keypair::new();
    println!(
        "\nCreating account1: {} (owner: verifier1)",
        account1.pubkey()
    );

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
    println!(
        "\nCreating account2: {} (owner: verifier2)",
        account2.pubkey()
    );

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
    // Structure layout: mode(1 byte) + padding(7 bytes) + front_index(8 bytes) + back_index(8 bytes)
    // We want: mode=0, front_index=100, back_index=CAPACITY (65536)
    let mut init_data = vec![0u8; 24]; // mode + padding + front_index + back_index
    init_data[0] = 0; // mode = 0
    init_data[8..16].copy_from_slice(&100_u64.to_le_bytes()); // front_index = 100
    init_data[16..24].copy_from_slice(&65536_u64.to_le_bytes()); // back_index = CAPACITY

    let init_ix = Instruction::new_with_borsh(
        verifier1_program_id,
        &Verifier1Instruction::SetAccountData(0, init_data),
        vec![AccountMeta::new(account1.pubkey(), false)],
    );

    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );
    client.send_and_confirm_transaction(&init_tx).await?;
    println!("✓ Account1 initialized with test data (front_index=100)");

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

    // Read account1 data and verify owner
    let account1_info = client.get_account(&account1.pubkey()).await?;
    assert_eq!(
        account1_info.owner, verifier1_program_id,
        "Account1 should still be owned by verifier1"
    );

    let account1_data = client.get_account_data(&account1.pubkey()).await?;
    // front_index is at offset 8 (after mode field + padding)
    let front_index = u64::from_le_bytes(account1_data[8..16].try_into().unwrap());
    println!("  Account1 front_index after verifier1: {}", front_index);
    println!("  Account1 owner verified: verifier1");

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

    // Verify data was copied and owners are correct
    let account2_info = client.get_account(&account2.pubkey()).await?;
    assert_eq!(
        account2_info.owner, verifier2_program_id,
        "Account2 should be owned by verifier2"
    );

    let account2_data = client.get_account_data(&account2.pubkey()).await?;
    // front_index is at offset 8
    let front_index2 = u64::from_le_bytes(account2_data[8..16].try_into().unwrap());
    println!("  Account2 front_index after copy: {}", front_index2);
    println!("  Account2 owner verified: verifier2");

    assert_eq!(front_index, front_index2, "Data should match after copy");
    println!("✓ Copy test passed! Data matches: {}", front_index);

    // STEP 3: Transfer ownership of account1 from verifier1 to verifier2
    println!("\n=== STEP 3: Transfer ownership account1: verifier1 → verifier2 ===");
    println!("NOTE: Transfer will ZERO the account data!");

    // Verify current owner
    let account1_info = client.get_account(&account1.pubkey()).await?;
    println!("Account1 current owner: {}", account1_info.owner);
    println!("Verifier1 program ID: {}", verifier1_program_id);

    let transfer_ix = Instruction::new_with_borsh(
        verifier1_program_id,
        &Verifier1Instruction::TransferOwnership,
        vec![
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new_readonly(verifier2_program_id, false),
        ],
    );

    let transfer_tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    match client.send_and_confirm_transaction(&transfer_tx).await {
        Ok(sig) => {
            println!("✓ Ownership transferred successfully!");
            println!("  Transaction: {}", sig);
        }
        Err(e) => {
            println!("✗ Ownership transfer failed: {:?}", e);
            return Err(e.into());
        }
    }

    // ASSERT: Verify new owner
    let account1_info_after = client.get_account(&account1.pubkey()).await?;
    println!("Account1 new owner: {}", account1_info_after.owner);

    assert_eq!(
        account1_info_after.owner, verifier2_program_id,
        "Account1 should now be owned by verifier2 after transfer"
    );
    println!("✓ Ownership verified: account1 now owned by verifier2!");

    // ASSERT: Verify account1 is now zeroed (consequence of resize(0))
    let account1_after_transfer = client.get_account_data(&account1.pubkey()).await?;
    let zeroed_front_index = u64::from_le_bytes(account1_after_transfer[8..16].try_into().unwrap());
    println!(
        "  Account1 front_index after transfer (should be 0): {}",
        zeroed_front_index
    );

    assert_eq!(
        zeroed_front_index, 0,
        "Account1 should be zeroed after ownership transfer"
    );

    // STEP 4: Verifier2 copies data back from account2 → account1 (PING-PONG!)
    println!("\n=== STEP 4: Verifier2 copies data back: account2 → account1 ===");
    println!("This is the PING-PONG: restoring data after ownership transfer");

    let copy_back_ix = Instruction::new_with_borsh(
        verifier2_program_id,
        &Verifier2Instruction::CopyFromAccount,
        vec![
            AccountMeta::new_readonly(account2.pubkey(), false), // Source (has data)
            AccountMeta::new(account1.pubkey(), false), // Dest (zeroed, now owned by verifier2)
        ],
    );

    let copy_back_tx = Transaction::new_signed_with_payer(
        &[copy_back_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    match client.send_and_confirm_transaction(&copy_back_tx).await {
        Ok(sig) => {
            println!("✓ Data copied back: account2 → account1!");
            println!("  Transaction: {}", sig);
        }
        Err(e) => {
            println!("✗ Copy back failed: {:?}", e);
            return Err(e.into());
        }
    }

    // ASSERT: Verify data was restored correctly
    let account1_info_restored = client.get_account(&account1.pubkey()).await?;
    assert_eq!(
        account1_info_restored.owner, verifier2_program_id,
        "Account1 should still be owned by verifier2"
    );

    let account1_restored = client.get_account_data(&account1.pubkey()).await?;
    let restored_front_index = u64::from_le_bytes(account1_restored[8..16].try_into().unwrap());
    println!(
        "  Account1 front_index after restore: {}",
        restored_front_index
    );
    println!("  Account1 owner still verified: verifier2");

    assert_eq!(
        restored_front_index, front_index2,
        "Data should be restored to match account2 after PING-PONG copy"
    );
    println!("✓ Data successfully restored to account1!");

    // STEP 5: Verifier2 modifies account1 (now has data and correct ownership)
    println!("\n=== STEP 5: Verifier2 modifies account1 (final step) ===");

    let test_ix2 = Instruction::new_with_borsh(
        verifier2_program_id,
        &Verifier2Instruction::TestExecute,
        vec![AccountMeta::new(account1.pubkey(), false)],
    );

    let test_tx2 = Transaction::new_signed_with_payer(
        &[test_ix2],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    match client.send_and_confirm_transaction(&test_tx2).await {
        Ok(sig) => {
            println!("✓ Verifier2 successfully modified account1!");
            println!("  Transaction: {}", sig);
        }
        Err(e) => {
            println!("✗ Verifier2 modification failed: {:?}", e);
            return Err(e.into());
        }
    }

    // ASSERT: Verify final state and modification
    let account1_final_info = client.get_account(&account1.pubkey()).await?;
    assert_eq!(
        account1_final_info.owner, verifier2_program_id,
        "Account1 should be owned by verifier2 at the end"
    );

    let account1_final = client.get_account_data(&account1.pubkey()).await?;
    let final_front_index = u64::from_le_bytes(account1_final[8..16].try_into().unwrap());
    println!(
        "  Account1 front_index after verifier2 modify: {}",
        final_front_index
    );
    println!("  Account1 owner still verified: verifier2");

    assert_eq!(
        final_front_index,
        restored_front_index + 1,
        "Verifier2 should have incremented front_index by 1"
    );
    println!("✓ Account1 was successfully modified by new owner (verifier2)!");

    // FINAL ASSERTIONS: Verify complete final state
    println!("\n=== FINAL STATE VERIFICATION ===");

    let final_account1 = client.get_account(&account1.pubkey()).await?;
    let final_account2 = client.get_account(&account2.pubkey()).await?;

    assert_eq!(
        final_account1.owner, verifier2_program_id,
        "Final: Account1 should be owned by verifier2"
    );
    assert_eq!(
        final_account2.owner, verifier2_program_id,
        "Final: Account2 should be owned by verifier2"
    );

    assert_eq!(
        final_account1.data.len(),
        space,
        "Final: Account1 should maintain correct size"
    );
    assert_eq!(
        final_account2.data.len(),
        space,
        "Final: Account2 should maintain correct size"
    );

    let final_account1_data = client.get_account_data(&account1.pubkey()).await?;
    let final_account2_data = client.get_account_data(&account2.pubkey()).await?;
    let final_index1 = u64::from_le_bytes(final_account1_data[8..16].try_into().unwrap());
    let final_index2 = u64::from_le_bytes(final_account2_data[8..16].try_into().unwrap());

    println!(
        "✓ Account1: owner=verifier2, size={} bytes, front_index={}",
        space, final_index1
    );
    println!(
        "✓ Account2: owner=verifier2, size={} bytes, front_index={}",
        space, final_index2
    );

    // Summary
    println!("\n=== PING-PONG TEST SUMMARY ===");
    println!("✓ Step 1: Verifier1 modified account1 (original owner)");
    println!("✓ Step 2: Verifier2 copied account1 → account2 (cross-program read)");
    println!("✓ Step 3: Ownership transferred account1: verifier1 → verifier2 (data zeroed)");
    println!("✓ Step 4: Verifier2 copied account2 → account1 (PING-PONG restore!)");
    println!("✓ Step 5: Verifier2 modified account1 (new owner, restored data)");
    println!("\n✓✓✓ ALL ASSERTIONS PASSED! PING-PONG MECHANISM WORKS! ✓✓✓");
    println!("\nAccount1 pubkey: {}", account1.pubkey());
    println!("Account2 pubkey: {}", account2.pubkey());

    Ok(())
}
