# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
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
- guard slack target: `16`
- combined certification weight: `0.250`
- accepted candidates before truncation: `1613`
- candidates shown: `120`
- rejected candidates: `27871`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 160 | 128 | 5 | 14 | 0 | 5 | 1229 | 1229 | 6667.6 | 5438.6 | 80 | 160 | -216.8 | 283.5 | 283.5 | 317.2 | 533.3 | 119 | -636.4 | 96 | 480 | 608 | 112 | 80 | 280 | 1656 | 312 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 160 | 128 | 6 | 14 | 0 | 6 | 1024 | 1024 | 6667.6 | 5643.6 | 64 | 128 | -217.1 | 276.8 | 276.8 | 311.3 | 494.7 | 122 | -648.7 | 96 | 480 | 608 | 112 | 80 | 280 | 1656 | 312 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 160 | 192 | 5 | 14 | 0 | 5 | 1229 | 1229 | 6667.6 | 5438.6 | 80 | 160 | -217.3 | 283.5 | 283.5 | 317.2 | 533.3 | 119 | -636.4 | 96 | 480 | 864 | 112 | 80 | 280 | 1912 | 312 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 160 | 192 | 6 | 14 | 0 | 6 | 1024 | 1024 | 6667.6 | 5643.6 | 64 | 128 | -217.6 | 276.8 | 276.8 | 311.3 | 494.7 | 122 | -648.7 | 96 | 480 | 864 | 112 | 80 | 280 | 1912 | 312 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 80 | 128 | 32 | 160 | 128 | 8 | 14 | 0 | 8 | 768 | 768 | 6667.6 | 5899.6 | 64 | 128 | -217.7 | 265.9 | 265.9 | 301.8 | 432.8 | 128 | -668.3 | 96 | 480 | 608 | 112 | 80 | 280 | 1656 | 312 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 160 | 256 | 5 | 14 | 0 | 5 | 1229 | 1229 | 6667.6 | 5438.6 | 80 | 160 | -217.7 | 283.5 | 283.5 | 317.2 | 533.3 | 119 | -636.4 | 96 | 480 | 1120 | 112 | 80 | 280 | 2168 | 312 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 160 | 256 | 6 | 14 | 0 | 6 | 1024 | 1024 | 6667.6 | 5643.6 | 64 | 128 | -218.1 | 276.8 | 276.8 | 311.3 | 494.7 | 122 | -648.7 | 96 | 480 | 1120 | 112 | 80 | 280 | 2168 | 312 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 80 | 128 | 32 | 160 | 128 | 10 | 14 | 0 | 10 | 615 | 615 | 6667.6 | 6052.6 | 64 | 128 | -218.2 | 257.4 | 257.4 | 294.4 | 384.0 | 133 | -683.7 | 96 | 480 | 608 | 112 | 80 | 280 | 1656 | 312 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 80 | 128 | 32 | 160 | 192 | 8 | 14 | 0 | 8 | 768 | 768 | 6667.6 | 5899.6 | 64 | 128 | -218.2 | 265.9 | 265.9 | 301.8 | 432.8 | 128 | -668.3 | 96 | 480 | 864 | 112 | 80 | 280 | 1912 | 312 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 80 | 128 | 32 | 160 | 192 | 10 | 14 | 0 | 10 | 615 | 615 | 6667.6 | 6052.6 | 64 | 128 | -218.7 | 257.4 | 257.4 | 294.4 | 384.0 | 133 | -683.7 | 96 | 480 | 864 | 112 | 80 | 280 | 1912 | 312 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 768 | 12289 | 80 | 128 | 32 | 160 | 256 | 8 | 14 | 0 | 8 | 768 | 768 | 6667.6 | 5899.6 | 64 | 128 | -218.7 | 265.9 | 265.9 | 301.8 | 432.8 | 128 | -668.3 | 96 | 480 | 1120 | 112 | 80 | 280 | 2168 | 312 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 615 | 12289 | 80 | 128 | 32 | 160 | 256 | 10 | 14 | 0 | 10 | 615 | 615 | 6667.6 | 6052.6 | 64 | 128 | -219.1 | 257.4 | 257.4 | 294.4 | 384.0 | 133 | -683.7 | 96 | 480 | 1120 | 112 | 80 | 280 | 2168 | 312 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 104 | 32 | 136 | 96 | 10 | 17 | 0 | 10 | 3277 | 3277 | 9475.6 | 6198.6 | 64 | 128 | -220.1 | 260.0 | 260.0 | 296.1 | 426.6 | 97 | -587.7 | 96 | 600 | 672 | 112 | 80 | 289 | 1849 | 321 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 104 | 32 | 136 | 128 | 10 | 17 | 0 | 10 | 3277 | 3277 | 9475.6 | 6198.6 | 64 | 128 | -220.5 | 260.0 | 260.0 | 296.1 | 426.6 | 97 | -587.7 | 96 | 600 | 864 | 112 | 80 | 289 | 2041 | 321 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 104 | 32 | 136 | 192 | 10 | 17 | 0 | 10 | 3277 | 3277 | 9475.6 | 6198.6 | 64 | 128 | -221.2 | 260.0 | 260.0 | 296.1 | 426.6 | 97 | -587.7 | 96 | 600 | 1248 | 112 | 80 | 289 | 2425 | 321 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 104 | 32 | 136 | 256 | 10 | 17 | 0 | 10 | 3277 | 3277 | 9475.6 | 6198.6 | 64 | 128 | -222.0 | 260.0 | 260.0 | 296.1 | 426.6 | 97 | -587.7 | 96 | 600 | 1632 | 112 | 80 | 289 | 2809 | 321 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 112 | 32 | 144 | 96 | 10 | 17 | 0 | 10 | 3277 | 3277 | 7498.9 | 4221.9 | 64 | 128 | -231.3 | 260.0 | 260.0 | 342.8 | 391.5 | 97 | -502.5 | 96 | 624 | 672 | 112 | 80 | 306 | 1890 | 338 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 112 | 32 | 144 | 128 | 10 | 17 | 0 | 10 | 3277 | 3277 | 7498.9 | 4221.9 | 64 | 128 | -231.7 | 260.0 | 260.0 | 349.1 | 391.5 | 97 | -502.5 | 96 | 624 | 864 | 112 | 80 | 306 | 2082 | 338 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 112 | 32 | 144 | 192 | 10 | 17 | 0 | 10 | 3277 | 3277 | 7498.9 | 4221.9 | 64 | 128 | -232.4 | 260.0 | 260.0 | 349.1 | 391.5 | 97 | -502.5 | 96 | 624 | 1248 | 112 | 80 | 306 | 2466 | 338 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 65537 | 64 | 112 | 32 | 144 | 256 | 10 | 17 | 0 | 10 | 3277 | 3277 | 7498.9 | 4221.9 | 64 | 128 | -233.1 | 260.0 | 260.0 | 349.1 | 391.5 | 97 | -502.5 | 96 | 624 | 1632 | 112 | 80 | 306 | 2850 | 338 | 28 | 28 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
