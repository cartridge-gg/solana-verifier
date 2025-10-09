# Test Transferu Własności Konta (Ownership Transfer)

## Zmiany

### 1. Nowa Instrukcja w Verifier1: `TransferOwnership`

Dodano instrukcję umożliwiającą zmianę właściciela konta:

**Pliki zmodyfikowane:**
- `programs/verifier1/src/instruction.rs` - dodano `TransferOwnership`
- `programs/verifier1/src/processor.rs` - implementacja `process_transfer_ownership()`

**Jak działa:**
```rust
// Krok 1: Resize do 0 bajtów (wymagane przed transferem)
target_account.resize(0)?;

// Krok 2: Przypisz do nowego właściciela
target_account.assign(new_owner_account.key);

// Krok 3: Resize z powrotem do oryginalnego rozmiaru
target_account.resize(original_size)?;
```

**Uwaga:** Po transferze dane w koncie są wyzerowane! Trzeba je skopiować z powrotem.

### 2. Dodano `TestExecute` do Verifier2

**Pliki zmodyfikowane:**
- `programs/verifier2/src/instruction.rs` - dodano `TestExecute`
- `programs/verifier2/src/processor.rs` - implementacja `process_test_execute()`

Ta instrukcja po prostu inkrementuje `front_index` - służy do testowania czy program może modyfikować konto.

### 3. Rozszerzony Test: `test_two_accounts.rs`

Test teraz sprawdza pełny przepływ ping-pong z transferem własności:

**Krok 1:** Verifier1 modyfikuje account1 (właściciel: verifier1)
- Inkrementuje `front_index`

**Krok 2:** Verifier2 kopiuje z account1 → account2
- Account1: readonly, właściciel verifier1
- Account2: writable, właściciel verifier2

**Krok 3:** Transfer własności account1: verifier1 → verifier2 ✨ **NOWE**
- Verifier1 wykonuje `TransferOwnership`
- Dane są wyzerowane podczas transferu
- Weryfikacja nowego właściciela

**Krok 4:** Verifier2 modyfikuje account1 (nowy właściciel: verifier2) ✨ **NOWE**
- Weryfikuje że verifier2 może teraz pisać do account1
- Inkrementuje `front_index`

## Jak Uruchomić Test

### 1. Skompiluj programy
```bash
cargo build-sbf --manifest-path programs/verifier1/Cargo.toml
cargo build-sbf --manifest-path programs/verifier2/Cargo.toml
```

### 2. Uruchom lokalny validator (w osobnym terminalu)
```bash
solana-test-validator
```

### 3. Uruchom test
```bash
cargo run --example test_two_accounts
```

## Oczekiwany Output

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

## Implikacje dla Architektury Ping-Pong

Ten test potwierdza, że mechanizm transferu własności działa! Możemy teraz zaimplementować pełny przepływ dla 4 weryfikatorów:

1. **Verifier1** → modyfikuje account1
2. **Verifier2** → kopiuje z account1, modyfikuje account2
3. **Transfer** account1: verifier1 → verifier3
4. **Verifier3** → kopiuje z account2, modyfikuje account1
5. **Transfer** account2: verifier2 → verifier4
6. **Verifier4** → kopiuje z account1, modyfikuje account2 (wynik końcowy)

## Uwagi Techniczne

### Wyzerowanie Danych
Po `resize(0)` i `resize(original_size)` dane są wyzerowane! Dlatego po transferze trzeba:
- Albo skopiować dane z powrotem z drugiego konta
- Albo reinicjalizować strukturę danych

### Rent
Transfer zachowuje lamports (rent), więc nie trzeba ponownie płacić za rent exemption.

### Bezpieczeństwo
- Tylko aktualny właściciel może wykonać transfer
- Program weryfikuje ownership przed transferem
- Po transferze stary właściciel traci dostęp do zapisu

## Status w PING_PONG_ARCHITECTURE.md

Zaktualizuj checklist:
- [x] CopyFromAccount instruction in all verifiers
- [x] Test for verifier1 → verifier2 copy
- [x] Ownership transfer mechanism ✅ **DONE**
- [x] Test for ownership transfer ✅ **DONE**
- [ ] Full 4-verifier test
- [ ] Integration with existing verify_universal flow

