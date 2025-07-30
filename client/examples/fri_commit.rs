use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use stark::swiftness::stark::types::cast_struct_to_slice;
use std::{mem::size_of, path::Path};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::{AccountCast, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

use stark::stark_proof::stark_commit::FriCommit;

pub const CHUNK_SIZE: usize = 1000;

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

    println!("\nFriCommit Task on Solana");
    println!("==================================");

    let input = include_str!("../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let mut proof_verifier = proof.transform_to();

    let mut stack_init_input: [u64; 2] = [0, 65536];
    let stack_init_bytes = cast_struct_to_slice(&mut stack_init_input);
    let proof_bytes = cast_struct_to_slice(&mut proof_verifier).to_vec();
    let mut init_calldata = stack_init_bytes.to_vec();
    init_calldata.extend(proof_bytes.clone());

    println!("Proof bytes in kb: {:?}", init_calldata.len() / 1024);
    let instructions = init_calldata
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

    println!("Instructions number: {:?}", instructions.len());
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
    println!("Proof data set successfully");

    // Push the FriCommit task to the stack
    let validate_task = FriCommit::new();

    println!("Using FriCommit with TYPE_TAG: {}", FriCommit::TYPE_TAG);

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(validate_task.to_vec_with_type_tag()),
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
    println!("FriCommit task pushed: {signature}");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(800_000);

    // Execute all steps until task is complete - split into chunks of max 5000
    const MAX_CHUNK_SIZE: usize = 5000;

    let simulation_steps_usize = simulation_steps as usize;

    for chunk_start in (0..simulation_steps_usize).step_by(MAX_CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + MAX_CHUNK_SIZE, simulation_steps_usize);
        let chunk_size = chunk_end - chunk_start;

        println!(
            "Processing steps {}-{} ({} steps)",
            chunk_start,
            chunk_end - 1,
            chunk_size
        );

        let mut transactions = Vec::new();
        for i in chunk_start..chunk_end {
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
        println!("Chunk {}-{} completed", chunk_start, chunk_end - 1);
    }

    println!("All execution steps completed");
    println!("\nFriCommit successfully executed on Solana!");

    Ok(())
}
