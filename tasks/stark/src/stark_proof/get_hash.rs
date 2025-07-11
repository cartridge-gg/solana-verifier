use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::{
    felt::Felt, pedersen::PedersenHash, poseidon::PoseidonHashMany, swiftness::stark::types::{cast_slice_to_struct, StarkProof}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetHashStep {
    Init,
    PedersenHashingAddress,
    WaitForPedersenAddress,
    PedersenHashingValue,
    WaitForPedersenValue,
    MainPageHash,
    Program,
    Done,
}

#[repr(C)]
pub struct GetHash {
    step: GetHashStep,
    main_page_hash: Felt,
    hash_data: Vec<Felt>,
    main_page_len: usize,
    current_memory_index: usize,
    n_verifier_friendly_commitment_layers: Felt,
    accumulated_hash: Felt,
}

impl_type_identifiable!(GetHash);

impl GetHash {
    pub fn new(n_verifier_friendly_commitment_layers: Felt) -> Self {
        Self {
            step: GetHashStep::Init,
            main_page_hash: Felt::ZERO,
            hash_data: Vec::new(),
            main_page_len: 0,
            current_memory_index: 0,
            n_verifier_friendly_commitment_layers,
            accumulated_hash: Felt::ZERO,
        }
    }
}

impl Default for GetHash {
    fn default() -> Self {
        Self::new(Felt::ZERO)
    }
}

impl Executable for GetHash {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GetHashStep::Init => {
                let proof_reference: &mut [u8] = stack.get_proof_reference();
                let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                self.main_page_len = proof.public_input.main_page.0.len();
                self.current_memory_index = 0;
                self.accumulated_hash = Felt::ZERO;

                if self.main_page_len == 0 {
                    // If no main page data, go directly to final pedersen hash
                    self.step = GetHashStep::MainPageHash;
                    return self.execute_final_pedersen_hash(stack);
                }

                self.step = GetHashStep::PedersenHashingAddress;
                self.execute_pedersen_for_current_address(stack)
            }
            GetHashStep::PedersenHashingAddress => {
                self.execute_pedersen_for_current_address(stack)
            }
            GetHashStep::WaitForPedersenAddress => {
                // Get the result from the address hash
                let bytes = stack.borrow_front();
                let pedersen_result = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                self.accumulated_hash = pedersen_result;
                
                // Now hash with the value
                self.step = GetHashStep::PedersenHashingValue;
                self.execute_pedersen_for_current_value(stack)
            }
            GetHashStep::PedersenHashingValue => {
                self.execute_pedersen_for_current_value(stack)
            }
            GetHashStep::WaitForPedersenValue => {
                // Get the result from the value hash
                let bytes = stack.borrow_front();
                let pedersen_result = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                self.accumulated_hash = pedersen_result;
                self.current_memory_index += 1;

                if self.current_memory_index < self.main_page_len {
                    // Continue with next memory entry (start with address)
                    self.step = GetHashStep::PedersenHashingAddress;
                    self.execute_pedersen_for_current_address(stack)
                } else {
                    // All memory entries processed, do final hash with length
                    self.step = GetHashStep::MainPageHash;
                    self.execute_final_pedersen_hash(stack)
                }
            }
            GetHashStep::MainPageHash => {
                // Get the final main page hash result
                let bytes = stack.borrow_front();
                self.main_page_hash = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                // Now prepare the final hash data
                let proof_reference: &mut [u8] = stack.get_proof_reference();
                let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                let public_input = &proof.public_input;

                // Build hash_data vector for final hashing
                let mut hash_data = vec![
                    self.n_verifier_friendly_commitment_layers,
                    public_input.log_n_steps,
                    public_input.range_check_min,
                    public_input.range_check_max,
                    public_input.layout,
                ];

                // Add dynamic params if they exist
                if let Some(dynamic_params) = &public_input.dynamic_params {
                    let dynamic_params_vec: Vec<u32> = (*dynamic_params).into();
                    hash_data.extend(dynamic_params_vec.into_iter().map(Felt::from));
                }

                // Add segments
                hash_data.extend(
                    public_input
                        .segments
                        .iter()
                        .flat_map(|s| vec![s.begin_addr, s.stop_ptr]),
                );

                hash_data.push(public_input.padding_addr);
                hash_data.push(public_input.padding_value);
                hash_data.push(Felt::from(public_input.continuous_page_headers.len() + 1));

                // Add main page info
                hash_data.push(Felt::from(public_input.main_page.0.len()));
                hash_data.push(self.main_page_hash);

                // Add continuous page headers
                hash_data.extend(
                    public_input
                        .continuous_page_headers
                        .iter()
                        .flat_map(|h| vec![h.start_address, h.size, h.hash]),
                );
                
                self.hash_data = hash_data;

                // Use PoseidonHashMany::push_input to properly prepare the stack
                PoseidonHashMany::push_input(&self.hash_data, stack);

                self.step = GetHashStep::Program;
                vec![PoseidonHashMany::new(self.hash_data.len()).to_vec_with_type_tag()]
            }
            GetHashStep::Program => {
                let bytes = stack.borrow_front();
                let poseidon_result = Felt::from_bytes_be_slice(bytes);
                println!("GetHash result: {:?}", poseidon_result);
                
                self.step = GetHashStep::Done;
                vec![]
            }
            GetHashStep::Done => {
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == GetHashStep::Done
    }
}

impl GetHash {
    #[inline(always)]
    fn execute_pedersen_for_current_address<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let proof_reference: &mut [u8] = stack.get_proof_reference();
        let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
        let memory = proof.public_input.main_page.0.as_slice();
        
        let address = memory[self.current_memory_index].address;
        
        PedersenHash::push_input(self.accumulated_hash, address, stack);
        
        self.step = GetHashStep::WaitForPedersenAddress;
        vec![PedersenHash::new().to_vec_with_type_tag()]
    }
    #[inline(always)]
    fn execute_pedersen_for_current_value<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        let proof_reference: &mut [u8] = stack.get_proof_reference();
        let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
        let memory = proof.public_input.main_page.0.as_slice();
        
        let value = memory[self.current_memory_index].value;

        PedersenHash::push_input(self.accumulated_hash, value, stack);
        
        self.step = GetHashStep::WaitForPedersenValue;
        vec![PedersenHash::new().to_vec_with_type_tag()]
    }
    #[inline(always)]
    fn execute_final_pedersen_hash<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        // Final hash with the length multiplier
        let length_multiplier = Felt::TWO * Felt::from(self.main_page_len);
        
        PedersenHash::push_input(self.accumulated_hash,length_multiplier, stack);
        
        vec![PedersenHash::new().to_vec_with_type_tag()]
    }
}