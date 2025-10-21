#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct Config {
    // Proof of work difficulty (number of bits required to be 0).
    pub n_bits: u8,
}
