# Parameter Search Report: descender-256

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `256`
- objective mode: `bandwidth`
- proof strategies: `full-surjective`
- primal distance models: `guard-only`
- q-ary amplitude strategies: `product-block-lee`
- Lee repetitions: `3277`
- Lee rows per product block: `5`
- m0 source: direct values `80, 88, 96, 104, 112, 128, 160, 192, 224, 256, 320, 384, 448, 512, 544`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `32.0` bits
- guard slack target: `16`
- combined certification weight: `0.250`
- accepted candidates before truncation: `680`
- candidates shown: `200`
- rejected candidates: `14440`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 10888.1 | 7611.1 | 80 | 160 | -241.0 | 323.1 | 323.1 | 351.9 | 646.5 | 114 | -680.2 | 96 | 480 | 608 | 112 | 80 | 320 | 1696 | 352 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 128 | 6 | 16 | 0 | 6 | 2731 | 3277 | 10888.1 | 8157.1 | 80 | 160 | -241.1 | 316.4 | 316.4 | 346.0 | 607.9 | 116 | -692.6 | 96 | 480 | 608 | 112 | 80 | 320 | 1696 | 352 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 128 | 8 | 16 | 0 | 8 | 2049 | 3277 | 10888.1 | 8839.1 | 80 | 160 | -241.1 | 305.6 | 305.6 | 336.5 | 546.0 | 121 | -712.2 | 96 | 480 | 608 | 112 | 80 | 320 | 1696 | 352 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 128 | 10 | 16 | 0 | 10 | 1639 | 3277 | 10888.1 | 9249.1 | 80 | 160 | -241.1 | 297.0 | 297.0 | 329.0 | 497.2 | 125 | -727.5 | 96 | 480 | 608 | 112 | 80 | 320 | 1696 | 352 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 10888.1 | 7611.1 | 80 | 160 | -241.2 | 280.7 | 323.1 | 280.7 | 646.5 | 114 | -680.2 | 96 | 480 | 480 | 112 | 80 | 320 | 1568 | 352 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 96 | 6 | 16 | 0 | 6 | 2731 | 3277 | 10888.1 | 8157.1 | 80 | 160 | -241.4 | 276.1 | 316.4 | 276.1 | 607.9 | 116 | -692.6 | 96 | 480 | 480 | 112 | 80 | 320 | 1568 | 352 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 10888.1 | 7611.1 | 80 | 160 | -241.5 | 323.1 | 323.1 | 351.9 | 646.5 | 114 | -680.2 | 96 | 480 | 864 | 112 | 80 | 320 | 1952 | 352 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 192 | 6 | 16 | 0 | 6 | 2731 | 3277 | 10888.1 | 8157.1 | 80 | 160 | -241.5 | 316.4 | 316.4 | 346.0 | 607.9 | 116 | -692.6 | 96 | 480 | 864 | 112 | 80 | 320 | 1952 | 352 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 192 | 8 | 16 | 0 | 8 | 2049 | 3277 | 10888.1 | 8839.1 | 80 | 160 | -241.6 | 305.6 | 305.6 | 336.5 | 546.0 | 121 | -712.2 | 96 | 480 | 864 | 112 | 80 | 320 | 1952 | 352 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 192 | 10 | 16 | 0 | 10 | 1639 | 3277 | 10888.1 | 9249.1 | 80 | 160 | -241.6 | 297.0 | 297.0 | 329.0 | 497.2 | 125 | -727.5 | 96 | 480 | 864 | 112 | 80 | 320 | 1952 | 352 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 96 | 8 | 16 | 0 | 8 | 2049 | 3277 | 10888.1 | 8839.1 | 80 | 160 | -241.8 | 268.7 | 305.6 | 268.7 | 546.0 | 121 | -712.2 | 96 | 480 | 480 | 112 | 80 | 320 | 1568 | 352 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 256 | 5 | 16 | 0 | 5 | 3277 | 3277 | 10888.1 | 7611.1 | 80 | 160 | -242.0 | 323.1 | 323.1 | 351.9 | 646.5 | 114 | -680.2 | 96 | 480 | 1120 | 112 | 80 | 320 | 2208 | 352 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 256 | 6 | 16 | 0 | 6 | 2731 | 3277 | 10888.1 | 8157.1 | 80 | 160 | -242.0 | 316.4 | 316.4 | 346.0 | 607.9 | 116 | -692.6 | 96 | 480 | 1120 | 112 | 80 | 320 | 2208 | 352 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 256 | 8 | 16 | 0 | 8 | 2049 | 3277 | 10888.1 | 8839.1 | 80 | 160 | -242.1 | 305.6 | 305.6 | 336.5 | 546.0 | 121 | -712.2 | 96 | 480 | 1120 | 112 | 80 | 320 | 2208 | 352 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 256 | 10 | 16 | 0 | 10 | 1639 | 3277 | 10888.1 | 9249.1 | 80 | 160 | -242.1 | 297.0 | 297.0 | 329.0 | 497.2 | 125 | -727.5 | 96 | 480 | 1120 | 112 | 80 | 320 | 2208 | 352 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 96 | 10 | 16 | 0 | 10 | 1639 | 3277 | 10888.1 | 9249.1 | 80 | 160 | -242.1 | 262.9 | 297.0 | 262.9 | 497.2 | 125 | -727.5 | 96 | 480 | 480 | 112 | 80 | 320 | 1568 | 352 | 28 | 28 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 384 | 5 | 16 | 0 | 5 | 3277 | 3277 | 10888.1 | 7611.1 | 80 | 160 | -243.0 | 323.1 | 323.1 | 351.9 | 646.5 | 114 | -680.2 | 96 | 480 | 1632 | 112 | 80 | 320 | 2720 | 352 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 384 | 6 | 16 | 0 | 6 | 2731 | 3277 | 10888.1 | 8157.1 | 80 | 160 | -243.0 | 316.4 | 316.4 | 346.0 | 607.9 | 116 | -692.6 | 96 | 480 | 1632 | 112 | 80 | 320 | 2720 | 352 | 36 | 36 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 384 | 8 | 16 | 0 | 8 | 2049 | 3277 | 10888.1 | 8839.1 | 80 | 160 | -243.0 | 305.6 | 305.6 | 336.5 | 546.0 | 121 | -712.2 | 96 | 480 | 1632 | 112 | 80 | 320 | 2720 | 352 | 32 | 32 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 128 | 32 | 160 | 384 | 10 | 16 | 0 | 10 | 1639 | 3277 | 10888.1 | 9249.1 | 80 | 160 | -243.1 | 297.0 | 297.0 | 329.0 | 497.2 | 125 | -727.5 | 96 | 480 | 1632 | 112 | 80 | 320 | 2720 | 352 | 28 | 28 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
