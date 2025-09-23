use crate::funvec::FunVec;
use crate::swiftness::commitment::table;
use felt::Felt;

const _MAX_LAST_LAYER_LOG_DEGREE_BOUND: u64 = 15;
const _MAX_FRI_LAYERS: u64 = 15;
const MAX_FRI_LAYERS_USIZE: usize = 15;
const _MIN_FRI_LAYERS: u64 = 2;
const _MAX_FRI_STEP: u64 = 4;
const _MIN_FRI_STEP: u64 = 1;

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct Config {
    // Log2 of the size of the input layer to FRI.
    pub log_input_size: Felt,
    // Number of layers in the FRI. Inner + last layer.
    pub n_layers: Felt,
    // Array of size n_layers - 1, each entry is a configuration of a table commitment for the
    // corresponding inner layer.
    pub inner_layers: FunVec<table::config::Config, MAX_FRI_LAYERS_USIZE>,
    // Array of size n_layers, each entry represents the FRI step size,
    // i.e. the number of FRI-foldings between layer i and i+1.
    pub fri_step_sizes: FunVec<Felt, MAX_FRI_LAYERS_USIZE>,
    pub log_last_layer_degree_bound: Felt,
}
