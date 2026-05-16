# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `331`
- top rows per profile: `300`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-192 | 331 | 331 | 260 B | 228 B |

## First Passing Rows

| profile | source | proof | primal model | q | n | m0 | m1 | view | k | eta | pk | ct | correctness distance | lossy diag req | lossy diag slack | bounded dual | view route | verdict |
|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| descender-192 | accepted_top | full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 24 | 4 | 285 B | 253 B | 25 | 53 | -357.2 | 3 | full-view | pass |

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Increase bounded dual-witness search depth on the smallest frontier rows.
3. Add lattice/code attack estimator costs for the passing rows.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
