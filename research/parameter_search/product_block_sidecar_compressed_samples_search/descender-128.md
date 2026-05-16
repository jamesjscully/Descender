# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- q-ary amplitude strategies: `product-block-lee`
- Lee repetitions: `1`
- Lee rows per product block: `1`
- product-block sidecar manifests: `research/parameter_search/product_block_lee_sidecars_q32771_eta5_m32_r3_5_try1/manifest.json, research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/manifest.json`
- m0 source: direct values `80, 88, 96, 104, 112, 120, 128, 136, 144, 152, 160`
- m1 source: direct values `32`
- view coordinate counts: `0`
- pk seed bytes: `0`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `2996`
- candidates shown: `100`
- rejected candidates: `5320`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 104 | 32 | 136 | 112 | 8 | 12 | 5 | 13 | 1261 | 2049 | 6838.5 | 5577.5 | 64 | 128 | -310.8 | 229.5 | 229.5 | 268.2 | 313.3 | 104 | -579.8 | 64 | 0 | 0 | 0 | 0 | 204 | 268 | 236 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 104 | 32 | 136 | 128 | 8 | 12 | 5 | 13 | 1261 | 2049 | 6838.5 | 5577.5 | 64 | 128 | -310.8 | 229.5 | 229.5 | 268.2 | 313.3 | 104 | -579.8 | 64 | 0 | 0 | 0 | 0 | 204 | 268 | 236 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 104 | 32 | 136 | 160 | 8 | 12 | 5 | 13 | 1261 | 2049 | 6838.5 | 5577.5 | 64 | 128 | -310.8 | 229.5 | 229.5 | 268.2 | 313.3 | 104 | -579.8 | 64 | 0 | 0 | 0 | 0 | 204 | 268 | 236 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 104 | 32 | 136 | 192 | 8 | 12 | 5 | 13 | 1261 | 2049 | 6838.5 | 5577.5 | 64 | 128 | -310.8 | 229.5 | 229.5 | 268.2 | 313.3 | 104 | -579.8 | 64 | 0 | 0 | 0 | 0 | 204 | 268 | 236 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 104 | 32 | 136 | 80 | 8 | 12 | 5 | 13 | 1261 | 2049 | 6838.5 | 5577.5 | 64 | 128 | -310.8 | 229.5 | 229.5 | 262.4 | 313.3 | 104 | -579.8 | 64 | 0 | 0 | 0 | 0 | 204 | 268 | 236 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 104 | 32 | 136 | 96 | 8 | 12 | 5 | 13 | 1261 | 2049 | 6838.5 | 5577.5 | 64 | 128 | -310.8 | 229.5 | 229.5 | 268.2 | 313.3 | 104 | -579.8 | 64 | 0 | 0 | 0 | 0 | 204 | 268 | 236 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 104 | 32 | 136 | 64 | 8 | 12 | 5 | 13 | 1261 | 2049 | 6838.5 | 5577.5 | 64 | 128 | -310.8 | 215.6 | 229.5 | 215.6 | 313.3 | 104 | -579.8 | 64 | 0 | 0 | 0 | 0 | 204 | 268 | 236 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 112 | 8 | 13 | 3 | 11 | 1490 | 2049 | 8750.1 | 7260.1 | 64 | 128 | -315.0 | 223.7 | 234.7 | 223.7 | 381.0 | 101 | -646.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 128 | 8 | 13 | 3 | 11 | 1490 | 2049 | 8750.1 | 7260.1 | 64 | 128 | -315.0 | 223.7 | 234.7 | 223.7 | 381.0 | 101 | -646.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 160 | 8 | 13 | 3 | 11 | 1490 | 2049 | 8750.1 | 7260.1 | 64 | 128 | -315.0 | 223.7 | 234.7 | 223.7 | 381.0 | 101 | -646.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 192 | 8 | 13 | 3 | 11 | 1490 | 2049 | 8750.1 | 7260.1 | 64 | 128 | -315.0 | 223.7 | 234.7 | 223.7 | 381.0 | 101 | -646.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 80 | 8 | 13 | 3 | 11 | 1490 | 2049 | 8750.1 | 7260.1 | 64 | 128 | -315.0 | 223.7 | 234.7 | 223.7 | 381.0 | 101 | -646.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 96 | 8 | 13 | 3 | 11 | 1490 | 2049 | 8750.1 | 7260.1 | 64 | 128 | -315.0 | 223.7 | 234.7 | 223.7 | 381.0 | 101 | -646.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 64 | 8 | 13 | 3 | 11 | 1490 | 2049 | 8750.1 | 7260.1 | 64 | 128 | -315.0 | 217.7 | 234.7 | 217.7 | 381.0 | 101 | -646.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 112 | 32 | 144 | 112 | 8 | 12 | 5 | 13 | 1261 | 2049 | 5510.9 | 4249.9 | 64 | 128 | -326.3 | 229.5 | 229.5 | 315.7 | 275.3 | 104 | -505.7 | 64 | 0 | 0 | 0 | 0 | 216 | 280 | 248 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 112 | 32 | 144 | 128 | 8 | 12 | 5 | 13 | 1261 | 2049 | 5510.9 | 4249.9 | 64 | 128 | -326.3 | 229.5 | 229.5 | 315.7 | 275.3 | 104 | -505.7 | 64 | 0 | 0 | 0 | 0 | 216 | 280 | 248 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 112 | 32 | 144 | 160 | 8 | 12 | 5 | 13 | 1261 | 2049 | 5510.9 | 4249.9 | 64 | 128 | -326.3 | 229.5 | 229.5 | 315.7 | 275.3 | 104 | -505.7 | 64 | 0 | 0 | 0 | 0 | 216 | 280 | 248 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 112 | 32 | 144 | 192 | 8 | 12 | 5 | 13 | 1261 | 2049 | 5510.9 | 4249.9 | 64 | 128 | -326.3 | 229.5 | 229.5 | 315.7 | 275.3 | 104 | -505.7 | 64 | 0 | 0 | 0 | 0 | 216 | 280 | 248 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 112 | 32 | 144 | 80 | 8 | 12 | 5 | 13 | 1261 | 2049 | 5510.9 | 4249.9 | 64 | 128 | -326.3 | 229.5 | 229.5 | 264.4 | 275.3 | 104 | -505.7 | 64 | 0 | 0 | 0 | 0 | 216 | 280 | 248 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 112 | 32 | 144 | 96 | 8 | 12 | 5 | 13 | 1261 | 2049 | 5510.9 | 4249.9 | 64 | 128 | -326.3 | 229.5 | 229.5 | 310.1 | 275.3 | 104 | -505.7 | 64 | 0 | 0 | 0 | 0 | 216 | 280 | 248 | 22 | 22 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
