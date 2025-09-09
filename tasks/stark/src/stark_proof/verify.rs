use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::stark_proof::get_hash::GetHash;
use crate::stark_proof::stark_commit::StarkCommit;
use crate::stark_proof::stark_verify::StarkVerify;
use crate::stark_proof::validate_public_input::ValidatePublicInput;
use crate::stark_proof::VerifyPublicInput;
use crate::swiftness::air::domains::StarkDomains;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyStep {
    ValidatePublicInput,
    GetHash,
    StarkCommit,
    StarkVerify,
    VerifyPublicInput,
    Done,
}

#[repr(C)]
pub struct Verify {
    step: VerifyStep,
}

impl_type_identifiable!(Verify);

impl Verify {
    pub fn new() -> Self {
        Self {
            step: VerifyStep::ValidatePublicInput,
        }
    }
}

impl Default for Verify {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for Verify {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            VerifyStep::ValidatePublicInput => {
                self.step = VerifyStep::GetHash;
                vec![ValidatePublicInput::new().to_vec_with_type_tag()]
            }
            VerifyStep::GetHash => {
                let proof =
                    stack.get_proof_reference::<crate::swiftness::stark::types::StarkProof>();
                let n_verifier_friendly_commitment_layers =
                    proof.config.n_verifier_friendly_commitment_layers;
                assert_eq!(
                    stack.is_empty_front(),
                    true,
                    "Stack back should be empty before GetHash"
                );
                self.step = VerifyStep::StarkCommit;
                vec![GetHash::new(n_verifier_friendly_commitment_layers).to_vec_with_type_tag()]
            }
            VerifyStep::StarkCommit => {
                let result = stack.borrow_front().to_owned();
                stack.pop_front();

                assert_eq!(
                    stack.is_empty_front(),
                    true,
                    "Stack back should be empty after GetHash"
                );

                let proof =
                    stack.get_proof_reference::<crate::swiftness::stark::types::StarkProof>();

                let stark_domain = StarkDomains::new(
                    proof.config.log_trace_domain_size,
                    proof.config.log_n_cosets,
                );

                stack
                    .push_front(stark_domain.trace_generator.to_bytes_be().as_slice())
                    .unwrap();

                stack
                    .push_front(stark_domain.trace_domain_size.to_bytes_be().as_slice())
                    .unwrap();

                stack.push_front(&result).unwrap();

                self.step = VerifyStep::StarkVerify;
                vec![StarkCommit::new().to_vec_with_type_tag()]
            }
            VerifyStep::StarkVerify => {
                self.step = VerifyStep::VerifyPublicInput;
                vec![StarkVerify::new(0, 0).to_vec_with_type_tag()]
            }
            VerifyStep::VerifyPublicInput => {
                assert_eq!(
                    stack.is_empty_front(),
                    true,
                    "Stack should be empty before verifying public input"
                );
                self.step = VerifyStep::Done;
                vec![VerifyPublicInput::new().to_vec_with_type_tag()]
            }
            VerifyStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == VerifyStep::Done
    }
}
