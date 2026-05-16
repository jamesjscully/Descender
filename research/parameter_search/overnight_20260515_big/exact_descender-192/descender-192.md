# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- m0 source: direct values `24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- view coordinate counts: `0`
- security extra cap: `24.0` bits
- guard slack target: `12`
- combined certification weight: `0.250`
- accepted candidates before truncation: `72470077`
- candidates shown: `800`
- rejected candidates: `23529923`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 24 | 4 | 17 | 0 | 4 | -234.1 | 193.6 | 193.6 | 204.8 | 350.3 | 53 | -357.2 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 32 | 4 | 17 | 0 | 4 | -234.1 | 193.6 | 193.6 | 212.5 | 350.3 | 53 | -357.2 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 48 | 4 | 17 | 0 | 4 | -234.1 | 193.6 | 193.6 | 212.5 | 350.3 | 53 | -357.2 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 64 | 4 | 17 | 0 | 4 | -234.1 | 193.6 | 193.6 | 212.5 | 350.3 | 53 | -357.2 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 48 | 72 | 48 | 120 | 32 | 4 | 16 | 0 | 4 | -239.9 | 194.5 | 204.2 | 194.5 | 355.1 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 48 | 72 | 48 | 120 | 48 | 4 | 16 | 0 | 4 | -239.9 | 194.5 | 204.2 | 194.5 | 355.1 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 48 | 72 | 48 | 120 | 64 | 4 | 16 | 0 | 4 | -239.9 | 194.5 | 204.2 | 194.5 | 355.1 | 66 | -454.6 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 24 | 3 | 17 | 0 | 3 | -241.9 | 198.7 | 198.7 | 209.1 | 388.0 | 52 | -347.5 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 18 | 18 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 32 | 3 | 17 | 0 | 3 | -241.9 | 198.7 | 198.7 | 217.1 | 388.0 | 52 | -347.5 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 18 | 18 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 48 | 3 | 17 | 0 | 3 | -241.9 | 198.7 | 198.7 | 217.1 | 388.0 | 52 | -347.5 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 18 | 18 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 64 | 40 | 104 | 64 | 3 | 17 | 0 | 3 | -241.9 | 198.7 | 198.7 | 217.1 | 388.0 | 52 | -347.5 | 64 | 0 | 0 | 0 | 0 | 221 | 285 | 253 | 18 | 18 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 48 | 72 | 48 | 120 | 32 | 3 | 16 | 0 | 3 | -247.6 | 199.0 | 210.2 | 199.0 | 398.6 | 64 | -444.9 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 18 | 18 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 48 | 72 | 48 | 120 | 48 | 3 | 16 | 0 | 3 | -247.6 | 199.0 | 210.2 | 199.0 | 398.6 | 64 | -444.9 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 18 | 18 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 48 | 72 | 48 | 120 | 64 | 3 | 16 | 0 | 3 | -247.6 | 199.0 | 210.2 | 199.0 | 398.6 | 64 | -444.9 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 18 | 18 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 72 | 40 | 112 | 24 | 4 | 17 | 0 | 4 | -248.8 | 193.6 | 193.6 | 206.2 | 325.0 | 53 | -253.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 72 | 40 | 112 | 32 | 4 | 17 | 0 | 4 | -248.8 | 193.6 | 193.6 | 267.8 | 325.0 | 53 | -253.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 72 | 40 | 112 | 48 | 4 | 17 | 0 | 4 | -248.8 | 193.6 | 193.6 | 275.4 | 325.0 | 53 | -253.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 40 | 72 | 40 | 112 | 64 | 4 | 17 | 0 | 4 | -248.8 | 193.6 | 193.6 | 275.4 | 325.0 | 53 | -253.9 | 64 | 0 | 0 | 0 | 0 | 238 | 302 | 270 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 40 | 64 | 40 | 104 | 24 | 1 | 16 | 0 | 1 | -251.3 | 192.3 | 192.3 | 203.7 | 448.0 | 48 | -287.9 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 40 | 64 | 40 | 104 | 32 | 1 | 16 | 0 | 1 | -251.3 | 192.3 | 192.3 | 211.4 | 448.0 | 48 | -287.9 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 22 | 22 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
