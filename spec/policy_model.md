# Policy Model

Status: ACTIVE_NORMATIVE

This file defines Descender's account-policy layer. Policy is a chain/state
object over admitted Descender authorizations; it is not hidden inside a
signature byte string, and it is not native Schnorr/BLS algebra.

## Scope

The policy model covers:

- M-of-N account authorization over admitted slots;
- threshold custody routing;
- validator or bridge committee authorization;
- guardian, timelock, rotation, burn, and recovery behavior;
- adaptor-style workflows expressed as state/protocol conditions around slot
  consumption;
- the state roots and policy commitments that checked-core spends or aggregate
  proofs must bind.

It does not define a native aggregate signature, a native threshold signature,
or a Schnorr-style adaptor signature. Those require separate ADRs and proof
surfaces before they can become product claims.

## Policy Object Boundary

A Descender policy is identified by a canonical policy commitment:

```text
policy_commitment =
  H(policy_domain || policy_version || account_namespace || policy_body)
```

The hash `H` and the byte encoding remain open until the relevant wire-format
row is promoted. The commitment must be bound into the account state or into an
aggregate public input before it can affect authorization.

Every policy object must specify:

| Field | Requirement |
|---|---|
| `policy_domain` | versioned domain separator for the policy family |
| `policy_version` | explicit version used by consensus and wallets |
| `account_namespace` | chain/application account scope |
| `policy_body` | canonical policy rule data |
| `lifecycle_anchor` | state root or lifecycle anchor consumed by spend checks |
| `slot_class` | optional class restricting which admitted slots may satisfy the policy |
| `failure_class` | deterministic public rejection class |

Policy evaluation must not inspect hidden witnesses, pad seeds, projection
masks, sampler randomness, or other signer-internal material.

## M-of-N Policy

An M-of-N policy admits a spend only when at least `M` eligible authorizations
from a signer set of size `N` are accepted under the same account namespace,
message digest, state context, and lifecycle transition.

The concrete object must define:

- signer-set representation, such as sorted public-key digests or a state-root
  commitment;
- duplicate prevention for signer identity, slot id, and
  `slot_authorization_digest`;
- eligibility rules for slot classes and lifecycle state;
- ordering rules for direct verification or aggregate proof inputs;
- deterministic rejection for malformed sets, duplicates, stale slots, and
  insufficient threshold weight.

Direct Mode A may implement this as repeated independent admitted
authorizations plus account-state policy. Mode B may compress the same facts
behind an aggregate or recursive proof, but it must preserve the same duplicate
and lifecycle semantics.

## Threshold Custody Route

Threshold custody has two acceptable routes until a later ADR chooses one:

| Route | Meaning |
|---|---|
| true threshold signing | a reviewed threshold protocol produces one Descender authorization while preserving signer-side safety |
| proof-compressed M-of-N | independent admitted authorizations are checked or proven under a policy/committee state root |

The project must not claim threshold custody from the base direct-mode
signature alone. Any true threshold route must specify setup, share refresh,
abort behavior, fault handling, leakage evidence, and recovery semantics. Any
proof-compressed route must specify data availability, duplicate prevention,
aggregate statement format, and light-client semantics if light clients consume
the result.

## Committee And Quorum Policy

Validator and bridge committees are weighted policy objects. A committee policy
must define:

- committee id and epoch or update root;
- signer identity representation;
- weights and quorum rule;
- bitmap, Merkle, or proof input format for participating signers;
- slashing, dispute, or replacement hooks if the target chain needs them;
- data availability rules for signer-set and slot evidence;
- finality or settlement assumptions outside the cryptographic theorem.

A committee proof is a Mode B surface unless every signer authorization remains
directly visible to consensus. Direct mode must not inherit bridge or validator
finality assumptions.

## Recovery And Rotation

Recovery and rotation policies may admit replacement slots, burn old slots, or
mark uncertain slots as recoverable. They must never resurrect a consumed
`slot_authorization_digest` as live.

Each recovery policy must define:

- guardian set representation;
- timelock and delay semantics;
- rotation target and new-slot admission rule;
- burn or recoverable-state transition for old slots;
- backup/restore behavior for local signer state;
- deterministic rejection if recovery conflicts with consumed-slot history.

Recovery policy must compose with `spec/lifecycle_model.md`: a chain reorg or
wallet restore may force account recovery, but it must not authorize a second
spend with the same released slot.

## Adaptor-Style Workflows

Adaptor-style workflows are represented as policy/state conditions around slot
consumption. Descender may support conditional release, escrow, atomic-swap, or
state-channel-style flows only when the condition is visible in account state,
message context, or aggregate proof inputs.

The project must not claim native Schnorr-style adaptor algebra. A workflow that
needs adaptor-like behavior must specify:

- the public condition being checked;
- when a slot becomes pending, consumed, burned, or recoverable;
- what secret, if any, is intentionally revealed by completion;
- how replay and duplicate-consumption are prevented;
- deterministic failure behavior for partial completion or abort.

## Public / Secret Boundary

Policy objects may contain public commitments, signer-set roots, account
namespaces, slot classes, weights, thresholds, timelocks, and state roots. They
must not contain signer secrets or diagnostic internals.

Allowed public policy inputs:

- account namespace;
- policy commitment and version;
- signer, guardian, or committee public identifiers;
- weights, threshold values, slot classes, and timelocks;
- lifecycle anchors and state roots;
- public aggregate proof inputs;
- fixed public rejection classes.

Forbidden policy inputs:

- secret keys or threshold shares;
- hidden account witnesses;
- pad seeds, projection masks, or sampler randomness;
- private proof openings not explicitly public under the selected proof system;
- secret-dependent retry counts, logs, or diagnostic errors.

## Required Evidence

Tracker ownership: `SPEC-POLICY-001`, `POL-REQ-001`, `POL-MSIG-001`,
`POL-THRESH-001`, `POL-RECOVERY-001`, `POL-ADAPTOR-001`,
`AGG-COMMITTEE-001`, `DIR-LIFE-001`, `DIR-REJECT-001`, `DIR-KAT-001`, and
`ADOPT-HW-STATE-001`.

Before a policy claim can be marked `verified`, the repo needs:

- canonical policy byte formats and domain separators;
- positive and negative KATs for policy parsing and evaluation;
- M-of-N duplicate-prevention tests for signer id, slot id, and
  `slot_authorization_digest`;
- lifecycle KATs for burn, recovery, rotation, pending, consumed, and
  recoverable slots;
- deterministic public error mapping for every policy rejection;
- target-chain profiles for account namespace, state root, timelock, and
  reorg/finality interaction;
- a threshold-custody ADR choosing true threshold signing, proof-compressed
  M-of-N, or both with separate assumptions;
- committee proof semantics before validator or bridge quorum claims;
- adaptor-style workflow specs before any conditional-release claim.

Policy requirements that are not backed by these artifacts remain below
`verified` in `TRACKER.md` and below `ready` in
`spec/blockchain_requirements.md`.
