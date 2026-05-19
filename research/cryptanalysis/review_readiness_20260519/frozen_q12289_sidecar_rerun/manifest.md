# Product-Block Lee Sidecar Sweep

- family: `shared-random-pair-block`
- m1: `32`
- group size: `2`
- trials per point: `16`
- seed: `1`

| q | eta | target l1 | rows/group | used rows | qamp ops | pass | trial | checked pairs | sidecar |
|---:|---:|---:|---:|---:|---:|---|---:|---:|---|
| 12289 | - | 512 | 3 | 48 | 96 | true | 15 | 523264 | research/cryptanalysis/review_readiness_20260519/frozen_q12289_sidecar_rerun/q12289_m132_target512_rows_per_group3.json |

Passing entries are exact sidecars for product-block-lee validation.
The local check enumerates every nonzero centered pair whose free l1 is below target_l1.
Rows/group controls the proof-object cost: used_rows = rows_per_group * (m1 / 2).
