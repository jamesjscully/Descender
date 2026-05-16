# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `combined-proxy`
- m0 source: direct values `8, 12, 16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96`
- view coordinate counts: `0`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.100`
- accepted candidates before truncation: `10339234`
- candidates shown: `200`
- rejected candidates: `2460766`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 24 | 4 | 16 | 0 | 4 | -247.7 | 131.5 | 136.1 | 131.5 | 236.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 32 | 4 | 16 | 0 | 4 | -247.7 | 131.5 | 136.1 | 131.5 | 236.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 48 | 4 | 16 | 0 | 4 | -247.7 | 131.5 | 136.1 | 131.5 | 236.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 64 | 4 | 16 | 0 | 4 | -247.7 | 131.5 | 136.1 | 131.5 | 236.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 24 | 4 | 16 | 0 | 4 | -247.9 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 32 | 4 | 16 | 0 | 4 | -247.9 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 48 | 4 | 16 | 0 | 4 | -247.9 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 64 | 4 | 16 | 0 | 4 | -247.9 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 40 | 8 | 0.0 | 40 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 24 | 5 | 16 | 0 | 5 | -247.9 | 129.1 | 132.9 | 129.1 | 213.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 38 | 6 | 0.0 | 38 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 32 | 5 | 16 | 0 | 5 | -247.9 | 129.1 | 132.9 | 129.1 | 213.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 38 | 6 | 0.0 | 38 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 48 | 5 | 16 | 0 | 5 | -247.9 | 129.1 | 132.9 | 129.1 | 213.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 38 | 6 | 0.0 | 38 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 64 | 5 | 16 | 0 | 5 | -247.9 | 129.1 | 132.9 | 129.1 | 213.5 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 38 | 6 | 0.0 | 38 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 24 | 3 | 16 | 0 | 3 | -255.5 | 134.6 | 140.2 | 134.6 | 265.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 32 | 3 | 16 | 0 | 3 | -255.5 | 134.6 | 140.2 | 134.6 | 265.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 48 | 3 | 16 | 0 | 3 | -255.5 | 134.6 | 140.2 | 134.6 | 265.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |
| full-surjective | combined-proxy | 40961 | 32 | 48 | 32 | 80 | 64 | 3 | 16 | 0 | 3 | -255.5 | 134.6 | 140.2 | 134.6 | 265.7 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 24 | 3 | 16 | 0 | 3 | -255.7 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 32 | 3 | 16 | 0 | 3 | -255.7 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 48 | 3 | 16 | 0 | 3 | -255.7 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |
| full-surjective | combined-proxy | 32771 | 32 | 48 | 32 | 80 | 64 | 3 | 16 | 0 | 3 | -255.7 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 42 | 10 | 0.0 | 42 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
