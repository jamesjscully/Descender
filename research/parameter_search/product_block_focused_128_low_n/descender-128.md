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
- m0 source: direct values `144, 160, 176, 192, 208, 224`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `90`
- candidates shown: `64`
- rejected candidates: `30`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 144 | 32 | 176 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 49358.2 | 46081.2 | 80 | 160 | -502.3 | 249.8 | 452.4 | 249.8 | 1071.2 | 159 | -1330.0 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 144 | 32 | 176 | 160 | 5 | 16 | 0 | 5 | 3277 | 3277 | 49358.2 | 46081.2 | 80 | 160 | -502.3 | 249.8 | 452.4 | 249.8 | 1071.2 | 159 | -1330.0 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 144 | 32 | 176 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 49358.2 | 46081.2 | 80 | 160 | -502.3 | 249.8 | 452.4 | 249.8 | 1071.2 | 159 | -1330.0 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 144 | 32 | 176 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 49358.2 | 46081.2 | 80 | 160 | -502.3 | 243.0 | 452.4 | 243.0 | 1071.2 | 159 | -1330.0 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 144 | 32 | 176 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 49358.2 | 46081.2 | 80 | 160 | -502.3 | 208.4 | 452.4 | 208.4 | 1071.2 | 159 | -1330.0 | 64 | 0 | 0 | 0 | 0 | 352 | 416 | 384 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 160 | 32 | 192 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 30979.0 | 27702.0 | 80 | 160 | -543.0 | 359.4 | 452.4 | 359.4 | 1015.8 | 159 | -1160.2 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 160 | 32 | 192 | 160 | 5 | 16 | 0 | 5 | 3277 | 3277 | 30979.0 | 27702.0 | 80 | 160 | -543.0 | 333.1 | 452.4 | 333.1 | 1015.8 | 159 | -1160.2 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 160 | 32 | 192 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 30979.0 | 27702.0 | 80 | 160 | -543.0 | 272.7 | 452.4 | 272.7 | 1015.8 | 159 | -1160.2 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 160 | 32 | 192 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 30979.0 | 27702.0 | 80 | 160 | -543.0 | 245.2 | 452.4 | 245.2 | 1015.8 | 159 | -1160.2 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 160 | 32 | 192 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 30979.0 | 27702.0 | 80 | 160 | -543.0 | 210.2 | 452.4 | 210.2 | 1015.8 | 159 | -1160.2 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 38 | 38 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 128 | 160 | 32 | 192 | 160 | 5 | 16 | 0 | 5 | 3277 | 3277 | 73681.6 | 70404.6 | 80 | 160 | -545.1 | 252.1 | 517.0 | 252.1 | 1255.8 | 182 | -1570.0 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 128 | 160 | 32 | 192 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 73681.6 | 70404.6 | 80 | 160 | -545.1 | 252.1 | 517.0 | 252.1 | 1255.8 | 182 | -1570.0 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 128 | 160 | 32 | 192 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 73681.6 | 70404.6 | 80 | 160 | -545.1 | 245.2 | 517.0 | 245.2 | 1255.8 | 182 | -1570.0 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 128 | 160 | 32 | 192 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 73681.6 | 70404.6 | 80 | 160 | -545.1 | 217.3 | 517.0 | 217.3 | 1255.8 | 182 | -1570.0 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 128 | 160 | 32 | 192 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 73681.6 | 70404.6 | 80 | 160 | -545.1 | 188.9 | 517.0 | 188.9 | 1255.8 | 182 | -1570.0 | 64 | 0 | 0 | 0 | 0 | 384 | 448 | 416 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 176 | 32 | 208 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 21022.2 | 17745.2 | 80 | 160 | -583.8 | 395.5 | 452.4 | 395.5 | 960.5 | 159 | -990.5 | 64 | 0 | 0 | 0 | 0 | 416 | 480 | 448 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 176 | 32 | 208 | 160 | 5 | 16 | 0 | 5 | 3277 | 3277 | 21022.2 | 17745.2 | 80 | 160 | -583.8 | 336.1 | 452.4 | 336.1 | 960.5 | 159 | -990.5 | 64 | 0 | 0 | 0 | 0 | 416 | 480 | 448 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 176 | 32 | 208 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 21022.2 | 17745.2 | 80 | 160 | -583.8 | 275.0 | 452.4 | 275.0 | 960.5 | 159 | -990.5 | 64 | 0 | 0 | 0 | 0 | 416 | 480 | 448 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 176 | 32 | 208 | 112 | 5 | 16 | 0 | 5 | 3277 | 3277 | 21022.2 | 17745.2 | 80 | 160 | -583.8 | 247.2 | 452.4 | 247.2 | 960.5 | 159 | -990.5 | 64 | 0 | 0 | 0 | 0 | 416 | 480 | 448 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 112 | 176 | 32 | 208 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 21022.2 | 17745.2 | 80 | 160 | -583.8 | 211.9 | 452.4 | 211.9 | 960.5 | 159 | -990.5 | 64 | 0 | 0 | 0 | 0 | 416 | 480 | 448 | 54 | 54 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
