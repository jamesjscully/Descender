# ADR-0003: Direct Mode Before Aggregation

Status: Accepted
Date: 2026-06-20

## Context

Aggregation, rollup compression, and committee proofs are central to the long-term
blockchain objective. They are also additional theorem modes with their own proof
system, data availability, ordering, and duplicate-prevention assumptions.

## Decision

Direct single-slot deployment is the first target. It must not consume
`optional_aggregation_soundness`.

Aggregation mode is reserved and designed for, but inactive until a concrete
accumulator, batch proof, rollup proof, or recursive STARK mode is selected and
reviewed.

## Consequences

- Direct mode remains a complete baseline.
- Aggregation-ready fields such as `aggregation_mode` and
  `slot_authorization_digest` are allowed in the direct-mode wire story.
- Enabling batch or aggregate handles requires a future ADR and tracker items.

## Supersedes

None.

## Superseded By

None.

## Affected Specs / Tracker Items

- `spec/architecture.md`
- `spec/blockchain_requirements.md`
- `spec/wire_formats.md`
- `TRACKER.md`: `AGG-MODE-001`, `AGG-ORDER-001`, `AGG-DA-001`
