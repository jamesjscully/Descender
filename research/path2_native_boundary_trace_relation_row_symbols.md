# Path 2 Native Boundary Trace Relation Row Symbols

Status: symbolic row model pinned; Lean row-class coverage gate applied; numeric estimates pending.

Purpose: replace the obsolete single `66-row` planning number with a symbolic
row model for `NativeBoundaryTraceRelationV1`.

## Fixed Lower-Bound Rows

Inherited semantic core:

```text
r_masked_projection_equations = 6
r_digit_reconstruction = 20
r_digit_range_checks = 40
```

Masked two-view lower bound:

```text
r_mask_consistency_pad_projection = 18
r_mask_consistency_digits = 40
```

Current fixed lower bound:

```text
r_fixed_native_trace_floor =
  6 + 20 + 40 + 18 + 40 = 124
```

Lean gate:
`Descender/Path2NativeBoundary.lean` now requires
`TraceRelationCoversNativeBoundary(profile.relationProfile)` inside
`SupportsRelationKnowledge(profile)`. At minimum, a relation profile must cover
`qFieldRow`, `base128RangeRow`, `maskConsistencyRow`, `roleRestrictionRow`,
`lifecycleContextRow`, and `proofSystemBookkeepingRow`.

## Open Symbolic Rows

| symbol | row group | status |
|---|---|---|
| `r_role_restriction_packed_witness` | packed witness role restriction | open |
| `r_role_restriction_randomizer` | randomizer role restriction or derivation | open |
| `r_role_restriction_pad_source` | pad-source role restriction | open |
| `r_role_restriction_opening_tuple` | opening tuple role restriction | open |
| `r_role_restriction_projection_relation` | projection relation role restriction | open |
| `r_context_projection_match` | public context projection equality | open |
| `r_lifecycle_slot_consistency` | live marker / consume-once namespace | open |
| `r_opening_tuple_account_binding` | account witness / randomizer / opening semantics | open |
| `r_checked_core_residue_compatibility` | checked-core carried residue compatibility | open |
| `r_statement_digest_source` | canonical statement source binding | open |
| `r_transcript_schedule_bindings` | proof-system id / relation id / schedule / policy bindings | open |
| `r_blinding_noise_bounds` | extractor-side blinding/noise bound rows | open |

## Total Symbolic Relation Size

```text
r_native_boundary_trace_v1 =
  124
  + r_role_restriction_packed_witness
  + r_role_restriction_randomizer
  + r_role_restriction_pad_source
  + r_role_restriction_opening_tuple
  + r_role_restriction_projection_relation
  + r_context_projection_match
  + r_lifecycle_slot_consistency
  + r_opening_tuple_account_binding
  + r_checked_core_residue_compatibility
  + r_statement_digest_source
  + r_transcript_schedule_bindings
  + r_blinding_noise_bounds
```

## Interpretation

The old 66-row relation is not wrong, but it is no longer the active proof
object. It was only:

```text
masked projection + digit reconstruction + digit range checks
```

The active relation must additionally prove:

- hidden/public masked-view consistency;
- role restriction binding under the native parent root;
- account/lifecycle/opening semantics;
- checked-core compatibility;
- transcript and policy schedule binding;
- blinding/noise bounds.

## Design Pressure

To keep the transparent route plausible, the open symbols should be kept mostly
linear or low-degree over q12289. Any symbol that hides a SHA-256 preimage check
must be rejected or moved to byte/data-availability binding unless we
deliberately switch back to a SHA-heavy generic proof.

## First Estimate

`research/path2_native_boundary_open_row_estimates.md` gives the first
conservative algebraic estimate:

```text
optimistic floor: 160 rows
working estimate: 176 rows
stress estimate: 220 rows
```

This estimate assumes:

```text
r_statement_digest_source = 0
r_transcript_schedule_bindings = 0
```

inside the hidden relation. Those obligations are handled by public transcript,
QROM/domain separation, and data-availability binding, not SHA preimage rows.

The previously open context/lifecycle and blinding/noise symbols are now
conditioned by:

- `research/path2_native_boundary_context_lifecycle_row_estimate.md`;
- `research/path2_native_boundary_blinding_noise_row_estimate.md`.

The active estimate remains 176 rows.

## Next Move

Estimate each open symbol conservatively using the current native-boundary
column profile. The immediate high-risk symbols are:

```text
r_role_restriction_*
r_opening_tuple_account_binding
r_checked_core_residue_compatibility
```

because these decide whether the relation remains STARK/IOP-friendly or should
move to MPC-in-the-head / lattice-SIS ZK.

## Non-Claims

- This is not a final row count.
- This is not a proof-byte estimate.
- This does not instantiate a range gadget or commitment scheme.
