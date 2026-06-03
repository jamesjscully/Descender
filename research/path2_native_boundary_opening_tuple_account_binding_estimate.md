# Path 2 Native Boundary Opening Tuple Account-Binding Estimate

Status: first-pass estimate; semantic rows pinned, exact encoding pending.

Purpose: stress-test `r_opening_tuple_account_binding`, the highest-risk open
row group in the current 176-row working estimate.

## Rule

Opening tuple binding must be algebraic. It must not prove the old
`opening_commitment` SHA preimage inside `NativeBoundaryTraceRelationV1`.

The public `opening_tuple_handle` is a typed restriction of
`p_section_root(b)`, not an independent digest witness.

## Required Semantics

The opening tuple must bind:

| semantic item | relation role |
|---|---|
| account witness packed state | same witness used by `packed_witness` role |
| randomizer | same randomizer role or derived randomizer subrole |
| lifecycle anchor | same `p_context(b).lifecycleAnchor` |
| slot id/account namespace | same `p_context(b)` fields |
| pad source link | same pad source used by projection relation |
| spend-opening coordinates | opening data consumed by spend handoff |
| live marker | same `p_lifecycle(b)` |
| no postchallenge dependence | independent of spend message/challenge/authorization digest |

## Candidate Row Decomposition

| subgroup | estimate | rationale |
|---|---:|---|
| `opening_account_witness_link` | 2 | packed witness/opening tuple equality or low-degree link for packed account witness limbs |
| `opening_randomizer_link` | 1 | randomizer handle/subrole equality |
| `opening_lifecycle_anchor_link` | 1 | lifecycle anchor/context equality |
| `opening_slot_account_namespace_link` | 2 | account/key/slot namespace consistency, assuming packed public context columns |
| `opening_pad_source_link` | 2 | opening tuple pad seed/source agrees with pad source role without proving SHA |
| `opening_spend_coordinate_link` | 2 | spend-opening coordinates are bound to the opening tuple role |
| `opening_live_marker_link` | 1 | live marker consistency |
| `opening_no_postchallenge_dependency` | 1 | proof-shape/domain row; no spend challenge/message in creation tuple |

Total:

```text
r_opening_tuple_account_binding = 12
```

This matches the working estimate in
`research/path2_native_boundary_open_row_estimates.md`.

## Why This Is Plausible

All rows above are equality/link rows over already extracted trace columns or
public constants from `b`. None require:

- SHA-256 preimage checks;
- checking the spend verifier;
- opening raw hidden cells to the public verifier;
- recomputing `slot_authorization_digest` inside the hidden relation.

The spend-facing digest remains an outer byte-binding projection of the admitted
boundary.

## Failure Modes

This estimate fails if:

- `opening_commitment` must remain a SHA digest over private opening bytes;
- pad seed digest preimage is required inside the relation;
- spend authorization digest must be recomputed inside the trace;
- randomizer semantics are not representable as an algebraic subrole;
- lifecycle consume-once semantics require external state transition logic
  inside account creation.

Any of those would move the route toward generic circuit ZK or the parked
private proof fallback.

## Decision

Keep:

```text
r_opening_tuple_account_binding = 12
```

for the current algebraic native-boundary estimate.

Mark it `conditional_on_algebraic_opening_handle`: valid only if the
native-boundary route treats `opening_tuple_handle` as a typed role restriction
and keeps SHA outer digests as byte binding / data availability.

## Next Move

Stress-test:

```text
r_checked_core_residue_compatibility = 12
```

The key question is whether checked-core compatibility is just 12 q12289 residue
equalities or whether it forces checked-core verifier logic into account
creation.

## Non-Claims

- This does not assign column indices.
- This does not prove the opening tuple relation.
- This does not implement spend handoff.
