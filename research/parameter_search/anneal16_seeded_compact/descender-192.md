# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `192`
- accepted candidates before truncation: `22393347`
- candidates shown: `5000`
- rejected candidates: `3206653`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 112 | 144 | 112 | 32 | 4 | 215.7 | 257.0 | 463.7 | 257.0 | 868.5 | 224 | 1952 | 544 | 24 |
| 16411 | 112 | 144 | 112 | 32 | 4 | 215.7 | 241.0 | 424.6 | 241.0 | 756.8 | 224 | 1952 | 544 | 24 |
| 8209 | 112 | 144 | 112 | 32 | 4 | 215.7 | 225.0 | 385.5 | 225.0 | 644.8 | 224 | 1952 | 544 | 24 |
| 32771 | 128 | 160 | 128 | 32 | 4 | 215.7 | 259.9 | 530.0 | 259.9 | 1007.1 | 224 | 2176 | 608 | 24 |
| 16411 | 128 | 160 | 128 | 32 | 4 | 215.7 | 243.9 | 485.3 | 243.9 | 879.4 | 224 | 2176 | 608 | 24 |
| 8209 | 128 | 160 | 128 | 32 | 4 | 215.7 | 227.9 | 440.5 | 227.9 | 751.4 | 224 | 2176 | 608 | 24 |
| 32771 | 112 | 144 | 112 | 32 | 5 | 215.7 | 252.4 | 452.4 | 252.4 | 794.4 | 224 | 1952 | 544 | 22 |
| 16411 | 112 | 144 | 112 | 32 | 5 | 215.7 | 236.4 | 413.3 | 236.4 | 682.7 | 224 | 1952 | 544 | 22 |
| 32771 | 112 | 144 | 112 | 48 | 4 | 215.6 | 264.3 | 463.7 | 264.3 | 868.5 | 288 | 2016 | 544 | 24 |
| 16411 | 112 | 144 | 112 | 48 | 4 | 215.6 | 247.8 | 424.6 | 247.8 | 756.8 | 288 | 2016 | 544 | 24 |
| 8209 | 112 | 144 | 112 | 48 | 4 | 215.6 | 231.3 | 385.5 | 231.3 | 644.8 | 288 | 2016 | 544 | 24 |
| 32771 | 128 | 160 | 128 | 32 | 5 | 215.6 | 255.2 | 517.0 | 255.2 | 923.7 | 224 | 2176 | 608 | 22 |
| 16411 | 128 | 160 | 128 | 32 | 5 | 215.6 | 239.3 | 472.3 | 239.3 | 796.0 | 224 | 2176 | 608 | 22 |
| 32771 | 112 | 144 | 112 | 32 | 6 | 215.6 | 248.5 | 442.9 | 248.5 | 732.7 | 224 | 1952 | 544 | 20 |
| 16411 | 112 | 144 | 112 | 32 | 6 | 215.6 | 232.5 | 403.8 | 232.5 | 621.0 | 224 | 1952 | 544 | 20 |
| 32771 | 96 | 128 | 96 | 32 | 4 | 215.6 | 253.7 | 397.5 | 253.7 | 729.9 | 224 | 1728 | 480 | 24 |
| 16411 | 96 | 128 | 96 | 32 | 4 | 215.6 | 237.7 | 364.0 | 237.7 | 634.2 | 224 | 1728 | 480 | 24 |
| 32771 | 128 | 160 | 128 | 48 | 4 | 215.6 | 267.3 | 530.0 | 267.3 | 1007.1 | 288 | 2240 | 608 | 24 |
| 16411 | 128 | 160 | 128 | 48 | 4 | 215.6 | 250.8 | 485.3 | 250.8 | 879.4 | 288 | 2240 | 608 | 24 |
| 8209 | 128 | 160 | 128 | 48 | 4 | 215.6 | 234.3 | 440.5 | 234.3 | 751.4 | 288 | 2240 | 608 | 24 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
