# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only`
- q-ary amplitude strategies: `product-block-lee`
- Lee repetitions: `512`
- Lee rows per product block: `3`
- product-block sidecar manifests: `research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json`
- m0 source: direct values `80`
- m1 source: direct values `32`
- view coordinate counts: `80`
- pk seed bytes: `32`
- security extra cap: `32.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `1`
- candidates shown: `1`
- rejected candidates: `0`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | guard-only | product-block-lee | 512 | 12289 | 48 | 80 | 32 | 80 | 128 | 12 | 14 | 0 | 12 | 512 | 512 | 8697.5 | 8185.5 | 48 | 96 | -266.3 | 150.2 | 150.2 | 194.7 | 280.6 | 82 | -389.1 | 32 | 0 | 0 | 0 | 0 | 140 | 172 | 172 | 8 | 8 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
