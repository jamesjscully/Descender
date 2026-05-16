# Systematic Lee Bounded Certificate

- source: `research/cryptanalysis/n32_m1_32_structure/systematic_lee_search_smoke.json`
- q: `65537`
- pivot rows: `48`
- free columns: `32`
- eta: `10`
- target centered Lee l1: `3277`
- max support: `2`
- coefficient bound: `64`

| family | seed | pass | best l1 | checked | support scope | coeff scope | best free support | best relation support | basis |
|---|---:|---|---:|---:|---:|---:|---:|---:|---|
| dense-random | 4064276564124724818 | true | 474638 | 8130560 | 2 | 64 | 2 | 50 | 0000_dense_random_4064276564124724818.dual_qary_basis.txt |
| dense-random | 18338708898557712457 | true | 461424 | 8130560 | 2 | 64 | 2 | 50 | 0001_dense_random_18338708898557712457.dual_qary_basis.txt |

The checked relation is `(-P y, y)` for the regenerated systematic candidate.
Pass means exact absence of a bad vector only inside the declared bounded domain.
Full Lean use still requires a global `SystematicLeeDistanceAtLeast` proof or a solver certificate whose domain is global.
