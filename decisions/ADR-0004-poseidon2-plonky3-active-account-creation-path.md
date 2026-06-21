# ADR-0004: Poseidon2 / Plonky3 Is The Active Account-Creation Path

Status: Accepted
Date: 2026-06-20

## Context

The current implementation path uses Poseidon2 over KoalaBear through Plonky3 for
account-creation proofs. Older q12289-native, SHA-heavy, and projection-carrier
routes remain in the repository as research history.

## Decision

The active account-creation proof path is the Poseidon2 / Plonky3 / KoalaBear
path in:

```text
tools/descender-account-stark
tools/descender-account-verifier
tools/descender-recursion
```

The recursion path should preserve this Poseidon2-friendly design and avoid
reintroducing in-circuit SHA on hot paths unless explicitly approved by a future
ADR.

## Consequences

- Specs and trackers should name Poseidon2/Plonky3 as the active path.
- Old paths should be labeled as archived, negative controls, or fallback
  research.
- The current width-16 Poseidon2 124-bit floor must be stated plainly until a
  hard-128 profile is selected.

## Supersedes

This ADR supersedes implicit SHA-heavy or q12289-native account-creation proof
routes as the active implementation path.

## Superseded By

None.

## Affected Specs / Tracker Items

- `spec/architecture.md`
- `spec/security_model.md`
- `spec/wire_formats.md`
- `TRACKER.md`: `DIR-STARK-001`, `AGG-ROLLUP-001`
