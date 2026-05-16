# Moment Lee Stratified Certificate Probe

- source: `research/parameter_search/vandermonde_lee_probe_g32_r42_shift1_random_seed1_128.json`
- q: `32771`
- group size: `32`
- rows: `42`
- target l1: `3277`
- exponent start: `1`
- column scale: `random`
- exact column certificate pass: `true`
- best l1 found: `3349`

| support | complete | restarts/checks | best l1 | method |
|---:|---|---:|---:|---|
| 1 | true | 104832 | 3349 | exact-column |
| 2 | false | 512 | 198135 | support-coordinate-descent |
| 3 | false | 512 | 174166 | support-coordinate-descent |
| 4 | false | 512 | 196097 | anchored-coordinate-descent |
| 5 | false | 512 | 176563 | anchored-coordinate-descent |

Only support 1 is exhaustive. Higher-support rows are heuristic coordinate-descent attacks.
`best l1 < target l1` is a real counterexample; otherwise this is not a proof.
