# Product-Block Lee Sidecar Sweep

- family: `shared-random-pair-block`
- m1: `32`
- group size: `2`
- trials per point: `64`
- seed: `20260517`

| q | eta | target l1 | rows/group | used rows | qamp ops | pass | trial | checked pairs | sidecar |
|---:|---:|---:|---:|---:|---:|---|---:|---:|---|
| 32771 | 6 | 2731 | 3 | 48 | 96 | false | 21 | 561799 | research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2731_rows_per_group3.json |
| 32771 | 6 | 2731 | 4 | 64 | 128 | false | 23 | 1551518 | research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2731_rows_per_group4.json |
| 32771 | 8 | 2049 | 3 | 48 | 96 | false | 0 | 370176 | research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2049_rows_per_group3.json |
| 32771 | 8 | 2049 | 4 | 64 | 128 | true | 1 | 8392704 | research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2049_rows_per_group4.json |

Passing entries are exact sidecars for product-block-lee validation.
The local check enumerates every nonzero centered pair whose free l1 is below target_l1.
Rows/group controls the proof-object cost: used_rows = rows_per_group * (m1 / 2).
