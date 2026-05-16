# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `256`
- accepted candidates before truncation: `21852779`
- candidates shown: `5000`
- rejected candidates: `3747221`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 176 | 224 | 176 | 48 | 4 | 280.6 | 387.8 | 728.7 | 387.8 | 1372.1 | 288 | 2944 | 832 | 40 |
| 16411 | 176 | 224 | 176 | 48 | 4 | 280.6 | 363.8 | 667.3 | 363.8 | 1196.4 | 288 | 2944 | 832 | 40 |
| 8209 | 176 | 224 | 176 | 48 | 4 | 280.6 | 339.8 | 605.7 | 339.8 | 1020.6 | 288 | 2944 | 832 | 40 |
| 4099 | 176 | 224 | 176 | 48 | 4 | 280.6 | 315.8 | 544.0 | 315.8 | 844.2 | 288 | 2944 | 832 | 40 |
| 32771 | 192 | 240 | 192 | 48 | 4 | 280.6 | 390.6 | 795.0 | 390.6 | 1510.6 | 288 | 3168 | 896 | 40 |
| 16411 | 192 | 240 | 192 | 48 | 4 | 280.6 | 366.7 | 727.9 | 366.7 | 1319.0 | 288 | 3168 | 896 | 40 |
| 8209 | 192 | 240 | 192 | 48 | 4 | 280.6 | 342.7 | 660.8 | 342.7 | 1127.2 | 288 | 3168 | 896 | 40 |
| 4099 | 192 | 240 | 192 | 48 | 4 | 280.6 | 318.6 | 593.5 | 318.6 | 934.8 | 288 | 3168 | 896 | 40 |
| 32771 | 176 | 224 | 176 | 64 | 4 | 280.6 | 395.1 | 728.7 | 395.1 | 1372.1 | 352 | 3008 | 832 | 40 |
| 16411 | 176 | 224 | 176 | 64 | 4 | 280.6 | 370.7 | 667.3 | 370.7 | 1196.4 | 352 | 3008 | 832 | 40 |
| 8209 | 176 | 224 | 176 | 64 | 4 | 280.6 | 346.2 | 605.7 | 346.2 | 1020.6 | 352 | 3008 | 832 | 40 |
| 4099 | 176 | 224 | 176 | 64 | 4 | 280.6 | 321.6 | 544.0 | 321.6 | 844.2 | 352 | 3008 | 832 | 40 |
| 32771 | 160 | 208 | 160 | 48 | 4 | 280.5 | 384.7 | 662.5 | 384.7 | 1233.5 | 288 | 2720 | 768 | 40 |
| 16411 | 160 | 208 | 160 | 48 | 4 | 280.5 | 360.7 | 606.6 | 360.7 | 1073.8 | 288 | 2720 | 768 | 40 |
| 8209 | 160 | 208 | 160 | 48 | 4 | 280.5 | 336.8 | 550.7 | 336.8 | 913.9 | 288 | 2720 | 768 | 40 |
| 4099 | 160 | 208 | 160 | 48 | 4 | 280.5 | 312.7 | 494.5 | 312.7 | 753.6 | 288 | 2720 | 768 | 40 |
| 32771 | 176 | 224 | 176 | 48 | 5 | 280.5 | 380.8 | 710.9 | 380.8 | 1256.3 | 288 | 2944 | 832 | 38 |
| 16411 | 176 | 224 | 176 | 48 | 5 | 280.5 | 356.9 | 649.4 | 356.9 | 1080.6 | 288 | 2944 | 832 | 38 |
| 8209 | 176 | 224 | 176 | 48 | 5 | 280.5 | 332.9 | 587.9 | 332.9 | 904.8 | 288 | 2944 | 832 | 38 |
| 4099 | 176 | 224 | 176 | 48 | 5 | 280.5 | 308.8 | 526.2 | 308.8 | 728.4 | 288 | 2944 | 832 | 38 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
