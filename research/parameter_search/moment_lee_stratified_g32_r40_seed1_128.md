# Moment Lee Stratified Certificate Probe

- source: `research/parameter_search/vandermonde_lee_probe_g32_r40_shift1_random_seed1_128.json`
- q: `32771`
- group size: `32`
- rows: `40`
- target l1: `3277`
- exponent start: `1`
- column scale: `random`
- exact column certificate pass: `false`
- best l1 found: `3183`

| support | complete | restarts/checks | best l1 | method |
|---:|---|---:|---:|---|
| 1 | true | 104832 | 3183 | exact-column |
| 2 | false | 128 | 175229 | support-coordinate-descent |
| 3 | false | 128 | 184471 | support-coordinate-descent |
| 4 | false | 128 | 177478 | support-coordinate-descent |

Only support 1 is exhaustive. Higher-support rows are heuristic coordinate-descent attacks.
`best l1 < target l1` is a real counterexample; otherwise this is not a proof.
