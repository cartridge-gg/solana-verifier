use client::{
    initialize_client, send_and_confirm_transactions, setup_payer, setup_program, ClientError,
    Config,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use std::path::Path;
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};
use types::swiftness::stark::types::{cast_struct_to_slice, StarkProof};
use utils::AccountCast;
use utils::BidirectionalStack;
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

pub const CHUNK_SIZE: usize = 1000;
#[repr(C)]
pub struct Input {
    pub front_index: usize,
    pub back_index: usize,
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

    let account_data_after_set_proof = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast(&account_data_after_set_proof);

    println!("Stack front index: {:?}", stack.front_index);
    println!("Stack back index: {:?}", stack.back_index);
    println!("Stack is empty front: {:?}", stack.is_empty_front());
    println!("Stack is empty back: {:?}", stack.is_empty_back());
    Ok(())
}
