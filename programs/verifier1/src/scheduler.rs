use crate::state::BidirectionalStackAccount;
use utils::{BidirectionalStack, Executable, Scheduler};

// Include the generated dispatch code
include!(concat!(
    env!("OUT_DIR"),
    "/verifier-1_executable_dispatch.rs"
));

impl Scheduler for BidirectionalStackAccount {}

impl BidirectionalStackAccount {
    pub fn execute(&mut self) {
        solana_program::msg!("Before execute() call");
        let (tasks, is_finished) = execute(self);
        solana_program::msg!("After execute() call");

        println!("tasks: {:?}", tasks);

        if is_finished {
            self.pop_back();
        }

        for task in tasks.iter().rev() {
            let _ = self.push_back(task);
        }
    }
}
