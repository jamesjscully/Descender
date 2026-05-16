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

The hypothetical product-block challenge route is much more interesting.  The
size model finds rows around `212 B` to `248 B` with practical retry cost, but
these are not yet schemes.  They require the public viewed relation to support
a block-local challenge action that the verifier can apply to the public sample
without knowing the secret.

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
The next signature-specific work is the distributional rejection-sampling proof
and serialized Fiat-Shamir verifier equation rather than another
parameter-search filter.
