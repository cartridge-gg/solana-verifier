cargo clean 
rm -rf keypairs/
rm -rf test-ledger/
cd programs/verifier1
cargo build-sbf
cd ../verifier2
cargo build-sbf
cd ../verifier3
cargo build-sbf
cd ../verifier4
cargo build-sbf
cd ../..
cargo run --example verify -- --proof example_proof/fibonnaci_stone6_keccak_160_lsb.json