use super::config::StarkConfig;
use crate::funvec::{
    FunVec, FUNVEC_AUTHENTICATIONS, FUNVEC_COSET_ELEMENTS, FUNVEC_DECOMMITMENT_VALUES,
    FUNVEC_FRI_LAYER_QUERIES, FUNVEC_LEAVES, FUNVEC_OODS, FUNVEC_QUERIES, FUNVEC_QUERY_INDICES,
};
use crate::swiftness;
use crate::swiftness::air::public_memory::PublicInput;
use crate::swiftness::air::trace;
use crate::swiftness::commitment::table;
use crate::swiftness::{fri, pow::pow};
use felt::Felt;
use fri::types::Witness as FriWitness;
use swiftness::air::trace::Commitment as TracesCommitment;
use swiftness::commitment::table::types::Commitment as TableCommitment;
use swiftness::fri::types::Commitment as FriCommitment;
use table::types::Decommitment as TableDecommitment;
use table::types::Witness as TableWitness;
use trace::Decommitment as TracesDecommitment;
use trace::Witness as TracesWitness;

pub fn cast_slice_to_struct<T>(slice: &[u8]) -> &T
where
    T: Sized,
{
    assert_eq!(slice.len(), std::mem::size_of::<T>());
    unsafe { &*(slice.as_ptr() as *const T) }
}
pub fn cast_slice_to_struct_mut<T>(slice: &mut [u8]) -> &mut T
where
    T: Sized,
{
    assert_eq!(slice.len(), std::mem::size_of::<T>());
    unsafe { &mut *(slice.as_ptr() as *mut T) }
}
pub fn cast_struct_to_slice<T>(s: &T) -> &[u8]
where
    T: Sized,
{
    let ptr = s as *const T as *const u8;
    let len = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(ptr, len) }
}
pub fn cast_struct_to_slice_mut<T>(s: &mut T) -> &mut [u8]
where
    T: Sized,
{
    let ptr = s as *mut T as *mut u8;
    let len = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts_mut(ptr, len) }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StarkProof {
    pub config: StarkConfig,
    pub public_input: PublicInput,
    pub unsent_commitment: StarkUnsentCommitment,
    pub witness: StarkWitness,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct StarkUnsentCommitment {
    pub traces: trace::UnsentCommitment,
    pub composition: Felt,
    // n_oods_values elements. The i-th value is the evaluation of the i-th mask item polynomial at
    // the OODS point, where the mask item polynomial is the interpolation polynomial of the
    // corresponding column shifted by the corresponding row_offset.
    pub oods_values: FunVec<Felt, FUNVEC_OODS>,
    pub fri: fri::types::UnsentCommitment,
    pub proof_of_work: pow::UnsentCommitment,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct StarkWitness {
    pub traces_decommitment: TracesDecommitment,
    pub traces_witness: TracesWitness,
    pub composition_decommitment: TableDecommitment,
    pub composition_witness: TableWitness,
    pub fri_witness: FriWitness,
}

#[derive(Debug, PartialEq, Default, Clone)]
#[repr(C)]
pub struct StarkCommitment<InteractionElements> {
    pub traces: TracesCommitment<InteractionElements>,
    pub composition: TableCommitment,
    pub interaction_after_composition: Felt,
    pub oods_values: FunVec<Felt, FUNVEC_OODS>,
    pub interaction_after_oods: FunVec<Felt, FUNVEC_OODS>,
    pub fri: FriCommitment,
}
#[derive(Debug)]
#[repr(C)]
pub struct VerifyVariables {
    // Store queries as pairs of (index, value, depth) - each query takes 3 Felts
    // fix this as it's not intuitive - we have temp_queries to store queries that have two fields (index, value)
    pub queries: [Felt; FUNVEC_QUERIES],
    pub authentications: [Felt; FUNVEC_AUTHENTICATIONS],
    pub decommitment_values: [Felt; FUNVEC_DECOMMITMENT_VALUES],
    pub montgomery_values: [Felt; FUNVEC_DECOMMITMENT_VALUES],
    pub temp_queries: [Felt; FUNVEC_QUERIES],
    pub queries_indexes: [Felt; 16],
}

impl Default for VerifyVariables {
    fn default() -> Self {
        Self {
            queries: [Felt::ZERO; FUNVEC_QUERIES],
            authentications: [Felt::ZERO; FUNVEC_AUTHENTICATIONS],
            decommitment_values: [Felt::ZERO; FUNVEC_DECOMMITMENT_VALUES],
            montgomery_values: [Felt::ZERO; FUNVEC_DECOMMITMENT_VALUES],
            temp_queries: [Felt::ZERO; FUNVEC_QUERIES],
            queries_indexes: [Felt::ZERO; 16],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[repr(C)]
pub struct FriVerifyData {
    pub queries: FunVec<Felt, FUNVEC_QUERIES>,
    pub fri_decommitment: swiftness::commitment::types::Decommitment,
    pub current_layer: usize,
    pub layer_queries: FunVec<fri::types::FriLayerQuery, FUNVEC_FRI_LAYER_QUERIES>,
    pub active_query_count: usize,
    pub working_elements: FunVec<Felt, FUNVEC_COSET_ELEMENTS>,
    pub sibling_witness: FunVec<Felt, FUNVEC_LEAVES>,
    pub working_indices: FunVec<Felt, FUNVEC_QUERY_INDICES>,
    pub working_y_values: FunVec<Felt, FUNVEC_LEAVES>,
    pub coset_size: Felt,
    pub eval_point: Felt,
    pub next_x_inv_value: Felt,
    pub coset_x_inv: Felt,
    pub current_coset_index: usize,
}

impl Default for FriVerifyData {
    fn default() -> Self {
        Self {
            queries: FunVec::default(),
            fri_decommitment: swiftness::commitment::types::Decommitment::default(),
            current_layer: 0,
            layer_queries: FunVec::default(),
            active_query_count: 0,
            working_elements: FunVec::default(),
            sibling_witness: FunVec::default(),
            working_indices: FunVec::default(),
            working_y_values: FunVec::default(),
            coset_size: Felt::ZERO,
            eval_point: Felt::ZERO,
            next_x_inv_value: Felt::ZERO,
            coset_x_inv: Felt::ZERO,
            current_coset_index: 0,
        }
    }
}

impl FriVerifyData {
    #[inline]
    pub fn get_first_active_query(&self) -> Option<&fri::types::FriLayerQuery> {
        if self.active_query_count > 0 {
            self.layer_queries.get(0)
        } else {
            None
        }
    }

    #[inline]
    pub fn remove_first_active_query(&mut self) {
        if self.active_query_count > 0 {
            self.layer_queries.shift(1);
            self.active_query_count = self.active_query_count.saturating_sub(1);
        }
    }

    #[inline]
    pub fn push_next_query(&mut self, query: fri::types::FriLayerQuery) {
        self.layer_queries.push(query);
    }

    pub fn advance_layer(&mut self) {
        self.layer_queries.shift(self.active_query_count);
        self.active_query_count = self.layer_queries.len();
    }

    #[inline]
    pub fn init_active_queries(&mut self) {
        self.active_query_count = self.layer_queries.len();
    }
}
#[cfg(test)]
mod test {
    use crate::{
        funvec::FunVec,
        swiftness::{
            air::{dynamic::DynamicParams, public_memory::PublicInput, types::Page},
            stark::{
                config::StarkConfig,
                types::{
                    cast_slice_to_struct, cast_struct_to_slice, StarkProof, StarkUnsentCommitment,
                    StarkWitness,
                },
            },
        },
    };
    use felt::Felt;

    #[test]
    fn test_stark_proof() {
        let proof = StarkProof {
            public_input: PublicInput {
                log_n_steps: Felt::from(1),
                range_check_min: Felt::from(2),
                range_check_max: Felt::from(3),
                layout: Felt::from(4),
                // dynamic_params: DynamicParams::default(),
                segments: FunVec::default(),
                padding_addr: Felt::from(5),
                padding_value: Felt::from(6),
                main_page: Page::default(),
                continuous_page_headers: FunVec::default(),
            },
            config: StarkConfig::default(),
            unsent_commitment: StarkUnsentCommitment::default(),
            witness: StarkWitness::default(),
        };
        println!("proof: {proof:?}");
        let mut proof_clone = proof.clone();
        let bytes = cast_struct_to_slice(&mut proof_clone);

        let proof_from_bytes = cast_slice_to_struct::<StarkProof>(bytes);
        assert_eq!(proof_from_bytes, &proof);
    }
}
