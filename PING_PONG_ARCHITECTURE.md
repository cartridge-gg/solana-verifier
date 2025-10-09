# Ping-Pong Architecture for 4 Verifiers

## Problem
4 verifier programs need to share and modify the same large data (904176 bytes), but Solana only allows a program to modify accounts it owns.

## Solution: Two-Account Ping-Pong

Use 2 accounts that alternate ownership between verifiers.

```
account1 ←→ account2
  904KB      904KB
```

## Flow

### Initialization
```
Client creates:
  - account1 (owner: verifier1, size: 904176 bytes)
  - account2 (owner: verifier2, size: 904176 bytes)

Client initializes account1 with initial data
```

### Stage 1: Verifier1
```
Owner: account1 = verifier1, account2 = verifier2

verifier1.Execute(account1)
  → Computes in account1
  → Results stored in account1
```

### Stage 2: Verifier2
```
Owner: account1 = verifier1, account2 = verifier2

verifier2.CopyFromAccount(source: account1, dest: account2)
  → Reads account1 (readonly, OK even though owner is verifier1)
  → Copies to account2 (writable, OK because owner is verifier2)

verifier2.Execute(account2)
  → Computes in account2
  → Results stored in account2
```

### Stage 3: Verifier3
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

### Stage 4: Verifier4
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

## Ownership Transfer

Solana requires accounts to be zeroed before ownership transfer:

```rust
// In verifier program or via System Program:
account.realloc(0, false)?;  // Zero the data
account.assign(&new_owner_program_id)?;  // Transfer ownership
account.realloc(904176, false)?;  // Resize back
```

## Key Benefits

✅ Each verifier can modify its own account
✅ Each verifier can read from the other account (readonly)
✅ No PDA or invoke_signed complexity
✅ Simple ping-pong pattern

## Implementation Status

- [x] CopyFromAccount instruction in all verifiers
- [x] Test for verifier1 → verifier2 copy
- [x] Ownership transfer mechanism (TransferOwnership instruction in verifier1)
- [x] Test for ownership transfer (test_two_accounts.rs now includes transfer test)
- [ ] Full 4-verifier test
- [ ] Integration with existing verify_universal flow

See OWNERSHIP_TRANSFER_TEST.md for detailed documentation of the transfer mechanism.
