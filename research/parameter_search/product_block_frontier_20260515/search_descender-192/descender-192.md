# Parameter Search Report: descender-192

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.
The bandwidth objective treats proof-side conditions as hard filters, caps extra security credit, and strongly prioritizes ciphertext bytes before public-key bytes and dimension.

- target security bits: `192`
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
- guard slack target: `12`
- combined certification weight: `0.250`
- accepted candidates before truncation: `972`
- candidates shown: `200`
- rejected candidates: `14148`

| proof | primal model | q-amp | rep | q | n | m0 | m1 | view | k | eta | sample bits | round err | eff eta | amp req | amp cert | Minkowski L1 <= | geom margin | amp rows | qamp ops | objective | security proxy | primal | dual | self-rand | lossy diag req | lossy diag slack | seed bytes | cauchy bytes | coeff bytes | incidence bytes | diagonal bytes | sample bytes | pk bytes | ct bytes | correctness slack | guard slack | combined pressure | combined margin |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 96 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -288.0 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 96 | 448 | 480 | 112 | 80 | 288 | 1504 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 96 | 6 | 16 | 0 | 6 | 2731 | 3277 | 17496.1 | 14765.1 | 80 | 160 | -288.0 | 240.5 | 316.4 | 240.5 | 667.1 | 116 | -858.3 | 96 | 448 | 480 | 112 | 80 | 288 | 1504 | 320 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 96 | 8 | 16 | 0 | 8 | 2049 | 3277 | 17496.1 | 15447.1 | 80 | 160 | -288.1 | 234.1 | 305.6 | 234.1 | 611.4 | 121 | -871.5 | 96 | 448 | 480 | 112 | 80 | 288 | 1504 | 320 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 96 | 10 | 16 | 0 | 10 | 1639 | 3277 | 17496.1 | 15857.1 | 80 | 160 | -288.1 | 229.1 | 297.0 | 229.1 | 567.5 | 125 | -881.8 | 96 | 448 | 480 | 112 | 80 | 288 | 1504 | 320 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 128 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -288.5 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 96 | 448 | 608 | 112 | 80 | 288 | 1632 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 128 | 6 | 16 | 0 | 6 | 2731 | 3277 | 17496.1 | 14765.1 | 80 | 160 | -288.5 | 240.5 | 316.4 | 240.5 | 667.1 | 116 | -858.3 | 96 | 448 | 608 | 112 | 80 | 288 | 1632 | 320 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 128 | 8 | 16 | 0 | 8 | 2049 | 3277 | 17496.1 | 15447.1 | 80 | 160 | -288.5 | 234.1 | 305.6 | 234.1 | 611.4 | 121 | -871.5 | 96 | 448 | 608 | 112 | 80 | 288 | 1632 | 320 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 128 | 10 | 16 | 0 | 10 | 1639 | 3277 | 17496.1 | 15857.1 | 80 | 160 | -288.6 | 229.1 | 297.0 | 229.1 | 567.5 | 125 | -881.8 | 96 | 448 | 608 | 112 | 80 | 288 | 1632 | 320 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 192 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -289.5 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 96 | 448 | 864 | 112 | 80 | 288 | 1888 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 192 | 6 | 16 | 0 | 6 | 2731 | 3277 | 17496.1 | 14765.1 | 80 | 160 | -289.5 | 240.5 | 316.4 | 240.5 | 667.1 | 116 | -858.3 | 96 | 448 | 864 | 112 | 80 | 288 | 1888 | 320 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 192 | 8 | 16 | 0 | 8 | 2049 | 3277 | 17496.1 | 15447.1 | 80 | 160 | -289.5 | 234.1 | 305.6 | 234.1 | 611.4 | 121 | -871.5 | 96 | 448 | 864 | 112 | 80 | 288 | 1888 | 320 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 192 | 10 | 16 | 0 | 10 | 1639 | 3277 | 17496.1 | 15857.1 | 80 | 160 | -289.6 | 229.1 | 297.0 | 229.1 | 567.5 | 125 | -881.8 | 96 | 448 | 864 | 112 | 80 | 288 | 1888 | 320 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 256 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -290.4 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 96 | 448 | 1120 | 112 | 80 | 288 | 2144 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 256 | 6 | 16 | 0 | 6 | 2731 | 3277 | 17496.1 | 14765.1 | 80 | 160 | -290.5 | 240.5 | 316.4 | 240.5 | 667.1 | 116 | -858.3 | 96 | 448 | 1120 | 112 | 80 | 288 | 2144 | 320 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 256 | 8 | 16 | 0 | 8 | 2049 | 3277 | 17496.1 | 15447.1 | 80 | 160 | -290.5 | 234.1 | 305.6 | 234.1 | 611.4 | 121 | -871.5 | 96 | 448 | 1120 | 112 | 80 | 288 | 2144 | 320 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 256 | 10 | 16 | 0 | 10 | 1639 | 3277 | 17496.1 | 15857.1 | 80 | 160 | -290.5 | 229.1 | 297.0 | 229.1 | 567.5 | 125 | -881.8 | 96 | 448 | 1120 | 112 | 80 | 288 | 2144 | 320 | 12 | 12 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 384 | 5 | 16 | 0 | 5 | 3277 | 3277 | 17496.1 | 14219.1 | 80 | 160 | -292.4 | 244.5 | 323.1 | 244.5 | 701.9 | 114 | -850.0 | 96 | 448 | 1632 | 112 | 80 | 288 | 2656 | 320 | 22 | 22 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 384 | 6 | 16 | 0 | 6 | 2731 | 3277 | 17496.1 | 14765.1 | 80 | 160 | -292.4 | 240.5 | 316.4 | 240.5 | 667.1 | 116 | -858.3 | 96 | 448 | 1632 | 112 | 80 | 288 | 2656 | 320 | 20 | 20 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 384 | 8 | 16 | 0 | 8 | 2049 | 3277 | 17496.1 | 15447.1 | 80 | 160 | -292.4 | 234.1 | 305.6 | 234.1 | 611.4 | 121 | -871.5 | 96 | 448 | 1632 | 112 | 80 | 288 | 2656 | 320 | 16 | 16 | 0.0 | 0 |
| full-surjective | guard-only | product-block-lee | 3277 | 32771 | 80 | 112 | 32 | 144 | 384 | 10 | 16 | 0 | 10 | 1639 | 3277 | 17496.1 | 15857.1 | 80 | 160 | -292.5 | 229.1 | 297.0 | 229.1 | 567.5 | 125 | -881.8 | 96 | 448 | 1632 | 112 | 80 | 288 | 2656 | 320 | 12 | 12 | 0.0 | 0 |

## Next Checks

1. Treat `qary_amplitude_minkowski_obstruction` as a hard first-principles rejection, not a tunable proxy.
2. Run view-only q-ary centered-L1/BKZ reductions for top frontier rows before estimator work.
3. Run exact finite-field rank, projection-distance, and bounded Lee-dual checks for survivors.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
