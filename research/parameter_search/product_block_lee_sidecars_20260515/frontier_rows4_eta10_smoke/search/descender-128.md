# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `128`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- q-ary amplitude strategies: `product-block-lee`
- Lee repetitions: `1639`
- Lee rows per product block: `4`
- m0 source: direct values `104, 112, 128`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `32.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `6`
- candidates shown: `6`
- rejected candidates: `3`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 1639 | 32771 | 80 | 112 | 32 | 144 | 64 | 10 | 16 | 0 | 10 | 1639 | 1639 | 17496.1 | 15857.1 | 64 | 128 | -423.8 | 178.8 | 297.0 | 178.8 | 567.5 | 125 | -881.8 | 96 | 448 | 352 | 112 | 80 | 288 | 1376 | 320 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1639 | 32771 | 80 | 112 | 32 | 144 | 48 | 10 | 16 | 0 | 10 | 1639 | 1639 | 17496.1 | 15857.1 | 64 | 128 | -424.3 | 139.6 | 297.0 | 139.6 | 567.5 | 125 | -881.8 | 96 | 448 | 288 | 112 | 80 | 288 | 1312 | 320 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1639 | 32771 | 80 | 112 | 32 | 144 | 96 | 10 | 16 | 0 | 10 | 1639 | 1639 | 17496.1 | 15857.1 | 64 | 128 | -424.7 | 229.1 | 297.0 | 229.1 | 567.5 | 125 | -881.8 | 96 | 448 | 480 | 112 | 80 | 288 | 1504 | 320 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1639 | 32771 | 80 | 128 | 32 | 160 | 64 | 10 | 16 | 0 | 10 | 1639 | 1639 | 10888.1 | 9249.1 | 64 | 128 | -464.7 | 180.8 | 297.0 | 180.8 | 497.2 | 125 | -727.5 | 96 | 480 | 352 | 112 | 80 | 320 | 1440 | 352 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1639 | 32771 | 80 | 128 | 32 | 160 | 48 | 10 | 16 | 0 | 10 | 1639 | 1639 | 10888.1 | 9249.1 | 64 | 128 | -465.2 | 141.1 | 297.0 | 141.1 | 497.2 | 125 | -727.5 | 96 | 480 | 288 | 112 | 80 | 320 | 1376 | 352 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 1639 | 32771 | 80 | 128 | 32 | 160 | 96 | 10 | 16 | 0 | 10 | 1639 | 1639 | 10888.1 | 9249.1 | 64 | 128 | -465.7 | 262.9 | 297.0 | 262.9 | 497.2 | 125 | -727.5 | 96 | 480 | 480 | 112 | 80 | 320 | 1568 | 352 | 28 | 28 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
