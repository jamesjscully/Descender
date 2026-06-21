# Hidden-Source Projection Design Fork

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This file is supporting design history for why the project moved toward Path 2
> prevalidated slots. It is not an ADR and does not supersede
> `../decisions/ADR-0002-select-path2-prevalidated-slots.md`,
> `../decisions/ADR-0005-archive-legacy-projection-carrier-product-target.md`, or
> `../TRACKER.md`.

The old 308-byte projection-carrier target exposed twelve q12289 projection
lanes in the 56-byte carrier. The historical generalized proof model used 64 hidden-source
projection variables. Exact replay found an inactive-coordinate survivor in that
64-variable space.

Superseding deployment note: the long-term blockchain path is now
account-creation / prevalidated-slot authorization. The active-lane
projection-carrier work remains the executable baseline for creating and
auditing prevalidation records, but the intended spend path no longer tries to
carry all hidden-source proof semantics in each transaction.

## Facts

- Carrier-visible lanes: `12`
- Account-witness lanes: `6`
- Pad-source lanes: `6`
- Historical hidden-source projection variables: `64`
- Inactive variables under fixed carrier: `52`
- Exported survivor: hidden-source projection variable `0`, not selected by the
  carrier lane map
- Current verifier disposition: fail-closed

## Decision

The selected hidden-source relation for current executable evidence remains the
active-lane relation. The selected deployment architecture is Path 2:
account/slot creation validates that relation before a slot becomes live, and
spend-time verification consumes a prevalidated one-use slot.

The wider-carrier option remains a fallback only if later checks refute the
twelve-lane relation at account creation.

## Alternatives Considered

There were two coherent ways forward:

1. Active-lane relation.
   The deployed hidden-source relation is defined only over the twelve
   carrier-visible lanes. The account opening frame carries six selected account
   source values, the pad seed derives six selected pad values, and the verifier
   checks exactly those lanes. This matches the executable byte surface, but it
   is a relation redesign and requires promotion through preflight and theorem
   integration.

2. Wider carrier.
   The deployed relation keeps the broader 64-variable hidden-source projection
   space. The signature/account state must then carry enough public information
   or commitments/openings to bind the inactive variables, which likely means
   changing the selected byte target.

3. Account-creation prevalidation.
   The chain validates the hidden-source relation during slot creation or slot
   refresh and commits a prevalidation handle into account state. Spend-time
   verification checks authorization to consume that slot plus checked-core
   message binding. This is now the selected blockchain direction because it is
   more aggregation-friendly and avoids adding roughly 100 transient proof bytes
   to every spend.

## Rejected Option

Do not accept the current 64-variable proof model with the fixed 12-lane
carrier. The exact replay already found a bounded survivor in an inactive
projection coordinate, so that relation is underconstrained.

Do not pursue simple extra carrier-lane expansion as the compact proof
replacement. The sidecar rank probe shows one added lane buys one linear
constraint, no rank amplification, and no checked-core kernel improvement.

## Active-Lane Evidence

`scripts/research/hidden_source_fallback_projection_active_subspace_exact_replay.py`
regenerates
`research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.{json,md}`.
The replay reports:

- prior survivor in active subspace: `false`
- active projection nullity: `0`
- open replay rows: `0`
- signature byte increase: `0`

`scripts/research/generate_hidden_source_active_subspace_lean.py` regenerates
`Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean`, which proves the
finite active-lane facts in Lean.

## Next Work

Move planning and implementation to the account-creation prevalidation track:

- define the prevalidation leaf/certificate byte frame;
- define the account-creation verifier and fail-closed negative controls;
- define the spend-time slot-authorization verifier;
- update production preflight so the hidden-source blocker is expressed as
  account-creation prevalidation semantics;
- retain the active-lane executable relation as the candidate relation that
  account creation validates.

If later checks fail, the target must change rather than relaxing the verifier.
