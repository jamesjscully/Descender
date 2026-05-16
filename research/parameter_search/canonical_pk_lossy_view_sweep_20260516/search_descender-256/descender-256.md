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
- m0 source: direct values `80, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448`
- m1 source: direct values `32`
- view coordinate counts: `80, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448`
- pk seed bytes: `32`
- security extra cap: `32.0` bits
- guard slack target: `16`
- combined certification weight: `0.250`
- accepted candidates before truncation: `32236`
- candidates shown: `160`
- rejected candidates: `1274132`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 112 | 128 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -197.7 | 271.0 | 271.0 | 306.3 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 34 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 112 | 192 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -197.7 | 271.0 | 271.0 | 306.3 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 34 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 128 | 32 | 112 | 256 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -197.7 | 271.0 | 271.0 | 306.3 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 34 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 112 | 128 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -197.9 | 265.9 | 265.9 | 301.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 32 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 112 | 192 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -197.9 | 265.9 | 265.9 | 301.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 32 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 128 | 32 | 112 | 256 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -197.9 | 265.9 | 265.9 | 301.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 32 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 160 | 32 | 112 | 128 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -200.9 | 271.0 | 271.0 | 325.0 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 66 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 160 | 32 | 112 | 192 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -200.9 | 271.0 | 271.0 | 458.7 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 66 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 160 | 32 | 112 | 256 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -200.9 | 271.0 | 271.0 | 484.2 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 66 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 160 | 32 | 112 | 128 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -201.1 | 265.9 | 265.9 | 320.3 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 64 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 160 | 32 | 112 | 192 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -201.1 | 265.9 | 265.9 | 451.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 64 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 160 | 32 | 112 | 256 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -201.1 | 265.9 | 265.9 | 476.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 64 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 192 | 32 | 112 | 128 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -204.1 | 271.0 | 271.0 | 331.5 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 98 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 192 | 32 | 112 | 192 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -204.1 | 271.0 | 271.0 | 469.2 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 98 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1229 | 12289 | 80 | 192 | 32 | 112 | 256 | 5 | 12 | 2 | 7 | 878 | 1229 | 35377.0 | 34499.0 | 80 | 160 | -204.1 | 271.0 | 271.0 | 602.8 | 649.2 | 125 | -798.8 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 18 | 98 | 0.0 | 18 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 192 | 32 | 112 | 128 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -204.3 | 265.9 | 265.9 | 326.8 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 96 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 192 | 32 | 112 | 192 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -204.3 | 265.9 | 265.9 | 462.3 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 96 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 80 | 192 | 32 | 112 | 256 | 6 | 12 | 2 | 8 | 768 | 1024 | 35377.0 | 34609.0 | 64 | 128 | -204.3 | 265.9 | 265.9 | 593.5 | 629.0 | 128 | -805.0 | 32 | 0 | 0 | 0 | 0 | 168 | 200 | 200 | 16 | 96 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 4096 | 40961 | 80 | 128 | 32 | 112 | 128 | 5 | 13 | 3 | 8 | 2560 | 4096 | 83596.6 | 81036.6 | 80 | 160 | -205.4 | 314.6 | 314.6 | 344.4 | 768.0 | 120 | -886.6 | 32 | 0 | 0 | 0 | 0 | 182 | 214 | 214 | 16 | 32 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 4096 | 40961 | 80 | 128 | 32 | 112 | 192 | 5 | 13 | 3 | 8 | 2560 | 4096 | 83596.6 | 81036.6 | 80 | 160 | -205.4 | 314.6 | 314.6 | 344.4 | 768.0 | 120 | -886.6 | 32 | 0 | 0 | 0 | 0 | 182 | 214 | 214 | 16 | 32 | 0.0 | 16 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
