use super::transcript::Transcript;
use felt::Felt;

#[derive(Debug, PartialEq)]
pub struct EcPoint {
    pub x: Felt,
    pub y: Felt,
}

// Accumulation of member expressions for auto generated composition polynomial code.
#[derive(Debug, PartialEq)]
pub struct GlobalValues {
    // Public input.
    pub trace_length: Felt,
    pub initial_pc: Felt,
    pub final_pc: Felt,
    pub initial_ap: Felt,
    pub final_ap: Felt,
    pub initial_pedersen_addr: Felt,
    pub initial_range_check_addr: Felt,
    pub initial_bitwise_addr: Felt,
    pub initial_poseidon_addr: Felt,
    pub range_check_min: Felt,
    pub range_check_max: Felt,
    // Constants.
    pub offset_size: Felt,
    pub half_offset_size: Felt,
    pub pedersen_shift_point: EcPoint,
    // Periodic columns.
    pub pedersen_points_x: Felt,
    pub pedersen_points_y: Felt,
    pub poseidon_poseidon_full_round_key0: Felt,
    pub poseidon_poseidon_full_round_key1: Felt,
    pub poseidon_poseidon_full_round_key2: Felt,
    pub poseidon_poseidon_partial_round_key0: Felt,
    pub poseidon_poseidon_partial_round_key1: Felt,
    // Interaction elements.
    pub memory_multi_column_perm_perm_interaction_elm: Felt,
    pub memory_multi_column_perm_hash_interaction_elm0: Felt,
    pub range_check16_perm_interaction_elm: Felt,
    pub diluted_check_permutation_interaction_elm: Felt,
    pub diluted_check_interaction_z: Felt,
    pub diluted_check_interaction_alpha: Felt,
    // Permutation products.
    pub memory_multi_column_perm_perm_public_memory_prod: Felt,
    pub range_check16_perm_public_memory_prod: Felt,
    pub diluted_check_first_elm: Felt,
    pub diluted_check_permutation_public_memory_prod: Felt,
    pub diluted_check_final_cum_val: Felt,
}

impl Default for GlobalValues {
    fn default() -> Self {
        Self {
            trace_length: Felt::ZERO,
            initial_pc: Felt::ZERO,
            final_pc: Felt::ZERO,
            initial_ap: Felt::ZERO,
            final_ap: Felt::ZERO,
            initial_pedersen_addr: Felt::ZERO,
            initial_range_check_addr: Felt::ZERO,
            initial_bitwise_addr: Felt::ZERO,
            initial_poseidon_addr: Felt::ZERO,
            range_check_min: Felt::ZERO,
            range_check_max: Felt::ZERO,
            offset_size: Felt::ZERO,
            half_offset_size: Felt::ZERO,
            pedersen_shift_point: EcPoint {
                x: Felt::ZERO,
                y: Felt::ZERO,
            },
            pedersen_points_x: Felt::ZERO,
            pedersen_points_y: Felt::ZERO,
            poseidon_poseidon_full_round_key0: Felt::ZERO,
            poseidon_poseidon_full_round_key1: Felt::ZERO,
            poseidon_poseidon_full_round_key2: Felt::ZERO,
            poseidon_poseidon_partial_round_key0: Felt::ZERO,
            poseidon_poseidon_partial_round_key1: Felt::ZERO,
            memory_multi_column_perm_perm_interaction_elm: Felt::ZERO,
            memory_multi_column_perm_hash_interaction_elm0: Felt::ZERO,
            range_check16_perm_interaction_elm: Felt::ZERO,
            diluted_check_permutation_interaction_elm: Felt::ZERO,
            diluted_check_interaction_z: Felt::ZERO,
            diluted_check_interaction_alpha: Felt::ZERO,
            memory_multi_column_perm_perm_public_memory_prod: Felt::ZERO,
            range_check16_perm_public_memory_prod: Felt::ZERO,
            diluted_check_first_elm: Felt::ZERO,
            diluted_check_permutation_public_memory_prod: Felt::ZERO,
            diluted_check_final_cum_val: Felt::ZERO,
        }
    }
}

// Elements that are sent from the prover after the commitment on the original trace.
// Used for components after the first interaction, e.g., memory and range check.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InteractionElements {
    pub memory_multi_column_perm_perm_interaction_elm: Felt,
    pub memory_multi_column_perm_hash_interaction_elm0: Felt,
    pub range_check16_perm_interaction_elm: Felt,
    pub diluted_check_permutation_interaction_elm: Felt,
    pub diluted_check_interaction_z: Felt,
    pub diluted_check_interaction_alpha: Felt,
}

impl InteractionElements {
    pub fn new(transcript: &mut Transcript) -> Self {
        Self {
            memory_multi_column_perm_perm_interaction_elm: transcript.random_felt_to_prover(),
            memory_multi_column_perm_hash_interaction_elm0: transcript.random_felt_to_prover(),
            range_check16_perm_interaction_elm: transcript.random_felt_to_prover(),
            diluted_check_permutation_interaction_elm: transcript.random_felt_to_prover(),
            diluted_check_interaction_z: transcript.random_felt_to_prover(),
            diluted_check_interaction_alpha: transcript.random_felt_to_prover(),
        }
    }
}
