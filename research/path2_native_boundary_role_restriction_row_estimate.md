# Path 2 Native Boundary Role Restriction Row Estimate

Status: first-pass estimate; depends on proof-family subcommitment API.

Purpose: stress-test the `r_role_restriction_*` symbols in
`NativeBoundaryTraceRelationV1`.

## Rule

Role restrictions should be authenticated by the proof-family commitment layer,
not by SHA preimage rows inside the trace relation.

The native relation should only need low-degree equality/link rows when a role
is derived or cross-linked. The statement:

```text
role_handle_i = Restrict(parent_root, profile, role_i, column_set_i)
```

is primarily a commitment/opening theorem obligation, not a hidden arithmetic
row, if the proof family exposes typed subcommitments.

## Estimates

| symbol | estimate | condition |
|---|---:|---|
| `r_role_restriction_packed_witness` | 2 | one role/profile binding and one packed/source link row |
| `r_role_restriction_randomizer` | 1 | randomizer is derived from or linked to opening tuple |
| `r_role_restriction_pad_source` | 2 | one role/profile binding and one pad-source column-set link |
| `r_role_restriction_opening_tuple` | 2 | one role/profile binding and one opening column-set link |
| `r_role_restriction_projection_relation` | 2 | one role/profile binding and one projection column-set link |

Total:

```text
r_role_restriction_total = 9
```

## Interpretation

These rows are not Merkle authentication paths and not hash checks. They are
trace relation rows asserting that the extracted columns are the columns named
by the role/profile metadata already authenticated by the parent trace proof.

If the proof family cannot authenticate typed role subtraces as metadata, these
numbers are too optimistic.

## Alternative Counts

| mode | total | meaning |
|---|---:|---|
| `typed_subcommitment_api` | 9 | active estimate |
| `explicit_algebraic_link_rows` | 15-25 | if each role needs extra in-trace selector/equality rows |
| `sha_digest_role_handles` | reject | SHA preimages inside hidden relation; use only SHA-heavy fallback |
| `public_linear_role_commitments` | dead | leaks private role vectors at current dimensions |

## Failure Modes

The active estimate fails if:

- role handles are ordinary SHA digests of raw hidden role leaves;
- role subroots are not authenticated under `p_section_root(b)`;
- role id / column set / profile are not bound before query derivation;
- randomizer cannot be derived from opening tuple without changing safety
  semantics;
- public role commitments are needed for verifier checks.

## Decision

Keep:

```text
r_role_restriction_total = 9
```

for the active estimate, conditional on:

```text
typed_subcommitment_api
```

This condition is exactly the commitment-layer side of
`ParentTraceCommitmentExtractionSoundness`.

## Next Move

The remaining row-estimate uncertainties are lower risk:

- `r_context_projection_match`;
- `r_lifecycle_slot_consistency`;
- `r_blinding_noise_bounds`.

Pin those, then update the current row estimate from conditional pieces into a
single scenario table.

## Non-Claims

- This does not instantiate Merkle/STARK subcommitments.
- This does not prove role-handle binding.
- This does not assign concrete column ids.
