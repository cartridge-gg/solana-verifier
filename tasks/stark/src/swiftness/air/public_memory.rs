use crate::funvec::{FunVec, FUNVEC_SEGMENTS};
use felt::Felt;

use super::{
    dynamic::DynamicParams,
    types::{ContinuousPageHeader, Page, SegmentInfo},
};
use felt::NonZeroFelt;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PublicInput {
    pub log_n_steps: Felt,
    pub range_check_min: Felt,
    pub range_check_max: Felt,
    pub layout: Felt,
    pub dynamic_params: Option<DynamicParams>,
    pub segments: FunVec<SegmentInfo, FUNVEC_SEGMENTS>,
    pub padding_addr: Felt,
    pub padding_value: Felt,
    pub main_page: Page,
    pub continuous_page_headers: FunVec<ContinuousPageHeader, 0>,
}

impl PublicInput {
    // Returns the ratio between the product of all public memory cells and z^|public_memory|.
    // This is the value that needs to be at the memory_multi_column_perm_perm_public_memory_prod
    // member expression.
    #[inline(always)]
    pub fn get_public_memory_product_ratio(
        &self,
        z: Felt,
        alpha: Felt,
        public_memory_column_size: Felt,
    ) -> Felt {
        let (pages_product, total_length) = self.get_public_memory_product(z, alpha);

        // Pad and divide
        let numerator = z.pow_felt(&public_memory_column_size);
        let padded = z - (self.padding_addr + alpha * self.padding_value);

        assert!(total_length <= public_memory_column_size);
        let denominator_pad = padded.pow_felt(&(public_memory_column_size - total_length));

        numerator
            .field_div(&NonZeroFelt::from_felt_unchecked(pages_product))
            .field_div(&NonZeroFelt::from_felt_unchecked(denominator_pad))
    }
    // Returns the product of all public memory cells.
    #[inline(always)]
    pub fn get_public_memory_product(&self, z: Felt, alpha: Felt) -> (Felt, Felt) {
        let main_page_prod = self.main_page.get_product(z, alpha);

        let (continuous_pages_prod, continuous_pages_total_length) =
            get_continuous_pages_product(&self.continuous_page_headers);

        let prod = main_page_prod * continuous_pages_prod;
        let total_length = Felt::from(self.main_page.0.len()) + continuous_pages_total_length;

        (prod, total_length)
    }
}
#[inline(always)]
fn get_continuous_pages_product(page_headers: &FunVec<ContinuousPageHeader, 0>) -> (Felt, Felt) {
    let mut res = Felt::ONE;
    let mut total_length = Felt::ZERO;

    for header in page_headers {
        res *= header.prod;
        total_length += header.size
    }

    (res, total_length)
}
