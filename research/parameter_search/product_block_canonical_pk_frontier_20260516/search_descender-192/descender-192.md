# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- q-ary amplitude strategies: `product-block-lee`
- Lee repetitions: `1`
- Lee rows per product block: `1`
- product-block sidecar manifests: `research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json`
- m0 source: direct values `64, 80, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `32.0` bits
- guard slack target: `12`
- combined certification weight: `0.250`
- accepted candidates before truncation: `2907`
- candidates shown: `120`
- rejected candidates: `26577`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 128 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -228.4 | 214.0 | 221.4 | 214.0 | 395.8 | 98 | -574.4 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 192 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -228.4 | 214.0 | 221.4 | 214.0 | 395.8 | 98 | -574.4 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 256 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -228.4 | 214.0 | 221.4 | 214.0 | 395.8 | 98 | -574.4 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 96 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -228.4 | 214.0 | 221.4 | 214.0 | 395.8 | 98 | -574.4 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 64 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -228.7 | 208.3 | 221.4 | 208.3 | 395.8 | 98 | -574.4 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 128 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -228.7 | 207.6 | 212.7 | 207.6 | 346.2 | 102 | -587.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 192 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -228.7 | 207.6 | 212.7 | 207.6 | 346.2 | 102 | -587.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 256 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -228.7 | 207.6 | 212.7 | 207.6 | 346.2 | 102 | -587.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 96 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -228.7 | 207.6 | 212.7 | 207.6 | 346.2 | 102 | -587.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 64 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -229.0 | 202.0 | 212.7 | 202.0 | 346.2 | 102 | -587.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 128 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -229.0 | 202.6 | 205.9 | 202.6 | 307.2 | 106 | -597.9 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 192 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -229.0 | 202.6 | 205.9 | 202.6 | 307.2 | 106 | -597.9 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 256 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -229.0 | 202.6 | 205.9 | 202.6 | 307.2 | 106 | -597.9 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 96 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -229.0 | 202.6 | 205.9 | 202.6 | 307.2 | 106 | -597.9 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 64 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -229.3 | 197.2 | 205.9 | 197.2 | 307.2 | 106 | -597.9 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 104 | 32 | 136 | 128 | 6 | 14 | 0 | 6 | 1024 | 1024 | 4310.3 | 3286.3 | 64 | 128 | -240.2 | 221.4 | 221.4 | 260.8 | 366.2 | 98 | -502.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 104 | 32 | 136 | 192 | 6 | 14 | 0 | 6 | 1024 | 1024 | 4310.3 | 3286.3 | 64 | 128 | -240.2 | 221.4 | 221.4 | 260.8 | 366.2 | 98 | -502.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 104 | 32 | 136 | 256 | 6 | 14 | 0 | 6 | 1024 | 1024 | 4310.3 | 3286.3 | 64 | 128 | -240.2 | 221.4 | 221.4 | 260.8 | 366.2 | 98 | -502.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 104 | 32 | 136 | 96 | 6 | 14 | 0 | 6 | 1024 | 1024 | 4310.3 | 3286.3 | 64 | 128 | -240.2 | 221.4 | 221.4 | 260.8 | 366.2 | 98 | -502.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 104 | 32 | 136 | 128 | 8 | 14 | 0 | 8 | 768 | 768 | 4310.3 | 3542.3 | 64 | 128 | -240.7 | 212.7 | 212.7 | 252.8 | 313.6 | 102 | -519.3 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 24 | 24 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
