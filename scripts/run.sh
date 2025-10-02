cargo clean 
rm -rf keypairs/
rm -rf test-ledger/
cd programs/verifier4
cargo build-sbf
cd ../..
cargo run --example verify