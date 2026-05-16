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
- product-block sidecar manifests: `research/parameter_search/product_block_lee_sidecars_broad_20260516/manifest_minimal.json`
- m0 source: direct values `64, 80, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `32.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `4152`
- candidates shown: `120`
- rejected candidates: `22056`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 112 | 32 | 12 | 14 | 0 | 12 | 512 | 512 | 2400.4 | 1888.4 | 48 | 96 | -303.2 | 131.2 | 150.2 | 131.2 | 132.0 | 82 | -389.1 | 96 | 320 | 224 | 112 | 80 | 196 | 1028 | 228 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 112 | 48 | 12 | 14 | 0 | 12 | 512 | 512 | 2400.4 | 1888.4 | 48 | 96 | -303.7 | 132.0 | 150.2 | 189.6 | 132.0 | 82 | -389.1 | 96 | 320 | 288 | 112 | 80 | 196 | 1092 | 228 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 112 | 64 | 12 | 14 | 0 | 12 | 512 | 512 | 2400.4 | 1888.4 | 48 | 96 | -304.2 | 132.0 | 150.2 | 194.7 | 132.0 | 82 | -389.1 | 96 | 320 | 352 | 112 | 80 | 196 | 1156 | 228 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 112 | 96 | 12 | 14 | 0 | 12 | 512 | 512 | 2400.4 | 1888.4 | 48 | 96 | -305.2 | 132.0 | 150.2 | 194.7 | 132.0 | 82 | -389.1 | 96 | 320 | 480 | 112 | 80 | 196 | 1284 | 228 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 112 | 128 | 12 | 14 | 0 | 12 | 512 | 512 | 2400.4 | 1888.4 | 48 | 96 | -306.1 | 132.0 | 150.2 | 194.7 | 132.0 | 82 | -389.1 | 96 | 320 | 608 | 112 | 80 | 196 | 1412 | 228 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 112 | 192 | 12 | 14 | 0 | 12 | 512 | 512 | 2400.4 | 1888.4 | 48 | 96 | -308.1 | 132.0 | 150.2 | 194.7 | 132.0 | 82 | -389.1 | 96 | 320 | 864 | 112 | 80 | 196 | 1668 | 228 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 112 | 256 | 12 | 14 | 0 | 12 | 512 | 512 | 2400.4 | 1888.4 | 48 | 96 | -310.0 | 132.0 | 150.2 | 194.7 | 132.0 | 82 | -389.1 | 96 | 320 | 1120 | 112 | 80 | 196 | 1924 | 228 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 48 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -340.6 | 161.4 | 221.4 | 161.4 | 395.8 | 98 | -574.4 | 96 | 384 | 288 | 112 | 80 | 224 | 1184 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 48 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -340.8 | 156.8 | 212.7 | 156.8 | 346.2 | 102 | -587.6 | 96 | 384 | 288 | 112 | 80 | 224 | 1184 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 48 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -341.0 | 153.1 | 205.9 | 153.1 | 307.2 | 106 | -597.9 | 96 | 384 | 288 | 112 | 80 | 224 | 1184 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 64 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -341.1 | 208.3 | 221.4 | 208.3 | 395.8 | 98 | -574.4 | 96 | 384 | 352 | 112 | 80 | 224 | 1248 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 64 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -341.1 | 202.0 | 212.7 | 202.0 | 346.2 | 102 | -587.6 | 96 | 384 | 352 | 112 | 80 | 224 | 1248 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 64 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -341.1 | 197.2 | 205.9 | 197.2 | 307.2 | 106 | -597.9 | 96 | 384 | 352 | 112 | 80 | 224 | 1248 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 64 | 96 | 32 | 128 | 64 | 12 | 14 | 0 | 12 | 512 | 512 | 5358.3 | 4846.3 | 48 | 96 | -341.2 | 193.1 | 200.3 | 193.1 | 275.0 | 110 | -606.4 | 96 | 384 | 352 | 112 | 80 | 224 | 1248 | 256 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 64 | 96 | 32 | 128 | 48 | 12 | 14 | 0 | 12 | 512 | 512 | 5358.3 | 4846.3 | 48 | 96 | -341.2 | 150.1 | 200.3 | 150.1 | 275.0 | 110 | -606.4 | 96 | 384 | 288 | 112 | 80 | 224 | 1184 | 256 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 96 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -342.0 | 214.0 | 221.4 | 214.0 | 395.8 | 98 | -574.4 | 96 | 384 | 480 | 112 | 80 | 224 | 1376 | 256 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 128 | 96 | 8 | 14 | 0 | 8 | 768 | 768 | 5358.3 | 4590.3 | 64 | 128 | -342.1 | 207.6 | 212.7 | 207.6 | 346.2 | 102 | -587.6 | 96 | 384 | 480 | 112 | 80 | 224 | 1376 | 256 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 64 | 96 | 32 | 128 | 96 | 10 | 14 | 0 | 10 | 615 | 615 | 5358.3 | 4743.3 | 64 | 128 | -342.1 | 202.6 | 205.9 | 202.6 | 307.2 | 106 | -597.9 | 96 | 384 | 480 | 112 | 80 | 224 | 1376 | 256 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 512 | 12289 | 64 | 96 | 32 | 128 | 96 | 12 | 14 | 0 | 12 | 512 | 512 | 5358.3 | 4846.3 | 48 | 96 | -342.2 | 198.4 | 200.3 | 198.4 | 275.0 | 110 | -606.4 | 96 | 384 | 480 | 112 | 80 | 224 | 1376 | 256 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 128 | 128 | 6 | 14 | 0 | 6 | 1024 | 1024 | 5358.3 | 4334.3 | 64 | 128 | -343.0 | 214.0 | 221.4 | 214.0 | 395.8 | 98 | -574.4 | 96 | 384 | 608 | 112 | 80 | 224 | 1504 | 256 | 20 | 20 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
