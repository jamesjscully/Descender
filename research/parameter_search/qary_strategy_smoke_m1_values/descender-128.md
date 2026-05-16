# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The screening objective caps excess security credit and penalizes public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `128`
- objective mode: `screening`
- proof strategies: `lossy-budget`
- primal distance models: `guard-only`
- q-ary amplitude strategies: `fallback-support, solved-repetition`
- Lee repetitions: `1, 2, 4, 8, 16`
- m0 source: `n + overhead`, overheads `64`
- m1 source: direct values `4, 8, 16, 32, 64`
- view coordinate counts: `0`
- security extra cap: `32.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `0`
- candidates shown: `0`
- rejected candidates: `100`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
