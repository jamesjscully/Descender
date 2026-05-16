# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `102`
- top rows per profile: `32`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-128 | 36 | 36 | 201 B | 169 B |
| descender-192 | 34 | 34 | 262 B | 230 B |
| descender-256 | 32 | 32 | 316 B | 284 B |

## First Passing Rows

| profile | source | q | n | m0 | m1 | k | eta | pk | ct | guard distance | bounded dual | verdict |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| descender-128 | accepted_top | 32771 | 32 | 52 | 32 | 24 | 2 | 201 B | 169 B | 21 | 2 | pass |
| descender-192 | accepted_top | 16411 | 56 | 88 | 56 | 32 | 2 | 262 B | 230 B | 33 | 2 | pass |
| descender-256 | accepted_top | 65537 | 64 | 104 | 64 | 48 | 2 | 316 B | 284 B | 41 | 2 | pass |

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Increase bounded dual-witness search depth on the smallest frontier rows.
3. Add lattice/code attack estimator costs for the passing rows.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
