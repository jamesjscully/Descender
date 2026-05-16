# Bandwidth Frontier Validation

- validation model: `seeded_sparse_tilt_v1`
- rows checked: `1`
- top rows per profile: `1`

This expands a deterministic seeded sparse-tilt generator for each row, then validates the generated finite-field matrix together with the Cauchy guard/base blocks.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-128 | 1 | 1 | 201 B | 169 B |

## First Passing Rows

| profile | source | proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | pk | ct | correctness distance | lossy diag req | lossy diag slack | amp l1 req | amp l1 cert | amp rows | qamp ops | bounded dual | view route | verdict |
|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| descender-128 | accepted_top | lossy-budget | guard-only | direct-solved-lee | 4096 | 32771 | 32 | 52 | 32 | 84 | 24 | 2 | 201 B | 169 B | 21 | 46 | -257.2 | 3277 | 4096 | 32 | 1024 | 2 | full-view | pass |

## Next Checks

1. Lift the deterministic sparse-tilt generator into the Lean `SeededSetup` certificate boundary.
2. Replace the conservative q-ary amplitude lower bound with an exact `l1` dual-relation certificate, or search parameters where the certified lower bound meets the actual-noise amplitude requirement.
3. Add lattice/code attack estimator costs for the passing rows.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
