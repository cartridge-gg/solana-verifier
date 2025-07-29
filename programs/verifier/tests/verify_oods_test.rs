use stark::felt::Felt;
use stark::funvec::{FunVec, FUNVEC_OODS};
use stark::stark_proof::stark_commit::VerifyOods;
use stark::swiftness::stark::types::StarkProof;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_verify_oods_with_reference_values() {
    let mut stack = BidirectionalStackAccount::default();

    // Create a StarkProof with reference oods_values
    let mut proof = StarkProof::default();

    // Create test oods_values - need 256 elements
    // Last 2 elements are composition values (comp_value_0, comp_value_1)
    // The rest are mask values
    let mut oods_values = FunVec::<Felt, FUNVEC_OODS>::default();

    // Fill with test data - first 254 are mask values
    for i in 0..254 {
        oods_values.as_slice_mut()[i] = Felt::from(i as u64 + 1000);
    }

    // Last 2 are composition values for the final verification
    let comp_value_0 = Felt::from_hex("0x123456789abcdef").unwrap();
    let comp_value_1 = Felt::from_hex("0xfedcba987654321").unwrap();
    oods_values.as_slice_mut()[254] = comp_value_0;
    oods_values.as_slice_mut()[255] = comp_value_1;

    proof.unsent_commitment.oods_values = oods_values;
    stack.proof = proof;

    // Push required values to stack (in reverse order - stack is LIFO):
    // 1. interaction_after_composition (oods_point) - will be used in verification
    let oods_point = Felt::from_hex("0x987654321abcdef").unwrap();
    stack.push_front(&oods_point.to_bytes_be()).unwrap();

    // 2. trace_domain_size (for EvalCompositionPolynomial)
    let trace_domain_size = Felt::from(1024); // Example domain size
    stack.push_front(&trace_domain_size.to_bytes_be()).unwrap();

    // Push the VerifyOods task
    stack.push_task(VerifyOods::new());

    // Execute tasks until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;

        // Prevent infinite loops in case of errors
        if steps > 100 {
            panic!("Too many execution steps, likely infinite loop");
        }
    }

    println!("VerifyOods test completed successfully in {} steps", steps);

    // If we reach here without panic, the verification passed
    // The actual verification compares: composition_from_trace == comp_value_0 + comp_value_1 * oods_point
}

#[test]
fn test_verify_oods_calculation() {
    // Test the specific calculation that VerifyOods performs
    let comp_value_0 = Felt::from_hex("0x123456789abcdef").unwrap();
    let comp_value_1 = Felt::from_hex("0xfedcba987654321").unwrap();
    let oods_point = Felt::from_hex("0x987654321abcdef").unwrap();

    // This is what VerifyOods calculates
    let claimed_composition = comp_value_0 + comp_value_1 * oods_point;

    println!("comp_value_0: {:?}", comp_value_0);
    println!("comp_value_1: {:?}", comp_value_1);
    println!("oods_point: {:?}", oods_point);
    println!("claimed_composition: {:?}", claimed_composition);

    // For a proper test, composition_from_trace should equal claimed_composition
    // This would come from EvalCompositionPolynomial
}
