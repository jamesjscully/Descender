# Bandwidth Frontier Validation

- validation model: `cauchy_mds_surrogate_v1`
- rows checked: `207`
- top rows per profile: `64`

This validates the algebraic Cauchy/MDS surrogate exactly. It does not validate the final seeded coefficient/incidence generator because parameter-search rows do not yet contain concrete seeds or expanded matrices.

## Summary

| profile | rows checked | rows passed | min passing pk | min passing ct |
|---|---:|---:|---:|---:|
| descender-128 | 68 | 68 | 672 B | 192 B |
| descender-192 | 69 | 69 | 816 B | 272 B |
| descender-256 | 70 | 70 | 960 B | 352 B |

## First Passing Rows

| profile | source | q | n | m0 | m1 | k | eta | pk | ct | guard distance | dual distance | verdict |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| descender-128 | accepted_top | 32771 | 32 | 48 | 32 | 24 | 4 | 704 B | 192 B | 17 | 33 | pass |
| descender-192 | accepted_top | 32771 | 48 | 72 | 48 | 32 | 3 | 848 B | 272 B | 25 | 49 | pass |
| descender-256 | accepted_top | 32771 | 64 | 96 | 64 | 48 | 3 | 1024 B | 352 B | 33 | 65 | pass |

## Next Checks

1. Materialize the actual seeded coefficient/incidence/diagonal generator for the first passing rows.
2. Re-run this validator on the concrete expanded matrices instead of the Cauchy/MDS surrogate.
3. Add lattice/code attack estimator costs for the passing rows.
4. Keep theorem hypotheses unchanged until concrete generated rows fail for a specific proof-side reason.
