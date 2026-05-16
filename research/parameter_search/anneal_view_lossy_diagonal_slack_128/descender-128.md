# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 448, 512, 640`
- view coordinate counts: `64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 448, 512, 640`
- security extra cap: `16.0` bits
- guard slack target: `35`
- combined certification weight: `0.250`
- accepted candidates before truncation: `2871797`
- candidates shown: `200`
- rejected candidates: `968203`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 24 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 176.5 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 12 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 32 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 228.0 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 12 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 48 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 234.3 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 12 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 64 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 234.3 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 12 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 24 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 176.5 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 32 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 228.0 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 48 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 234.3 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 64 | 1 | 12 | 9 | 10 | -187.5 | 130.0 | 130.0 | 234.3 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 12 | 0.0 | 0 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 24 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 174.9 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 10 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 32 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 225.9 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 10 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 48 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 232.1 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 10 |
| lossy-budget | combined-proxy | 65537 | 32 | 64 | 32 | 64 | 64 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 232.1 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 10 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 24 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 174.9 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 32 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 225.9 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 48 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 232.1 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 32 | 64 | 32 | 64 | 64 | 2 | 12 | 9 | 11 | -188.0 | 128.5 | 128.5 | 232.1 | 222.5 | 49 | -165.2 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 10 | 10 | 0.0 | 0 |
| lossy-budget | combined-proxy | 65537 | 32 | 72 | 32 | 64 | 24 | 1 | 12 | 9 | 10 | -188.3 | 130.0 | 130.0 | 178.1 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 20 | 0.0 | 12 |
| lossy-budget | combined-proxy | 65537 | 32 | 72 | 32 | 64 | 32 | 1 | 12 | 9 | 10 | -188.3 | 130.0 | 130.0 | 230.2 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 20 | 0.0 | 12 |
| lossy-budget | combined-proxy | 65537 | 32 | 72 | 32 | 64 | 48 | 1 | 12 | 9 | 10 | -188.3 | 130.0 | 130.0 | 286.4 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 20 | 0.0 | 12 |
| lossy-budget | combined-proxy | 65537 | 32 | 72 | 32 | 64 | 64 | 1 | 12 | 9 | 10 | -188.3 | 130.0 | 130.0 | 286.4 | 230.9 | 49 | -160.8 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 12 | 20 | 0.0 | 12 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
