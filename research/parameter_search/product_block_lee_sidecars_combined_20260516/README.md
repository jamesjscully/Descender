# Combined Minimal Product-Block Sidecar Manifest, 2026-05-16

This manifest merges the broad `q <= 65537` sidecar library with the targeted
`q = 131071` high-eta sidecars, then keeps only the cheapest passing sidecar for
each `(q, eta, m1)`.

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin product_block_manifest_minimize -- \
  --input-json research/parameter_search/product_block_lee_sidecars_broad_20260516/manifest.json \
  --input-json research/parameter_search/product_block_lee_sidecars_q131071_20260516/manifest.json \
  --output-json research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json \
  --output-md research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.md
```

The combined minimal manifest has `27` entries over:

```text
q = 12289, 32771, 40961, 65537, 131071
```

The validator can now load exact sidecars through the manifest itself:

```bash
--shared-lee-cert-manifest research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json
```

This avoids coupling validation to a single sidecar directory.
