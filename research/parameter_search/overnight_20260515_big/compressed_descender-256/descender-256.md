# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- proof strategies: `full-surjective, support-covered, lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- view coordinate counts: `0, 24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- security extra cap: `24.0` bits
- guard slack target: `16`
- combined certification weight: `0.750`
- accepted candidates before truncation: `160720039`
- candidates shown: `1200`
- rejected candidates: `108079961`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| support-covered | combined-proxy | 12289 | 72 | 112 | 72 | 96 | 48 | 4 | 14 | 0 | 4 | -148.5 | 262.5 | 262.5 | 282.0 | 673.8 | 103 | -740.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 16 | 32 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 72 | 112 | 72 | 96 | 64 | 4 | 14 | 0 | 4 | -148.5 | 262.5 | 262.5 | 282.0 | 673.8 | 103 | -740.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 16 | 32 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 72 | 128 | 72 | 96 | 48 | 4 | 14 | 0 | 4 | -150.1 | 262.5 | 262.5 | 327.5 | 673.8 | 103 | -740.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 16 | 48 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 72 | 128 | 72 | 96 | 64 | 4 | 14 | 0 | 4 | -150.1 | 262.5 | 262.5 | 381.0 | 673.8 | 103 | -740.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 16 | 48 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 64 | 96 | 64 | 88 | 48 | 4 | 16 | 0 | 4 | -150.4 | 257.4 | 272.2 | 257.4 | 701.7 | 88 | -699.7 | 64 | 0 | 0 | 0 | 0 | 176 | 240 | 208 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 64 | 96 | 64 | 88 | 64 | 4 | 16 | 0 | 4 | -150.4 | 257.4 | 272.2 | 257.4 | 701.7 | 88 | -699.7 | 64 | 0 | 0 | 0 | 0 | 176 | 240 | 208 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 64 | 112 | 64 | 88 | 48 | 4 | 16 | 0 | 4 | -151.2 | 272.2 | 272.2 | 364.1 | 701.7 | 88 | -699.7 | 64 | 0 | 0 | 0 | 0 | 176 | 240 | 208 | 16 | 40 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 64 | 112 | 64 | 88 | 64 | 4 | 16 | 0 | 4 | -151.2 | 272.2 | 272.2 | 370.9 | 701.7 | 88 | -699.7 | 64 | 0 | 0 | 0 | 0 | 176 | 240 | 208 | 16 | 40 | 0.0 | 16 |
| support-covered | combined-proxy | 65537 | 56 | 88 | 56 | 80 | 32 | 3 | 17 | 0 | 3 | -152.4 | 258.6 | 258.6 | 264.3 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 26 | 0.0 | 18 |
| support-covered | combined-proxy | 65537 | 56 | 88 | 56 | 80 | 48 | 3 | 17 | 0 | 3 | -152.4 | 258.6 | 258.6 | 271.8 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 26 | 0.0 | 18 |
| support-covered | combined-proxy | 65537 | 56 | 88 | 56 | 80 | 64 | 3 | 17 | 0 | 3 | -152.4 | 258.6 | 258.6 | 271.8 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 26 | 0.0 | 18 |
| support-covered | combined-proxy | 40961 | 64 | 128 | 64 | 88 | 48 | 4 | 16 | 0 | 4 | -152.8 | 272.2 | 272.2 | 367.6 | 701.7 | 88 | -699.7 | 64 | 0 | 0 | 0 | 0 | 176 | 240 | 208 | 16 | 56 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 64 | 128 | 64 | 88 | 64 | 4 | 16 | 0 | 4 | -152.8 | 272.2 | 272.2 | 475.0 | 701.7 | 88 | -699.7 | 64 | 0 | 0 | 0 | 0 | 176 | 240 | 208 | 16 | 56 | 0.0 | 16 |
| support-covered | combined-proxy | 65537 | 56 | 96 | 56 | 80 | 32 | 3 | 17 | 0 | 3 | -153.2 | 258.6 | 258.6 | 265.7 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 34 | 0.0 | 18 |
| support-covered | combined-proxy | 65537 | 56 | 96 | 56 | 80 | 48 | 3 | 17 | 0 | 3 | -153.2 | 258.6 | 258.6 | 332.5 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 34 | 0.0 | 18 |
| support-covered | combined-proxy | 65537 | 56 | 96 | 56 | 80 | 64 | 3 | 17 | 0 | 3 | -153.2 | 258.6 | 258.6 | 332.5 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 34 | 0.0 | 18 |
| support-covered | combined-proxy | 12289 | 72 | 160 | 72 | 96 | 48 | 4 | 14 | 0 | 4 | -153.3 | 262.5 | 262.5 | 333.3 | 673.8 | 103 | -740.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 16 | 80 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 72 | 160 | 72 | 96 | 64 | 4 | 14 | 0 | 4 | -153.3 | 262.5 | 262.5 | 429.8 | 673.8 | 103 | -740.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 16 | 80 | 0.0 | 16 |
| support-covered | combined-proxy | 65537 | 56 | 112 | 56 | 80 | 32 | 3 | 17 | 0 | 3 | -154.8 | 258.6 | 258.6 | 268.3 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 50 | 0.0 | 18 |
| support-covered | combined-proxy | 65537 | 56 | 112 | 56 | 80 | 48 | 3 | 17 | 0 | 3 | -154.8 | 258.6 | 258.6 | 387.2 | 671.4 | 73 | -588.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 18 | 50 | 0.0 | 18 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
