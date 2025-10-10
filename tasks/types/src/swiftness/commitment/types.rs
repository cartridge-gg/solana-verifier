use crate::{
    funvec::{FunVec, FUNVEC_DECOMMITMENT_VALUES, FUNVEC_LAST_LAYER, FUNVEC_LAYERS, FUNVEC_LEAVES},
    swiftness::{self, commitment::table},
};
use felt::Felt;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    pub inner_layers: FunVec<Felt, FUNVEC_LAYERS>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: FunVec<Felt, FUNVEC_LAST_LAYER>,
}

// A witness for the decommitment of the FRI layers over queries.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Witness {
    // An array of size n_layers - 1, containing a witness for each inner layer.
    pub layers: FunVec<LayerWitness, FUNVEC_LAYERS>,
}

// A witness for a single FRI layer. This witness is required to verify the transition from an
// inner layer to the following layer.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LayerWitness {
    pub leaves: FunVec<Felt, FUNVEC_LEAVES>,
    // Table commitment witnesses for decommiting all the leaves.
    pub table_witness: table::types::Witness,
}

#[repr(C)]
#[derive(Debug, PartialEq, Default)]
pub struct Commitment {
    pub config: swiftness::fri::config::Config,
    // Array of size n_layers - 1 containing table commitments for each inner layer.
    // pub inner_layers: Vec<swiftness::commitment::table::types::Commitment>,
    pub inner_layers: FunVec<swiftness::commitment::table::types::Commitment, FUNVEC_LAYERS>,
    // Array of size n_layers, of one evaluation point for each layer.
    pub eval_points: FunVec<Felt, FUNVEC_LAYERS>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: FunVec<Felt, FUNVEC_LAST_LAYER>,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
#[repr(C)]
pub struct Decommitment {
    // Array of size n_values, containing the values of the input layer at query indices.
    pub values: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
    // Array of size n_values, containing the field elements that correspond to the query indices
    pub points: FunVec<Felt, FUNVEC_DECOMMITMENT_VALUES>,
    // (See queries_to_points).
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Default, Copy)]
pub struct FriLayerQuery {
    pub index: Felt,
    pub y_value: Felt,
    pub x_inv_value: Felt,
}
