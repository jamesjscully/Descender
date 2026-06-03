# Path 2 Native Boundary Row Estimate Scenarios

Status: scenario table pinned; not a byte model.

Purpose: summarize current `NativeBoundaryTraceRelationV1` row estimates under
different commitment/relation choices.

## Scenario Summary

| scenario | estimated rows | standing | assumptions |
|---|---:|---|---|
| `active_algebraic_typed_subcommitments` | 176 | active | typed role subcommitment API, algebraic opening tuple, residue-only checked-core boundary, no SHA preimages in hidden relation |
| `expanded_algebraic_links` | 190-215 | viable fallback | role restrictions need explicit algebraic link rows, lifecycle/blinding rows are larger, still no SHA preimages |
| `stress_algebraic_no_sha` | 220 | watch threshold | conservative upper bound before proof-family overhead; still algebraic/low-degree |
| `sha_heavy_hidden_relation` | reject for active path | dead/fallback only | statement/opening/pad/authorization SHA preimages enter hidden relation |
| `public_linear_role_commitments` | reject | dead | leaks private role vectors at current dimensions |

## Active Estimate Composition

```text
fixed_native_trace_floor = 124
role_restriction_total = 9
context_projection_match = 8
lifecycle_slot_consistency = 3
opening_tuple_account_binding = 12
checked_core_residue_compatibility = 12
statement_digest_source = 0
transcript_schedule_bindings = 0
blinding_noise_bounds = 8

total = 176
```

## Conditions For The Active Scenario

The 176-row estimate is valid only if:

- role handles are typed subcommitments / proof-family metadata;
- opening tuple semantics stay algebraic;
- checked-core compatibility is residue-boundary only;
- statement digest and transcript schedule binding are public/QROM obligations;
- blinding/noise bounds use compact extractor-side range/bound rows;
- account creation binds lifecycle namespace/live marker only, while spend/state
  machinery proves consume-once transition;
- no hidden SHA preimage relation is introduced.

## What This Means

The row count itself is not currently the blocker. A 176-row relation is larger
than the old 66-row comparator but still plausible for a compact account-
creation proof.

Byte-model overlay:
`research/path2_transparent_proof_byte_model.md` now applies the 160/176/220-row
native-boundary relation overlay. The working estimate remains below the
16416-byte parked lattice-sigma floor in the planning rows, while the 1024-row
stress overlay is close.
`research/path2_native_boundary_proof_family_overhead_budget.md` pins the
remaining headroom: 7712 bytes at 256-row working, 4912 bytes at 512-row
working, 1856 bytes at 1024-row working, and only 320 bytes at 1024-row stress.

The blocker remains proof-family quality:

```text
full trace extraction + relation knowledge + no-export
```

for a roughly 176-row hidden native-boundary relation.

## Decision Rule

Continue the masked trace/STARK-like route while the active or expanded
algebraic scenarios hold.

Pivot to MPC-in-the-head or lattice/SIS ZK if:

- any SHA-heavy hidden relation row becomes mandatory;
- public raw hidden openings become necessary;
- typed role subcommitments cannot be supplied by the proof family;
- relation rows exceed the stress estimate before proof-system overhead;
- proof-family knowledge soundness cannot be stated for the full trace.

## Next Move

Pin the low-risk remaining estimates:

```text
r_context_projection_match = 8
r_lifecycle_slot_consistency = 3
r_blinding_noise_bounds = 8
```

Then the current row-estimate phase can close and the next proof task becomes
choosing/stating the concrete proof-family knowledge assumption.

Done:
`research/path2_native_boundary_context_lifecycle_row_estimate.md` and
`research/path2_native_boundary_blinding_noise_row_estimate.md` pin these rows.
The row-estimate phase is now ready to feed a transparent byte-model update.
`research/path2_native_boundary_transparent_proof_family_v1.md` packages the
active transparent route as a deployability-facing named assumption with row
and overhead gates.

## Non-Claims

- This does not estimate proof bytes.
- This does not instantiate a proof system.
- This does not prove relation knowledge soundness.
