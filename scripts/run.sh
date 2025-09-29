cargo clean 
rm -rf keypairs/
rm -rf test-ledger/
cd programs/verifier3
cargo build-sbf
cd ../..
cargo run --example eval_oods_boundary_poly_at_points