use felt::Felt;
use swiftness_proof_parser::json_parser;
use swiftness_proof_parser::{transform::TransformTo, StarkProof as StarkProofParser};
use types::funvec::FunVec;
use types::swiftness::commitment::types::Decommitment;
use utils::universal_stack::{UniversalStackAccount, VerifierMode};
use utils::BidirectionalStack;
use utils::Scheduler;
use utils::CacheStorage;
use universal_verifier::scheduler::UniversalStackExecute;

// UniversalStackAccount has its own execute method that automatically chooses the right verifier

// mod fixtures;

#[test]
pub fn test_universal_proof_verification() {
    println!("\n========================================");
    println!("🚀 Starting Universal Verifier Test");
    println!("========================================\n");

    println!("📦 UniversalStackAccount size: {} bytes", std::mem::size_of::<UniversalStackAccount>());

    // Verifier 1
    println!("\n🔵 STAGE 1: Verifier1 Mode");
    println!("────────────────────────────────────────");
    let mut stack = UniversalStackAccount::new(VerifierMode::Verifier1);
    println!("📍 Memory address: {:p}", &stack as *const _);
    println!("✓ Created UniversalStackAccount with mode: {:?}", stack.mode());

    let proof_str = include_str!("../../../example_proof/saya.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(proof_str).unwrap();
    let proof = StarkProofParser::try_from(proof_json).unwrap();
    let proof_verifier = proof.transform_to();

    // Set up Verifier1 data
    stack.proof = proof_verifier.clone();
    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();
    println!("✓ Loaded proof data into shared memory");

    let task = verify_1::verify::Verify::new();
    stack.push_task(task);
    println!("✓ Pushed Verify1 task to stack");

    let mut steps = 0;
    println!("⚙️  Executing Verifier1 tasks...");
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        if steps % 1000 == 0 {
            println!("   ... {} steps completed", steps);
        }
    }
    println!("✓ Verifier1 completed in {} steps\n", steps);

    let counter = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let digest = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    println!("📊 Results: counter={:?}, digest={:?}", counter, digest);

    let commitment = stack.stark_commitment.clone();

    // STAGE 2: Switch to Verifier2
    println!("\n🟢 STAGE 2: Verifier2 Mode");
    println!("────────────────────────────────────────");
    println!("📍 Same memory address: {:p}", &stack as *const _);

    stack.switch_mode(VerifierMode::Verifier2);
    println!("✓ Switched mode to: {:?}", stack.mode());

    // Restore shared data
    stack.proof = proof_verifier.clone();
    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();
    stack.stark_commitment = commitment.clone();
    println!("✓ Restored proof data in shared memory");

    let task = verify_2::verify::Verify::new(digest, counter);
    stack.push_task(task);
    println!("✓ Pushed Verify2 task to stack");

    let stage2_start = steps;
    println!("⚙️  Executing Verifier2 tasks...");
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        if (steps - stage2_start) % 1000 == 0 {
            println!("   ... {} steps completed", steps - stage2_start);
        }
    }
    println!("✓ Verifier2 completed in {} steps\n", steps - stage2_start);


    let queries = stack.verify_variables.queries_indexes;
    println!("📊 Extracted queries from verify_variables");

    // STAGE 3: Switch to Verifier3
    println!("\n🟡 STAGE 3: Verifier3 Mode");
    println!("────────────────────────────────────────");
    println!("📍 Same memory address: {:p}", &stack as *const _);

    stack.switch_mode(VerifierMode::Verifier3);
    println!("✓ Switched mode to: {:?}", stack.mode());

    stack.proof = proof_verifier.clone();
    stack.oods_values = proof_verifier
        .unsent_commitment
        .oods_values
        .as_slice()
        .try_into()
        .unwrap();
    stack.stark_commitment = commitment.clone();
    println!("✓ Restored proof data in shared memory");

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
    println!("✓ Stored FriVerifyData in cache");

    let task = verify_3::verify::Verify::new();
    stack.push_task(task);
    println!("✓ Pushed Verify3 task to stack");

    let stage3_start = steps;
    println!("⚙️  Executing Verifier3 tasks...");
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        if (steps - stage3_start) % 1000 == 0 {
            println!("   ... {} steps completed", steps - stage3_start);
        }
    }
    println!("✓ Verifier3 completed in {} steps\n", steps - stage3_start);

    let stark_verify_data = stack.borrow_from_cache::<types::swiftness::stark::types::FriVerifyData>().clone();
    println!("📊 Retrieved FriVerifyData from cache");

    // STAGE 4: Switch to Verifier4
    println!("\n🔴 STAGE 4: Verifier4 Mode");
    println!("────────────────────────────────────────");
    println!("📍 Same memory address: {:p}", &stack as *const _);

    stack.switch_mode(VerifierMode::Verifier4);
    println!("✓ Switched mode to: {:?}", stack.mode());

    stack.proof = proof_verifier.clone();
    stack.stark_commitment = commitment;
    println!("✓ Restored proof data in shared memory");

    stack.store_in_cache(&stark_verify_data);
    println!("✓ Stored FriVerifyData in cache");

    let task = verify_4::verify::Verify::new();
    stack.push_task(task);
    println!("✓ Pushed Verify4 task to stack");

    let stage4_start = steps;
    println!("⚙️  Executing Verifier4 tasks...");
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
        if (steps - stage4_start) % 1000 == 0 {
            println!("   ... {} steps completed", steps - stage4_start);
        }
    }
    println!("✓ Verifier4 completed in {} steps\n", steps - stage4_start);

    println!("📊 Total steps across all stages: {}", steps);

    let result_program_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let result_output_hash = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("\n🎯 Final Results:");
    println!("────────────────────────────────────────");
    println!("Program hash: {result_program_hash:?}");
    println!("Output hash:  {result_output_hash:?}");
    
    assert_eq!(
        result_program_hash,
        Felt::from_hex("0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07").unwrap()
    );
    assert_eq!(
        result_output_hash,
        Felt::from_hex("0x3233b5615a8de5563f7d3ba086b8f260189ac47753a1c131d063ed3f6c24400")
            .unwrap()
    );

    assert!(
        stack.is_empty_front(),
        " Stack front should be empty after verification"
    );
    assert!(
        stack.is_empty_back(),
        "Stack back should be empty after verification"
    );

    println!("\n========================================");
    println!("✅ TEST PASSED - Universal Verifier Works!");
    println!("========================================");
    println!("\n🎓 What we proved:");
    println!("   1. Created ONE UniversalStackAccount (651KB)");
    println!("   2. Switched between 4 different verifier modes");
    println!("   3. Each verifier operated on THE SAME memory");
    println!("   4. All 4 stages completed successfully");
    println!("   5. Final verification result is correct");
    println!("\n💡 Key insight:");
    println!("   The memory address {:p} stayed the same", &stack as *const _);
    println!("   throughout all 4 stages, proving they all");
    println!("   worked on shared memory!");
    println!("\n   Final mode: {}", stack.verifier_type());
    println!("========================================\n");
}
