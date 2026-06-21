# ADR-0006: Security-Level Position For The Active Path

Status: Accepted
Date: 2026-06-20

## Context

The active width-16 Poseidon2 path has a generic capacity / digest birthday floor
around 124 bits. This is close to, but below, a hard 128-bit claim.

## Decision

The project may describe the active path as having a conservative practical
124-bit floor, or as meeting stronger bounded-query terms where the relevant
query caps are explicitly stated and reviewed.

The project must not claim hard 128-bit security everywhere while the width-16
Poseidon2 floor remains active.

## Consequences

- The security model must state the 124-bit floor plainly.
- A hard-128 profile requires a future ADR, implementation plan, KAT refresh, and
  review.
- The 124-bit position is not itself a blocker for direct-mode engineering, but
  it is a disclosure and adoption-review item.

## Supersedes

This ADR supersedes any unqualified hard-128 claim for the active width-16
Poseidon2 path.

## Superseded By

None.

## Affected Specs / Tracker Items

- `spec/security_model.md`
- `spec/blockchain_requirements.md`
- `TRACKER.md`: `ADOPT-128-001`, `DIR-QROM-001`
