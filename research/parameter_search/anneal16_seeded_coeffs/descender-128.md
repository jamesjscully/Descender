# Parameter Search Report: descender-128

This is a coarse screening report. The `*_proxy_bits` and `objective` columns are not security claims;
they are filters for deciding which candidates deserve exact projection, dual-witness,
and lattice/coding-estimator follow-up.  The objective caps excess security credit and penalizes
public size, ciphertext size, arithmetic cost, tiny noise, and guard redundancy.

- target security bits: `128`
- accepted candidates before truncation: `21363376`
- candidates shown: `5000`
- rejected candidates: `4236624`

| q | n | m0 | m1 | k | eta | objective | security proxy | primal | dual | self-rand | coeff bytes | pk bytes | ct bytes | guard slack |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 8388617 | 512 | 896 | 1536 | 96 | 4 | 450.6 | 3491.0 | 3553.5 | 3491.0 | 4066.7 | 672 | 23424 | 7328 | 376 |
| 8388617 | 512 | 896 | 1536 | 96 | 3 | 448.1 | 3543.2 | 3618.5 | 3543.2 | 4948.5 | 672 | 23424 | 7328 | 378 |
| 8388617 | 512 | 896 | 1536 | 96 | 2 | 446.9 | 3613.1 | 3705.5 | 3613.1 | 6129.1 | 672 | 23424 | 7328 | 380 |
| 4194319 | 512 | 896 | 1536 | 96 | 4 | 446.5 | 3347.0 | 3374.4 | 3347.0 | 3554.7 | 672 | 23424 | 7328 | 376 |
| 8388617 | 512 | 1024 | 1536 | 96 | 4 | 446.2 | 3502.4 | 3553.5 | 3502.4 | 3661.0 | 672 | 23808 | 7712 | 504 |
| 8388617 | 480 | 864 | 1440 | 96 | 4 | 444.3 | 3331.5 | 3331.5 | 3479.1 | 3736.5 | 672 | 22080 | 6944 | 376 |
| 4194319 | 512 | 896 | 1536 | 96 | 3 | 444.2 | 3399.2 | 3439.3 | 3399.2 | 4436.5 | 672 | 23424 | 7328 | 378 |
| 8388617 | 512 | 1024 | 1536 | 96 | 3 | 443.8 | 3554.6 | 3618.5 | 3554.6 | 4589.2 | 672 | 23808 | 7712 | 506 |
| 4194319 | 512 | 896 | 1536 | 96 | 2 | 443.0 | 3469.1 | 3526.3 | 3469.1 | 5617.1 | 672 | 23424 | 7328 | 380 |
| 8388617 | 480 | 864 | 1440 | 96 | 3 | 442.6 | 3392.4 | 3392.4 | 3531.3 | 4571.9 | 672 | 22080 | 6944 | 378 |
| 8388617 | 512 | 1024 | 1536 | 96 | 2 | 442.5 | 3624.5 | 3705.5 | 3624.5 | 5831.9 | 672 | 23808 | 7712 | 508 |
| 8388617 | 576 | 960 | 1728 | 96 | 4 | 442.2 | 3513.1 | 3997.7 | 3513.1 | 4727.2 | 672 | 26112 | 8096 | 376 |
| 8388617 | 480 | 864 | 1440 | 96 | 2 | 442.2 | 3473.9 | 3473.9 | 3601.2 | 5690.3 | 672 | 22080 | 6944 | 380 |
| 8388617 | 512 | 896 | 1536 | 96 | 1 | 442.0 | 3719.3 | 3837.6 | 3719.3 | 7921.4 | 672 | 23424 | 7328 | 382 |
| 8388617 | 512 | 896 | 1536 | 96 | 5 | 441.6 | 3362.7 | 3501.7 | 3449.3 | 3362.7 | 672 | 23424 | 7328 | 374 |
| 2097169 | 512 | 896 | 1536 | 96 | 3 | 440.0 | 3255.2 | 3260.1 | 3255.2 | 3924.5 | 672 | 23424 | 7328 | 378 |
| 4194319 | 512 | 1024 | 1536 | 96 | 3 | 439.8 | 3410.6 | 3439.3 | 3410.6 | 4077.2 | 672 | 23808 | 7712 | 506 |
| 8388617 | 576 | 960 | 1728 | 96 | 3 | 439.8 | 3565.3 | 4070.8 | 3565.3 | 5701.8 | 672 | 26112 | 8096 | 378 |
| 8388617 | 576 | 960 | 1728 | 96 | 5 | 439.4 | 3471.4 | 3939.4 | 3471.4 | 3949.0 | 672 | 26112 | 8096 | 374 |
| 8388617 | 480 | 992 | 1440 | 96 | 4 | 439.0 | 3330.7 | 3331.5 | 3491.0 | 3330.7 | 672 | 22464 | 7328 | 504 |

## Next Checks

1. Run exact finite-field rank and projection-distance checks for the top frontier rows.
2. Run dual-witness searches on scaled toy instances preserving the same ratios.
3. Replace screening proxies with a real lattice/coding attack estimator.
4. Keep only candidates whose proof-side constraints can be instantiated in Lean.
