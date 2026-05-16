# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `12`
- accepted candidates before truncation: `4389393`
- candidates shown: `80`
- rejected candidates: `3290607`

| q | n | m0 | m1 | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16411 | 56 | 88 | 56 | 32 | 2 | 11 | 5 | 7 | -208.6 | 197.9 | 197.9 | 214.7 | 221.5 | 64 | 0 | 0 | 0 | 0 | 198 | 262 | 230 | 18 |
| 16411 | 56 | 88 | 56 | 48 | 2 | 11 | 5 | 7 | -208.6 | 197.9 | 197.9 | 220.7 | 221.5 | 64 | 0 | 0 | 0 | 0 | 198 | 262 | 230 | 18 |
| 16411 | 56 | 88 | 56 | 64 | 2 | 11 | 5 | 7 | -208.6 | 197.9 | 197.9 | 220.7 | 221.5 | 64 | 0 | 0 | 0 | 0 | 198 | 262 | 230 | 18 |
| 16411 | 56 | 88 | 56 | 32 | 3 | 11 | 5 | 8 | -208.8 | 194.3 | 194.3 | 211.8 | 195.5 | 64 | 0 | 0 | 0 | 0 | 198 | 262 | 230 | 16 |
| 16411 | 56 | 88 | 56 | 48 | 3 | 11 | 5 | 8 | -208.8 | 194.3 | 194.3 | 217.7 | 195.5 | 64 | 0 | 0 | 0 | 0 | 198 | 262 | 230 | 16 |
| 16411 | 56 | 88 | 56 | 64 | 3 | 11 | 5 | 8 | -208.8 | 194.3 | 194.3 | 217.7 | 195.5 | 64 | 0 | 0 | 0 | 0 | 198 | 262 | 230 | 16 |
| 8209 | 64 | 96 | 64 | 48 | 2 | 10 | 5 | 7 | -212.5 | 203.8 | 203.8 | 207.0 | 207.1 | 64 | 0 | 0 | 0 | 0 | 200 | 264 | 232 | 18 |
| 8209 | 64 | 96 | 64 | 64 | 2 | 10 | 5 | 7 | -212.5 | 203.8 | 203.8 | 207.0 | 207.1 | 64 | 0 | 0 | 0 | 0 | 200 | 264 | 232 | 18 |
| 8209 | 64 | 96 | 64 | 32 | 2 | 10 | 5 | 7 | -212.6 | 201.4 | 203.8 | 201.4 | 207.1 | 64 | 0 | 0 | 0 | 0 | 200 | 264 | 232 | 18 |
| 65537 | 48 | 80 | 48 | 32 | 2 | 13 | 5 | 7 | -214.4 | 203.2 | 203.2 | 243.6 | 267.9 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 18 |
| 65537 | 48 | 80 | 48 | 48 | 2 | 13 | 5 | 7 | -214.4 | 203.2 | 203.2 | 250.4 | 267.9 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 18 |
| 65537 | 48 | 80 | 48 | 64 | 2 | 13 | 5 | 7 | -214.4 | 203.2 | 203.2 | 250.4 | 267.9 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 18 |
| 32771 | 48 | 80 | 48 | 32 | 2 | 13 | 3 | 5 | -214.5 | 193.9 | 193.9 | 234.8 | 277.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 22 |
| 32771 | 48 | 80 | 48 | 48 | 2 | 13 | 3 | 5 | -214.5 | 193.9 | 193.9 | 241.3 | 277.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 22 |
| 32771 | 48 | 80 | 48 | 64 | 2 | 13 | 3 | 5 | -214.5 | 193.9 | 193.9 | 241.3 | 277.2 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 22 |
| 65537 | 48 | 80 | 48 | 32 | 3 | 13 | 5 | 8 | -214.5 | 200.1 | 200.1 | 240.7 | 244.8 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 16 |
| 65537 | 48 | 80 | 48 | 48 | 3 | 13 | 5 | 8 | -214.5 | 200.1 | 200.1 | 247.4 | 244.8 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 16 |
| 65537 | 48 | 80 | 48 | 64 | 3 | 13 | 5 | 8 | -214.5 | 200.1 | 200.1 | 247.4 | 244.8 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 16 |
| 65537 | 48 | 80 | 48 | 32 | 4 | 13 | 5 | 9 | -214.7 | 197.4 | 197.4 | 238.1 | 224.3 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 14 |
| 65537 | 48 | 80 | 48 | 48 | 4 | 13 | 5 | 9 | -214.7 | 197.4 | 197.4 | 244.8 | 224.3 | 64 | 0 | 0 | 0 | 0 | 208 | 272 | 240 | 14 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
