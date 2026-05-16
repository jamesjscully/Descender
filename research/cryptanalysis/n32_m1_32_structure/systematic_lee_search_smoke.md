# Systematic Lee Search

- q: `65537`
- pivot rows: `48`
- free columns: `32`
- eta: `10`
- target centered Lee l1: `3277`
- families: `dense-random, block-random, toeplitz-scrambled, vandermonde-scrambled`
- seeds per family: `2`

| family | seed | best l1 | margin | counterexample | method | free sparsity | support | linf | mul work | cert work |
|---|---:|---:|---:|---|---|---:|---:|---:|---:|---:|
| dense-random | 4064276564124724818 | 550127 | 546850 | false | pair-exact | 2 | 50 | 31500 | 1536 | 39840 |
| dense-random | 18338708898557712457 | 540383 | 537106 | false | pair-exact | 2 | 50 | 30702 | 1536 | 39840 |
| toeplitz-scrambled | 12352654184583275142 | 535966 | 532689 | false | random-sparse | 6 | 54 | 31275 | 1536 | 39840 |
| toeplitz-scrambled | 10422444917664798613 | 533054 | 529777 | false | pair-exact | 2 | 50 | 30941 | 1536 | 39840 |
| vandermonde-scrambled | 10228898693943419361 | 525690 | 522413 | false | unit-exact | 1 | 49 | 30366 | 1536 | 39840 |
| vandermonde-scrambled | 14479137014785669896 | 518450 | 515173 | false | random-sparse | 5 | 53 | 32186 | 1536 | 39840 |
| block-random | 215571865830012030 | 78929 | 75652 | false | unit-exact | 1 | 13 | 25639 | 1536 | 39840 |
| block-random | 14281626879191192045 | 77411 | 74134 | false | unit-exact | 1 | 13 | 12960 | 1536 | 39840 |

Each row is a systematic public-view candidate `A = [I | P]`.
The relation checked is `(-P y, y)`, matching `SystematicLeeDistanceAtLeast`.
A found counterexample is real for the generated `P`; absence of one is a search sidecar, not a proof.
Survivors still need exact bounded Lee-dual certification before Lean instantiation.
