# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.

- target security bits: `128`
- accepted candidates before truncation: `178154`
- candidates shown: `5000`
- rejected candidates: `23409046`

| q | n | m0 | m1 | k | eta | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 536870923 | 80 | 128 | 80 | 48 | 1 | 737.0 | 767.6 | 737.0 | 1990.3 | 32768 | 864 | 46 |
| 536870923 | 80 | 128 | 80 | 48 | 2 | 719.4 | 747.0 | 719.4 | 1837.0 | 32768 | 864 | 44 |
| 268435459 | 80 | 128 | 80 | 48 | 1 | 713.0 | 739.6 | 713.0 | 1910.3 | 32768 | 864 | 46 |
| 536870923 | 80 | 128 | 80 | 48 | 3 | 707.7 | 733.4 | 707.7 | 1736.1 | 32768 | 864 | 42 |
| 536870923 | 80 | 128 | 80 | 48 | 4 | 699.0 | 723.2 | 699.0 | 1660.7 | 32768 | 864 | 40 |
| 268435459 | 80 | 128 | 80 | 48 | 2 | 695.4 | 719.0 | 695.4 | 1757.0 | 32768 | 864 | 44 |
| 536870923 | 80 | 128 | 80 | 48 | 5 | 692.1 | 715.1 | 692.1 | 1600.4 | 32768 | 864 | 38 |
| 134217757 | 80 | 128 | 80 | 48 | 1 | 689.0 | 711.6 | 689.0 | 1830.3 | 32768 | 864 | 46 |
| 536870923 | 80 | 128 | 80 | 48 | 6 | 686.3 | 708.4 | 686.3 | 1550.3 | 32768 | 864 | 36 |
| 268435459 | 80 | 128 | 80 | 48 | 3 | 683.7 | 705.4 | 683.7 | 1656.1 | 32768 | 864 | 42 |
| 536870923 | 80 | 128 | 80 | 48 | 8 | 677.0 | 697.6 | 677.0 | 1469.8 | 32768 | 864 | 32 |
| 268435459 | 80 | 128 | 80 | 48 | 4 | 675.0 | 695.2 | 675.0 | 1580.7 | 32768 | 864 | 40 |
| 134217757 | 80 | 128 | 80 | 48 | 2 | 671.4 | 691.0 | 671.4 | 1677.0 | 32768 | 864 | 44 |
| 536870923 | 80 | 128 | 80 | 48 | 10 | 669.7 | 689.0 | 669.7 | 1406.4 | 32768 | 864 | 28 |
| 268435459 | 80 | 128 | 80 | 48 | 5 | 668.1 | 687.1 | 668.1 | 1520.4 | 32768 | 864 | 38 |
| 67108879 | 80 | 128 | 80 | 48 | 1 | 665.0 | 683.6 | 665.0 | 1750.3 | 32768 | 864 | 46 |
| 536870923 | 80 | 128 | 80 | 48 | 12 | 663.6 | 682.0 | 663.6 | 1354.1 | 32768 | 864 | 24 |
| 268435459 | 80 | 128 | 80 | 48 | 6 | 662.3 | 680.4 | 662.3 | 1470.3 | 32768 | 864 | 36 |
| 134217757 | 80 | 128 | 80 | 48 | 3 | 659.7 | 677.4 | 659.7 | 1576.1 | 32768 | 864 | 42 |
| 536870923 | 80 | 272 | 400 | 8 | 1 | 655.7 | 767.6 | 655.7 | 1254.9 | 32704 | 2720 | 190 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
