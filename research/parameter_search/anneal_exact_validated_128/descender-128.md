# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- m0 source: direct values `48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320`
- view coordinate counts: `0`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `3161605`
- candidates shown: `200`
- rejected candidates: `678395`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 24 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 32 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 48 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 64 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 24 | 4 | 16 | 0 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 44 | -294.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 32 | 4 | 16 | 0 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 44 | -294.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 48 | 4 | 16 | 0 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 44 | -294.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 64 | 4 | 16 | 0 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 44 | -294.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 24 | 3 | 16 | 0 | 3 | -255.9 | 134.6 | 140.2 | 134.6 | 265.7 | 43 | -292.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 32 | 3 | 16 | 0 | 3 | -255.9 | 134.6 | 140.2 | 134.6 | 265.7 | 43 | -292.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 48 | 3 | 16 | 0 | 3 | -255.9 | 134.6 | 140.2 | 134.6 | 265.7 | 43 | -292.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 64 | 3 | 16 | 0 | 3 | -255.9 | 134.6 | 140.2 | 134.6 | 265.7 | 43 | -292.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 24 | 3 | 16 | 0 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 43 | -287.9 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 32 | 3 | 16 | 0 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 43 | -287.9 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 48 | 3 | 16 | 0 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 43 | -287.9 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 32771 | 32 | 48 | 32 | 80 | 64 | 3 | 16 | 0 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 43 | -287.9 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 32 | 48 | 32 | 80 | 24 | 4 | 17 | 0 | 4 | -260.3 | 137.3 | 143.7 | 137.3 | 258.4 | 43 | -309.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 32 | 48 | 32 | 80 | 32 | 4 | 17 | 0 | 4 | -260.3 | 137.3 | 143.7 | 137.3 | 258.4 | 43 | -309.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 32 | 48 | 32 | 80 | 48 | 4 | 17 | 0 | 4 | -260.3 | 137.3 | 143.7 | 137.3 | 258.4 | 43 | -309.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 32 | 48 | 32 | 80 | 64 | 4 | 17 | 0 | 4 | -260.3 | 137.3 | 143.7 | 137.3 | 258.4 | 43 | -309.5 | 64 | 0 | 0 | 0 | 0 | 170 | 234 | 202 | 8 | 8 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
