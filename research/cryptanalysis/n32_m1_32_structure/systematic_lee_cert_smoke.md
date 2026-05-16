# Systematic Lee Bounded Certificate

- source: `research/cryptanalysis/n32_m1_32_structure/systematic_lee_search_smoke.json`
- q: `65537`
- pivot rows: `48`
- free columns: `32`
- eta: `10`
- target centered Lee l1: `3277`
- max support: `2`
- coefficient bound: `4`

| family | seed | pass | best l1 | checked | support scope | coeff scope | best free support | best relation support |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| dense-random | 4064276564124724818 | true | 550127 | 32000 | 2 | 4 | 2 | 50 |
| dense-random | 18338708898557712457 | true | 540383 | 32000 | 2 | 4 | 2 | 50 |
| toeplitz-scrambled | 12352654184583275142 | true | 537635 | 32000 | 2 | 4 | 2 | 50 |
| toeplitz-scrambled | 10422444917664798613 | true | 533054 | 32000 | 2 | 4 | 2 | 50 |
| vandermonde-scrambled | 10228898693943419361 | true | 527953 | 32000 | 2 | 4 | 2 | 50 |
| vandermonde-scrambled | 14479137014785669896 | true | 555589 | 32000 | 2 | 4 | 2 | 50 |
| block-random | 215571865830012030 | true | 84348 | 32000 | 2 | 4 | 2 | 14 |
| block-random | 14281626879191192045 | true | 78886 | 32000 | 2 | 4 | 2 | 14 |

The checked relation is `(-P y, y)` for the regenerated systematic candidate.
Pass means exact absence of a bad vector only inside the declared bounded domain.
Full Lean use still requires a global `SystematicLeeDistanceAtLeast` proof or a solver certificate whose domain is global.
