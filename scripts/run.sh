cargo clean 
rm -rf keypairs/
rm -rf test-ledger/
cd programs/verifier2
cargo build-sbf
cd ../..
cargo run --example vector_decommit