# Canonical PK Coordinate-Subview Certificate, 2026-05-16

This run tests the new view-reduction proof route:

```text
coordinate-subview-lee-dual-restricted
```

The route is based on the Lean certificate boundary added in
`Descender/EfficientPublicView.lean`:

```text
LeeDualViewRestrictionCertificate.restrictLeeDualCertificate
```

Mathematically, a viewed dual relation is extended by zero into the full
coordinate system.  If the viewed dual map agrees with the full dual map after
extension and the extension preserves centered Lee `L1`, then the full
Lee-dual certificate restricts to the viewed coordinate system.

The validator accepts this route only when:

- `proof_strategy = lossy-budget`
- the row is not full-view
- samples are full-width, i.e. `sample_bits = 0`
- the view contains the whole guard block, so correctness is inherited from the
  guard distance
- a matching product-block Lee sidecar certifies the full lifted dual relation

## Validated Results

| profile | pk | ct | q | n | m0 | m1 | view | eta | route |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| descender-128 | 172 B | 172 B | 12289 | 48 | 80 | 32 | 80 | 12 | `coordinate-subview-lee-dual-restricted` |
| descender-192 | 200 B | 200 B | 12289 | 64 | 96 | 32 | 96 | 6 | `coordinate-subview-lee-dual-restricted` |
| descender-256 | 253 B | 253 B | 65537 | 64 | 104 | 32 | 104 | 10 | `coordinate-subview-lee-dual-restricted` |

Compared to the previous full-view canonical line:

| profile | full-view pk/ct | subview pk/ct | saved |
|---|---:|---:|---:|
| descender-128 | 228 B | 172 B | 56 B |
| descender-192 | 256 B | 200 B | 56 B |
| descender-256 | 312 B | 253 B | 59 B |

Sample compression remains outside this certificate: this route is coordinate
restriction with full-width samples, not quantization.
