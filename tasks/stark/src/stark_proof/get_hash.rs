use utils::{impl_type_identifiable, BidirectionalStack, Executable, TypeIdentifiable};

use crate::swiftness::air::domains::StarkDomains;
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
}

impl_type_identifiable!(GetHash);

impl GetHash {
    pub fn new() -> Self {
        Self {
            step: GetHashStep::Init,
            main_page_hash: Felt::ZERO,
            hash_data: Vec::new(),
            main_page_len: 0,
        }
    }
}

impl Default for GetHash {
    fn default() -> Self {
        Self::new()
    }
}

impl Executable for GetHash {
    fn execute<T: BidirectionalStack>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GetHashStep::Init => {
                let proof_reference: &mut [u8] = stack.get_proof_reference();
                let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                self.main_page_len = proof.public_input.main_page.0.len();

                println!("______");
                println!("GetHashStep::Init");
                self.step = GetHashStep::HashData;
                vec![]
            }
            GetHashStep::HashData => {
                // Push main page data for hashing (address and value pairs)
                //+3 because we add zero and 2*main_page_len and 2*main_page_len
                let inputs_len = self.main_page_len * 2 + 3;
                let zero_count = inputs_len.div_ceil(2) * 2 - inputs_len;
                for _ in 0..zero_count {
                    stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                }

                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();

                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                for i in (0..self.main_page_len).rev() {
                    let proof_reference: &mut [u8] = stack.get_proof_reference();
                    let proof: &StarkProof = cast_slice_to_struct::<StarkProof>(proof_reference);
                    let memory = proof.public_input.main_page.0.as_slice();
                    let address = memory[i].address;
                    let value = memory[i].value;

                    stack.push_front(&address.to_bytes_be()).unwrap();
                    stack.push_front(&value.to_bytes_be()).unwrap();

                    // main_page_hash = pedersen_hash(&main_page_hash, &memory.address);
                    // main_page_hash = pedersen_hash(&main_page_hash, &memory.value);
                }
                stack
                    .push_front(&(Felt::TWO * Felt::from(self.main_page_len)).to_bytes_be())
                    .unwrap();

                // Add padding (3 zeros)
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                // main_page_hash =
                //     pedersen_hash(&main_page_hash, &(FELT_2 * Felt::from(self.main_page.len())));

                self.step = GetHashStep::MainPageHash;
                //+2 because we add zero and 2*main_page_len
                let poseidon_len = self.main_page_len * 2 + 2;
                vec![PoseidonHashMany::new(poseidon_len).to_vec_with_type_tag()]
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
                    public_input.log_n_steps,
                    public_input.range_check_min,
                    public_input.range_check_max,
                    public_input.layout,
                ];

                // Add dynamic params if they exist
                if let Some(dynamic_params) = &public_input.dynamic_params {
                    let dynamic_params_vec: Vec<u32> = dynamic_params.clone().into();
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

                println!("GetHashStep::MainPageHash");
                self.step = GetHashStep::Program;
                vec![]
            }
            GetHashStep::Program => {
                // Push all hash_data elements to stack for final hashing
                for felt in self.hash_data.iter().rev() {
                    stack.push_front(&felt.to_bytes_be()).unwrap();
                }

                println!("GetHashStep::Program");
                self.step = GetHashStep::Done;
                vec![PoseidonHashMany::new(self.hash_data.len()).to_vec_with_type_tag()]
            }
            GetHashStep::Done => {
                println!("GetHashStep::Done");
                println!("______");
                vec![]
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.step == GetHashStep::Done
    }
}
