use crate::swiftness::commitment;
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Config {
    pub original: commitment::table::config::Config,
    pub interaction: commitment::table::config::Config,
}
