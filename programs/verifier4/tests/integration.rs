use felt::Felt;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier_1::state::BidirectionalStackAccount as Verifier1StackAccount;
use verifier_2::state::BidirectionalStackAccount as Verifier2StackAccount;
use verifier_3::state::BidirectionalStackAccount as Verifier3StackAccount;
use verifier_4::state::BidirectionalStackAccount as Verifier4StackAccount;
mod fixtures;

#[test]
pub fn test_proof_verification() {
    println!("\n========================================");
    println!("  PING-PONG Verifier - 2 Stacks (Off-chain)");
    println!("========================================\n");

    // Prepare initial proof data
    let proof_str = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();
    let proof_verifier = proof.transform_to();

    // Create TWO stacks for ping-pong
    let mut stack1 = Verifier1StackAccount::default();
    let mut stack2 = Verifier2StackAccount::default();
    let mut stack3 = Verifier3StackAccount::default();
    let mut stack4 = Verifier4StackAccount::default();

    // ========== STAGE 1: Verifier1 on Stack1 ==========
    println!("========== STAGE 1: Verifier1 on Stack1 ==========");

    stack1.proof = proof_verifier.clone();
    stack1.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();

    let task = verify_1::verify::Verify::new();
    stack1.push_task(task);

    let mut steps = 0;
    while !stack1.is_empty_back() {
        stack1.execute();
        println!("Step stage 1: {}", steps);
        steps += 1;
    }

    println!("✓ Stage 1 completed on stack1 ({} steps)", steps);

    // ========== STAGE 2: Verifier2 on Stack2 (PING-PONG: copy stack1 → stack2) ==========
    println!("\n========== STAGE 2: Verifier2 on Stack2 ==========");
    println!("  Copying stack1 → stack2...");

    // Copy entire stack1 to stack2 (simulates CopyFromAccount instruction)
    unsafe {
        std::ptr::copy_nonoverlapping(
            &stack1 as *const _ as *const u8,
            &mut stack2 as *mut _ as *mut u8,
            std::mem::size_of::<Verifier1StackAccount>(),
        );
    }

    let task = verify_2::verify::Verify::default();
    stack2.push_task(task);

    while !stack2.is_empty_back() {
        stack2.execute();
        steps += 1;
    }

    println!("✓ Stage 2 completed on stack2");

    // ========== STAGE 3: Verifier3 on Stack1 (PING-PONG: copy stack2 → stack1) ==========
    println!("\n========== STAGE 3: Verifier3 on Stack1 ==========");
    println!("  Copying stack2 → stack1...");

    // Copy entire stack2 to stack1 (simulates CopyFromAccount instruction)
    unsafe {
        std::ptr::copy_nonoverlapping(
            &stack2 as *const _ as *const u8,
            &mut stack3 as *mut _ as *mut u8,
            std::mem::size_of::<Verifier2StackAccount>(),
        );
    }

    let task = verify_3::verify::Verify::new();
    stack3.push_task(task);

    let mut steps_stage_3 = 0;
    while !stack3.is_empty_back() {
        stack3.execute();
        steps += 1;
        steps_stage_3 += 1;
    }
    println!("✓ Stage 3 completed on stack1 ({} steps)", steps_stage_3);

    // ========== STAGE 4: Verifier4 on Stack2 (PING-PONG: copy stack1 → stack2) ==========
    println!("\n========== STAGE 4: Verifier4 on Stack2 ==========");
    println!("  Copying stack1 → stack2...");

    // Copy entire stack1 to stack2 (simulates CopyFromAccount instruction)
    unsafe {
        std::ptr::copy_nonoverlapping(
            &stack3 as *const _ as *const u8,
            &mut stack4 as *mut _ as *mut u8,
            std::mem::size_of::<Verifier3StackAccount>(),
        );
    }

    let task = verify_4::verify::Verify::new();
    stack4.push_task(task);

    let mut steps_stage_4 = 0;
    while !stack4.is_empty_back() {
        stack4.execute();
        // steps_stage_4 += 1;
        // println!("Step stage 4: {}", steps_stage_4);
        steps += 1;
    }

    println!("✓ Stage 4 completed on stack2");
    println!("\nTotal steps: {}", steps);

    // Read final results from stack2
    let result_program_hash = Felt::from_bytes_be_slice(stack4.borrow_front());
    stack4.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack4.borrow_front());
    stack4.pop_front();

    println!("\n========== VERIFICATION RESULTS ==========");
    println!("  Program Hash: {:?}", result_program_hash);
    println!("  Output Hash:  {:?}", result_output_hash);

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

    assert!(
        stack4.is_empty_front(),
        "Stack front should be empty after verification"
    );
    assert!(
        stack4.is_empty_back(),
        "Stack back should be empty after verification"
    );

    println!("\n✓ All 4 stages completed successfully using PING-PONG architecture!");
    println!("✓ All verifications passed!");
}
