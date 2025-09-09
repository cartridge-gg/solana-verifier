use felt::Felt;

#[derive(PartialEq, Eq, Debug)]
pub struct FriLayerQuery {
    pub index: Felt,
    pub y_value: Felt,
    pub x_inv_value: Felt,
}