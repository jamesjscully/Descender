# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only`
- m0 source: direct values `112, 128, 160, 192, 224`
- view coordinate counts: `112, 128, 160, 192, 224`
- security extra cap: `24.0` bits
- guard slack target: `16`
- combined certification weight: `0.000`
- accepted candidates before truncation: `121538344`
- candidates shown: `1200`
- rejected candidates: `32061656`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | guard-only | 65537 | 64 | 160 | 64 | 160 | 48 | 1 | 12 | 9 | 10 | -195.5 | 260.0 | 260.0 | 360.6 | 321.2 | 97 | 8.3 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 76 | 76 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 160 | 64 | 160 | 64 | 1 | 12 | 9 | 10 | -195.5 | 260.0 | 260.0 | 466.1 | 321.2 | 97 | 8.3 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 76 | 76 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 32 | 1 | 14 | 4 | 5 | -195.8 | 265.4 | 265.4 | 276.1 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 62 | 62 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 48 | 1 | 14 | 4 | 5 | -195.8 | 265.4 | 265.4 | 399.2 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 62 | 62 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 64 | 1 | 14 | 4 | 5 | -195.8 | 265.4 | 265.4 | 517.0 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 62 | 62 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 160 | 64 | 160 | 48 | 2 | 12 | 9 | 11 | -196.2 | 257.1 | 257.1 | 357.4 | 300.2 | 98 | -4.7 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 74 | 74 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 160 | 64 | 160 | 64 | 2 | 12 | 9 | 11 | -196.2 | 257.1 | 257.1 | 461.9 | 300.2 | 98 | -4.7 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 74 | 74 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 128 | 64 | 128 | 48 | 1 | 12 | 9 | 10 | -196.5 | 260.0 | 260.0 | 354.5 | 461.8 | 97 | -332.2 | 64 | 0 | 0 | 0 | 0 | 192 | 256 | 224 | 44 | 44 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 128 | 64 | 128 | 64 | 1 | 12 | 9 | 10 | -196.5 | 260.0 | 260.0 | 457.6 | 461.8 | 97 | -332.2 | 64 | 0 | 0 | 0 | 0 | 192 | 256 | 224 | 44 | 44 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 112 | 64 | 112 | 48 | 1 | 12 | 9 | 10 | -197.4 | 260.0 | 260.0 | 351.0 | 532.1 | 97 | -502.5 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 112 | 64 | 112 | 64 | 1 | 12 | 9 | 10 | -197.4 | 260.0 | 260.0 | 357.5 | 532.1 | 97 | -502.5 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 112 | 56 | 112 | 32 | 1 | 14 | 4 | 5 | -197.4 | 265.4 | 265.4 | 273.8 | 564.5 | 76 | -233.4 | 64 | 0 | 0 | 0 | 0 | 196 | 260 | 228 | 46 | 46 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 112 | 56 | 112 | 48 | 1 | 14 | 4 | 5 | -197.4 | 265.4 | 265.4 | 395.5 | 564.5 | 76 | -233.4 | 64 | 0 | 0 | 0 | 0 | 196 | 260 | 228 | 46 | 46 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 112 | 56 | 112 | 64 | 1 | 14 | 4 | 5 | -197.4 | 265.4 | 265.4 | 461.6 | 564.5 | 76 | -233.4 | 64 | 0 | 0 | 0 | 0 | 196 | 260 | 228 | 46 | 46 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 128 | 64 | 128 | 48 | 2 | 12 | 9 | 11 | -197.5 | 257.1 | 257.1 | 351.4 | 445.0 | 98 | -341.0 | 64 | 0 | 0 | 0 | 0 | 192 | 256 | 224 | 42 | 42 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 128 | 64 | 128 | 64 | 2 | 12 | 9 | 11 | -197.5 | 257.1 | 257.1 | 453.4 | 445.0 | 98 | -341.0 | 64 | 0 | 0 | 0 | 0 | 192 | 256 | 224 | 42 | 42 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 32 | 2 | 14 | 4 | 6 | -197.9 | 260.7 | 260.7 | 272.3 | 478.3 | 78 | -50.0 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 60 | 60 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 48 | 2 | 14 | 4 | 6 | -197.9 | 260.7 | 260.7 | 393.4 | 478.3 | 78 | -50.0 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 60 | 60 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 64 | 2 | 14 | 4 | 6 | -197.9 | 260.7 | 260.7 | 509.3 | 478.3 | 78 | -50.0 | 64 | 0 | 0 | 0 | 0 | 224 | 288 | 256 | 60 | 60 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 64 | 112 | 64 | 112 | 48 | 2 | 12 | 9 | 11 | -198.2 | 257.1 | 257.1 | 347.9 | 517.4 | 98 | -509.1 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 26 | 26 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
