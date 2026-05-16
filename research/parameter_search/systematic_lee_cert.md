# Systematic Lee Bounded Certificate

- source: `research/parameter_search/systematic_lee_search.json`
- q: `65537`
- pivot rows: `48`
- free columns: `32`
- eta: `10`
- target centered Lee l1: `3277`
- max support: `2`
- coefficient bound: `64`

| family | seed | pass | best l1 | checked | support scope | coeff scope | best free support | best relation support | basis |
|---|---:|---|---:|---:|---:|---:|---:|---:|---|
| toeplitz-scrambled | 12352654184583275142 | true | 465007 | 8130560 | 2 | 64 | 2 | 50 | 0000_toeplitz_scrambled_12352654184583275142.dual_qary_basis.txt |
| toeplitz-scrambled | 11778656130436721325 | true | 449217 | 8130560 | 2 | 64 | 2 | 50 | 0001_toeplitz_scrambled_11778656130436721325.dual_qary_basis.txt |
| toeplitz-scrambled | 5235844545145924579 | true | 428377 | 8130560 | 2 | 64 | 2 | 50 | 0002_toeplitz_scrambled_5235844545145924579.dual_qary_basis.txt |
| toeplitz-scrambled | 10422444917664798613 | true | 469203 | 8130560 | 2 | 64 | 2 | 50 | 0003_toeplitz_scrambled_10422444917664798613.dual_qary_basis.txt |
| vandermonde-scrambled | 8566826915835479536 | true | 467408 | 8130560 | 2 | 64 | 2 | 50 | 0004_vandermonde_scrambled_8566826915835479536.dual_qary_basis.txt |
| vandermonde-scrambled | 14936036508700161403 | true | 446157 | 8130560 | 2 | 64 | 2 | 50 | 0005_vandermonde_scrambled_14936036508700161403.dual_qary_basis.txt |

The checked relation is `(-P y, y)` for the regenerated systematic candidate.
Pass means exact absence of a bad vector only inside the declared bounded domain.
Full Lean use still requires a global `SystematicLeeDistanceAtLeast` proof or a solver certificate whose domain is global.
