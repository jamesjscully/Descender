# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
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
- guard slack target: `12`
- combined certification weight: `0.250`
- accepted candidates before truncation: `57782`
- candidates shown: `160`
- rejected candidates: `1248586`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 128 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 192 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 256 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 96 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 16 |
| lossy-budget | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 128 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 192 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 256 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 96 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.4 | 207.6 | 212.7 | 207.6 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 0 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 64 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.7 | 202.0 | 212.7 | 202.0 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 16 |
| lossy-budget | guard-only | product-block-lee | 1024 | 12289 | 64 | 96 | 32 | 96 | 64 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -214.7 | 202.0 | 212.7 | 202.0 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 16 | 0.0 | 0 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 128 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 12 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 192 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 12 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 256 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 12 |
| lossy-budget | combined-proxy | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 96 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 12 |
| lossy-budget | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 128 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 192 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 256 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 768 | 12289 | 64 | 96 | 32 | 96 | 96 | 8 | 12 | 2 | 10 | 615 | 768 | 19444.9 | 18829.9 | 64 | 128 | -214.7 | 202.6 | 205.9 | 202.6 | 447.8 | 106 | -597.9 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 12 | 12 | 0.0 | 0 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 64 | 104 | 32 | 96 | 128 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -215.0 | 212.7 | 212.7 | 252.8 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 24 | 0.0 | 16 |
| lossy-budget | combined-proxy | product-block-lee | 1024 | 12289 | 64 | 104 | 32 | 96 | 192 | 6 | 12 | 2 | 8 | 768 | 1024 | 19444.9 | 18676.9 | 64 | 128 | -215.0 | 212.7 | 212.7 | 252.8 | 477.0 | 102 | -587.6 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 16 | 24 | 0.0 | 16 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
