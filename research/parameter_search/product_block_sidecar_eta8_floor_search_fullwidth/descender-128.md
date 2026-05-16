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
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `553`
- candidates shown: `80`
- rejected candidates: `833`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 112 | 8 | 16 | 0 | 8 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | -353.5 | 179.0 | 244.4 | 179.0 | 469.5 | 97 | -711.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 128 | 8 | 16 | 0 | 8 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | -353.5 | 179.0 | 244.4 | 179.0 | 469.5 | 97 | -711.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 160 | 8 | 16 | 0 | 8 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | -353.5 | 179.0 | 244.4 | 179.0 | 469.5 | 97 | -711.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 192 | 8 | 16 | 0 | 8 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | -353.5 | 179.0 | 244.4 | 179.0 | 469.5 | 97 | -711.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 64 | 8 | 16 | 0 | 8 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | -353.5 | 179.0 | 244.4 | 179.0 | 469.5 | 97 | -711.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 80 | 8 | 16 | 0 | 8 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | -353.5 | 179.0 | 244.4 | 179.0 | 469.5 | 97 | -711.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 88 | 32 | 120 | 96 | 8 | 16 | 0 | 8 | 2049 | 2049 | 11618.2 | 9569.2 | 64 | 128 | -353.5 | 179.0 | 244.4 | 179.0 | 469.5 | 97 | -711.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 112 | 8 | 16 | 0 | 8 | 2049 | 2049 | 8750.1 | 6701.1 | 64 | 128 | -373.9 | 230.9 | 244.4 | 230.9 | 436.8 | 97 | -631.5 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 128 | 8 | 16 | 0 | 8 | 2049 | 2049 | 8750.1 | 6701.1 | 64 | 128 | -373.9 | 230.9 | 244.4 | 230.9 | 436.8 | 97 | -631.5 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 160 | 8 | 16 | 0 | 8 | 2049 | 2049 | 8750.1 | 6701.1 | 64 | 128 | -373.9 | 230.9 | 244.4 | 230.9 | 436.8 | 97 | -631.5 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 192 | 8 | 16 | 0 | 8 | 2049 | 2049 | 8750.1 | 6701.1 | 64 | 128 | -373.9 | 230.9 | 244.4 | 230.9 | 436.8 | 97 | -631.5 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 80 | 8 | 16 | 0 | 8 | 2049 | 2049 | 8750.1 | 6701.1 | 64 | 128 | -373.9 | 230.9 | 244.4 | 230.9 | 436.8 | 97 | -631.5 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 96 | 8 | 16 | 0 | 8 | 2049 | 2049 | 8750.1 | 6701.1 | 64 | 128 | -373.9 | 230.9 | 244.4 | 230.9 | 436.8 | 97 | -631.5 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 64 | 96 | 32 | 128 | 64 | 8 | 16 | 0 | 8 | 2049 | 2049 | 8750.1 | 6701.1 | 64 | 128 | -373.9 | 224.7 | 244.4 | 224.7 | 436.8 | 97 | -631.5 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 72 | 96 | 32 | 128 | 112 | 8 | 16 | 0 | 8 | 2049 | 2049 | 16758.3 | 14709.3 | 64 | 128 | -374.9 | 180.3 | 275.0 | 180.3 | 556.8 | 109 | -831.1 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 72 | 96 | 32 | 128 | 128 | 8 | 16 | 0 | 8 | 2049 | 2049 | 16758.3 | 14709.3 | 64 | 128 | -374.9 | 180.3 | 275.0 | 180.3 | 556.8 | 109 | -831.1 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 72 | 96 | 32 | 128 | 160 | 8 | 16 | 0 | 8 | 2049 | 2049 | 16758.3 | 14709.3 | 64 | 128 | -374.9 | 180.3 | 275.0 | 180.3 | 556.8 | 109 | -831.1 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 72 | 96 | 32 | 128 | 192 | 8 | 16 | 0 | 8 | 2049 | 2049 | 16758.3 | 14709.3 | 64 | 128 | -374.9 | 180.3 | 275.0 | 180.3 | 556.8 | 109 | -831.1 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 72 | 96 | 32 | 128 | 64 | 8 | 16 | 0 | 8 | 2049 | 2049 | 16758.3 | 14709.3 | 64 | 128 | -374.9 | 180.3 | 275.0 | 180.3 | 556.8 | 109 | -831.1 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 2049 | 32771 | 72 | 96 | 32 | 128 | 80 | 8 | 16 | 0 | 8 | 2049 | 2049 | 16758.3 | 14709.3 | 64 | 128 | -374.9 | 180.3 | 275.0 | 180.3 | 556.8 | 109 | -831.1 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 8 | 8 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
