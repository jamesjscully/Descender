# Deployment Profiles

Status: ACTIVE_NORMATIVE

This file defines the required shape of a Descender target-chain deployment
profile. A deployment profile is the bridge between the generic authorization
stack and a concrete chain, rollup, bridge, smart-account system, precompile, or
frame-transaction environment.

## Scope

A deployment profile covers:

- chain/application domain and account namespace;
- verifier profile, public result classes, and consensus-client mapping;
- lifecycle, finality, reorg, rollback, burn, and recovery policy;
- state storage and data availability for slot records and aggregate evidence;
- gas, resource, and denial-of-service limits;
- wallet/HSM local-state behavior expected by the deployment;
- signer-state persistence, restore-anchor, crash/retry, and fault policy;
- signer API transport profile and fixed public error mapping;
- entropy-source class and domain-separated stream registry;
- adoption-specific surfaces such as Ethereum ERC-4337, delegated EOAs,
  precompile registries, frame transactions, rollups, bridges, and light
  clients.

It does not change the generic direct-mode theorem or create a deployment
claim by itself. A profile becomes adoption evidence only when the corresponding
tracker rows are `verified` and the profile has executable KATs, state tests,
and review evidence for its stated surface.

## Profile Identity

Every deployment profile must define:

| Field | Requirement |
|---|---|
| `profile_id` | stable identifier for the target deployment |
| `profile_version` | explicit version used in public encodings and tests |
| `chain_or_application_domain` | domain bound into checked-core frames |
| `account_namespace` | account/state namespace for admitted slots |
| `verifier_profile` | supported verifier version, parameter set, and public result classes |
| `lifecycle_profile` | finality, reorg, burn, recovery, and rollback policy |
| `signer_state_profile` | local persistence, restore-anchor, retry, crash, and fault-state policy |
| `policy_profile` | account policy model used by the deployment, if any |
| `data_availability_profile` | where slot, policy, aggregate, and proof records remain available |
| `resource_profile` | deterministic size, gas, memory, proof, and state limits |
| `entropy_profile` | entropy-source class, stream registry, and deterministic-test separation rules |
| `review_profile` | review evidence and residual assumptions allowed for this deployment |

The profile identity must be bound either directly into the public frame, into
the account state root consumed by the frame, or into a domain-separated digest
that the frame already binds. A profile must not rely on off-chain convention
when two consensus clients could otherwise verify the same bytes differently.

## Mode A Profile Requirements

A direct Mode A profile must specify:

- slot admission input format and state update;
- spend input format and state update;
- exact `slot_authorization_digest` lookup key and storage scope;
- finality depth or finality source;
- mempool and pre-finality signer behavior;
- replay, duplicate admission, stale leaf, wrong account, wrong state root, and
  consumed-slot rejection behavior;
- gas or fee accounting for parsing, proof checking, state access, and failure;
- resource limits for public byte length, proof length, recursion depth if any,
  state reads, and state writes;
- wallet/HSM local-state expectations for pending, consumed, burned, and
  restore-uncertain slots.
- signer-state persistence, restore-anchor, crash/retry, and fault-state
  expectations.
- signer API transport binding, chunking, and fixed public result mapping.
- entropy-source class, stream registry, and deterministic test/deployment
  separation.

Mode A must not require Mode B aggregation assumptions. A profile may include
aggregate-compatible fields only when the direct verifier accepts or rejects
without relying on aggregate proof soundness.

## Mode B / Rollup / Bridge Profile Requirements

A Mode B profile must add:

- aggregate statement format;
- canonical ordering and duplicate rejection for slots, signers, accounts, and
  messages;
- data availability rules for any record not directly stored in consensus
  state;
- light-client proof semantics, if any client accepts the aggregate;
- recursion or accumulator proof system and parameter set;
- aggregate consume-once semantics and failure mapping;
- committee set, weights, quorum rule, bitmap/root format, and update policy
  when the profile is for bridge or validator committees;
- settlement/finality assumptions outside the cryptographic theorem.

Mode B claims must remain separate from direct-mode claims. A bridge or
validator profile must not imply finality, slashing, or committee safety unless
those assumptions are stated in the profile and traced to tracker rows.

## Ethereum Profile Families

Ethereum-facing profiles are adoption targets, not claims that Ethereum has
selected Descender or that any draft EIP will ship unchanged. A concrete
Ethereum profile must name one family:

| Family | Required profile content |
|---|---|
| `erc4337_smart_account` | verifier ABI, calldata layout, storage layout, paymaster/bundler assumptions, deterministic rejection cases, and gas report |
| `eip7702_delegated_eoa` | delegation flow, migration state, key rotation, recovery behavior, and how long-term ECDSA reliance is removed |
| `secondary_signature_precompile` | algorithm/profile id, precompile ABI, canonical encodings, gas model, versioning, malformed-frame KATs, and fallback behavior if the registry changes |
| `frame_transaction_validation` | validation-frame payload, account-state access, gas-payment separation, deterministic failure modes, and static-validation constraints if applicable |
| `rollup_aggregate_handle` | aggregate handle, ordered batch statement, DA location, recursion endpoint, bridge/light-client assumptions, and proof-system residual assumptions |

Ethereum profiles should cite `research/ethereum_account_abstraction_positioning.md`
for positioning context. Concrete Ethereum profile requirements live in
`spec/ethereum_profiles.md`; a deployment profile must cite the relevant
Ethereum family from that file before it can support an adoption claim.

## Resource And DoS Policy

Every profile must set deterministic limits before deployment readiness:

- maximum public input bytes;
- maximum proof or aggregate bytes;
- maximum state reads and writes;
- maximum live-slot, policy, or committee records consumed by one transaction;
- maximum verification time or gas under the target execution model;
- public rejection class for exceeding each limit;
- whether failed verification consumes gas, fees, or wallet-local slot state.

Resource limits are public consensus parameters. They must not depend on
secret signer behavior, host timing, local heap availability, or implementation
exceptions.

## Required Evidence

Tracker ownership: `SPEC-DEPLOY-001`, `SPEC-LIFE-001`, `SPEC-VERIFY-001`,
`DIR-LIFE-001`, `DIR-REJECT-001`, `DIR-KAT-001`, `AGG-DA-001`, `AGG-LC-001`,
`AGG-COMMITTEE-001`, `ADOPT-ETH-AA-001`, `ADOPT-ETH-7702-001`,
`ADOPT-ETH-7932-001`, `ADOPT-ETH-8141-001`, `ADOPT-ETH-ROLLUP-001`,
`ADOPT-HW-STATE-001`, `ADOPT-HW-ENT-001`, `SPEC-STATE-001`, and
`SPEC-ETH-001`.

Before a deployment profile can be marked `verified`, the repo needs:

- canonical profile document or object for the target chain family;
- byte-level positive and negative KATs for profile-bound public inputs;
- lifecycle tests for admission, spend, replay, burn, recovery, rollback, and
  restore behavior under the profile;
- deterministic rejection and state-no-mutation tests for every public failure
  class;
- gas/resource measurements or deterministic resource proofs for the target
  execution environment;
- data-availability tests or proofs for any nonlocal slot, policy, aggregate, or
  committee record;
- wallet/HSM state expectations linked to `spec/signer_profile.md`;
- signer state/fault evidence linked to `spec/signer_state_profile.md`;
- signer API transport-profile evidence linked to `spec/signer_api.md`;
- entropy-source and stream-registry evidence linked to
  `spec/entropy_profile.md`;
- Ethereum-specific evidence linked to `spec/ethereum_profiles.md` when the
  deployment profile targets an Ethereum family;
- explicit non-claims and residual assumptions for finality, bridge safety,
  draft EIP stability, external proof systems, and hard-128 security if not
  adopted.

Until those artifacts exist, deployment surfaces remain below `ready` in
`spec/blockchain_requirements.md`.
