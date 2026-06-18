# HIGH-2 In-Clear Blind Leak — transparent-descent opening protocol fixed

**Date opened:** 2026-06-11 · **Severity:** CRITICAL (hiding) ·
**Current status:** FIX APPLIED in the Rust reference opening protocol; focused
regression passes.

The original HIGH-2 transparent-descent "fix" (commit `742c6b8`) did **not**
hide the witness: the full hidden-source `account_projection` and the projection
`mask = PRF(pad_seed, slot)` were recoverable key-free from serialized
creation-proof bytes by one modular subtraction.

**Fix applied:** the transparent opening now uses relation-preserving masked
shares instead of raw cells plus a public per-row blind.  The proof body no
longer serializes `blind`.  For each lane, the prover opens masked shares

```text
account' = account + s + 128*t
mask'    = mask - (s + 128*t)
lo'      = lo + s
hi'      = hi + t
```

and the verifier checks the affine constraints directly on those shares:

```text
account' + mask' = masked_projection
account' = lo' + 128*hi'
```

without receiving `s`, `t`, `account`, or `mask`.

Current regression:

```text
redteam_transparent_descent_zk_proof_blocks_inclear_blind_recovery
```

decodes only public proof bytes, verifies the opened row still satisfies the
public C5 seam, and confirms the legacy blind-subtraction recovery no longer
returns either the raw account projection or the PRF masks.

## Historical Break

The transparent-descent ZK arithmetizes the witness into a trace whose row cells are the
**raw** secret values, then "blinds" each opened row with a single additive scalar `b_row`
— but transmits `b_row` **in clear** next to the blinded cells. De-blinding is `cell =
blinded_cells[idx] − blind (mod Q)`.

1. **Trace stores the raw witness** — `production_reference.rs:46778-46785`
   (`path2_transparent_zk_build_trace`):
   - `row[lane]     = opening.account_projection[lane]`  (raw hidden-source witness, lanes 0..6)
   - `row[6 + lane] = masks[lane]`                       (raw projection mask, lanes 6..12)
   - `row[12 + lane]= opening.masked_projection[lane]`   (public anyway)
   - `row[18 + 2·lane (+1)]` = base-128 digits of `account_projection` (also raw)
   - `row[30] = b_row` — the per-row blind, derived from `blinding_seed` (not `pad_seed`).

2. **The opening emits blinded value AND blind together, in clear** — prover open site
   `production_reference.rs:47436-47453`:
   ```rust
   let b_row = trace.cells[row][30];
   for (idx, slot) in blinded_cells.iter_mut().enumerate() {
       *slot = if idx == 30 { trace.cells[row][idx] }
               else { add_mod(trace.cells[row][idx], b_row) };   // cell + b_row
   }
   opened_rows.push(Path2TransparentZkOpenedRow {
       blinded_cells,          // serialized in clear (encode body :47063-47065)
       blind: b_row,           // serialized in clear (encode body :47066)
       ...
   });
   ```
   Body serialization writes both fields verbatim (`path2_transparent_zk_encode_body`,
   `:47063-47066`); decode reads them back (`:47171-47177`).

3. **Recovery** — for any opened row, `account_projection[lane] = blinded_cells[lane] −
   blind` and `mask[lane] = blinded_cells[6+lane] − blind`. All 16 query rows replicate the
   SAME witness (`:46770-46786`), so **one** opened row suffices. From `mask` + the public
   `masked_projection` the witness is doubly confirmable.

The Merkle **commitment** is genuinely hiding — `path2_transparent_zk_trace_leaf`
(`:46824-46848`) commits `cell + b_row` and the doc-comment correctly claims the *leaf*
reveals nothing without the nonce. The leak is entirely in the **opening**, which hands the
verifier (and any observer) both the blinded value and the blind. Blinding that is opened
alongside its own mask provides **zero** hiding.

## Why the existing hiding test missed it (false assurance)

`path2_transparent_zk_proof_hides_pad_seed_and_witness` (`:54309`) only runs
`contains_window` — a **contiguous-byte substring scan** — for `pad_seed`, the mask vector,
and `account_projection`. A value recoverable by modular **arithmetic** is invisible to a
substring scan. Confirmed this session: the substring hiding test and the recovery PoC
**both pass simultaneously**.

## Historical Reproduction

The original PoC decoded only public `out.proof_bytes`, took `opened_rows[0]`,
subtracted the in-clear blind, and asserted equality with the true secret
`account_projection` and `mask =
reviewed_fq12_packed_witness_state18_projection_masks(pad_seed, slot)`.

```
test ...redteam_transparent_descent_zk_proof_leaks_witness_and_mask_via_inclear_blind ... ok
test ...path2_transparent_zk_proof_hides_pad_seed_and_witness ... ok    # still green — blind to arithmetic
```

## Applied Fix

The selected repair is the masked-share version of option (a): open values whose
affine constraints are directly checkable but whose raw witness components remain
one-time padded by unopened share randomness.  The proof body and decoded opened
row no longer include a public `blind` field.

The 2026-06-11 follow-up soundness pass also found that the constraint-column
residual was only a fixed linear aggregate, so failures in different lanes could
cancel.  The verifier now checks each opened row's same-representative relation
per lane: the same `account_share` cell must satisfy both
`account_share + mask_share = masked_projection` and
`account_share = lo_share + 128*hi_share`.  Regression
`path2_transparent_zk_opened_row_relation_prevents_split_witness_cancellation`
constructs a split row with aggregate residual zero and confirms the explicit
per-lane digit check rejects it.

The same pass then found the larger blocker-1 failure: the current opening core
does not enforce the full hidden relation.  Red-team regression
`redteam_transparent_zk_core_accepts_public_only_synthetic_trace` constructs an
accepting proof from public `masked_projection` alone.  That proof exercises the
real envelope, Merkle paths, sampled-row checks, and FRI verifier, but it proves
neither pad-seed binding, PRF mask derivation, packed-witness repack equality,
nor role-commitment consistency.

The current Rust metadata now binds this as a fail-closed contract:
`Path2TransparentFullHiddenRelationProofContractV1` is carried through production
obligations and the proof-family capability, but its backend/verifier are
uninstantiated and `full_hidden_relation_subproof_implemented = false`.

This is still not a full soundness claim.  The visible base-128 digit range check
is now an extractor/range-proof obligation, because checking raw digit ranges
while hiding the digits requires a real range argument rather than opening the
digits themselves.

## Scope / what is NOT affected

- **Deployable spend signature (262/318 B): unaffected.** Independent audit this session
  confirmed every serialized spend field is public-derived or an intended sigma opening;
  `pad_seed` / hidden source / masked representative are not serialized; projection mask is
  PRF-fresh per (slot,lane), pad excluded. (See spend-leak audit.)
- This break was in the **creation-time** transparent-descent ZK only — the same surface as
  the original HIGH-2 (`Path2MaskedState18CreationProofV1` pad_seed-in-clear).
- Production stays **fail-closed** at the deployment preflight, so this is not live on a
  mainnet today — but the dossier's "HIGH-2 hiding RESOLVED" claim was **false** and is
  corrected.

## Secondary findings (proof-obligation over-reach, lower severity)

- **EC §2 row #6 over-states the multi-target term.** `break_checked_core_message_binding.ec`
  carries `axiom sha256_multi_target_spr` (the T-fold union is **axiomatized**), while the
  dossier prose calls it "proven finite union accounting." The bound *direction* is correct
  and conservative (T·eps_spr), but row #6 should read **"PROVEN given a multi-target-union
  axiom,"** not "PROVEN."
- **The EC spine is not formally linked.** The three spine files share no `require import`;
  the end-to-end `Pr[forgery] ≤ 2⁻¹²⁸` is composed on paper across independently-checked
  modules (the known GAP-B/D seam). Honest, but should not be read as one machine-checked
  theorem.
- **The pad_seed-in-clear `Path2MaskedState18CreationProofV1` path is now hard-gated
  (2026-06-11).** `verify_path2_direct_creation_masked_state18_proof_v1` and the proof-carrying
  dispatcher return `PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR` for a valid old payload. The
  masked-state18 relation checker remains internal/debug-only for fixtures and red-team regression.
- **Partial-boundary caveat.** The bindings-only transparent payload boundaries still return the
  historical `"Path 2 transparent descent proof verifier is not implemented"` sentinel until the
  caller supplies proof bytes. The full `*_with_proof_bytes` boundary now runs the incomplete core
  first, then fail-closes with
  `PATH2_TRANSPARENT_DESCENT_FULL_HIDDEN_RELATION_DISABLED_ERROR` for otherwise-valid transcripts.

## Bottom Line

The specific in-clear per-row blind leak is fixed in the transparent opening
protocol and covered by an executable regression.  The immediate sampled-row
same-representative/cancellation bug is also fixed by explicit per-lane checks.
Remaining HIGH-2 work is now the full hidden-relation backend certificate, then
the separate `p2_sis_extractor` / range-proof soundness obligation, plus
widening the external coded-SIS review to the final public surface. The public
proof-bytes boundary
now rejects otherwise-valid transcripts with
`PATH2_TRANSPARENT_DESCENT_FULL_HIDDEN_RELATION_DISABLED_ERROR`, so there is no
public accepting transparent creation proof at this stage. The transparent proof-family
capability object also carries a digest of
`Path2TransparentProductionSoundnessObligationsV1`, which now records
`sampled_row_same_representative_enforced = true` and
`aggregate_cancellation_regression_covered = true`, while the current blockers
are backed by a `Path2TransparentHiddenRelationConstraintPlanV1` digest and
`Path2TransparentFullRelationBackendSufficiencyTargetV1` digest and remain
`pad_seed_binding_constraint_enforced = false`,
`prf_mask_derivation_constraint_enforced = false`,
`packed_witness_repack_constraint_enforced = false`,
`role_commitment_consistency_constraint_enforced = false`,
`full_hidden_relation_constraints_enforced = false`,
`full_relation_backend_certificate_instantiated = false`,
`p2_sis_extractor_instantiated = false`,
`hidden_range_extractor_instantiated = false`, and
`external_coded_sis_review_complete = false`. It rejects a forged
obligations digest or forged `production_soundness_obligations_discharged =
true`, so the missing theorem state is digest-bound rather than only
documented. Public views are pinned to the current capability digest through
`proof_system_id`; a self-consistent view under any forged proof-system id is
rejected before query or proof verification.
