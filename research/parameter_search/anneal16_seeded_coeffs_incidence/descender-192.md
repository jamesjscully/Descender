# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `192`
- accepted candidates before truncation: `22399636`
- candidates shown: `5000`
- rejected candidates: `3200364`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | incidence bytes | diagonal bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 144 | 176 | 144 | 32 | 4 | 216.7 | 262.4 | 596.2 | 262.4 | 1145.6 | 224 | 112 | 80 | 1440 | 672 | 24 |
| 16411 | 144 | 176 | 144 | 32 | 4 | 216.7 | 246.5 | 546.0 | 246.5 | 1002.0 | 224 | 112 | 80 | 1440 | 672 | 24 |
| 8209 | 144 | 176 | 144 | 32 | 4 | 216.7 | 230.5 | 495.6 | 230.5 | 858.1 | 224 | 112 | 80 | 1440 | 672 | 24 |
| 32771 | 128 | 160 | 128 | 32 | 4 | 216.7 | 259.9 | 530.0 | 259.9 | 1007.1 | 224 | 112 | 80 | 1344 | 608 | 24 |
| 16411 | 128 | 160 | 128 | 32 | 4 | 216.7 | 243.9 | 485.3 | 243.9 | 879.4 | 224 | 112 | 80 | 1344 | 608 | 24 |
| 8209 | 128 | 160 | 128 | 32 | 4 | 216.7 | 227.9 | 440.5 | 227.9 | 751.4 | 224 | 112 | 80 | 1344 | 608 | 24 |
| 32771 | 160 | 192 | 160 | 32 | 4 | 216.7 | 264.8 | 662.5 | 264.8 | 1284.2 | 224 | 112 | 80 | 1536 | 736 | 24 |
| 16411 | 160 | 192 | 160 | 32 | 4 | 216.7 | 248.8 | 606.6 | 248.8 | 1124.6 | 224 | 112 | 80 | 1536 | 736 | 24 |
| 8209 | 160 | 192 | 160 | 32 | 4 | 216.7 | 232.8 | 550.7 | 232.8 | 964.7 | 224 | 112 | 80 | 1536 | 736 | 24 |
| 32771 | 144 | 176 | 144 | 32 | 5 | 216.7 | 257.8 | 581.7 | 257.8 | 1053.0 | 224 | 112 | 80 | 1440 | 672 | 22 |
| 16411 | 144 | 176 | 144 | 32 | 5 | 216.7 | 241.8 | 531.4 | 241.8 | 909.3 | 224 | 112 | 80 | 1440 | 672 | 22 |
| 8209 | 144 | 176 | 144 | 32 | 5 | 216.7 | 225.9 | 481.0 | 225.9 | 765.4 | 224 | 112 | 80 | 1440 | 672 | 22 |
| 32771 | 128 | 160 | 128 | 32 | 5 | 216.7 | 255.2 | 517.0 | 255.2 | 923.7 | 224 | 112 | 80 | 1344 | 608 | 22 |
| 16411 | 128 | 160 | 128 | 32 | 5 | 216.7 | 239.3 | 472.3 | 239.3 | 796.0 | 224 | 112 | 80 | 1344 | 608 | 22 |
| 32771 | 144 | 176 | 144 | 48 | 4 | 216.6 | 269.9 | 596.2 | 269.9 | 1145.6 | 288 | 112 | 80 | 1504 | 672 | 24 |
| 16411 | 144 | 176 | 144 | 48 | 4 | 216.6 | 253.5 | 546.0 | 253.5 | 1002.0 | 288 | 112 | 80 | 1504 | 672 | 24 |
| 8209 | 144 | 176 | 144 | 48 | 4 | 216.6 | 237.0 | 495.6 | 237.0 | 858.1 | 288 | 112 | 80 | 1504 | 672 | 24 |
| 32771 | 160 | 192 | 160 | 32 | 5 | 216.6 | 260.1 | 646.3 | 260.1 | 1182.3 | 224 | 112 | 80 | 1536 | 736 | 22 |
| 16411 | 160 | 192 | 160 | 32 | 5 | 216.6 | 244.1 | 590.4 | 244.1 | 1022.7 | 224 | 112 | 80 | 1536 | 736 | 22 |
| 8209 | 160 | 192 | 160 | 32 | 5 | 216.6 | 228.2 | 534.4 | 228.2 | 862.8 | 224 | 112 | 80 | 1536 | 736 | 22 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
