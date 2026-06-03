# Distributional Signature Proof

This note records the formal distributional proof boundary for the
product-block Fiat-Shamir-with-aborts signature route.

## Formalized Theorem

`Descender/SignatureDistribution.lean` defines a finite rejection-sampling
interface:

- `RejectionSampler`: a mask distribution, a response function, an accept
  predicate, and a target response distribution;
- `acceptedTestMass`: the unnormalized mass of accepted responses passing an
  arbitrary Boolean test;
- `conditionalAcceptedProb`: the accepted-response probability after dividing
  by the acceptance mass;
- `RejectionSamplingCertificate`: the exact condition that every accepted
  response test has mass `acceptance * target.acceptProb test`.

Lean proves:

- `acceptedMass_eq_acceptance`: the abort probability is independent of the
  secret and challenge;
- `conditionalAcceptedProb_eq_target`: the conditional accepted-response
  distribution equals the target distribution;
- `secret_independent`: accepted responses reveal no information about the
  secret;
- `challenge_independent`: the fixed-challenge accepted distribution is also
  common across challenges when the same target is used;
- `encoded_secret_independent`: any deterministic response encoding or compact
  transcript projection remains secret-independent.

For leakage-bounded samplers, Lean also defines
`RejectionSampler.LeakageBoundedCertificate`.  This certificate supplies a
finite leakage budget `leakage` and proves that every Boolean accepted-response
test distinguishes two secrets by at most that budget.  Lean proves:

- `responseAdvantage_le`: every response distinguisher is bounded by
  `leakage`;
- `encoded_responseAdvantage_le`: deterministic response encodings, including
  byte serialization, inherit the same bound;
- `secret_independent_of_zero_leakage`: the bounded theorem collapses back to
  exact secret independence when `leakage = 0`.

For product-block signatures, the file also defines the concrete linear
response shape

```text
z = y + chi * s
```

as `productBlockLinearResponse`, and packages it as
`productBlockRejectionSampler`.  The theorem
`productBlock_accepted_response_secret_independent` is the distributional
statement we need for sparse signed product-block challenges.

## What This Proves

If a concrete masking distribution and rejection predicate satisfy the
`RejectionSamplingCertificate`, then the accepted signature response has the
same distribution for every secret.  Therefore the transcript does not leak the
small secret through the linear response `z = y + chi * s`.

This is the formal version of the Lyubashevsky/Dilithium-style rejection
sampling argument, stated in the repo's lightweight finite-probability
framework.

## What Remains

The proof is now factored correctly, and the first concrete exact sampler is
analyzed in `sampler_analysis_n48.md` and `sampler_analysis_n32.md`.  The
sampler uses an enlarged uniform box for the mask and accepts exactly when the
shifted response falls in the target box.  This satisfies the exact
accepted-test-mass equation coordinatewise.

The reports now also include a leakage-bounded sampler screen.  This is a
separate route from the exact `RejectionSamplingCertificate`: the signer may
emit valid responses whose law is not identical for every secret, but the
finite response distributions are compared directly by total variation and KL
divergence, then charged over every serialized response coordinate.  The fast
interior shifted-uniform sampler fails this screen because worst-case shifted
supports give saturated product TV and unsupported KL.  Conditioned shifted
discrete-Gaussian rows are the first plausible bounded-leakage family, but any
use of them must add a theorem that consumes the concrete leakage budget and
hides or bounds secret-dependent retry timing.

That theorem surface is now present in Lean as
`productBlock_accepted_response_leakage_bounded` and
`productBlock_encoded_response_leakage_bounded`.  A concrete conditioned
Gaussian signer must instantiate the certificate from the finite report row;
after that, every verifier-facing serialized transcript distinguisher is
bounded by the row's charged leakage budget.

The finite report row is no longer only a Markdown table.  The checked-in
artifacts `conditioned_gaussian_leakage_certificate_n32.json` and
`conditioned_gaussian_leakage_certificate_n48.json` record the selected rows,
metric definitions, and rational interval proof rows.  Each proof row carries
a 20-bit dyadic integer weight table for the actual quantized sampler
distribution and a rational interval upper bound on total-variation leakage.
The command `signature_sampler_analysis --verify-certificate <path>` parses
each artifact, reconstructs the search configuration, recomputes the finite
shifted-Gaussian table metrics, checks that the supplied integer weights are
the canonical 20-bit dyadic Gaussian table for the row parameters, and
independently recomputes the rational TV bound from those weights.  These
rational interval rows are the intended concrete inputs to the Lean
`LeakageBoundedCertificate` hypothesis.

The bridge to Lean is now explicit.  After verification, the same tool can
emit `Descender/Generated/ConditionedGaussianLeakageN32.lean` and
`Descender/Generated/ConditionedGaussianLeakageN48.lean`.  These generated
modules record the source JSON SHA-256 and import the verified rational
interval constants into Lean as `DyadicGaussianLeakageProofRow` values.  The
common module `Descender.SignatureLeakageCertificate` supplies the Lean shape
of the rows and the theorem that specializes a
`LeakageBoundedCertificate.leakage` hypothesis to one of these checked
signature-leakage constants.

The executable full-response reference verifier has a separate checked row:
`current_verifier_conditioned_gaussian_leakage_certificate_n32_w15_r7.json`.
It matches the current verifier's active challenge profile (`block_weight=15`,
`repetitions=7`, `210` active coordinates, `z=224`, `sigma=896`).  The current
byte format still carries inactive response coordinates; those coordinates are
secret-independent because the challenge coefficient is zero.  The Rust signer
mode `--reference-certified-dyadic` uses exactly that canonical dyadic table
and rejects outside the certified active-response box.  This closes the
sampler implementation link for the current verifier profile, while the
smaller compact `block_weight=1` row still requires the compact
verifier/signature format before it can be claimed as the executable scheme.

The compact target checklist now lives in `compact_signature_todos.md`.  The
first concrete implementation step is
`signature_q12289_compact_format.rs`: it fixes the 69-byte structural payload
for the `block_weight=1`, `repetitions=26` target and reconstructs active
local commitments.  Its default Fiat-Shamir check now uses completed-response
implicit commitment reconstruction:
`17 B challenge || 728 B completed responses = 745 B`.  This avoids the
active-only fixed-point problem because the verifier reconstructs and hashes
the full prechallenge commitment view.  The older
`17 B challenge || 52 B active responses || 3640 B commitments = 3709 B`
explicit baseline remains available only as a comparison mode.

The remaining proof obligation is not distributional secrecy.  It is
block-soundness and extraction for sparse product-block challenges.  Low-weight
challenges produce very small signatures, but the extractor needs opposite
signed active forks for each block when the transcript omits inactive response
coordinates.  The concrete reports therefore separate:

- entropy-and-retry rows, which can be extremely small but have weak compact
  fork-delta diagnostics;
- intermediate compact fork-delta rows, such as `255 B` for the `n = 32`
  profile at `27.33` exact any-block compact fork-delta bits;
- full-block rows, such as `304 B` for the `n = 32` profile and `496 B` for
  the `n = 48` profile, both at `36` compact fork-delta bits with six
  challenge draws per repeated commitment.

For further size reductions, the likely route is a sharper soundness theorem
for multi-fork compact extraction, not a more conservative sampler.

## Current State

The product-block route now has three formal layers:

1. block-local challenge equivariance for the public systematic relation;
2. signed-challenge error preservation for noisy and serialized views;
3. rejection-sampling distributional secrecy, conditional on the accepted mass
   matching a secret-independent target law;
4. a concrete exact-box sampler analysis that instantiates the target law and
   quantifies response size versus retry cost;
5. a deterministic forked block-extraction theorem, plus exact
   inclusion-exclusion accounting for active coverage and compact fork-delta
   failure.
6. a deterministic multi-fork extraction theorem: several accepting draws for
   one commitment/mask recover the whole product-block secret when compact
   opposite-sign coverage holds.
7. a finite-probability Fiat-Shamir multi-fork shell proving
   `Pr[extractable] >= forgerSuccess - forkingLoss -
   coverageFailureBound`, conditional on an external bound for producing
   accepting same-commitment forks.
8. a random-oracle replay refinement proving
   `Pr[acceptingForks] >= Pr[forgery] - Pr[queryMiss] -
   Pr[replayFailure]`.
9. a six-draw replay certificate reducing the open replay loss to one
   challenge-guess bound and six per-draw replay-failure bounds:
   `Pr[extractable] >= Pr[forgery] - challengeGuessBound -
   sum drawFailureBound - coverageFailureBound`.
10. a leakage-bounded distribution theorem for conditioned samplers: if the
    finite sampler row supplies a rational interval leakage certificate,
    deterministic response serialization preserves that exact leakage budget.

The remaining blocker is no longer the sampler distribution.  It is whether an
intermediate compact fork-delta row can be connected to concrete
`drawFailure` bounds for six accepting draws, or whether we must pay for
full-block responses.  For the current `n = 32`, `255 B` row, the query-miss
target is already structurally identified as the `2^-133` blind challenge
guess term once the canonical challenge serializer is fixed.
