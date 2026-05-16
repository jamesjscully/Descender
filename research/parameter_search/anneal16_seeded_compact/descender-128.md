# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `128`
- accepted candidates before truncation: `22135432`
- candidates shown: `5000`
- rejected candidates: `3464568`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32771 | 80 | 104 | 80 | 24 | 4 | 149.9 | 191.6 | 331.2 | 191.6 | 616.7 | 192 | 1456 | 400 | 16 |
| 16411 | 80 | 104 | 80 | 24 | 4 | 149.9 | 179.6 | 303.3 | 179.6 | 536.9 | 192 | 1456 | 400 | 16 |
| 8209 | 80 | 104 | 80 | 24 | 4 | 149.9 | 167.6 | 275.3 | 167.6 | 457.0 | 192 | 1456 | 400 | 16 |
| 32771 | 64 | 88 | 64 | 24 | 4 | 149.9 | 188.0 | 265.0 | 188.0 | 478.2 | 192 | 1232 | 336 | 16 |
| 16411 | 64 | 88 | 64 | 24 | 4 | 149.9 | 176.1 | 242.6 | 176.1 | 414.3 | 192 | 1232 | 336 | 16 |
| 8209 | 64 | 88 | 64 | 24 | 4 | 149.9 | 164.1 | 220.3 | 164.1 | 350.4 | 192 | 1232 | 336 | 16 |
| 32771 | 80 | 104 | 80 | 24 | 5 | 149.9 | 188.1 | 323.1 | 188.1 | 563.5 | 192 | 1456 | 400 | 14 |
| 16411 | 80 | 104 | 80 | 24 | 5 | 149.9 | 176.1 | 295.2 | 176.1 | 483.7 | 192 | 1456 | 400 | 14 |
| 8209 | 80 | 104 | 80 | 24 | 5 | 149.9 | 164.1 | 267.2 | 164.1 | 403.7 | 192 | 1456 | 400 | 14 |
| 32771 | 80 | 104 | 80 | 32 | 4 | 149.8 | 198.8 | 331.2 | 198.8 | 616.7 | 224 | 1488 | 400 | 16 |
| 16411 | 80 | 104 | 80 | 32 | 4 | 149.8 | 186.4 | 303.3 | 186.4 | 536.9 | 224 | 1488 | 400 | 16 |
| 8209 | 80 | 104 | 80 | 32 | 4 | 149.8 | 173.9 | 275.3 | 173.9 | 457.0 | 224 | 1488 | 400 | 16 |
| 4099 | 80 | 104 | 80 | 32 | 4 | 149.8 | 161.3 | 247.3 | 161.3 | 376.8 | 224 | 1488 | 400 | 16 |
| 32771 | 64 | 88 | 64 | 24 | 5 | 149.8 | 184.6 | 258.5 | 184.6 | 434.2 | 192 | 1232 | 336 | 14 |
| 16411 | 64 | 88 | 64 | 24 | 5 | 149.8 | 172.6 | 236.2 | 172.6 | 370.3 | 192 | 1232 | 336 | 14 |
| 8209 | 64 | 88 | 64 | 24 | 5 | 149.8 | 160.6 | 213.8 | 160.6 | 306.4 | 192 | 1232 | 336 | 14 |
| 32771 | 80 | 104 | 80 | 24 | 6 | 149.8 | 185.2 | 316.4 | 185.2 | 519.1 | 192 | 1456 | 400 | 12 |
| 16411 | 80 | 104 | 80 | 24 | 6 | 149.8 | 173.3 | 288.5 | 173.3 | 439.3 | 192 | 1456 | 400 | 12 |
| 8209 | 80 | 104 | 80 | 24 | 6 | 149.8 | 161.3 | 260.5 | 161.3 | 359.4 | 192 | 1456 | 400 | 12 |
| 32771 | 64 | 88 | 64 | 32 | 4 | 149.8 | 195.1 | 265.0 | 195.1 | 478.2 | 224 | 1264 | 336 | 16 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
