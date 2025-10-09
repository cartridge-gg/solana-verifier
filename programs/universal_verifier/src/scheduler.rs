// Include the minimal dispatch code from build.rs
// On Solana: this is empty stub that panics
// In tests: we need actual dispatch, so we'll override it
include!(concat!(
    env!("OUT_DIR"),
    "/universal-verifier_executable_dispatch.rs"
));

pub trait UniversalStackExecute {
    fn execute(&mut self);
}
// For tests: we need to actually execute tasks since CPI doesn't work locally
#[cfg(not(target_os = "solana"))]
impl UniversalStackExecute for utils::UniversalStackAccount {
    fn execute(&mut self) {
        use utils::BidirectionalStack;

        // In tests, we execute based on current mode
        // Each mode corresponds to a different verifier's task set
        match self.mode() {
            utils::VerifierMode::Verifier1 => {
                let (tasks, is_finished) = verifier_1::scheduler::execute(self);
                if is_finished {
                    self.pop_back();
                }
                for task in tasks.iter().rev() {
                    let _ = self.push_back(task);
                }
            }
            utils::VerifierMode::Verifier2 => {
                let (tasks, is_finished) = verifier_2::scheduler::execute(self);
                if is_finished {
                    self.pop_back();
                }
                for task in tasks.iter().rev() {
                    let _ = self.push_back(task);
                }
            }
            utils::VerifierMode::Verifier3 => {
                let (tasks, is_finished) = verifier_3::scheduler::execute(self);
                if is_finished {
                    self.pop_back();
                }
                for task in tasks.iter().rev() {
                    let _ = self.push_back(task);
                }
            }
            utils::VerifierMode::Verifier4 => {
                let (tasks, is_finished) = verifier_4::scheduler::execute(self);
                if is_finished {
                    self.pop_back();
                }
                for task in tasks.iter().rev() {
                    let _ = self.push_back(task);
                }
            }
        }
    }
}
// On Solana: this should never be called directly
#[cfg(target_os = "solana")]
impl UniversalStackExecute for utils::UniversalStackAccount {
    fn execute(&mut self) {
        // On-chain, this should never be called - use CPI instead
        panic!("Universal verifier should not execute tasks directly on-chain");
    }
}
