use std::path::Path;

use solana_sdk::{
    signature::{write_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use verifier::state::BidirectionalStackAccount;

use crate::{initialize_client, setup_payer, setup_program, Config, Result};

pub async fn deploy(config: &Config) -> Result<()> {
    let client = initialize_client(config).await?;
    let payer = if let Some(ref payer_keypair) = config.payer_keypair {
        Keypair::from_base58_string(payer_keypair)
    } else {
        setup_payer(&client, config).await?
    };
    println!("Using payer: {}", payer.pubkey());

    let program_path = Path::new("target/deploy/verifier.so");
    let program_id = setup_program(&client, &payer, config, program_path).await?;
    println!("Using program ID: {program_id}");

    let stack_account = Keypair::new();
    write_keypair_file(
        &stack_account,
        config.keypairs_dir.join("stack-account-keypair.json"),
    )
    .unwrap();
    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<BidirectionalStackAccount>();
    println!("Account space: {space} bytes");

    let create_account_ix = solana_system_interface::instruction::create_account(
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
    Ok(())
}
