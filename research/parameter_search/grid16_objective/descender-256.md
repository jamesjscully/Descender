# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `256`
- accepted candidates before truncation: `488218`
- candidates shown: `5000`
- rejected candidates: `23098982`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 240 | 368 | 480 | 64 | 3 | 383.9 | 1024.2 | 1024.2 | 1037.5 | 1219.4 | 128992 | 1728 | 122 |
| 32771 | 240 | 368 | 480 | 64 | 2 | 383.8 | 1065.0 | 1065.0 | 1068.6 | 1631.0 | 128992 | 1728 | 124 |
| 32771 | 240 | 432 | 480 | 64 | 3 | 382.3 | 1024.2 | 1024.2 | 1044.8 | 1039.7 | 129120 | 1856 | 186 |
| 32771 | 240 | 432 | 480 | 64 | 2 | 382.2 | 1065.0 | 1065.0 | 1075.9 | 1482.4 | 129120 | 1856 | 188 |
| 32771 | 240 | 496 | 480 | 64 | 2 | 381.0 | 1065.0 | 1065.0 | 1082.6 | 1333.8 | 129248 | 1984 | 252 |
| 8388617 | 160 | 256 | 160 | 96 | 4 | 380.0 | 1110.5 | 1110.5 | 1111.7 | 2361.3 | 95424 | 1280 | 88 |
| 32771 | 176 | 272 | 176 | 96 | 4 | 379.0 | 728.7 | 728.7 | 733.5 | 1219.9 | 70336 | 928 | 88 |
| 32771 | 224 | 352 | 448 | 64 | 3 | 378.9 | 955.9 | 955.9 | 1031.7 | 1114.1 | 120416 | 1632 | 122 |
| 32771 | 240 | 368 | 480 | 64 | 1 | 378.8 | 1115.8 | 1126.9 | 1115.8 | 2256.0 | 128992 | 1728 | 126 |
| 32771 | 224 | 352 | 448 | 64 | 2 | 378.6 | 994.0 | 994.0 | 1062.8 | 1502.5 | 120416 | 1632 | 124 |
| 8388617 | 160 | 288 | 160 | 96 | 4 | 378.5 | 1110.5 | 1110.5 | 1117.5 | 2259.9 | 95520 | 1376 | 120 |
| 32771 | 240 | 432 | 480 | 64 | 1 | 378.3 | 1123.0 | 1126.9 | 1123.0 | 2154.5 | 129120 | 1856 | 190 |
| 4194319 | 160 | 256 | 160 | 96 | 4 | 378.2 | 1054.5 | 1054.5 | 1063.7 | 2201.3 | 95424 | 1280 | 88 |
| 8388617 | 160 | 256 | 160 | 96 | 5 | 377.7 | 1094.3 | 1094.3 | 1097.8 | 2240.9 | 95424 | 1280 | 86 |
| 32771 | 176 | 304 | 176 | 96 | 4 | 377.7 | 728.7 | 728.7 | 738.9 | 1118.5 | 70400 | 992 | 120 |
| 8388617 | 160 | 256 | 320 | 48 | 4 | 377.7 | 1110.5 | 1110.5 | 1136.8 | 1854.1 | 97344 | 1760 | 88 |
| 32771 | 240 | 496 | 480 | 64 | 1 | 377.6 | 1126.9 | 1126.9 | 1129.8 | 2053.1 | 129248 | 1984 | 254 |
| 32771 | 192 | 288 | 192 | 96 | 4 | 377.3 | 738.9 | 795.0 | 738.9 | 1358.5 | 76704 | 992 | 88 |
| 32771 | 192 | 288 | 384 | 48 | 4 | 377.2 | 749.8 | 795.0 | 764.4 | 749.8 | 78624 | 1376 | 88 |
| 32771 | 176 | 272 | 176 | 96 | 3 | 377.1 | 750.9 | 751.1 | 750.9 | 1382.3 | 70336 | 928 | 90 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
