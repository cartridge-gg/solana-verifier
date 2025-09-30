use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, CacheStorage, Executable, ProofData, TypeIdentifiable};

use types::swiftness::stark::types::FriVerifyData;

// Constants representing primitive roots of unity for orders 2, 4, 8, and 16.
// These are calculated based on the formula 1 / 3^((PRIME - 1) / 16) where 3 is a generator.
const OMEGA_16: Felt =
    Felt::from_hex_unchecked("0x5c3ed0c6f6ac6dd647c9ba3e4721c1eb14011ea3d174c52d7981c5b8145aa75");
const OMEGA_8: Felt =
    Felt::from_hex_unchecked("0x446ed3ce295dda2b5ea677394813e6eab8bfbc55397aacac8e6df6f4bc9ca34");
const OMEGA_4: Felt =
    Felt::from_hex_unchecked("0x1dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc81e");

// Task for computing FRI formula
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FriFormula {
    stage: FriFormulaStep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FriFormulaStep {
    Compute,
    Done,
}

impl_type_identifiable!(FriFormula);

impl FriFormula {
    pub fn new() -> Self {
        Self {
            stage: FriFormulaStep::Compute,
        }
    }
}

impl Default for FriFormula {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for FriFormula {
    fn execute<T: BidirectionalStack + ProofData + CacheStorage>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            FriFormulaStep::Compute => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                let mut values = Vec::new();
                for i in 0..fri_verify_data.working_elements.len() {
                    if let Some(value) = fri_verify_data.working_elements.get(i) {
                        values.push(*value);
                    }
                }

                // Compute FRI formula based on coset size
                let coset_size_u64: u64 =
                    fri_verify_data.coset_size.to_biguint().try_into().unwrap();

                let result = match coset_size_u64 {
                    2 => {
                        if values.len() < 2 {
                            panic!(
                                "Invalid values length for coset size 2: expected 2, got {}",
                                values.len()
                            );
                        }
                        self.fri_formula2(
                            values[0],
                            values[1],
                            fri_verify_data.eval_point,
                            fri_verify_data.coset_x_inv,
                        )
                    }
                    4 => {
                        if values.len() < 4 {
                            panic!(
                                "Invalid values length for coset size 4: expected 4, got {}",
                                values.len()
                            );
                        }
                        self.fri_formula4(
                            &values[0..4],
                            fri_verify_data.eval_point,
                            fri_verify_data.coset_x_inv,
                        )
                    }
                    8 => {
                        if values.len() < 8 {
                            panic!(
                                "Invalid values length for coset size 8: expected 8, got {}",
                                values.len()
                            );
                        }
                        self.fri_formula8(
                            &values[0..8],
                            fri_verify_data.eval_point,
                            fri_verify_data.coset_x_inv,
                        )
                    }
                    16 => {
                        if values.len() < 16 {
                            panic!(
                                "Invalid values length for coset size 16: expected 16, got {}",
                                values.len()
                            );
                        }
                        self.fri_formula16(
                            &values[0..16],
                            fri_verify_data.eval_point,
                            fri_verify_data.coset_x_inv,
                        )
                    }
                    _ => {
                        panic!("Invalid coset size: {}", coset_size_u64);
                    }
                };

                if let Some(coset_index) = fri_verify_data.working_indices.get(fri_verify_data.current_coset_index) {
                    let next_query = types::swiftness::fri::types::FriLayerQuery {
                        index: *coset_index,
                        y_value: result,
                        x_inv_value: fri_verify_data.next_x_inv_value,
                    };
                    
                    fri_verify_data.push_next_query(next_query);
                    fri_verify_data.current_coset_index += 1;
                }
                self.stage = FriFormulaStep::Done;
                vec![]
            }
            FriFormulaStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == FriFormulaStep::Done
    }
}

impl FriFormula {
    // Function to fold 2 elements into one using one layer of FRI
    fn fri_formula2(&self, f_x: Felt, f_minus_x: Felt, eval_point: Felt, x_inv: Felt) -> Felt {
        f_x + f_minus_x + eval_point * x_inv * (f_x - f_minus_x)
    }

    // Function to fold 4 elements into one using 2 layers of FRI
    fn fri_formula4(&self, values: &[Felt], eval_point: Felt, x_inv: Felt) -> Felt {
        // Applying the first layer of folding
        let g0 = self.fri_formula2(values[0], values[1], eval_point, x_inv);
        let g1 = self.fri_formula2(values[2], values[3], eval_point, x_inv * OMEGA_4);

        // Last layer, combining the results of the first layer
        self.fri_formula2(g0, g1, eval_point * eval_point, x_inv * x_inv)
    }

    // Function to fold 8 elements into one using 3 layers of FRI
    fn fri_formula8(&self, values: &[Felt], eval_point: Felt, x_inv: Felt) -> Felt {
        // Applying the first layer of folding
        let g0 = self.fri_formula4(&values[0..4], eval_point, x_inv);
        let g1 = self.fri_formula4(&values[4..8], eval_point, x_inv * OMEGA_8);

        // Preparing variables for the last layer
        let eval_point2 = eval_point * eval_point;
        let eval_point4 = eval_point2 * eval_point2;
        let x_inv2 = x_inv * x_inv;
        let x_inv4 = x_inv2 * x_inv2;

        // Last layer, combining the results of the second layer
        self.fri_formula2(g0, g1, eval_point4, x_inv4)
    }

    // Function to fold 16 elements into one using 4 layers of FRI
    fn fri_formula16(&self, values: &[Felt], eval_point: Felt, x_inv: Felt) -> Felt {
        // Applying the first layer of folding
        let g0 = self.fri_formula8(&values[0..8], eval_point, x_inv);
        let g1 = self.fri_formula8(&values[8..16], eval_point, x_inv * OMEGA_16);

        // Preparing variables for the last layer
        let eval_point2 = eval_point * eval_point;
        let eval_point4 = eval_point2 * eval_point2;
        let eval_point8 = eval_point4 * eval_point4;
        let x_inv2 = x_inv * x_inv;
        let x_inv4 = x_inv2 * x_inv2;
        let x_inv8 = x_inv4 * x_inv4;

        // Last layer, combining the results of the second layer
        self.fri_formula2(g0, g1, eval_point8, x_inv8)
    }

    // // Get the computed result (for testing or direct access)
    // pub fn get_result(&self) -> Felt {
    //     self.result
    // }

    // Parse result from stack data
    pub fn from_stack<T: BidirectionalStack + ProofData>(stack: &mut T) -> Felt {
        let result = Felt::from_bytes_be_slice(stack.borrow_front());
        stack.pop_front();
        result
    }
}
