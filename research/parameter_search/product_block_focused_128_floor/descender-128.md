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
- Lee repetitions: `3277`
- Lee rows per product block: `5`
- product-block sidecar manifests: `none`
- m0 source: direct values `112, 120, 128, 136, 144, 152, 160`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `174`
- candidates shown: `64`
- rejected candidates: `36`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -416.6 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -416.6 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 160 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -416.6 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -416.6 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 224 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -416.6 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -416.6 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 112 | 32 | 144 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 31174.6 | 27897.6 | 80 | 160 | -417.7 | 190.4 | 355.5 | 190.4 | 821.9 | 125 | -1054.8 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 112 | 32 | 144 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 31174.6 | 27897.6 | 80 | 160 | -417.7 | 190.4 | 355.5 | 190.4 | 821.9 | 125 | -1054.8 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 112 | 32 | 144 | 160 | 5 | 16 | 0 | 5 | 3277 | 3277 | 31174.6 | 27897.6 | 80 | 160 | -417.7 | 190.4 | 355.5 | 190.4 | 821.9 | 125 | -1054.8 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 112 | 32 | 144 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 31174.6 | 27897.6 | 80 | 160 | -417.7 | 190.4 | 355.5 | 190.4 | 821.9 | 125 | -1054.8 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 112 | 32 | 144 | 224 | 5 | 16 | 0 | 5 | 3277 | 3277 | 31174.6 | 27897.6 | 80 | 160 | -417.7 | 190.4 | 355.5 | 190.4 | 821.9 | 125 | -1054.8 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 112 | 32 | 144 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 31174.6 | 27897.6 | 80 | 160 | -417.7 | 190.4 | 355.5 | 190.4 | 821.9 | 125 | -1054.8 | 64 | 0 | 0 | 0 | 0 | 288 | 352 | 320 | 14 | 14 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 120 | 32 | 152 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 13612.2 | 10335.2 | 80 | 160 | -437.0 | 298.6 | 323.1 | 298.6 | 674.2 | 114 | -765.1 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 30 | 30 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 120 | 32 | 152 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 13612.2 | 10335.2 | 80 | 160 | -437.0 | 298.6 | 323.1 | 298.6 | 674.2 | 114 | -765.1 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 30 | 30 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 120 | 32 | 152 | 160 | 5 | 16 | 0 | 5 | 3277 | 3277 | 13612.2 | 10335.2 | 80 | 160 | -437.0 | 298.6 | 323.1 | 298.6 | 674.2 | 114 | -765.1 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 30 | 30 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 120 | 32 | 152 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 13612.2 | 10335.2 | 80 | 160 | -437.0 | 298.6 | 323.1 | 298.6 | 674.2 | 114 | -765.1 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 30 | 30 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 120 | 32 | 152 | 224 | 5 | 16 | 0 | 5 | 3277 | 3277 | 13612.2 | 10335.2 | 80 | 160 | -437.0 | 298.6 | 323.1 | 298.6 | 674.2 | 114 | -765.1 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 30 | 30 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 120 | 32 | 152 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 13612.2 | 10335.2 | 80 | 160 | -437.0 | 279.1 | 323.1 | 279.1 | 674.2 | 114 | -765.1 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 30 | 30 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 120 | 32 | 152 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23528.0 | 20251.0 | 80 | 160 | -438.0 | 246.0 | 355.5 | 246.0 | 794.2 | 125 | -970.0 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 88 | 120 | 32 | 152 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 23528.0 | 20251.0 | 80 | 160 | -438.0 | 246.0 | 355.5 | 246.0 | 794.2 | 125 | -970.0 | 64 | 0 | 0 | 0 | 0 | 304 | 368 | 336 | 22 | 22 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
