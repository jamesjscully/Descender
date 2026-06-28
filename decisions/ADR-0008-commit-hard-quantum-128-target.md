# ADR-0008: Commit To Hard Quantum-128 Security Target

Status: Accepted
Date: 2026-06-27

## Context

`ADR-0006` recorded that the active width-16 Poseidon2 path has a ~124-bit
generic capacity/birthday floor and that a hard-128 profile would require a future
ADR, implementation plan, KAT refresh, and review. A 2026-06-27 assumption-boundary
stress test (`research/cryptanalysis/assumption_boundary_stress_test_20260627.md`)
made the gap concrete and post-quantum-relevant: under non-interactive Fiat-Shamir
the full FRI query plus proof-of-work grinding budget is a single Grover search, so
the deployed parameters (56 queries, 16-bit PoW) give only ~2^-64 quantum knowledge
soundness, and the 8-element Merkle/sponge digest gives ~2^-82..-99 quantum
collision (binding). For a stack whose stated purpose is post-quantum authorization
on a blockchain, ~64-bit quantum security defeats the premise.

## Decision

Descender commits to a hard quantum-128 security target for the deployable
standard. The target requires, at minimum:

- raising the FRI parameters so the conservative classical grinding budget is
  ~256 bits (about 120 queries at log-blowup 4, keeping a 16-bit PoW), so the
  Grover-halved quantum knowledge soundness reaches ~128 bits;
- widening the Poseidon2 sponge capacity to c=9 (and the Merkle digest
  accordingly) so the quantum collision/binding floor reaches ~128 bits, replacing
  the width-16 c=8 instance on the deployed path;
- a full KAT/byte-model refresh after the parameter and permutation change;
- re-pricing the FRI-AoK, QROM, and binding terms at the new parameters.

Hard-128 is the deployable bar; intermediate builds may run reduced parameters for
development only and must be labelled non-deployable.

## Consequences

- The honest classical 124-bit floor of `ADR-0006` remains the accurate disclosure
  until the c=9 permutation and the larger FRI parameters are implemented; this ADR
  is the commitment and plan, not a claim that 128 is met.
- Proof size and on-chain verification cost rise (more FRI queries, wider
  permutation); economic viability therefore depends on the Mode B aggregation
  path, which moves onto the deployable critical path.
- `spec/security_model.md`, `spec/blockchain_requirements.md`, and the account-STARK
  config must be updated to the quantum-128 parameters and the new headline.
- External third-party cryptanalysis remains required and is out of scope for this
  ADR; this commitment covers everything except that external analysis.

## Supersedes

None.

## Superseded By

None.

## Affected Specs / Tracker Items

- `spec/security_model.md`
- `spec/blockchain_requirements.md`
- `TRACKER.md`: `ADOPT-128-001`, `DIR-QROM-001`
