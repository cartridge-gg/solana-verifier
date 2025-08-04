use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack};
use utils::{Executable, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HadesPhase {
    FirstHalfFullRounds,
    SecondHalfFullRounds,
    Finished,
}

#[repr(C)]
pub struct HadesPermutation {
    state: [Felt; 3],
    phase: HadesPhase,
    constants_index: usize,
}

impl_type_identifiable!(HadesPermutation);

impl HadesPermutation {
    pub fn new(state: [Felt; 3]) -> Self {
        Self {
            state,
            phase: HadesPhase::FirstHalfFullRounds,
            constants_index: 0,
        }
    }

    /// Redefined mix function for optimization purposes
    #[inline(always)]
    fn mix(state: &mut [Felt]) {
        let t = state[0] + state[1] + state[2];
        state[0] = t + state[0].double();
        state[1] = t - state[1].double();
        state[2] = t - (state[2] + state[2] + state[2]);
    }

    #[inline(always)]
    fn full_round(&mut self) {
        // Perform full round
        for (i, value) in self.state.iter_mut().enumerate() {
            *value += Self::ROUND_CONSTANTS[self.constants_index + i];
            *value = (*value).square() * *value;
        }

        // Mix step is common for both round types
        Self::mix(&mut self.state);
    }

    #[inline(always)]
    fn partial_round(&mut self) {
        // Perform partial round
        self.state[2] += Self::ROUND_CONSTANTS[self.constants_index];
        self.state[2] = self.state[2].square() * self.state[2];

        // Mix step
        Self::mix(&mut self.state);
    }
}

impl Executable for HadesPermutation {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.phase {
            HadesPhase::FirstHalfFullRounds => {
                // First half of full rounds
                for _ in 0..Self::N_FULL_ROUNDS / 2 {
                    self.full_round();

                    self.constants_index += Self::N_ROUND_CONSTANTS_COLS;
                }
                // Partial rounds
                for _ in 0..Self::N_PARTIAL_ROUNDS / 2 {
                    self.partial_round();

                    self.constants_index += 1;
                }
                self.phase = HadesPhase::SecondHalfFullRounds;
            }
            HadesPhase::SecondHalfFullRounds => {
                // Partial rounds
                for _ in 0..(Self::N_PARTIAL_ROUNDS - Self::N_PARTIAL_ROUNDS / 2) {
                    self.partial_round();

                    self.constants_index += 1;
                }
                // Second half of full rounds
                for _ in 0..Self::N_FULL_ROUNDS / 2 {
                    self.full_round();

                    self.constants_index += Self::N_ROUND_CONSTANTS_COLS;
                }
                self.state.iter().rev().for_each(|value| {
                    stack.push_front(&value.to_bytes_be()).unwrap();
                });
                self.phase = HadesPhase::Finished;
            }
            HadesPhase::Finished => {}
        }

        vec![]
    }

    fn is_finished(&mut self) -> bool {
        self.phase == HadesPhase::Finished
    }
}
