# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `12`
- accepted candidates before truncation: `4869468`
- candidates shown: `500`
- rejected candidates: `2810532`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | incidence bytes | diagonal bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 48 | 72 | 48 | 32 | 3 | -249.9 | 195.0 | 204.8 | 195.0 | 383.1 | 224 | 112 | 80 | 848 | 272 | 18 |
| 32771 | 48 | 72 | 48 | 48 | 3 | -250.1 | 195.0 | 204.8 | 195.0 | 383.1 | 288 | 112 | 80 | 912 | 272 | 18 |
| 32771 | 48 | 72 | 48 | 64 | 3 | -250.4 | 195.0 | 204.8 | 195.0 | 383.1 | 352 | 112 | 80 | 976 | 272 | 18 |
| 32771 | 48 | 80 | 48 | 32 | 4 | -255.5 | 198.7 | 198.7 | 239.4 | 314.3 | 224 | 112 | 80 | 864 | 288 | 24 |
| 32771 | 48 | 80 | 48 | 48 | 4 | -255.8 | 198.7 | 198.7 | 246.1 | 314.3 | 288 | 112 | 80 | 928 | 288 | 24 |
| 32771 | 48 | 80 | 48 | 32 | 5 | -255.8 | 193.9 | 193.9 | 234.8 | 277.2 | 224 | 112 | 80 | 864 | 288 | 22 |
| 32771 | 48 | 80 | 48 | 64 | 4 | -256.0 | 198.7 | 198.7 | 246.1 | 314.3 | 352 | 112 | 80 | 992 | 288 | 24 |
| 32771 | 48 | 80 | 48 | 48 | 5 | -256.0 | 193.9 | 193.9 | 241.3 | 277.2 | 288 | 112 | 80 | 928 | 288 | 22 |
| 32771 | 48 | 80 | 48 | 64 | 5 | -256.3 | 193.9 | 193.9 | 241.3 | 277.2 | 352 | 112 | 80 | 992 | 288 | 22 |
| 32771 | 48 | 72 | 48 | 32 | 2 | -259.6 | 201.0 | 213.0 | 201.0 | 441.4 | 224 | 112 | 80 | 848 | 272 | 20 |
| 32771 | 48 | 72 | 48 | 24 | 2 | -259.8 | 193.7 | 213.0 | 193.7 | 441.4 | 192 | 112 | 80 | 816 | 272 | 20 |
| 32771 | 48 | 72 | 48 | 48 | 2 | -259.8 | 201.0 | 213.0 | 201.0 | 441.4 | 288 | 112 | 80 | 912 | 272 | 20 |
| 32771 | 48 | 72 | 48 | 64 | 2 | -260.1 | 201.0 | 213.0 | 201.0 | 441.4 | 352 | 112 | 80 | 976 | 272 | 20 |
| 32771 | 48 | 80 | 48 | 32 | 3 | -263.2 | 204.8 | 204.8 | 245.2 | 360.7 | 224 | 112 | 80 | 864 | 288 | 26 |
| 32771 | 48 | 80 | 48 | 48 | 3 | -263.5 | 204.8 | 204.8 | 252.1 | 360.7 | 288 | 112 | 80 | 928 | 288 | 26 |
| 32771 | 48 | 80 | 48 | 64 | 3 | -263.7 | 204.8 | 204.8 | 252.1 | 360.7 | 352 | 112 | 80 | 992 | 288 | 26 |
| 32771 | 48 | 88 | 48 | 32 | 4 | -269.3 | 198.7 | 198.7 | 241.0 | 288.9 | 224 | 112 | 80 | 880 | 304 | 32 |
| 32771 | 48 | 88 | 48 | 48 | 4 | -269.6 | 198.7 | 198.7 | 300.7 | 288.9 | 288 | 112 | 80 | 944 | 304 | 32 |
| 32771 | 48 | 88 | 48 | 32 | 5 | -269.6 | 193.9 | 193.9 | 236.4 | 249.5 | 224 | 112 | 80 | 880 | 304 | 30 |
| 32771 | 48 | 88 | 48 | 64 | 4 | -269.8 | 198.7 | 198.7 | 300.7 | 288.9 | 352 | 112 | 80 | 1008 | 304 | 32 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
