/// Universal Stack Account Verification Example - ON-CHAIN
///
/// This example demonstrates using UniversalStackAccount with the universal-verifier
/// program on Solana. It uses a SINGLE account that switches modes between verifiers.

use client::{
    initialize_client, send_and_confirm_transactions, setup_payer, setup_program, Config,
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
use types::swiftness::stark::types::FriVerifyData;
use universal_verifier::instruction::UniversalVerifierInstruction;
use universal_verifier::scheduler::UniversalStackExecute;
use utils::{AccountCast, BidirectionalStack, CacheStorage, Executable, UniversalStackAccount, VerifierMode};

use verify_1::verify::Verify as Verify_Stage_One;
use verify_2::verify::Verify as Verify_Stage_Two;
use verify_3::verify::Verify as Verify_Stage_Three;
use verify_4::verify::Verify as Verify_Stage_Four;

pub const CHUNK_SIZE: usize = 1000;
const MAX_CHUNK_SIZE: usize = 3000;

fn simulate_steps(stack: &mut UniversalStackAccount) -> u128 {
    let mut simulation_steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        simulation_steps += 1;
    }
    simulation_steps
}

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

    println!("\n========================================");
    println!("  Universal Verifier - Multi-Program CPI");
    println!("========================================\n");

    // Deploy ALL programs: universal_verifier + verifier_1/2/3/4
    println!("Deploying programs...");

    let universal_program_id = setup_program(&client, &payer, &config,
        Path::new("target/deploy/universal_verifier.so")).await?;
    println!("✓ Universal Verifier (orchestrator): {}", universal_program_id);

    let verifier1_program_id = setup_program(&client, &payer, &config,
        Path::new("target/deploy/verifier_1.so")).await?;
    println!("✓ Verifier 1 (worker): {}", verifier1_program_id);

    let verifier2_program_id = setup_program(&client, &payer, &config,
        Path::new("target/deploy/verifier_2.so")).await?;
    println!("✓ Verifier 2 (worker): {}", verifier2_program_id);

    let verifier3_program_id = setup_program(&client, &payer, &config,
        Path::new("target/deploy/verifier_3.so")).await?;
    println!("✓ Verifier 3 (worker): {}", verifier3_program_id);

    let verifier4_program_id = setup_program(&client, &payer, &config,
        Path::new("target/deploy/verifier_4.so")).await?;
    println!("✓ Verifier 4 (worker): {}", verifier4_program_id);

    // TODO: Update verifier_programs.rs with these IDs and recompile
    println!("\n⚠️  IMPORTANT: Update programs/universal_verifier/src/verifier_programs.rs");
    println!("   with the following Program IDs and rebuild:");
    println!("   VERIFIER_1_PROGRAM_ID = {}", verifier1_program_id);
    println!("   VERIFIER_2_PROGRAM_ID = {}", verifier2_program_id);
    println!("   VERIFIER_3_PROGRAM_ID = {}", verifier3_program_id);
    println!("   VERIFIER_4_PROGRAM_ID = {}", verifier4_program_id);

    // Create ONE universal stack account (owned by universal_verifier)
    let stack_account = Keypair::new();
    println!("\n✓ Creating single universal account: {}", stack_account.pubkey());

    let space = size_of::<UniversalStackAccount>();
    println!("  UniversalStackAccount size: {} bytes", space);
    create_account_tx(&client, &payer, &stack_account, space, &universal_program_id).await?;

    // ========== STAGE 1: Verifier1 Mode ==========
    println!("\n========== STAGE 1: Verifier1 Mode ==========");

    // Prepare account data for Verifier1
    let stack_bytes = prepare_input::get_bytes_stage1();
    set_account_data_chunked(&client, &payer, &universal_program_id, &stack_account, &stack_bytes).await?;

    // Switch to Verifier1 mode
    switch_mode(&client, &payer, &universal_program_id, &stack_account, VerifierMode::Verifier1).await?;
    println!("✓ Switched to Verifier1 mode");

    // Push Verify task
    let verify_task = Verify_Stage_One::new();
    push_task(&client, &payer, &universal_program_id, &stack_account, verify_task.to_vec_with_type_tag()).await?;

    // Calculate exact number of steps needed via simulation
    let mut account_data = client.get_account_data(&stack_account.pubkey()).await?;
    let stack = UniversalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = simulate_steps(stack);
    println!("  Steps in simulation: {}", simulation_steps);

    execute_transactions(&client, &payer, &universal_program_id, &stack_account, simulation_steps as u32, &verifier1_program_id).await?;

    println!("Transactions executed");
    // Read results
    let mut account_data = client.get_account_data(&stack_account.pubkey()).await?;
    let stack = UniversalStackAccount::cast_mut(&mut account_data);

    let counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let commitment = stack.stark_commitment.clone();

    println!("  Counter: {:?}", counter);
    println!("  Digest: {:?}", digest);
    println!("✓ Stage 1 completed");

    // ========== STAGE 2: Verifier2 Mode ==========
    println!("\n========== STAGE 2: Verifier2 Mode ==========");

    // Prepare account data for Verifier2
    let stack_bytes = prepare_input::get_bytes_stage2(&commitment);
    set_account_data_chunked(&client, &payer, &universal_program_id, &stack_account, &stack_bytes).await?;

    // Switch to Verifier2 mode
    switch_mode(&client, &payer, &universal_program_id, &stack_account, VerifierMode::Verifier2).await?;
    println!("✓ Switched to Verifier2 mode");

    // Push Verify task
    let verify_task = Verify_Stage_Two::new(digest, counter);
    push_task(&client, &payer, &universal_program_id, &stack_account, verify_task.to_vec_with_type_tag()).await?;

    // Execute verification (max steps as safety limit)
    let max_steps = 50000; // Will stop early when task completes
    println!("  Executing (max {} steps)", max_steps);

    execute_transactions(&client, &payer, &universal_program_id, &stack_account, max_steps, &verifier2_program_id).await?;

    // Read results
    let mut account_data = client.get_account_data(&stack_account.pubkey()).await?;
    let stack = UniversalStackAccount::cast_mut(&mut account_data);

    let queries = stack.verify_variables.queries_indexes.to_vec();
    println!("  Extracted {} queries", queries.len());
    println!("✓ Stage 2 completed");

    // ========== STAGE 3: Verifier3 Mode ==========
    println!("\n========== STAGE 3: Verifier3 Mode ==========");

    // Prepare account data for Verifier3
    let stack_bytes = prepare_input::get_bytes_stage3(&commitment, &queries);
    set_account_data_chunked(&client, &payer, &universal_program_id, &stack_account, &stack_bytes).await?;

    // Switch to Verifier3 mode
    switch_mode(&client, &payer, &universal_program_id, &stack_account, VerifierMode::Verifier3).await?;
    println!("✓ Switched to Verifier3 mode");

    // Push Verify task
    let verify_task = Verify_Stage_Three::new();
    push_task(&client, &payer, &universal_program_id, &stack_account, verify_task.to_vec_with_type_tag()).await?;

    // Execute verification (max steps as safety limit)
    let max_steps = 50000; // Will stop early when task completes
    println!("  Executing (max {} steps)", max_steps);

    execute_transactions(&client, &payer, &universal_program_id, &stack_account, max_steps, &verifier3_program_id).await?;

    // Read results from cache
    let mut account_data = client.get_account_data(&stack_account.pubkey()).await?;
    let stack = UniversalStackAccount::cast_mut(&mut account_data);
    let stark_verify_data = stack.borrow_from_cache::<FriVerifyData>().clone();
    println!("✓ Stage 3 completed");

    // ========== STAGE 4: Verifier4 Mode ==========
    println!("\n========== STAGE 4: Verifier4 Mode ==========");

    // Prepare account data for Verifier4
    let stack_bytes = prepare_input::get_bytes_stage4(&commitment, &stark_verify_data);
    set_account_data_chunked(&client, &payer, &universal_program_id, &stack_account, &stack_bytes).await?;

    // Switch to Verifier4 mode
    switch_mode(&client, &payer, &universal_program_id, &stack_account, VerifierMode::Verifier4).await?;
    println!("✓ Switched to Verifier4 mode");

    // Push Verify task
    let verify_task = Verify_Stage_Four::new();
    push_task(&client, &payer, &universal_program_id, &stack_account, verify_task.to_vec_with_type_tag()).await?;

    // Execute verification (max steps as safety limit)
    let max_steps = 50000; // Will stop early when task completes
    println!("  Executing (max {} steps)", max_steps);

    execute_transactions(&client, &payer, &universal_program_id, &stack_account, max_steps, &verifier4_program_id).await?;

    // Read final results
    let mut account_data = client.get_account_data(&stack_account.pubkey()).await?;
    let stack = UniversalStackAccount::cast_mut(&mut account_data);

    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("\n========== VERIFICATION RESULTS ==========");
    println!("  Program Hash: {:?}", result_program_hash);
    println!("  Output Hash:  {:?}", result_output_hash);

    // Verify expected values
    assert_eq!(
        result_program_hash,
        Felt::from_hex("0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07").unwrap(),
        "Program hash mismatch"
    );
    assert_eq!(
        result_output_hash,
        Felt::from_hex("0x3233b5615a8de5563f7d3ba086b8f260189ac47753a1c131d063ed3f6c24400").unwrap(),
        "Output hash mismatch"
    );

    println!("\n✓ All 4 stages completed successfully using ONE account!");
    println!("✓ All verifications passed!");
    println!("✓ Universal verifier test completed on Solana!");

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

    client.send_and_confirm_transaction(&create_account_tx).await?;
    println!("  Account created successfully");
    Ok(())
}

async fn switch_mode(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    mode: VerifierMode,
) -> client::Result<()> {
    let switch_mode_ix = Instruction::new_with_borsh(
        *program_id,
        &UniversalVerifierInstruction::SwitchMode(mode),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let tx = Transaction::new_signed_with_payer(
        &[switch_mode_ix],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&tx).await?;
    Ok(())
}

async fn set_account_data_chunked(
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
            &UniversalVerifierInstruction::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
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
    println!("  Account data set successfully");
    Ok(())
}

async fn push_task(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    task_data: Vec<u8>,
) -> client::Result<()> {
    let push_task_ix = Instruction::new_with_borsh(
        *program_id,
        &UniversalVerifierInstruction::PushTask(task_data),
        vec![AccountMeta::new(stack_account.pubkey(), false)],
    );

    let tx = Transaction::new_signed_with_payer(
        &[push_task_ix],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&tx).await?;
    println!("  Task pushed successfully");
    Ok(())
}

async fn execute_transactions(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    stack_account: &Keypair,
    simulation_steps: u32,
    verifier_program_id: &solana_sdk::pubkey::Pubkey,
) -> client::Result<()> {
       let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
    let simulation_steps_usize = simulation_steps as usize;

    let mut step = 0;
    while step < simulation_steps_usize {
        let chunk_size = if step >= 24500 { 1 } else { MAX_CHUNK_SIZE };
        let chunk_end = std::cmp::min(step + chunk_size, simulation_steps_usize);

        println!("Processing steps {}-{}", step, chunk_end - 1);

        let mut transactions = Vec::new();
        for i in step..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &UniversalVerifierInstruction::ExecuteWithProgramId(i as u32, verifier_program_id.to_bytes()),
                vec![
                    AccountMeta::new(stack_account.pubkey(), false),
                    AccountMeta::new(*verifier_program_id, false),
                ],
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
        println!("Chunk {}-{} completed", step, chunk_end - 1);

        step = chunk_end;
    }
    Ok(())
}

mod prepare_input {
    use felt::Felt;
    use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
    use types::funvec::FunVec;
    use types::swiftness::commitment::types::Decommitment;
    use types::swiftness::global_values::InteractionElements;
    use types::swiftness::stark::types::cast_struct_to_slice_mut;
    use utils::{CacheStorage, StarkCommitmentTrait, UniversalStackAccount, VerifierMode};

    pub fn get_bytes_stage1() -> Vec<u8> {
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();

        let mut stack = UniversalStackAccount::new(VerifierMode::Verifier1);
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

        let mut stack = UniversalStackAccount::new(VerifierMode::Verifier2);
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
        let proof_str = include_str!("../../example_proof/saya.json");
        let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
        let proof = StarkProofParser::try_from(proof_json).unwrap();
        let proof_verifier = proof.transform_to();

        let mut stack = UniversalStackAccount::new(VerifierMode::Verifier3);
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

        let mut stack = UniversalStackAccount::new(VerifierMode::Verifier4);
        stack.proof = proof_verifier;
        stack.set_stark_commitment(stark_commitment);
        stack.store_in_cache(stark_verify_data);

        cast_struct_to_slice_mut(&mut stack).to_vec()
    }
}
