# Product-Block Challenge Action

This note records the first formal step behind the small-signature
product-block Fiat-Shamir-with-aborts direction.

## Formalized Algebra

`Descender/SystematicLee.lean` now defines three block-local challenge actions:

- `blockScaleFree`: scale free/witness coordinates by a per-block scalar.
- `blockScalePivot`: scale pivot/check coordinates by the same per-block scalar.
- `blockScaleView`: scale the full systematic public-coordinate view blockwise.

For any block-diagonal systematic matrix

```text
P ((b, pivot), (b', free)) = Plocal b pivot free  if b = b'
P ((b, pivot), (b', free)) = 0                  otherwise,
```

Lean proves:

- `blockDiagonalSystematicPivotImage_blockScale`: the pivot image of a
  block-scaled witness equals the block-scaled pivot image.
- `blockScaleView_systematicRelation`: the full systematic relation commutes
  with block-local scaling.
- `blockScalePivot_systematicDualMap`: the verifier-facing dual map commutes
  with block-local scaling.

The scalar `chi b` is arbitrary in `ZMod q`.  For signatures, the intended
challenge coefficients are sparse signed values in `{-1, 0, 1}`, but the proof
is stronger than that.

## Formalized Noise Preservation

`Descender/ProductBlockSignature.lean` adds the verifier-error side for the
signature setting.  It defines:

- `SignedUnitOrZero`: a challenge scalar is `0`, `1`, or `-1`;
- `BlockSignedUnitOrZero`: every block challenge scalar is signed-or-zero.

Lean then proves:

- `centeredResidueNoiseSupport_blockScaleFree`
- `centeredResidueNoiseSupport_blockScalePivot`
- `centeredResidueNoiseSupport_blockScaleView`
- `blockScaleView_systematicRelation_add_error`
- `blockScaleView_noisySystematicRelation_preserves_error`
- `blockScaleView_noisySerializedSystematicRelation_preserves_error`

Together these show that a noisy public systematic relation with coordinate
error bounded by `eta` remains a noisy systematic relation with the same
coordinate error bound after applying a sparse signed product-block challenge.
The serialized version also handles a separate rounding-error term and preserves
the combined `eta + roundingEta` bound.  This is the correctness-side reason
the product-block response bound does not pay for a large scalar challenge.

## Formalized Distributional Proof

`Descender/SignatureDistribution.lean` adds the rejection-sampling
distributional theorem.  Given a `RejectionSamplingCertificate` saying that
every accepted-response test has mass
`acceptance * target.acceptProb test`, Lean proves that the conditional
accepted-response distribution equals the target distribution and is
secret-independent.  The product-block specialization applies this theorem to
the linear response `z = y + chi * s`.

This completes the abstract distributional proof shape.  The remaining work is
to instantiate the certificate for a concrete mask distribution and rejection
predicate, or to replace the exact certificate with an explicit statistical
distance bound.

`signature_sampler_analysis.rs` now gives the first concrete instantiation: an
enlarged uniform-box mask with exact target-box rejection.  It shows that the
accepted-test-mass equation is achievable, but also corrects the earlier size
screen by charging rejection over every active response coordinate.

`Descender/BlockChallengeSoundness.lean` adds the deterministic extraction
target for sparse block challenges.  If forked challenge deltas cover every
needed block, the extracted product-block secret equals the real secret.  The
file also proves the response-difference identity used by the extractor and
records the compact active-only condition: opposite signed active challenges
give an extractable delta when `2` is invertible.  The deterministic multi-fork
version is now included: several accepting challenge/response draws sharing one
commitment/mask recover the whole secret when compact opposite-sign coverage
holds.

## Signature Meaning

The proof discharges the main algebraic objection to a product-block sparse
challenge: if the public view is block-diagonal systematic, then the verifier
can apply a block-local challenge directly to the public coordinates without
knowing the secret.

This is the structural reason the size model can use many challenge bits from
choosing signed block subsets while keeping each touched secret/noise coordinate
multiplied only by `1` in absolute value.  It avoids the scalar-challenge
bottleneck where challenge entropy comes from a large scalar and therefore
inflates the response bound.

## Remaining Gaps

This is not yet a complete signature scheme.  The still-open proof obligations
are:

- the exact bit-level serialized/high-bit transcript equation, including how
  rounding errors are encoded and decoded;
- a probabilistic multi-fork Fiat-Shamir proof/game that justifies enough
  accepting challenge draws per repeated commitment for compact fork-delta
  coverage;
- transcript binding for the challenge derivation and repeated commitments;
- concrete challenge encoding and canonical serialization for the block subset
  and signs;
- cryptanalysis of the key-generation structure required to expose the
  block-diagonal systematic view.

## Consequence

The `212 B` to `248 B` rows in `aborts_size_search.md` should no longer be
described as blocked by a missing equivariance lemma.  They should also not be
treated as final concrete sampler sizes: the exact sampler reports move the
full-block targets to about `304 B` for `n = 32` and `496 B` for `n = 48`.
Compact fork-delta accounting with six challenge draws gives intermediate
targets of about `255 B` for `n = 32` and `415 B` for `n = 48` above the
`24`-bit line.  The deterministic multi-fork extractor now matches that metric.
The finite-probability forking shell also matches that metric by proving
extractor success at least `forgerSuccess - forkingLoss -
coverageFailureBound`.  The random-oracle replay refinement splits
`forkingLoss` into `queryMissBound + replayFailureBound`.  The next blocker is
the concrete transcript-specific bounds for those two terms plus the serialized
verifier equation.
