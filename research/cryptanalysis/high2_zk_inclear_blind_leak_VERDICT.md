# HIGH-2 REOPENED — transparent-descent ZK leaks the witness via an in-clear per-row blind

**Date:** 2026-06-11 · **Severity:** CRITICAL (hiding) · **Status:** REPRODUCED (passing PoC) ·
**Effect:** the HIGH-2 "fix" (commit `742c6b8`) does **NOT** hide the witness. The full
hidden-source `account_projection` AND the projection `mask = PRF(pad_seed, slot)` are
recoverable **key-free** from the serialized creation-proof bytes by a single modular
subtraction. HIGH-2 hiding is **NOT resolved**.

## The break (verified by direct code reading + executable PoC)

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

## Reproduction (executable, this session)

`production_reference.rs::tests::redteam_transparent_descent_zk_proof_leaks_witness_and_mask_via_inclear_blind`
(`:54396`) decodes only the public `out.proof_bytes`, takes `opened_rows[0]`, subtracts the
in-clear blind, and asserts equality with the true secret `account_projection` and
`mask = reviewed_fq12_packed_witness_state18_projection_masks(pad_seed, slot)`.

```
test ...redteam_transparent_descent_zk_proof_leaks_witness_and_mask_via_inclear_blind ... ok
test ...path2_transparent_zk_proof_hides_pad_seed_and_witness ... ok    # still green — blind to arithmetic
```

## Required fix (genuine ZK redesign — NOT applied; flagged for maintainer decision)

The blind must never be opened in the clear for a cell that carries the witness. Options:
- **(a)** Open only blind-invariant linear combinations of cells (the constraint check is
  already affine — express acceptance purely over differences that cancel the per-row
  blind), and never reveal `b_row`.
- **(b)** Use independent per-cell blinds drawn from a committed-but-unopened randomness
  column, with the constraint verified over blind-invariant combinations.
- **(c)** A standard commit-and-prove / masked-evaluation argument where opened evaluations
  are of a *blinded* low-degree extension and the blinding polynomial is committed but its
  evaluations are never individually revealed.

All three are real changes to the prover/verifier opening protocol (and to the proof byte
layout / KAT). This is the "significant redesign" class — do not land silently.

## Scope / what is NOT affected

- **Deployable spend signature (262/318 B): unaffected.** Independent audit this session
  confirmed every serialized spend field is public-derived or an intended sigma opening;
  `pad_seed` / hidden source / masked representative are not serialized; projection mask is
  PRF-fresh per (slot,lane), pad excluded. (See spend-leak audit.)
- This break is in the **creation-time** transparent-descent ZK only — the same surface as
  the original HIGH-2 (`Path2MaskedState18CreationProofV1` pad_seed-in-clear). HIGH-2 is
  **reopened**, not a new class.
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
- **The pad_seed-in-clear `Path2MaskedState18CreationProofV1` path is not hard-gated** — only
  a doc label + the deployment preflight. `verify_path2_direct_creation_masked_state18_proof_v1`
  still `Ok`-accepts a valid pad_seed-leaking payload; no compile/runtime guard prevents a
  caller from wiring it as the active acceptor.
- **Stale docs.** HEAD `742c6b8` says "hiding RESOLVED," but in-file comments
  (`:1495,:1519,:26641`) still say the transparent path is "IN PROGRESS (stubbed)" and the
  bindings dispatcher (`:26577-26578`) rejects TRANSPARENT_DESCENT as "not implemented." The
  commit message over-claimed relative to the code's own comments.

## Bottom line

The riskiest hiding obstruction reopened: the HIGH-2 transparent-descent ZK does not hide.
This is exactly the risk-first payoff — caught internally, with a passing regression, before
any external review or value-capped deployment. The fix is a real opening-protocol redesign;
flagged for maintainer decision, not applied.
