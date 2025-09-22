use utils_1::TypeIdentifiable;
use utils_1::{impl_type_identifiable, Executable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatePublicInputStep {
    Validate,
    Done,
}

#[repr(C)]
pub struct ValidatePublicInput {
    step: ValidatePublicInputStep,
}
impl_type_identifiable!(ValidatePublicInput);

impl ValidatePublicInput {
    pub fn new() -> Self {
        Self {
            step: ValidatePublicInputStep::Validate,
        }
    }
}

impl Default for ValidatePublicInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ValidatePublicInput {
    fn execute<T: utils_1::BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            ValidatePublicInputStep::Validate => {
                // Here you would add the actual validation logic
                // For demonstration, we just print a message
                println!("Validating public input...");

                // After validation, move to the Done step
                self.step = ValidatePublicInputStep::Done;
            }
            ValidatePublicInputStep::Done => {
                // No further action needed
                println!("Validation already completed.");
            }
        }
        vec![]
    }
    fn is_finished(&mut self) -> bool {
        self.step == ValidatePublicInputStep::Done
    }
}
