# Product-Block Lee Focused 128-Bit Search

Generated 2026-05-15.

## Certified Sidecars

The first validation runs used:

```text
research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.json
```

and the independently verified local certificate:

```text
research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.verify.md
```

The sidecar certifies a repeated product block with:

```text
q = 32771
m1 = 32
group size = 2
group count = 16
rows per group = 5
product-block rows = 80
target centered Lee l1 = 3277
actual verified local minimum l1 = 3283
eta = 5, because 32771 <= 2 * 5 * 3277 + 1
```

Lean consumes this through:

```text
Descender.SharedRowProductBlock32771.noNarrowDualFromRepeatedLocal
```

A later sidecar sweep found a smaller proof-object row budget by accepting
larger noise:

```text
research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2049_rows_per_group4.json
research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2049_rows_per_group4.verify.md
```

That sidecar certifies:

```text
q = 32771
m1 = 32
group size = 2
group count = 16
rows per group = 4
product-block rows = 64
target centered Lee l1 = 2049
actual verified local minimum l1 = 2136
eta = 8, because 32771 <= 2 * 8 * 2049 + 1
checked local centered pairs = 8392704
artifact consistent = true
```

## Search Runs

Initial focused grid:

```text
research/parameter_search/product_block_focused_128
```

No rows passed; the grid was too small for the security/correctness filters.

Wide grid:

```text
research/parameter_search/product_block_focused_128_wide
research/parameter_search/product_block_focused_128_wide_validated
```

Best validated row:

```text
q = 32771
n = 192
m0 = 256
m1 = 32
k = 96
eta = 5
pk = 640 B
ct = 608 B
validation = pass
```

Refined grid:

```text
research/parameter_search/product_block_focused_128_refined
research/parameter_search/product_block_focused_128_refined_validated
```

Best validated row:

```text
q = 32771
n = 160
m0 = 192
m1 = 32
k = 128
eta = 5
pk = 512 B
ct = 480 B
validation = pass
```

Low-n floor grid:

```text
research/parameter_search/product_block_focused_128_floor
research/parameter_search/product_block_focused_128_floor_validated
```

Best validated row:

```text
q = 32771
n = 80
m0 = 112
m1 = 32
k = 112
eta = 5
pk = 352 B
ct = 320 B
security proxy = 244.5 bits
correctness slack = 22
amp l1 required = 3277
amp l1 certified = 3277
product-block rows = 80
validation = pass
```

Sub-80 probe:

```text
research/parameter_search/product_block_focused_128_sub80
```

No rows passed.  Every row hit `shared_pair_requires_group_rows`, because this
sidecar requires 80 product-block rows.  This makes `n = 80` the current floor
for the certified `rows_per_group = 5`, `m1 = 32` product-block construction.

## Interpretation

The product-block route is now the best proof-carrying concrete path found so
far.  It has:

- a verified local finite certificate;
- a Lean local-to-global product-block theorem;
- a Lean public-view no-narrow-dual theorem;
- frontier validation passing with the sidecar supplied.

The earlier eta-5 row was:

```text
pk = 352 B
ct = 320 B
q = 32771
n = 80
m0 = 112
m1 = 32
k = 112
eta = 5
```

The next structural optimization is not another `n < 80` scan with this same
sidecar.  To go below this floor, we need a new sidecar that reduces product
block rows, for example:

```text
rows_per_group = 4 for group size 2, or
larger local group sizes with fewer total rows.
```

That was the next meaningful public-size lever after the eta-5 run.

The eta-8 sweep supplied exactly that lever:

```text
research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4
research/parameter_search/product_block_sidecar_eta8_floor_search_fullwidth
research/parameter_search/product_block_sidecar_eta8_floor_search_fullwidth_validated
```

Best validated row:

```text
q = 32771
n = 64
m0 = 88
m1 = 32
k = 112
eta = 8
public sample = full field width (16 bits per coordinate)
pk = 304 B
ct = 272 B
security proxy = 179.0 bits
correctness slack = 8
guard slack = 8
amp l1 required = 2049
amp l1 certified = 2049
product-block rows = 64
validation = pass
```

This is now the current best validated 128-bit-screening row in the model.  It
beats the eta-5 row on bandwidth because the sidecar lowers the row floor from
80 to 64, but it spends the correctness slack: eta rises from 5 to 8 and the
row is exactly at the configured guard/correctness slack target.

Compressed-sample follow-up:

```text
research/parameter_search/product_block_sidecar_compressed_samples_search
research/parameter_search/product_block_sidecar_compressed_samples_search_validated
```

The search model was updated to treat public-sample compression as deterministic
post-processing of the uncompressed public sample.  Security should follow by
data processing from the uncompressed proof, while correctness and amplitude
checks pay the conservative rounding penalty through `effective_eta`.

Best validated compressed row:

```text
q = 32771
n = 64
m0 = 104
m1 = 32
k = 112
eta = 8
sample bits = 12
rounding error = 5
effective eta = 13
pk = 268 B
ct = 236 B
security proxy = 229.5 bits
correctness distance = 41
correctness slack = 14
guard slack = 14
amp l1 required = 1261
amp l1 certified = 2049
product-block rows = 64
validation = pass
```

This is now the smallest validated 128-bit-screening row found by the current
pipeline.  The compression bridge is now represented in Lean by:

```text
Advantage.AbstractProbability.map
Advantage.DistinguishingGame.map
Advantage.DistinguishingGame.advantage_map_eq
Advantage.DistinguishingGame.secureAgainst_map_of_source_secure
P_TND_Decision.withRoundingNoise
P_TND_Decision.withRoundingNoise_noiseSupport_of_bounds
P_TND_Decision.withRoundingNoise_realSearch_success_of_bounds
PTNDCorrectnessCertificate.withRoundingNoise
SampleView.restrictExperiment
SampleView.restrictExperiment_advantage_eq_pullback
SampleView.restrictExperiment_secureAgainst_of_full
```

So the prior hand-wavy compression assumption has been split into two checked
facts: deterministic serialization cannot increase distinguishing advantage,
and correctness may replace `eta` by `eta + roundingEta` when the decoded
sample contributes a bounded rounding vector.  The remaining work for this row
is to instantiate the concrete 12-bit decoder/rounding bound and connect it to
the final KEM serialization code.
