# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.

- target security bits: `128`
- accepted candidates before truncation: `62714`
- candidates shown: `500`
- rejected candidates: `1135366`

| q | n | m0 | m1 | k | eta | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 4194319 | 96 | 608 | 192 | 24 | 1 | 618.9 | 685.9 | 618.9 | 844.0 | 32160 | 2432 | 510 |
| 4194319 | 96 | 480 | 192 | 24 | 1 | 612.7 | 685.9 | 612.7 | 1046.9 | 31776 | 2048 | 382 |
| 4194319 | 96 | 352 | 288 | 16 | 1 | 610.9 | 685.9 | 610.9 | 1097.6 | 32544 | 1952 | 254 |
| 4194319 | 96 | 288 | 288 | 16 | 1 | 607.1 | 685.9 | 607.1 | 1199.1 | 32352 | 1760 | 190 |
| 4194319 | 96 | 352 | 192 | 24 | 1 | 605.0 | 685.9 | 605.0 | 1249.8 | 31392 | 1664 | 254 |
| 4194319 | 96 | 224 | 288 | 16 | 1 | 602.8 | 685.9 | 602.8 | 1300.5 | 32160 | 1568 | 126 |
| 4194319 | 96 | 288 | 192 | 24 | 1 | 600.5 | 685.9 | 600.5 | 1351.2 | 31200 | 1472 | 190 |
| 4194319 | 96 | 192 | 288 | 16 | 1 | 600.5 | 685.9 | 600.5 | 1351.2 | 32064 | 1472 | 94 |
| 4194319 | 96 | 160 | 288 | 16 | 1 | 598.0 | 685.9 | 598.0 | 1401.9 | 31968 | 1376 | 62 |
| 4194319 | 96 | 144 | 288 | 16 | 1 | 596.6 | 685.9 | 596.6 | 1427.3 | 31920 | 1328 | 46 |
| 4194319 | 96 | 224 | 192 | 24 | 1 | 595.3 | 685.9 | 595.3 | 1452.7 | 31008 | 1280 | 126 |
| 2097169 | 96 | 608 | 192 | 24 | 1 | 594.9 | 652.3 | 594.9 | 748.0 | 32160 | 2432 | 510 |
| 4194319 | 96 | 352 | 288 | 16 | 2 | 593.2 | 661.2 | 593.2 | 626.0 | 32544 | 1952 | 252 |
| 4194319 | 96 | 192 | 192 | 24 | 1 | 592.3 | 685.9 | 592.3 | 1503.4 | 30912 | 1184 | 94 |
| 4194319 | 96 | 288 | 288 | 16 | 2 | 589.4 | 661.2 | 589.4 | 774.6 | 32352 | 1760 | 188 |
| 4194319 | 96 | 160 | 192 | 24 | 1 | 589.1 | 685.9 | 589.1 | 1554.1 | 30816 | 1088 | 62 |
| 2097169 | 96 | 480 | 192 | 24 | 1 | 588.7 | 652.3 | 588.7 | 950.9 | 31776 | 2048 | 382 |
| 4194319 | 96 | 144 | 192 | 24 | 1 | 587.4 | 685.9 | 587.4 | 1579.5 | 30768 | 1040 | 46 |
| 4194319 | 96 | 352 | 192 | 24 | 2 | 587.4 | 661.2 | 587.4 | 848.9 | 31392 | 1664 | 252 |
| 2097169 | 96 | 352 | 288 | 16 | 1 | 586.9 | 652.3 | 586.9 | 1001.6 | 32544 | 1952 | 254 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
