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

- a rejection-sampling theorem showing that accepted responses are statistically
  independent of the secret;
- the exact bit-level serialized/high-bit transcript equation, including how
  rounding errors are encoded and decoded;
- a Fiat-Shamir proof/game with challenge derivation, rewinding or forking
  analysis, and transcript binding;
- concrete challenge encoding and canonical serialization for the block subset
  and signs;
- cryptanalysis of the key-generation structure required to expose the
  block-diagonal systematic view.

## Consequence

The `212 B` to `248 B` rows in `aborts_size_search.md` should no longer be
described as blocked by a missing equivariance lemma.  They are still
research candidates, but the next blocker has moved to the signature proof
itself: rejection sampling plus the serialized Fiat-Shamir verifier equation.
