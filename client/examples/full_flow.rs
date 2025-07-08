use std::path::Path;

use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark::{
    felt::Felt,
    stark_proof::verify_public_input::VerifyPublicInput,
    swiftness::stark::types::{cast_struct_to_slice, StarkProof},
};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::AccountCast;
use utils::BidirectionalStack;
use utils::Executable;
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 1000;

pub struct Input {
    pub front_index: u32,
    pub back_index: u32,
    pub proof: StarkProof,
}
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
    println!("\nSet Proof on Solana");
    println!("====================");

    let mut input: [u64; 2] = [0, 65536];
    let proof_bytes = cast_struct_to_slice(&mut input);
    let new_offset = proof_bytes.len();
    let stack_set_instructions = proof_bytes
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetAccountData(i * CHUNK_SIZE, chunk.to_vec()),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect::<Vec<_>>();

    let input = include_str!("../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();
    let mut proof_verifier = proof.transform_to();

    let proof_bytes = cast_struct_to_slice(&mut proof_verifier);
    println!("Proof bytes in kb: {:?}", proof_bytes.len() / 1024);
    let mut instructions = proof_bytes
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            Instruction::new_with_borsh(
                program_id,
                &VerifierInstruction::SetAccountData(new_offset + (i * CHUNK_SIZE), chunk.to_vec()),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            )
        })
        .collect::<Vec<_>>();
    instructions.extend(stack_set_instructions);

    println!("Instructions count: {:?}", instructions.len());
    let mut transactions = Vec::new();
    for instruction in instructions.iter() {
        let set_proof_tx = Transaction::new_signed_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
            &[&payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(set_proof_tx.clone());
    }
    send_and_confirm_transactions(&client, &transactions).await?;

    let task = VerifyPublicInput::new();

    let verify_public_input_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(task.to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let signature = interact_with_program_instructions(
        &client,
        &payer,
        &program_id,
        &stack_account,
        &[verify_public_input_ix],
    )
    .await?;
    println!("Verify public input: {signature}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(800_000);

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
            &[limit_instructions.clone(), execute_ix],
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
    println!("\nHash Public Inputs successfully executed on Solana!");

    Ok(())
}
