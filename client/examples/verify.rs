use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use felt::Felt;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use std::{mem::size_of, path::Path};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use types::swiftness::commitment::types::Decommitment;
use types::swiftness::{
    global_values::InteractionElements, stark::types::cast_struct_to_slice_mut,
};
use types::{
    funvec::FunVec,
    swiftness::stark::types::{FriVerifyData, StarkCommitment},
};
use utils::BidirectionalStack;
use utils::StarkCommitmentTrait;
use utils::{AccountCast, CacheStorage, Executable};

use verifier_1::{
    instruction::VerifierInstruction as VI1, state::BidirectionalStackAccount as Stack1,
};
use verifier_2::{
    instruction::VerifierInstruction as VI2, state::BidirectionalStackAccount as Stack2,
};
use verifier_3::{
    instruction::VerifierInstruction as VI3, state::BidirectionalStackAccount as Stack3,
};
use verifier_4::{
    instruction::VerifierInstruction as VI4, state::BidirectionalStackAccount as Stack4,
};

use verify_1::verify::Verify as Verify_Stage_One;
use verify_2::verify::Verify as Verify_Stage_Two;
use verify_3::verify::Verify as Verify_Stage_Three;
use verify_4::verify::Verify as Verify_Stage_Four;

pub const CHUNK_SIZE: usize = 1000;
const MAX_CHUNK_SIZE: usize = 3000;

fn main() {
    let result = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .thread_stack_size(32 * 1024 * 1024)
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async_main())
        })
        .unwrap()
        .join()
        .unwrap();

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

#[allow(clippy::result_large_err)]
async fn async_main() -> client::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("client", log::LevelFilter::Info)
        .init();

    let config = Config::parse_args();
    let client = initialize_client(&config).await?;
    let payer = setup_payer(&client, &config).await?;

    // ========== STAGE 1 ==========
    println!("\n========== STAGE 1 ==========");
    let (stark_commitment, digest, counter) = execute_stage_1(&client, &payer, &config).await?;

    // ========== STAGE 2 ==========
    println!("\n========== STAGE 2 ==========");
    let queries = execute_stage_2(
        &client,
        &payer,
        &config,
        &stark_commitment,
        &digest,
        &counter,
    )
    .await?;

    // let stark_commitment = StarkCommitment::default();
    // let queries = Vec::new();
    // ========== STAGE 3 ==========
    println!("\n========== STAGE 3 ==========");
    let stark_verify_data =
        execute_stage_3(&client, &payer, &config, &stark_commitment, &queries).await?;

    // ========== STAGE 4 ==========
    println!("\n========== STAGE 4 ==========");
    execute_stage_4(
        &client,
        &payer,
        &config,
        &stark_commitment,
        &stark_verify_data,
    )
    .await?;

    println!("\n✓ All 4 stages completed successfully!");
    println!("✓ All verifications passed!");
    println!("✓ Verify test completed successfully on Solana!");

    Ok(())
}

async fn execute_stage_1(
    client: &RpcClient,
    payer: &Keypair,
    config: &Config,
) -> client::Result<(
    types::swiftness::stark::types::StarkCommitment<InteractionElements>,
    Felt,
    Felt,
)> {
    println!("Starting Verifier 1");
    let program_path = Path::new("target/deploy/verifier_1.so");
    let program_id = setup_program(client, payer, config, program_path).await?;
    println!("Using program ID: {program_id}");

    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<Stack1>();
    println!("Stack1 size: {} bytes", space);
    create_account_tx(client, payer, &stack_account, space, &program_id).await?;

    let stack_bytes = prepare_input::get_bytes_stage1();
    set_account_data_chunked_v1(client, payer, &program_id, &stack_account, &stack_bytes).await?;

    let verify_task = Verify_Stage_One::new();
    println!("Using Verify with TYPE_TAG: {}", Verify_Stage_One::TYPE_TAG);

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VI1::PushTask(verify_task.to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    interact_with_program_instructions(client, payer, &program_id, &stack_account, &[push_task_ix])
        .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = Stack1::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    execute_transactions_v1(
        client,
        payer,
        &program_id,
        &stack_account,
        simulation_steps as u32,
    )
    .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = Stack1::cast_mut(&mut account_data);
    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");

    let counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");
    assert_eq!(stack.is_empty_front(), true, "Stack should be empty");

    let commitment = stack.stark_commitment.clone();
    println!("commitment: {:?}", commitment);
    println!("digest: {:?}", digest);
    println!("counter: {:?}", counter);
    println!("✓ Stage 1 completed");
    Ok((commitment, digest, counter))
}

async fn execute_stage_2(
    client: &RpcClient,
    payer: &Keypair,
    config: &Config,
    stark_commitment: &types::swiftness::stark::types::StarkCommitment<InteractionElements>,
    digest: &Felt,
    counter: &Felt,
) -> client::Result<Vec<Felt>> {
    println!("Starting Verifier 2");
    let program_path = Path::new("target/deploy/verifier_2.so");
    let program_id = setup_program(client, payer, config, program_path).await?;

    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<Stack2>();
    println!("Stack2 size: {} bytes", space);
    create_account_tx(client, payer, &stack_account, space, &program_id).await?;

    let stack_bytes = prepare_input::get_bytes_stage2(stark_commitment);
    set_account_data_chunked_v2(client, payer, &program_id, &stack_account, &stack_bytes).await?;

    let verify_task = Verify_Stage_Two::new(*digest, *counter);
    println!("Using Verify with TYPE_TAG: {}", Verify_Stage_Two::TYPE_TAG);

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VI2::PushTask(verify_task.to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    interact_with_program_instructions(client, payer, &program_id, &stack_account, &[push_task_ix])
        .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = Stack2::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    execute_transactions_v2(
        client,
        payer,
        &program_id,
        &stack_account,
        simulation_steps as u32,
    )
    .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = Stack2::cast_mut(&mut account_data);
    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");

    let queries = stack.verify_variables.queries_indexes.to_vec();
    println!("✓ Stage 2 completed with {} queries", queries.len());
    Ok(queries)
}

async fn execute_stage_3(
    client: &RpcClient,
    payer: &Keypair,
    config: &Config,
    stark_commitment: &types::swiftness::stark::types::StarkCommitment<InteractionElements>,
    queries: &[Felt],
) -> client::Result<types::swiftness::stark::types::FriVerifyData> {
    println!("Starting Verifier 3");
    let program_path = Path::new("target/deploy/verifier_3.so");
    let program_id = setup_program(client, payer, config, program_path).await?;

    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<Stack3>();
    println!("Stack3 size: {} bytes", space);
    create_account_tx(client, payer, &stack_account, space, &program_id).await?;

    let stack_bytes = prepare_input::get_bytes_stage3(stark_commitment, queries);
    set_account_data_chunked_v3(client, payer, &program_id, &stack_account, &stack_bytes).await?;

    let verify_task = Verify_Stage_Three::new();
    println!(
        "Using Verify with TYPE_TAG: {}",
        Verify_Stage_Three::TYPE_TAG
    );
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VI3::PushTask(verify_task.to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );
    interact_with_program_instructions(client, payer, &program_id, &stack_account, &[push_task_ix])
        .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = Stack3::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");
    let fri_after_simulation =
        stack.borrow_from_cache::<types::swiftness::stark::types::FriVerifyData>();
    println!(
        "AFTER simulation (LOCAL) - cache values len: {:?}",
        fri_after_simulation.fri_decommitment
    );

    execute_transactions_v3(
        client,
        payer,
        &program_id,
        &stack_account,
        simulation_steps as u32,
    )
    .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    println!("account_data length: {}", account_data.len());
    let offset_do_cached_data =
        size_of::<Stack3>() - size_of::<types::swiftness::stark::types::FriVerifyData>();
    println!(
        "First 100 bytes of cached_data region: {:?}",
        &account_data[offset_do_cached_data..offset_do_cached_data + 100]
    );

    let stack = Stack3::cast_mut(&mut account_data);
    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");

    let fri_verify_data: &types::swiftness::stark::types::FriVerifyData = stack.borrow_from_cache();
    println!(
        "AFTER transaction (ONCHAIN) - cache values len: {:?}",
        fri_verify_data.fri_decommitment
    );
    println!("✓ Stage 3 completed");
    Ok(fri_verify_data.clone())
}

async fn execute_stage_4(
    client: &RpcClient,
    payer: &Keypair,
    config: &Config,
    stark_commitment: &types::swiftness::stark::types::StarkCommitment<InteractionElements>,
    stark_verify_data: &types::swiftness::stark::types::FriVerifyData,
) -> client::Result<()> {
    println!(
        "DEBUG Stage 4 - fri_decommitment {:?}",
        stark_verify_data.fri_decommitment
    );

    println!("Starting Verifier 4");
    let program_path = Path::new("target/deploy/verifier_4.so");
    let program_id = setup_program(client, payer, config, program_path).await?;

    let stack_account = Keypair::new();
    println!("Creating new account: {}", stack_account.pubkey());

    let space = size_of::<Stack4>();
    println!("Stack4 size: {} bytes", space);
    create_account_tx(client, payer, &stack_account, space, &program_id).await?;

    let stack_bytes = prepare_input::get_bytes_stage4(stark_commitment, stark_verify_data);
    set_account_data_chunked_v4(client, payer, &program_id, &stack_account, &stack_bytes).await?;

    let verify_task = Verify_Stage_Four::new();
    println!(
        "Using Verify with TYPE_TAG: {}",
        Verify_Stage_Four::TYPE_TAG
    );

    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VI4::PushTask(verify_task.to_vec_with_type_tag()),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    interact_with_program_instructions(client, payer, &program_id, &stack_account, &[push_task_ix])
        .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = Stack4::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    execute_transactions_v4(
        client,
        payer,
        &program_id,
        &stack_account,
        simulation_steps as u32,
    )
    .await?;

    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = Stack4::cast_mut(&mut account_data);
    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");

    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("Result program hash: {:?}", result_program_hash);
    println!("Result output hash: {:?}", result_output_hash);

    assert_eq!(
        result_program_hash,
        Felt::from_hex("0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07").unwrap(),
        "Program hash mismatch"
    );
    assert_eq!(
        result_output_hash,
        Felt::from_hex("0x3233b5615a8de5563f7d3ba086b8f260189ac47753a1c131d063ed3f6c24400")
            .unwrap(),
        "Output hash mismatch"
    );

    assert!(stack.is_empty_front(), "Stack front should be empty");
    assert!(stack.is_empty_back(), "Stack back should be empty");

    println!("✓ Stage 4 completed with correct hashes");
    Ok(())
}

// Helper functions
async fn create_account_tx(
    client: &RpcClient,
    payer: &Keypair,
    stack_account: &Keypair,
    space: usize,
    program_id: &solana_sdk::pubkey::Pubkey,
) -> client::Result<()> {
    let create_account_ix = create_account(
        &payer.pubkey(),
        &stack_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(space).await?,
        space as u64,
        program_id,
    );

    let create_account_tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[payer, stack_account],
        client.get_latest_blockhash().await?,
    );

    client
        .send_and_confirm_transaction(&create_account_tx)
        .await?;
    println!("Account created successfully");
    Ok(())
}

// Dedykowane funkcje dla każdego verifier'a
async fn set_account_data_chunked_v1(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    stack_bytes: &[u8],
) -> client::Result<()> {
    let mut transactions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            *program_id,
            &VI1::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        let tx = Transaction::new_signed_with_payer(
            &[set_data_ix],
            Some(&payer.pubkey()),
            &[payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(tx);
    }
    send_and_confirm_transactions(client, &transactions).await?;
    println!("Data set successfully");
    Ok(())
}

async fn set_account_data_chunked_v2(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    stack_bytes: &[u8],
) -> client::Result<()> {
    let mut transactions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            *program_id,
            &VI2::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        let tx = Transaction::new_signed_with_payer(
            &[set_data_ix],
            Some(&payer.pubkey()),
            &[payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(tx);
    }
    send_and_confirm_transactions(client, &transactions).await?;
    println!("Data set successfully");
    Ok(())
}

async fn set_account_data_chunked_v3(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    stack_bytes: &[u8],
) -> client::Result<()> {
    let mut transactions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            *program_id,
            &VI3::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        let tx = Transaction::new_signed_with_payer(
            &[set_data_ix],
            Some(&payer.pubkey()),
            &[payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(tx);
    }
    send_and_confirm_transactions(client, &transactions).await?;
    println!("Data set successfully");
    Ok(())
}

async fn set_account_data_chunked_v4(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    stack_bytes: &[u8],
) -> client::Result<()> {
    let mut transactions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            *program_id,
            &VI4::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(stack_account.pubkey(), false)],
        );
        let tx = Transaction::new_signed_with_payer(
            &[set_data_ix],
            Some(&payer.pubkey()),
            &[payer],
            client.get_latest_blockhash().await?,
        );
        transactions.push(tx);
    }
    send_and_confirm_transactions(client, &transactions).await?;
    println!("Data set successfully");
    Ok(())
}

async fn execute_transactions_v1(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    simulation_steps: u32,
) -> client::Result<()> {
    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_200_000);
    let simulation_steps_usize = simulation_steps as usize;

    for chunk_start in (0..simulation_steps_usize).step_by(MAX_CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + MAX_CHUNK_SIZE, simulation_steps_usize);
        println!("Processing steps {}-{}", chunk_start, chunk_end - 1);

        let mut transactions = Vec::new();
        for i in chunk_start..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &VI1::Execute(i as u32),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            );
            let tx = Transaction::new_signed_with_payer(
                &[limit_instructions.clone(), execute_ix],
                Some(&payer.pubkey()),
                &[payer],
                client.get_latest_blockhash().await?,
            );
            transactions.push(tx);
        }

        send_and_confirm_transactions(client, &transactions).await?;
        println!("Chunk {}-{} completed", chunk_start, chunk_end - 1);
    }
    Ok(())
}

async fn execute_transactions_v2(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    simulation_steps: u32,
) -> client::Result<()> {
    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_200_000);
    let simulation_steps_usize = simulation_steps as usize;

    for chunk_start in (0..simulation_steps_usize).step_by(MAX_CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + MAX_CHUNK_SIZE, simulation_steps_usize);
        println!("Processing steps {}-{}", chunk_start, chunk_end - 1);

        let mut transactions = Vec::new();
        for i in chunk_start..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &VI2::Execute(i as u32),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            );
            let tx = Transaction::new_signed_with_payer(
                &[limit_instructions.clone(), execute_ix],
                Some(&payer.pubkey()),
                &[payer],
                client.get_latest_blockhash().await?,
            );
            transactions.push(tx);
        }

        send_and_confirm_transactions(client, &transactions).await?;
        println!("Chunk {}-{} completed", chunk_start, chunk_end - 1);
    }
    Ok(())
}

async fn execute_transactions_v3(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    simulation_steps: u32,
) -> client::Result<()> {
    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_200_000);
    let simulation_steps_usize = simulation_steps as usize;

    for chunk_start in (0..simulation_steps_usize).step_by(MAX_CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + MAX_CHUNK_SIZE, simulation_steps_usize);
        println!("Processing steps {}-{}", chunk_start, chunk_end - 1);

        let mut transactions = Vec::new();
        for i in chunk_start..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &VI3::Execute(i as u32),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            );
            let tx = Transaction::new_signed_with_payer(
                &[limit_instructions.clone(), execute_ix],
                Some(&payer.pubkey()),
                &[payer],
                client.get_latest_blockhash().await?,
            );
            transactions.push(tx);
        }

        send_and_confirm_transactions(client, &transactions).await?;
        println!("Chunk {}-{} completed", chunk_start, chunk_end - 1);
    }
    Ok(())
}

async fn execute_transactions_v4(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    simulation_steps: u32,
) -> client::Result<()> {
    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_200_000);
    let simulation_steps_usize = simulation_steps as usize;

    for chunk_start in (0..simulation_steps_usize).step_by(MAX_CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + MAX_CHUNK_SIZE, simulation_steps_usize);
        println!("Processing steps {}-{}", chunk_start, chunk_end - 1);

        let mut transactions = Vec::new();
        for i in chunk_start..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &VI4::Execute(i as u32),
                vec![AccountMeta::new(stack_account.pubkey(), false)],
            );
            let tx = Transaction::new_signed_with_payer(
                &[limit_instructions.clone(), execute_ix],
                Some(&payer.pubkey()),
                &[payer],
                client.get_latest_blockhash().await?,
            );
            transactions.push(tx);
        }

        send_and_confirm_transactions(client, &transactions).await?;
        println!("Chunk {}-{} completed", chunk_start, chunk_end - 1);
    }
    Ok(())
}

mod prepare_input {
    use felt::Felt;
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::funvec::FunVec;
    use types::swiftness::commitment::types::Decommitment;
    use types::swiftness::global_values::InteractionElements;
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use utils::{CacheStorage, StarkCommitmentTrait};
    use verifier_1::state::BidirectionalStackAccount as Stack1;
    use verifier_2::state::BidirectionalStackAccount as Stack2;
    use verifier_3::state::BidirectionalStackAccount as Stack3;
    use verifier_4::state::BidirectionalStackAccount as Stack4;

    pub fn get_bytes_stage1() -> Vec<u8> {
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();

        let mut stack = Stack1::default();
        stack.proof = proof_verifier.clone();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }

    pub fn get_bytes_stage2(
        stark_commitment: &types::swiftness::stark::types::StarkCommitment<InteractionElements>,
    ) -> Vec<u8> {
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();

        let mut stack = Stack2::default();
        stack.proof = proof_verifier.clone();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();
        stack.set_stark_commitment(stark_commitment);

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }

    pub fn get_bytes_stage3(
        stark_commitment: &types::swiftness::stark::types::StarkCommitment<InteractionElements>,
        queries: &[Felt],
    ) -> Vec<u8> {
        use std::mem::size_of;

        println!("Size of Stack3: {}", size_of::<Stack3>());
        println!(
            "Size of FriVerifyData: {}",
            size_of::<types::swiftness::stark::types::FriVerifyData>()
        );

        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();

        let mut stack = Stack3::default();
        stack.proof = proof_verifier.clone();
        stack.oods_values = proof_verifier
            .unsent_commitment
            .oods_values
            .as_slice()
            .try_into()
            .unwrap();
        stack.set_stark_commitment(stark_commitment);

        let stark_verify_data = types::swiftness::stark::types::FriVerifyData {
            queries: FunVec::from_vec(queries.to_vec()),
            fri_decommitment: Decommitment::default(),
            current_layer: 0,
            layer_queries: FunVec::default(),
            active_query_count: 0,
            working_elements: FunVec::default(),
            working_indices: FunVec::default(),
            working_y_values: FunVec::default(),
            coset_size: Felt::ZERO,
            eval_point: Felt::ZERO,
            sibling_witness: FunVec::default(),
            next_x_inv_value: Felt::ZERO,
            coset_x_inv: Felt::ZERO,
            current_coset_index: 0,
        };
        stack.store_in_cache(&stark_verify_data);

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }

    pub fn get_bytes_stage4(
        stark_commitment: &types::swiftness::stark::types::StarkCommitment<InteractionElements>,
        stark_verify_data: &types::swiftness::stark::types::FriVerifyData,
    ) -> Vec<u8> {
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();

        let mut stack = Stack4::default();
        stack.proof = proof_verifier;
        stack.set_stark_commitment(stark_commitment);
        stack.store_in_cache(stark_verify_data);

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }
}
