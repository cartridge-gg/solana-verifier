use stark::swiftness::commitment::types::{LayerWitness, Witness};

mod layer_0;
mod layer_1;
mod layer_2;
mod layer_3;
mod layer_4;
mod layer_5;
mod layer_6;
mod layer_7;
pub fn get() -> Witness {
    Witness {
        layers: get_layers(),
    }
}
pub fn get_layers() -> Vec<LayerWitness> {
    vec![
        layer_0::get(),
        layer_1::get(),
        layer_2::get(),
        layer_3::get(),
        layer_4::get(),
        layer_5::get(),
        layer_6::get(),
        layer_7::get(),
    ]
}
