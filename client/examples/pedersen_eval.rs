use client::{
    initialize_client, interact_with_program_instructions, setup_payer, setup_program, ClientError,
    Config,
};
use felt::Felt;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark::pedersen::points::PedersenEval;
use stark::swiftness::stark::types::cast_struct_to_slice;
use std::{mem::size_of, path::Path};
use utils::{AccountCast, BidirectionalStack, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

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

    // Initialize the account
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

    // Cast to stack account to see if initialized correctly
    let account_data_after_init = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast(&account_data_after_init);
    println!("Stack front_index: {}", stack.front_index);
    println!("Stack back_index: {}", stack.back_index);

    println!("\nEval Pedersen on Solana");
    println!("=========================");

    // Print information about the Hades operation
    println!(
        "Using PedersenEval with TYPE_TAG: {}",
        PedersenEval::TYPE_TAG
    );

    let point = Felt::from_hex("0x55883272fbc6be7532b78c2758584f9f15fd43055c0a06eb3471051bf2d0d4a")
        .unwrap();

    // Push the task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(PedersenEval::new(point).to_vec_with_type_tag()),
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
    println!("Pedersen eval task pushed: {signature}");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);

    let simulation_steps = stack.simulate();
    println!("Simulation steps: {simulation_steps}");
    let mut signatures = Vec::new();

    for i in 0..simulation_steps {
        let execute_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::Execute(i as u32),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );

        let limit = ComputeBudgetInstruction::set_compute_unit_limit(700_000);

        let execute_tx = Transaction::new_signed_with_payer(
            &[limit, execute_ix],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );

        let sim = client.simulate_transaction(&execute_tx).await?;
        println!(
            "Sim step {i}, CU (sim): {}",
            sim.value.units_consumed.unwrap_or(0)
        );

        let sig = client.send_and_confirm_transaction(&execute_tx).await?;
        println!("Execute step {i} signature: {sig}");
        signatures.push(sig);
    }

    // Read and display the result
    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let result_bytes = stack.borrow_front();
    let pedersen_point_y = Felt::from_bytes_be(&result_bytes.try_into().unwrap());
    stack.pop_front();

    let result_bytes = stack.borrow_front();
    let pedersen_point_x = Felt::from_bytes_be(&result_bytes.try_into().unwrap());
    stack.pop_front();
    println!("\nPedersen point Y: {pedersen_point_y}");
    println!("Pedersen point X: {pedersen_point_x}");
    println!("Stack front index: {}", stack.front_index);
    println!("Stack back index: {}", stack.back_index);

    // The expected output should match the result we got
    let expected_point_y =
        Felt::from_hex("0x4fe4068e06eefa17eefab622b3c9d9433bc11552fd96bf324893028770e40f4")
            .unwrap();

    let expected_point_x =
        Felt::from_hex("0x598904d65b0434a87c175e65222359d01fff2522cade3bb409c28885b7671e").unwrap();
    assert_eq!(pedersen_point_y, expected_point_y);
    assert_eq!(pedersen_point_x, expected_point_x);
    println!("Pedersen evaluation successful and verified!");
    Ok(())
}
