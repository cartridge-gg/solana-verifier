use crate::fixtures::decommitment;
use stark::funvec::FunVec;
use stark::swiftness::air::trace::Decommitment as TraceDecommitment;
use stark::swiftness::commitment::table::types::Decommitment as TableDecommitment;
use swiftness_proof_parser::transform::MONTGOMERY_R;

pub fn get() -> TraceDecommitment {
    let original_decommitment_values = decommitment::get_original_decommitment();
    let original_montgomery_values = FunVec::from_vec(
        original_decommitment_values
            .iter()
            .map(|x| x * MONTGOMERY_R)
            .collect(),
    );

    let interaction_decommitment_values = decommitment::get_interaction_decommitment();

    let interaction_montgomery_values = FunVec::from_vec(
        interaction_decommitment_values
            .iter()
            .map(|x| x * MONTGOMERY_R)
            .collect(),
    );

    let table_decommitment_original =
        TableDecommitment::new(original_decommitment_values, original_montgomery_values);
    let table_decommitment_interaction = TableDecommitment::new(
        interaction_decommitment_values,
        interaction_montgomery_values,
    );

    let trace_decommitment = TraceDecommitment {
        original: table_decommitment_original,
        interaction: table_decommitment_interaction,
    };

    trace_decommitment
}
