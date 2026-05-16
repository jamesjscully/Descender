# Manifest-Driven Product-Block Frontier, 2026-05-16

This frontier run consumes the broad exact sidecar manifest directly:

```text
--product-block-sidecar-manifest research/parameter_search/product_block_lee_sidecars_broad_20260516/manifest.json
```

The validator then loads the same sidecar library:

```text
--shared-lee-cert-dir research/parameter_search/product_block_lee_sidecars_broad_20260516
```

This removes manual `lee_repetition` and `lee_rows_per_group` tuning from the
product-block lane.  The search only emits product-block rows backed by passing
manifest entries.

## Validated Results

| profile | rows checked | rows passed | min pk | min ct | representative row |
|---|---:|---:|---:|---:|---|
| descender-128 | 120 | 120 | 1028 B | 228 B | `q=12289, n=48, m0=80, m1=32, k=32, eta=12, rep=512, rows/group=3` |
| descender-192 | 120 | 120 | 1248 B | 256 B | `q=12289, n=64, m0=96, m1=32, k=64, eta=6, rep=1024, rows/group=4` |
| descender-256 | 120 | 120 | 1568 B | 312 B | `q=12289, n=80, m0=128, m1=32, k=128, eta=5, rep=1229, rows/group=5` |

The 128-bit row is the cleanest demonstration of the structural improvement:
the exact product-block amplitude proof now consumes `48` pivot rows instead of
the earlier `80`, because the manifest supplies a certified
`rows/group = 3` sidecar for `q=12289, eta=12`.
