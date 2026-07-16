# ADR-0011: Account-STARK Adaptive QROM Game And Capacity-9 Challenger

Status: Accepted
Date: 2026-07-10

## Context

`ADR-0008` commits the deployable standard to hard quantum-128 and requires the
deployed Poseidon2 path to replace its capacity-8 sponge with capacity 9. The
Merkle/MMCS path has moved to rate 7 / capacity 9, but the account-STARK
Fiat-Shamir `DuplexChallenger` still uses rate 8 / capacity 8.

The account-STARK FRI/QROM ledger also imported `T = 2^32` from the checked-core
multi-target collision ledger and multiplied it into the BCS adaptive-soundness
error. The cited BCS/FRI result instead defines adaptive soundness against one
adversary that may choose its false statement adaptively and is bounded by its
total random-oracle query count `Q`. Treating multiple statements or proof
attempts as one adversary already charges those attempts through the global
query bound. Multiplying the resulting adaptive-soundness error by an account
count again is not part of that theorem.

The exact selected single-leaf substitution has `degree_bits = 2`, FRI domain
`|L0| = 64`, and a published candidate RBR bound at most `2^-218`. With the
selected QROM allowance, the theorem-native adaptive terms are at most
`2^-154` and `2^-165`; adding the unrelated `2^32` account factor would prevent
this bound from establishing q128.

## Decision

The account-STARK deployment theorem uses the adaptive QROM soundness game:

- one adversary may choose account statements and proof attempts adaptively;
- `Q` is a global bound on all random-oracle queries made by that adversary for
  the theorem instance, not a per-account allowance;
- no independent account-count factor `T` is multiplied into the BCS adaptive
  soundness error;
- explicit target counts remain required for primitive collision, commitment
  search, mask-bias, and algebraic-action events whose reductions genuinely
  union-bound independent targets;
- a deployment profile demanding a stronger wall-clock lifetime failure bound
  must specify it as a separate profile and re-parameterize for it rather than
  silently changing the adaptive-soundness theorem.

The deployed account-STARK Fiat-Shamir challenger must move to width-16 rate 7
/ capacity 9. The Merkle/MMCS and Fiat-Shamir sponges may share the same
Poseidon2 permutation while retaining separate framing and transcript roles.
The implementation, recursive verifier mirror, KATs, and proof byte models must
be refreshed together before the profile can be called ready.

The `256` FRI schedule label remains a sizing label only. Closure must use the
published RBR/BCS/QROM error decomposition and must not use an informal
whole-exponent Grover-halving argument.

## Consequences

- The exact single-leaf adaptive arithmetic can be imported without the
  checked-core account-horizon multiplier, while the protocol mapping, QROM
  lift constant, sponge/QRO replacement, extractor composition, and review
  remain open.
- The current rate-8 account-STARK proof format is transitional and cannot
  satisfy the capacity-9 deployment requirement.
- Changing to rate 7 alters every account and spend proof transcript and the
  recursive verifier's extension-challenge squeeze schedule because the
  degree-8 challenge consumes more than one seven-element rate block.
- Direct and recursive proof KATs must be re-pinned only after the rate-7
  transcript mirror is complete and tamper-tested.
- The checked-core `T` ledger is unchanged; this decision only removes its
  accidental reuse in the account-STARK adaptive-soundness term.

## Supersedes

None.

## Superseded By

None.

## Affected Specs / Tracker Items

- `decisions/ADR-0008-commit-hard-quantum-128-target.md`
- `spec/security_model.md`
- `spec/blockchain_requirements.md`
- `TRACKER.md`: `DIR-QROM-001`, `ADOPT-128-001`
