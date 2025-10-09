# Account Ownership Transfer Test

## Changes

### 1. New Instruction in Verifier1: `TransferOwnership`

Added an instruction to enable account ownership transfer:

**Modified Files:**
- `programs/verifier1/src/instruction.rs` - added `TransferOwnership`
- `programs/verifier1/src/processor.rs` - implementation of `process_transfer_ownership()`

**How it works:**
```rust
// Step 1: Resize to 0 bytes (required before transfer)
target_account.resize(0)?;

// Step 2: Assign to new owner
target_account.assign(new_owner_account.key);

// Step 3: Resize back to original size
target_account.resize(original_size)?;
```

**Note:** After transfer, the account data is zeroed! You need to copy it back.

### 2. Added `TestExecute` to Verifier2

**Modified Files:**
- `programs/verifier2/src/instruction.rs` - added `TestExecute`
- `programs/verifier2/src/processor.rs` - implementation of `process_test_execute()`

This instruction simply increments `front_index` - used to test if the program can modify the account.

### 3. Extended Test: `test_two_accounts.rs`

The test now validates the full ping-pong flow with ownership transfer:

**Step 1:** Verifier1 modifies account1 (owner: verifier1)
- Increments `front_index`

**Step 2:** Verifier2 copies from account1 → account2
- Account1: readonly, owner verifier1
- Account2: writable, owner verifier2

**Step 3:** Transfer ownership account1: verifier1 → verifier2 ✨ **NEW**
- Verifier1 executes `TransferOwnership`
- Data is zeroed during transfer
- Verify new owner

**Step 4:** Verifier2 modifies account1 (new owner: verifier2) ✨ **NEW**
- Verifies that verifier2 can now write to account1
- Increments `front_index`

## How to Run the Test

### 1. Compile programs
```bash
cargo build-sbf --manifest-path programs/verifier1/Cargo.toml
cargo build-sbf --manifest-path programs/verifier2/Cargo.toml
```

### 2. Start local validator (in separate terminal)
```bash
solana-test-validator
```

### 3. Run the test
```bash
cargo run --example test_two_accounts
```

## Expected Output

```
=== Two Accounts Test: Verifier1 → Verifier2 ===

Account space: 904176 bytes

Creating account1: <pubkey> (owner: verifier1)
✓ Account1 created (owner: verifier1)

Creating account2: <pubkey> (owner: verifier2)
✓ Account2 created (owner: verifier2)

✓ Account1 initialized with test data

=== STEP 1: Verifier1 modifies account1 ===
✓ Verifier1 successfully modified account1!
  Account1 front_index after verifier1: 65537

=== STEP 2: Verifier2 copies account1 → account2 ===
✓ Verifier2 successfully copied account1 → account2!
  Account2 front_index after copy: 65537
✓ Copy test passed!

=== STEP 3: Transfer ownership account1: verifier1 → verifier2 ===
Account1 current owner: <verifier1_id>
Verifier1 program ID: <verifier1_id>
✓ Ownership transferred successfully!
Account1 new owner: <verifier2_id>
✓ Ownership verified: account1 now owned by verifier2!

=== STEP 4: Verifier2 modifies account1 (new owner) ===
✓ Verifier2 successfully modified account1!
  Account1 front_index after verifier2 modify: 1
✓ Account1 was successfully modified by new owner (verifier2)!

=== TEST SUMMARY ===
✓ Step 1: Verifier1 modified account1 (original owner)
✓ Step 2: Verifier2 copied from account1 to account2
✓ Step 3: Ownership transferred from verifier1 to verifier2
✓ Step 4: Verifier2 modified account1 (new owner)

✓✓✓ ALL TESTS PASSED! ✓✓✓
```

## Implications for Ping-Pong Architecture

This test confirms that the ownership transfer mechanism works! We can now implement the full flow for 4 verifiers:

1. **Verifier1** → modifies account1
2. **Verifier2** → copies from account1, modifies account2
3. **Transfer** account1: verifier1 → verifier3
4. **Verifier3** → copies from account2, modifies account1
5. **Transfer** account2: verifier2 → verifier4
6. **Verifier4** → copies from account1, modifies account2 (final result)

## Technical Notes

### Data Zeroing
After `resize(0)` and `resize(original_size)`, data is zeroed! Therefore, after transfer you need to:
- Either copy data back from the second account
- Or reinitialize the data structure

### Rent
Transfer preserves lamports (rent), so you don't need to pay for rent exemption again.

### Security
- Only the current owner can execute transfer
- Program verifies ownership before transfer
- After transfer, the old owner loses write access

## Status in PING_PONG_ARCHITECTURE.md

Update checklist:
- [x] CopyFromAccount instruction in all verifiers
- [x] Test for verifier1 → verifier2 copy
- [x] Ownership transfer mechanism ✅ **DONE**
- [x] Test for ownership transfer ✅ **DONE**
- [ ] Full 4-verifier test
- [ ] Integration with existing verify_universal flow

