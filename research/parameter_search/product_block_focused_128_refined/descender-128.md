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
- m0 source: direct values `192, 208, 224, 240, 256, 272, 288, 304, 320`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `103`
- candidates shown: `64`
- rejected candidates: `122`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 192 | 32 | 224 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 140712.2 | 137435.2 | 80 | 160 | -630.7 | 199.0 | 646.3 | 199.0 | 1625.1 | 227 | -2050.0 | 64 | 0 | 0 | 0 | 0 | 448 | 512 | 480 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 192 | 32 | 224 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 140712.2 | 137435.2 | 80 | 160 | -630.7 | 177.1 | 646.3 | 177.1 | 1625.1 | 227 | -2050.0 | 64 | 0 | 0 | 0 | 0 | 448 | 512 | 480 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 192 | 32 | 224 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 140712.2 | 137435.2 | 80 | 160 | -630.7 | 154.9 | 646.3 | 154.9 | 1625.1 | 227 | -2050.0 | 64 | 0 | 0 | 0 | 0 | 448 | 512 | 480 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 192 | 32 | 224 | 80 | 5 | 16 | 0 | 5 | 3277 | 3277 | 140712.2 | 137435.2 | 80 | 160 | -631.3 | 132.3 | 646.3 | 132.3 | 1625.1 | 227 | -2050.0 | 64 | 0 | 0 | 0 | 0 | 448 | 512 | 480 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 208 | 32 | 240 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 91804.8 | 88527.8 | 80 | 160 | -671.5 | 200.3 | 646.3 | 200.3 | 1569.8 | 227 | -1880.2 | 64 | 0 | 0 | 0 | 0 | 480 | 544 | 512 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 208 | 32 | 240 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 91804.8 | 88527.8 | 80 | 160 | -671.5 | 178.2 | 646.3 | 178.2 | 1569.8 | 227 | -1880.2 | 64 | 0 | 0 | 0 | 0 | 480 | 544 | 512 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 208 | 32 | 240 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 91804.8 | 88527.8 | 80 | 160 | -671.5 | 155.8 | 646.3 | 155.8 | 1569.8 | 227 | -1880.2 | 64 | 0 | 0 | 0 | 0 | 480 | 544 | 512 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 208 | 32 | 240 | 80 | 5 | 16 | 0 | 5 | 3277 | 3277 | 91804.8 | 88527.8 | 80 | 160 | -672.0 | 133.1 | 646.3 | 133.1 | 1569.8 | 227 | -1880.2 | 64 | 0 | 0 | 0 | 0 | 480 | 544 | 512 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 176 | 208 | 32 | 240 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 183610.8 | 180333.8 | 80 | 160 | -673.6 | 185.6 | 710.9 | 185.6 | 1809.8 | 249 | -2290.0 | 64 | 0 | 0 | 0 | 0 | 480 | 544 | 512 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 176 | 208 | 32 | 240 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 183610.8 | 180333.8 | 80 | 160 | -673.6 | 163.3 | 710.9 | 163.3 | 1809.8 | 249 | -2290.0 | 64 | 0 | 0 | 0 | 0 | 480 | 544 | 512 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 176 | 208 | 32 | 240 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 183610.8 | 180333.8 | 80 | 160 | -673.7 | 140.7 | 710.9 | 140.7 | 1809.8 | 249 | -2290.0 | 64 | 0 | 0 | 0 | 0 | 480 | 544 | 512 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 224 | 32 | 256 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 63443.9 | 60166.9 | 80 | 160 | -712.2 | 201.5 | 646.3 | 201.5 | 1514.4 | 227 | -1710.5 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 224 | 32 | 256 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 63443.9 | 60166.9 | 80 | 160 | -712.2 | 179.3 | 646.3 | 179.3 | 1514.4 | 227 | -1710.5 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 224 | 32 | 256 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 63443.9 | 60166.9 | 80 | 160 | -712.2 | 156.8 | 646.3 | 156.8 | 1514.4 | 227 | -1710.5 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 160 | 224 | 32 | 256 | 80 | 5 | 16 | 0 | 5 | 3277 | 3277 | 63443.9 | 60166.9 | 80 | 160 | -712.7 | 133.9 | 646.3 | 133.9 | 1514.4 | 227 | -1710.5 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 176 | 224 | 32 | 256 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 121508.8 | 118231.8 | 80 | 160 | -714.3 | 186.8 | 710.9 | 186.8 | 1754.4 | 249 | -2120.2 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 176 | 224 | 32 | 256 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 121508.8 | 118231.8 | 80 | 160 | -714.3 | 164.3 | 710.9 | 164.3 | 1754.4 | 249 | -2120.2 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 176 | 224 | 32 | 256 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 121508.8 | 118231.8 | 80 | 160 | -714.4 | 141.5 | 710.9 | 141.5 | 1754.4 | 249 | -2120.2 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 192 | 224 | 32 | 256 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 232715.8 | 229438.8 | 80 | 160 | -716.4 | 171.8 | 775.5 | 171.8 | 1994.4 | 272 | -2530.0 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 192 | 224 | 32 | 256 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 232715.8 | 229438.8 | 80 | 160 | -716.4 | 149.2 | 775.5 | 149.2 | 1994.4 | 272 | -2530.0 | 64 | 0 | 0 | 0 | 0 | 512 | 576 | 544 | 22 | 22 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
