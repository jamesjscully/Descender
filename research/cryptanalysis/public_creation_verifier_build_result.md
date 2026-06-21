# BUILD RESULT — Real Public Creation-Proof Verifier (GAP A / #22 / #4 prevalidationDataAvailability)

> MIGRATION STATUS: `NEGATIVE_CONTROL`.
> This is a historical build record for a public masked-state18 verifier that is
> now hard-gated for leaking `pad_seed`. Keep it as fail-closed evidence, not as
> a current deployable verifier claim.

**Date:** 2026-06-10 · **n=32, q=12289, Fq12**
**Status update (2026-06-11):** HISTORICAL / SUPERSEDED. The masked-state18 public verifier was
hard-gated after HIGH-2 confirmed that `Path2MaskedState18CreationProofV1` serializes `pad_seed` in
clear. `verify_path2_direct_creation_masked_state18_proof_v1` and
`verify_path2_direct_creation_proof_payload_with_masked_state18_proof_v1` now fail closed with
`PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR`; the equivalence checker described below survives
only as the internal debug helper `verify_path2_direct_creation_masked_state18_proof_debug_relation_v1`.
**Target file:** `tools/parameter-search/src/production_reference.rs`
**Predecessor:** `creation_time_relation_validity_VERDICT.md` (relation arm SOUND; residual = data-availability)

---

## SUMMARY

A real (non-toy) public, harness-free account-creation-proof verifier was originally built and
tested for functional equivalence to the harness-replay relation. That acceptance surface is now
disabled because it is non-hiding. Keep this document as the historical build record for the relation
checker, not as a current deployable verifier claim.

| Item | Status |
|---|---|
| Real (non-toy) public verifier built | HISTORICAL; now hard-gated |
| Toy commitment de-toyed | YES — real SHA-256/masked_state18 commitment replaces `P2Q1` toy |
| New proof kind added | YES — `PATH2_DIRECT_CREATION_PROOF_KIND_MASKED_STATE18 = 3` |
| Wired into the dispatcher | YES — handled kind; now returns `PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR` |
| Equivalence to harness-replay relation | TESTED on internal debug relation checker |
| Negative controls reject | YES — binding-field + claim + pad-seed mutations all reject |
| Production stays fail-closed (preflight test) | YES — `deployment_preflight_fails_closed…` passes |
| Source-marker gates / 314-byte fixture / KAT | ALL GREEN |
| #4 prevalidationDataAvailability | CLOSED only as a historical/debug relation check; not an accepting public path |

---

## WHAT WAS BUILT

### 1. Real verifier — `verify_path2_direct_creation_masked_state18_proof_v1`

It replays the HARNESS's own masked-state18 packed-witness opening derivation
(`reviewed_fq12_masked_state18_packed_witness_opening`) over public payload data. Steps:

1. `verify_path2_direct_creation_proof_payload_common` — all frame / leaf-context (9 fields) /
   certificate-digest / payload-digest bindings + proof-kind check.
2. Bind the revealed `pad_seed` to `leaf.pad_seed_digest` via `reviewed_fq12_pad_seed_opening_digest`.
3. Replay `reviewed_fq12_masked_state18_packed_witness_opening(packed_witness_commitment, pad_seed,
   slot_id)` — byte-for-byte the harness relation: derive `masked_projection` from the leaf-fixed
   commitment residues, subtract `PRF(pad_seed, slot_id)` masks to obtain `account_projection`,
   repack and assert `repacked == commitment` (fail-closed at the opening function, `:6556`).
4. Assert `opening.account_projection == proof.account_projection` (relation-derived == public claim).
5. **De-toy commitment binding:** assert `binding.pad_source_commitment` and
   `binding.projection_relation_commitment` equal the **real SHA-256/masked_state18 commitments**
   (`path2_masked_state18_commitment_bytes`) over `masked_projection` / `account_projection`. This
   replaces the toy `P2Q1` 7-byte commitment scheme (`path2_toy_q_linear_commitment_*`).
6. Per-lane mask consistency: `masked_projection[i] == (account_projection[i] + mask[i]) mod q`.

The ONLY change relative to the toy verifier (`verify_path2_direct_creation_toy_q_linear_proof_v1`)
is exactly the commitment scheme: the masked-state18 verifier binds via the real SHA-256/masked_state18
commitment and replays the real harness opening relation, not the toy q-linear Sigma over `P2Q1`.

### 2. Wiring (gated / labeled; default stays fail-closed)

- Bindings-only dispatcher `verify_path2_direct_creation_proof_payload_v1` (`:26475`) now RECOGNIZES
  `PATH2_DIRECT_CREATION_PROOF_KIND_MASKED_STATE18` as a **handled** kind (it no longer returns the
  generic "payload kind is unsupported"). Because it has no proof body, it returns a distinct error
  routing the caller to the proof-carrying dispatcher.
- Proof-carrying dispatcher
  `verify_path2_direct_creation_proof_payload_with_masked_state18_proof_v1` is now hard-gated and
  returns `PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR` for this non-hiding proof kind.

### 3. New public API

- const `PATH2_DIRECT_CREATION_PROOF_KIND_MASKED_STATE18: u16 = 3`
- const `PATH2_MASKED_STATE18_COMMITMENT_DOMAIN`, `…ROLE_PAD_SOURCE`, `…ROLE_PROJECTION_RELATION`
- struct `Path2MaskedState18CreationProofV1 { pad_seed, account_projection }`
- fn `path2_masked_state18_commitment_bytes(role, lanes) -> [u8; 32]` (real SHA-256 commitment)
- fn `verify_path2_direct_creation_masked_state18_proof_v1(...)`
- fn `verify_path2_direct_creation_proof_payload_with_masked_state18_proof_v1(...)`

---

## EQUIVALENCE EVIDENCE

`path2_masked_state18_verifier_is_equivalent_to_harness_relation_replay` (test): drives BOTH the new
public verifier and `verify_path2_prevalidated_slot_creation_relation_replay` from the SAME fixture
(`path2_creation_relation_replay_fixture`, the fixture the harness-replay tests use).

- **Positive direction:** valid creation evidence → public verifier ACCEPTS *and* harness-replay
  ACCEPTS the same evidence.
- **Negative direction:** a relation-invalidating mutation (wrong `pad_seed`) → public verifier
  REJECTS *and* harness-replay REJECTS the same evidence.

This is exact functional equivalence on the masked-state18 relation because the public verifier calls
the harness's own opening-derivation function: a valid public proof is accepted iff that derivation
succeeds and matches the claim, which is precisely the relation the harness-replay path enforces.

### Negative-control matrix (`account_creation_proof_mechanism_decision.md:169-191`)

`path2_masked_state18_creation_proof_negative_controls_reject` asserts rejection for: account-id,
slot-id, opening-commitment mutations (binding leaf-context); a `packed_witness_commitment` carrying a
different projection (relation-derived ≠ claim); pad-source and projection-relation commitment
mutations (real SHA-256 commitment); `pad_seed` mutation (pad-seed-digest binding); claimed
`account_projection` lane mutation (relation mismatch); and `proof_kind` mutation. All reject with the
expected error.

---

## PRODUCTION STAYS FAIL-CLOSED (confirmed)

- `production_deployment_preflight` is UNCHANGED. It unconditionally pushes
  `PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS`, `…CONSTANT_TIME`, `…STORAGE`,
  `…EXTERNAL_REVIEW`, so `deployment_ready == false` regardless of the new verifier.
- Both blocker constants `PRODUCTION_DEPLOYMENT_BLOCKER_SELECTED_VERIFIER` and
  `…HIDDEN_SOURCE_PROOF_SEMANTICS` are intact and not weakened.
- Test `deployment_preflight_fails_closed_until_selected_verifier_and_reviews_land` PASSES.
- Test `path2_masked_state18_path_does_not_flip_production_preflight` asserts `!deployment_ready` and
  the presence of the hidden-source + external-review blockers after the new path exists.

The new path is verifier LOGIC that accepts a valid public proof; the DEPLOYMENT gate stays closed
pending external review, exactly as the D3 staging ladder requires.

---

## GATE / KAT / TEST RESULTS

- `cargo build` (descender-parameter-search): clean.
- New tests (5): all pass — accepts-valid; bindings-only-dispatcher routing; equivalence (IFF);
  negative controls; preflight-not-flipped.
- Gate tests: `deployment_preflight_fails_closed…` PASS; `selected_projection_valid_314_byte_fixture_accepts`
  (renamed 314-byte fixture) PASS.
- KAT: `--kat-check` reproduces byte-for-byte (kat_v0 corpus).
- Full `cargo test` for the package: PASS (see commit-prep run).
- Source-inspection marker gates: untouched — no gated function was renamed; the new functions are
  additive. The preflight source-present flags all remain true (asserted by the passing preflight test).

---

## RESIDUAL OBLIGATION (named, not a soundness break)

The public proof carries `pad_seed` in the clear so the verifier can recompute the PRF projection
masks and replay the harness opening. This is the honest data-availability mechanism: it makes the
relation checkable from public proof data, and it binds `pad_seed` to the leaf's `pad_seed_digest`.
It does NOT add a hiding/zero-knowledge guarantee over `pad_seed`. The masked-state18 relation
equivalence is exact and tested; a ZK/hiding public proof that withholds `pad_seed` (e.g. the
transparent-descent kind) remains future work and is still fail-closed. The 52-inactive-column
verifier-exporter bridge assumption from the relation-validity VERDICT is unchanged by this build.

**DO NOT git commit — left for maintainer review.**
