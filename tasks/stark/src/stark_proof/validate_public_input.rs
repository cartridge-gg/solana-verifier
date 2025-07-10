use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatePublicInputStep {
    Init,
    Output,
    Program,
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
            step: ValidatePublicInputStep::Init,
        }
    }
}

impl Default for ValidatePublicInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ValidatePublicInput {
    fn execute<T: BidirectionalStack>(&mut self, _stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            ValidatePublicInputStep::Init => {
                self.step = ValidatePublicInputStep::Output;
                vec![]
            }
            ValidatePublicInputStep::Output => {
                self.step = ValidatePublicInputStep::Program;
                vec![]
            }
            ValidatePublicInputStep::Program => {
                self.step = ValidatePublicInputStep::Done;

                vec![]
            }
            ValidatePublicInputStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == ValidatePublicInputStep::Done
    }
}
