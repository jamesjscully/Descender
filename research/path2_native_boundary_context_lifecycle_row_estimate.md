# Path 2 Native Boundary Context/Lifecycle Row Estimate

Status: first-pass estimate pinned.

Purpose: close the lower-risk context and lifecycle row estimates in
`NativeBoundaryTraceRelationV1`.

## Context Projection Rows

The context projection is:

```text
p_context(b) =
  (
    profile_version_digest,
    verifier_version_digest,
    account_id,
    public_key_identity,
    slot_id,
    account_state_root_before,
    lifecycle_anchor,
    aggregation_context_digest
  )
```

Estimate:

```text
r_context_projection_match = 8
```

Rationale: one equality/constant-binding row for each context component. These
are public projection checks. They must not prove SHA preimages; digest-valued
fields are public constants bound by the native-boundary digest and transcript.

## Lifecycle Slot Rows

The lifecycle slot is derived from:

- `p_context(b).slot_id`;
- `p_context(b).account_state_root_before`;
- `p_context(b).lifecycle_anchor`;
- `p_lifecycle(b)` / live marker;
- opening tuple lifecycle link;
- consume-once namespace consumed by the spend theorem.

Estimate:

```text
r_lifecycle_slot_consistency = 3
```

Candidate decomposition:

| subgroup | rows | meaning |
|---|---:|---|
| `live_marker_link` | 1 | trace live marker equals `p_lifecycle(b)` |
| `lifecycle_anchor_link` | 1 | lifecycle column agrees with context/opening tuple |
| `consume_once_namespace_link` | 1 | consume-once namespace is determined by account/slot/lifecycle context |

## Important Boundary

Account creation does not prove a full state transition. It binds the admitted
slot namespace and live marker. The consume-once state transition is a spend/
state theorem obligation.

So this row group must not include:

- blockchain state-root update verification;
- rollback/reorg policy logic;
- spend-time nullifier checks;
- SHA/Merkle account-state root recomputation.

Those are external lifecycle/state-machine obligations, not hidden
account-creation trace rows.

## Failure Modes

The estimate fails if:

- lifecycle semantics require proving a Merkle state update inside account
  creation;
- consume-once logic depends on hidden representative choices not determined by
  `b`;
- `p_lifecycle(b)` is not enough to bind the live marker;
- account creation and spend disagree on the slot namespace.

If any fail, the problem is not row count; it is a boundary-design error.

## Decision

Keep:

```text
r_context_projection_match = 8
r_lifecycle_slot_consistency = 3
```

conditional on:

```text
account_creation_binds_lifecycle_namespace_only
```

and spend/state machinery proving actual consume-once transition.

## Non-Claims

- This does not prove consume-once semantics.
- This does not implement account-state updates.
- This does not prove lifecycle rollback safety.
