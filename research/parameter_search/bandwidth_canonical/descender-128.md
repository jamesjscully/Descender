# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `8`
- accepted candidates before truncation: `4664610`
- candidates shown: `500`
- rejected candidates: `3015390`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 32 | 48 | 32 | 24 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 32 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 48 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 64 | 4 | -248.2 | 128.8 | 132.5 | 128.8 | 226.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 8 |
| 32771 | 32 | 48 | 32 | 24 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 |
| 32771 | 32 | 48 | 32 | 32 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 |
| 32771 | 32 | 48 | 32 | 48 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 |
| 32771 | 32 | 48 | 32 | 64 | 3 | -256.0 | 131.9 | 136.6 | 131.9 | 255.4 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 10 |
| 32771 | 32 | 52 | 32 | 24 | 4 | -258.2 | 132.5 | 132.5 | 156.6 | 213.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 32 | 4 | -258.2 | 132.5 | 132.5 | 156.6 | 213.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 48 | 4 | -258.2 | 132.5 | 132.5 | 156.6 | 213.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 64 | 4 | -258.2 | 132.5 | 132.5 | 156.6 | 213.7 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 12 |
| 32771 | 32 | 52 | 32 | 24 | 5 | -258.4 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 10 |
| 32771 | 32 | 52 | 32 | 32 | 5 | -258.4 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 10 |
| 32771 | 32 | 52 | 32 | 48 | 5 | -258.4 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 10 |
| 32771 | 32 | 52 | 32 | 64 | 5 | -258.4 | 129.3 | 129.3 | 153.6 | 189.4 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 10 |
| 32771 | 32 | 48 | 32 | 24 | 2 | -265.8 | 136.0 | 142.0 | 136.0 | 294.2 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 12 |
| 32771 | 32 | 48 | 32 | 32 | 2 | -265.8 | 136.0 | 142.0 | 136.0 | 294.2 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 12 |
| 32771 | 32 | 48 | 32 | 48 | 2 | -265.8 | 136.0 | 142.0 | 136.0 | 294.2 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 12 |
| 32771 | 32 | 48 | 32 | 64 | 2 | -265.8 | 136.0 | 142.0 | 136.0 | 294.2 | 64 | 0 | 0 | 0 | 0 | 160 | 224 | 192 | 12 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
