# descender-account-stark AIR — adversarial soundness audit (2026-06-18)

> Classification: `ACTIVE_SUPPORT`. This audit supports the active
> Poseidon2/Plonky3 account-creation path; it is evidence for the spec spine and
> tracker, not normative authority by itself.

Triggered after confirming the STARK as PRIMARY for the creation relation: before
building the recursion layer on it, verify the leaf proof is actually sound. A
harsh adversarial audit read the full AIR (`src/air.rs`), reduce gadget, public
wiring (`src/lib.rs`), config, and all tests, hunting for under-constrained columns
that would let a FALSE statement accept.

## Headline

**No CRITICAL "false statement accepts" hole in the AIR.** Contrary to the project's
"internal proofs over-reach the as-built constraints" history, all 8 relation
classes are genuinely wired into the constraint polynomial (not declared-only):

- Ranges/canonicity (account_projection, syndrome, masked_projection) are REAL
  base-128 / 14-bit decomposition + slack constraints; a lane ≥ q cannot pass.
- C4 mod-q reduction is constrained (boolean wrap carry; `acc,mask ∈ [0,q)` ⇒ exact),
  reduce_gadget Horner reduction in-AIR.
- C2 masks are PRF-DETERMINED — Poseidon2 sponge constrained round-by-round; masks
  are not free columns (cross-checked by tests/mask_prf_equivalence.rs).
- C1/C6 commitments constrained round-by-round; C6 preimage order incl. account_id
  enforced (openingTupleAccountBinding real).
- role_root + packed_witness_commitment constrained equal to functions of the
  committed columns; 14-bit-LE packing bit-exact.
- Public values bound in single mode (per-row `== public[idx]`) AND batch mode
  (in-AIR Poseidon2 chain digest over per-row pubio, enable boolean+monotone,
  count + last-row digest bound). Reorder/fabricate/inflate attacks reject.
- The single-instance "2 identical rows" is benign (both rows bound to the same
  public commitments ⇒ pinned to one statement; a differing row needs a collision).

## Findings (MED — single-points-of-failure, not active breaks)

**MED-1 — `account_state_root` + `verifier_version` have ZERO in-AIR constraints.**
Bound ONLY by observation into the FS transcript via `public_values_vec`
(`lib.rs` context_field_elems). Same shape as the prior batch "verify with empty
public values" break: any downstream path that verifies with a different/empty pv
silently loses the binding. Status: **negative-tested** (tests/soundness_context_binding.rs
— tampering each is rejected via transcript divergence). Recommended hardening
(defense-in-depth, not yet applied): give these two context blocks dedicated in-AIR
trace anchors in single mode (batch already binds ctx==pubio), so the binding does
not rest solely on the verifier wiring.

**MED-2 — `syndrome` is not tied to a real spend checked-core frame in-AIR.** The
AIR only proves the 12 syndrome residues are canonical and packed into
packed_witness_commitment; a malicious prover may pick any canonical residues. The
creation→spend tie is deferred to EXTERNAL wiring (honest per lib.rs doc). Named
obligation: the spend verifier must check packed_witness_commitment byte-equal to
the deployed leaf, and reject attacker-chosen syndromes. (NOT a defect of this
crate — a wiring obligation to track + test at the integration site.)

**MED-3 — C4 canonicity inverse-witness corner (`is96`/`inv`, air.rs:1397-1406).**
Correct as built (`is96=[d_hi==96]`, then `is96·d_lo=0`), but it's the one
equality-via-prover-inverse gadget — historically where under-constraint edits hide.
Recommend a check_constraints unit test for the `d_hi=96, d_lo≠0` corner so a future
edit dropping the `(d_hi-96)·inv = 1-is96` line is caught.

## Caveats (out of AIR scope, load-bearing for the end-to-end AoK claim)

- **FRI soundness label:** PROD_SOUNDNESS_BITS=128 is a conservative-Johnson
  rule-of-thumb over a ~124-bit challenge field, not a proven AoK bound (named-
  standard posture, consistent with the dossier).
- **Knowledge extractor:** a fully-constrained AIR is necessary but NOT sufficient
  for argument-of-knowledge; the FRI-IOP extractor (`p2_sis_extractor`-style)
  remains the named standard/external obligation.

## Remediation applied (2026-06-18, same session)

- **MED-1 — negative-tested AND hardened in-AIR.** `tests/soundness_context_binding.rs`
  (2/2 pass): tampering `account_state_root` / `verifier_version` is rejected
  (transcript binding confirmed working). PLUS the AIR now pins both ctx blocks to
  the public statement in single mode (`air.rs` block (d); batch already binds
  ctx==pubio via the digest chain), so the binding is enforced by the CONSTRAINT
  system, not the transcript alone. Re-validated: 14 unit + 10 roundtrip + 2
  context-binding tests pass after the change (honest proving unaffected).
- **MED-3 — regression guard added.** `air.rs` test
  `c4_is96_inverse_corner_violates_constraints`: lane 5 (= q-1) flips only `is96`→0
  to isolate the `(d_hi-96)*inv == 1-is96` line; catches a future edit that would
  re-open the non-canonical-lane corner.
- **MED-2 — CLOSED at the live admission boundary (2026-06-18).** Originally tracked
  as an external wiring obligation. On review, it was a LIVE gap in the new STARK
  boundary (`verify_path2_direct_creation_account_stark_payload_boundary`): it bound
  `masked_projection` + context + role-root but NOT the packed witness, so a prover
  could supply arbitrary canonical `syndrome` lanes. FIX: `DeployedCreationAnchors`
  now carries `packed_witness_commitment` and the verifier binds the PROVEN
  `public.packed_witness_commitment` == the deployed binding's
  (`descender-account-verifier/src/lib.rs` step 5; parameter-search boundary sets it
  from `binding.packed_witness_commitment`). Since the AIR proves
  `packed_witness_commitment = pack(syndrome ‖ masked_projection)`, pinning those 32
  bytes ties BOTH the syndrome and the masked projection to the exact packed witness
  the spend's checked-core frame consumes. Negative-tested
  (`tests/deployed_binding.rs`: a disagreeing packed_witness_commitment is rejected).
  (At creation time the deployed binding's packed_witness_commitment is the anchor;
  the spend separately consumes the same packed witness — so creation↔spend is now
  bound through this shared commitment.)

## Net

The leaf STARK's arithmetic is sound as built; MED-1 is now both transcript-bound
(tested) AND in-AIR-anchored (defense-in-depth), MED-2 is CLOSED at the live admission
boundary (packed-witness binding, negative-tested), and MED-3 has a regression guard.
ALL THREE MED findings remediated. Remaining: the standard FRI-AoK extractor
(`p2_sis_extractor`) obligation and the conservative-Johnson FRI soundness label —
both named-standard, neither an active false-accept.
