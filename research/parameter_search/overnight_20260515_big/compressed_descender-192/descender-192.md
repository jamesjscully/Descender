# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- proof strategies: `full-surjective, support-covered, lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- view coordinate counts: `0, 24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- security extra cap: `24.0` bits
- guard slack target: `12`
- combined certification weight: `0.750`
- accepted candidates before truncation: `168313211`
- candidates shown: `1200`
- rejected candidates: `100486789`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 32 | 4 | 14 | 0 | 4 | -160.9 | 204.1 | 204.1 | 219.8 | 507.2 | 81 | -523.3 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 48 | 4 | 14 | 0 | 4 | -160.9 | 204.1 | 204.1 | 225.9 | 507.2 | 81 | -523.3 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 64 | 4 | 14 | 0 | 4 | -160.9 | 204.1 | 204.1 | 225.9 | 507.2 | 81 | -523.3 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 32 | 5 | 14 | 0 | 5 | -161.2 | 198.5 | 198.5 | 215.2 | 484.0 | 83 | -530.9 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 14 | 22 | 0.0 | 14 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 48 | 5 | 14 | 0 | 5 | -161.2 | 198.5 | 198.5 | 221.1 | 484.0 | 83 | -530.9 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 14 | 22 | 0.0 | 14 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 64 | 5 | 14 | 0 | 5 | -161.2 | 198.5 | 198.5 | 221.1 | 484.0 | 83 | -530.9 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 14 | 22 | 0.0 | 14 |
| support-covered | combined-proxy | 40961 | 48 | 72 | 48 | 72 | 32 | 4 | 16 | 0 | 4 | -161.4 | 194.5 | 204.2 | 194.5 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 16 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 48 | 72 | 48 | 72 | 48 | 4 | 16 | 0 | 4 | -161.4 | 194.5 | 204.2 | 194.5 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 16 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 48 | 72 | 48 | 72 | 64 | 4 | 16 | 0 | 4 | -161.4 | 194.5 | 204.2 | 194.5 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 16 | 0.0 | 16 |
| support-covered | guard-only | 40961 | 48 | 72 | 48 | 72 | 32 | 4 | 16 | 0 | 4 | -161.4 | 194.5 | 204.2 | 194.5 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 16 | 0.0 | 0 |
| support-covered | guard-only | 40961 | 48 | 72 | 48 | 72 | 48 | 4 | 16 | 0 | 4 | -161.4 | 194.5 | 204.2 | 194.5 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 16 | 0.0 | 0 |
| support-covered | guard-only | 40961 | 48 | 72 | 48 | 72 | 64 | 4 | 16 | 0 | 4 | -161.4 | 194.5 | 204.2 | 194.5 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 16 | 0.0 | 0 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 32 | 6 | 14 | 0 | 6 | -161.5 | 193.7 | 193.7 | 211.3 | 464.7 | 86 | -537.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 12 | 20 | 0.0 | 12 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 48 | 6 | 14 | 0 | 6 | -161.5 | 193.7 | 193.7 | 217.2 | 464.7 | 86 | -537.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 12 | 20 | 0.0 | 12 |
| support-covered | combined-proxy | 12289 | 56 | 88 | 56 | 80 | 64 | 6 | 14 | 0 | 6 | -161.5 | 193.7 | 193.7 | 217.2 | 464.7 | 86 | -537.2 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 12 | 20 | 0.0 | 12 |
| support-covered | combined-proxy | 40961 | 48 | 80 | 48 | 72 | 32 | 4 | 16 | 0 | 4 | -161.7 | 204.2 | 204.2 | 244.5 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 48 | 80 | 48 | 72 | 48 | 4 | 16 | 0 | 4 | -161.7 | 204.2 | 204.2 | 251.4 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 40961 | 48 | 80 | 48 | 72 | 64 | 4 | 16 | 0 | 4 | -161.7 | 204.2 | 204.2 | 251.4 | 507.2 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 16 | 24 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 56 | 96 | 56 | 80 | 32 | 4 | 14 | 0 | 4 | -161.7 | 204.1 | 204.1 | 221.2 | 507.2 | 81 | -523.3 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 16 | 32 | 0.0 | 16 |
| support-covered | combined-proxy | 12289 | 56 | 96 | 56 | 80 | 48 | 4 | 14 | 0 | 4 | -161.7 | 204.1 | 204.1 | 275.5 | 507.2 | 81 | -523.3 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 16 | 32 | 0.0 | 16 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
