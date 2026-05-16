# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.

- target security bits: `128`
- accepted candidates before truncation: `1874`
- candidates shown: `50`
- rejected candidates: `13246`

| q | n | m0 | m1 | k | eta | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 65537 | 96 | 352 | 288 | 16 | 1 | 466.9 | 484.3 | 466.9 | 521.6 | 32544 | 1952 | 254 |
| 40961 | 96 | 160 | 384 | 16 | 1 | 461.6 | 461.6 | 579.6 | 608.7 | 29024 | 1120 | 62 |
| 40961 | 96 | 224 | 384 | 16 | 1 | 461.6 | 461.6 | 585.0 | 507.3 | 29152 | 1248 | 126 |
| 65537 | 96 | 224 | 288 | 16 | 1 | 458.8 | 484.3 | 458.8 | 724.5 | 32160 | 1568 | 126 |
| 40961 | 128 | 384 | 384 | 16 | 1 | 457.2 | 615.4 | 457.2 | 744.0 | 29536 | 1568 | 254 |
| 65537 | 96 | 160 | 288 | 16 | 1 | 454.0 | 484.3 | 454.0 | 825.9 | 31968 | 1376 | 62 |
| 40961 | 96 | 352 | 288 | 16 | 1 | 450.6 | 461.6 | 450.6 | 456.5 | 22304 | 1312 | 254 |
| 40961 | 128 | 256 | 384 | 16 | 1 | 450.6 | 615.4 | 450.6 | 946.8 | 29280 | 1312 | 126 |
| 40961 | 128 | 192 | 384 | 16 | 1 | 446.8 | 615.4 | 446.8 | 1048.3 | 29152 | 1184 | 62 |
| 40961 | 96 | 224 | 288 | 16 | 1 | 442.6 | 461.6 | 442.6 | 659.4 | 22048 | 1056 | 126 |
| 40961 | 96 | 160 | 288 | 16 | 1 | 437.7 | 461.6 | 437.7 | 760.8 | 21920 | 928 | 62 |
| 65537 | 96 | 160 | 288 | 16 | 2 | 436.3 | 459.6 | 436.3 | 495.8 | 31968 | 1376 | 60 |
| 40961 | 128 | 256 | 384 | 16 | 2 | 432.9 | 582.4 | 432.9 | 475.2 | 29280 | 1312 | 124 |
| 40961 | 128 | 192 | 384 | 16 | 2 | 429.2 | 582.4 | 429.2 | 623.8 | 29152 | 1184 | 60 |
| 40961 | 96 | 160 | 288 | 16 | 2 | 420.0 | 436.8 | 420.0 | 430.7 | 21920 | 928 | 60 |
| 12289 | 128 | 384 | 384 | 16 | 1 | 415.5 | 537.6 | 415.5 | 521.6 | 29536 | 1568 | 254 |
| 12289 | 128 | 256 | 384 | 16 | 1 | 408.9 | 537.6 | 408.9 | 724.5 | 29280 | 1312 | 126 |
| 12289 | 128 | 192 | 384 | 16 | 1 | 405.2 | 537.6 | 405.2 | 826.0 | 29152 | 1184 | 62 |
| 12289 | 96 | 160 | 384 | 16 | 1 | 403.2 | 403.2 | 524.0 | 441.9 | 29024 | 1120 | 62 |
| 12289 | 96 | 224 | 288 | 16 | 1 | 400.9 | 403.2 | 400.9 | 492.7 | 22048 | 1056 | 126 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
