use crate::swiftness::commitment::table::types::Commitment as TableCommitment;
use crate::swiftness::commitment::table::types::Witness as TableWitness;
use crate::swiftness::fri::config::Config;
use felt::Felt;

// Commitment values for FRI. Used to generate a commitment by "reading" these values
// from the transcript.
#[derive(Debug, Clone, PartialEq)]
pub struct UnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    pub inner_layers: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: Vec<Felt>,
}

#[derive(Debug, PartialEq)]
pub struct Commitment {
    pub config: Config,
    // Array of size n_layers - 1 containing table commitments for each inner layer.
    pub inner_layers: Vec<TableCommitment>,
    // Array of size n_layers, of one evaluation point for each layer.
    pub eval_points: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    pub last_layer_coefficients: Vec<Felt>,
}

#[derive(Debug, PartialEq)]
pub struct Decommitment {
    // Array of size n_values, containing the values of the input layer at query indices.
    pub values: Vec<Felt>,
    // Array of size n_values, containing the field elements that correspond to the query indices
    // (See queries_to_points).
    pub points: Vec<Felt>,
}

// A witness for the decommitment of the FRI layers over queries.
#[derive(Debug, Clone, PartialEq)]
pub struct Witness {
    // An array of size n_layers - 1, containing a witness for each inner layer.
    pub layers: Vec<LayerWitness>,
}

// A witness for a single FRI layer. This witness is required to verify the transition from an
// inner layer to the following layer.
#[derive(Debug, Clone, PartialEq)]
pub struct LayerWitness {
    // Values for the sibling leaves required for decommitment.
    pub leaves: Vec<Felt>,
    // Table commitment witnesses for decommiting all the leaves.
    pub table_witness: TableWitness,
}
