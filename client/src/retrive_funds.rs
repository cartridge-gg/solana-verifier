use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    signature::Keypair,
    signer::{EncodableKey, Signer},
    transaction::Transaction,
};
use solana_system_interface::program::ID as SYSTEM_PROGRAM_ID;
use verifier::instruction::VerifierInstruction;

use crate::{initialize_client, setup_payer, Config, Result};
use log::info;
#[allow(clippy::result_large_err)]
pub async fn retrive_funds(config: &Config) -> Result<()> {
    let client = initialize_client(config).await?;
    let payer = if let Some(ref payer_keypair) = config.payer_keypair {
        Keypair::from_base58_string(payer_keypair)
    } else {
        setup_payer(&client, config).await?
    };
    info!(public_key:% = payer.pubkey(); "Using payer");

    let program_keypair = Keypair::read_from_file("keypairs/verifier-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();

    info!(program_id:% = program_id; "Using program");
    let stack_account = Keypair::read_from_file("keypairs/stack-account-keypair.json").unwrap();

    info!("Closing account");

    let balance = client.get_balance(&payer.pubkey()).await?;
    let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    info!(balance_sol:% = balance_sol; "Balance");

    let close_account_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::Close,
        vec![
            AccountMeta::new(stack_account.pubkey(), true),
            AccountMeta::new(payer.pubkey(), false),
            AccountMeta::new(SYSTEM_PROGRAM_ID, false),
        ],
    );

    let close_account_tx = Transaction::new_signed_with_payer(
        &[close_account_ix],
        Some(&payer.pubkey()),
        &[&stack_account, &payer],
        client.get_latest_blockhash().await?,
    );
    let close_account_signature = client
        .send_and_confirm_transaction(&close_account_tx)
        .await?;

    info!(signature:% = close_account_signature; "Account closed successfully");

    let balance = client.get_balance(&payer.pubkey()).await?;
    let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    info!(balance_sol:% = balance_sol; "Balance after closing");

    Ok(())
}
