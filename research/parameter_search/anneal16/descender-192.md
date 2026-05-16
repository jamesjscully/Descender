# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `192`
- accepted candidates before truncation: `21748007`
- candidates shown: `5000`
- rejected candidates: `3851993`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 128 | 192 | 128 | 64 | 4 | 279.9 | 492.0 | 530.0 | 492.0 | 905.6 | 34784 | 672 | 56 |
| 32771 | 128 | 192 | 256 | 32 | 4 | 279.3 | 499.9 | 530.0 | 509.0 | 499.9 | 36064 | 928 | 56 |
| 32771 | 128 | 224 | 128 | 64 | 4 | 278.8 | 496.9 | 530.0 | 496.9 | 804.2 | 34848 | 736 | 88 |
| 32771 | 160 | 256 | 160 | 96 | 4 | 278.2 | 662.5 | 662.5 | 727.7 | 1081.3 | 63968 | 864 | 88 |
| 32771 | 128 | 256 | 128 | 64 | 4 | 278.1 | 501.3 | 530.0 | 501.3 | 702.8 | 34912 | 800 | 120 |
| 32771 | 128 | 192 | 256 | 32 | 3 | 278.0 | 520.6 | 546.2 | 520.6 | 662.3 | 36064 | 928 | 58 |
| 32771 | 112 | 176 | 112 | 64 | 4 | 277.9 | 463.7 | 463.7 | 486.5 | 767.1 | 30464 | 608 | 56 |
| 32771 | 128 | 192 | 128 | 64 | 5 | 277.7 | 482.7 | 517.0 | 482.7 | 813.0 | 34784 | 672 | 54 |
| 32771 | 144 | 208 | 288 | 32 | 4 | 277.3 | 514.0 | 596.2 | 514.0 | 587.7 | 40544 | 1024 | 56 |
| 32771 | 160 | 288 | 160 | 96 | 4 | 276.7 | 662.5 | 662.5 | 733.5 | 979.9 | 64032 | 928 | 120 |
| 32771 | 128 | 224 | 128 | 64 | 5 | 276.7 | 487.6 | 517.0 | 487.6 | 702.3 | 34848 | 736 | 86 |
| 32771 | 128 | 224 | 256 | 32 | 3 | 276.6 | 524.0 | 546.2 | 524.0 | 572.5 | 36128 | 992 | 90 |
| 32771 | 128 | 192 | 128 | 64 | 3 | 276.6 | 503.6 | 546.2 | 503.6 | 1021.7 | 34784 | 672 | 58 |
| 32771 | 144 | 208 | 144 | 64 | 4 | 276.1 | 496.9 | 596.2 | 496.9 | 1044.2 | 39104 | 736 | 56 |
| 32771 | 160 | 256 | 160 | 96 | 3 | 275.9 | 682.8 | 682.8 | 745.1 | 1232.2 | 63968 | 864 | 90 |
| 32771 | 128 | 256 | 128 | 64 | 5 | 275.9 | 492.0 | 517.0 | 492.0 | 591.6 | 34912 | 800 | 118 |
| 32771 | 128 | 192 | 128 | 64 | 6 | 275.8 | 475.0 | 506.2 | 475.0 | 735.9 | 34784 | 672 | 52 |
| 32771 | 112 | 208 | 112 | 64 | 4 | 275.5 | 463.7 | 463.7 | 492.0 | 665.6 | 30528 | 672 | 88 |
| 32771 | 128 | 224 | 128 | 64 | 3 | 275.5 | 508.5 | 546.2 | 508.5 | 931.8 | 34848 | 736 | 90 |
| 32771 | 128 | 192 | 256 | 32 | 2 | 275.4 | 536.1 | 568.0 | 536.1 | 879.8 | 36064 | 928 | 60 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
