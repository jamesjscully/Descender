# Systematic Lee Search

- q: `65537`
- pivot rows: `48`
- free columns: `32`
- eta: `10`
- target centered Lee l1: `3277`
- families: `dense-random, block-random, toeplitz-scrambled, vandermonde-scrambled`
- seeds per family: `32`

| family | seed | best l1 | margin | counterexample | method | free sparsity | support | linf | mul work | cert work |
|---|---:|---:|---:|---|---|---:|---:|---:|---:|---:|
| toeplitz-scrambled | 12352654184583275142 | 500845 | 497568 | false | pair-exact | 2 | 50 | 30705 | 1536 | 2788000 |
| toeplitz-scrambled | 11778656130436721325 | 496077 | 492800 | false | pair-exact | 2 | 50 | 28282 | 1536 | 2788000 |
| toeplitz-scrambled | 5235844545145924579 | 495085 | 491808 | false | pair-exact | 2 | 50 | 29594 | 1536 | 2788000 |
| toeplitz-scrambled | 10422444917664798613 | 493693 | 490416 | false | pair-exact | 2 | 50 | 28640 | 1536 | 2788000 |
| vandermonde-scrambled | 8566826915835479536 | 493642 | 490365 | false | anneal | 10 | 58 | 27226 | 1536 | 2788000 |
| vandermonde-scrambled | 14936036508700161403 | 493063 | 489786 | false | anneal | 9 | 57 | 29705 | 1536 | 2788000 |
| dense-random | 12385927021175510165 | 492326 | 489049 | false | pair-exact | 2 | 50 | 28795 | 1536 | 2788000 |
| toeplitz-scrambled | 6835334853489448765 | 492272 | 488995 | false | pair-exact | 2 | 50 | 31078 | 1536 | 2788000 |
| dense-random | 4367092679015170301 | 491469 | 488192 | false | pair-exact | 2 | 50 | 31455 | 1536 | 2788000 |
| vandermonde-scrambled | 160928449250163134 | 491314 | 488037 | false | pair-exact | 2 | 50 | 31524 | 1536 | 2788000 |
| toeplitz-scrambled | 10999322847774336324 | 491186 | 487909 | false | pair-exact | 2 | 50 | 29321 | 1536 | 2788000 |
| vandermonde-scrambled | 14911015122584580597 | 491177 | 487900 | false | pair-exact | 2 | 50 | 31422 | 1536 | 2788000 |

Each row is a systematic public-view candidate `A = [I | P]`.
The relation checked is `(-P y, y)`, matching `SystematicLeeDistanceAtLeast`.
A found counterexample is real for the generated `P`; absence of one is a search sidecar, not a proof.
Survivors still need exact bounded Lee-dual certification before Lean instantiation.
