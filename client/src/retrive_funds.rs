use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    signature::Keypair,
    signer::{EncodableKey, Signer},
    transaction::Transaction,
};
use solana_system_interface::program::ID as SYSTEM_PROGRAM_ID;
use verifier_1::instruction::VerifierInstruction as Verifier1Instruction;
use verifier_2::instruction::VerifierInstruction as Verifier2Instruction;
use verifier_3::instruction::VerifierInstruction as Verifier3Instruction;
use verifier_4::instruction::VerifierInstruction as Verifier4Instruction;

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

    let program_keypair_1 = Keypair::read_from_file("keypairs/verifier_1-keypair.json").unwrap();
    let program_id_1 = program_keypair_1.pubkey();

    info!(program_id:% = program_id_1; "Using program");
    let stack_account_1 =
        Keypair::read_from_file("keypairs/verifier-1-account-keypair.json").unwrap();

    info!("Loaded verifier-1 account");
    let program_keypair_2 = Keypair::read_from_file("keypairs/verifier_2-keypair.json").unwrap();
    let program_id_2 = program_keypair_2.pubkey();

    info!(program_id:% = program_id_2; "Using program");
    let stack_account_2 =
        Keypair::read_from_file("keypairs/verifier-2-account-keypair.json").unwrap();

    info!("Loaded verifier-2 account");

    let program_keypair_3 = Keypair::read_from_file("keypairs/verifier_3-keypair.json").unwrap();
    let program_id_3 = program_keypair_3.pubkey();

    info!(program_id:% = program_id_3; "Using program");
    let stack_account_3 =
        Keypair::read_from_file("keypairs/verifier-3-account-keypair.json").unwrap();

    info!("Loaded verifier-3 account");

    let program_keypair_4 = Keypair::read_from_file("keypairs/verifier_4-keypair.json").unwrap();
    let program_id_4 = program_keypair_4.pubkey();

    info!(program_id:% = program_id_4; "Using program");
    let stack_account_4 =
        Keypair::read_from_file("keypairs/verifier-4-account-keypair.json").unwrap();

    info!("Loaded verifier-4 account");

    let balance = client.get_balance(&payer.pubkey()).await?;
    let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    info!(balance_sol:% = balance_sol; "Balance");

    close_account(
        &client,
        &payer,
        program_id_1,
        stack_account_1,
        Verifier1Instruction::Close,
    )
    .await?;
    close_account(
        &client,
        &payer,
        program_id_2,
        stack_account_2,
        Verifier2Instruction::Close,
    )
    .await?;
    close_account(
        &client,
        &payer,
        program_id_3,
        stack_account_3,
        Verifier3Instruction::Close,
    )
    .await?;
    close_account(
        &client,
        &payer,
        program_id_4,
        stack_account_4,
        Verifier4Instruction::Close,
    )
    .await?;

    Ok(())
}

async fn close_account<T: borsh::BorshSerialize>(
    client: &solana_client::nonblocking::rpc_client::RpcClient,
    payer: &Keypair,
    program_id: solana_sdk::pubkey::Pubkey,
    stack_account: Keypair,
    close: T,
) -> Result<()> {
    let close_account_ix = Instruction::new_with_borsh(
        program_id,
        &close,
        vec![
            AccountMeta::new(stack_account.pubkey(), true),
            AccountMeta::new(payer.pubkey(), false),
            AccountMeta::new(SYSTEM_PROGRAM_ID, false),
        ],
    );
    let close_account_tx = Transaction::new_signed_with_payer(
        &[close_account_ix],
        Some(&payer.pubkey()),
        &[&stack_account, payer],
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
