# Path 2 Native Boundary Norm/Range Selection

Status: theorem statement selected.

Purpose: choose the concrete norm/range statement for the native-boundary
transparent path before updating payload byte estimates.

## Decision

Select the hybrid representation:

```text
public masked projection: raw canonical q12289 residues
hidden role openings: base-128 digit openings with reconstruction/range checks
```

This means:

- `p_public_projection(b)` / `masked_projection[6]` remains six canonical
  q12289 residues;
- hidden pad-source role values are represented as base-128 digits;
- hidden projection-relation role values are represented as base-128 digits;
- hidden opening-tuple values are represented as base-128 digits;
- range witnesses remain private and must satisfy the no-export policy;
- each digit must be in `[0, 127]`;
- digit reconstruction must equal the semantic q12289 residue used by R_core_66;
- modular wrap must reject or export `path2_sis_norm_or_range_break`.

## Why Not Raw Centered Residues

Raw centered q12289 residues keep the column count low, but they do not give a
clean binding hardness surface. Two canonical openings can differ by up to:

```text
q - 1 = 12288
```

per lane. That exports a kernel vector, but not one that is compellingly short
for the intended SIS/module-SIS binding claim unless we introduce a new,
awkward bounded-SIS assumption with a very large bound.

The native boundary fixes cross-object consistency. It does not make raw
q12289 deltas short.

## Why Not Full Digit Public Projection

Digitizing the public masked projection would add public/profile complexity
without much theorem value. The chain needs the public quotient as the active
six-lane q12289 value. The binding-sensitive objects are the hidden role
openings under `p_section_root(b)`, not the public residue itself.

So the quotient remains:

```text
p_public_projection(b) : Fin 6 -> F_q
```

while the extracted hidden representative carries digit witnesses proving how
the hidden role values reconstruct to that quotient relation.

## Selected Theorem Shape

```text
HybridBase128NormRange:

accepted transparent proof
compatible native boundary b
extracted h in Fiber(B_c, b)
  =>
  1. p_public_projection(b) is canonical q12289;
  2. pad_source role digits are base-128 and reconstruct to pad residues;
  3. projection_relation role digits are base-128 and reconstruct to account
     projection residues;
  4. opening_tuple role digits are base-128 and reconstruct to opening tuple
     semantic values;
  5. masked_projection_i = pad_i + account_projection_i mod q;
  6. all digit/range witnesses are private under NoExportSoundness;
  7. the SIS binding vector uses digit coordinates, not raw q12289 residues.
```

or a named break.

## Bound Surface

For each semantic residue represented by two base-128 digits:

```text
x = d0 + 128*d1
0 <= d0 < 128
0 <= d1 < 128
x canonicalizes to the intended q12289 residue
```

Two accepting digit openings differ by at most:

```text
|delta_digit| <= 127
```

per digit coordinate.

The existing cost probe gives:

| role | residues | base-128 columns | base-128 delta L2 |
|---|---:|---:|---:|
| pad_source | 6 | 12 | 439.941 |
| projection_relation | 6 | 12 | 439.941 |
| opening_tuple | 8 | 16 | 508.0 |

The total selected digit role footprint is:

```text
20 semantic residues -> 40 base-128 digit columns
```

This matches the existing masked-trace range floor that already budgeted 40
digit/range rows conservatively.

## Byte-Model Consequence

For now, keep the current transparent planning floor:

```text
range/account lane floor = 194
```

Do not lower it yet. The native-boundary refactor may remove some role-link
overhead, but the selected hybrid range theorem keeps the 40 base-128
digit/range lane assumption. The byte model should be updated only after the
concrete column profile says whether role-link lanes shrink.

## Required Column-Profile Delta

`Path2NativeBoundaryColumnProfileV1` must include:

- digit columns for `pad_source`;
- digit columns for `projection_relation`;
- digit columns for `opening_tuple`;
- reconstruction constraints from digits to semantic residues;
- private range-witness / range-check classes;
- no direct public openings for digit or range witnesses;
- masked/simulated query policy for any digit-derived public checks.

## Lean Surface

`Descender/Path2NativeBoundary.lean` should treat this as the selected
`NormRangeMode`:

```text
hybridBase128HiddenRoles
```

The existing `NormRangeSoundness` predicate remains abstract, but downstream
instantiations should target this selected mode.

## Dead / Parked Alternatives

| alternative | disposition | reason |
|---|---|---|
| raw centered residues for all roles | dead for clean binding | deltas up to `q-1`; requires awkward large-bound SIS claim |
| base-128 public projection plus hidden roles | parked | more public/profile complexity with little benefit |
| base-16 hidden roles | fallback | cleaner smaller digit deltas, but doubles digit columns and likely payload bytes |
| masked short lift | research-only | attractive if a natural descent lift is found, but not the current closure path |

## Next Proof Move

Update the column profile and Lean interface with the selected
`hybridBase128HiddenRoles` mode, then update the byte model only if the profile
changes the current `194`-lane planning floor.

## Non-Claims

- This does not prove the range gadget.
- This does not prove SIS hardness for the final dimensions.
- This does not instantiate query openings.
- This does not change spend signature bytes.
