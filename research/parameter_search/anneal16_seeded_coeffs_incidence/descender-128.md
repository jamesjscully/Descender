# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `128`
- accepted candidates before truncation: `22154061`
- candidates shown: `5000`
- rejected candidates: `3445939`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | incidence bytes | diagonal bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 96 | 120 | 96 | 24 | 4 | 151.1 | 194.5 | 397.5 | 194.5 | 755.3 | 192 | 112 | 80 | 1104 | 464 | 16 |
| 16411 | 96 | 120 | 96 | 24 | 4 | 151.1 | 182.6 | 364.0 | 182.6 | 659.5 | 192 | 112 | 80 | 1104 | 464 | 16 |
| 8209 | 96 | 120 | 96 | 24 | 4 | 151.1 | 170.6 | 330.4 | 170.6 | 563.6 | 192 | 112 | 80 | 1104 | 464 | 16 |
| 32771 | 96 | 120 | 96 | 24 | 5 | 151.0 | 191.1 | 387.8 | 191.1 | 692.8 | 192 | 112 | 80 | 1104 | 464 | 14 |
| 16411 | 96 | 120 | 96 | 24 | 5 | 151.0 | 179.1 | 354.2 | 179.1 | 597.0 | 192 | 112 | 80 | 1104 | 464 | 14 |
| 8209 | 96 | 120 | 96 | 24 | 5 | 151.0 | 167.1 | 320.7 | 167.1 | 501.0 | 192 | 112 | 80 | 1104 | 464 | 14 |
| 32771 | 80 | 104 | 80 | 24 | 4 | 151.0 | 191.6 | 331.2 | 191.6 | 616.7 | 192 | 112 | 80 | 1008 | 400 | 16 |
| 16411 | 80 | 104 | 80 | 24 | 4 | 151.0 | 179.6 | 303.3 | 179.6 | 536.9 | 192 | 112 | 80 | 1008 | 400 | 16 |
| 8209 | 80 | 104 | 80 | 24 | 4 | 151.0 | 167.6 | 275.3 | 167.6 | 457.0 | 192 | 112 | 80 | 1008 | 400 | 16 |
| 32771 | 96 | 120 | 96 | 32 | 4 | 151.0 | 201.9 | 397.5 | 201.9 | 755.3 | 224 | 112 | 80 | 1136 | 464 | 16 |
| 16411 | 96 | 120 | 96 | 32 | 4 | 151.0 | 189.5 | 364.0 | 189.5 | 659.5 | 224 | 112 | 80 | 1136 | 464 | 16 |
| 8209 | 96 | 120 | 96 | 32 | 4 | 151.0 | 177.0 | 330.4 | 177.0 | 563.6 | 224 | 112 | 80 | 1136 | 464 | 16 |
| 4099 | 96 | 120 | 96 | 32 | 4 | 151.0 | 164.4 | 296.7 | 164.4 | 467.4 | 224 | 112 | 80 | 1136 | 464 | 16 |
| 32771 | 96 | 120 | 96 | 24 | 6 | 151.0 | 188.2 | 379.7 | 188.2 | 640.7 | 192 | 112 | 80 | 1104 | 464 | 12 |
| 16411 | 96 | 120 | 96 | 24 | 6 | 151.0 | 176.2 | 346.1 | 176.2 | 544.9 | 192 | 112 | 80 | 1104 | 464 | 12 |
| 8209 | 96 | 120 | 96 | 24 | 6 | 151.0 | 164.2 | 312.6 | 164.2 | 449.0 | 192 | 112 | 80 | 1104 | 464 | 12 |
| 32771 | 112 | 136 | 112 | 24 | 4 | 151.0 | 197.1 | 463.7 | 197.1 | 893.9 | 192 | 112 | 80 | 1200 | 528 | 16 |
| 16411 | 112 | 136 | 112 | 24 | 4 | 151.0 | 185.1 | 424.6 | 185.1 | 782.1 | 192 | 112 | 80 | 1200 | 528 | 16 |
| 8209 | 112 | 136 | 112 | 24 | 4 | 151.0 | 173.1 | 385.5 | 173.1 | 670.2 | 192 | 112 | 80 | 1200 | 528 | 16 |
| 4099 | 112 | 136 | 112 | 24 | 4 | 151.0 | 161.1 | 346.2 | 161.1 | 558.0 | 192 | 112 | 80 | 1200 | 528 | 16 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
