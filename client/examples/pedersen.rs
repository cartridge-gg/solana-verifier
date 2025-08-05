use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use felt::Felt;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark::pedersen::PedersenHash;
use stark::swiftness::stark::types::cast_struct_to_slice;
use std::{mem::size_of, path::Path};
use utils::{AccountCast, BidirectionalStack, Executable};
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
    // Initialize the account
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
    println!("Account initialized: {signature}");

    let account_data_after_init = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast(&account_data_after_init);
    println!("Stack front_index: {}", stack.front_index);
    println!("Stack back_index: {}", stack.back_index);

    println!("\nPedersen Hash on Solana");
    println!("======================");

    // Print information about the Pedersen operation
    println!(
        "Using PedersenHash with TYPE_TAG: {}",
        PedersenHash::TYPE_TAG
    );

    // Create inputs for Pedersen hash
    let x =
        Felt::from_hex("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb").unwrap();
    let y =
        Felt::from_hex("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a").unwrap();

    println!("Input values:");
    println!("  x: {:?}", x);
    println!("  y: {:?}", y);

    // Push input data to the stack
    let push_x_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(x.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let push_x_tx = Transaction::new_signed_with_payer(
        &[push_x_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    let _push_x_sig = client.send_and_confirm_transaction(&push_x_tx).await?;
    println!("Pushed x value: {:?}", x);

    let push_y_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushData(y.to_bytes_be().to_vec()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let push_y_tx = Transaction::new_signed_with_payer(
        &[push_y_ix],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().await?,
    );

    let _push_y_sig = client.send_and_confirm_transaction(&push_y_tx).await?;
    println!("Pushed y value: {:?}", y);

    let pedersen_task = PedersenHash::new();

    // Push the task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(pedersen_task.to_vec_with_type_tag()),
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
    println!("\nPedersen hash task pushed: {signature}");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(800_000);

    // Execute until task is complete
    let mut transactions = Vec::new();
    for i in 0..simulation_steps {
        let execute_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::Execute(i as u32),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        let execute_tx = Transaction::new_signed_with_payer(
            &[limit_instructions.clone(), execute_ix],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(execute_tx.clone());
        println!("transactions: {:?}", transactions.len());
    }
    send_and_confirm_transactions(&client, &transactions).await?;

    // Read and display the result
    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let result_bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(result_bytes);
    stack.pop_front(); // Pop the result to properly empty the stack
    println!("\nPedersen hash result: {:?}", result);
    println!("Stack front index: {}", stack.front_index);
    println!("Stack back index: {}", stack.back_index);

    // The expected output should match the result we got
    let expected_result =
        Felt::from_hex("030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662").unwrap();

    assert_eq!(result, expected_result);
    println!("\nPedersen hash successfully executed on Solana!");

    // Verify stack is properly empty
    assert_eq!(
        stack.front_index, 0,
        "Stack front_index should be 0 after operation"
    );
    assert_eq!(
        stack.back_index, 65536,
        "Stack back_index should be 65536 after operation"
    );

    Ok(())
}
