use crate::{
    initialize_client, interact_with_program_instructions, send_and_confirm_with_limit,
    setup_payer, ClientError,
};
use crate::{read_keypair_file, Config, Result};
use felt::Felt;
use log::info;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
};
use stark::{stark_proof::VerifyPublicInput, swiftness::stark::types::cast_struct_to_slice};
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
    info!(public_key:% = payer.pubkey(); "Using payer");

    let program_keypair = read_keypair_file("keypairs/verifier-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();
    info!(program_id:% = program_id; "Using program");

    let stack_account = read_keypair_file("keypairs/stack-account-keypair.json").unwrap();
    info!(public_key:% = stack_account.pubkey(); "Using stack account");

    let time = std::time::Instant::now();
    let input: [u64; 2] = [0, 65536];
    let proof_bytes = cast_struct_to_slice(&input);
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

    let proof_verifier = proof.transform_to();
    let proof_bytes = cast_struct_to_slice(&proof_verifier);

    info!(size_in_bytes:% = proof_bytes.len() / 1024; "Proof bytes in kb");
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
    info!(instructions_number:% = proof_set_instructions.len(); "Instructions number");
    send_and_confirm_with_limit(&client, &proof_set_instructions, &payer, 1_000).await?;
    info!(time_in_seconds:% = time.elapsed().as_secs(); "Time taken to set proof");
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
    info!(signature:% = signature; "Verify public input");

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();

    info!(simulation_steps:% = simulation_steps; "Simulation steps");

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

    info!(time_in_seconds:% = time2.elapsed().as_secs(); "Time taken to execute");
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
    info!(result_program_hash:% = result_program_hash; "Program Hash");
    info!(result_output_hash:% = result_output_hash; "Output Hash");
    info!(front_index:% = stack.front_index; "Stack front index");
    info!(back_index:% = stack.back_index; "Stack back index");

    info!("Hash Public Inputs successfully executed on Solana!");
    Ok(())
}
