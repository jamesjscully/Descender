# Path 2 Native Boundary Open Row Estimates

Status: first conservative estimate; not a final byte model.

Purpose: estimate the open row groups in
`NativeBoundaryTraceRelationV1` enough to decide whether the STARK-like /
masked trace route is still plausible.

Rule: these estimates count only algebraic/proof-friendly relation rows. SHA
preimage checks remain rejected from the hidden trace relation unless explicitly
reintroduced as a SHA-heavy fallback.

## Baseline

Already pinned fixed floor:

```text
r_fixed_native_trace_floor = 124
```

This includes:

- 66 inherited semantic rows;
- 58 lower-bound mask-consistency rows.

## Conservative Open-Group Estimates

| symbol | estimate | confidence | rationale |
|---|---:|---|---|
| `r_role_restriction_packed_witness` | 2 | medium | one parent-root/profile/role binding row plus one column-set/handle equality row, assuming role handle is proof-family metadata not SHA-preimage relation |
| `r_role_restriction_randomizer` | 1 | low-medium | derived from opening tuple if possible; count one explicit derivation/equality row |
| `r_role_restriction_pad_source` | 2 | medium | role handle/profile binding plus pad role column-set binding |
| `r_role_restriction_opening_tuple` | 2 | medium | role handle/profile binding plus opening role column-set binding |
| `r_role_restriction_projection_relation` | 2 | medium | role handle/profile binding plus projection role column-set binding |
| `r_context_projection_match` | 8 | medium | one row each for profile/verifier/account/key/slot/state-root/lifecycle/aggregation context as public constants or packed equality checks |
| `r_lifecycle_slot_consistency` | 3 | low-medium | live marker, consume-once namespace, lifecycle/opening tuple consistency |
| `r_opening_tuple_account_binding` | 12 | low | account witness/randomizer/opening semantics; assumes algebraic packing and no SHA preimage |
| `r_checked_core_residue_compatibility` | 12 | medium | one compatibility row for each carried checked-core residue |
| `r_statement_digest_source` | 0 | medium | keep digest construction outside hidden proof semantics; bind canonical statement source by public transcript/data-availability |
| `r_transcript_schedule_bindings` | 0 | medium | handled by QROM ledger and verifier transcript, not hidden trace rows |
| `r_blinding_noise_bounds` | 8 | low | extractor-side bound checks for pad/projection/opening/range auxiliaries; exact gadget pending |

## Estimated Relation Size

Conservative algebraic estimate:

```text
r_open_estimate =
  2 + 1 + 2 + 2 + 2
  + 8 + 3 + 12 + 12
  + 0 + 0 + 8
  = 52

r_native_boundary_trace_v1_estimate =
  124 + 52 = 176 rows
```

## Range

Because several rows are still gadget-dependent, use:

```text
optimistic floor: 160 rows
working estimate: 176 rows
stress estimate: 220 rows
```

The stress estimate allows for:

- nontrivial opening-tuple account binding;
- explicit lifecycle/nullifier rows;
- extra blinding/noise bound rows;
- role restriction rows that cannot be compressed into proof metadata.

## Core Constraint

The STARK-like route remains plausible only if the following stay true:

1. `r_statement_digest_source = 0` inside the hidden relation.
2. `r_transcript_schedule_bindings = 0` inside the hidden relation.
3. role handles are proof-family/native-boundary handles, not SHA preimages.
4. opening tuple semantics are algebraic over q12289/base-128 columns.
5. checked-core compatibility is carried as q12289 residue equality, not as a
   checked-core verifier circuit.

If any of these fail, the row count is no longer the main issue; the route
becomes SHA/generic-circuit dominated.

## Findings

The row expansion is not yet fatal. A 160-220 row native-boundary trace is
materially larger than the old 66-row comparator, but still plausibly smaller
than a generic SHA-heavy account-creation proof.

## Stress-Tested Groups

`research/path2_native_boundary_opening_tuple_account_binding_estimate.md`
keeps:

```text
r_opening_tuple_account_binding = 12
```

but marks it `conditional_on_algebraic_opening_handle`: the estimate is valid
only if `opening_tuple_handle` is a typed role restriction and old SHA-shaped
opening commitments remain outer byte/data-availability binding.

`research/path2_native_boundary_checked_core_compat_estimate.md` keeps:

```text
r_checked_core_residue_compatibility = 12
```

but marks it `conditional_on_residue_boundary_only`: the estimate is valid only
if account creation binds checked-core carried residues and leaves
authorization digest/challenge recomputation to the spend theorem and QROM
ledger.

`research/path2_native_boundary_role_restriction_row_estimate.md` keeps:

```text
r_role_restriction_total = 9
```

for the active estimate, conditional on a proof-family
`typed_subcommitment_api`. If role handles become SHA digests or public linear
commitments, the active estimate is rejected rather than increased.

`research/path2_native_boundary_context_lifecycle_row_estimate.md` keeps:

```text
r_context_projection_match = 8
r_lifecycle_slot_consistency = 3
```

conditional on account creation binding only the lifecycle namespace/live
marker, while spend/state machinery proves consume-once transition.

`research/path2_native_boundary_blinding_noise_row_estimate.md` keeps:

```text
r_blinding_noise_bounds = 8
```

conditional on a compact private bound gadget inside the selected proof family.

The actual blocker remains theorem quality:

```text
Can the proof family provide full trace extraction and relation knowledge for
this 160-220 row hidden relation without raw openings?
```

## Next Move

The row-estimate phase is now pinned enough for proof-family planning. Next,
update the transparent byte model with the 160-220 row range and compare it
against the MPC-in-the-head and lattice/SIS ZK fallback surfaces.

Scenario summary:
`research/path2_native_boundary_row_estimate_scenarios.md` records the active
176-row algebraic typed-subcommitment estimate, the 190-215 expanded algebraic
fallback, and the 220-row stress watch threshold.

## Non-Claims

- This is not a final proof size.
- This does not instantiate range/blinding gadgets.
- This does not prove relation knowledge soundness.
