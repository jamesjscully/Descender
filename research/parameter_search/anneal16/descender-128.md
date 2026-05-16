# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `128`
- accepted candidates before truncation: `21814529`
- candidates shown: `5000`
- rejected candidates: `3785471`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 96 | 144 | 96 | 48 | 4 | 194.3 | 368.6 | 397.5 | 368.6 | 679.2 | 19968 | 512 | 40 |
| 32771 | 96 | 160 | 96 | 48 | 4 | 193.3 | 371.1 | 397.5 | 371.1 | 628.5 | 20000 | 544 | 56 |
| 32771 | 96 | 144 | 192 | 24 | 4 | 192.7 | 374.9 | 397.5 | 381.3 | 374.9 | 20928 | 704 | 40 |
| 32771 | 96 | 144 | 96 | 48 | 5 | 192.5 | 361.6 | 387.8 | 361.6 | 609.7 | 19968 | 512 | 38 |
| 32771 | 96 | 192 | 96 | 48 | 4 | 191.6 | 375.5 | 397.5 | 375.5 | 527.1 | 20064 | 608 | 88 |
| 32771 | 80 | 128 | 80 | 48 | 4 | 191.6 | 331.2 | 331.2 | 363.0 | 540.7 | 16672 | 448 | 40 |
| 32771 | 96 | 160 | 96 | 48 | 5 | 191.4 | 364.1 | 387.8 | 364.1 | 554.4 | 20000 | 544 | 54 |
| 32771 | 96 | 144 | 96 | 48 | 6 | 190.9 | 355.8 | 379.7 | 355.8 | 551.9 | 19968 | 512 | 36 |
| 32771 | 96 | 144 | 96 | 48 | 3 | 190.6 | 377.3 | 409.7 | 377.3 | 766.2 | 19968 | 512 | 42 |
| 32771 | 96 | 144 | 192 | 24 | 3 | 190.5 | 390.0 | 409.7 | 390.0 | 496.7 | 20928 | 704 | 42 |
| 32771 | 96 | 224 | 96 | 48 | 4 | 190.3 | 379.5 | 397.5 | 379.5 | 425.6 | 20128 | 672 | 120 |
| 32771 | 112 | 176 | 112 | 64 | 4 | 190.3 | 463.7 | 463.7 | 486.5 | 767.1 | 30464 | 608 | 56 |
| 32771 | 96 | 160 | 96 | 48 | 6 | 189.9 | 358.3 | 379.7 | 358.3 | 492.7 | 20000 | 544 | 52 |
| 32771 | 96 | 192 | 96 | 48 | 5 | 189.8 | 368.6 | 387.8 | 368.6 | 443.7 | 20064 | 608 | 86 |
| 32771 | 80 | 144 | 80 | 48 | 4 | 189.7 | 331.2 | 331.2 | 365.9 | 489.9 | 16704 | 480 | 56 |
| 32771 | 96 | 160 | 96 | 48 | 3 | 189.6 | 379.8 | 409.7 | 379.8 | 721.3 | 20000 | 544 | 58 |
| 32771 | 96 | 160 | 192 | 24 | 3 | 189.3 | 391.8 | 409.7 | 391.8 | 451.8 | 20960 | 736 | 58 |
| 32771 | 80 | 128 | 80 | 48 | 5 | 189.2 | 323.1 | 323.1 | 356.1 | 480.4 | 16672 | 448 | 38 |
| 32771 | 80 | 128 | 80 | 48 | 3 | 188.5 | 341.4 | 341.4 | 371.7 | 616.1 | 16672 | 448 | 42 |
| 32771 | 96 | 144 | 96 | 48 | 8 | 188.3 | 346.5 | 366.7 | 346.5 | 459.0 | 19968 | 512 | 32 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
