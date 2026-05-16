# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- m0 source: direct values `24, 28, 32, 36, 40, 44, 48, 56, 64, 72, 80, 88, 96, 112, 128, 160, 192, 224, 256, 320, 384, 512`
- view coordinate counts: `0`
- security extra cap: `24.0` bits
- guard slack target: `16`
- combined certification weight: `0.250`
- accepted candidates before truncation: `72052435`
- candidates shown: `800`
- rejected candidates: `23947565`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | 65537 | 56 | 88 | 56 | 144 | 32 | 3 | 17 | 0 | 3 | -235.6 | 258.6 | 258.6 | 264.3 | 491.7 | 73 | -490.1 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 26 | 26 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 56 | 88 | 56 | 144 | 48 | 3 | 17 | 0 | 3 | -235.6 | 258.6 | 258.6 | 271.8 | 491.7 | 73 | -490.1 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 26 | 26 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 56 | 88 | 56 | 144 | 64 | 3 | 17 | 0 | 3 | -235.6 | 258.6 | 258.6 | 271.8 | 491.7 | 73 | -490.1 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 26 | 26 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 64 | 96 | 64 | 160 | 48 | 4 | 16 | 0 | 4 | -238.4 | 257.4 | 272.2 | 257.4 | 473.4 | 88 | -609.9 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 64 | 96 | 64 | 160 | 64 | 4 | 16 | 0 | 4 | -238.4 | 257.4 | 272.2 | 257.4 | 473.4 | 88 | -609.9 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 32 | 4 | 17 | 0 | 4 | -239.0 | 271.1 | 271.1 | 274.5 | 495.5 | 74 | -525.9 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 48 | 4 | 17 | 0 | 4 | -239.0 | 271.1 | 271.1 | 282.3 | 495.5 | 74 | -525.9 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 64 | 4 | 17 | 0 | 4 | -239.0 | 271.1 | 271.1 | 282.3 | 495.5 | 74 | -525.9 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 24 | 24 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 32 | 5 | 17 | 0 | 5 | -239.3 | 265.4 | 265.4 | 269.8 | 453.8 | 76 | -535.9 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 48 | 5 | 17 | 0 | 5 | -239.3 | 265.4 | 265.4 | 277.5 | 453.8 | 76 | -535.9 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 64 | 5 | 17 | 0 | 5 | -239.3 | 265.4 | 265.4 | 277.5 | 453.8 | 76 | -535.9 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 32 | 6 | 17 | 0 | 6 | -239.5 | 260.7 | 260.7 | 266.0 | 419.1 | 78 | -544.2 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 48 | 6 | 17 | 0 | 6 | -239.5 | 260.7 | 260.7 | 273.5 | 419.1 | 78 | -544.2 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | 131071 | 56 | 88 | 56 | 144 | 64 | 6 | 17 | 0 | 6 | -239.5 | 260.7 | 260.7 | 273.5 | 419.1 | 78 | -544.2 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | 12289 | 72 | 112 | 72 | 184 | 48 | 4 | 14 | 0 | 4 | -242.6 | 262.5 | 262.5 | 282.0 | 394.9 | 103 | -588.7 | 64 | 0 | 0 | 0 | 0 | 322 | 386 | 354 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | 12289 | 72 | 112 | 72 | 184 | 64 | 4 | 14 | 0 | 4 | -242.6 | 262.5 | 262.5 | 282.0 | 394.9 | 103 | -588.7 | 64 | 0 | 0 | 0 | 0 | 322 | 386 | 354 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 56 | 88 | 56 | 144 | 32 | 2 | 17 | 0 | 2 | -245.1 | 268.1 | 268.1 | 272.0 | 561.6 | 70 | -472.6 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 56 | 88 | 56 | 144 | 48 | 2 | 17 | 0 | 2 | -245.1 | 268.1 | 268.1 | 279.8 | 561.6 | 70 | -472.6 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | 65537 | 56 | 88 | 56 | 144 | 64 | 2 | 17 | 0 | 2 | -245.1 | 268.1 | 268.1 | 279.8 | 561.6 | 70 | -472.6 | 64 | 0 | 0 | 0 | 0 | 306 | 370 | 338 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | 40961 | 64 | 96 | 64 | 160 | 48 | 3 | 16 | 0 | 3 | -246.1 | 263.4 | 280.3 | 263.4 | 531.4 | 85 | -597.1 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 26 | 26 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
