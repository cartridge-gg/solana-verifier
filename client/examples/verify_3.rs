/// Universal Stack Account Verification Example - ON-CHAIN (PING-PONG)
///
/// This example demonstrates the PING-PONG architecture with 2 accounts alternating
/// ownership between 4 verifier programs. Data is copied between accounts as ownership transfers.
use client::{
    initialize_client, send_and_confirm_transactions, setup_payer, setup_program, Config,
};
use felt::Felt;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use std::{mem::size_of, path::Path};
use utils::{AccountCast, BidirectionalStack, Executable};
use verifier_1::instruction::VerifierInstruction as Verifier1Instruction;
use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;
use verifier_2::instruction::VerifierInstruction as Verifier2Instruction;
use verifier_2::state::BidirectionalStackAccount as Verifier2StackAccount;
use verifier_3::state::BidirectionalStackAccount as Verifier3StackAccount;
use verifier_4::state::BidirectionalStackAccount as Verifier4StackAccount;

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

    println!("\n========================================");
    println!("  PING-PONG Verifier - 2 Accounts");
    println!("========================================\n");

    // Deploy ALL 4 verifier programs
    println!("Deploying program...");

    let verifier3_program_id = setup_program(
        &client,
        &payer,
        &config,
        Path::new("target/deploy/verifier_3.so"),
    )
    .await?;
    println!("✓ Verifier 3: {}", verifier3_program_id);

    // Create TWO accounts for ping-pong
    let account1 = Keypair::new();

    let space = size_of::<Verifier1StackAccount>();
    println!(
        "\n✓ Creating account1: {} (owner: verifier1)",
        account1.pubkey()
    );
    println!("  Account size: {} bytes", space);
    create_account_tx(&client, &payer, &account1, space, &verifier1_program_id).await?;

    // ========== STAGE 1: Verifier1 on Account1 ==========
    println!("\n========== STAGE 1: Verifier1 on Account1 ==========");

    // Prepare initial data for Verifier1 (only needed for Stage 1)
    let stack_bytes = prepare_input::get_bytes_stage1();
    set_account_data_chunked(
        &client,
        &payer,
        &verifier3_program_id,
        &account1,
        &stack_bytes,
    )
    .await?;

    // Push Verify task (Verifier3 uses copied data from account2)
    let verify_task = Verify_Stage_Three::new();
    push_task(
        &client,
        &payer,
        &verifier3_program_id,
        &account1,
        verify_task.to_vec_with_type_tag(),
    )
    .await?;

    // Calculate exact number of steps needed via simulation
    let mut account_data = client.get_account_data(&account1.pubkey()).await?;
    let stack = Verifier3StackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("  Steps in simulation: {}", simulation_steps);

    execute_verifier(
        &client,
        &payer,
        &verifier3_program_id,
        &account1,
        simulation_steps as u32,
        3, // Stage 3
    )
    .await?;

    println!("✓ Stage 3 completed on account1 (owner: verifier3)");

    // Read final results
    let mut account_data = client.get_account_data(&account1.pubkey()).await?;
    let stack = Verifier3StackAccount::cast_mut(&mut account_data);

    assert_eq!(stack.is_empty_back(), true, "Stack should be empty");
    assert_eq!(stack.is_empty_front(), true, "Stack should be empty");
    println!("✓ All verifications passed!");
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
    println!("  Account created successfully");
    Ok(())
}

async fn copy_from_account(
    client: &RpcClient,
    payer: &Keypair,
    dest_program_id: &solana_sdk::pubkey::Pubkey,
    source_account: &Keypair,
    dest_account: &Keypair,
) -> client::Result<()> {
    let copy_ix = Instruction::new_with_borsh(
        *dest_program_id,
        &Verifier2Instruction::CopyFromAccount, // All verifiers have the same instruction
        vec![
            AccountMeta::new_readonly(source_account.pubkey(), false),
            AccountMeta::new(dest_account.pubkey(), false),
        ],
    );

    let tx = Transaction::new_signed_with_payer(
        &[copy_ix],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&tx).await?;
    println!("  ✓ Copied successfully");
    Ok(())
}

async fn transfer_ownership(
    client: &RpcClient,
    payer: &Keypair,
    current_owner_program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    new_owner_program_id: &solana_sdk::pubkey::Pubkey,
) -> client::Result<()> {
    let transfer_ix = Instruction::new_with_borsh(
        *current_owner_program_id,
        &Verifier1Instruction::TransferOwnership, // All verifiers have the same instruction
        vec![
            AccountMeta::new(account.pubkey(), false),
            AccountMeta::new_readonly(*new_owner_program_id, false),
        ],
    );

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&tx).await?;
    println!("  ✓ Ownership transferred successfully");
    Ok(())
}

async fn set_account_data_chunked(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    stack_bytes: &[u8],
) -> client::Result<()> {
    let mut transactions = Vec::new();
    for (chunk_index, chunk) in stack_bytes.chunks(CHUNK_SIZE).enumerate() {
        let set_data_ix = Instruction::new_with_borsh(
            *program_id,
            &Verifier1Instruction::SetAccountData(chunk_index * CHUNK_SIZE, chunk.to_vec()),
            vec![AccountMeta::new(account.pubkey(), false)],
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
    account: &Keypair,
    task_data: Vec<u8>,
) -> client::Result<()> {
    let push_task_ix = Instruction::new_with_borsh(
        *program_id,
        &Verifier1Instruction::PushTask(task_data),
        vec![AccountMeta::new(account.pubkey(), false)],
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

async fn execute_verifier(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &solana_sdk::pubkey::Pubkey,
    account: &Keypair,
    simulation_steps: u32,
    stage_number: u8,
) -> client::Result<()> {
    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
    let simulation_steps_usize = simulation_steps as usize;

    let mut step = 0;
    while step < simulation_steps_usize {
        let mut chunk_size = MAX_CHUNK_SIZE;
        // Stage 2 has stack overflow issues around step 389
        if stage_number == 3 {
            if step >= 0 {
                chunk_size = 1;
            }
            if step >= 2500 {
                chunk_size = 1;
            }
        }
        let chunk_end = std::cmp::min(step + chunk_size, simulation_steps_usize);

        println!("Processing steps {}-{}", step, chunk_end - 1);

        let mut transactions = Vec::new();
        for i in step..chunk_end {
            let execute_ix = Instruction::new_with_borsh(
                *program_id,
                &Verifier1Instruction::Execute(i as u32),
                vec![AccountMeta::new(account.pubkey(), false)],
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
    use swiftness_proof_parser::{
        json_parser, transform::TransformTo, StarkProof as StarkProofParser,
    };
    use types::swiftness::commitment::types::Decommitment;
    use types::{
        funvec::FunVec,
        swiftness::stark::types::{cast_struct_to_slice_mut, StarkCommitment},
    };
    use utils::CacheStorage;
    use verifier_3::state::BidirectionalStackAccount as Stack3;

    /// Prepare initial data for Stage 1
    /// Only Stage 1 needs initial data setup - all other stages use ping-pong copying
    pub fn get_bytes_stage1() -> Vec<u8> {
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
        stack.stark_commitment = StarkCommitment::default();
        let queries = vec![
            "0xd20990",
            "0x1702a2dc",
            "0x233bfb24",
            "0x2fc8f32e",
            "0x367bcdcb",
            "0x44445cc6",
            "0x4bf4ed93",
            "0x8df252ca",
            "0x97a48b5b",
            "0xafea6443",
            "0xc62f63b8",
            "0xd76e5257",
            "0xecca885b",
            "0xedc42f8b",
            "0xf6821efe",
            "0xf7769c26",
        ];
        queries
            .iter()
            .map(|f| Felt::from_hex_unchecked(f))
            .collect::<Vec<Felt>>();

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
}
