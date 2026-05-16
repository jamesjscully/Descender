# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `16`
- accepted candidates before truncation: `4799728`
- candidates shown: `500`
- rejected candidates: `2880272`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | incidence bytes | diagonal bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 64 | 96 | 64 | 48 | 3 | -247.6 | 258.1 | 273.1 | 258.1 | 510.8 | 288 | 112 | 80 | 1024 | 352 | 26 |
| 32771 | 64 | 96 | 64 | 64 | 3 | -247.7 | 258.1 | 273.1 | 258.1 | 510.8 | 352 | 112 | 80 | 1088 | 352 | 26 |
| 32771 | 64 | 104 | 64 | 48 | 4 | -249.8 | 265.0 | 265.0 | 308.0 | 427.5 | 288 | 112 | 80 | 1040 | 368 | 32 |
| 32771 | 64 | 104 | 64 | 64 | 4 | -249.9 | 265.0 | 265.0 | 308.0 | 427.5 | 352 | 112 | 80 | 1104 | 368 | 32 |
| 32771 | 64 | 104 | 64 | 48 | 5 | -250.1 | 258.5 | 258.5 | 302.0 | 378.8 | 288 | 112 | 80 | 1040 | 368 | 30 |
| 32771 | 64 | 104 | 64 | 64 | 5 | -250.3 | 258.5 | 258.5 | 302.0 | 378.8 | 352 | 112 | 80 | 1104 | 368 | 30 |
| 32771 | 64 | 96 | 64 | 48 | 2 | -257.2 | 266.1 | 284.0 | 266.1 | 588.5 | 288 | 112 | 80 | 1024 | 352 | 28 |
| 32771 | 64 | 96 | 64 | 64 | 2 | -257.3 | 266.1 | 284.0 | 266.1 | 588.5 | 352 | 112 | 80 | 1088 | 352 | 28 |
| 32771 | 64 | 104 | 64 | 48 | 3 | -257.4 | 273.1 | 273.1 | 315.4 | 488.4 | 288 | 112 | 80 | 1040 | 368 | 34 |
| 32771 | 64 | 96 | 64 | 32 | 2 | -257.4 | 258.8 | 284.0 | 258.8 | 588.5 | 224 | 112 | 80 | 960 | 352 | 28 |
| 32771 | 64 | 104 | 64 | 64 | 3 | -257.6 | 273.1 | 273.1 | 315.4 | 488.4 | 352 | 112 | 80 | 1104 | 368 | 34 |
| 32771 | 64 | 112 | 64 | 48 | 4 | -260.3 | 265.0 | 265.0 | 356.4 | 402.1 | 288 | 112 | 80 | 1056 | 384 | 40 |
| 32771 | 64 | 112 | 64 | 64 | 4 | -260.4 | 265.0 | 265.0 | 363.0 | 402.1 | 352 | 112 | 80 | 1120 | 384 | 40 |
| 32771 | 64 | 112 | 64 | 48 | 5 | -260.7 | 258.5 | 258.5 | 349.4 | 351.1 | 288 | 112 | 80 | 1056 | 384 | 38 |
| 32771 | 64 | 112 | 64 | 64 | 5 | -260.8 | 258.5 | 258.5 | 355.9 | 351.1 | 352 | 112 | 80 | 1120 | 384 | 38 |
| 32771 | 64 | 104 | 64 | 48 | 2 | -267.4 | 284.0 | 284.0 | 325.3 | 569.9 | 288 | 112 | 80 | 1040 | 368 | 36 |
| 32771 | 64 | 104 | 64 | 64 | 2 | -267.5 | 284.0 | 284.0 | 325.3 | 569.9 | 352 | 112 | 80 | 1104 | 368 | 36 |
| 32771 | 64 | 104 | 64 | 32 | 2 | -267.9 | 260.0 | 284.0 | 260.0 | 569.9 | 224 | 112 | 80 | 976 | 368 | 36 |
| 16411 | 64 | 104 | 64 | 48 | 2 | -267.9 | 261.6 | 261.6 | 304.9 | 506.1 | 288 | 112 | 80 | 1040 | 368 | 36 |
| 32771 | 64 | 112 | 64 | 48 | 3 | -267.9 | 273.1 | 273.1 | 365.1 | 465.9 | 288 | 112 | 80 | 1056 | 384 | 42 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
