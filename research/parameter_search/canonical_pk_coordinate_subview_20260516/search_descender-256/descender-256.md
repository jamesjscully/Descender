# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- q-ary amplitude strategies: `product-block-lee`
- Lee repetitions: `1`
- Lee rows per product block: `1`
- product-block sidecar manifests: `research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json`
- m0 source: direct values `64, 80, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448`
- m1 source: direct values `32`
- view coordinate counts: `64, 80, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448`
- pk seed bytes: `32`
- security extra cap: `32.0` bits
- guard slack target: `16`
- combined certification weight: `0.250`
- accepted candidates before truncation: `13210`
- candidates shown: `200`
- rejected candidates: `497846`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 112 | 128 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -214.1 | 283.5 | 283.5 | 317.2 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 38 | 0.0 | 22 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 112 | 192 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -214.1 | 283.5 | 283.5 | 317.2 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 38 | 0.0 | 22 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 112 | 256 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -214.1 | 283.5 | 283.5 | 317.2 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 38 | 0.0 | 22 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 112 | 128 | 6 | 14 | 0 | 6 | 1024 | 1024 | 35377.0 | 34353.0 | 64 | 128 | -214.5 | 276.8 | 276.8 | 311.3 | 672.4 | 122 | -791.7 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 20 | 36 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 112 | 192 | 6 | 14 | 0 | 6 | 1024 | 1024 | 35377.0 | 34353.0 | 64 | 128 | -214.5 | 276.8 | 276.8 | 311.3 | 672.4 | 122 | -791.7 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 20 | 36 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 112 | 256 | 6 | 14 | 0 | 6 | 1024 | 1024 | 35377.0 | 34353.0 | 64 | 128 | -214.5 | 276.8 | 276.8 | 311.3 | 672.4 | 122 | -791.7 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 20 | 36 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 80 | 128 | 32 | 112 | 128 | 8 | 14 | 0 | 8 | 768 | 768 | 35377.0 | 34609.0 | 64 | 128 | -215.1 | 265.9 | 265.9 | 301.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 16 | 32 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 80 | 128 | 32 | 112 | 192 | 8 | 14 | 0 | 8 | 768 | 768 | 35377.0 | 34609.0 | 64 | 128 | -215.1 | 265.9 | 265.9 | 301.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 16 | 32 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 80 | 128 | 32 | 112 | 256 | 8 | 14 | 0 | 8 | 768 | 768 | 35377.0 | 34609.0 | 64 | 128 | -215.1 | 265.9 | 265.9 | 301.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 16 | 32 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 160 | 32 | 112 | 128 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -217.3 | 283.5 | 283.5 | 336.4 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 70 | 0.0 | 22 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 160 | 32 | 112 | 192 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -217.3 | 283.5 | 283.5 | 475.7 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 70 | 0.0 | 22 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 160 | 32 | 112 | 256 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -217.3 | 283.5 | 283.5 | 502.3 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 70 | 0.0 | 22 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 160 | 32 | 112 | 128 | 6 | 14 | 0 | 6 | 1024 | 1024 | 35377.0 | 34353.0 | 64 | 128 | -217.7 | 276.8 | 276.8 | 330.2 | 672.4 | 122 | -791.7 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 20 | 68 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 160 | 32 | 112 | 192 | 6 | 14 | 0 | 6 | 1024 | 1024 | 35377.0 | 34353.0 | 64 | 128 | -217.7 | 276.8 | 276.8 | 466.5 | 672.4 | 122 | -791.7 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 20 | 68 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 160 | 32 | 112 | 256 | 6 | 14 | 0 | 6 | 1024 | 1024 | 35377.0 | 34353.0 | 64 | 128 | -217.7 | 276.8 | 276.8 | 492.6 | 672.4 | 122 | -791.7 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 20 | 68 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 80 | 160 | 32 | 112 | 128 | 8 | 14 | 0 | 8 | 768 | 768 | 35377.0 | 34609.0 | 64 | 128 | -218.3 | 265.9 | 265.9 | 320.3 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 16 | 64 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 80 | 160 | 32 | 112 | 192 | 8 | 14 | 0 | 8 | 768 | 768 | 35377.0 | 34609.0 | 64 | 128 | -218.3 | 265.9 | 265.9 | 451.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 16 | 64 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 80 | 160 | 32 | 112 | 256 | 8 | 14 | 0 | 8 | 768 | 768 | 35377.0 | 34609.0 | 64 | 128 | -218.3 | 265.9 | 265.9 | 476.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 16 | 64 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 160 | 32 | 112 | 96 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -218.5 | 259.4 | 283.5 | 259.4 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 70 | 0.0 | 22 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 192 | 32 | 112 | 128 | 5 | 14 | 0 | 5 | 1229 | 1229 | 35377.0 | 34148.0 | 80 | 160 | -220.5 | 283.5 | 283.5 | 342.9 | 699.4 | 119 | -783.4 | 32 | 0 | 0 | 0 | 0 | 196 | 228 | 228 | 22 | 102 | 0.0 | 22 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
