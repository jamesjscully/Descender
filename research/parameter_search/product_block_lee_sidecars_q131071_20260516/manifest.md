# Product-Block Lee Sidecar Sweep

- family: `shared-random-pair-block`
- m1: `32`
- group size: `2`
- trials per point: `8`
- seed: `20260516`

| q | eta | target l1 | rows/group | used rows | qamp ops | pass | trial | checked pairs | sidecar |
|---:|---:|---:|---:|---:|---:|---|---:|---:|---|
| 131071 | 10 | 6554 | 4 | 64 | 128 | false | 6 | 16578211 | - |
| 131071 | 10 | 6554 | 5 | 80 | 160 | true | 0 | 85896724 | research/parameter_search/product_block_lee_sidecars_q131071_20260516/q131071_m132_target6554_rows_per_group5.json |
| 131071 | 10 | 6554 | 6 | 96 | 192 | true | 0 | 85896724 | research/parameter_search/product_block_lee_sidecars_q131071_20260516/q131071_m132_target6554_rows_per_group6.json |
| 131071 | 12 | 5462 | 4 | 64 | 128 | false | 1 | 10920092 | - |
| 131071 | 12 | 5462 | 5 | 80 | 160 | true | 0 | 59655964 | research/parameter_search/product_block_lee_sidecars_q131071_20260516/q131071_m132_target5462_rows_per_group5.json |
| 131071 | 12 | 5462 | 6 | 96 | 192 | true | 0 | 59655964 | research/parameter_search/product_block_lee_sidecars_q131071_20260516/q131071_m132_target5462_rows_per_group6.json |
| 131071 | 16 | 4096 | 4 | 64 | 128 | true | 0 | 33546240 | research/parameter_search/product_block_lee_sidecars_q131071_20260516/q131071_m132_target4096_rows_per_group4.json |
| 131071 | 16 | 4096 | 5 | 80 | 160 | true | 0 | 33546240 | research/parameter_search/product_block_lee_sidecars_q131071_20260516/q131071_m132_target4096_rows_per_group5.json |
| 131071 | 16 | 4096 | 6 | 96 | 192 | true | 0 | 33546240 | research/parameter_search/product_block_lee_sidecars_q131071_20260516/q131071_m132_target4096_rows_per_group6.json |

Passing entries are exact sidecars for product-block-lee validation.
The local check enumerates every nonzero centered pair whose free l1 is below target_l1.
Rows/group controls the proof-object cost: used_rows = rows_per_group * (m1 / 2).
