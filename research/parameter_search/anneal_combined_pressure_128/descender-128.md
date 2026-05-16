# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `full-surjective, support-covered, lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384`
- view coordinate counts: `0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `5601876`
- candidates shown: `200`
- rejected candidates: `2078124`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 24 | 1 | 11 | 4 | 5 | -108.3 | 141.8 | 141.8 | 160.3 | 432.7 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 14 | 14 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 32 | 1 | 11 | 4 | 5 | -108.3 | 141.8 | 141.8 | 166.2 | 432.7 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 14 | 14 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 48 | 1 | 11 | 4 | 5 | -108.3 | 141.8 | 141.8 | 166.2 | 432.7 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 14 | 14 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 64 | 1 | 11 | 4 | 5 | -108.3 | 141.8 | 141.8 | 166.2 | 432.7 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 14 | 14 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 24 | 2 | 11 | 4 | 6 | -108.5 | 138.4 | 138.4 | 157.4 | 425.0 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 32 | 2 | 11 | 4 | 6 | -108.5 | 138.4 | 138.4 | 163.2 | 425.0 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 48 | 2 | 11 | 4 | 6 | -108.5 | 138.4 | 138.4 | 163.2 | 425.0 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 64 | 2 | 11 | 4 | 6 | -108.5 | 138.4 | 138.4 | 163.2 | 425.0 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 24 | 3 | 11 | 4 | 7 | -108.7 | 135.5 | 135.5 | 154.9 | 418.4 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 10 | 10 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 32 | 3 | 11 | 4 | 7 | -108.7 | 135.5 | 135.5 | 160.6 | 418.4 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 10 | 10 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 48 | 3 | 11 | 4 | 7 | -108.7 | 135.5 | 135.5 | 160.6 | 418.4 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 10 | 10 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 64 | 3 | 11 | 4 | 7 | -108.7 | 135.5 | 135.5 | 160.6 | 418.4 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 10 | 10 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 24 | 4 | 11 | 4 | 8 | -108.8 | 133.0 | 133.0 | 152.8 | 412.6 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 8 | 8 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 32 | 4 | 11 | 4 | 8 | -108.8 | 133.0 | 133.0 | 158.3 | 412.6 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 8 | 8 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 48 | 4 | 11 | 4 | 8 | -108.8 | 133.0 | 133.0 | 158.3 | 412.6 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 8 | 8 | 0.0 | 0 |
| lossy-budget | guard-only | 12289 | 40 | 64 | 40 | 32 | 64 | 4 | 11 | 4 | 8 | -108.8 | 133.0 | 133.0 | 158.3 | 412.6 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 8 | 8 | 0.0 | 0 |
| lossy-budget | guard-only | 32771 | 40 | 80 | 40 | 32 | 24 | 1 | 11 | 9 | 10 | -109.8 | 148.5 | 148.5 | 168.9 | 459.5 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 32771 | 40 | 80 | 40 | 32 | 32 | 1 | 11 | 9 | 10 | -109.8 | 148.5 | 148.5 | 218.1 | 459.5 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 32771 | 40 | 80 | 40 | 32 | 48 | 1 | 11 | 9 | 10 | -109.8 | 148.5 | 148.5 | 271.2 | 459.5 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 32771 | 40 | 80 | 40 | 32 | 64 | 1 | 11 | 9 | 10 | -109.8 | 148.5 | 148.5 | 271.2 | 459.5 | 64 | 0 | 0 | 0 | 0 | 44 | 108 | 76 | 20 | 20 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
