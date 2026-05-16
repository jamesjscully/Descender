# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only`
- m0 source: direct values `72, 80, 88, 96, 112`
- view coordinate counts: `64, 72, 80, 88, 96, 112`
- security extra cap: `24.0` bits
- guard slack target: `8`
- combined certification weight: `0.000`
- accepted candidates before truncation: `122296628`
- candidates shown: `1200`
- rejected candidates: `31303372`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 24 | 1 | 12 | 9 | 10 | -194.4 | 130.0 | 130.0 | 178.1 | 195.8 | 49 | -75.7 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 32 | 1 | 12 | 9 | 10 | -194.4 | 130.0 | 130.0 | 230.2 | 195.8 | 49 | -75.7 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 48 | 1 | 12 | 9 | 10 | -194.4 | 130.0 | 130.0 | 286.4 | 195.8 | 49 | -75.7 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 64 | 1 | 12 | 9 | 10 | -194.4 | 130.0 | 130.0 | 286.4 | 195.8 | 49 | -75.7 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 24 | 2 | 12 | 9 | 11 | -195.0 | 128.5 | 128.5 | 176.5 | 186.3 | 49 | -81.2 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 18 | 18 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 32 | 2 | 12 | 9 | 11 | -195.0 | 128.5 | 128.5 | 228.1 | 186.3 | 49 | -81.2 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 18 | 18 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 48 | 2 | 12 | 9 | 11 | -195.0 | 128.5 | 128.5 | 283.7 | 186.3 | 49 | -81.2 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 18 | 18 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 64 | 2 | 12 | 9 | 11 | -195.0 | 128.5 | 128.5 | 283.7 | 186.3 | 49 | -81.2 | 64 | 0 | 0 | 0 | 0 | 108 | 172 | 140 | 18 | 18 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 24 | 1 | 13 | 5 | 6 | -202.1 | 137.8 | 137.8 | 186.4 | 245.6 | 46 | -46.4 | 64 | 0 | 0 | 0 | 0 | 117 | 181 | 149 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 32 | 1 | 13 | 5 | 6 | -202.1 | 137.8 | 137.8 | 241.3 | 245.6 | 46 | -46.4 | 64 | 0 | 0 | 0 | 0 | 117 | 181 | 149 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 48 | 1 | 13 | 5 | 6 | -202.1 | 137.8 | 137.8 | 300.6 | 245.6 | 46 | -46.4 | 64 | 0 | 0 | 0 | 0 | 117 | 181 | 149 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 72 | 32 | 72 | 64 | 1 | 13 | 5 | 6 | -202.1 | 137.8 | 137.8 | 300.6 | 245.6 | 46 | -46.4 | 64 | 0 | 0 | 0 | 0 | 117 | 181 | 149 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 24 | 1 | 12 | 9 | 10 | -202.3 | 130.0 | 130.0 | 179.5 | 160.6 | 49 | 9.5 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 32 | 1 | 12 | 9 | 10 | -202.3 | 130.0 | 130.0 | 232.3 | 160.6 | 49 | 9.5 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 48 | 1 | 12 | 9 | 10 | -202.3 | 130.0 | 130.0 | 331.9 | 160.6 | 49 | 9.5 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 64 | 1 | 12 | 9 | 10 | -202.3 | 130.0 | 130.0 | 337.9 | 160.6 | 49 | 9.5 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 24 | 2 | 12 | 9 | 11 | -202.4 | 128.5 | 128.5 | 178.0 | 150.1 | 49 | 2.9 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 26 | 26 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 32 | 2 | 12 | 9 | 11 | -202.4 | 128.5 | 128.5 | 230.2 | 150.1 | 49 | 2.9 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 26 | 26 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 48 | 2 | 12 | 9 | 11 | -202.4 | 128.5 | 128.5 | 328.7 | 150.1 | 49 | 2.9 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 26 | 26 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 80 | 32 | 80 | 64 | 2 | 12 | 9 | 11 | -202.4 | 128.5 | 128.5 | 334.7 | 150.1 | 49 | 2.9 | 64 | 0 | 0 | 0 | 0 | 120 | 184 | 152 | 26 | 26 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
