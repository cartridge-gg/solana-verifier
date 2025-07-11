use client::{
    initialize_client, interact_with_program_instructions, send_and_confirm_transactions,
    setup_payer, setup_program, ClientError, Config,
};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, instruction::{AccountMeta, Instruction}, signature::{Keypair, Signer}, transaction::Transaction
};
use solana_system_interface::instruction::create_account;
use stark::{felt::Felt, swiftness::stark::types::cast_struct_to_slice};
use std::{mem::size_of, path::Path};
use utils::{AccountCast, BidirectionalStack, Executable};
use verifier::{instruction::VerifierInstruction, state::BidirectionalStackAccount};
use starknet_crypto::{pedersen_hash, poseidon_hash_many, Felt as StarkFelt};
use swiftness_proof_parser::{json_parser, transform::TransformTo, StarkProof as StarkProofParser};

// Import your GetHash task
use stark::stark_proof::get_hash::GetHash;

pub const CHUNK_SIZE: usize = 1000;

/// E2E test for GetHash task on Solana
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

    // Initialize the stack account
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

    println!("\nGetHash Task on Solana");
    println!("=====================");

    // Parse the proof from JSON and transform it to verifier format
    let input = include_str!("../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();

    let mut proof_verifier = proof.transform_to();
    
    // // Replace the public_input with the one from get() for faster testing
    // // This uses a much smaller dataset than the full proof
    proof_verifier.public_input = get_test_public_input();
    
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

    // Create GetHash task with n_verifier_friendly_commitment_layers
    let n_verifier_friendly_commitment_layers = Felt::from(4u32);
    let get_hash_task = GetHash::new(n_verifier_friendly_commitment_layers);

    println!(
        "Using GetHash with TYPE_TAG: {}",
        GetHash::TYPE_TAG
    );
    println!(
        "n_verifier_friendly_commitment_layers: {}",
        n_verifier_friendly_commitment_layers
    );

    // Push the GetHash task to the stack
    let push_task_ix = Instruction::new_with_borsh(
        program_id,
        &VerifierInstruction::PushTask(get_hash_task.to_vec_with_type_tag()),
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
    println!("GetHash task pushed: {signature}");

    // Get the current stack state to run simulation
    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;

    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let simulation_steps = stack.simulate();
    println!("Steps in simulation: {simulation_steps}");

    let limit_instructions = ComputeBudgetInstruction::set_compute_unit_limit(800_000);

    // Execute all steps until task is complete
    let mut transactions = Vec::new();
    for i in 0..simulation_steps {
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
    
    // Send all transactions
    send_and_confirm_transactions(&client, &transactions).await?;
    println!("All execution steps completed");

    // Read and display the result
    let mut account_data = client
        .get_account_data(&stack_account.pubkey())
        .await
        .map_err(ClientError::SolanaClientError)?;
    
    let stack = BidirectionalStackAccount::cast_mut(&mut account_data);
    let result_bytes = stack.borrow_front();
    let result = Felt::from_bytes_be_slice(result_bytes);
    
    // Clean up the stack (remove the result)
    stack.pop_front();
    stack.pop_front();
    stack.pop_front();
    
    println!("\nGetHash result: {:?}", result);
    println!("Stack front index: {}", stack.front_index);
    println!("Stack back index: {}", stack.back_index);

    // Calculate expected result using the original synchronous method
    let expected_result = calculate_expected_get_hash(&proof_verifier.public_input, n_verifier_friendly_commitment_layers);
    
    println!("Expected result: {:?}", expected_result);
    
    // Verify the result matches
    assert_eq!(result, expected_result);
    println!("\nGetHash successfully executed on Solana!");

    Ok(())
}

// Helper function to calculate expected result using the original synchronous method
fn calculate_expected_get_hash(public_input: &stark::swiftness::air::public_memory::PublicInput, n_verifier_friendly_commitment_layers: Felt) -> Felt {
    // This implements the original get_hash logic synchronously for comparison
    
    let mut main_page_hash = StarkFelt::ZERO;
    for memory in public_input.main_page.0.iter() {
        let address_bytes = memory.address.to_bytes_be();
        let value_bytes = memory.value.to_bytes_be();
        let address_starknet = StarkFelt::from_bytes_be(&address_bytes);
        let value_starknet = StarkFelt::from_bytes_be(&value_bytes);
        main_page_hash = pedersen_hash(&main_page_hash, &address_starknet);
        main_page_hash = pedersen_hash(&main_page_hash, &value_starknet);
    }
    let length_multiplier_bytes = (Felt::TWO * Felt::from(public_input.main_page.0.len())).to_bytes_be();
    let length_multiplier = StarkFelt::from_bytes_be(&length_multiplier_bytes);
    main_page_hash = pedersen_hash(&main_page_hash, &length_multiplier);

    let mut hash_data = vec![
        StarkFelt::from_bytes_be(&n_verifier_friendly_commitment_layers.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.log_n_steps.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.range_check_min.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.range_check_max.to_bytes_be()),
        StarkFelt::from_bytes_be(&public_input.layout.to_bytes_be()),
    ];

    if let Some(dynamic_params) = &public_input.dynamic_params {
        let dynamic_params_vec: Vec<u32> = (*dynamic_params).into();
        hash_data.extend(dynamic_params_vec.into_iter().map(|x| StarkFelt::from(x)));
    }

    // Add segments
    hash_data.extend(public_input.segments.iter().flat_map(|s| {
        let begin_addr_bytes = s.begin_addr.to_bytes_be();
        let stop_ptr_bytes = s.stop_ptr.to_bytes_be();
        vec![
            StarkFelt::from_bytes_be(&begin_addr_bytes),
            StarkFelt::from_bytes_be(&stop_ptr_bytes),
        ]
    }));

    // Add padding values
    let padding_addr_bytes = public_input.padding_addr.to_bytes_be();
    let padding_value_bytes = public_input.padding_value.to_bytes_be();
    hash_data.push(StarkFelt::from_bytes_be(&padding_addr_bytes));
    hash_data.push(StarkFelt::from_bytes_be(&padding_value_bytes));
    
    // Add continuous_page_headers.len() + 1
    hash_data.push(StarkFelt::from(public_input.continuous_page_headers.len() + 1));

    // Add main page info
    hash_data.push(StarkFelt::from(public_input.main_page.0.len()));
    hash_data.push(main_page_hash);

    hash_data.extend(
        public_input.continuous_page_headers.iter().flat_map(|h| {
            let start_address_bytes = h.start_address.to_bytes_be();
            let size_bytes = h.size.to_bytes_be();
            let hash_bytes = h.hash.to_bytes_be();
            vec![
                StarkFelt::from_bytes_be(&start_address_bytes),
                StarkFelt::from_bytes_be(&size_bytes),
                StarkFelt::from_bytes_be(&hash_bytes),
            ]
        }),
    );

    let result_starknet = poseidon_hash_many(&hash_data[..]);
    Felt::from_bytes_be(&result_starknet.to_bytes_be())
}

// Helper function to get a smaller test public_input for faster testing
fn get_test_public_input() -> stark::swiftness::air::public_memory::PublicInput {
    use stark::funvec::FunVec;
    use stark::swiftness::air::types::{AddrValue, Page, SegmentInfo};
    
    stark::swiftness::air::public_memory::PublicInput {
        log_n_steps: Felt::from_hex_unchecked("0xe"),
        range_check_min: Felt::from_hex_unchecked("0x7ffa"),
        range_check_max: Felt::from_hex_unchecked("0x8001"),
        layout: Felt::from_hex_unchecked("0x726563757273697665"),
        dynamic_params: None,
        segments: FunVec::from_vec(vec![
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1"),
                stop_ptr: Felt::from_hex_unchecked("0x5"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x25"),
                stop_ptr: Felt::from_hex_unchecked("0x68"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x68"),
                stop_ptr: Felt::from_hex_unchecked("0x6a"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x6a"),
                stop_ptr: Felt::from_hex_unchecked("0x6a"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x1ea"),
                stop_ptr: Felt::from_hex_unchecked("0x1ea"),
            },
            SegmentInfo {
                begin_addr: Felt::from_hex_unchecked("0x9ea"),
                stop_ptr: Felt::from_hex_unchecked("0x9ea"),
            },
        ]),
        padding_addr: Felt::from_hex_unchecked("0x1"),
        padding_value: Felt::from_hex_unchecked("0x40780017fff7fff"),
        main_page: Page(FunVec::from_vec(vec![
            AddrValue {
                address: Felt::from_hex_unchecked("0x1"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x2"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x3"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x4"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x5"),
                value: Felt::from_hex_unchecked("0x10780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x6"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x7"),
                value: Felt::from_hex_unchecked("0x40780017fff7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x8"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x9"),
                value: Felt::from_hex_unchecked("0x400380007ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xa"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xb"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xc"),
                value: Felt::from_hex_unchecked("0x480680017fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xd"),
                value: Felt::from_hex_unchecked("0x1"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xe"),
                value: Felt::from_hex_unchecked("0x480a80007fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0xf"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x10"),
                value: Felt::from_hex_unchecked("0x9"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x11"),
                value: Felt::from_hex_unchecked("0x400280017ffa7fff"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x12"),
                value: Felt::from_hex_unchecked("0x482680017ffa8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x13"),
                value: Felt::from_hex_unchecked("0x2"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x14"),
                value: Felt::from_hex_unchecked("0x480a7ffb7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x15"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x16"),
                value: Felt::from_hex_unchecked("0x480a7ffd7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x17"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x18"),
                value: Felt::from_hex_unchecked("0x20780017fff7ffd"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x19"),
                value: Felt::from_hex_unchecked("0x4"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1a"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1b"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1c"),
                value: Felt::from_hex_unchecked("0x480a7ffc7fff8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1d"),
                value: Felt::from_hex_unchecked("0x482a7ffc7ffb8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1e"),
                value: Felt::from_hex_unchecked("0x482680017ffd8000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x1f"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000011000000000000000000000000000000000000000000000000",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x20"),
                value: Felt::from_hex_unchecked("0x1104800180018000"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x21"),
                value: Felt::from_hex_unchecked(
                    "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffff9",
                ),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x22"),
                value: Felt::from_hex_unchecked("0x208b7fff7fff7ffe"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x23"),
                value: Felt::from_hex_unchecked("0x25"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x24"),
                value: Felt::from_hex_unchecked("0x0"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x25"),
                value: Felt::from_hex_unchecked("0x68"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x26"),
                value: Felt::from_hex_unchecked("0x6a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x27"),
                value: Felt::from_hex_unchecked("0x1ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x28"),
                value: Felt::from_hex_unchecked("0x9ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x64"),
                value: Felt::from_hex_unchecked("0x6a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x65"),
                value: Felt::from_hex_unchecked("0x6a"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x66"),
                value: Felt::from_hex_unchecked("0x1ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x67"),
                value: Felt::from_hex_unchecked("0x9ea"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x68"),
                value: Felt::from_hex_unchecked("0xa"),
            },
            AddrValue {
                address: Felt::from_hex_unchecked("0x69"),
                value: Felt::from_hex_unchecked("0x90"),
            },
        ])),
        continuous_page_headers: FunVec::from_vec(vec![]),
    }
}