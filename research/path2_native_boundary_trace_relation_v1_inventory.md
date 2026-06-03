# Path 2 Native Boundary Trace Relation V1 Inventory

Status: relation inventory pinned; Lean row-class coverage and selected row
floor gates applied; exact row indices pending.

Purpose: make `TraceRelationValid(b, profile, trace)` concrete enough to judge
whether a compact transparent proof family can plausibly supply
`ExtractedTraceRelationKnowledgeSoundness`.

## Relation Name

```text
NativeBoundaryTraceRelationV1
```

This relation is the native-boundary replacement for the older isolated
`R_core_66` target. It includes the 66-row semantic core plus the extra rows
needed to bind the trace to `b`, typed roles, lifecycle/context, and no-export
mask consistency.

## Constraint Classes

Lean now names the row classes:

```text
TraceConstraintClass =
  qFieldRow
  | base128RangeRow
  | maskConsistencyRow
  | roleRestrictionRow
  | openingTupleAccountBindingRow
  | checkedCoreResidueCompatibilityRow
  | lifecycleContextRow
  | proofSystemBookkeepingRow
```

and the row inventory carrier:

```text
TraceRelationProfile
```

Lean now also defines:

```text
TraceRelationCoversNativeBoundary
```

and `SupportsRelationKnowledge(profile)` requires the profile's relation
inventory to contain at least one row in every mandatory class. A proof family
therefore cannot close the active relation-knowledge theorem with only the old
66-row semantic core, or with a profile that omits mask-consistency,
role-restriction, opening-tuple account-binding, checked-core compatibility,
lifecycle/context, or proof-system bookkeeping rows.

For the selected active profile, `UsesSelectedTraceKnowledgeModes(profile)` now
also requires:

```text
TraceRelationMeetsSelectedNativeBoundaryRowFloor(profile.relationProfile)
TraceRelationMeetsSelectedNativeBoundaryClassFloors(profile.relationProfile)
selectedNativeBoundaryTraceRelationRowFloor = 176
```

This is a working lower-bound gate, not an exact final row layout. It prevents
closing the active theorem with a one-row-per-class placeholder relation.

The selected class floors are:

| class | selected floor |
|---|---:|
| `qFieldRow` | 26 |
| `base128RangeRow` | 40 |
| `maskConsistencyRow` | 58 |
| `roleRestrictionRow` | 5 |
| `openingTupleAccountBindingRow` | 1 |
| `checkedCoreResidueCompatibilityRow` | 1 |
| `lifecycleContextRow` | 2 |
| `proofSystemBookkeepingRow` | 2 |

## Row Inventory

| group | class | current rows | status | purpose |
|---|---|---:|---|---|
| `masked_projection_equations` | `qFieldRow` | 6 | inherited | enforce `masked_projection_i = pad_i + projection_i mod q` on hidden semantic residues |
| `digit_reconstruction` | `qFieldRow` | 20 | inherited | reconstruct q12289 residues from base-128 hidden digits |
| `digit_range_checks` | `base128RangeRow` | 40 | inherited, gadget-dependent | prove each base-128 digit is in `[0,127]` without raw public openings |
| `mask_consistency_pad_projection` | `maskConsistencyRow` | at least 18 | lower-bound | bind opened masked values to hidden pad/projection residues and one-time masks |
| `mask_consistency_digits` | `maskConsistencyRow` | at least 40 | lower-bound | bind opened masked digit views to hidden digit columns |
| `role_restriction_packed_witness` | `roleRestrictionRow` | open | binds packed witness columns to `packedWitnessHandle` under `p_section_root(b)` |
| `role_restriction_randomizer` | `roleRestrictionRow` | open | binds randomizer columns or derived randomizer role |
| `role_restriction_pad_source` | `roleRestrictionRow` | open | binds pad-source columns to `padSourceHandle` |
| `role_restriction_opening_tuple` | `roleRestrictionRow` | open | binds opening tuple columns to `openingTupleHandle` |
| `role_restriction_projection_relation` | `roleRestrictionRow` | open | binds projection relation columns to `projectionRelationHandle` |
| `context_projection_match` | `lifecycleContextRow` | open | binds account/key/slot/state/lifecycle/aggregation context to `p_context(b)` |
| `lifecycle_slot_consistency` | `lifecycleContextRow` | open | binds live marker and consume-once namespace to the admitted boundary |
| `opening_tuple_account_binding` | `openingTupleAccountBindingRow` | open | connects opening tuple to account witness, randomizer, lifecycle anchor, and spend-opening semantics |
| `checked_core_residue_compatibility` | `checkedCoreResidueCompatibilityRow` | open | connects carried residues to checked-core spend compatibility |
| `statement_digest_source` | `proofSystemBookkeepingRow` | open | binds canonical statement source to `p_statement(b)` without treating SHA as hidden proof semantics |
| `transcript_schedule_bindings` | `proofSystemBookkeepingRow` | open | binds relation id, proof id, query schedule, role modes, no-export/range/blinding policies |

## Lower-Bound Size Signal

The inherited semantic core has:

```text
6 + 20 + 40 = 66 rows
```

The masked-transform lower bound adds:

```text
18 relation mask rows + 40 digit mask rows = 58 rows
```

So before role restrictions, lifecycle/context, account binding, and proof
bookkeeping, the realistic relation is already at least:

```text
124 rows
```

This is not yet a proof-byte estimate. It is a warning that the old 66-row
planning object is no longer the active relation.

## Relation Knowledge Requirement

`ExtractedTraceRelationKnowledgeSoundness` must show that an accepting proof
extracts a trace satisfying every row group above. It is not enough to check:

- only the 66-row semantic core;
- only sampled row openings;
- only masked public equations;
- only role handles without hidden role columns;
- only transcript byte binding.

## Credibility Assessment

The STARK-like route remains plausible if:

- all open row groups can be represented as ordinary algebraic constraints or
  proof-system metadata constraints;
- hidden columns are never opened raw;
- the proof family can prove relation knowledge for the full row inventory;
- role restriction checks do not require SHA-heavy circuits;
- statement/transcript bindings stay outside hidden proof semantics unless
  represented by proof-friendly handles.

The route becomes unattractive if role restriction or statement binding rows
force generic SHA circuits or if the range/mask rows inflate proof size near
the parked lattice-sigma floor.

First row estimate:
`research/path2_native_boundary_open_row_estimates.md` estimates the active
native-boundary relation at:

```text
160-220 rows, working estimate 176
```

under the rule that statement digest and transcript schedule binding remain
public/QROM/data-availability obligations, not hidden SHA-preimage relation
rows.

## Immediate Next

Pin row counts or symbolic row variables for the open groups:

```text
r_role_restriction_*
r_context_projection
r_lifecycle
r_opening_account_binding
r_checked_core_compat
r_statement_source
r_transcript_schedule
```

Then the byte model can be updated honestly from the 66-row comparator to a
native-boundary relation floor.

## Non-Claims

- This does not prove relation knowledge soundness.
- This does not choose a concrete STARK/IOP.
- This does not update final byte counts.
