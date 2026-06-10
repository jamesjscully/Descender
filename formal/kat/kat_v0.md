# KAT v0 — index (human-readable)

This is a generated index of [`kat_v0.json`](kat_v0.json), the frozen
Known-Answer-Test corpus that every Descender implementation must reproduce
byte-for-byte. **Do not hand-edit either file.** Regenerate both with:

```
cd tools/parameter-search
cargo run --bin signature_q12289_local_delta_stateful -- --kat-generate
```

and check reproduction (this is what CI runs) with:

```
cargo run --bin signature_q12289_local_delta_stateful -- --kat-check
```

## Provenance

- **Profile:** `Descender.Signature.q12289.local_delta.stateful.v1`
- **Producing pipeline:** `production_keygen` → `production_sign` →
  `production_verify` in `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`,
  which wraps `production_reference.rs` (`verify_with_account_binding` /
  `complete_verified_signature`). Keygen/sign/verify are fully deterministic
  (SHA256-seeded, no randomness), so the corpus is exactly reproducible.
- **Producing-version digest:** `3f2d4c5f370f22909fee6a0c8f2a5cba1d78a5a8cef9672d6625b59db1e9982c`
  — SHA256 over the structural constants plus a freshly recomputed positive
  signature. Any drift in the reference logic changes this digest.
- **q = 12289**, public key = 172 bytes, transaction signature = **260 bytes**.

> Note: this corpus pins the *current* deployable reference path (the
> 260-byte linear-sigma target transaction signature, which still wraps the
> legacy proof). It is the byte-for-byte anchor; it is not itself a soundness
> claim about that path.

## Signature byte layout (260 bytes)

| field | offset | length |
|---|---:|---:|
| slot_id | 0 | 4 |
| challenge | 4 | 27 |
| response | 31 | 52 |
| openings | 83 | 104 |
| linear_sigma_proof (projection carrier) | 187 | 73 |

## Positive vectors (3) — expect ACCEPT

| name | keygen slots | slot id | description |
|---|---:|---:|---|
| `pos_slot0_2slot` | 2 | 0 | keygen(2) → sign slot 0 → verify accepts. |
| `pos_slot1_2slot` | 2 | 1 | keygen(2) → sign slot 1 → verify accepts (distinct slot). |
| `pos_slot0_4slot` | 4 | 0 | keygen(4) → sign slot 0 → verify accepts (distinct account size). |

## Negative vectors (8) — expect REJECT

Each `expected_reason` below was captured from the real verifier, not guessed.
There are **7 distinct reject reasons** (≥ 4 required).

| name | expected reason | what it probes |
|---|---|---|
| `neg_noncanonical_truncated` | `signature has 259 bytes, expected 260` | non-canonical encoding (wrong length) |
| `neg_mutated_challenge` | `challenge mismatch` | mutated challenge field |
| `neg_mutated_response` | `local fiber-state mismatch in round 0` | mutated active response |
| `neg_mutated_carrier_commitment` | `account randomizer commitment mismatch` | carrier proof-commitment tamper (account-binding surface) |
| `neg_mutated_carrier_masked_witness` | `slot digest does not match public proof q-syndrome response` | carrier masked-witness tamper (sigma surface) |
| `neg_replayed_slot` | `slot is already consumed` | replayed slot (genuine sig re-presented after consumption) |
| `neg_stale_account_root` | `account randomizer commitment mismatch` | stale account root (slot randomizer mutated) |
| `neg_public_representative_forgery_slot_oob` | `slot id is out of range` | public-representative forgery probe (out-of-range slot claim) |
