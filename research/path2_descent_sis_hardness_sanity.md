# Path 2 Descent/SIS Hardness Sanity

Status: executable sanity blocker pinned; not a hardness estimate.

## Scope

This artifact records the active hybrid base-128 hidden-role layout used by the
Path 2 descent relation. It supersedes the old raw-residue `30/30/48` role
layout for binding-surface analysis.

Executable anchor:

```text
tools/parameter-search/src/production_reference.rs
KAT: path2_descent_sis_hardness_sanity_rejects_wide_blinding_kernel_surfaces
```

## Active Role Layout

| role | base-128 semantic digit columns | blinding columns | range/aux columns | total columns |
|---|---:|---:|---:|---:|
| `pad_source` | 12 | 24 | 0 | 36 |
| `projection_relation` | 12 | 24 | 0 | 36 |
| `opening_tuple` | 16 | 32 | 8 | 56 |
| `transcript_commitment` | 40 | 80 | 8 | 128 |

Norm/range policy:

- semantic digit columns are nonnegative and at most `127`;
- range/aux digit columns are nonnegative and at most `127`;
- blinding columns are centered and at most `2048` per opening, so an extracted
  double-opening delta can be as large as `4096` in each blinding coordinate;
- digit reconstruction back to canonical q12289 residues is a relation/proof
  obligation, not a norm-only check.

## Rank And Kernel Read

The KAT computes the exact role matrix rank over q12289 for the active 48/64 row
bands.

| role | rows | columns | rank | kernel dimension | sanity verdict |
|---|---:|---:|---:|---:|---|
| `pad_source` | 48 | 36 | 36 | 0 | full-column-rank private-only surface |
| `projection_relation` | 64 | 36 | 36 | 0 | full-column-rank private-only surface |
| `opening_tuple` | 48 | 56 | 48 | 8 | bound too wide for short-SIS surface |
| `opening_tuple` | 64 | 56 | 56 | 0 | full-column-rank private-only surface |
| `transcript_commitment` | 48 | 128 | 48 | 80 | bound too wide for short-SIS surface |
| `transcript_commitment` | 64 | 128 | 64 | 64 | bound too wide for short-SIS surface |

## Interpretation

This exposes a real theorem-design constraint:

- Full-column-rank role matrices can give uniqueness if kept private inside a
  ZK/transparent relation, but public `C = A*v` would still reveal the opening.
- Kernel-bearing surfaces cannot currently be justified by a clean short-SIS
  assumption with blinding bound `2048`; the accepted delta bound is dominated
  by blinding coordinates.
- The aggregate transcript commitment should not be used as the cryptographic
  binding theorem for the active deployable path unless the blinding model,
  row count, or commitment design changes.

## Next Move

For the active transparent payload route, use this result to keep binding inside
the proof-system/native-boundary trace authentication layer. Do not promote a
public aggregate SIS commitment, and do not claim short-SIS binding for the
48-row opening-tuple surface under the current blinding bound.

The next proof work is role-restriction/authentication and transparent
proof-family knowledge soundness over the corrected hybrid base-128 relation.

## Non-Claims

- This is not a lattice-estimator result.
- This does not prove SIS hardness or insecurity.
- This does not close the transparent proof extractor row.
- This does not change spend signature bytes.
