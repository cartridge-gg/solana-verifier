use stark_commit::stark_commit::StarkCommit;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use get_hash::hash::GetHash;
use types::swiftness::air::domains::StarkDomains;
use validate_public_input::validate::ValidatePublicInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum VerifyStep {
    ValidatePublicInput,
    GetHash,
    StarkCommit,
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
                    stack.get_proof_reference();
                let n_verifier_friendly_commitment_layers =
                    proof.config.n_verifier_friendly_commitment_layers;
                assert!(
                    stack.is_empty_front(),
                    "Stack back should be empty before GetHash"
                );
                self.step = VerifyStep::StarkCommit;
                vec![GetHash::new(n_verifier_friendly_commitment_layers).to_vec_with_type_tag()]
            }
            VerifyStep::StarkCommit => {
                let result = stack.borrow_front().to_owned();
                stack.pop_front();

                assert!(
                    stack.is_empty_front(),
                    "Stack back should be empty after GetHash"
                );

                let proof =
                    stack.get_proof_reference();

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

                self.step = VerifyStep::Done;
                vec![StarkCommit::new().to_vec_with_type_tag()]
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
