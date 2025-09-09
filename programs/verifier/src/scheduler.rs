use crate::state::BidirectionalStackAccount;
use utils::{BidirectionalStack, Executable, Scheduler};

// Include the generated dispatch code
include!(concat!(env!("OUT_DIR"), "/verifier_executable_dispatch.rs"));

impl Scheduler for BidirectionalStackAccount {}

impl BidirectionalStackAccount {
    pub fn execute(&mut self) {
        let data = self.borrow_back();
        let task_name = get_task_name(&data);
        *self.executed_tasks.entry(task_name).or_insert(0) += 1;

        let (tasks, is_finished) = execute(self);

        if is_finished {
            self.pop_back();
        }

        for task in tasks.iter().rev() {
            let _ = self.push_back(task);
        }
    }
}

pub fn get_task_name(data: &[u8]) -> String {
    let type_tag = u32::from_be_bytes(data[0..4].try_into().unwrap());
    match type_tag {
        // TYPE_TAG from stark crate
        stark::stark_proof::get_hash::GetHash::TYPE_TAG => {
            "GetHash".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::mul::MulInternal::TYPE_TAG => {
            "MulInternal".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::eval_composition_polynomial::EvalCompositionPolynomial::TYPE_TAG => {
            "EvalCompositionPolynomial".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::fib::Fibonacci::TYPE_TAG => {
            "Fibonacci".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::StarkVerify::TYPE_TAG => {
            "StarkVerify".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::traces_commit::TracesCommit::TYPE_TAG => {
            "TracesCommit".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::verify::Verify::TYPE_TAG => {
            "Verify".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::increment::Increment::TYPE_TAG => {
            "Increment".to_string()
        },
        // TYPE_TAG from stark crate
        stark::poseidon::PoseidonHash::TYPE_TAG => {
            "PoseidonHash".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::eval_oods_boundary_poly_at_points::ComputeQueryPoints::TYPE_TAG => {
            "ComputeQueryPoints".to_string()
        },
        // TYPE_TAG from stark crate
        stark::swiftness::transcript::TranscriptReadFeltVector::TYPE_TAG => {
            "TranscriptReadFeltVector".to_string()
        },
        // TYPE_TAG from stark crate
        stark::swiftness::transcript::TranscriptRandomFelt::TYPE_TAG => {
            "TranscriptRandomFelt".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::eval_composition_polynomial_inner::EvalCompositionPolynomialInner::TYPE_TAG => {
            "EvalCompositionPolynomialInner".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::traces_commit::VectorCommit::TYPE_TAG => {
            "VectorCommit".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::compute_root_recursive::ComputeRootRecursive::TYPE_TAG => {
            "ComputeRootRecursive".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::proof_of_work::ProofOfWork::TYPE_TAG => {
            "ProofOfWork".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::HashPublicInputs::TYPE_TAG => {
            "HashPublicInputs".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::fri_commit::FriCommit::TYPE_TAG => {
            "FriCommit".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::table_commit::TableCommit::TYPE_TAG => {
            "TableCommit".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::proof_of_work::UpdateTranscriptU64::TYPE_TAG => {
            "UpdateTranscriptU64".to_string()
        },
        // TYPE_TAG from stark crate
        stark::pedersen::LookupAndAccumulate::TYPE_TAG => {
            "LookupAndAccumulate".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::add::Add::TYPE_TAG => {
            "Add".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::exp::Exp::TYPE_TAG => {
            "Exp".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::eval_oods_polynomial_inner::EvalOodsPolynomialInner::TYPE_TAG => {
            "EvalOodsPolynomialInner".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::fib::FibonacciCombiner::TYPE_TAG => {
            "FibonacciCombiner".to_string()
        },
        // TYPE_TAG from stark crate
        stark::pedersen::PedersenHash::TYPE_TAG => {
            "PedersenHash".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::vector_decommit::VectorDecommit::TYPE_TAG => {
            "VectorDecommit".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::helpers::PowersArray::TYPE_TAG => {
            "PowersArray".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::verify_oods::VerifyOods::TYPE_TAG => {
            "VerifyOods".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::exp::ExpInternal::TYPE_TAG => {
            "ExpInternal".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::validate_public_input::ValidatePublicInput::TYPE_TAG => {
            "ValidatePublicInput".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::VerifyPublicInput::TYPE_TAG => {
            "VerifyPublicInput".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::eval_oods_boundary_poly_at_points::EvalOodsBoundaryPolyAtPoints::TYPE_TAG => {
            "EvalOodsBoundaryPolyAtPoints".to_string()
        },
        // TYPE_TAG from stark crate
        stark::poseidon::PoseidonHashMany::TYPE_TAG => {
            "PoseidonHashMany".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::hash_computation::HashComputationWithQueries::TYPE_TAG => {
            "HashComputationWithQueries".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::fri_verify::FriVerify::TYPE_TAG => {
            "FriVerify".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_verify::hash_computation::HashComputation::TYPE_TAG => {
            "HashComputation".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::StarkCommit::TYPE_TAG => {
            "StarkCommit".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::proof_of_work::ComputeHash::TYPE_TAG => {
            "ComputeHash".to_string()
        },
        // TYPE_TAG from stark crate
        stark::stark_proof::stark_commit::traces_commit::GenerateInteractionElements::TYPE_TAG => {
            "GenerateInteractionElements".to_string()
        },
        // TYPE_TAG from stark crate
        stark::poseidon::hades::HadesPermutation::TYPE_TAG => {
            "HadesPermutation".to_string()
        },
        // TYPE_TAG from stark crate
        stark::swiftness::transcript::TranscriptReadFelt::TYPE_TAG => {
            "TranscriptReadFelt".to_string()
        },
        // TYPE_TAG from arithmetic crate
        arithmetic::mul::Mul::TYPE_TAG => {
            "Mul".to_string()
        },

        _ => {
            panic!("Unknown type tag: {type_tag}");
        }
    }
}
