# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `27`
- top rows per profile: `20`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-128 | 27 | 27 | 1280 B | 304 B |

## First Passing Rows

| profile | source | proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | pk | ct | correctness distance | lossy diag req | lossy diag slack | amp l1 req | amp l1 cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | bounded dual | view route | verdict |
|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| descender-128 | accepted_top | full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 64 | 5 | 1344 B | 304 B | 25 | 114 | -934.8 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | 2 | full-view | pass |

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Reject rows with `qary_amplitude_minkowski_obstruction`; no arrangement with the same public-view dimensions can meet the required centered-L1 threshold.
3. Run view-only q-ary centered-L1/BKZ reductions, then exact bounded Lee-dual checks, before estimator work.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
