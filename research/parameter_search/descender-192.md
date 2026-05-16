# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.

- target security bits: `192`
- accepted candidates before truncation: `1059`
- candidates shown: `50`
- rejected candidates: `14061`

| q | n | m0 | m1 | k | eta | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 65537 | 128 | 384 | 512 | 16 | 1 | 625.5 | 645.8 | 625.5 | 627.9 | 56928 | 2720 | 254 |
| 40961 | 192 | 448 | 768 | 16 | 1 | 618.3 | 923.1 | 618.3 | 1014.5 | 58208 | 2464 | 254 |
| 65537 | 128 | 256 | 512 | 16 | 1 | 618.1 | 645.8 | 618.1 | 830.8 | 56544 | 2336 | 126 |
| 65537 | 128 | 192 | 512 | 16 | 1 | 613.9 | 645.8 | 613.9 | 932.2 | 56352 | 2144 | 62 |
| 40961 | 192 | 320 | 768 | 16 | 1 | 613.0 | 923.1 | 613.0 | 1217.4 | 57952 | 2208 | 126 |
| 40961 | 192 | 256 | 768 | 16 | 1 | 610.1 | 923.1 | 610.1 | 1318.8 | 57824 | 2080 | 62 |
| 40961 | 128 | 256 | 512 | 16 | 1 | 596.4 | 615.4 | 596.4 | 744.0 | 38752 | 1568 | 126 |
| 40961 | 128 | 192 | 512 | 16 | 1 | 592.2 | 615.4 | 592.2 | 845.4 | 38624 | 1440 | 62 |
| 40961 | 192 | 256 | 768 | 16 | 2 | 564.2 | 873.6 | 586.6 | 564.2 | 57824 | 2080 | 60 |
| 12289 | 192 | 448 | 768 | 16 | 1 | 562.7 | 806.4 | 562.7 | 681.0 | 58208 | 2464 | 254 |
| 12289 | 192 | 320 | 768 | 16 | 1 | 557.4 | 806.4 | 557.4 | 883.9 | 57952 | 2208 | 126 |
| 12289 | 192 | 256 | 768 | 16 | 1 | 554.6 | 806.4 | 554.6 | 985.3 | 57824 | 2080 | 62 |
| 40961 | 128 | 384 | 512 | 16 | 1 | 541.1 | 615.4 | 603.8 | 541.1 | 39008 | 1824 | 254 |
| 7681 | 192 | 448 | 768 | 16 | 1 | 541.0 | 760.8 | 541.0 | 550.8 | 58208 | 2464 | 254 |
| 12289 | 128 | 192 | 512 | 16 | 1 | 536.6 | 537.6 | 536.6 | 623.1 | 38624 | 1440 | 62 |
| 7681 | 192 | 320 | 768 | 16 | 1 | 535.7 | 760.8 | 535.7 | 753.7 | 57952 | 2208 | 126 |
| 7681 | 192 | 256 | 768 | 16 | 1 | 532.9 | 760.8 | 532.9 | 855.2 | 57824 | 2080 | 62 |
| 12289 | 128 | 256 | 512 | 16 | 1 | 521.6 | 537.6 | 540.8 | 521.6 | 38752 | 1568 | 126 |
| 7681 | 128 | 192 | 512 | 16 | 1 | 507.2 | 507.2 | 514.9 | 536.3 | 38624 | 1440 | 62 |
| 3329 | 192 | 320 | 768 | 16 | 1 | 497.1 | 679.8 | 497.1 | 522.1 | 57952 | 2208 | 126 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
