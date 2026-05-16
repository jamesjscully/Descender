# Signature Readiness Cryptanalysis

This pass tests whether the 172-byte coordinate-subview KEM/public-view
candidate should immediately be used as a signature primitive.

## Target

The default probe targets the current leading 128-bit coordinate-subview row:

- proof profile: `Descender128CoordinateSubviewProfile.serialization`
- `q = 12289`
- `n = 48`
- `m0 = 80`
- `m1 = 32`
- public view coordinates: `80`
- secret/noise radius used by the row: `eta = 12`
- public key size: `172 B`
- ciphertext size: `172 B`

## Probe Model

The implemented probe attacks the most direct Fiat-Shamir-style signing idea:

```text
z = y + c*s mod q
```

where `s` is the small secret, `c` is a public scalar challenge, and `y` is a
bounded signing mask.  Given several transcripts `(c, z)`, the attacker
enumerates each possible small secret coordinate and scores whether the residual
`z - c*s'` lies inside the mask interval.

This is not an attack on the KEM/public-view candidate.  It is an attack on a
naive signature transcript relation that exposes linearly masked small secrets.

## Result

The direct scalar-response design is not viable.  With `secret_eta = 12`, the
coordinate-wise maximum-likelihood attack recovers the secret quickly:

- mask bound 12, 32, or 64: usually broken with one transcript;
- mask bound 128 or 256: essentially broken by two transcripts;
- mask bound 1024: broken by four to eight transcripts;
- mask bound 2048: broken by eight transcripts.

The problem is structural: each coordinate of `s` has only 25 possible values,
and the transcript gives a public noisy equation for that coordinate.

## Abort-Size Search

The follow-up size search asks whether Fiat-Shamir-with-aborts can still be
small if the response distribution is made independent of the secret by
rejection sampling.

Two challenge models were compared:

1. **Scalar challenges.**  These are compatible with the current plain linear
   public relation because the verifier can compute `c * t`.
2. **Product-block sparse challenges.**  These are hypothetical.  They assume a
   new block-local challenge action with roughly 16 product blocks and signed
   sparse challenges, so each challenge has high entropy while each affected
   secret coefficient is still multiplied only by `-1`, `0`, or `1`.

The scalar route is not obviously impossible, but it is not tiny.  The best
screened rows are around `1124 B` with roughly `2^13` to `2^18` expected retry
cost, depending on the scalar bound.  This is likely above the target if small
signatures are the main goal.

The product-block challenge route is much more interesting.  The original
size model found rows around `212 B` to `248 B`, but that was a proof-shape
screen: it did not charge the rejection exponent over every active response
coordinate.  The concrete exact-box sampler analysis gives a sharper picture:
for `n = 48`, the clean full-block target is around `496 B`; for the `n = 32`
candidate, the clean full-block target is around `304 B`.  Intermediate rows
are smaller, but they must now be judged by fork-delta coverage, not just by
whether a single challenge touches each block.  With six challenge draws per
repeated commitment, the compact active-only transcript has a `255 B` `n = 32`
row above `24` bits of exact any-block fork-delta coverage.

This is the most promising small-signature research target found so far.

## Product-Block Challenge Action

The first algebraic blocker for the product-block route is now discharged for
block-diagonal systematic views.  `Descender/SystematicLee.lean` defines
`blockScaleFree`, `blockScalePivot`, and `blockScaleView`, then proves:

- `blockDiagonalSystematicPivotImage_blockScale`
- `blockScaleView_systematicRelation`
- `blockScalePivot_systematicDualMap`

Together these say that block-local challenge scaling commutes with the
systematic public relation and with the verifier-facing dual map.  In signature
terms, the verifier can apply a sparse signed product-block challenge directly
to the public view, while the response bound only sees coefficients in
`{-1, 0, 1}` instead of a large scalar challenge.

This does not complete the signature proof.  The remaining blockers are now the
rejection-sampling theorem, the serialized/high-bit verifier equation,
Fiat-Shamir soundness, concrete challenge serialization, and key-generation
cryptanalysis for the structured view.  See
`product_block_challenge_action.md`.

`Descender/ProductBlockSignature.lean` also proves the first correctness-side
lemma for this route: sparse signed block challenges preserve the centered
coordinate noise bound, and a noisy systematic public relation remains noisy
systematic after challenge application with the same `eta`.  The theorem also
has a serialized-error form that preserves the combined `eta + roundingEta`
bound when rounding is represented as an additional bounded error term.

## Distributional Rejection Proof

`Descender/SignatureDistribution.lean` now factors the Fiat-Shamir-with-aborts
distributional proof.  A `RejectionSamplingCertificate` states that every
accepted-response test has unnormalized mass
`acceptance * target.acceptProb test`; Lean proves from this that:

- the acceptance mass is independent of the secret and challenge;
- the conditional accepted-response distribution equals the target distribution;
- accepted responses, and any deterministic compact encoding of them, are
  secret-independent.

For product-block signatures this is specialized to the response
`z = y + chi * s` via `productBlockLinearResponse` and
`productBlockRejectionSampler`.  See `distributional_signature_proof.md`.

## Concrete Sampler Analysis

`tools/parameter-search/src/bin/signature_sampler_analysis.rs` analyzes an
exact finite sampler: for each active coordinate with shift `|v| <= eta`, it
samples `y` uniformly from `[-z-eta,z+eta]`, emits `y + v`, and accepts exactly
when the response lies in `[-z,z]`.  This exactly realizes a
secret-independent uniform target law on `[-z,z]`.

The key correction is that rejection cost is paid per active response
coordinate.  This makes very sparse challenges look tiny but weak in
block-coverage soundness.  The current reports distinguish three coverage
notions:

- active challenge coverage: a single challenge touches a block;
- full-response fork-delta coverage: an idealized transcript exposes all
  response coordinates, so zero-vs-nonzero forks can extract;
- compact fork-delta coverage: the actual small transcript exposes only active
  coordinates, so extraction only counts opposite signed active openings.

The compact fork-delta metric is the relevant one for small signatures.  The
current reports use six challenge draws for the same commitment per repeated
round:

- `sampler_analysis_n48.md`: best full-block row `496 B` with log2 retry
  `14.63`; best compact fork-delta `>=24` row `415 B`; best compact
  fork-delta `>=16` row `281 B`.
- `sampler_analysis_n32.md`: best full-block row `304 B` with log2 retry
  `19.23`; best compact fork-delta `>=24` row `255 B`; best compact
  fork-delta `>=16` row `197 B`.

Sizes use canonical challenge encoding.  Carrying a fixed 32-byte challenge
digest instead adds about `13 B` to `16 B` in these rows.

## Block-Challenge Soundness

`Descender/BlockChallengeSoundness.lean` now formalizes the deterministic
extraction target behind those coverage rows.  It proves that if repeated
challenges cover every needed block, local active-block extractors assemble a
globally valid block witness.  It also proves the response-difference identity
and forked extraction theorem needed for Fiat-Shamir analysis:

```text
(y + chi0 * s) - (y + chi1 * s) = (chi0 - chi1) * s.
```

For compact active-only transcripts, the formal boundary also records that
opposite signed active challenges give an extractable delta when `2` is
invertible.  The multi-fork boundary is now formalized too: several accepting
draws for the same commitment/mask recover the whole product-block secret if
their compact opposite-sign deltas cover every block.  For the concrete report
modulus, Lean records `(6145 : ZMod 12289) * 2 = 1`.  The sampler reports
compute exact any-block-missed probabilities for both active coverage and
compact fork-delta coverage.  See `block_challenge_soundness.md`.

## Pay-To-Hash

Hash-only authorization can be tiny, but it is not a reusable signature scheme.
For example, a blockchain account can commit to `H(secret)` and authorize once
by revealing `secret`, giving a very small witness.  That is a stateful one-time
spend primitive, not a general signature:

- the secret must not be reused;
- the transaction format must bind the preimage to the intended spend to avoid
  theft/front-running;
- account state must rotate after every spend;
- it does not give ordinary reusable message signatures.

Pay-to-hash may be viable as a blockchain-specific mode, but it is separate
from a reusable Descender signature scheme.

## Signature Constraints

A signature construction should not expose responses of the form
`small mask + public challenge * small secret` unless it has a full
Lyubashevsky/Dilithium-style rejection-sampling proof with masks large enough
to make the response distribution statistically independent of `s`.

More promising directions:

1. A ZK/MPC proof of knowledge for the public-view relation, where linear
   masked secrets are not exposed directly.
2. A lattice-signature design with a large, carefully analyzed masking
   distribution and rejection sampling, treated as a separate signature scheme
   rather than a simple wrapper around the KEM row.
3. A hash-and-sign route only if we can construct and analyze a trapdoor sampler
   for the structured public view, which is not available yet.

## Current Position

The 172-byte row remains the leading compact KEM/public-view candidate.  It is
not yet a safe signature primitive.  The smallest plausible reusable-signature
route is now a product-block Fiat-Shamir-with-aborts design.  The
verifier-computable block challenge action is formalized for block-diagonal
systematic views, and the basic signed-challenge noise bound is formalized.
The distributional rejection-sampling theorem is also formalized as a generic
finite-probability proof.  The exact-box sampler, compact fork-delta coverage
analysis, and deterministic multi-fork extractor are now concrete.  The next
signature-specific work is the probabilistic Fiat-Shamir forking lemma that
produces six accepting draws for one commitment with a quantified loss, plus
the serialized verifier equation.
