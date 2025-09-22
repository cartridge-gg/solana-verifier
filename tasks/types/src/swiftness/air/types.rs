use crate::funvec::{FunVec, FUNVEC_PAGES};
use felt::Felt;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SegmentInfo {
    // Start address of the memory segment.
    pub begin_addr: Felt,
    // Stop pointer of the segment - not necessarily the end of the segment.
    pub stop_ptr: Felt,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Page(pub FunVec<AddrValue, FUNVEC_PAGES>);

impl Page {
    // Returns the product of (z - (addr + alpha * val)) over a single page.
    pub fn get_product(&self, z: Felt, alpha: Felt) -> Felt {
        let mut res = Felt::ONE;
        let mut i = 0;
        loop {
            if i == self.0.len() {
                break res;
            }
            let current = &self.0.as_slice()[i];

            res *= z - (current.address + alpha * current.value);
            i += 1;
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AddrValue {
    pub address: Felt,
    pub value: Felt,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ContinuousPageHeader {
    // Start address.
    pub start_address: Felt,
    // Size of the page.
    pub size: Felt,
    // Hash of the page.
    pub hash: Felt,
    // Cumulative product of the page.
    pub prod: Felt,
}
