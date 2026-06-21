# ADR-0001: Pivot To Chain-Native PQ Authorization

Status: Accepted
Date: 2026-06-20

## Context

Descender has accumulated several signature/proof paths, including old
projection-carrier and spend-time hidden-witness extraction lines. The new
project objective is to make Descender the obvious post-quantum authorization
choice for blockchain applications.

Raw PQ signatures do not solve the full blockchain authorization problem:
account state, one-use lifecycle, policy, aggregation, rollups, bridge
committees, light-client compression, and wallet/custody deployability remain
separate burdens.

## Decision

Descender is now organized as a chain-native PQ authorization stack, not a
standalone signature primitive.

The active product direction is:

```text
compact PQ spend authorization
+ prevalidated one-use slots
+ state/lifecycle binding
+ account policy layer
+ proof aggregation and rollup compatibility
```

## Consequences

- The normative spec spine lives in `spec/`.
- Append-only decisions live in `decisions/`.
- The consolidated tracker is `TRACKER.md`.
- Legacy signature-only or projection-carrier paths are preserved as evidence,
  regression tests, or negative controls unless explicitly revived by a future
  ADR.

## Supersedes

This ADR supersedes any implicit project framing that treats the 314-byte
projection-carrier lineage as the current product target.

## Superseded By

None.

## Affected Specs / Tracker Items

- `spec/README.md`
- `spec/architecture.md`
- `spec/blockchain_requirements.md`
- `TRACKER.md`: `ADR-001`, `SPEC-DIR-001`, `PROC-ADR-001`
