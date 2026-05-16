# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `216`
- top rows per profile: `64`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-128 | 75 | 75 | 224 B | 192 B |
| descender-192 | 71 | 71 | 304 B | 272 B |
| descender-256 | 70 | 70 | 384 B | 352 B |

## First Passing Rows

| profile | source | q | n | m0 | m1 | k | eta | pk | ct | guard distance | bounded dual | verdict |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| descender-128 | accepted_top | 32771 | 32 | 48 | 32 | 24 | 4 | 224 B | 192 B | 17 | 2 | pass |
| descender-192 | accepted_top | 32771 | 48 | 72 | 48 | 32 | 3 | 304 B | 272 B | 25 | 2 | pass |
| descender-256 | accepted_top | 32771 | 64 | 96 | 64 | 48 | 3 | 384 B | 352 B | 33 | 2 | pass |

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Increase bounded dual-witness search depth on the smallest frontier rows.
3. Add lattice/code attack estimator costs for the passing rows.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
