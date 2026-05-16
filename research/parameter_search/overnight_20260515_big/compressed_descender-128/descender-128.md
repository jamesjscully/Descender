# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `full-surjective, support-covered, lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- view coordinate counts: `0, 24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- security extra cap: `24.0` bits
- guard slack target: `8`
- combined certification weight: `0.750`
- accepted candidates before truncation: `164716037`
- candidates shown: `1200`
- rejected candidates: `104083963`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| support-covered | combined-proxy | 40961 | 32 | 48 | 32 | 48 | 24 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 48 | 32 | 48 | 32 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 48 | 32 | 48 | 48 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 48 | 32 | 48 | 64 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 8 |
| support-covered | guard-only | 40961 | 32 | 48 | 32 | 48 | 24 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 0 |
| support-covered | guard-only | 40961 | 32 | 48 | 32 | 48 | 32 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 0 |
| support-covered | guard-only | 40961 | 32 | 48 | 32 | 48 | 48 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 0 |
| support-covered | guard-only | 40961 | 32 | 48 | 32 | 48 | 64 | 4 | 16 | 0 | 4 | -169.4 | 131.5 | 136.1 | 131.5 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 8 | 0.0 | 0 |
| support-covered | combined-proxy | 40961 | 32 | 56 | 32 | 48 | 24 | 4 | 16 | 0 | 4 | -170.0 | 136.1 | 136.1 | 181.3 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 16 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 56 | 32 | 48 | 32 | 4 | 16 | 0 | 4 | -170.0 | 136.1 | 136.1 | 188.1 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 16 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 56 | 32 | 48 | 48 | 4 | 16 | 0 | 4 | -170.0 | 136.1 | 136.1 | 188.1 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 16 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 56 | 32 | 48 | 64 | 4 | 16 | 0 | 4 | -170.0 | 136.1 | 136.1 | 188.1 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 16 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 64 | 32 | 48 | 24 | 4 | 16 | 0 | 4 | -170.8 | 136.1 | 136.1 | 183.1 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 24 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 64 | 32 | 48 | 32 | 4 | 16 | 0 | 4 | -170.8 | 136.1 | 136.1 | 236.7 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 24 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 64 | 32 | 48 | 48 | 4 | 16 | 0 | 4 | -170.8 | 136.1 | 136.1 | 243.3 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 24 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 64 | 32 | 48 | 64 | 4 | 16 | 0 | 4 | -170.8 | 136.1 | 136.1 | 243.3 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 24 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 72 | 32 | 48 | 24 | 4 | 16 | 0 | 4 | -171.6 | 136.1 | 136.1 | 184.6 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 32 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 72 | 32 | 48 | 32 | 4 | 16 | 0 | 4 | -171.6 | 136.1 | 136.1 | 239.0 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 32 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 72 | 32 | 48 | 48 | 4 | 16 | 0 | 4 | -171.6 | 136.1 | 136.1 | 297.6 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 32 | 0.0 | 8 |
| support-covered | combined-proxy | 40961 | 32 | 72 | 32 | 48 | 64 | 4 | 16 | 0 | 4 | -171.6 | 136.1 | 136.1 | 297.6 | 338.1 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 96 | 160 | 128 | 8 | 32 | 0.0 | 8 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
