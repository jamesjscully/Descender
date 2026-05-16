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
- Lee repetitions: `3277`
- Lee rows per product block: `5`
- m0 source: direct values `80, 88, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448, 512, 544`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `32.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `1321`
- candidates shown: `200`
- rejected candidates: `13799`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 64 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | -403.2 | 189.3 | 323.1 | 189.3 | 729.5 | 114 | -934.8 | 96 | 432 | 352 | 112 | 80 | 272 | 1344 | 304 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 64 | 6 | 16 | 0 | 6 | 2731 | 3277 | 23237.8 | 20506.8 | 80 | 160 | -403.2 | 186.3 | 316.4 | 186.3 | 696.8 | 116 | -941.1 | 96 | 432 | 352 | 112 | 80 | 272 | 1344 | 304 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 64 | 8 | 16 | 0 | 8 | 2049 | 3277 | 23237.8 | 21188.8 | 80 | 160 | -403.2 | 181.5 | 305.6 | 181.5 | 644.1 | 121 | -951.1 | 96 | 432 | 352 | 112 | 80 | 272 | 1344 | 304 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 48 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | -403.3 | 147.6 | 323.1 | 147.6 | 729.5 | 114 | -934.8 | 96 | 432 | 288 | 112 | 80 | 272 | 1280 | 304 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 48 | 6 | 16 | 0 | 6 | 2731 | 3277 | 23237.8 | 20506.8 | 80 | 160 | -403.4 | 145.3 | 316.4 | 145.3 | 696.8 | 116 | -941.1 | 96 | 432 | 288 | 112 | 80 | 272 | 1280 | 304 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 48 | 8 | 16 | 0 | 8 | 2049 | 3277 | 23237.8 | 21188.8 | 80 | 160 | -403.6 | 141.7 | 305.6 | 141.7 | 644.1 | 121 | -951.1 | 96 | 432 | 288 | 112 | 80 | 272 | 1280 | 304 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | -404.1 | 189.3 | 323.1 | 189.3 | 729.5 | 114 | -934.8 | 96 | 432 | 480 | 112 | 80 | 272 | 1472 | 304 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 96 | 6 | 16 | 0 | 6 | 2731 | 3277 | 23237.8 | 20506.8 | 80 | 160 | -404.2 | 186.3 | 316.4 | 186.3 | 696.8 | 116 | -941.1 | 96 | 432 | 480 | 112 | 80 | 272 | 1472 | 304 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 96 | 8 | 16 | 0 | 8 | 2049 | 3277 | 23237.8 | 21188.8 | 80 | 160 | -404.2 | 181.5 | 305.6 | 181.5 | 644.1 | 121 | -951.1 | 96 | 432 | 480 | 112 | 80 | 272 | 1472 | 304 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | -405.1 | 189.3 | 323.1 | 189.3 | 729.5 | 114 | -934.8 | 96 | 432 | 608 | 112 | 80 | 272 | 1600 | 304 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 128 | 6 | 16 | 0 | 6 | 2731 | 3277 | 23237.8 | 20506.8 | 80 | 160 | -405.1 | 186.3 | 316.4 | 186.3 | 696.8 | 116 | -941.1 | 96 | 432 | 608 | 112 | 80 | 272 | 1600 | 304 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 128 | 8 | 16 | 0 | 8 | 2049 | 3277 | 23237.8 | 21188.8 | 80 | 160 | -405.2 | 181.5 | 305.6 | 181.5 | 644.1 | 121 | -951.1 | 96 | 432 | 608 | 112 | 80 | 272 | 1600 | 304 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | -407.1 | 189.3 | 323.1 | 189.3 | 729.5 | 114 | -934.8 | 96 | 432 | 864 | 112 | 80 | 272 | 1856 | 304 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 192 | 6 | 16 | 0 | 6 | 2731 | 3277 | 23237.8 | 20506.8 | 80 | 160 | -407.1 | 186.3 | 316.4 | 186.3 | 696.8 | 116 | -941.1 | 96 | 432 | 864 | 112 | 80 | 272 | 1856 | 304 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 192 | 8 | 16 | 0 | 8 | 2049 | 3277 | 23237.8 | 21188.8 | 80 | 160 | -407.1 | 181.5 | 305.6 | 181.5 | 644.1 | 121 | -951.1 | 96 | 432 | 864 | 112 | 80 | 272 | 1856 | 304 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 256 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | -409.0 | 189.3 | 323.1 | 189.3 | 729.5 | 114 | -934.8 | 96 | 432 | 1120 | 112 | 80 | 272 | 2112 | 304 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 256 | 6 | 16 | 0 | 6 | 2731 | 3277 | 23237.8 | 20506.8 | 80 | 160 | -409.0 | 186.3 | 316.4 | 186.3 | 696.8 | 116 | -941.1 | 96 | 432 | 1120 | 112 | 80 | 272 | 2112 | 304 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 256 | 8 | 16 | 0 | 8 | 2049 | 3277 | 23237.8 | 21188.8 | 80 | 160 | -409.1 | 181.5 | 305.6 | 181.5 | 644.1 | 121 | -951.1 | 96 | 432 | 1120 | 112 | 80 | 272 | 2112 | 304 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 384 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23237.8 | 19960.8 | 80 | 160 | -412.9 | 189.3 | 323.1 | 189.3 | 729.5 | 114 | -934.8 | 96 | 432 | 1632 | 112 | 80 | 272 | 2624 | 304 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 104 | 32 | 136 | 384 | 6 | 16 | 0 | 6 | 2731 | 3277 | 23237.8 | 20506.8 | 80 | 160 | -412.9 | 186.3 | 316.4 | 186.3 | 696.8 | 116 | -941.1 | 96 | 432 | 1632 | 112 | 80 | 272 | 2624 | 304 | 12 | 12 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
