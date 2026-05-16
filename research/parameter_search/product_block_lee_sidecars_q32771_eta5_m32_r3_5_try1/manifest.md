# Product-Block Lee Sidecar Sweep

- family: `shared-random-pair-block`
- m1: `32`
- group size: `2`
- trials per point: `64`
- seed: `20260515`

| q | eta | target l1 | rows/group | used rows | qamp ops | pass | trial | checked pairs | sidecar |
|---:|---:|---:|---:|---:|---:|---|---:|---:|---|
| 32771 | 5 | 3277 | 3 | 48 | 96 | false | 22 | 262659 | research/parameter_search/product_block_lee_sidecars_q32771_eta5_m32_r3_5_try1/q32771_m132_target3277_rows_per_group3.json |
| 32771 | 5 | 3277 | 4 | 64 | 128 | false | 2 | 2260566 | research/parameter_search/product_block_lee_sidecars_q32771_eta5_m32_r3_5_try1/q32771_m132_target3277_rows_per_group4.json |
| 32771 | 5 | 3277 | 5 | 80 | 160 | true | 0 | 21470904 | research/parameter_search/product_block_lee_sidecars_q32771_eta5_m32_r3_5_try1/q32771_m132_target3277_rows_per_group5.json |

Passing entries are exact sidecars for product-block-lee validation.
The local check enumerates every nonzero centered pair whose free l1 is below target_l1.
Rows/group controls the proof-object cost: used_rows = rows_per_group * (m1 / 2).
