mod fixtures;

use std::fs;
use std::result;

use felt::Felt;
use stark::swiftness::fri::layer::FriLayerQuery;
use stark::swiftness::stark::types::cast_slice_to_struct;
use stark::swiftness::stark::types::cast_struct_to_slice;
use utils::BidirectionalStack;
use utils::Scheduler;
use verifier::state::BidirectionalStackAccount;
#[test]
fn test_fri_verify() {
    let mut stack = BidirectionalStackAccount::default();
    let task = stark::stark_proof::stark_verify::FriVerify::new();

    stack.push_task(task);
    push_data(&mut stack);
    while !stack.is_empty_back() {
        stack.execute();
    }
    let result_data = stack.borrow_front();
    let result: &Vec<FriLayerQuery> = cast_slice_to_struct::<Vec<FriLayerQuery>>(result_data);
    let expected = get_expected();
    assert_eq!(result, &expected);
}

fn push_data(stack: &mut BidirectionalStackAccount) {
    let queries = fixtures::queries::get();
    let fri_commitment: stark::swiftness::fri::types::Commitment = fixtures::fri_commitment::get();
    let fri_decommitment: stark::swiftness::commitment::types::Decommitment =
        fixtures::fri_decommitment::get();
    let witness: stark::swiftness::commitment::types::Witness = fixtures::witness::get();
    let mut input = stark::swiftness::fri::types::FriVerifyInput {
        queries,
        fri_commitment,
        fri_decommitment,
        witness,
    };
    let bytes = cast_struct_to_slice(&mut input);
    stack
        .push_front(bytes)
        .expect("Failed to push data onto the stack")
}

pub fn get_expected() -> Vec<FriLayerQuery> {
    vec![
        "0xd20990",
        "0x56589147f36eee3f7976a1542599dd32be46d202f4ec49dccef821f43ade30f",
        "0x66935f069103aee8c80a1bc8b642632abf68f474bf61195af1bd171f58d301a",
        "0x1702a2dc",
        "0x6da23461f6dc6aac5624da021558eaea6f8039c59a3a1596694aaade6ae5aea",
        "0x2f1e1459a9f18dee29b44e204b6757d97e81a1c2f9ca7b20fa3f5be032f2d6f",
        "0x233bfb24",
        "0x7c2cb3f9065f1c08480be0521698325689a3346e6fd358e65d98f43ef91848e",
        "0x690c9a2b28d95a0860b4358c3e5aa75b38cf1a4b09164108bf345c1720a16f1",
        "0x2fc8f32e",
        "0x7272da9be8a83b5007e3b63487265431b894626aabe48070e87412a33f06e21",
        "0x596f9b7d83a52bfa9529f1b60eca157b94067465c0daa6e610aff92f290494",
        "0x367bcdcb",
        "0x48b12d9655668770fbb57fa2aaa241df1aff1195a68c44ea912563e633c0311",
        "0x7c7cec3a10c3b98319330121f8f1da42f5228cbd85af2109f3a3af16bbc0eba",
        "0x44445cc6",
        "0x5613f5cb362f21af6a28237858c8e25930ee6d1f03d615991862c966b696b07",
        "0x62d12d879bae009f8954a37142737d4b2196dafdd6e879d24343a11957d64d1",
        "0x4bf4ed93",
        "0x1daf84477265f19fbcbb8fa7b62d85a14221de9add62996cb6a1eba477532c",
        "0x30cf8476593c01bdb2e96d422e43b7d750bc38b14518ac1e48248cfee59829f",
        "0x8df252ca",
        "0x255f150abc9f168bbf353a77445b26a0c4c3243be19985398cef35916b39349",
        "0x796fbe57f98a6a0e1669cf941d8637f8982569e0634f5b743003bfce8e47887",
        "0x97a48b5b",
        "0x3d99e7912b03d046b302ba451fd39d4a2f22173c5d3facd40eaf8e4ca160729",
        "0x2b89255cdce76fb660391f3add4eeaa4efa2940665314984aee291c419b944f",
        "0xafea6443",
        "0x3931a734c9e17b5d11721226625ce4d8c2ce416cd05168442c636717b8f2b7c",
        "0x4f99d1ed9f21dc27366ea9ec62962ef4138645098cd6832afd2ff3558eef800",
        "0xc62f63b8",
        "0x501483805f53ae20ff3317425627bab5a8a31487ce9e62bf09f2ad591d4d636",
        "0x60e79a171d9f13fa90ae340ab405471bd34066be1f8e24b9afdf1485ced9fe6",
        "0xd76e5257",
        "0x55bf2ccb8e98ecd75c23c941d8201b3ff3cce32f4c2fedeea787307cd42f275",
        "0x414892add709d6bf9e570a569f4bd9de2d2a115c29372a04b7b34caa038d5c0",
        "0xecca885b",
        "0x2872e8b5f38ac80c1db5cd85801c20696a1480e7a35d532a8d06d51428d7417",
        "0x3854a74220154f3ca646f96b8d08a2fcd49e821892dab343f91fb4dfb7cab2c",
        "0xedc42f8b",
        "0x2217dfcf29dd655b6a85d1769e7cf444ecefa2cd276e1c6de73d5d039c6cf8e",
        "0x2bb7c4f4c587d4584fa0deb8ebd1cd3ccac1f6b6461e76e7405d7457088f127",
        "0xf6821efe",
        "0x1558aa1be37c22f07b2b0422b37a5f67ef6285c8a33a94f7d46347bfc64b9e2",
        "0x57a8652c3c206b52fe0905d7f1e5e61ba4c4e323756a1eeec4fe93390b6ab3f",
        "0xf7769c26",
        "0x43bbcf9a0483a1f8e74570452b870ef248e4d5aa227bf64910c0c92d0afa598",
        "0x771db9fb286e2646b1e36e0dd2830df1aede51b8f2c118103c2808b6e49c591",
    ]
    .chunks(3)
    .map(|chunk| FriLayerQuery {
        index: Felt::from_hex_unchecked(chunk[0]),
        y_value:Felt::from_hex_unchecked(chunk[1]),
        x_inv_value:Felt::from_hex_unchecked(chunk[2]),
    })
    .collect()
}
