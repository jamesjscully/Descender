# Product-Block Lee Sidecar Sweep

- family: `shared-random-pair-block`
- m1: `32`
- group size: `2`
- trials per point: `8`
- seed: `20260515`

| q | eta | target l1 | rows/group | used rows | qamp ops | pass | trial | checked pairs | sidecar |
|---:|---:|---:|---:|---:|---:|---|---:|---:|---|
| 12289 | 5 | 1229 | 3 | 48 | 96 | false | 4 | 168481 | - |
| 12289 | 5 | 1229 | 4 | 64 | 128 | false | 2 | 308728 | - |
| 12289 | 5 | 1229 | 5 | 80 | 160 | true | 0 | 3018424 | research/parameter_search/product_block_lee_sidecars_20260515/q12289_m132_target1229_rows_per_group5.json |
| 12289 | 6 | 1024 | 3 | 48 | 96 | false | 7 | 89097 | - |
| 12289 | 6 | 1024 | 4 | 64 | 128 | false | 5 | 634383 | - |
| 12289 | 6 | 1024 | 5 | 80 | 160 | true | 0 | 2095104 | research/parameter_search/product_block_lee_sidecars_20260515/q12289_m132_target1024_rows_per_group5.json |
| 12289 | 8 | 768 | 3 | 48 | 96 | false | 6 | 108521 | - |
| 12289 | 8 | 768 | 4 | 64 | 128 | true | 0 | 1178112 | research/parameter_search/product_block_lee_sidecars_20260515/q12289_m132_target768_rows_per_group4.json |
| 12289 | 8 | 768 | 5 | 80 | 160 | true | 0 | 1178112 | research/parameter_search/product_block_lee_sidecars_20260515/q12289_m132_target768_rows_per_group5.json |
| 12289 | 10 | 615 | 3 | 48 | 96 | false | 5 | 225099 | - |
| 12289 | 10 | 615 | 4 | 64 | 128 | true | 0 | 755220 | research/parameter_search/product_block_lee_sidecars_20260515/q12289_m132_target615_rows_per_group4.json |
| 12289 | 10 | 615 | 5 | 80 | 160 | true | 0 | 755220 | research/parameter_search/product_block_lee_sidecars_20260515/q12289_m132_target615_rows_per_group5.json |
| 32771 | 5 | 3277 | 3 | 48 | 96 | false | 3 | 127880 | - |
| 32771 | 5 | 3277 | 4 | 64 | 128 | false | 3 | 3191973 | - |
| 32771 | 5 | 3277 | 5 | 80 | 160 | true | 2 | 21470904 | research/parameter_search/product_block_lee_sidecars_20260515/q32771_m132_target3277_rows_per_group5.json |
| 32771 | 6 | 2731 | 3 | 48 | 96 | false | 0 | 793250 | - |
| 32771 | 6 | 2731 | 4 | 64 | 128 | false | 4 | 2130479 | - |
| 32771 | 6 | 2731 | 5 | 80 | 160 | true | 0 | 14911260 | research/parameter_search/product_block_lee_sidecars_20260515/q32771_m132_target2731_rows_per_group5.json |
| 32771 | 8 | 2049 | 3 | 48 | 96 | false | 2 | 558871 | - |
| 32771 | 8 | 2049 | 4 | 64 | 128 | false | 6 | 2909450 | - |
| 32771 | 8 | 2049 | 5 | 80 | 160 | true | 0 | 8392704 | research/parameter_search/product_block_lee_sidecars_20260515/q32771_m132_target2049_rows_per_group5.json |
| 32771 | 10 | 1639 | 3 | 48 | 96 | false | 1 | 136778 | - |
| 32771 | 10 | 1639 | 4 | 64 | 128 | true | 3 | 5369364 | research/parameter_search/product_block_lee_sidecars_20260515/q32771_m132_target1639_rows_per_group4.json |
| 32771 | 10 | 1639 | 5 | 80 | 160 | true | 0 | 5369364 | research/parameter_search/product_block_lee_sidecars_20260515/q32771_m132_target1639_rows_per_group5.json |

Passing entries are exact sidecars for product-block-lee validation.
The local check enumerates every nonzero centered pair whose free l1 is below target_l1.
Rows/group controls the proof-object cost: used_rows = rows_per_group * (m1 / 2).
