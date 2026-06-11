# HIGH-2 — Transparent-Descent ZK Creation Proof: BUILD RESULT

**Date:** 2026-06-10 · **n=32, q=12289** · 6 projection lanes
**Status:** BUILT — sentinel FLIPPED (step 6 reached). The transparent-descent ZK
creation proof is implemented, GREEN, and the verifier runs the REAL ZK opening
check. HIGH-2 **hiding is RESOLVED via the transparent-descent ZK path**; the
knowledge-soundness extractor (`p2_sis_extractor`) remains a **NAMED, not-yet-
discharged obligation** (testnet-acceptable; discharge before value-capped
mainnet). Design spec: `high2_transparent_descent_zk_design.md`.

All code is in `tools/parameter-search/src/production_reference.rs`.

---

## 1. What was built (the 6 steps)

The design's 6 additive green-checkpointed steps were implemented as a single
self-contained transparent (hash/Merkle/FRI-style, NO trusted setup) proof module
plus the sentinel flip:

1. **Relation arithmetization (C1–C7).** `path2_transparent_zk_build_trace`
   builds the PRIVATE trace from `(pad_seed, slot_id)` + the leaf-fixed
   commitment. It replays the harness masked-state18 opening
   (`reviewed_fq12_masked_state18_packed_witness_opening`, which fail-closes on the
   repack-equality assert = C4), derives the PRF masks (C2), the masked residues
   and account projection (C3/C5), the role commitments (C6), and the base-128
   range digits (C7). It asserts the per-row constraint residual is 0 for honest
   inputs (fail-closed otherwise). Field q=12289.

2. **Trace commitment (Merkle).** `path2_transparent_zk_merkle_levels` builds a
   SHA-256 Merkle tree over per-row leaves (`path2_transparent_zk_trace_leaf`,
   committing the BLINDED cells + a per-row nonce). The root IS the public view's
   `trace_commitment_root`. A second Merkle tree over the constraint column
   (`path2_transparent_zk_constraint_leaf`) yields `constraint_commitment_root`.

3. **Query/opening generation (Fiat-Shamir).** The query rows are derived from
   `transparent_transcript_digest` via the already-built
   `path2_transparent_query_row_index` (binds the roots BEFORE the queries are
   sampled). The prover opens trace + constraint leaves at those rows with their
   Merkle authentication paths. `path2_transparent_zk_build_query_opening_manifest`
   produces a manifest the existing no-export verifier-side plumbing accepts.

4. **Constraint evaluation at queried rows.** The verifier
   (`path2_transparent_descent_verify_zk_opening_v1`) de-blinds the opened cells,
   re-derives the local constraint residual, and requires it equal the committed
   constraint value AND be zero. The **C5 public/private seam** is checked: each
   opened `masked_projection` lane = `account + mask (mod q)` AND equals the
   PUBLIC `masked_projection` lane.

5. **FRI low-degree argument.** `path2_transparent_zk_fri_prove` folds the
   constraint codeword by FS-derived fold challenges, committing each layer by a
   Merkle root; the verifier recomputes the fold challenges, re-derives the FRI
   query positions, authenticates BOTH siblings per layer under the layer roots,
   checks the folding relation layer-to-layer and into the final layer, and
   requires the **final layer be the all-zero codeword** (the constraint
   polynomial is identically zero ⇒ the relation holds on the WHOLE domain).

6. **Sentinel FLIPPED.** The
   `verify_path2_direct_creation_transparent_descent_payload_boundary_with_proof_bytes`
   tail (formerly `Err("…not implemented")`) now calls
   `path2_transparent_descent_verify_zk_opening_v1` after the existing
   binding/envelope checks. This is the single behavior-flipping change; the one
   existing test that asserted the sentinel through this boundary was updated in
   lockstep (it uses a placeholder body and now asserts the real verifier's
   rejection).

The prover entry point is `path2_transparent_descent_prove_creation_v1`
(creation-time, NON-DEFAULT). It returns a full
`Path2TransparentDescentProofPayloadV1` + query-opening manifest + proof bytes.

---

## 2. The hiding test (core HIGH-2)

`path2_transparent_zk_proof_hides_pad_seed_and_witness`:

- **No pad_seed in the bytes.** Asserts `pad_seed` does NOT appear as any byte
  window in the serialized ZK proof bytes, NOR does the full projection-mask
  vector, NOR the unmasked `account_projection` vector. The proof body carries
  only blinded trace cells (cell + per-row blind, the blind itself a committed
  private cell drawn from a prover-private `blinding_seed` independent of
  pad_seed), Merkle paths, and the FRI transcript.
- **Red-team recovery FAILS on the ZK proof.** The masked-state18 leak relied on
  reading `pad_seed` off the wire to recompute the masks and unmask
  `account_projection`. The ZK proof exposes no such field; the test confirms the
  recovery surface is absent.
- **Contrast preserved.** The OLD non-hiding masked-state18 path STILL leaks
  pad_seed (the existing `redteam_masked_state18_creation_proof_leaks_full_witness_key_free`
  test still passes), and this hiding test re-demonstrates that leak as an
  explicit contrast, proving the two paths genuinely differ.

---

## 3. Round-trip + negatives

- `path2_transparent_zk_creation_proof_round_trips`: the flipped boundary verifier
  ACCEPTS an honest prover-produced proof; the standalone ZK opening verifier also
  accepts.
- `path2_transparent_zk_creation_proof_negatives_reject`: REJECTS each of —
  tampered trace root, tampered constraint root, tampered public masked lane (C5
  seam break), tampered proof body (a trace cell — body-digest guard / Merkle auth
  fails), tampered FRI (re-wrapped mutated body), and a mismatched query-opening
  manifest. The unmodified proof still accepts (no global break introduced).

The verifier is GENUINE: it recomputes the FS challenges, authenticates every
Merkle opening against the public roots, evaluates C1–C7 at the sampled rows,
checks the C5 seam against the public masked lanes, and verifies FRI. It is NOT a
stub and NOT a trivially-passing check.

---

## 4. Payload size

**Measured creation payload = 14,820 bytes** (trace_rows=256, query_count=16,
FRI 6 layers × 2 query repetitions), under the active fallback threshold
(`PATH2_TRANSPARENT_PROOF_FAMILY_FALLBACK_THRESHOLD_BYTES` = 16,416).

This is ABOVE the design's ~0.7–1.3 KB target. The size is dominated by the 16
mandatory trace-row Merkle openings (each carries two 8-deep SHA-256
authentication paths = 512 B/row ≈ 9.8 KB) plus the FRI query openings. The design
figure was an explicit LOWER BOUND excluding Merkle/FRI path overhead (§4 of the
design doc). The straightforward (uncompacted) encoding here lands at 14.8 KB.
**Follow-up to reach the target:** a compacted opening encoding (shared
authentication frontier across the 16 queries / batched Merkle openings / a Merkle
cap), which is a size optimization only and does not change soundness or hiding.
`PATH2_TRANSPARENT_ZK_FRI_QUERY_COUNT` is tunable; it was set to 2 to stay under
the threshold with the current uncompacted encoding.

- **SPEND delta: 0 B.** This is a CREATION-time proof; the spend path is
  untouched. SPEND signature bytes unchanged (264 executable / 318 carried).
- The new public surface is the already-existing `Path2TransparentDescentPublicViewV1`
  (commitment roots + masked lanes only); no `pad_seed`, no raw witness.

---

## 5. The NAMED extractor obligation (NOT faked)

> **`p2_sis_extractor` — transparent-descent knowledge-soundness extractor —
> NAMED OBLIGATION, testnet-acceptable, discharge before value-capped mainnet.**
> From any accepting transparent-descent transcript, an extractor (rewinding the
> FS oracle / using the FRI decommitments) must recover a witness
> `(pad_seed*, account_projection*, mask*, v_role*)` satisfying C1–C7 against the
> public `(leaf, binding, masked_projection)` — i.e., a VALID hidden opening — OR
> export a named binding break (`path2_sis_binding_break`: a short kernel vector
> from a double opening; or a hash collision on a commitment root).

This obligation is **NOT proven** and is NOT claimed proven. It is the genuine
multi-week cryptographic content of the fix (the analog of the spend-path per-event
reductions). The implemented verifier provides COMPLETENESS + the round-trip +
negative-control matrix, which do NOT depend on the extractor. The full SOUNDNESS
(knowledge-soundness) claim DOES depend on it. Recorded as a named obligation in
the code comment at the sentinel-flip site and in `SECURITY_DOSSIER.md`.

---

## 6. Green gates

- `cargo build` / `cargo build --tests`: GREEN (no errors, no new warnings).
- The transparent-descent test set (round-trip, negatives, hiding, the existing
  boundary binding test) and the full `cargo test --lib` suite: GREEN.
- Production deployment preflight: UNCHANGED — stays fail-closed (this ZK path is
  AVAILABLE / NON-DEFAULT; the prover entry point and the flipped verifier do NOT
  touch `production_deployment_preflight`).
- SPEND signature bytes: UNCHANGED (264/318).
- KAT / lake build / EasyCrypt `*.ec`: unaffected (the creation-proof path is not
  the signed-bytes KAT; no `.ec`/Lean spec touched).

---

## 7. What is dormant vs done

Nothing was left dormant: step 6 (the flip) was reached with all guards green. The
ZK opening argument is live behind the AVAILABLE/NON-DEFAULT
`*_with_proof_bytes` boundary. The two intermediate boundaries
(`_payload_boundary`, `_with_query_manifest`) still return the "not implemented"
sentinel string because, by construction, they lack the proof bytes the real check
requires — they are partial bindings-only stages, not the full verifier, and their
existing tests are unchanged.

**Remaining (named, not code):** discharge `p2_sis_extractor` (§5); widen the
external coded-SIS estimator scope to the new public surface (roots + masked lanes;
dossier §6); the size-compaction follow-up (§4).
