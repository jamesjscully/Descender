# ADR-0002: Select Path 2 Prevalidated Slots

Status: Accepted
Date: 2026-06-20

## Context

Spend-time extraction from the old projection-carrier transcript created a
complicated and brittle deployability story. The Path 2 design moves
hidden-source validation to account creation or slot refresh and lets spend-time
authorization consume a live prevalidated slot.

## Decision

The selected active architecture is Path 2 prevalidated one-use slots.

Account creation or refresh validates and commits slot authorization material.
Spend-time verification checks a compact checked-core authorization bound to
`slot_authorization_digest` and consumes the exact live slot.

## Consequences

- Spend-time verification no longer claims to extract the hidden-source witness
  from the spend transcript itself.
- Direct single-slot mode is the first deployable theorem mode.
- Creation evidence may be larger than a spend signature because it is outside
  the recurring spend hot path.
- The old projection-carrier path remains a negative-control and lineage source,
  not the product target.

## Active Supporting Files

- `research/account_creation_prevalidated_slot_plan.md`
- `research/prevalidated_slot_leaf_v1_frame.md`
- `research/account_creation_evidence_v1_spec.md`
- `research/spend_authorization_relation_v1_spec.md`

## Supersedes

This ADR supersedes the implicit spend-time hidden-witness extraction direction
from the old projection-carrier lineage.

## Superseded By

None.

## Affected Specs / Tracker Items

- `spec/architecture.md`
- `spec/lifecycle_model.md`
- `spec/wire_formats.md`
- `TRACKER.md`: `ADR-002`, `DIR-LIFE-001`, `DIR-SPEND-001`
