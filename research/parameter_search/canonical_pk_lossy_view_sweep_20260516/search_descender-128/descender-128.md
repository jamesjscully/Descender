# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
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
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `92110`
- candidates shown: `160`
- rejected candidates: `1214258`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 96 | 128 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -259.6 | 146.6 | 146.6 | 191.1 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 4 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 96 | 192 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -259.6 | 146.6 | 146.6 | 191.1 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 4 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 96 | 256 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -259.6 | 146.6 | 146.6 | 191.1 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 4 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 96 | 48 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -259.6 | 146.6 | 146.6 | 186.1 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 4 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 96 | 64 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -259.6 | 146.6 | 146.6 | 191.1 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 4 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 96 | 96 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -259.6 | 146.6 | 146.6 | 191.1 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 4 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 96 | 32 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -260.5 | 128.9 | 146.6 | 128.9 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 4 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 128 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 192 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 256 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 48 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 189.7 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 64 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 239.8 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 20 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 96 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 20 |
| lossy-budget | guard-only | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 128 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 192 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 256 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 48 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 189.7 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 64 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 239.8 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 96 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -261.2 | 146.6 | 146.6 | 273.3 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 0 |
| lossy-budget | combined-proxy | product-block-lee | 512 | 12289 | 48 | 96 | 32 | 96 | 32 | 12 | 12 | 2 | 14 | 439 | 512 | 4047.8 | 3608.8 | 48 | 96 | -262.0 | 131.1 | 146.6 | 131.1 | 185.7 | 85 | -272.2 | 32 | 0 | 0 | 0 | 0 | 144 | 176 | 176 | 20 | 20 | 0.0 | 20 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
