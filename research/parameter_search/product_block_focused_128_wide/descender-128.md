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
- m0 source: direct values `192, 256, 320, 384, 512, 640, 768, 1024, 1280`
- m1 source: direct values `32`
- view coordinate counts: `0`
- security extra cap: `16.0` bits
- guard slack target: `8`
- combined certification weight: `0.250`
- accepted candidates before truncation: `5`
- candidates shown: `5`
- rejected candidates: `427`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 192 | 256 | 32 | 288 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 109920.9 | 106643.9 | 80 | 160 | -798.3 | 135.3 | 775.5 | 135.3 | 1883.7 | 272 | -2190.5 | 64 | 0 | 0 | 0 | 0 | 576 | 640 | 608 | 54 | 54 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 192 | 320 | 32 | 352 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 38018.4 | 34741.4 | 80 | 160 | -961.6 | 137.6 | 775.5 | 137.6 | 1662.3 | 272 | -1511.6 | 64 | 0 | 0 | 0 | 0 | 704 | 768 | 736 | 118 | 118 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 192 | 384 | 32 | 416 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 18749.0 | 15472.0 | 80 | 160 | -1125.2 | 139.6 | 775.5 | 139.6 | 1440.9 | 272 | -832.7 | 64 | 0 | 0 | 0 | 0 | 832 | 896 | 864 | 182 | 182 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 192 | 512 | 32 | 544 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 7911.4 | 4634.4 | 80 | 160 | -1452.3 | 142.7 | 775.5 | 142.7 | 998.1 | 272 | 525.1 | 64 | 0 | 0 | 0 | 0 | 1088 | 1152 | 1120 | 310 | 310 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 192 | 640 | 32 | 672 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 4851.9 | 1574.9 | 80 | 160 | -1779.4 | 145.2 | 775.5 | 145.2 | 555.3 | 272 | 1882.9 | 64 | 0 | 0 | 0 | 0 | 1344 | 1408 | 1376 | 438 | 438 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
