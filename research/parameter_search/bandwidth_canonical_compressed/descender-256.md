# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `16`
- accepted candidates before truncation: `4157189`
- candidates shown: `80`
- rejected candidates: `3522811`

| q | n | m0 | m1 | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 65537 | 64 | 104 | 64 | 48 | 2 | 12 | 9 | 11 | -198.0 | 257.1 | 257.1 | 300.7 | 264.0 | 64 | 0 | 0 | 0 | 0 | 252 | 316 | 284 | 18 |
| 65537 | 64 | 104 | 64 | 64 | 2 | 12 | 9 | 11 | -198.0 | 257.1 | 257.1 | 300.7 | 264.0 | 64 | 0 | 0 | 0 | 0 | 252 | 316 | 284 | 18 |
| 65537 | 64 | 96 | 64 | 48 | 2 | 13 | 5 | 7 | -202.1 | 256.4 | 270.9 | 256.4 | 398.9 | 64 | 0 | 0 | 0 | 0 | 260 | 324 | 292 | 18 |
| 65537 | 64 | 96 | 64 | 64 | 2 | 13 | 5 | 7 | -202.1 | 256.4 | 270.9 | 256.4 | 398.9 | 64 | 0 | 0 | 0 | 0 | 260 | 324 | 292 | 18 |
| 65537 | 64 | 104 | 64 | 48 | 2 | 13 | 5 | 7 | -210.1 | 270.9 | 270.9 | 313.3 | 367.6 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 26 |
| 65537 | 64 | 104 | 64 | 64 | 2 | 13 | 5 | 7 | -210.1 | 270.9 | 270.9 | 313.3 | 367.6 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 26 |
| 65537 | 64 | 104 | 64 | 48 | 3 | 13 | 5 | 8 | -210.3 | 266.8 | 266.8 | 309.6 | 337.3 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 24 |
| 65537 | 64 | 104 | 64 | 64 | 3 | 13 | 5 | 8 | -210.3 | 266.8 | 266.8 | 309.6 | 337.3 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 24 |
| 32771 | 64 | 104 | 64 | 48 | 2 | 13 | 3 | 5 | -210.4 | 258.5 | 258.5 | 302.0 | 378.8 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 30 |
| 32771 | 64 | 104 | 64 | 64 | 2 | 13 | 3 | 5 | -210.4 | 258.5 | 258.5 | 302.0 | 378.8 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 30 |
| 65537 | 64 | 104 | 64 | 48 | 4 | 13 | 5 | 9 | -210.5 | 263.2 | 263.2 | 306.4 | 310.3 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 22 |
| 65537 | 64 | 104 | 64 | 64 | 4 | 13 | 5 | 9 | -210.5 | 263.2 | 263.2 | 306.4 | 310.3 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 22 |
| 65537 | 64 | 104 | 64 | 48 | 5 | 13 | 5 | 10 | -210.7 | 260.0 | 260.0 | 303.4 | 286.1 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 20 |
| 65537 | 64 | 104 | 64 | 64 | 5 | 13 | 5 | 10 | -210.7 | 260.0 | 260.0 | 303.4 | 286.1 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 20 |
| 65537 | 64 | 104 | 64 | 48 | 6 | 13 | 5 | 11 | -210.9 | 257.1 | 257.1 | 300.7 | 264.0 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 18 |
| 65537 | 64 | 104 | 64 | 64 | 6 | 13 | 5 | 11 | -210.9 | 257.1 | 257.1 | 300.7 | 264.0 | 64 | 0 | 0 | 0 | 0 | 273 | 337 | 305 | 18 |
| 65537 | 64 | 96 | 64 | 48 | 2 | 14 | 3 | 5 | -214.0 | 263.8 | 280.9 | 263.8 | 470.5 | 64 | 0 | 0 | 0 | 0 | 280 | 344 | 312 | 22 |
| 65537 | 64 | 96 | 64 | 64 | 2 | 14 | 3 | 5 | -214.0 | 263.8 | 280.9 | 263.8 | 470.5 | 64 | 0 | 0 | 0 | 0 | 280 | 344 | 312 | 22 |
| 65537 | 64 | 96 | 64 | 48 | 3 | 14 | 3 | 6 | -214.2 | 259.8 | 275.5 | 259.8 | 431.9 | 64 | 0 | 0 | 0 | 0 | 280 | 344 | 312 | 20 |
| 65537 | 64 | 96 | 64 | 64 | 3 | 14 | 3 | 6 | -214.2 | 259.8 | 275.5 | 259.8 | 431.9 | 64 | 0 | 0 | 0 | 0 | 280 | 344 | 312 | 20 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
