# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `256`
- accepted candidates before truncation: `21934244`
- candidates shown: `5000`
- rejected candidates: `3665756`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | incidence bytes | diagonal bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 208 | 256 | 208 | 48 | 4 | 281.4 | 393.2 | 861.2 | 393.2 | 1649.2 | 288 | 112 | 80 | 1920 | 960 | 40 |
| 16411 | 208 | 256 | 208 | 48 | 4 | 281.4 | 369.3 | 788.6 | 369.3 | 1441.6 | 288 | 112 | 80 | 1920 | 960 | 40 |
| 8209 | 208 | 256 | 208 | 48 | 4 | 281.4 | 345.3 | 715.8 | 345.3 | 1233.8 | 288 | 112 | 80 | 1920 | 960 | 40 |
| 4099 | 208 | 256 | 208 | 48 | 4 | 281.4 | 321.2 | 642.9 | 321.2 | 1025.4 | 288 | 112 | 80 | 1920 | 960 | 40 |
| 32771 | 224 | 272 | 224 | 48 | 4 | 281.4 | 395.7 | 927.5 | 395.7 | 1787.7 | 288 | 112 | 80 | 2016 | 1024 | 40 |
| 16411 | 224 | 272 | 224 | 48 | 4 | 281.4 | 371.7 | 849.3 | 371.7 | 1564.2 | 288 | 112 | 80 | 2016 | 1024 | 40 |
| 8209 | 224 | 272 | 224 | 48 | 4 | 281.4 | 347.7 | 770.9 | 347.7 | 1340.4 | 288 | 112 | 80 | 2016 | 1024 | 40 |
| 4099 | 224 | 272 | 224 | 48 | 4 | 281.4 | 323.6 | 692.4 | 323.6 | 1116.0 | 288 | 112 | 80 | 2016 | 1024 | 40 |
| 32771 | 192 | 240 | 192 | 48 | 4 | 281.4 | 390.6 | 795.0 | 390.6 | 1510.6 | 288 | 112 | 80 | 1824 | 896 | 40 |
| 16411 | 192 | 240 | 192 | 48 | 4 | 281.4 | 366.7 | 727.9 | 366.7 | 1319.0 | 288 | 112 | 80 | 1824 | 896 | 40 |
| 8209 | 192 | 240 | 192 | 48 | 4 | 281.4 | 342.7 | 660.8 | 342.7 | 1127.2 | 288 | 112 | 80 | 1824 | 896 | 40 |
| 4099 | 192 | 240 | 192 | 48 | 4 | 281.4 | 318.6 | 593.5 | 318.6 | 934.8 | 288 | 112 | 80 | 1824 | 896 | 40 |
| 32771 | 208 | 256 | 208 | 64 | 4 | 281.4 | 400.7 | 861.2 | 400.7 | 1649.2 | 352 | 112 | 80 | 1984 | 960 | 40 |
| 16411 | 208 | 256 | 208 | 64 | 4 | 281.4 | 376.2 | 788.6 | 376.2 | 1441.6 | 352 | 112 | 80 | 1984 | 960 | 40 |
| 8209 | 208 | 256 | 208 | 64 | 4 | 281.4 | 351.7 | 715.8 | 351.7 | 1233.8 | 352 | 112 | 80 | 1984 | 960 | 40 |
| 4099 | 208 | 256 | 208 | 64 | 4 | 281.4 | 327.2 | 642.9 | 327.2 | 1025.4 | 352 | 112 | 80 | 1984 | 960 | 40 |
| 32771 | 208 | 256 | 208 | 48 | 5 | 281.4 | 386.3 | 840.2 | 386.3 | 1514.9 | 288 | 112 | 80 | 1920 | 960 | 38 |
| 16411 | 208 | 256 | 208 | 48 | 5 | 281.4 | 362.3 | 767.5 | 362.3 | 1307.3 | 288 | 112 | 80 | 1920 | 960 | 38 |
| 8209 | 208 | 256 | 208 | 48 | 5 | 281.4 | 338.3 | 694.8 | 338.3 | 1099.4 | 288 | 112 | 80 | 1920 | 960 | 38 |
| 4099 | 208 | 256 | 208 | 48 | 5 | 281.4 | 314.2 | 621.8 | 314.2 | 891.0 | 288 | 112 | 80 | 1920 | 960 | 38 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
