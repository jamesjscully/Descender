# VECTOR 4 — Statistical Leakage / Key-Recovery on the historical masked-state18 path

**Date:** 2026-06-10. RISK-FIRST. Authorized red-team of the maintainer's own scheme.
**Status update (2026-06-11):** ACCEPTANCE SURFACE HARD-GATED. The serialized proof object remains
leaky, and the regression still demonstrates key-free witness recovery from that object, but
`verify_path2_direct_creation_masked_state18_proof_v1` now rejects valid masked-state18 public proofs
with `PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR`.

## VERDICT: BROKEN OBJECT; PUBLIC ACCEPTANCE NOW CLOSED

The claim under attack (prompt): *"the deployable masked design (PRF-fresh projection
mask, pad excluded from serialization) hides the witness."*

**This claim was false for the historical masked-state18 creation proof.** The proof object
(`Path2MaskedState18CreationProofV1`, kind `PATH2_DIRECT_CREATION_PROOF_KIND_MASKED_STATE18 = 3`)
**serializes `pad_seed` in clear**.
`pad_seed` is the master witness secret: it deterministically derives (a) the projection
masks, (b) the per-block secret pad residue table, and (c) the hidden-source projection
values themselves. So any observer of the PUBLIC creation proof recovers the **entire hidden
source / witness with ZERO key material**. The "PRF-fresh projection mask hides the
projection" defense is moot: the mask's own seed is published next to it.

Empirical witness-hiding of the deployable creation proof = **0 bits**, against a claimed
hiding design. This is the SAME failure mode as the known-leaky 260B linear-sigma scaffold
(`reference_sign_with_proof`), now confirmed present on the deployable Path-2 *creation*
proof the maintainer actually built (GAP A, MEMORY 2026-06-10) — i.e., it is NOT confined to
the scaffold.

**Severity calibration (current):** The acceptance surface is now closed in code. This remains a
high-value regression because serializing this object publicly would still leak the witness, but the
public verifier and proof-carrying dispatcher no longer accept it.

## Reproduction (machine-checked)

Added test `redteam_masked_state18_creation_proof_leaks_full_witness_key_free` in
`tools/parameter-search/src/production_reference.rs` (in the `#[cfg(test)] mod tests`, right
after `path2_masked_state18_creation_proof_accepts_valid_public_proof`, ~line 52744).

```
cd tools/parameter-search
cargo test --release --lib redteam_masked_state18_creation_proof_leaks_full_witness_key_free
# -> test result: ok. 1 passed
```

What it proves, step by step, against the historical fixture:

1. `verify_path2_direct_creation_masked_state18_proof_v1(...)` **rejects** the honest proof with
   `PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR`; the internal debug checker still accepts the
   relation so the fixture remains meaningful.
2. `serde_json::to_vec(&proof)` (the public Serialize wire surface) contains the `pad_seed`
   field; the test re-parses the JSON and reconstructs `pad_seed` byte-for-byte (==
   `fixture.pad_seed`). **The attacker reads pad_seed off the wire with no key.**
3. From that public `pad_seed` ALONE:
   - `hidden_source_fallback_pad_prf_projection_values_from_seed(&attacker_pad_seed)` ==
     ground truth → **full hidden-source projection (the witness) recovered key-free**;
   - `reviewed_fq12_pad_residues_from_seed_bounded(&attacker_pad_seed, block)` == ground
     truth for all 16 blocks → **secret pad table recovered key-free**;
   - `reviewed_fq12_packed_witness_state18_projection_masks(&attacker_pad_seed, slot_id)` ==
     ground truth → **the "PRF-fresh" masks recovered key-free**.
4. `reviewed_fq12_masked_state18_packed_witness_opening(commitment, attacker_pad_seed,
   slot_id).account_projection == proof.account_projection` → **the committed masked
   projection is unmasked to the witness with zero key material.**

## Why the prior `deployable_pad_hiding_VERDICT.md` (CONDITIONAL-GAP-FIXABLE) under-called this

That verdict examined two surfaces and concluded "NO inherit":
- the pre-spend `ProductionReviewedFq12StateAssistedOpening` (777-785) — digest-only,
  pad_seed held as a **verifier-side witness argument** (genuinely not serialized);
- the masked-state18 *packed witness opening* — pad disclosure labeled "intentional
  post-spend, charged."

Both observations are correct **for those surfaces**. But the verdict's own residual already
flagged the live gap, and the prompt asked me to attack the **deployable CREATION proof
path** specifically. The built creation proof `Path2MaskedState18CreationProofV1`
(struct at `production_reference.rs:1484-1487`, `#[derive(... Serialize)]`) carries `pad_seed`
as a **serialized public field**, not a verifier-side witness. The MEMORY note for the GAP-A
build is explicit and honest about this: *"proof carries pad_seed in clear (bound to
leaf.pad_seed_digest) ... no hiding/ZK over pad_seed; a ZK public proof (transparent-descent
kind) stays fail-closed/future."* My contribution: **machine-verified that this disclosure is
not merely a pad leak but a FULL witness leak** — pad_seed determines the projection values
themselves (`hidden_source_fallback_pad_prf_projection_values_from_seed`, line 7139, via
`reviewed_fq12_pad_residues_from_seed_bounded`, line 37383), so there is nothing left hidden.
The PRF-freshness of the mask buys ZERO hiding because the seed travels with it.

## Code citations (all `tools/parameter-search/src/production_reference.rs`)

- `1484-1487` — `Path2MaskedState18CreationProofV1 { pad_seed, account_projection }`,
  `#[derive(Serialize)]` ⇒ pad_seed on the public wire.
- `verify_path2_direct_creation_masked_state18_proof_v1` — hard-gated public verifier for this proof.
- `verify_path2_direct_creation_masked_state18_proof_debug_relation_v1` — internal historical
  relation checker that still binds `proof.pad_seed` to `leaf.pad_seed_digest`.
- `6404-6420` — `reviewed_fq12_packed_witness_state18_projection_masks(pad_seed, slot_id)`:
  masks are a deterministic SHA-256 PRF of `(pad_seed, slot_id, idx)`.
- `37383-37405` — `reviewed_fq12_pad_residues_from_seed_bounded`: pad table from pad_seed.
- `7139-7168` — `hidden_source_fallback_pad_prf_projection_values_from_seed`: the
  hidden-source projection (the witness) derives from pad_seed.
- The transparent-descent verifier is now the active hiding path under test; it no longer serializes
  `pad_seed` and its opening uses masked-share rows.
- `53061-53073` — `path2_masked_state18_path_does_not_flip_production_preflight`: production
  stays fail-closed (the gating that makes this pre-deployment, not live).

## Other vectors attacked (no additional break found; reported for the record)

**(a) Statistical battery on the spend/opening surface — clean, by construction.** The
genuinely-hiding pre-spend surface holds pad_seed as a verifier-side witness, never
serializing it (`ProductionReviewedFq12StateAssistedOpening`, 777-785). The verify functions
also suppress projection material in error strings (`assert_no_selected_projection_material`
in the `masked_projection_*` tests, ~62892). No statistical channel needed here once (4)
above lands the witness directly.

**(b) Mask reuse / one-time-pad-reuse on masked_projection — does NOT leak.** The mask
`uniform_mod_from_hash(MASK_DOMAIN, pad_seed||slot_id, slot_id, idx, Q)` is deterministic in
`(pad_seed, slot_id, idx)`, so the SAME (slot, lane) mask is reused across every opening for
that slot. That would be a classic many-time-pad leak IF the masked value varied across
observations while the mask stayed fixed. It does not: the consume-once lifecycle binds one
`account_projection` per slot, so `masked_projection[idx]` is constant per (slot, lane); cross
slot the mask differs (`slot_id` is in the seed). No exploitable repetition channel here.
The leak is purely the pad_seed-in-clear creation proof, which dominates everything.

**(c) Fiber over-determination from accumulated openings — subsumed.** The
`accepted_fiber_constraint_report` warning about accumulating openings is moot on the
creation path: a single creation proof already publishes pad_seed = the whole fiber. No
accumulation argument is needed.

**(d) ML/regression distinguisher — unnecessary.** A learned distinguisher would be a weaker,
probabilistic version of an EXACT deterministic recovery. The recovery here is algebraic and
exact (r=1.0 trivially), so a regression model was not run; it would only re-derive the same
`masked - PRF(pad_seed,...) = projection` map the test reproduces exactly.

## Residual for a better-resourced adversary

- The defense's intended fix is the **transparent-descent ZK creation proof** (hides
  pad_seed, proves the relation in zero knowledge). It is unimplemented. The next adversarial
  step once it lands: attack THAT proof system's soundness/zero-knowledge directly — its
  `query_count`, `column_profile`, masked-column-set hiding, and the SHA-256 frame-PRF /
  RO-PoK assumption (`A_sha256_projection_mask_frame_prf`) the global composition theorem
  carries. Until then there is no hiding path to attack — the only one that exists is broken.
- Confirm at the network/serialization layer that no other deployable proof kind
  (state-assisted spend certificate `ProductionReviewedFq12MaskedProjectionSoundnessCertificate`,
  739-748, which embeds the full `packed_witness_opening` with `account_projection` +
  `projection_masks` in clear) is ever emitted on a public surface pre-spend; that struct is
  a second full-witness disclosure object if it escapes the verifier boundary.

## Artifacts

- Reproduction test: `tools/parameter-search/src/production_reference.rs`
  (`redteam_masked_state18_creation_proof_leaks_full_witness_key_free`).
- This report.
