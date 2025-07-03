use std::path::Path;

use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark::{
    felt::Felt, stark_proof::hash_public_inputs::HashPublicInputs,
    swiftness::stark::types::cast_struct_to_slice,
};
use utils::BidirectionalStack;
use utils::{AccountCast, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> client::Result<()> {
    let config = Config::parse_args();
    let client = initialize_client(&config).await?;

    let payer = setup_payer(&client, &config).await?;

    let program_path = Path::new("target/deploy/verifier.so");

    let program_id = setup_program(&client, &payer, &config, program_path).await?;

    println!("Using program ID: {program_id}");

    let stack_account = Keypair::new();

    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<BidirectionalStackAccount>();
    println!("Account space: {space} bytes");

    let create_account_ix = create_account(
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

    println!("\nHash Public Inputs on Solana");
    println!("============================");

    //print information about the Public Inputs operation
    println!(
        "Using PublicInputs with TYPE_TAG: {}",
        HashPublicInputs::TYPE_TAG
    );
    // Create inputs for Poseidon hash (using example from test)

    let program = vec![
        Felt::from_hex("0x1").unwrap(),
        Felt::from_hex("0x28").unwrap(),
        Felt::from_hex("0x54d3603ed14fb897d0925c48f26330ea9950bd4ca95746dad4f7f09febffe0d")
            .unwrap(),
        Felt::from_hex("0x60c63419890752e8e6ad268e965269cc682c1f8e78a314fc25e6ca8bdb30460")
            .unwrap(),
        Felt::from_hex("0x1adad196432230def36424f84d0a6c2b69377edfebe3512afece557d718f6f4")
            .unwrap(),
        Felt::from_hex("0x10").unwrap(),
        Felt::from_hex("0x11").unwrap(),
        Felt::from_hex("0x438a577de394189296b6d1e1f3196cd5e7a0ace493d89a1a9e6aa1c7a118711")
            .unwrap(),
        Felt::from_hex("0x21b737ecac6043ce49e7993b4b3c50238573c5d5f6f99dfb5ec9f67da55efd9")
            .unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0xb2954ff8d3985ab83ce945953c9e91db03e5e6a8841f8f46661ad21d9763f8").unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0x1").unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0x3").unwrap(),
        Felt::from_hex("0x1").unwrap(),
        Felt::from_hex("0x6").unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0x7").unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0x73d6376a3885b342aebfd86ec0290493e10f6e58e75afd29790b6bcdf82684c")
            .unwrap(),
        Felt::from_hex("0x2e7442625bab778683501c0eadbc1ea17b3535da040a12ac7d281066e915eea")
            .unwrap(),
        Felt::from_hex("0xa").unwrap(),
        Felt::from_hex("0xa2475bc66197c751d854ea8c39c6ad9781eb284103bcd856b58e6b500078ac").unwrap(),
        Felt::from_hex("0xa2475bc66197c751d854ea8c39c6ad9781eb284103bcd856b58e6b500078ac").unwrap(),
        Felt::from_hex("0x2b4690e832e4dbc7982a01f7c7c369dd85dbfc6993d42f89b789a9e3b315801")
            .unwrap(),
        Felt::from_hex("0x18913d6e28e3565eea5").unwrap(),
        Felt::from_hex("0x18913d6e28e3565db04").unwrap(),
        Felt::from_hex("0x7b62949c85c6af8a50c11c22927f9302f7a2e40bc93b4c988415915b0f97f09")
            .unwrap(),
        Felt::from_hex("0x7c539").unwrap(),
        Felt::from_hex("0x7d8da").unwrap(),
        Felt::from_hex("0x6d19755b067c9bc924da6f9907fa7d8128b8d1ae6850d4860fc5e9d5525a29b")
            .unwrap(),
        Felt::from_hex("0x2c000000000000003002").unwrap(),
        Felt::from_hex("0x7dc7899aa655b0aae51eadff6d801a58e97dd99cf4666ee59e704249e51adf2")
            .unwrap(),
        Felt::from_hex("0x7dc7899aa655b0aae51eadff6d801a58e97dd99cf4666ee59e704249e51adf2")
            .unwrap(),
        Felt::from_hex("0x1").unwrap(),
        Felt::from_hex("0x1922d2cd8b63eccf66321673234a52126cd9f0ab1bf6298c5abee6ee80c8990")
            .unwrap(),
        Felt::from_hex("0x0").unwrap(),
        Felt::from_hex("0x13ac240a60aa7ae09a00ea9bf47622d31c07642091d461b5f9250c993eca3d5")
            .unwrap(),
    ];
    let output = vec![
        Felt::from_hex("0x1").unwrap(),
        Felt::from_hex("0x2").unwrap(),
        Felt::from_hex("0x3").unwrap(),
    ];

    let input = [program.clone(), output.clone()];
    for input in input.iter().rev() {
        //Pad input with 1 followed by 0's (if necessary).
        let mut padded_input = input.clone();
        padded_input.push(Felt::ONE);
        let len = padded_input.len().div_ceil(2) * 2;
        padded_input.resize(len, Felt::ZERO);
        println!("Padded input length: {len}");
        for input in padded_input.iter().rev() {
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
            let push_signature = client.send_and_confirm_transaction(&push_data_tx).await?;
            println!("pushed data signature: {push_signature}");
        }
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
            let push_signature = client.send_and_confirm_transaction(&push_data_tx).await?;
            println!("pushed zero value signature: {push_signature}");
        }
    }

    let task = HashPublicInputs::new(program.len(), output.len());

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(task.to_vec_with_type_tag()),
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
    println!("\nHash Public Inputs task pushed: {signature}");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
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
    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    println!("\nProgram Hash: {result_program_hash:?}");
    println!("Output Hash: {result_output_hash:?}");
    println!("Stack front index: {}", stack.front_index);
    println!("Stack back index: {}", stack.back_index);
    println!("Simulation steps: {simulation_steps}");

    let expected_result =
        Felt::from_hex("0xa6830417400f5f63d8f1d81fc73a968a6ea4d677da62da24365bd0536b4233").unwrap();

    assert_eq!(result_program_hash, expected_result);
    println!("\nHash Public Inputs successfully executed on Solana!");

    Ok(())
}
