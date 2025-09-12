use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use crate::swiftness::stark::types::{cast_slice_to_struct_mut, FriVerifyData};

// Task for computing coset elements
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeCosetElements {
    stage: ComputeCosetElementsStep,
    current_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum ComputeCosetElementsStep {
    Init,
    ProcessElement,
    Done,
}

impl_type_identifiable!(ComputeCosetElements);

impl ComputeCosetElements {
    pub fn new() -> Self {
        Self {
            stage: ComputeCosetElementsStep::Init,
            current_index: 0,
        }
    }
}

impl Default for ComputeCosetElements {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for ComputeCosetElements {
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.stage {
            ComputeCosetElementsStep::Init => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                // Initialize computation
                self.current_index = 0;
                fri_verify_data.working_elements.flush();

                self.stage = ComputeCosetElementsStep::ProcessElement;

                // Dane są już zmodyfikowane bezpośrednio na stosie!
                vec![]
            }
            ComputeCosetElementsStep::ProcessElement => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                let coset_size_usize: usize =
                    fri_verify_data.coset_size.to_biguint().try_into().unwrap();

                if self.current_index < coset_size_usize {
                    // Compute coset start index based on current query
                    let coset_start_index =
                        if let Some(first_query) = fri_verify_data.working_queries.get(0) {
                            let query_uint = first_query.index.to_biguint();
                            let coset_index = query_uint / fri_verify_data.coset_size.to_biguint();
                            let coset_index_felt =
                                Felt::from_bytes_be_slice(coset_index.to_bytes_be().as_slice());
                            coset_index_felt * fri_verify_data.coset_size
                        } else {
                            Felt::ZERO
                        };

                    let target_index = coset_start_index + Felt::from(self.current_index as u64);

                    // Check if we have a query for this index
                    let mut found_query = false;
                    let mut query_y_value = Felt::ZERO;
                    let mut coset_x_inv = Felt::ZERO;

                    // Search for matching query
                    for i in 0..fri_verify_data.working_queries.len() {
                        if let Some(query) = fri_verify_data.working_queries.get(i) {
                            if query.index == target_index {
                                query_y_value = query.y_value;
                                // Calculate coset_x_inv using FRI group (hardcoded for now)
                                let fri_group_element = match self.current_index {
                                    0 => Felt::ONE,
                                    1 => Felt::from_hex_unchecked("0x446ed3ce295dda2b5ea677394813e6eab8bfbc55397aacac8e6df6f4bc9ca34"), // OMEGA_8
                                    _ => Felt::ONE, // Simplified
                                };
                                coset_x_inv = query.x_inv_value * fri_group_element;
                                found_query = true;
                                break;
                            }
                        }
                    }

                    if found_query {
                        fri_verify_data.working_elements.push(query_y_value);
                    } else {
                        // Use sibling witness from working_elements (if available)
                        if let Some(witness_value) =
                            fri_verify_data.working_elements.get(self.current_index)
                        {
                            // Value already in working_elements from sibling witness
                        } else {
                            fri_verify_data.working_elements.push(Felt::ZERO); // Fallback
                        }
                    }

                    self.current_index += 1;
                    vec![]
                } else {
                    // All elements processed
                    self.stage = ComputeCosetElementsStep::Done;

                    vec![]
                }
            }
            ComputeCosetElementsStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.stage == ComputeCosetElementsStep::Done
    }
}

impl ComputeCosetElements {
    // Get the computed results from FriVerifyData
    pub fn get_results_from_data(fri_verify_data: &FriVerifyData) -> Vec<Felt> {
        let mut coset_elements = Vec::new();
        for i in 0..fri_verify_data.working_elements.len() {
            if let Some(element) = fri_verify_data.working_elements.get(i) {
                coset_elements.push(*element);
            }
        }
        coset_elements
    }
}
