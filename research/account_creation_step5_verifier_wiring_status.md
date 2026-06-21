# Step #5 — account-creation STARK verifier wiring (Option A) — STATUS

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting implementation-status note, not the top-level project
> tracker. Top-level status lives in `../TRACKER.md`.

Date: 2026-06-18. Branch: `deployed-mask-prf-swap`. Architecture chosen by James:
**Option A (separate verifier crate) with re-export discipline.**

## What was built (verify side — DONE, isolation-tested)

1. **New crate `tools/descender-account-verifier`** — the single edge through which
   the Plonky3 STARK verifier enters the deployed path.
   - **Re-export discipline:** re-exports `descender-account-stark`'s pinned config
     (`PROD_LOG_BLOWUP`, `PROD_NUM_QUERIES`, `PROD_QUERY_POW_BITS`,
     `PROD_SOUNDNESS_BITS`) verbatim — never re-declares params, so verifier cannot
     drift from prover.
   - **Transport bundle:** `pack_bundle` / `unpack_bundle` carry an
     `(AccountCreationPublic, Proof)` blob opaquely (public statement packed via the
     field canonical-u32 API; `Proof` via bincode). The deployed crate never
     inspects it — only this crate (which has Plonky3) does.
   - **`verify_deployed_account_creation(bundle, anchors)`:** runs the STARK
     relation verifier, THEN binds the verified statement to the deployed leaf's
     plain anchors — direct equalities on account_id / slot_id / state_root /
     lifecycle / verifier_version / masked_projection[6], plus the role-root export
     bridge.
   - **`role_root_export_digest`** (`sha256(DS ‖ role_root field bytes)`): the one
     named seam (the dossier's "verifier-exporter bridge"). The producer MUST set
     the deployed `native_role_root_commitment` via this exact function; the
     verifier enforces it. Re-exported so the two sides cannot drift.
   - **Tests (`tests/deployed_binding.rs`, both PASS):** one real single-instance
     prove → pack → verify accepts; every tampered anchor (account_id, slot_id,
     state_root, lifecycle, verifier_version, masked_projection, role-root
     commitment) is rejected with `AnchorMismatch`; a corrupted bundle never
     verifies.

2. **parameter-search wiring (feature-gated, OFF by default):**
   - `Cargo.toml`: optional dep + feature `account-creation-stark-verifier =
     ["dep:descender-account-verifier"]`.
   - `verify_path2_direct_creation_account_stark_payload_boundary(...)`
     (`#[cfg(feature = "account-creation-stark-verifier")]`): enforces the
     proof-family-agnostic view↔leaf bindings (leaf-context digest, creation-proof
     binding, native role-root, aggregation context, slot_id), extracts plain
     anchors (NO Plonky3 type crosses the boundary), and calls
     `verify_deployed_account_creation`.

## Verified

- **Default build: 0 `p3-*` crates** — the deployed signer stays Plonky3-free
  (`cargo tree -e no-dev | grep -c p3-` == 0).
- **Feature build: 96 `p3-*` crates** — Plonky3 enters ONLY through the verifier
  crate, ONLY under the feature.
- Both builds compile clean (one pre-existing unrelated dead_code warning).
- Verifier crate: `cargo test --release` 2/2 pass (~1.5 s; one prove, memory-light).

## Producer migration (2026-06-18, "producer migration only, keep old path" — James)

Built the feature-gated producer + a decoupled payload so the LIVE admission path
uses the sound STARK (HIGH-2 closed in practice); the transparent-descent code is
LEFT IN PLACE (kept, per James) for separate-review cleanup.

- **`PATH2_ACCOUNT_STARK_VERIFIER_VERSION: u32 = 1`** — the raw verifier version
  the STARK binds in-circuit (deployed leaf carries only the digest); canonical
  source on both producer and verify sides.
- **`Path2AccountStarkCreationPayloadV1`** (plain data, ungated, `Serialize`):
  leaf_context_digest, creation_proof_binding_digest, native_role_root_commitment
  (= export bridge), aggregation_context_digest, slot_id, masked_projection[6],
  account_stark_bundle. Decoupled from the transparent-descent types entirely.
- **`path2_account_stark_prove_creation_v1`** (feature-gated PRODUCER): recovers the
  account witness (raw projection + checked-core syndrome) from
  `reviewed_fq12_masked_state18_packed_witness_opening` — whose projection masks
  come from the SAME Poseidon2 PRF the STARK uses in-circuit (the mask swap), so the
  proven `masked_projection` equals the deployed opening's — derives the STARK C1/C6
  nonces from the blinding seed, proves via `descender-account-stark`, packs the
  bundle, and sets `native_role_root_commitment = role_root_export_digest(role_root)`.
- **`verify_path2_direct_creation_account_stark_payload_boundary`** rewritten to take
  the new payload, bind it to the leaf (context digest, binding digest, aggregation,
  slot), and run the STARK verify + anchor binding. The redundant view↔binding
  native-role-root check was dropped — the role root is now bound to the proven
  statement via the export bridge (stronger).
- Verified: default build still 0 `p3-*`; feature build compiles (producer +
  boundary). End-to-end test `path2_account_stark_creation_roundtrip_and_binds`
  (feature-gated, inline test module): **PASS** (0.51 s, one prove) — honest payload
  verifies; tampered masked_projection / role-root export / leaf swap all rejected.
  This EMPIRICALLY confirms the mask-swap consistency end-to-end: the boundary binds
  the deployed opening's `masked_projection` against the PROVEN
  `public.masked_projection`, and they agree (else verify would reject).

## Production parity + fail-closed gate (added + tested same run)

- The boundary now runs the standard deployed-frame checks (parity with the other
  creation paths): `verify_path2_prevalidated_slot_creation_payload_bindings`,
  binding-digest canonicity, and the binding↔leaf field block.
- `verify_path2_direct_creation_account_stark_payload_boundary_for_production`
  mirrors the transparent-descent `_for_production` wrapper: full verify THEN a
  fail-closed gate `path2_account_stark_production_verification_enabled()` → `false`
  (external-review-gated, Step #8; do NOT flip autonomously).
- Re-verified after these additions: default build 0 `p3-*`; feature end-to-end test
  PASS (0.53 s).

## Residual / "the damage" (remaining after producer migration)

- The live admission CALLER must be switched to call the new producer +
  `..._for_production` (the new path exists, is self-consistent, and stays
  fail-closed; flipping the actual call site is the last wiring touch — and is moot
  for liveness while the gate is closed).
- Transparent-descent path kept (not deleted) — deletion deferred to separate review.
- Original residual notes below.

## (original residual — pre-producer-migration)

The verify side is wired and sound in isolation. The end-to-end cutover requires a
**producer-side migration**, which is ALSO the HIGH-2 fix:

- **KEY INSIGHT:** the path this replaces —
  `verify_path2_direct_creation_transparent_descent_payload_boundary_with_proof_bytes`'s
  full-hidden-relation backend — IS the HIGH-2 reopened in-clear-blind leak path.
  Migrating the producer to emit the sound `descender-account-stark` bundle and
  retiring the transparent-descent suite **closes HIGH-2**. Step #5 completion and
  the HIGH-2 fix are the same work.
- The admission-proof PRODUCER must: (a) generate a `descender-account-stark` proof
  and emit `pack_bundle(public, proof)` as the deployed `proof_bytes`; (b) set the
  leaf/view `native_role_root_commitment = role_root_export_digest(&public.role_root)`;
  (c) carry the raw `verifier_version` (the deployed leaf currently stores only
  `verifier_version_digest`; the new boundary takes raw u32 as a parameter).
- Until then the new boundary is present + tested but not exercised by the live
  admission flow; the old (broken) transparent-descent path is still present
  alongside it (additive, reversible).

## Gate

Unchanged: production stays fail-closed; the gate flip (Step #8) is
external-review-gated and NOT autonomous.
