# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `200`
- top rows per profile: `200`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-128 | 200 | 0 | - | - |

## First Passing Rows

| profile | source | proof | primal model | q | n | m0 | m1 | view | k | eta | pk | ct | correctness distance | bounded dual | view route | verdict |
|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Increase bounded dual-witness search depth on the smallest frontier rows.
3. Add lattice/code attack estimator costs for the passing rows.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
