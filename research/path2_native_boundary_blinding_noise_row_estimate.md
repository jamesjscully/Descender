# Path 2 Native Boundary Blinding/Noise Row Estimate

Status: selected-policy row estimate pinned; exact bound gadget pending.

Purpose: close the `r_blinding_noise_bounds` estimate in
`NativeBoundaryTraceRelationV1`.

## Background

The active path rejected public full-rank linear role commitments. Blinding and
noise are now extractor-side trace coordinates:

```text
BlindingNoiseBounded(b, policy, h)
```

They are not public hiding commitments.

The active theorem and Rust interface now bind this extractor-side policy to
policy id `1` with centered bound `2048` for pad-source,
projection-relation, opening-tuple, and range-auxiliary coordinates. A
different policy must change the active proof-family interface digest.

## Required Checks

The relation must prove enough bound information so extracted blinding/noise
coordinates cannot destroy the SIS/binding theorem:

| role | semantic columns | blinding/noise columns | bound check need |
|---|---:|---:|---|
| `pad_source` | 12 | 24 | bound aggregate or per-block policy |
| `projection_relation` | 12 | 24 | bound aggregate or per-block policy |
| `opening_tuple` | 16 | 32 | bound aggregate plus range auxiliary policy |
| `range_auxiliary` | 8 | policy-dependent | bound only if used by selected range gadget |

## Estimate

Keep:

```text
r_blinding_noise_bounds = 8
```

Candidate decomposition:

| subgroup | rows | meaning |
|---|---:|---|
| `pad_source_blinding_bound` | 2 | bound pad-source blinding/noise aggregate |
| `projection_relation_blinding_bound` | 2 | bound projection blinding/noise aggregate |
| `opening_tuple_blinding_bound` | 2 | bound opening tuple blinding/noise aggregate |
| `range_auxiliary_bound` | 2 | bound range auxiliary / rejection-sampling witness |

## Why Not More Rows?

The estimate assumes the proof family supplies a compact range/bound gadget for
private coordinates. If every blinding coordinate needs independent base-128 or
bit decomposition rows, this estimate is too low.

The active theorem only needs extractor-side boundedness for binding. It does
not need to serialize these values or prove they hide public commitments.

## Failure Modes

The estimate fails if:

- the selected proof family has no compact private bound gadget;
- blinding/noise coordinates require public openings;
- binding requires per-coordinate range checks for all 80 blinding/noise lanes;
- rejection sampling distribution must be proven inside the relation rather
  than handled by the proof-family simulator theorem.

## Decision

Keep:

```text
r_blinding_noise_bounds = 8
```

conditional on:

```text
compact_private_bound_gadget
UsesSelectedBlindingNoisePolicy(policy)
```

Stress estimate still allows this row group to expand before crossing the
220-row watch threshold.

## Non-Claims

- This does not prove the bound gadget.
- This does not prove hiding.
- This does not choose the blinding/noise distribution.
