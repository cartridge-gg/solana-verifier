#![allow(deprecated)]

use futures::future::join_all;
use solana_program::{
    bpf_loader_upgradeable,
    instruction::{AccountMeta, Instruction},
    system_instruction,
};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    bpf_loader_upgradeable::UpgradeableLoaderState,
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};

use std::{fs, path::Path, thread::sleep};

use crate::{ClientError, Config, Result};
use log::{error, info, trace, warn};
/// Initialize the Solana RPC client and verify connection
pub async fn initialize_client(config: &Config) -> Result<RpcClient> {
    info!(rpc_url:% = config.rpc_url;"Using RPC URL");

    let client = RpcClient::new_with_timeout_and_commitment(
        config.rpc_url.clone(),
        config.rpc_timeout_duration(),
        CommitmentConfig::confirmed(),
    );

    // Verify connection to validator
    client
        .get_version()
        .await
        .map(|version| {
            info!(solana_version:% = version.solana_core;
                "Connected to Solana"
            );
            client
        })
        .map_err(|err| {
            ClientError::ConnectionError(format!(
                "{err}.\nPlease ensure a local validator is running with 'solana-test-validator'"
            ))
        })
}

/// Setup the payer account, creating a new one and funding it if necessary
pub async fn setup_payer(client: &RpcClient, config: &Config) -> Result<Keypair> {
    let payer_keypair_path = config.keypairs_dir.join("payer-keypair.json");

    match read_keypair_file(&payer_keypair_path) {
        Ok(keypair) => {
            info!(public_key:% = keypair.pubkey(); "Using existing payer");
            Ok(keypair)
        }
        Err(_) => {
            let keypair = Keypair::new();

            // Ensure keypairs directory exists
            if !config.keypairs_dir.exists() {
                fs::create_dir_all(&config.keypairs_dir).map_err(ClientError::IoError)?;
            }

            write_keypair_file(&keypair, &payer_keypair_path)?;

            info!(public_key:% = keypair.pubkey(); "Created new payer");

            // Fund the account with airdrops
            request_and_confirm_airdrop(client, &keypair, config.airdrop_amount, config).await?;

            request_and_confirm_airdrop(
                client,
                &keypair,
                config.airdrop_amount * config.additional_airdrop_multiplier,
                config,
            )
            .await?;

            info!(amount:% = (config.airdrop_amount * (1 + config.additional_airdrop_multiplier)) as f64
                    / 1_000_000_000.0; "Airdropped SOL to payer");

            Ok(keypair)
        }
    }
}

/// Request an airdrop and confirm the transaction
pub async fn request_and_confirm_airdrop(
    client: &RpcClient,
    keypair: &Keypair,
    amount: u64,
    config: &Config,
) -> Result<()> {
    let message = if amount == config.airdrop_amount {
        "Airdrop"
    } else {
        "Additional airdrop"
    };
    trace!("{message} requested, waiting for confirmation...");

    let sig = client
        .request_airdrop(&keypair.pubkey(), amount)
        .await
        .map_err(|e| {
            ClientError::TransactionError(format!(
                "Failed to request {} of {} SOL: {}",
                message,
                amount as f64 / 1_000_000_000.0,
                e
            ))
        })?;

    confirm_transaction_with_retries(client, &sig, config.transaction_retry_count, config).await?;

    trace!("{message} confirmed!");
    Ok(())
}

/// Confirm a transaction with retries
pub async fn confirm_transaction_with_retries(
    client: &RpcClient,
    signature: &solana_sdk::signature::Signature,
    retries: usize,
    config: &Config,
) -> Result<()> {
    for attempt in 1..=retries {
        match client.confirm_transaction(signature).await {
            Ok(true) => return Ok(()),
            Ok(false) if attempt < retries => {
                sleep(config.retry_sleep_duration());
            }
            Ok(false) => {
                return Err(ClientError::TransactionError(format!(
                    "Transaction not confirmed after {retries} attempts"
                )));
            }
            Err(err) if attempt < retries => {
                warn!(attempt:% = attempt,retries:% = retries, error:% = err; "Confirmation attempt failed");
                sleep(config.retry_sleep_duration());
            }
            Err(err) => {
                return Err(ClientError::TransactionError(format!(
                    "Failed to confirm transaction: {err}"
                )));
            }
        }
    }

    Err(ClientError::TransactionError(format!(
        "Transaction confirmation failed after {retries} attempts"
    )))
}

/// Setup the program - either use existing deployment or deploy a new one
pub async fn setup_program(
    client: &RpcClient,
    payer: &Keypair,
    config: &Config,
    program_path: &Path,
) -> Result<solana_sdk::pubkey::Pubkey> {
    // Read the program binary

    let program_data = fs::read(program_path).map_err(ClientError::IoError)?;
    info!(size_in_bytes:% = program_data.len(); "Program binary");

    // Extract program name from path for the keypair filename
    let program_name = program_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("program");

    let program_keypair_path = config
        .keypairs_dir
        .join(format!("{program_name}-keypair.json"));

    // Deploy the program or use existing deployment
    if program_keypair_path.exists() {
        let program_keypair = read_keypair_file(&program_keypair_path)?;
        let program_id = program_keypair.pubkey();

        // Check if the program is already deployed
        match client.get_account(&program_id).await {
            Ok(_) => {
                info!(program_id:% = program_id; "Program already deployed");
                Ok(program_id)
            }
            Err(_) => {
                info!(program_id:% = program_id; "Deploying program");
                let signature =
                    deploy_program(client, payer, &program_keypair, &program_data, config).await?;
                info!(signature:% = signature; "Program deployed successfully");
                Ok(program_id)
            }
        }
    } else {
        // Create a new program deployment
        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();
        info!(program_id:% = program_id; "Deploying new program");

        let signature =
            deploy_program(client, payer, &program_keypair, &program_data, config).await?;
        info!(signature:% = signature; "Program deployed successfully");

        // Ensure keypairs directory exists
        if !config.keypairs_dir.exists() {
            fs::create_dir_all(&config.keypairs_dir).map_err(ClientError::IoError)?;
        }

        write_keypair_file(&program_keypair, &program_keypair_path)?;

        info!("Program deployed successfully!");
        Ok(program_id)
    }
}

/// Setup a program account - either use existing or create a new one
pub async fn setup_account(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    config: &Config,
    space: usize,
    account_name: &str,
) -> Result<Keypair> {
    let account_keypair_path = config
        .keypairs_dir
        .join(format!("{account_name}-keypair.json"));

    if account_keypair_path.exists() {
        let account_keypair = read_keypair_file(&account_keypair_path)?;
        info!(public_key:% = account_keypair.pubkey(); "Using existing account");
        Ok(account_keypair)
    } else {
        let account_keypair = Keypair::new();
        info!(public_key:% = account_keypair.pubkey(); "Creating account");

        // Calculate the space needed for the account
        let rent = client
            .get_minimum_balance_for_rent_exemption(space)
            .await
            .map_err(ClientError::SolanaClientError)?;

        // Create a transaction to create the account
        let create_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &account_keypair.pubkey(),
            rent,
            space as u64,
            program_id,
        );

        let blockhash = client
            .get_latest_blockhash()
            .await
            .map_err(ClientError::SolanaClientError)?;

        let create_tx = Transaction::new_signed_with_payer(
            &[create_account_ix],
            Some(&payer.pubkey()),
            &[payer, &account_keypair],
            blockhash,
        );

        // Send and confirm the transaction
        let create_sig = client
            .send_and_confirm_transaction(&create_tx)
            .await
            .map_err(|e| {
                ClientError::TransactionError(format!(
                    "Failed to send and confirm account creation transaction: {e}"
                ))
            })?;
        info!(signature:% = create_sig; "Created account");

        // Ensure keypairs directory exists
        if !config.keypairs_dir.exists() {
            fs::create_dir_all(&config.keypairs_dir).map_err(ClientError::IoError)?;
        }

        // Save the keypair for future use
        write_keypair_file(&account_keypair, &account_keypair_path)?;

        Ok(account_keypair)
    }
}

/// Send an instruction to a program
pub async fn send_instruction(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    accounts: Vec<AccountMeta>,
    instruction_data: &[u8],
) -> Result<solana_sdk::signature::Signature> {
    // Create an instruction to call the program
    let instruction = Instruction::new_with_bytes(*program_id, instruction_data, accounts);

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(ClientError::SolanaClientError)?;

    // Create a transaction with the instruction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    // Send and confirm the transaction
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .await
        .map_err(|e| {
            ClientError::TransactionError(format!("Failed to send and confirm transaction: {e}"))
        })?;

    info!(signature:% = signature; "Transaction sent");

    Ok(signature)
}

/// Function to interact with a program using a vector of instructions
///
/// This function allows direct interaction with a Solana program by providing a vector of
/// instructions to execute in a single transaction. Unlike `interact_with_program`, this function
/// doesn't require implementing the `ProgramInteraction` trait, making it more flexible for
/// simple use cases.
///
/// # Parameters
/// * `client` - The RPC client to use for the transaction
/// * `payer` - The keypair that will pay for the transaction
/// * `_program_id` - The program ID (kept for API consistency but not used directly)
/// * `_account` - The account to interact with (kept for API consistency but not used directly)
/// * `instructions` - The vector of instructions to include in the transaction
pub async fn interact_with_program_instructions(
    client: &RpcClient,
    payer: &Keypair,
    _program_id: &solana_sdk::pubkey::Pubkey, // Unused but kept for API consistency
    _account: &Keypair,                       // Unused but kept for API consistency
    instructions: &[Instruction],
) -> Result<Signature> {
    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(ClientError::SolanaClientError)?;

    // Create a transaction with the instructions
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    // Send and confirm the transaction
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .await
        .map_err(|e| {
            ClientError::TransactionError(format!("Failed to send and confirm transaction: {e}"))
        })?;

    info!(signature:% = signature; "Transaction sent");

    Ok(signature)
}

/// Deploy a program to the blockchain using BPF loader
pub async fn deploy_program(
    client: &RpcClient,
    payer: &Keypair,
    program_keypair: &Keypair,
    program_data: &[u8],
    config: &Config,
) -> Result<Signature> {
    // Calculate the buffer size needed
    let program_len = program_data.len();
    info!(size_in_bytes:% = program_len; "Program size");

    // Create a buffer account
    let buffer_keypair = Keypair::new();
    info!(public_key:% = buffer_keypair.pubkey(); "Creating buffer account");

    // Calculate rent for the buffer
    let buffer_data_len = program_len;
    let buffer_balance = client
        .get_minimum_balance_for_rent_exemption(
            buffer_data_len + UpgradeableLoaderState::size_of_buffer_metadata(),
        )
        .await
        .map_err(ClientError::SolanaClientError)?;

    // Create buffer account
    let create_buffer_ix = bpf_loader_upgradeable::create_buffer(
        &payer.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        buffer_balance,
        buffer_data_len,
    )
    .map_err(|e| ClientError::DeploymentError(e.to_string()))?;

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(ClientError::SolanaClientError)?;

    // Create and send transaction
    let create_buffer_tx = Transaction::new_signed_with_payer(
        &create_buffer_ix,
        Some(&payer.pubkey()),
        &[payer, &buffer_keypair],
        blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&create_buffer_tx)
        .await
        .map_err(|e| {
            ClientError::TransactionError(format!("Failed to create buffer account: {e}"))
        })?;
    info!(signature:% = signature; "Buffer account created");

    // Write program data to the buffer account in chunks
    write_program_to_buffer(client, payer, &buffer_keypair, program_data, config).await?;

    // Calculate rent for the program data
    let programdata_len = program_len;
    let programdata_balance = client
        .get_minimum_balance_for_rent_exemption(
            programdata_len + UpgradeableLoaderState::size_of_programdata_metadata(),
        )
        .await
        .map_err(ClientError::SolanaClientError)?;

    // Create deploy instruction
    let deploy_ix = bpf_loader_upgradeable::deploy_with_max_program_len(
        &payer.pubkey(),
        &program_keypair.pubkey(),
        &buffer_keypair.pubkey(),
        &payer.pubkey(),
        programdata_balance,
        programdata_len,
    )
    .map_err(|e| ClientError::DeploymentError(e.to_string()))?;

    // Get latest blockhash
    let blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(ClientError::SolanaClientError)?;

    // Create and send transaction
    let deploy_tx = Transaction::new_signed_with_payer(
        &deploy_ix,
        Some(&payer.pubkey()),
        &[payer, program_keypair],
        blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&deploy_tx)
        .await
        .map_err(|e| ClientError::TransactionError(format!("Failed to deploy program: {e}")))?;

    Ok(signature)
}

/// Write program data to buffer in chunks
pub async fn write_program_to_buffer(
    client: &RpcClient,
    payer: &Keypair,
    buffer_keypair: &Keypair,
    program_data: &[u8],
    config: &Config,
) -> Result<()> {
    let mut offset = 0;

    while offset < program_data.len() {
        let chunk_end = std::cmp::min(offset + config.buffer_chunk_size, program_data.len());
        let chunk = &program_data[offset..chunk_end];

        let write_ix = bpf_loader_upgradeable::write(
            &buffer_keypair.pubkey(),
            &payer.pubkey(),
            offset as u32,
            chunk.to_vec(),
        );

        // Get latest blockhash for each chunk to avoid expired blockhash issues
        let blockhash = client
            .get_latest_blockhash()
            .await
            .map_err(ClientError::SolanaClientError)?;

        let write_tx = Transaction::new_signed_with_payer(
            &[write_ix],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        );

        // Send transaction without waiting for confirmation
        let signature = client.send_transaction(&write_tx).await.map_err(|e| {
            ClientError::TransactionError(format!("Failed to send chunk at offset {offset}: {e}"))
        })?;
        trace!(offset:% = offset, signature:% = signature; "Chunk sent");
        offset = chunk_end;
    }

    // Now verify the buffer data
    info!("Verifying buffer data...");

    // Retry verification with exponential backoff
    let mut retry_count = 0;
    let max_retries = config.transaction_retry_count;
    let mut verified = false;

    while !verified && retry_count < max_retries {
        if retry_count > 0 {
            info!(attempt:% = retry_count + 1, retries:% = max_retries; "Retrying verification attempt");
            sleep(config.retry_sleep_duration());
        }

        match verify_buffer_data(client, buffer_keypair, program_data).await {
            Ok(true) => {
                verified = true;
                info!("Buffer data verified successfully!");
            }
            Ok(false) => {
                retry_count += 1;
                warn!("Buffer data verification failed, data mismatch.");
            }
            Err(e) => {
                retry_count += 1;
                error!("Buffer data verification error: {}", e);
            }
        }
    }

    if !verified {
        return Err(ClientError::DeploymentError(
            "Failed to verify buffer data after maximum retries".to_string(),
        ));
    }

    Ok(())
}

/// Verify that the buffer account contains the expected program data
async fn verify_buffer_data(
    client: &RpcClient,
    buffer_keypair: &Keypair,
    expected_data: &[u8],
) -> Result<bool> {
    // Get the buffer account data
    let account_data = client
        .get_account_data(&buffer_keypair.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    // The buffer account data starts with metadata (UpgradeableLoaderState::Buffer), followed by the program data
    // The size of the metadata is defined by size_of_buffer_metadata()
    let data_offset = UpgradeableLoaderState::size_of_buffer_metadata();

    // Make sure the account data is long enough
    if account_data.len() < data_offset + expected_data.len() {
        warn!(
            account_data_len:% = account_data.len(),
            expected_min_len:% = data_offset + expected_data.len(); "Buffer account data too short"
        );
        return Ok(false);
    }

    // Extract just the program data portion
    let buffer_data = &account_data[data_offset..data_offset + expected_data.len()];

    // Verify the content matches
    if buffer_data != expected_data {
        return Ok(false);
    }

    Ok(true)
}

/// Read a keypair from file with improved error handling
#[allow(clippy::result_large_err)]
pub fn read_keypair_file<P: AsRef<Path>>(path: P) -> Result<Keypair> {
    let file_content = fs::read_to_string(&path).map_err(ClientError::IoError)?;

    let bytes: Vec<u8> = serde_json::from_str(&file_content).map_err(ClientError::SerdeError)?;

    Keypair::from_bytes(&bytes)
        .map_err(|e| ClientError::KeypairError(format!("Failed to create keypair from bytes: {e}")))
}

/// Write a keypair to file with improved error handling
#[allow(clippy::result_large_err)]
pub fn write_keypair_file<P: AsRef<Path>>(keypair: &Keypair, path: P) -> Result<()> {
    let json =
        serde_json::to_string(&keypair.to_bytes().to_vec()).map_err(ClientError::SerdeError)?;
    fs::write(&path, json).map_err(ClientError::IoError)
}

pub async fn send_and_confirm_transactions(
    client: &RpcClient,
    transactions: &[Transaction],
) -> Result<()> {
    let futures = transactions.iter().map(|tx| {
        let client = &client;
        async move {
            let confirmed = client.send_and_confirm_transaction(tx).await;
            (tx.clone(), confirmed)
        }
    });
    let results = join_all(futures).await;
    for (_, result) in results {
        match result {
            Ok(signature) => trace!(signature:% = signature; "Transaction confirmed"),
            Err(e) => warn!(error:% = e; "Transaction NOT confirmed (timeout)"),
        }
    }
    Ok(())
}

pub async fn send_and_confirm_with_limit(
    client: &RpcClient,
    instructions: &[Instruction],
    payer: &Keypair,
    limit: u32,
) -> Result<()> {
    const BATCH_SIZE: usize = 20;
    for (i, chunk) in instructions.chunks(BATCH_SIZE).enumerate() {
        let futures = chunk.iter().map(|instruction| {
            let client = &client;
            async move {
                // the size is of this instruction is always
                // the same or smaller as we have const chunk size
                let limit = ComputeBudgetInstruction::set_compute_unit_limit(limit);
                let tx = Transaction::new_signed_with_payer(
                    &[instruction.clone(), limit],
                    Some(&payer.pubkey()),
                    &[&payer],
                    client.get_latest_blockhash().await.unwrap(),
                );

                let confirmed = client.send_and_confirm_transaction(&tx).await;
                (tx.clone(), confirmed)
            }
        });

        let results = join_all(futures).await;
        info!(chunk_index:% = i+1, total_chunks:% = (instructions.len().div_ceil(BATCH_SIZE)); "Chunk confirmed");
        for (_, result) in results {
            match result {
                Ok(signature) => trace!(signature:% = signature; "Transaction confirmed"),
                Err(e) => warn!(error:% = e; "Transaction NOT confirmed (timeout)"),
            }
        }
    }
    Ok(())
}
