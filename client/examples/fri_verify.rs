use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark::stark_proof::stark_verify::FriVerify;
use stark::swiftness::stark::types::cast_struct_to_slice;
use std::{mem::size_of, path::Path};
use utils::{AccountCast, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

/// Main entry point for the Solana program client
#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> client::Result<()> {
    // Parse command-line arguments
    let config = Config::parse_args();

    // Initialize the Solana client
    let client = initialize_client(&config).await?;

    // Setup the payer account
    let payer = setup_payer(&client, &config).await?;

    // Define program path
    let program_path = Path::new("target/deploy/verifier.so");

    // Deploy or use existing program
    let program_id = setup_program(&client, &payer, &config, program_path).await?;

    println!("Using program ID: {program_id}");

    // Create a new account that's owned by our program
    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    // Calculate the space needed for our account
    let space = size_of::<BidirectionalStackAccount>();
    println!("Account space: {space} bytes");

    // Create account instruction
    let create_account_ix = create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        &program_id,
    );

    // Create and send the transaction
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

    let mut stack_init_input: [u64; 2] = [0, 65536];
    let stack_init_bytes = cast_struct_to_slice(&mut stack_init_input);

    let init_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::SetAccountData(0, stack_init_bytes.to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[init_ix],
    )
    .await?;
    println!("Account initialize chunk: {signature}");

    // Cast to stack account to see if initialized correctly
    let account_data_after_init = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast(&account_data_after_init);
    println!("Stack front_index: {}", stack.front_index);
    println!("Stack back_index: {}", stack.back_index);

    // Print information about the FRI verification operation
    println!("Using FRI_VERIFY with TYPE_TAG: {}", FriVerify::TYPE_TAG);

    // Push the task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(FriVerify::new().to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_task_ix],
    )
    .await?;

    println!("\nFRI_VERIFY task pushed: {signature}");

    let input_data =
        include_bytes!("../../programs/verifier/tests/fixtures/fri_verify.bin").to_vec();

    let input_data_len = input_data.len();
    for (i, chunk) in input_data.chunks(500).enumerate() {
        let push_data_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::PushDataChunk(chunk.to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        let signature = interact_with_program_instructions(
            &client,
            &payer,
            &program_id,
            &stack_account,
            &[push_data_ix],
        )
        .await?;
        println!("Pushed data chunk of size {}: {signature}", i);
    }
    let push_data_complete_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushDataChunkComplete(input_data_len as u32),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );
    let signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[push_data_complete_ix],
    )
    .await?;
    println!("Pushed data complete: {signature}");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Simulation steps: {simulation_steps}");

    let mut transactions = Vec::new();
    for i in 0..simulation_steps {
        // Execute the task
        let execute_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::Execute(i as u32),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );

        let execute_tx = Transaction::new_signed_with_payer(
            &[execute_ix],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );

        transactions.push(execute_tx.clone());
    }
    send_and_confirm_transactions(&client, &transactions).await?;
    // Read and display the result
    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    assert_eq!(stack.back_index, 65536, "Stack back index should be 65536");
    assert_eq!(stack.front_index, 0, "Stack front index should be 0");
    println!("FRI_VERIFY completed successfully.");
    Ok(())
}
