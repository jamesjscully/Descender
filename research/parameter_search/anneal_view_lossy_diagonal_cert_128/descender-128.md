# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320`
- view coordinate counts: `64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320`
- security extra cap: `16.0` bits
- guard slack target: `40`
- combined certification weight: `0.250`
- accepted candidates before truncation: `3441636`
- candidates shown: `200`
- rejected candidates: `398364`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 24 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 191.2 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 32 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 247.6 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 48 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 254.5 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 64 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 254.5 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 24 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 191.2 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 32 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 247.6 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 48 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 254.5 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 64 | 1 | 14 | 3 | 4 | -202.3 | 143.7 | 143.7 | 254.5 | 309.1 | 43 | -118.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | combined-proxy | 40961 | 32 | 64 | 32 | 64 | 24 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 183.1 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 40961 | 32 | 64 | 32 | 64 | 32 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 236.7 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 40961 | 32 | 64 | 32 | 64 | 48 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 243.3 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 40961 | 32 | 64 | 32 | 64 | 64 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 243.3 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | guard-only | 40961 | 32 | 64 | 32 | 64 | 24 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 183.1 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | guard-only | 40961 | 32 | 64 | 32 | 64 | 32 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 236.7 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | guard-only | 40961 | 32 | 64 | 32 | 64 | 48 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 243.3 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | guard-only | 40961 | 32 | 64 | 32 | 64 | 64 | 2 | 14 | 2 | 4 | -202.4 | 136.1 | 136.1 | 243.3 | 287.4 | 44 | -119.6 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 0 |
| lossy-budget | combined-proxy | 32771 | 32 | 64 | 32 | 64 | 24 | 2 | 14 | 2 | 4 | -202.6 | 132.5 | 132.5 | 179.2 | 277.1 | 44 | -119.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 32771 | 32 | 64 | 32 | 64 | 32 | 2 | 14 | 2 | 4 | -202.6 | 132.5 | 132.5 | 231.6 | 277.1 | 44 | -119.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 32771 | 32 | 64 | 32 | 64 | 48 | 2 | 14 | 2 | 4 | -202.6 | 132.5 | 132.5 | 238.0 | 277.1 | 44 | -119.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |
| lossy-budget | combined-proxy | 32771 | 32 | 64 | 32 | 64 | 64 | 2 | 14 | 2 | 4 | -202.6 | 132.5 | 132.5 | 238.0 | 277.1 | 44 | -119.9 | 64 | 0 | 0 | 0 | 0 | 112 | 176 | 144 | 24 | 24 | 0.0 | 24 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
