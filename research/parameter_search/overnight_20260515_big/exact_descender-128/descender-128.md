# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- m0 source: direct values `24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- view coordinate counts: `0`
- security extra cap: `24.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `72834004`
- candidates shown: `800`
- rejected candidates: `23165996`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | 65537 | 28 | 44 | 28 | 72 | 16 | 3 | 17 | 0 | 3 | -246.5 | 129.3 | 129.3 | 131.5 | 245.9 | 37 | -238.9 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 28 | 44 | 28 | 72 | 24 | 3 | 17 | 0 | 3 | -246.5 | 129.3 | 129.3 | 138.9 | 245.9 | 37 | -238.9 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 28 | 44 | 28 | 72 | 32 | 3 | 17 | 0 | 3 | -246.5 | 129.3 | 129.3 | 138.9 | 245.9 | 37 | -238.9 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 28 | 44 | 28 | 72 | 48 | 3 | 17 | 0 | 3 | -246.5 | 129.3 | 129.3 | 138.9 | 245.9 | 37 | -238.9 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 28 | 44 | 28 | 72 | 64 | 3 | 17 | 0 | 3 | -246.5 | 129.3 | 129.3 | 138.9 | 245.9 | 37 | -238.9 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 24 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 32 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 48 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 64 | 4 | 16 | 0 | 4 | -248.0 | 131.5 | 136.1 | 131.5 | 236.7 | 44 | -299.3 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 28 | 44 | 28 | 72 | 16 | 4 | 17 | 0 | 4 | -250.2 | 135.5 | 135.5 | 136.6 | 247.8 | 37 | -256.5 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 28 | 44 | 28 | 72 | 24 | 4 | 17 | 0 | 4 | -250.2 | 135.5 | 135.5 | 144.4 | 247.8 | 37 | -256.5 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 28 | 44 | 28 | 72 | 32 | 4 | 17 | 0 | 4 | -250.2 | 135.5 | 135.5 | 144.4 | 247.8 | 37 | -256.5 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 28 | 44 | 28 | 72 | 48 | 4 | 17 | 0 | 4 | -250.2 | 135.5 | 135.5 | 144.4 | 247.8 | 37 | -256.5 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 28 | 44 | 28 | 72 | 64 | 4 | 17 | 0 | 4 | -250.2 | 135.5 | 135.5 | 144.4 | 247.8 | 37 | -256.5 | 64 | 0 | 0 | 0 | 0 | 153 | 217 | 185 | 8 | 8 | 0.0 | 0 |
| full-surjective | guard-only | 12289 | 36 | 56 | 36 | 92 | 24 | 4 | 14 | 0 | 4 | -250.9 | 131.2 | 131.2 | 143.3 | 197.4 | 52 | -289.6 | 64 | 0 | 0 | 0 | 0 | 161 | 225 | 193 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | 12289 | 36 | 56 | 36 | 92 | 32 | 4 | 14 | 0 | 4 | -250.9 | 131.2 | 131.2 | 143.3 | 197.4 | 52 | -289.6 | 64 | 0 | 0 | 0 | 0 | 161 | 225 | 193 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | 12289 | 36 | 56 | 36 | 92 | 48 | 4 | 14 | 0 | 4 | -250.9 | 131.2 | 131.2 | 143.3 | 197.4 | 52 | -289.6 | 64 | 0 | 0 | 0 | 0 | 161 | 225 | 193 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | 12289 | 36 | 56 | 36 | 92 | 64 | 4 | 14 | 0 | 4 | -250.9 | 131.2 | 131.2 | 143.3 | 197.4 | 52 | -289.6 | 64 | 0 | 0 | 0 | 0 | 161 | 225 | 193 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 24 | 3 | 16 | 0 | 3 | -255.9 | 134.6 | 140.2 | 134.6 | 265.7 | 43 | -292.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 32 | 48 | 32 | 80 | 32 | 3 | 16 | 0 | 3 | -255.9 | 134.6 | 140.2 | 134.6 | 265.7 | 43 | -292.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 | 10 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
