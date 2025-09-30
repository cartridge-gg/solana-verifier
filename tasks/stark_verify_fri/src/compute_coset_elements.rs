use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, CacheStorage, Executable, ProofData, TypeIdentifiable};

use super::group::get_fri_group;
use types::funvec::FunVec;
use types::swiftness::stark::types::FriVerifyData;

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
    fn execute<T: BidirectionalStack + ProofData + CacheStorage>(
        &mut self, 
        stack: &mut T
    ) -> Vec<Vec<u8>> {
        match self.stage {
            ComputeCosetElementsStep::Init => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();
                self.current_index = 0;
                self.coset_x_inv = Felt::ZERO;
                fri_verify_data.working_elements.flush();
        
                self.stage = ComputeCosetElementsStep::ProcessElement;
                vec![]
            }
            ComputeCosetElementsStep::ProcessElement => {
                let fri_verify_data = stack.borrow_from_cache_mut::<FriVerifyData>();
                let coset_size_usize: usize = fri_verify_data.coset_size.to_biguint().try_into().unwrap();

                if self.current_index < coset_size_usize {
                    let target_index = self.coset_start_index + Felt::from(self.current_index as u64);

                    // ZMIANA: używamy get_first_active_query()
                    let q = fri_verify_data.get_first_active_query();
                    if q.is_some() && q.unwrap().index == target_index {
                        let query = *q.unwrap();
                        fri_verify_data.remove_first_active_query();

                        fri_verify_data.working_elements.push(query.y_value);
                        fri_verify_data.working_y_values.push(query.y_value);

                        let fri_group = get_fri_group();
                        let fri_group_element = *fri_group.get(self.current_index).unwrap();
                        self.coset_x_inv = query.x_inv_value * fri_group_element;
                    } else {
                        if !fri_verify_data.sibling_witness.is_empty() {
                            let witness_value = *fri_verify_data.sibling_witness.get(0).unwrap();
                            fri_verify_data.sibling_witness.shift(1);
                            fri_verify_data.working_elements.push(witness_value);
                            fri_verify_data.working_y_values.push(witness_value);
                        }
                    }

                    self.current_index += 1;
                    vec![]
                } else {
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