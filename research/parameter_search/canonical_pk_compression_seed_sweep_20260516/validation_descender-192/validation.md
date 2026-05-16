# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `160`
- top rows per profile: `160`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-192 | 160 | 160 | 256 B | 256 B |

## First Passing Rows

| profile | source | proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | pk | ct | correctness distance | lossy diag req | lossy diag slack | amp l1 req | amp l1 cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | bounded dual | view route | verdict |
|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| descender-192 | accepted_top | full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 128 | 6 | 256 B | 256 B | 33 | 98 | -574.4 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | 2 | full-view | pass |

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Reject rows with `qary_amplitude_minkowski_obstruction`; no arrangement with the same public-view dimensions can meet the required centered-L1 threshold.
3. Run view-only q-ary centered-L1/BKZ reductions, then exact bounded Lee-dual checks, before estimator work.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
