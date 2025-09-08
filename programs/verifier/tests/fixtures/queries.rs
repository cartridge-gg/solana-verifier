use felt::Felt;

/// Values obtained from:
/// https://github.com/iosis-tech/swiftness/blob/main/crates/stark/src/queries.rs#L30-L45
/// (function `queries_to_points`)
/// when running with proof from `example_proof/saya.json`:
///
/// ```bash
/// cargo run --features recursive_with_poseidon,stone6,keccak_160_lsb \
///   --no-default-features -- --proof solana-verifier/example_proof/saya.json
/// ```

pub fn get() -> Vec<Felt> {
    let queries = vec![
        "0xd20990",
        "0x1702a2dc",
        "0x233bfb24",
        "0x2fc8f32e",
        "0x367bcdcb",
        "0x44445cc6",
        "0x4bf4ed93",
        "0x8df252ca",
        "0x97a48b5b",
        "0xafea6443",
        "0xc62f63b8",
        "0xd76e5257",
        "0xecca885b",
        "0xedc42f8b",
        "0xf6821efe",
        "0xf7769c26",
    ];
    queries
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<Felt>>()
}

/// Result obtained from the same source as above.  
pub fn result() -> Vec<Felt> {
    let points = vec![
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
    ];
    points
        .iter()
        .map(|f| Felt::from_hex_unchecked(f))
        .collect::<Vec<Felt>>()
}
