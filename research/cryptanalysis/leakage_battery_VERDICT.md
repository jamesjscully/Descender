# WS4 #11 — Statistical Witness-Hiding of Real Transcripts: RISK-FIRST VERDICT

**Target:** Statistical witness-hiding of REAL signature transcripts emitted by the deployed
signer (`reference_sign_with_proof`, `signature_q12289_local_delta_stateful.rs:4678`),
measured against the existing zero-TV / `residual_hiding_meets_140` hiding certificate
(`MaskedResponseSamplerCertificate`, ~2546-2614; HIGH caveat at 6459).

## MAINTAINER RECLASSIFICATION (2026-06-10): CONDITIONAL-GAP (serious, pre-deployment blocker) — NOT design-BROKEN

The attack-agent verdict below says BROKEN. After verifying the load-bearing code facts
directly, I am **reclassifying to CONDITIONAL-GAP** — the empirical leak is REAL and
verified, but "BROKEN (redesign the design now)" overclaims, for two decisive reasons:

1. **The leaky signer is the known non-deployable SCAFFOLD.** `reference_sign_with_proof`
   emits a `LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES` signature (4748) — the 260B
   linear-sigma *representative-style* path (named as such at 6448), which the project
   already classifies as a byte-anchor / negative-control, NOT the deployable signer. Its
   constant mask `[12, Q-12]` (4703), constant `sample_proof_commitment` (`[7;..]`, 97), and
   static keygen-time seed make the pad recoverable key-free — verified — but this is the
   placeholder, the signer-side analog of the fail-closed verifier scaffold.
2. **The ≥140 certificate explicitly EXCLUDES the pad.** `MaskedResponseSamplerCertificate.
   excluded_conditioning` lists "active pad outputs", "active local openings",
   "challenge-selected block pad equations" (2596-2599); `residual_hiding_bits` is computed
   from the quotient-fiber bits only (2551-2552). So ≥140 is the *quotient-fiber* hiding and
   was never claimed to hide the pad — the certificate is INCOMPLETE for the pad, not
   contradicted. A conditioned sampler IS implemented for the fiber
   (`sample_conditioned_quotient_fiber_representative`, 3363); the pad-hiding predicate
   (`pad_constraints_layer: "existential quotient-opening predicate"`, 2602) is SPECIFIED but
   UNREALIZED, and already tracked by the existing HIGH finding at 6459 (`required_closure`).

**Honest status:** witness-hiding is **NOT established for the pad/openings**; the scaffold
signer demonstrably leaks it; `residual_hiding_meets_140` MUST NOT be cited as covering
witness-hiding. This is a genuine pre-deployment blocker with **real redesign risk** (closing
it needs: stop emitting `pad(seed)` openings in clear, fresh per-signature mask/representative,
realize the existential opening predicate, and charge `active_pad_constraint_bits` — the
agent's engineering fix below is correct). But it is CONDITIONAL because we have shown the
*scaffold* leaks + the obligation is *unbuilt* — we have NOT shown the *intended* conditioned-
pad design cannot hide it. **Next risk-first action: determine whether the deployable design
can hide the pad** (does the prevalidation / strict-sigma path avoid emitting pad openings, or
can a conditioned pad sampler / commitment-bound opening achieve it with the pad constraint
charged?). That call decides CONDITIONAL-GAP-fixable vs latent-needs-restructuring. Do NOT cry
wolf; do NOT minimize.

---

## ATTACK-AGENT RAW VERDICT: BROKEN (reclassified above)

Witness-hiding of the deployed transcript is **0 bits for the active-pad witness**, not
`>=140`. Two independent attacks — an empirical battery on 100k real signatures and a
sampler-discipline code audit — agree, and I re-confirmed every load-bearing fact directly
against the source. This is a measurable witness leak with a concrete, key-free recovery
procedure. **Redesign now.** The existing certificate does not cover the distribution the
signer actually emits.

## Why BROKEN, not CONDITIONAL

The CONDITIONAL bar is "discipline asserted-but-unenforced as a *latent* risk while the
empirical surface is clean." That bar is **not** met here: the empirical battery found a live
leak AND the audit found the precondition false in code. Both attacks land on the same
defect, so there is no clean-empirical/unenforced-discipline split to downgrade — the two are
the same broken object seen from two sides.

## Reconciliation of the two attacks (they agree, no tension)

- **Empirical (battery on 100k real sigs, 25k/slot, fixed keypair):** public openings reveal
  the SECRET pad table EXACTLY. All 128 pad cells (16 blocks x 2 coords x 4 slots)
  reconstructed bit-for-bit; Pearson(open-mask, true_pad) r=1.000000, p~0 over 5.2M pairs,
  BH-q=1.18e-300; 39091/39091 intra-signature repeated-block openings byte-identical;
  responses an exact deterministic public function of (challenge, public_free) for all 100k.
  Challenge uniformity correctly clean (block chi2=17.7, dof=15, p=0.279; sign p=0.893),
  confirming the battery has discriminating power and is not flagging noise. 11/13 hypotheses
  BH-significant.
- **Audit (signer + verifier + certificate dataflow):** the zero-TV precondition
  `active_pad_outputs_conditioned=false` (2601) is a hand-set report literal that is FALSE in
  code. The signer never runs `production_sampler_law`; it reads a representative sampled once
  at keygen, uses a constant mask, and publishes pad-conditioned openings that the verifier
  recomputes from the hidden seed and rejects unless they match.

The empirical "exactness" is the measured shadow of the audit's "static-seed PRF + constant
mask." Same leak, two methods. No reconciliation conflict.

## Code-derived facts (re-verified this pass, line-exact)

1. **Mask is a hard-coded constant.** `let mask_view = [12_u16, Q - 12];` (4703) — identical in
   every signature, not a uniform draw.
2. **Openings publish the pad in clear.** `openings.active_free[rep] = [add_mod(mask_view[0],
   pad[0]), add_mod(mask_view[1], pad[1])]` with `pad = expand_pad_residues(&seed, block)`
   (4745-4746). Since `mask_view` is the public constant `[12, Q-12]`, any observer computes
   `pad(seed, block) = opening - [12, Q-12] (mod q)` with **zero key material**.
3. **Seed is static per slot.** `masked_witness_response` (hence the pad `seed`,
   4698-4702) is `slot.ideal_masked_witness_response`, sampled ONCE at keygen
   (`reference_keygen` ~4593) and stored (4602). No per-signature sampling/rejection loop.
4. **No per-signature randomness.** `sample_proof_commitment()` returns the constant
   `[7; ..]` (97-99). The whole transcript is a deterministic function of (static
   representative, constant mask, message).
5. **Verifier conditions on the active pad inside the hidden representative.** It recomputes
   `pad = expand_pad_residues(&seed, block)` from the seed carried in
   `masked_witness_response` and rejects unless `openings == mask + pad` (4080-4084) — exactly
   the "active pad outputs conditioned" the certificate excludes (`excluded_conditioning` =
   {"active pad outputs", "active local openings", "challenge-selected block pad equations"},
   2596-2599) and the HIGH caveat at 6459/6464 says invalidates the zero-TV cert.
6. **The hiding budget never charges for the pad.** `residual_hiding_meets_140` (2571) keys
   off `residual_hiding_bits = raw_algebraic_fiber_bits - modeled_fiber_state_constraint_bits`
   (2551-2552); it subtracts only the 16 fiber-state bits and **never subtracts
   `active_pad_constraint_bits`**. `active_pad_outputs_conditioned` (2601) and
   `pad_constraints_layer: "existential quotient-opening predicate"` (2602) are report
   literals; the predicate is named but **never realized in code** (grep finds the closure at
   6464 only as a report string — no enforcing test).

## Concrete attack (key-free, single pass)

Sign across all 16 active blocks (a handful of messages suffices to cover the
challenge-selected blocks). For each observed block, read
`pad(seed, block) = opening - [12, Q-12] (mod q)`. After the blocks are covered the entire
per-slot pad table is reconstructed exactly and is **static** (fixed keygen seed), so all
future signatures on that slot are fully predictable in their pad/opening component. This
exposes `LOCAL_FREE * SIS_RESIDUE_BITS` pad-stream bits per distinct active block; multi-
message signing on a static representative accumulates pad outputs across blocks, driving
toward the fiber over-determination the codebase's own `accepted_fiber_constraint_report`
(2948-2973) warns about. Empirical pad-witness hiding = **0 bits**, against a certified
claim of `>=140`. The required closure at 6464 is **NOT met**.

## Residual hiding bits / assumptions (for the record)

- **Certified (idealized):** `residual_hiding_bits` ~ `raw_algebraic_fiber_bits - 16`,
  asserted `>=140`, with exact conditioned support `12289^9 * 3073` and TV distance exactly 0
  — but **only for the conditioned fiber sampler whose support EXCLUDES active pad outputs**.
- **Deployed (actual):** for the active-pad witness, **0 bits** (exact reconstruction). The
  `>=140` figure is the wrong distribution: it certifies an idealized sampler the signer does
  not run.
- Assumption gap: even the idealized `>=140` is a 14-bit capacity heuristic that does not
  subtract `active_pad_constraint_bits`; the exact min-entropy floor is tracked separately
  and would need the pad constraint charged before any hiding claim can stand.

## Single concrete next action

**Stop publishing the pad-conditioned openings as plaintext witness.** Replace the
in-the-clear `openings = [12, Q-12] + pad(seed, block)` (signer 4745-4746) with a hidden,
commitment-bound opening check — realize the named-but-absent `pad_constraints_layer`
"existential quotient-opening predicate" (2602) so the verifier validates the pad/opening
equations WITHOUT the signer emitting `pad(seed, block)` (and, in tandem, sample a FRESH
per-signature mask/representative so openings are not a deterministic PRF of a static keygen
seed). Then make `active_pad_outputs_conditioned` a **code-derived** value gated by a
regression test that fails if any pad output appears in the transcript, and recompute the
hiding budget with `active_pad_constraint_bits` subtracted. Until that single change lands,
WS4 #11 is **BROKEN** and `residual_hiding_meets_140` must not be treated as holding for the
deployed signer.
