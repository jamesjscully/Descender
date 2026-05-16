# Canonical PK Seed and Compression Sweep, 2026-05-16

This run benchmarks the canonical public-key model with explicit seed-size
choices and sample-compression candidates.

Changes tested:

- `--pk-seed-bytes 32,64`
- `--sample-bits 0,13,12,11,10`
- full-view `full-surjective` proof route
- exact product-block sidecars from
  `research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json`

## Result

| profile | best pk | best ct | seed bytes | sample bits | row |
|---|---:|---:|---:|---:|---|
| descender-128 | 228 B | 228 B | 32 | full width | `q=12289, n=48, m0=80, m1=32, eta=12` |
| descender-192 | 256 B | 256 B | 32 | full width | `q=12289, n=64, m0=96, m1=32, eta=6` |
| descender-256 | 312 B | 312 B | 32 | full width | `q=12289, n=80, m0=128, m1=32, eta=5` |

The 32-byte canonical seed is a clean win over the previous 64-byte default.
In these full-view rows, public key and ciphertext now match because:

```text
pk = 32-byte seed + full-view sample
ct = 32-byte tag/seed + full-view sample
```

Sample compression did not survive under `full-surjective`: the proof strategy
requires full-width samples.  Forced compressed-sample searches were rejected
with `exact_strategy_requires_full_width_samples`.

The next size reduction therefore requires a new proof route, not just a
packing change.
