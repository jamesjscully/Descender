# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `31`
- top rows per profile: `20`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-128 | 31 | 31 | 304 B | 272 B |

## First Passing Rows

| profile | source | proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | pk | ct | correctness distance | lossy diag req | lossy diag slack | amp l1 req | amp l1 cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | bounded dual | view route | verdict |
|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| descender-128 | accepted_top | full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 112 | 8 | 304 B | 272 B | 25 | 97 | -711.1 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | 2 | full-view | pass |

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Reject rows with `qary_amplitude_minkowski_obstruction`; no arrangement with the same public-view dimensions can meet the required centered-L1 threshold.
3. Run view-only q-ary centered-L1/BKZ reductions, then exact bounded Lee-dual checks, before estimator work.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
