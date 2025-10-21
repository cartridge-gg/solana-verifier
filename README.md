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

## ⚠️ Project Status

**This verifier is a working proof of concept.** The current implementation includes:

- ✅ **Full end-to-end verification flow working**
- ✅ Stack overflow fixes and compute unit optimizations
- ✅ Ping-pong architecture for data sharing between verifiers
- ✅ All 4 verifiers tested and passing
- ✅ Integration between verifiers functional
- ⚠️ **Production hardening needed** (error handling, edge cases)
- ⚠️ **Performance optimizations for larger proofs**
- ⚠️ **Security audit recommended before production use**

**Note**: This is a **proof of concept** that demonstrates the complete verification pipeline. While functional, it requires additional hardening and security review before production deployment.

### Required Improvements for Production

#### Security & Error Handling
- **Uncomment safety assertions** - Many critical safety checks are currently commented out for development
- **Remove unwrap() calls** - Replace with proper error handling and graceful failure modes
- **Add bounds checking** - Ensure array and FunVec access never goes out of bounds
- **Validate input data** - Add comprehensive validation for all external inputs

#### Scalability & Performance
- **Optimize FunVec/array sizes** - Current fixed sizes may not handle larger proofs
- **Implement chunked processing** - Replace single-transaction loops with chunked processing:
  ```rust
  // Instead of: for item in large_data { process(item) }
  // Use: process_in_chunks(large_data, chunk_size)
  ```
- **Dynamic sizing** - Allow FunVec and arrays to scale based on proof size
- **Transaction batching** - Split large operations across multiple transactions

#### Code Quality
- **Remove development shortcuts** - Clean up temporary workarounds
- **Add comprehensive logging** - Better debugging and monitoring capabilities
- **Improve documentation** - Add detailed comments for complex algorithms
- **Memory optimization** - Reduce stack allocations and improve memory efficiency

## Project Structure

- `client/`: A Rust client application that deploys and interacts with programs
- `programs/`: Solana programs written in Rust
  - `verifier1/`: Core verification program responsible for first stage of verification - validate-public-input, get-hash, stark-commit
  - `verifier2/`: Core verification program responsible for second stage of verification - stark-verify
  - `verifier3/`: Third stage verification program - stark-verify-verification
  - `verifier4/`: Fourth stage verification program - stark-verify-fri
  - `utils/`: Shared utilities for Solana programs
- `tasks/`: Task implementations for the verifier
  - `felt/`: Field element type
  - `get_hash/`: Hashing utilites
  - `pedersen/`: Pedersen hash implementation
  - `poseidon/`: Poseidon hash implementation
  - `stark_commit/`: STARK commitment logic
  - `stark_verify/`: STARK verification logic
  - `stark_verify_verification/`: STARK verification verification logic
  - `stark_verify_fri/`: STARK verification FRI logic
  - `types/`: Common types for tasks, proof and etc
  - `validate_public_input/`: Public input validation
  - `verify_1/`: First stage verification logic, combining all requiered steps
  - `verify_2/`: Second stage verification logic
  - `verify_3/`: Third stage verification logic
  - `verify_4/`: Fourth stage verification logic
  - `verify_public_input/`: Public input verification

## Architecture: Ping-Pong Pattern

This project uses a **two-account ping-pong architecture** to handle large data (904KB) across 4 verifier programs, since Solana only allows a program to modify accounts it owns.

### Problem
4 verifier programs need to share and modify the same large data (904176 bytes), but Solana only allows a program to modify accounts it owns.

### Solution: Two-Account Ping-Pong

Use 2 accounts that alternate ownership between verifiers:

```
account1 ←→ account2
  904KB      904KB
```

### Flow

#### Initialization
```
Client creates:
  - account1 (owner: verifier1, size: 904176 bytes)
  - account2 (owner: verifier2, size: 904176 bytes)

Client initializes account1 with initial data
```

#### Stage 1: Verifier1
```
Owner: account1 = verifier1, account2 = verifier2

verifier1.Execute(account1)
  → Computes in account1
  → Results stored in account1
```

#### Stage 2: Verifier2
```
Owner: account1 = verifier1, account2 = verifier2

verifier2.CopyFromAccount(source: account1, dest: account2)
  → Reads account1 (readonly, OK even though owner is verifier1)
  → Copies to account2 (writable, OK because owner is verifier2)

verifier2.Execute(account2)
  → Computes in account2
  → Results stored in account2
```

#### Stage 3: Verifier3
```
Need to transfer ownership!

Client calls:
  transfer_ownership(account1, new_owner: verifier3)

Owner: account1 = verifier3, account2 = verifier2

verifier3.CopyFromAccount(source: account2, dest: account1)
  → Reads account2 (readonly, OK)
  → Copies to account1 (writable, OK because owner is verifier3)

verifier3.Execute(account1)
  → Computes in account1
  → Results stored in account1
```

#### Stage 4: Verifier4
```
Client calls:
  transfer_ownership(account2, new_owner: verifier4)

Owner: account1 = verifier3, account2 = verifier4

verifier4.CopyFromAccount(source: account1, dest: account2)
  → Reads account1 (readonly, OK)
  → Copies to account2 (writable, OK because owner is verifier4)

verifier4.Execute(account2)
  → Computes in account2
  → FINAL RESULTS in account2
```

### Ownership Transfer

Solana requires accounts to be zeroed before ownership transfer:

```rust
// In verifier program or via System Program:
account.realloc(0, false)?;  // Zero the data
account.assign(&new_owner_program_id)?;  // Transfer ownership
account.realloc(904176, false)?;  // Resize back
```

### Key Benefits

✅ Each verifier can modify its own account  
✅ Each verifier can read from the other account (readonly)  
✅ No PDA or invoke_signed complexity  
✅ Simple ping-pong pattern  

### Implementation Status

- [x] CopyFromAccount instruction in all verifiers
- [x] Test for verifier1 → verifier2 copy
- [x] Ownership transfer mechanism (TransferOwnership instruction in verifier1)
- [x] Test for ownership transfer (test_two_accounts.rs now includes transfer test)
- [x] Full 4-verifier test
- [x] Integration with existing verify_universal flow

## Manual Setup

1. Start a Solana test validator:

  ```bash
  solana-test-validator
  ```

2. Build the Solana programs:

  ```bash
  ./scripts/compile.sh
  ```
  
3. Build and run the verification:

  ```bash
  cargo run deploy
  cargo run verify
  cargo run retrive-funds
  ```

## Client Features

The client demonstrates how to:

- Create and manage Solana keypairs
- Request airdrops of SOL for testing
- Deploy Solana programs programmatically using the Solana SDK
- Create program accounts
- Send transactions to interact with the program
- Read account data from the blockchain
