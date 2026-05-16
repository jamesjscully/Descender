# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only, combined-proxy`
- m0 source: direct values `128, 144, 160, 192, 224, 256, 320, 384, 448, 512, 640, 768, 896, 1024`
- view coordinate counts: `128, 144, 160, 192, 224, 256, 320, 384, 448, 512, 640, 768, 896, 1024`
- security extra cap: `16.0` bits
- guard slack target: `95`
- combined certification weight: `0.250`
- accepted candidates before truncation: `3613838`
- candidates shown: `200`
- rejected candidates: `226162`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 32 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 280.8 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 48 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 406.2 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 64 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 526.3 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 96 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 591.9 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 128 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 591.9 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 64 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 32 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 280.8 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 48 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 406.2 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 64 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 526.3 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 96 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 591.9 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 0 |
| lossy-budget | guard-only | 131071 | 56 | 128 | 56 | 128 | 128 | 3 | 16 | 1 | 4 | -213.4 | 271.1 | 271.1 | 591.9 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 64 | 0.0 | 0 |
| lossy-budget | combined-proxy | 131071 | 56 | 144 | 56 | 128 | 32 | 3 | 16 | 1 | 4 | -215.0 | 271.1 | 271.1 | 282.9 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 80 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 144 | 56 | 128 | 48 | 3 | 16 | 1 | 4 | -215.0 | 271.1 | 271.1 | 409.5 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 80 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 144 | 56 | 128 | 64 | 3 | 16 | 1 | 4 | -215.0 | 271.1 | 271.1 | 531.0 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 80 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 144 | 56 | 128 | 96 | 3 | 16 | 1 | 4 | -215.0 | 271.1 | 271.1 | 712.5 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 80 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 144 | 56 | 128 | 128 | 3 | 16 | 1 | 4 | -215.0 | 271.1 | 271.1 | 712.5 | 546.2 | 74 | -9.4 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 64 | 80 | 0.0 | 64 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 32 | 4 | 16 | 1 | 5 | -215.9 | 265.4 | 265.4 | 276.1 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 62 | 62 | 0.0 | 62 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 48 | 4 | 16 | 1 | 5 | -215.9 | 265.4 | 265.4 | 399.2 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 62 | 62 | 0.0 | 62 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 64 | 4 | 16 | 1 | 5 | -215.9 | 265.4 | 265.4 | 517.0 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 62 | 62 | 0.0 | 62 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 96 | 4 | 16 | 1 | 5 | -215.9 | 265.4 | 265.4 | 581.3 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 62 | 62 | 0.0 | 62 |
| lossy-budget | combined-proxy | 131071 | 56 | 128 | 56 | 128 | 128 | 4 | 16 | 1 | 5 | -215.9 | 265.4 | 265.4 | 581.3 | 509.2 | 76 | -31.6 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 62 | 62 | 0.0 | 62 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
