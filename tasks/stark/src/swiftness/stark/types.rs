use super::config::StarkConfig;
use crate::funvec::{FunVec, FUNVEC_OODS};
use crate::swiftness::air::public_memory::PublicInput;
use crate::swiftness::air::trace;
use crate::swiftness::commitment::table;
use crate::swiftness::{fri, pow::pow};
use felt::Felt;

pub fn cast_slice_to_struct<T>(slice: &[u8]) -> &T
where
    T: Sized,
{
    assert_eq!(slice.len(), std::mem::size_of::<T>());
    unsafe { &*(slice.as_ptr() as *const T) }
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
pub struct StarkWitness {
    pub traces_decommitment: trace::Decommitment,
    pub traces_witness: trace::Witness,
    pub composition_decommitment: table::types::Decommitment,
    pub composition_witness: table::types::Witness,
    pub fri_witness: fri::types::Witness,
}
#[cfg(test)]
mod test {
    use crate::{
        funvec::FunVec,
        swiftness::{
            air::public_memory::PublicInput,
            air::types::Page,
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
                dynamic_params: None,
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
