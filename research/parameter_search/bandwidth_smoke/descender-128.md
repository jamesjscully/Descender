# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- security extra cap: `16.0` bits
- guard slack target: `8`
- accepted candidates before truncation: `250265`
- candidates shown: `100`
- rejected candidates: `69735`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | incidence bytes | diagonal bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 64 | 80 | 64 | 24 | 4 | -419.4 | 136.6 | 265.0 | 136.6 | 503.5 | 192 | 112 | 80 | 896 | 320 | 8 |
| 32771 | 64 | 80 | 64 | 16 | 4 | -419.5 | 129.3 | 265.0 | 129.3 | 503.5 | 160 | 112 | 80 | 864 | 320 | 8 |
| 32771 | 64 | 80 | 64 | 32 | 4 | -419.6 | 136.6 | 265.0 | 136.6 | 503.5 | 224 | 112 | 80 | 928 | 320 | 8 |
| 16411 | 64 | 80 | 64 | 24 | 4 | -419.8 | 128.1 | 242.6 | 128.1 | 439.7 | 192 | 112 | 80 | 896 | 320 | 8 |
| 16411 | 64 | 80 | 64 | 32 | 4 | -420.1 | 128.1 | 242.6 | 128.1 | 439.7 | 224 | 112 | 80 | 928 | 320 | 8 |
| 32771 | 64 | 80 | 64 | 24 | 3 | -427.2 | 139.7 | 273.1 | 139.7 | 555.7 | 192 | 112 | 80 | 896 | 320 | 10 |
| 32771 | 64 | 80 | 64 | 16 | 3 | -427.3 | 132.2 | 273.1 | 132.2 | 555.7 | 160 | 112 | 80 | 864 | 320 | 10 |
| 32771 | 64 | 80 | 64 | 32 | 3 | -427.5 | 139.7 | 273.1 | 139.7 | 555.7 | 224 | 112 | 80 | 928 | 320 | 10 |
| 16411 | 64 | 80 | 64 | 24 | 3 | -427.6 | 131.2 | 250.8 | 131.2 | 491.9 | 192 | 112 | 80 | 896 | 320 | 10 |
| 16411 | 64 | 80 | 64 | 32 | 3 | -427.9 | 131.2 | 250.8 | 131.2 | 491.9 | 224 | 112 | 80 | 928 | 320 | 10 |
| 32771 | 64 | 80 | 64 | 24 | 2 | -437.0 | 143.8 | 284.0 | 143.8 | 625.7 | 192 | 112 | 80 | 896 | 320 | 12 |
| 32771 | 64 | 80 | 64 | 16 | 2 | -437.1 | 136.0 | 284.0 | 136.0 | 625.7 | 160 | 112 | 80 | 864 | 320 | 12 |
| 32771 | 64 | 80 | 64 | 32 | 2 | -437.2 | 143.8 | 284.0 | 143.8 | 625.7 | 224 | 112 | 80 | 928 | 320 | 12 |
| 16411 | 64 | 80 | 64 | 24 | 2 | -437.4 | 135.3 | 261.6 | 135.3 | 561.8 | 192 | 112 | 80 | 896 | 320 | 12 |
| 16411 | 64 | 80 | 64 | 16 | 2 | -437.5 | 128.1 | 261.6 | 128.1 | 561.8 | 160 | 112 | 80 | 864 | 320 | 12 |
| 16411 | 64 | 80 | 64 | 32 | 2 | -437.7 | 135.3 | 261.6 | 135.3 | 561.8 | 224 | 112 | 80 | 928 | 320 | 12 |
| 32771 | 64 | 88 | 64 | 24 | 4 | -439.4 | 188.0 | 265.0 | 188.0 | 478.2 | 192 | 112 | 80 | 912 | 336 | 16 |
| 16411 | 64 | 88 | 64 | 24 | 4 | -439.4 | 176.1 | 242.6 | 176.1 | 414.3 | 192 | 112 | 80 | 912 | 336 | 16 |
| 8209 | 64 | 88 | 64 | 24 | 4 | -439.4 | 164.1 | 220.3 | 164.1 | 350.4 | 192 | 112 | 80 | 912 | 336 | 16 |
| 4099 | 64 | 88 | 64 | 24 | 4 | -439.4 | 152.0 | 197.8 | 152.0 | 286.2 | 192 | 112 | 80 | 912 | 336 | 16 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
