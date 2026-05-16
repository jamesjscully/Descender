# Canonical PK Lossy/Partial-View Sweep, 2026-05-16

This run tests the currently implemented non-full-view route:

- `proof_strategy = lossy-budget`
- `primal_distance_model = guard-only,combined-proxy`
- `view_coords` set to guard-sized and larger views
- `sample_bits = 0,13,12`
- `pk_seed_bytes = 32`
- exact product-block sidecars from the combined minimal manifest

## Result

No tested row validated.

| profile | rows checked | rows passed | dominant failure |
|---|---:|---:|---|
| descender-128 | 160 | 0 | `view_reduction_certificate_missing` |
| descender-192 | 160 | 0 | `view_reduction_certificate_missing` |
| descender-256 | 160 | 0 | `view_reduction_certificate_missing` |

Interpretation: the accounting can reduce `view_len`, but the current proof
route does not yet certify those compressed views.  The validator is blocking
the intended issue: we need a real view-reduction certificate, not another
search knob.

The next proof target is a certified guard-block/partial-view theorem that
turns the existing `guard-block-lossy-diagonal-needs-certificate` candidate
route into a passing route.
