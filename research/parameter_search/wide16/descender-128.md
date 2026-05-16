# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.

- target security bits: `128`
- accepted candidates before truncation: `114764`
- candidates shown: `1000`
- rejected candidates: `4320436`

| q | n | m0 | m1 | k | eta | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 67108879 | 80 | 128 | 80 | 48 | 1 | 665.0 | 683.6 | 665.0 | 1750.3 | 32768 | 864 | 46 |
| 8388617 | 96 | 864 | 96 | 48 | 1 | 649.4 | 719.5 | 649.4 | 686.4 | 31776 | 2912 | 766 |
| 67108879 | 80 | 128 | 80 | 48 | 2 | 647.4 | 663.0 | 647.4 | 1597.0 | 32768 | 864 | 44 |
| 8388617 | 96 | 608 | 192 | 24 | 1 | 642.9 | 719.5 | 642.9 | 940.0 | 32160 | 2432 | 510 |
| 33554467 | 80 | 128 | 80 | 48 | 1 | 641.0 | 655.6 | 641.0 | 1670.3 | 32768 | 864 | 46 |
| 8388617 | 96 | 608 | 96 | 48 | 1 | 638.3 | 719.5 | 638.3 | 1092.2 | 31008 | 2144 | 510 |
| 8388617 | 96 | 480 | 192 | 24 | 1 | 636.7 | 719.5 | 636.7 | 1142.9 | 31776 | 2048 | 382 |
| 67108879 | 80 | 128 | 80 | 48 | 3 | 635.7 | 649.4 | 635.7 | 1496.1 | 32768 | 864 | 42 |
| 8388617 | 96 | 352 | 288 | 16 | 1 | 634.9 | 719.5 | 634.9 | 1193.6 | 32544 | 1952 | 254 |
| 8388617 | 96 | 480 | 96 | 48 | 1 | 631.1 | 719.5 | 631.1 | 1295.1 | 30624 | 1760 | 382 |
| 8388617 | 96 | 288 | 288 | 16 | 1 | 631.1 | 719.5 | 631.1 | 1295.1 | 32352 | 1760 | 190 |
| 8388617 | 96 | 352 | 192 | 24 | 1 | 629.0 | 719.5 | 629.0 | 1345.8 | 31392 | 1664 | 254 |
| 67108879 | 80 | 128 | 80 | 48 | 4 | 627.0 | 639.2 | 627.0 | 1420.7 | 32768 | 864 | 40 |
| 8388617 | 96 | 224 | 288 | 16 | 1 | 626.8 | 719.5 | 626.8 | 1396.5 | 32160 | 1568 | 126 |
| 8388617 | 96 | 288 | 192 | 24 | 1 | 624.5 | 719.5 | 624.5 | 1447.2 | 31200 | 1472 | 190 |
| 8388617 | 96 | 192 | 288 | 16 | 1 | 624.5 | 719.5 | 624.5 | 1447.2 | 32064 | 1472 | 94 |
| 33554467 | 80 | 128 | 80 | 48 | 2 | 623.4 | 635.0 | 623.4 | 1517.0 | 32768 | 864 | 44 |
| 8388617 | 96 | 352 | 96 | 48 | 1 | 622.0 | 719.5 | 622.0 | 1497.9 | 30240 | 1376 | 254 |
| 8388617 | 96 | 160 | 288 | 16 | 1 | 622.0 | 719.5 | 622.0 | 1497.9 | 31968 | 1376 | 62 |
| 8388617 | 96 | 144 | 288 | 16 | 1 | 620.6 | 719.5 | 620.6 | 1523.3 | 31920 | 1328 | 46 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
