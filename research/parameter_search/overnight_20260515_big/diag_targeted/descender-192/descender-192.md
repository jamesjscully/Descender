# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
- objective mode: `bandwidth`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only`
- m0 source: direct values `88, 96, 112, 128, 160`
- view coordinate counts: `88, 96, 112, 128, 160`
- security extra cap: `24.0` bits
- guard slack target: `12`
- combined certification weight: `0.000`
- accepted candidates before truncation: `125229134`
- candidates shown: `1200`
- rejected candidates: `28370866`

| proof | primal model | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 32 | 1 | 12 | 9 | 10 | -186.6 | 195.0 | 195.0 | 237.4 | 381.5 | 73 | -331.7 | 64 | 0 | 0 | 0 | 0 | 132 | 196 | 164 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 48 | 1 | 12 | 9 | 10 | -186.6 | 195.0 | 195.0 | 296.1 | 381.5 | 73 | -331.7 | 64 | 0 | 0 | 0 | 0 | 132 | 196 | 164 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 64 | 1 | 12 | 9 | 10 | -186.6 | 195.0 | 195.0 | 296.1 | 381.5 | 73 | -331.7 | 64 | 0 | 0 | 0 | 0 | 132 | 196 | 164 | 20 | 20 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 32 | 2 | 12 | 9 | 11 | -187.3 | 192.8 | 192.8 | 235.3 | 369.9 | 74 | -337.2 | 64 | 0 | 0 | 0 | 0 | 132 | 196 | 164 | 18 | 18 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 48 | 2 | 12 | 9 | 11 | -187.3 | 192.8 | 192.8 | 293.4 | 369.9 | 74 | -337.2 | 64 | 0 | 0 | 0 | 0 | 132 | 196 | 164 | 18 | 18 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 64 | 2 | 12 | 9 | 11 | -187.3 | 192.8 | 192.8 | 293.4 | 369.9 | 74 | -337.2 | 64 | 0 | 0 | 0 | 0 | 132 | 196 | 164 | 18 | 18 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 96 | 48 | 96 | 32 | 1 | 12 | 9 | 10 | -188.6 | 195.0 | 195.0 | 238.9 | 346.3 | 73 | -246.5 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 96 | 48 | 96 | 48 | 1 | 12 | 9 | 10 | -188.6 | 195.0 | 195.0 | 342.8 | 346.3 | 73 | -246.5 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 96 | 48 | 96 | 64 | 1 | 12 | 9 | 10 | -188.6 | 195.0 | 195.0 | 349.1 | 346.3 | 73 | -246.5 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 96 | 48 | 96 | 32 | 2 | 12 | 9 | 11 | -189.4 | 192.8 | 192.8 | 236.8 | 333.7 | 74 | -253.1 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 26 | 26 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 96 | 48 | 96 | 48 | 2 | 12 | 9 | 11 | -189.4 | 192.8 | 192.8 | 339.6 | 333.7 | 74 | -253.1 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 26 | 26 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 96 | 48 | 96 | 64 | 2 | 12 | 9 | 11 | -189.4 | 192.8 | 192.8 | 345.9 | 333.7 | 74 | -253.1 | 64 | 0 | 0 | 0 | 0 | 144 | 208 | 176 | 26 | 26 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 32 | 1 | 13 | 5 | 6 | -192.0 | 206.6 | 206.6 | 248.5 | 442.4 | 68 | -302.4 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 48 | 1 | 13 | 5 | 6 | -192.0 | 206.6 | 206.6 | 310.3 | 442.4 | 68 | -302.4 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 64 | 1 | 13 | 5 | 6 | -192.0 | 206.6 | 206.6 | 310.3 | 442.4 | 68 | -302.4 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 28 | 28 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 112 | 48 | 112 | 32 | 1 | 12 | 9 | 10 | -192.6 | 195.0 | 195.0 | 241.6 | 276.1 | 73 | -76.2 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 44 | 44 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 112 | 48 | 112 | 48 | 1 | 12 | 9 | 10 | -192.6 | 195.0 | 195.0 | 347.1 | 276.1 | 73 | -76.2 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 44 | 44 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 112 | 48 | 112 | 64 | 1 | 12 | 9 | 10 | -192.6 | 195.0 | 195.0 | 447.1 | 276.1 | 73 | -76.2 | 64 | 0 | 0 | 0 | 0 | 168 | 232 | 200 | 44 | 44 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 32 | 2 | 13 | 5 | 7 | -193.1 | 203.2 | 203.2 | 245.2 | 424.2 | 69 | -311.2 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 26 | 26 | 0.0 | 0 |
| lossy-budget | guard-only | 65537 | 48 | 88 | 48 | 88 | 48 | 2 | 13 | 5 | 7 | -193.1 | 203.2 | 203.2 | 306.1 | 424.2 | 69 | -311.2 | 64 | 0 | 0 | 0 | 0 | 143 | 207 | 175 | 26 | 26 | 0.0 | 0 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
