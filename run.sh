cargo clean 
rm -rf keypairs/
rm -rf test-ledger/
cd programs/verifier
RUSTFLAGS="-Cllvm-args=--inline-threshold=150" cargo build-sbf
cd ../..
cargo run --example eval_composition_polynomial