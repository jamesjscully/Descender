# Sampler-Discipline Audit: the zero-TV witness-hiding precondition

Target: WS4 open item #11 (statistical witness-hiding of REAL signature transcripts).
Object under audit: the precondition of `MaskedResponseSamplerCertificate` /
`QuotientWitnessHidingGameBound` — the zero-TV hiding claim is valid ONLY for the
"allowed conditioned quotient-fiber sampler" and is invalidated by conditioning on
active pad outputs (HIGH finding, `signature_q12289_local_delta_stateful.rs:6459`).

Question: does the PRODUCTION signer enforce that precondition in code, or is the
discipline asserted-in-a-report-only?

Verdict: **CONDITIONAL-GAP / discipline NOT enforced at sign time.** The hiding
certificate's own stated `production_sampler_law` is not what the deployed signer
runs, and the verifier binds published openings to the active pad outputs derived
from the hidden seed. The active-pad-output conditioning that the certificate names
as invalidating — is present in the live transcript. This is the leak path.

All line numbers are in
`tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`.

---

## 1. What the certificate claims as its precondition

`masked_response_sampler_certificate()` (2546) asserts a zero TV distance
(`statistical_distance_numerator: 0`, `statistical_distance_is_zero: true`, 2607-2609)
under an explicitly named sampling law (2603):

> production_sampler_law: "sample uniform kernel coefficients for the 12x23 quotient
> map, add them to a digest-syndrome representative, and reject only on canonical
> packing, fixed fiber-state equality, and response bounds"

and `excluded_conditioning` (2596-2601) lists, as things that must NOT be conditioned:
"active pad outputs", "active local openings", "challenge-selected block pad equations",
with `active_pad_outputs_conditioned: false` (2601). `quotient_hiding_sampler_review()`
(2694) restates the implementation obligation (2706):

> "production signing must sample uniform kernel coefficients and reject only on
> canonical packing, fixed fiber-state equality, and response bounds; any
> active-pad-output conditioning invalidates the zero-TV certificate"

and concludes (2707) that "the next review item is **code-level enforcement** of the
allowed conditioning predicate." The HIGH finding's `required_closure` (6464) is:
"enforce the allowed rejection predicate in the production signer API and add tests
that active pad equations are checked **outside** the hidden representative sampler."

So the certificate is honest that it is a precondition, and that enforcement is open.
This audit establishes that the precondition is FALSE in the deployed signer.

---

## 2. What `residual_hiding_meets_140` actually certifies (and under which assumptions)

`residual_hiding_meets_140` (2571) is `residual_hiding_bits >= 140`, where
`residual_hiding_bits = raw_algebraic_fiber_bits - modeled_fiber_state_constraint_bits`
(2551-2552). This is a **capacity / counting heuristic**, not the TV bound and not the
distinguishing advantage:

- `raw_algebraic_fiber_bits` = bit-capacity of the 11 free q-residues + slack (2547-2549).
- It subtracts only the modeled 16-bit fiber-state constraint.
- The certificate itself flags (2586-2587, `capacity_model_overstates_exact_floor_by_bits`)
  that this 14-bit capacity model OVERSTATES the exact support floor, and
  `QuotientWitnessHidingGameBound` (2617) demotes the support floor to "audit metric...
  not the distinguishing advantage" (2631, `support_role`).

Crucially, `residual_hiding_bits` subtracts ONLY the fiber-state bits. It does NOT
subtract the active-pad-output bits. The companion `accepted_fiber_constraint_report()`
(2939) computes exactly what IS subtracted once pad outputs are bound:
`active_pad_constraint_bits = distinct_active_blocks * LOCAL_FREE * SIS_RESIDUE_BITS`
(2948-2949), and its `interpretation` (2973) states outright:

> "active pad constraints still overdetermine the fiber unless pad checks are composed
> **outside** the hidden representative."

So the 140-bit number is conditional on the pad outputs being OUTSIDE the hidden
representative law. The "zero TV" is exact only as a tautology: conditioning a uniform
sampler by rejection on the predicate that DEFINES the ideal law gives the ideal law
(2610). It is exact arithmetic about an idealized sampler, **not** a statement about the
bytes the deployed signer emits.

---

## 3. What the production signer actually does (code, not report)

Production sign path: `production_sign` (4787) -> `prodref::sign_with_secret_key` ->
`reference_sign_with_proof` (4678). The relevant facts, read directly:

1. **No per-signature sampling of the representative.** The masked witness response is
   NOT sampled at sign time. It is read from `slot.ideal_masked_witness_response`
   (4691-4693) — a value sampled ONCE at keygen
   (`reference_keygen` 4582: `sample_conditioned_quotient_fiber_representative` at 4593,
   stored at 4602). Sign just re-serializes the stored representative (4694-4697).

2. **The mask is a hard-coded constant, not a sampled mask.** Every response coordinate
   is built from `mask_view = [12_u16, Q - 12]` (4703), the SAME constant in every
   signature: `response = add_mod(mask_view[coord], challenge_mul(sign, public_view...))`
   (4731-4734). The same constant appears in `reference_keygen` (4584),
   `build_local_delta_response_openings_for_masked_response` (3263 via
   `mask_view_for_fiber_state`), and the probes. There is no uniform mask draw.

3. **No per-signature randomness anywhere.** `sample_proof_commitment()` (97) returns a
   compile-time CONSTANT `[7; ...]` (98). The challenge is a deterministic hash of
   (pk, message, slot_id, slot.digest, constant proof_commitment) (4711-4719). The seed
   driving the pad is read from inside the stored representative (4698-4702). Therefore
   the ENTIRE transcript is a deterministic function of (fixed stored representative,
   fixed mask, message). Re-signing slot s under message m always yields the identical
   bytes; signing under a different message reuses the SAME seed and SAME mask.

4. **The rejection predicate at sign time is NOT the certificate's predicate.** The only
   sign-time checks (4704-4709) are: fiber-state equality of the stored representative,
   and digest match. There is no "sample uniform kernel coefficients ... reject on
   canonical packing/fiber-state/response bounds" loop at sign time at all — that loop
   only ever ran once, at keygen. The deployed signer does not implement
   `production_sampler_law`.

## 4. The active pad outputs ARE conditioned into the live transcript

The signer emits openings (4745-4746):

    let pad = expand_pad_residues(&seed, block);
    openings.active_free[rep] = [mask_view[0] + pad[0], mask_view[1] + pad[1]];

and the verifier `verify_local_delta_relation_with_masked_witness` (4053) RECOMPUTES
the pad from the seed embedded in the proof's masked_witness_response (4064, 4080) and
rejects unless `openings == mask_view + pad` (4081-4084). The pad seed is part of the
hidden representative (`semantic_hidden_preimage_from_masked_witness`, 1355). So the
active-pad-output equations are checked INSIDE/against the hidden representative, exactly
the opposite of the required closure "checked outside the hidden representative sampler."

The codebase's own test proves this. `quotient_stateful_fixture_emits_honest_local_delta_trace`
(8872, PASSES) asserts for every round:
- `reconstructed_mask == mask_view` i.e. `== [12, Q-12]` (8902) — the mask is the fixed
  constant, recoverable from the public transcript via
  `response - challenge*pk` with no secret.
- `openings.active_free[rep] == [mask_view[0]+pad[0], mask_view[1]+pad[1]]` (8905-8908) —
  the openings literally publish the pad outputs (shifted by the public constant mask)
  for the challenge-selected block.

Thus `active_pad_outputs_conditioned` is `false` in the REPORT (2601) but effectively
TRUE in the deployed transcript law. The report value is a hand-set literal, not derived
from the signer.

## 5. No test enforces the obligation

The required closure (6464) asks for "tests that active pad equations are checked outside
the hidden representative sampler." Grep over the file finds the obligation only as a
report string; there is no test asserting pad-outside-sampler. The 28 unit tests check
acceptance/mutation/parse behavior, and one (8872) actively demonstrates the OPPOSITE
(pad bound inside the representative). So the discipline is asserted-in-a-report,
not-enforced-in-code, and not-tested. `quotient_hiding_sampler_review.active_pad_outputs_excluded`
(2700) is computed as `!sampler.active_pad_outputs_conditioned` = `!false` = `true`,
i.e. it "passes" only because a literal was set to false — it does not inspect the signer.

---

## 6. Concrete leak the discipline violation enables

Because the openings publish `mask + pad(seed, block)` with `mask = [12, Q-12]` a public
constant, an observer recovers `pad(seed, block) = opening - mask (mod q)` for every
challenge-selected block, directly, from one signature, with no key material. Two
independent leaks follow:

(a) Per-block pad output recovery. Each signature reveals `LOCAL_FREE * SIS_RESIDUE_BITS`
    bits of the pad stream on each distinct active block (the quantity
    `active_pad_constraint_bits`, 2949). These are outputs of `expand_pad_residues(seed,.)`,
    a deterministic PRF of the hidden seed inside the representative. The hiding claim is
    that the hidden fiber retains >= 140 bits of entropy; `accepted_fiber_constraint_report`
    (2952-2959) shows that once these pad equations are counted as constraints on the fiber
    (which the verifier forces), the remaining hidden-fiber entropy collapses
    (`estimated_remaining_fiber_bits_after_acceptance`, 2970; the report even computes
    `max_distinct_active_blocks_for_128_bits`, the point past which < 128 bits remain).
    The zero-TV claim, which never subtracted these bits, is therefore not the operative
    distribution.

(b) Multi-signature pad accumulation on a static representative. Because the representative
    and mask are fixed per slot and the only per-message variation is which blocks the
    challenge selects, signing the same slot under many messages reveals pad outputs on
    progressively more distinct blocks of the SAME seed. (Note: the lifecycle policy marks a
    slot consumed after one accepted signature, but slot-reuse safety is "specified but not
    implemented as durable storage" per the side-channel finding at 6469; and even within a
    single signature REPS rounds expose multiple blocks.) Each additional distinct active
    block subtracts another `pad_output_bits_per_active_block` from the hidden fiber, driving
    toward the over-determination the report itself warns about (2973). The certificate's
    "independent stateful slots compose with zero TV" (2703) assumes the per-slot law is the
    zero-TV ideal law; it is not, so the composition argument does not transfer.

The break is not a new lattice attack — it is that the certified distribution is the wrong
distribution. The hiding certificate proves perfect hiding of an IDEALIZED sampler whose
support excludes pad-output conditioning, while the deployed signer publishes a transcript
that is exactly the pad-output-conditioned distribution. Any downstream "witness-hiding"
guarantee that cites `QuotientWitnessHidingGameBound.advantage_is_zero` (2628) for REAL
transcripts is unsupported.

---

## 7. Classification and required redesign

Classification: **CONDITIONAL-GAP (leak path open).** The zero-TV precondition is
asserted in `MaskedResponseSamplerCertificate` but is FALSE for the production signer:
(i) the signer does not run `production_sampler_law` (it serializes a keygen-time fixed
representative with a constant mask, no per-signature sampling/rejection); (ii) the
verifier binds published openings to active pad outputs of the hidden seed, i.e. the
transcript IS conditioned on active pad outputs; (iii) no test enforces the
"active pad equations checked outside the hidden representative" obligation, and the one
trace test demonstrates the opposite.

To close (any of these is structural, not cosmetic):
1. Make `active_pad_outputs_conditioned` a value DERIVED from the signer/verifier, not a
   hand-set literal, and add a test that fails if the verifier recomputes pad from the
   hidden seed (it currently does, 4080).
2. Move pad/opening checks to an existential quotient-opening predicate OUTSIDE the hidden
   representative (the `pad_constraints_layer` the report names at 2602 but the code does
   not realize), OR sample a FRESH per-signature mask/representative so the published
   openings are not a deterministic PRF of a static hidden seed.
3. Recompute the hiding budget with `active_pad_constraint_bits` subtracted
   (`accepted_fiber_constraint_report`, 2939) — the operative residual is the
   pad-conditioned figure, not `residual_hiding_bits`.

Until then, the witness-hiding of real transcripts (WS4 #11) is NOT supported by the
existing theoretical certificate.
