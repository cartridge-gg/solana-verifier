use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::{
    felt::Felt,
    poseidon::PoseidonHashMany,
    swiftness::stark::types::{cast_slice_to_struct, StarkProof},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetHashStep {
    Init,
    HashData,
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
    n_verifier_friendly_commitment_layers: Felt,
}

impl_type_identifiable!(GetHash);

impl GetHash {
    pub fn new(n_verifier_friendly_commitment_layers: Felt) -> Self {
        Self {
            step: GetHashStep::Init,
            main_page_hash: Felt::ZERO,
            hash_data: Vec::new(),
            main_page_len: 0,
            n_verifier_friendly_commitment_layers,
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

                self.step = GetHashStep::HashData;
                vec![]
            }
            GetHashStep::HashData => {
                // Prepare main page data for hashing
                let mut main_page_data = Vec::new();

                main_page_data.push(Felt::ZERO);

                for i in 0..self.main_page_len {
                    let proof_reference: &mut [u8] = stack.get_proof_reference();
                    let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                    let memory = proof.public_input.main_page.0.as_slice();
                    let address = memory[i].address;
                    let value = memory[i].value;
                    main_page_data.push(address);
                    main_page_data.push(value);
                }

                // Add the length information
                main_page_data.push(Felt::TWO * Felt::from(self.main_page_len));

                // Use PoseidonHashMany::push_input to properly prepare the stack
                PoseidonHashMany::push_input(&main_page_data, stack);

                self.step = GetHashStep::MainPageHash;
                vec![PoseidonHashMany::new(main_page_data.len()).to_vec_with_type_tag()]
            }
            GetHashStep::MainPageHash => {
                // Get the main page hash from the stack
                let bytes = stack.borrow_front();
                self.main_page_hash = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

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
                // let bytes = stack.borrow_front();
                // let get_hash_result = Felt::from_bytes_be_slice(bytes);
                // println!("get_hash_result: {:?}", get_hash_result);

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
