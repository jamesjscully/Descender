# Path 2 Native Boundary Blinding/Noise Bound

Status: selected-policy theorem interface pinned; public-commitment bound path rejected.

Purpose: isolate the remaining SIS binding blocker after selecting hybrid
base-128 hidden-role digits.

## Core Issue

The selected digit-coordinate representation bounds the semantic part of the
exported SIS kernel vector:

```text
|delta_digit| <= 127
```

But each committed role vector also includes blinding/noise coordinates for
hiding:

| role | digit/semantic columns | blinding/noise columns | range/aux columns |
|---|---:|---:|---:|
| `pad_source` | 12 | 24 | 0 |
| `projection_relation` | 12 | 24 | 0 |
| `opening_tuple` | 16 | 32 | 8 |

If blinding/noise coordinates are too wide, the extracted kernel vector may not
be short enough for a clean SIS/module-SIS binding claim. If they are too
narrow, the commitment may fail hiding.

This is now the real bottleneck.

## Required Policy Object

```text
Path2NativeBoundaryBlindingNoiseBoundV1 = (
  policy_id,
  pad_source_blinding_bound,
  projection_relation_blinding_bound,
  opening_tuple_blinding_bound,
  range_auxiliary_bound,
  distribution_family,
  rejection_rule,
  hiding_claim
)
```

## Theorem Interface

```text
BlindingNoiseBoundSoundness:

accepted transparent proof
compatible native boundary b
compatible column profile
UsesSelectedBlindingNoisePolicy(policy)
extracted h in Fiber(B_c, b)
  =>
BlindingNoiseBounded(b, policy, h)
```

or one of:

- `path2_sis_norm_or_range_break`;
- `inner_commitment_binding_break`;
- `samplerHidingBreak`;
- `transparentProofSoundnessBreak`;
- `qrom_or_domain_separation_break`.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now provides:

- `Descender.Path2NativeBoundary.BlindingNoisePolicy`
- `Descender.Path2NativeBoundary.UsesSelectedBlindingNoisePolicy`
- `Descender.Path2NativeBoundary.Predicates.blindingNoiseBounded`
- `Descender.Path2NativeBoundary.BlindingNoiseBoundSoundness`
- `Descender.Path2NativeBoundary.blinding_noise_bounded_or_break`

`transparent_view_full_interface_or_break` now includes both:

```text
P.normRangeValid b h
P.blindingNoiseBounded b policy h
```

for the extracted hidden representative.

The active selected policy is pinned to policy id `1` and centered bound `2048`
for pad-source, projection-relation, opening-tuple, and range-auxiliary
blinding/noise coordinates. The Rust proof-family interface digest binds the
same policy id and `PATH2_DESCENT_SIS_BLINDING_CENTERED_BOUND`.

## Design Options

| option | binding | hiding | concern |
|---|---|---|---|
| narrow bounded blinding | easier SIS binding | may not hide role vectors statistically/computationally | could recreate full-rank leakage |
| wide blinding | better hiding | weaker SIS binding bound | may force larger row bands or different assumption |
| rejection-sampled bounded blinding | balances both | proof complexity rises | must prove simulator/no-export and rejection distribution |
| module commitment with separate hiding randomness | cleaner abstraction | changes commitment model | may affect bytes/rows |

## Next Move

Use the active decision in
`research/path2_native_boundary_blinding_noise_decision.md`: keep role
commitments private inside the ZK/two-view proof. Blinding/noise bounds are now
extractor-side bounds, not a public linear commitment hiding fix.

Next proof work should instantiate the native-boundary ZK trace
binding/hiding assumption rather than trying to rescue public linear role
commitments.

## Non-Claims

- This rejects public linear role commitments with current dimensions as the
  active hiding path.
- This does not prove hiding.
- This does not prove SIS hardness.
- This does not change the selected hybrid base-128 digit mode.
