# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `96, 112, 128, 144, 160, 192, 224, 256, 320, 384, 448, 512, 640, 768`
- view coordinate counts: `96, 112, 128, 144, 160, 192, 224, 256, 320, 384, 448, 512, 640, 768`
- security extra cap: `16.0` bits
- guard slack target: `70`
- combined certification weight: `0.250`
- accepted candidates before truncation: `3347501`
- candidates shown: `200`
- rejected candidates: `492499`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | 131071 | 40 | 96 | 40 | 96 | 24 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 209.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 96 | 40 | 96 | 32 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 273.0 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 96 | 40 | 96 | 48 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 393.7 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 96 | 40 | 96 | 64 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 458.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 96 | 40 | 96 | 96 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 458.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 48 |
| lossy-budget | guard-only | 131071 | 40 | 96 | 40 | 96 | 24 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 209.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 40 | 96 | 40 | 96 | 32 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 273.0 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 40 | 96 | 40 | 96 | 48 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 393.7 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 40 | 96 | 40 | 96 | 64 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 458.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 40 | 96 | 40 | 96 | 96 | 2 | 15 | 2 | 4 | -203.9 | 193.6 | 193.6 | 458.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 48 | 0.0 | 0 |
| lossy-budget | combined-proxy | 131071 | 40 | 112 | 40 | 96 | 24 | 2 | 15 | 2 | 4 | -205.5 | 193.6 | 193.6 | 212.0 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 64 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 112 | 40 | 96 | 32 | 2 | 15 | 2 | 4 | -205.5 | 193.6 | 193.6 | 275.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 64 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 112 | 40 | 96 | 48 | 2 | 15 | 2 | 4 | -205.5 | 193.6 | 193.6 | 398.4 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 64 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 112 | 40 | 96 | 64 | 2 | 15 | 2 | 4 | -205.5 | 193.6 | 193.6 | 515.2 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 64 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 112 | 40 | 96 | 96 | 2 | 15 | 2 | 4 | -205.5 | 193.6 | 193.6 | 578.7 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 64 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 128 | 40 | 96 | 24 | 2 | 15 | 2 | 4 | -207.1 | 193.6 | 193.6 | 213.9 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 80 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 128 | 40 | 96 | 32 | 2 | 15 | 2 | 4 | -207.1 | 193.6 | 193.6 | 278.4 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 80 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 128 | 40 | 96 | 48 | 2 | 15 | 2 | 4 | -207.1 | 193.6 | 193.6 | 402.5 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 80 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 128 | 40 | 96 | 64 | 2 | 15 | 2 | 4 | -207.1 | 193.6 | 193.6 | 521.1 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 80 | 0.0 | 48 |
| lossy-budget | combined-proxy | 131071 | 40 | 128 | 40 | 96 | 96 | 2 | 15 | 2 | 4 | -207.1 | 193.6 | 193.6 | 697.2 | 375.7 | 53 | 56.0 | 64 | 0 | 0 | 0 | 0 | 180 | 244 | 212 | 48 | 80 | 0.0 | 48 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
