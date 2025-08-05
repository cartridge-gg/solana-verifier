use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use felt::Felt;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark::poseidon::PoseidonHashMany;
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
    // Cast to stack account to see if initialized correctly

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

    println!("\nPoseidon Hash on Solana");
    println!("======================");

    // Print information about the Poseidon operation
    println!(
        "Using PoseidonHashMany with TYPE_TAG: {}",
        PoseidonHashMany::TYPE_TAG
    );

    // Create inputs for Poseidon hash (using example from test)
    let inputs = vec![
        Felt::from_hex("0x1").unwrap(),
        Felt::from_hex("0x2").unwrap(),
        Felt::from_hex("0x3").unwrap(),
        Felt::from_hex("0x4").unwrap(),
    ];

    println!("Input values:");
    for (i, input) in inputs.iter().enumerate() {
        println!("  Input {}: {}", i + 1, input);
    }

    // Push all input data to the stack following the Poseidon algorithm
    // 1. Pad inputs with 1 followed by 0's if necessary to make even length
    let mut padded_inputs = inputs.clone();
    padded_inputs.push(Felt::ONE);
    let len = padded_inputs.len().div_ceil(2) * 2;
    padded_inputs.resize(len, Felt::ZERO);
    println!("Padded input length: {len}");

    // 2. Push values in reverse order
    for input in padded_inputs.iter().rev() {
        let push_data_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::PushData(input.to_bytes_be().to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );

        let push_data_tx = Transaction::new_signed_with_payer(
            &[push_data_ix],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );

        let _push_data_sig = client.send_and_confirm_transaction(&push_data_tx).await?;
        println!("Pushed input value: {input}");
    }

    // 3. Push three zeros
    for _ in 0..3 {
        let push_data_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::PushData(Felt::ZERO.to_bytes_be().to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );

        let push_data_tx = Transaction::new_signed_with_payer(
            &[push_data_ix],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );

        let _push_data_sig = client.send_and_confirm_transaction(&push_data_tx).await?;
        println!("Pushed zero value");
    }

    let poseidon_task = PoseidonHashMany::new(inputs.len());

    // Push the task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(poseidon_task.to_vec_with_type_tag()),
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
    println!("\nPoseidon hash task pushed: {signature}");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");
    // Execute until task is complete
    let mut transactions = Vec::new();
    for i in 0..simulation_steps {
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
    let result_bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(result_bytes);
    stack.pop_front();
    stack.pop_front();
    stack.pop_front();
    println!("\nPoseidon hash result: {result}");
    println!("Stack front index: {}", stack.front_index);
    println!("Stack back index: {}", stack.back_index);
    // The expected output should match the result we got (from the test for 3 inputs)
    let expected_result =
        Felt::from_hex("0x26e3ad8b876e02bc8a4fc43dad40a8f81a6384083cabffa190bcf40d512ae1d")
            .unwrap();

    assert_eq!(result, expected_result);
    println!("\nPoseidon hash successfully executed on Solana!");

    Ok(())
}
