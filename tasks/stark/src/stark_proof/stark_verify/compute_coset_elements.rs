use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

use super::group::get_fri_group;
use crate::funvec::FunVec;
use crate::swiftness::stark::types::FriVerifyData;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ComputeCosetElements {
    stage: ComputeCosetElementsStep,
    current_index: usize,
    coset_x_inv: Felt,
    coset_start_index: Felt,
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
            coset_x_inv: Felt::ZERO,
            coset_start_index: Felt::ZERO,
        }
    }

    pub fn with_coset_start_index(coset_start_index: Felt) -> Self {
        Self {
            stage: ComputeCosetElementsStep::Init,
            current_index: 0,
            coset_x_inv: Felt::ZERO,
            coset_start_index,
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
                self.current_index = 0;
                self.coset_x_inv = Felt::ZERO;
                fri_verify_data.coset_elements.flush();

                self.stage = ComputeCosetElementsStep::ProcessElement;
                vec![]
            }
            ComputeCosetElementsStep::ProcessElement => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                let coset_size_usize: usize =
                    fri_verify_data.coset_size.to_biguint().try_into().unwrap();

                if self.current_index < coset_size_usize {
                    // Compute target index for this coset position (like original: coset_start_index + index)
                    let target_index =
                        self.coset_start_index + Felt::from(self.current_index as u64);

                    // Check if first query matches target_index (like original: q.first() && q.unwrap().index == target_index)
                    let q = fri_verify_data.working_queries.get(0);
                    if q.is_some() && q.unwrap().index == target_index {
                        // Found matching query - consume it (simulate queries.drain(0..1))
                        let query = *q.unwrap();

                        // Remove first query (simulate drain(0..1))
                        let mut temp_queries = FunVec::default();
                        for i in 1..fri_verify_data.working_queries.len() {
                            if let Some(remaining_query) = fri_verify_data.working_queries.get(i) {
                                temp_queries.push(*remaining_query);
                            }
                        }
                        fri_verify_data.working_queries = temp_queries;

                        // Add query y_value to coset elements (like original: coset_elements.push(query[0].y_value))
                        fri_verify_data.coset_elements.push(query.y_value);

                        // Calculate coset_x_inv using FRI group (like original: query[0].x_inv_value * fri_group.get(index).unwrap())
                        let fri_group = get_fri_group();
                        let fri_group_element = *fri_group.get(self.current_index).unwrap();
                        self.coset_x_inv = query.x_inv_value * fri_group_element;
                    } else {
                        // Use sibling witness from global sibling_witness (like original else clause: sibling_witness.drain(0..1))
                        if !fri_verify_data.sibling_witness.is_empty() {
                            let witness_value = *fri_verify_data.sibling_witness.get(0).unwrap();

                            // Remove first witness from global sibling_witness (simulate drain(0..1))
                            let mut temp_witness = FunVec::default();
                            for i in 1..fri_verify_data.sibling_witness.len() {
                                if let Some(element) = fri_verify_data.sibling_witness.get(i) {
                                    temp_witness.push(*element);
                                }
                            }
                            fri_verify_data.sibling_witness = temp_witness;

                            // Add witness to coset elements (like original: coset_elements.push(withness[0]))
                            fri_verify_data.coset_elements.push(witness_value);
                        } else {
                            panic!("Insufficient sibling witness data in test fixtures at index {} (need 112, have {})", 
                                    self.current_index, fri_verify_data.sibling_witness.len());
                        }
                    }

                    self.current_index += 1;
                    vec![]
                } else {
                    // All elements processed - copy results to working data (like original return (coset_elements, coset_x_inv))
                    let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();

                    println!(
                        "DEBUG: ComputeCosetElements Done - coset_elements.len(): {}",
                        fri_verify_data.coset_elements.len()
                    );

                    // Add coset_elements to working_y_values (like original: verify_y_values.extend(coset_elements.iter()))
                    for i in 0..fri_verify_data.coset_elements.len() {
                        if let Some(element) = fri_verify_data.coset_elements.get(i) {
                            fri_verify_data.working_y_values.push(*element);
                        }
                    }

                    // Also copy to working_elements for FriFormula
                    fri_verify_data.working_elements.flush();
                    for i in 0..fri_verify_data.coset_elements.len() {
                        if let Some(element) = fri_verify_data.coset_elements.get(i) {
                            fri_verify_data.working_elements.push(*element);
                        }
                    }

                    fri_verify_data.coset_x_inv = self.coset_x_inv;

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
    // Get the computed results from FriVerifyData (like original return values)
    pub fn get_results_from_data(fri_verify_data: &FriVerifyData) -> Vec<Felt> {
        let mut coset_elements = Vec::new();
        for i in 0..fri_verify_data.working_y_values.len() {
            if let Some(element) = fri_verify_data.working_y_values.get(i) {
                coset_elements.push(*element);
            }
        }
        coset_elements
    }

    // Get the coset_x_inv value
    pub fn get_coset_x_inv(&self) -> Felt {
        self.coset_x_inv
    }
}
