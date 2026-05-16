# Broad Product-Block Lee Sidecar Library, 2026-05-16

Generated with the repair-enabled sidecar sweep:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin product_block_lee_sweep -- \
  --q-values 12289,32771,40961,65537 \
  --eta-values 5,6,8,10,12,16 \
  --m1 32 \
  --rows-per-group-values 3,4,5,6 \
  --trials 12 \
  --local-search-steps 8 \
  --seed 20260516 \
  --output-dir research/parameter_search/product_block_lee_sidecars_broad_20260516
```

The sweep produced `65` passing exact sidecars out of `96` tested points.  The
manifest is the search-facing proof-object library:

```text
research/parameter_search/product_block_lee_sidecars_broad_20260516/manifest.json
```

## Minimum Passing Rows

| q | eta | target l1 | rows/group | used rows |
|---:|---:|---:|---:|---:|
| 12289 | 5 | 1229 | 5 | 80 |
| 12289 | 6 | 1024 | 4 | 64 |
| 12289 | 8 | 768 | 4 | 64 |
| 12289 | 10 | 615 | 4 | 64 |
| 12289 | 12 | 512 | 3 | 48 |
| 12289 | 16 | 384 | 3 | 48 |
| 32771 | 5 | 3277 | 5 | 80 |
| 32771 | 6 | 2731 | 5 | 80 |
| 32771 | 8 | 2049 | 4 | 64 |
| 32771 | 10 | 1639 | 4 | 64 |
| 32771 | 12 | 1366 | 4 | 64 |
| 32771 | 16 | 1025 | 3 | 48 |
| 40961 | 5 | 4096 | 5 | 80 |
| 40961 | 6 | 3414 | 5 | 80 |
| 40961 | 8 | 2560 | 5 | 80 |
| 40961 | 10 | 2048 | 4 | 64 |
| 40961 | 12 | 1707 | 4 | 64 |
| 40961 | 16 | 1280 | 4 | 64 |
| 65537 | 5 | 6554 | 6 | 96 |
| 65537 | 6 | 5462 | 5 | 80 |
| 65537 | 8 | 4096 | 5 | 80 |
| 65537 | 10 | 3277 | 4 | 64 |
| 65537 | 12 | 2731 | 4 | 64 |
| 65537 | 16 | 2048 | 4 | 64 |

The local repair loop improved the useful boundary relative to the initial
random-only library, for example `q=12289, eta=6` and `q=32771, eta=8` now have
`rows/group = 4` exact sidecars.
