use std::path::Path;

use crate::{initialize_client, setup_payer, setup_program, Config, Result};
use log::info;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signature::{write_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};

use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;
use verifier_2::state::BidirectionalStackAccount as Verifier2StackAccount;

pub async fn deploy(config: &Config) -> Result<()> {
    let client = initialize_client(config).await?;
    let payer = if let Some(ref payer_keypair) = config.payer_keypair {
        Keypair::from_base58_string(payer_keypair)
    } else {
        setup_payer(&client, config).await?
    };

    let balance = client.get_balance(&payer.pubkey()).await?;
    let balance_sol_before_deployment = balance as f64 / LAMPORTS_PER_SOL as f64;
    info!(balance_sol:% = balance_sol_before_deployment; "Balance");
    info!(public_key:% = payer.pubkey(); "Using payer");

    let verifier_1_program_path = Path::new("target/deploy/verifier_1.so");
    let verifier_2_program_path = Path::new("target/deploy/verifier_2.so");
    let verifier_3_program_path = Path::new("target/deploy/verifier_3.so");
    let verifier_4_program_path = Path::new("target/deploy/verifier_4.so");

    let verifier_1_program_id =
        setup_program(&client, &payer, config, verifier_1_program_path).await?;
    info!(program_id:% = verifier_1_program_id; "Verifier 1 program id");

    let verifier_2_program_id =
        setup_program(&client, &payer, config, verifier_2_program_path).await?;
    info!(program_id:% = verifier_2_program_id; "Verifier 2 program id");

    let verifier_3_program_id =
        setup_program(&client, &payer, config, verifier_3_program_path).await?;
    info!(program_id:% = verifier_3_program_id; "Verifier 3 program id");

    let verifier_4_program_id =
        setup_program(&client, &payer, config, verifier_4_program_path).await?;
    info!(program_id:% = verifier_4_program_id; "Verifier 4 program id");

    let space = size_of::<Verifier1StackAccount>();
    deploy_verifier(
        config,
        &client,
        &payer,
        verifier_1_program_id,
        space,
        "verifier-1",
    )
    .await?;

    let space = size_of::<Verifier2StackAccount>();
    deploy_verifier(
        config,
        &client,
        &payer,
        verifier_2_program_id,
        space,
        "verifier-2",
    )
    .await?;

    let balance = client.get_balance(&payer.pubkey()).await?;
    let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;
    info!(balance_sol:% = balance_sol; "Balance after deployment");
    info!(total_cost:% = balance_sol_before_deployment - balance_sol; "Total cost of deployment (SOL)");
    Ok(())
}

async fn deploy_verifier(
    config: &Config,
    client: &solana_client::nonblocking::rpc_client::RpcClient,
    payer: &Keypair,
    program_id: solana_sdk::pubkey::Pubkey,
    space: usize,
    name: &str,
) -> Result<()> {
    let verifier_account = Keypair::new();
    write_keypair_file(
        &verifier_account,
        config
            .keypairs_dir
            .join(format!("{}-account-keypair.json", name)),
    )
    .unwrap();
    info!(public_key:% = verifier_account.pubkey(); "Creating new account");
    info!(size_in_bytes:% = space; "Account space");
    let signature = create_account(client, payer, program_id, verifier_account, space).await?;
    info!(signature:% = signature; "Account created successfully");
    Ok(())
}

async fn create_account(
    client: &solana_client::nonblocking::rpc_client::RpcClient,
    payer: &Keypair,
    program_id: solana_sdk::pubkey::Pubkey,
    stack_account: Keypair,
    space: usize,
) -> Result<solana_sdk::signature::Signature> {
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
        &[payer, &stack_account],
        client.get_latest_blockhash().await?,
    );
    let signature = client
        .send_and_confirm_transaction(&create_account_tx)
        .await?;
    Ok(signature)
}
