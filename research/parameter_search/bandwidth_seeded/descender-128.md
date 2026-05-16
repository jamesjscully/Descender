# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `8`
- accepted candidates before truncation: `5068633`
- candidates shown: `500`
- rejected candidates: `2611367`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | incidence bytes | diagonal bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 32 | 48 | 32 | 24 | 4 | -251.8 | 128.8 | 132.5 | 128.8 | 226.4 | 192 | 112 | 80 | 704 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 32 | 4 | -252.1 | 128.8 | 132.5 | 128.8 | 226.4 | 224 | 112 | 80 | 736 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 48 | 4 | -252.6 | 128.8 | 132.5 | 128.8 | 226.4 | 288 | 112 | 80 | 800 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 64 | 4 | -253.1 | 128.8 | 132.5 | 128.8 | 226.4 | 352 | 112 | 80 | 864 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 24 | 3 | -259.7 | 131.9 | 136.6 | 131.9 | 255.4 | 192 | 112 | 80 | 704 | 192 | 10 |
| 32771 | 32 | 48 | 32 | 32 | 3 | -259.9 | 131.9 | 136.6 | 131.9 | 255.4 | 224 | 112 | 80 | 736 | 192 | 10 |
| 32771 | 32 | 48 | 32 | 48 | 3 | -260.4 | 131.9 | 136.6 | 131.9 | 255.4 | 288 | 112 | 80 | 800 | 192 | 10 |
| 32771 | 32 | 48 | 32 | 64 | 3 | -260.9 | 131.9 | 136.6 | 131.9 | 255.4 | 352 | 112 | 80 | 864 | 192 | 10 |
| 32771 | 32 | 52 | 32 | 24 | 4 | -261.8 | 132.5 | 132.5 | 156.6 | 213.7 | 192 | 112 | 80 | 712 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 24 | 5 | -262.0 | 129.3 | 129.3 | 153.6 | 189.4 | 192 | 112 | 80 | 712 | 200 | 10 |
| 32771 | 32 | 52 | 32 | 32 | 4 | -262.1 | 132.5 | 132.5 | 156.6 | 213.7 | 224 | 112 | 80 | 744 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 32 | 5 | -262.3 | 129.3 | 129.3 | 153.6 | 189.4 | 224 | 112 | 80 | 744 | 200 | 10 |
| 32771 | 32 | 52 | 32 | 48 | 4 | -262.6 | 132.5 | 132.5 | 156.6 | 213.7 | 288 | 112 | 80 | 808 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 48 | 5 | -262.8 | 129.3 | 129.3 | 153.6 | 189.4 | 288 | 112 | 80 | 808 | 200 | 10 |
| 32771 | 32 | 52 | 32 | 64 | 4 | -263.1 | 132.5 | 132.5 | 156.6 | 213.7 | 352 | 112 | 80 | 872 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 64 | 5 | -263.2 | 129.3 | 129.3 | 153.6 | 189.4 | 352 | 112 | 80 | 872 | 200 | 10 |
| 32771 | 32 | 48 | 32 | 24 | 2 | -269.4 | 136.0 | 142.0 | 136.0 | 294.2 | 192 | 112 | 80 | 704 | 192 | 12 |
| 32771 | 32 | 48 | 32 | 16 | 2 | -269.6 | 128.7 | 142.0 | 128.7 | 294.2 | 160 | 112 | 80 | 672 | 192 | 12 |
| 32771 | 32 | 52 | 32 | 24 | 3 | -269.6 | 136.6 | 136.6 | 160.4 | 244.2 | 192 | 112 | 80 | 712 | 200 | 14 |
| 32771 | 32 | 48 | 32 | 32 | 2 | -269.7 | 136.0 | 142.0 | 136.0 | 294.2 | 224 | 112 | 80 | 736 | 192 | 12 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
