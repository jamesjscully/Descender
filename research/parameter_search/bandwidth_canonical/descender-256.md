# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `16`
- accepted candidates before truncation: `4828546`
- candidates shown: `500`
- rejected candidates: `2851454`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 64 | 96 | 64 | 48 | 3 | -246.4 | 258.1 | 273.1 | 258.1 | 510.8 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 26 |
| 32771 | 64 | 96 | 64 | 64 | 3 | -246.4 | 258.1 | 273.1 | 258.1 | 510.8 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 26 |
| 32771 | 64 | 104 | 64 | 48 | 4 | -248.6 | 265.0 | 265.0 | 308.0 | 427.5 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 32 |
| 32771 | 64 | 104 | 64 | 64 | 4 | -248.6 | 265.0 | 265.0 | 308.0 | 427.5 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 32 |
| 32771 | 64 | 104 | 64 | 48 | 5 | -248.9 | 258.5 | 258.5 | 302.0 | 378.8 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 30 |
| 32771 | 64 | 104 | 64 | 64 | 5 | -248.9 | 258.5 | 258.5 | 302.0 | 378.8 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 30 |
| 32771 | 64 | 96 | 64 | 48 | 2 | -256.0 | 266.1 | 284.0 | 266.1 | 588.5 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 28 |
| 32771 | 64 | 96 | 64 | 64 | 2 | -256.0 | 266.1 | 284.0 | 266.1 | 588.5 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 28 |
| 32771 | 64 | 104 | 64 | 48 | 3 | -256.2 | 273.1 | 273.1 | 315.4 | 488.4 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 34 |
| 32771 | 64 | 104 | 64 | 64 | 3 | -256.2 | 273.1 | 273.1 | 315.4 | 488.4 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 34 |
| 32771 | 64 | 96 | 64 | 32 | 2 | -256.3 | 258.8 | 284.0 | 258.8 | 588.5 | 64 | 0 | 0 | 0 | 0 | 320 | 384 | 352 | 28 |
| 32771 | 64 | 112 | 64 | 48 | 4 | -259.1 | 265.0 | 265.0 | 356.4 | 402.1 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 40 |
| 32771 | 64 | 112 | 64 | 64 | 4 | -259.1 | 265.0 | 265.0 | 363.0 | 402.1 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 40 |
| 32771 | 64 | 112 | 64 | 48 | 5 | -259.4 | 258.5 | 258.5 | 349.4 | 351.1 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 38 |
| 32771 | 64 | 112 | 64 | 64 | 5 | -259.4 | 258.5 | 258.5 | 355.9 | 351.1 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 38 |
| 32771 | 64 | 104 | 64 | 48 | 2 | -266.2 | 284.0 | 284.0 | 325.3 | 569.9 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 36 |
| 32771 | 64 | 104 | 64 | 64 | 2 | -266.2 | 284.0 | 284.0 | 325.3 | 569.9 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 36 |
| 16411 | 64 | 104 | 64 | 48 | 2 | -266.7 | 261.6 | 261.6 | 304.9 | 506.1 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 36 |
| 16411 | 64 | 104 | 64 | 64 | 2 | -266.7 | 261.6 | 261.6 | 304.9 | 506.1 | 64 | 0 | 0 | 0 | 0 | 336 | 400 | 368 | 36 |
| 32771 | 64 | 112 | 64 | 48 | 3 | -266.7 | 273.1 | 273.1 | 365.1 | 465.9 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 42 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
