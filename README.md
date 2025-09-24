# Solana Verifier

## Prerequisites

Before building or running the project, ensure you have the following dependencies installed:

- **System packages:**
  - `libudev-dev`
  - `build-essential`
  - `pkg-config`
  - `llvm`
  - `libclang-dev`
  - `protobuf-compiler`
  - `libssl-dev`

- **Solana CLI:**
  - Install using:

    ```bash
    sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
    ```

After installation, ensure Solana CLI binaries are in your PATH:

```bash
echo "$HOME/.local/share/solana/install/active_release/bin" >> ~/.bashrc
source ~/.bashrc
```

This project demonstrates how to build, deploy, and interact with Solana programs using Rust, with a focus on on-chain verification of various computational tasks.

## Project Structure

- `client/`: A Rust client application that deploys and interacts with programs
- `programs/`: Solana programs written in Rust
  - `verifier1/`: Core verification program responsible for first stage of verification - validate-public-input, get-hash, stark-commit
  - `verifier2/`: Core verification program responsible for second stage of verification - stark-verify
  - `utils/`: Shared utilities for Solana programs
- `tasks/`: Task implementations for the verifier
  - `felt/`: Field element type
  - `get_hash/`: Hashing utilites
  - `pedersen/`: Pedersen hash implementation
  - `poseidon/`: Poseidon hash implementation
  - `stark_commit/`: STARK commitment logic
  - `stark_verify/`: STARK verification logic
  - `types/`: Common types for tasks, proof and etc
  - `validate_public_input/`: Public input validation
  - `verify_1/`: First stage verification logic, combining all requiered steps
  - `verify_2/`: Second stage verification logic,
  - `verify_public_input/`: Public input verification

## Manual Setup

1. Start a Solana test validator:

  ```bash
  solana-test-validator
  ```

2. Build the Solana programs:

  ```bash
  ./scripts/compile.sh
  ```
  
3. Build and run the verification example:

  ```bash
  cargo run --example full_flow
  ```

## Client Features

The client demonstrates how to:

- Create and manage Solana keypairs
- Request airdrops of SOL for testing
- Deploy Solana programs programmatically using the Solana SDK
- Create program accounts
- Send transactions to interact with the program
- Read account data from the blockchain
