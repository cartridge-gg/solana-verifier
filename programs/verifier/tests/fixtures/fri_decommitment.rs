use felt::Felt;
use stark::swiftness::commitment::types::Decommitment;

pub fn get() -> Decommitment {
    Decommitment {
        values: get_values(),
        points: get_points(),
    }
}

pub fn get_values() -> Vec<Felt> {
    vec![
        "0x56589147f36eee3f7976a1542599dd32be46d202f4ec49dccef821f43ade30f",
        "0x6da23461f6dc6aac5624da021558eaea6f8039c59a3a1596694aaade6ae5aea",
        "0x7c2cb3f9065f1c08480be0521698325689a3346e6fd358e65d98f43ef91848e",
        "0x7272da9be8a83b5007e3b63487265431b894626aabe48070e87412a33f06e21",
        "0x48b12d9655668770fbb57fa2aaa241df1aff1195a68c44ea912563e633c0311",
        "0x5613f5cb362f21af6a28237858c8e25930ee6d1f03d615991862c966b696b07",
        "0x1daf84477265f19fbcbb8fa7b62d85a14221de9add62996cb6a1eba477532c",
        "0x255f150abc9f168bbf353a77445b26a0c4c3243be19985398cef35916b39349",
        "0x3d99e7912b03d046b302ba451fd39d4a2f22173c5d3facd40eaf8e4ca160729",
        "0x3931a734c9e17b5d11721226625ce4d8c2ce416cd05168442c636717b8f2b7c",
        "0x501483805f53ae20ff3317425627bab5a8a31487ce9e62bf09f2ad591d4d636",
        "0x55bf2ccb8e98ecd75c23c941d8201b3ff3cce32f4c2fedeea787307cd42f275",
        "0x2872e8b5f38ac80c1db5cd85801c20696a1480e7a35d532a8d06d51428d7417",
        "0x2217dfcf29dd655b6a85d1769e7cf444ecefa2cd276e1c6de73d5d039c6cf8e",
        "0x1558aa1be37c22f07b2b0422b37a5f67ef6285c8a33a94f7d46347bfc64b9e2",
        "0x43bbcf9a0483a1f8e74570452b870ef248e4d5aa227bf64910c0c92d0afa598",
    ]
    .iter()
    .map(|s| Felt::from_hex_unchecked(s))
    .collect()
}

pub fn get_points() -> Vec<Felt> {
    vec![
        "0x19def6309c27c3fa7844c5dcf97482dfb990623fffa356c0b6aa93a84840728",
        "0x492280f95460c8f9db2fecc27ee0a783fcf1deab4f327511844f9bb42425cf6",
        "0x71563605a5b60d9422cadbcfec42ad8e9c0852480122970c88133a7cbd8f56b",
        "0x3af83aef91f27a7940b894ae7ca082a482078c31a322a39b76b4f5b1c44b6e1",
        "0x5e4dfa204eab845ffa6b00b011a3745fd71106364d948a4fb048752c7bf954d",
        "0x5c0bdca0f6180c2b3cfca224a853cb9504c16b0a16f1025be8746e54335cf01",
        "0x7c1fbdcf0da9f44c6ee49a7cc2da7bfb5aae7fe8405a3fd42105c0a9d864a36",
        "0x587e32ddf511d3dd04193d0af898e18e80cae410ba411400e6185c162635419",
        "0xe1314b65854a3e4a87ffd44299dfa1fd5ec35c83cedad436204e7a12c8bd13",
        "0x22bd9975e69ab780c1bd874c99fb102d337e90d3a905eac19ce54c5d1b6bbd1",
        "0x67c3e65dd1624c47dce264322e2e6b2797d096fa76248f11e2182fe9a99f5f2",
        "0x38958ba48451e0157ffab3225716567beac30b44df4db2a251e743cbb93af49",
        "0x3bc1a9f0df58b8c03d1535e3b02c4b4a646ef22b21ef6d47241e7f781e57ce0",
        "0x77e2e9cca0a2415553be66e6ebd9393570c3ecf3426546e6944e74774010e03",
        "0x3561aa6ed23bb17fac27de9a4e314d768f5ea05a033bbcb1de2cff9ae90ab6",
        "0x7f90255cc310f54635400a0fc3ad5d4dcd9afb685485297d828f04cb9c29fcb",
    ]
    .iter()
    .map(|s| Felt::from_hex_unchecked(s))
    .collect()
}
