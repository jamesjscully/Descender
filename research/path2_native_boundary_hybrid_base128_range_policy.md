# Path 2 Native Boundary Hybrid Base-128 Range Policy

Status: range-policy interface pinned; not proved.

Purpose: instantiate the selected `hybridBase128HiddenRoles` norm/range mode at
the column-profile level.

## Policy

```text
RangePolicyId := PATH2_NATIVE_BOUNDARY_HYBRID_BASE128_RANGE_V1
```

This policy applies to hidden role openings only:

- `pad_source`;
- `projection_relation`;
- `opening_tuple`.

It does not digitize the public `masked_projection[6]` quotient.

## Digit Encoding

For each hidden semantic value `x` that participates in SIS binding:

```text
x = d0 + 128*d1
0 <= d0 < 128
0 <= d1 < 128
canonical_q12289(x)
```

The proof must bind the digit columns, reconstruction equation, and
canonicalization predicate to the same role restriction under:

```text
p_section_root(b)
p_role_profile(b)
```

## Role Footprint

| role | semantic values | digit columns | range policy |
|---|---:|---:|---|
| `pad_source` | 6 | 12 | base-128 private digits |
| `projection_relation` | 6 | 12 | base-128 private digits |
| `opening_tuple` | 8 | 16 | base-128 private digits |

Total:

```text
20 semantic values
40 digit columns
```

## Binding Bound

For two accepted digit openings:

```text
|delta_digit| <= 127
```

per digit coordinate.

The SIS/module-SIS sanity probe should use digit-coordinate bounds for these
roles, not raw q12289 residue deltas.

## No-Export Requirement

Digit columns and range witnesses are:

```text
VisibilityClass.rangePrivate
```

or an equivalent private/simulator-only class. They may not be opened raw.

Any query-visible check derived from digits must be:

- masked with a fresh one-time domain; or
- simulated by the proof family; or
- rejected as a no-export violation.

## Reconstruction Checks

The hidden extractor view must satisfy:

```text
pad_i = reconstruct_base128(pad_digits_i)
account_projection_i = reconstruct_base128(projection_digits_i)
opening_tuple_j = reconstruct_base128(opening_digits_j)
masked_projection_i = pad_i + account_projection_i mod q
```

For all active lanes `i : Fin 6` and opening tuple coordinates `j`.

## Rejection / Break Rules

Reject or export `path2_sis_norm_or_range_break` for:

- digit outside `[0, 127]`;
- digit reconstruction mismatch;
- noncanonical q12289 residue;
- modular-wrap ambiguity;
- range witness not bound to the authenticated role columns;
- public query opening of raw digit/range witness;
- profile claiming `hybridBase128HiddenRoles` without the required digit
  columns.

## Byte-Model Handling

This policy is already consistent with the current conservative 194-lane
planning floor:

```text
range/account lane floor = 194
```

Do not lower or raise the byte model until the role-link lane count is
re-evaluated under native-boundary projections.

## Next Move

The SIS/module-SIS sanity probe is pinned for:

```text
selected rows: 64
roles: pad_source, projection_relation, opening_tuple
coordinate bound: 127 for digit coordinates
blinding bound: 2048
digit columns: 12, 12, 16
```

The 48-row `opening_tuple` case is now a rejected comparator under the selected
blinding bound, not an active row band.

## Non-Claims

- This does not prove range-check soundness.
- This does not instantiate a concrete range gadget.
- This does not prove query-opening zero knowledge.
- This does not update bytes.
