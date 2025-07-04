use crate::{
    initialize_client, interact_with_program_instructions, send_and_confirm_with_limit,
    setup_payer, ClientError,
};
use crate::{read_keypair_file, Config, Result};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
};
use stark::{
    felt::Felt, stark_proof::VerifyPublicInput, swiftness::stark::types::cast_struct_to_slice,
};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use utils::AccountCast;
use utils::BidirectionalStack;
use utils::Executable;
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 900;

pub async fn verify(config: &Config) -> Result<()> {
    let client = initialize_client(config).await?;
    let payer = if let Some(ref payer_keypair) = config.payer_keypair {
        Keypair::from_base58_string(payer_keypair)
    } else {
        setup_payer(&client, config).await?
    };
    println!("Using payer: {}", payer.pubkey());

    let program_keypair = read_keypair_file("keypairs/verifier-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();
    println!("Using program ID: {program_id}");

    let stack_account = read_keypair_file("keypairs/stack-account-keypair.json").unwrap();
    println!("Using stack account: {}", stack_account.pubkey());

    let time = std::time::Instant::now();
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
    let mut proof_set_instructions = proof_bytes
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
    proof_set_instructions.extend(stack_set_instructions);
    println!("Instructions number: {:?}", proof_set_instructions.len());
    send_and_confirm_with_limit(&client, &proof_set_instructions, &payer, 1_000).await?;
    println!("Time taken to set proof: {:?}", time.elapsed());
    let time2 = std::time::Instant::now();
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

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();

    println!("Simulation steps: {simulation_steps}");

    let mut instructions = vec![];
    for i in 0..simulation_steps {
        // Execute the task
        let execute_ix = Instruction::new_with_borsh(
            program_id,
            &VerifierInstruction::Execute(i as u32),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        instructions.push(execute_ix);
    }

    send_and_confirm_with_limit(&client, &instructions, &payer, 500_000).await?;

    println!("Time taken to execute: {:?}", time2.elapsed());
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
