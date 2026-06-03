# Path 2 Native Boundary Hybrid Base-128 SIS Sanity

Status: first-pass sanity comparison complete; not a hardness estimate.

Purpose: compare the selected hybrid base-128 hidden-role norm surface against
the rejected raw-centered q12289 surface before any byte-model update.

## Inputs

- modulus: `q = 12289`
- row bands: `48`, `64`
- selected norm/range mode: `hybridBase128HiddenRoles`
- digit bound: `|delta_digit| <= 127`
- public masked projection: raw canonical q12289 residues
- hidden role openings: base-128 digits

## Role Bounds

| role | raw semantic residues | raw delta Linf | raw semantic delta L2 | selected digit columns | selected digit Linf | selected digit L2 |
|---|---:|---:|---:|---:|---:|---:|
| `pad_source` | 6 | 12288 | 30099.33 | 12 | 127 | 439.94 |
| `projection_relation` | 6 | 12288 | 30099.33 | 12 | 127 | 439.94 |
| `opening_tuple` | 8 | 12288 | 34755.71 | 16 | 127 | 508.00 |

The selected digit-coordinate surface reduces the semantic part of the exported
kernel vector by about:

```text
30099.33 / 439.94 ~= 68.4x
34755.71 / 508.00 ~= 68.4x
```

for the roles above.

## Column Impact For Role Commitments

The old role-vector feasibility surface used raw semantic lanes:

| role | old columns | selected semantic/digit change | selected planning columns |
|---|---:|---:|---:|
| `pad_source` | 30 | 6 raw -> 12 digits | 36 |
| `projection_relation` | 30 | 6 raw -> 12 digits | 36 |
| `opening_tuple` | 48 | 8 raw -> 16 digits | 56 |

Total for the three main role vectors:

```text
old:      108 columns
selected: 128 columns
delta:    +20 columns
```

This `+20` is a role-commitment binding-surface change. It does not
automatically change the transparent payload byte floor because the current
194-lane transparent planning floor already carried a conservative 40-lane
digit/range allowance. A concrete profile/proof family may still change bytes.

## Row-Band Read

The selected row bands remain:

| row band | old verifier mul-adds at 108 cols | selected verifier mul-adds at 128 cols |
|---:|---:|---:|
| 48 | 5184 | 6144 |
| 64 | 6912 | 8192 |

This is still small verifier arithmetic for account creation. The real question
is not mul-add count; it is whether the role-separated SIS/module-SIS binding
assumption is credible under the selected digit-coordinate bound and whatever
bound is assigned to blinding/noise columns.

## Sanity Result

| candidate | result | reason |
|---|---|---|
| raw centered q12289 for hidden roles | rejected as binding surface | exported semantic deltas are up to `q-1`, producing an awkward large-bound SIS claim |
| hybrid base-128 hidden roles | selected for theorem path | keeps public quotient compact while making the semantic part of the SIS witness digit-bounded |
| base-16 hidden roles | parked fallback | smaller digit bound, but doubles digit columns again |

## Remaining Hardness Blocker

This sanity pass fixes the semantic/digit coordinate bound, but not the whole
SIS vector bound. The commitment vector also includes blinding/noise columns:

- `pad_source`: 24 blinding/noise columns;
- `projection_relation`: 24 blinding/noise columns;
- `opening_tuple`: 32 blinding/noise columns plus range auxiliaries.

Before an SIS/module-SIS hardness statement is meaningful, we still need:

```text
BlindingNoiseBound:
  exact integer bound and distribution for all blinding/noise coordinates
```

Then the real estimator/sanity input is:

```text
role, rows, total columns, digit bound, blinding bound, range bound
```

## Next Move

Define `Path2NativeBoundaryBlindingNoiseBoundV1`:

- per-role blinding/noise coordinate bounds;
- whether blinding deltas are short enough for binding extraction;
- whether hiding requires a wider blinding distribution than binding can
  tolerate;
- fallback if binding and hiding bounds conflict.

## Non-Claims

- This is not a lattice-estimator result.
- This does not prove SIS hardness.
- This does not choose blinding/noise bounds.
- This does not update transparent payload bytes.
