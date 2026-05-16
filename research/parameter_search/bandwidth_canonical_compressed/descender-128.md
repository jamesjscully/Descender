# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `8`
- accepted candidates before truncation: `4300959`
- candidates shown: `80`
- rejected candidates: `3379041`

| q | n | m0 | m1 | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 32 | 52 | 32 | 24 | 2 | 13 | 3 | 5 | -220.3 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 137 | 201 | 169 | 10 |
| 32771 | 32 | 52 | 32 | 32 | 2 | 13 | 3 | 5 | -220.3 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 137 | 201 | 169 | 10 |
| 32771 | 32 | 52 | 32 | 48 | 2 | 13 | 3 | 5 | -220.3 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 137 | 201 | 169 | 10 |
| 32771 | 32 | 52 | 32 | 64 | 2 | 13 | 3 | 5 | -220.3 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 137 | 201 | 169 | 10 |
| 32771 | 32 | 48 | 32 | 24 | 2 | 14 | 2 | 4 | -223.6 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 8 |
| 32771 | 32 | 48 | 32 | 32 | 2 | 14 | 2 | 4 | -223.6 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 8 |
| 32771 | 32 | 48 | 32 | 48 | 2 | 14 | 2 | 4 | -223.6 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 8 |
| 32771 | 32 | 48 | 32 | 64 | 2 | 14 | 2 | 4 | -223.6 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 140 | 204 | 172 | 8 |
| 8209 | 40 | 60 | 40 | 24 | 2 | 11 | 3 | 5 | -223.8 | 133.6 | 133.6 | 135.6 | 174.2 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 10 |
| 8209 | 40 | 60 | 40 | 32 | 2 | 11 | 3 | 5 | -223.8 | 133.6 | 133.6 | 135.6 | 174.2 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 10 |
| 8209 | 40 | 60 | 40 | 48 | 2 | 11 | 3 | 5 | -223.8 | 133.6 | 133.6 | 135.6 | 174.2 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 10 |
| 8209 | 40 | 60 | 40 | 64 | 2 | 11 | 3 | 5 | -223.8 | 133.6 | 133.6 | 135.6 | 174.2 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 10 |
| 8209 | 40 | 60 | 40 | 24 | 3 | 11 | 3 | 6 | -224.0 | 130.2 | 130.2 | 133.1 | 150.1 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 8 |
| 8209 | 40 | 60 | 40 | 32 | 3 | 11 | 3 | 6 | -224.0 | 130.2 | 130.2 | 133.1 | 150.1 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 8 |
| 8209 | 40 | 60 | 40 | 48 | 3 | 11 | 3 | 6 | -224.0 | 130.2 | 130.2 | 133.1 | 150.1 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 8 |
| 8209 | 40 | 60 | 40 | 64 | 3 | 11 | 3 | 6 | -224.0 | 130.2 | 130.2 | 133.1 | 150.1 | 64 | 0 | 0 | 0 | 0 | 138 | 202 | 170 | 8 |
| 32771 | 32 | 56 | 32 | 24 | 2 | 13 | 3 | 5 | -228.0 | 129.3 | 129.3 | 174.0 | 175.6 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 14 |
| 32771 | 32 | 56 | 32 | 32 | 2 | 13 | 3 | 5 | -228.0 | 129.3 | 129.3 | 180.4 | 175.6 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 14 |
| 32771 | 32 | 56 | 32 | 48 | 2 | 13 | 3 | 5 | -228.0 | 129.3 | 129.3 | 180.4 | 175.6 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 14 |
| 32771 | 32 | 56 | 32 | 64 | 2 | 13 | 3 | 5 | -228.0 | 129.3 | 129.3 | 180.4 | 175.6 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 14 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
