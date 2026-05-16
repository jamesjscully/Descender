# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `12`
- accepted candidates before truncation: `4975848`
- candidates shown: `500`
- rejected candidates: `2704152`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 48 | 72 | 48 | 32 | 3 | -247.8 | 195.0 | 204.8 | 195.0 | 383.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 18 |
| 32771 | 48 | 72 | 48 | 48 | 3 | -247.8 | 195.0 | 204.8 | 195.0 | 383.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 18 |
| 32771 | 48 | 72 | 48 | 64 | 3 | -247.8 | 195.0 | 204.8 | 195.0 | 383.1 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 18 |
| 32771 | 48 | 80 | 48 | 32 | 4 | -253.5 | 198.7 | 198.7 | 239.4 | 314.3 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 24 |
| 32771 | 48 | 80 | 48 | 48 | 4 | -253.5 | 198.7 | 198.7 | 246.1 | 314.3 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 24 |
| 32771 | 48 | 80 | 48 | 64 | 4 | -253.5 | 198.7 | 198.7 | 246.1 | 314.3 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 24 |
| 32771 | 48 | 80 | 48 | 32 | 5 | -253.7 | 193.9 | 193.9 | 234.8 | 277.2 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 22 |
| 32771 | 48 | 80 | 48 | 48 | 5 | -253.7 | 193.9 | 193.9 | 241.3 | 277.2 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 22 |
| 32771 | 48 | 80 | 48 | 64 | 5 | -253.7 | 193.9 | 193.9 | 241.3 | 277.2 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 22 |
| 32771 | 48 | 72 | 48 | 32 | 2 | -257.5 | 201.0 | 213.0 | 201.0 | 441.4 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 20 |
| 32771 | 48 | 72 | 48 | 48 | 2 | -257.5 | 201.0 | 213.0 | 201.0 | 441.4 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 20 |
| 32771 | 48 | 72 | 48 | 64 | 2 | -257.5 | 201.0 | 213.0 | 201.0 | 441.4 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 20 |
| 32771 | 48 | 72 | 48 | 24 | 2 | -257.9 | 193.7 | 213.0 | 193.7 | 441.4 | 64 | 0 | 0 | 0 | 0 | 240 | 304 | 272 | 20 |
| 32771 | 48 | 80 | 48 | 32 | 3 | -261.1 | 204.8 | 204.8 | 245.2 | 360.7 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 26 |
| 32771 | 48 | 80 | 48 | 48 | 3 | -261.1 | 204.8 | 204.8 | 252.1 | 360.7 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 26 |
| 32771 | 48 | 80 | 48 | 64 | 3 | -261.1 | 204.8 | 204.8 | 252.1 | 360.7 | 64 | 0 | 0 | 0 | 0 | 256 | 320 | 288 | 26 |
| 32771 | 48 | 88 | 48 | 32 | 4 | -267.3 | 198.7 | 198.7 | 241.0 | 288.9 | 64 | 0 | 0 | 0 | 0 | 272 | 336 | 304 | 32 |
| 32771 | 48 | 88 | 48 | 48 | 4 | -267.3 | 198.7 | 198.7 | 300.7 | 288.9 | 64 | 0 | 0 | 0 | 0 | 272 | 336 | 304 | 32 |
| 32771 | 48 | 88 | 48 | 64 | 4 | -267.3 | 198.7 | 198.7 | 300.7 | 288.9 | 64 | 0 | 0 | 0 | 0 | 272 | 336 | 304 | 32 |
| 32771 | 48 | 88 | 48 | 32 | 5 | -267.5 | 193.9 | 193.9 | 236.4 | 249.5 | 64 | 0 | 0 | 0 | 0 | 272 | 336 | 304 | 30 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
