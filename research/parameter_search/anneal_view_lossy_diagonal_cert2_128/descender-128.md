# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `80, 88, 96, 112, 128, 160, 192, 224, 256, 320`
- view coordinate counts: `80, 88, 96, 112, 128, 160, 192, 224, 256, 320`
- security extra cap: `16.0` bits
- guard slack target: `50`
- combined certification weight: `0.250`
- accepted candidates before truncation: `3454827`
- candidates shown: `200`
- rejected candidates: `385173`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 24 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 194.2 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 32 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 251.8 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 48 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 361.2 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 64 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 367.8 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 24 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 194.2 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 32 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 251.8 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 48 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 361.2 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 64 | 1 | 14 | 3 | 4 | -226.4 | 143.7 | 143.7 | 367.8 | 258.4 | 43 | 71.7 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | combined-proxy | 40961 | 32 | 80 | 32 | 80 | 24 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 186.1 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | combined-proxy | 40961 | 32 | 80 | 32 | 80 | 32 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 241.0 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | combined-proxy | 40961 | 32 | 80 | 32 | 80 | 48 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 345.0 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | combined-proxy | 40961 | 32 | 80 | 32 | 80 | 64 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 351.2 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 40 |
| lossy-budget | guard-only | 40961 | 32 | 80 | 32 | 80 | 24 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 186.1 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | guard-only | 40961 | 32 | 80 | 32 | 80 | 32 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 241.0 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | guard-only | 40961 | 32 | 80 | 32 | 80 | 48 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 345.0 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | guard-only | 40961 | 32 | 80 | 32 | 80 | 64 | 2 | 14 | 2 | 4 | -226.4 | 136.1 | 136.1 | 351.2 | 236.7 | 44 | 60.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 40 | 40 | 0.0 | 0 |
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 24 | 2 | 14 | 3 | 5 | -226.5 | 140.5 | 140.5 | 190.7 | 235.2 | 45 | 56.8 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 38 | 38 | 0.0 | 38 |
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 32 | 2 | 14 | 3 | 5 | -226.5 | 140.5 | 140.5 | 247.2 | 235.2 | 45 | 56.8 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 38 | 38 | 0.0 | 38 |
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 48 | 2 | 14 | 3 | 5 | -226.5 | 140.5 | 140.5 | 354.3 | 235.2 | 45 | 56.8 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 38 | 38 | 0.0 | 38 |
| lossy-budget | combined-proxy | 65537 | 32 | 80 | 32 | 80 | 64 | 2 | 14 | 3 | 5 | -226.5 | 140.5 | 140.5 | 360.7 | 235.2 | 45 | 56.8 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 38 | 38 | 0.0 | 38 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
