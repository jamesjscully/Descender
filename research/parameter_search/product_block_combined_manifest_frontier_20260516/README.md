# Combined-Manifest Product-Block Frontier, 2026-05-16

This run uses the combined minimal exact sidecar manifest:

```text
--product-block-sidecar-manifest research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json
```

Validation loads the same manifest directly:

```text
--shared-lee-cert-manifest research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json
```

## Validated Results

| profile | rows checked | rows passed | min pk | min ct | representative row |
|---|---:|---:|---:|---:|---|
| descender-128 | 120 | 120 | 1028 B | 228 B | `q=12289, n=48, m0=80, m1=32, k=32, eta=12, rows/group=3` |
| descender-192 | 120 | 120 | 1248 B | 256 B | `q=12289, n=64, m0=96, m1=32, k=64, eta=6, rows/group=4` |
| descender-256 | 120 | 120 | 1568 B | 312 B | min-ct row: `q=12289, n=80, m0=128, m1=32, k=128, eta=5, rows/group=5` |

Adding `q=131071` sidecars did not beat these compact rows in this grid, but
the exact proof-object coverage is now available for higher-q lanes in the
overnight search.
