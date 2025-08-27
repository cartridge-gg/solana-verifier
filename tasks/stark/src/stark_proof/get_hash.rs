use crate::{
    pedersen::PedersenHash, poseidon::PoseidonHashMany, swiftness::stark::types::StarkProof,
};
use felt::Felt;
use utils::{impl_type_identifiable, BidirectionalStack, Executable, ProofData, TypeIdentifiable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetHashStep {
    Init,
    WaitForPedersenAddress,
    WaitForPedersenValue,
    MainPageHash,
    Program,
    Done,
}

#[repr(C)]
pub struct GetHash {
    step: GetHashStep,
    main_page_hash: Felt,
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
    fn execute<T: BidirectionalStack + ProofData>(&mut self, stack: &mut T) -> Vec<Vec<u8>> {
        match self.step {
            GetHashStep::Init => {
                let proof: &StarkProof = stack.get_proof_reference();
                self.main_page_len = proof.public_input.main_page.0.len();
                self.current_memory_index = 0;
                self.accumulated_hash = Felt::ZERO;

                if self.main_page_len == 0 {
                    self.step = GetHashStep::MainPageHash;
                    return self.execute_final_pedersen_hash(stack);
                }

                let memory = proof.public_input.main_page.0.as_slice();

                PedersenHash::push_input(
                    self.accumulated_hash,
                    memory[self.current_memory_index].address,
                    stack,
                );

                self.step = GetHashStep::WaitForPedersenAddress;
                vec![PedersenHash::new().to_vec_with_type_tag()]
            }
            GetHashStep::WaitForPedersenAddress => {
                let bytes = stack.borrow_front();
                let pedersen_result = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                self.accumulated_hash = pedersen_result;

                let proof: &StarkProof = stack.get_proof_reference();
                let memory = proof.public_input.main_page.0.as_slice();

                PedersenHash::push_input(
                    self.accumulated_hash,
                    memory[self.current_memory_index].value,
                    stack,
                );

                self.step = GetHashStep::WaitForPedersenValue;
                vec![PedersenHash::new().to_vec_with_type_tag()]
            }
            GetHashStep::WaitForPedersenValue => {
                let bytes = stack.borrow_front();
                let pedersen_result = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                self.accumulated_hash = pedersen_result;
                self.current_memory_index += 1;

                if self.current_memory_index < self.main_page_len {
                    let proof: &StarkProof = stack.get_proof_reference();
                    let memory = proof.public_input.main_page.0.as_slice();

                    PedersenHash::push_input(
                        self.accumulated_hash,
                        memory[self.current_memory_index].address,
                        stack,
                    );

                    self.step = GetHashStep::WaitForPedersenAddress;
                    vec![PedersenHash::new().to_vec_with_type_tag()]
                } else {
                    self.step = GetHashStep::MainPageHash;
                    let length_multiplier = Felt::TWO * Felt::from(self.main_page_len);

                    PedersenHash::push_input(self.accumulated_hash, length_multiplier, stack);
                    vec![PedersenHash::new().to_vec_with_type_tag()]
                }
            }
            GetHashStep::MainPageHash => {
                let bytes = stack.borrow_front();
                self.main_page_hash = Felt::from_bytes_be_slice(bytes);
                stack.pop_front();
                stack.pop_front();
                stack.pop_front();

                let (
                    n_verifier_friendly_commitment_layers,
                    log_n_steps,
                    range_check_min,
                    range_check_max,
                    layout,
                    has_dynamic_params,
                    dynamic_params_len,
                    segments_len,
                    padding_addr,
                    padding_value,
                    headers_len,
                    main_page_len,
                ) = {
                    let proof: &StarkProof = stack.get_proof_reference();
                    let public_input = &proof.public_input;

                    let dynamic_params_len = if let Some(ref dp) = public_input.dynamic_params {
                        let dynamic_params_vec: Vec<u32> = (*dp).into();
                        dynamic_params_vec.len()
                    } else {
                        0
                    };

                    (
                        self.n_verifier_friendly_commitment_layers,
                        public_input.log_n_steps,
                        public_input.range_check_min,
                        public_input.range_check_max,
                        public_input.layout,
                        public_input.dynamic_params.is_some(),
                        dynamic_params_len,
                        public_input.segments.len(),
                        public_input.padding_addr,
                        public_input.padding_value,
                        public_input.continuous_page_headers.len(),
                        public_input.main_page.0.len(),
                    )
                };

                let mut total_elements = 5; // basic fields
                total_elements += dynamic_params_len;
                total_elements += segments_len * 2;
                total_elements += 5; // padding_addr, padding_value, headers_len+1, main_page_len, main_page_hash
                total_elements += headers_len * 3;

                let inputs_with_one = total_elements + 1;
                let zero_count = inputs_with_one.div_ceil(2) * 2 - inputs_with_one;
                for _ in 0..zero_count {
                    stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                }

                stack.push_front(&Felt::ONE.to_bytes_be()).unwrap();

                for i in (0..headers_len).rev() {
                    let (start_address, size, hash) = {
                        let proof: &StarkProof = stack.get_proof_reference();
                        let header = proof.public_input.continuous_page_headers.as_slice();
                        let header = header[i];
                        (header.start_address, header.size, header.hash)
                    };

                    stack.push_front(&hash.to_bytes_be()).unwrap();
                    stack.push_front(&size.to_bytes_be()).unwrap();
                    stack.push_front(&start_address.to_bytes_be()).unwrap();
                }

                stack
                    .push_front(&self.main_page_hash.to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&Felt::from(main_page_len).to_bytes_be())
                    .unwrap();
                stack
                    .push_front(&Felt::from(headers_len + 1).to_bytes_be())
                    .unwrap();
                stack.push_front(&padding_value.to_bytes_be()).unwrap();
                stack.push_front(&padding_addr.to_bytes_be()).unwrap();

                for i in (0..segments_len).rev() {
                    let (begin_addr, stop_ptr) = {
                        let proof: &StarkProof = stack.get_proof_reference();
                        let segment = proof.public_input.segments.as_slice();
                        let segment = segment[i];
                        (segment.begin_addr, segment.stop_ptr)
                    };

                    stack.push_front(&stop_ptr.to_bytes_be()).unwrap();
                    stack.push_front(&begin_addr.to_bytes_be()).unwrap();
                }

                if has_dynamic_params {
                    let proof: &StarkProof = stack.get_proof_reference();
                    let public_input = &proof.public_input;
                    if let Some(dynamic_params) = &public_input.dynamic_params {
                        let dynamic_params_vec: Vec<u32> = (*dynamic_params).into();
                        for value in dynamic_params_vec.iter().rev() {
                            let felt = Felt::from(*value);
                            stack.push_front(&felt.to_bytes_be()).unwrap();
                        }
                    }
                }

                stack.push_front(&layout.to_bytes_be()).unwrap();
                stack.push_front(&range_check_max.to_bytes_be()).unwrap();
                stack.push_front(&range_check_min.to_bytes_be()).unwrap();
                stack.push_front(&log_n_steps.to_bytes_be()).unwrap();
                stack
                    .push_front(&n_verifier_friendly_commitment_layers.to_bytes_be())
                    .unwrap();

                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();
                stack.push_front(&Felt::ZERO.to_bytes_be()).unwrap();

                self.step = GetHashStep::Program;
                vec![PoseidonHashMany::new(total_elements).to_vec_with_type_tag()]
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
    fn execute_final_pedersen_hash<T: BidirectionalStack>(
        &mut self,
        stack: &mut T,
    ) -> Vec<Vec<u8>> {
        // Final hash with the length multiplier
        let length_multiplier = Felt::TWO * Felt::from(self.main_page_len);

        PedersenHash::push_input(self.accumulated_hash, length_multiplier, stack);
        vec![PedersenHash::new().to_vec_with_type_tag()]
    }
}
