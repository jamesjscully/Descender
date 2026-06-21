# Ethereum Adoption Profiles

Status: ACTIVE_NORMATIVE

This file defines Descender's Ethereum-facing adoption profiles. It is
normative for Descender profile artifacts only. It does not claim that Ethereum
has selected Descender, that any draft EIP will ship unchanged, or that a
prototype smart account is equivalent to L1 protocol adoption.

The generic deployment-profile requirements live in
`spec/deployment_profiles.md`. This file specializes those requirements for
Ethereum account-abstraction, delegated-EOA, secondary-signature, frame-
transaction, and rollup contexts.

## Source Snapshot

This snapshot is current as of 2026-06-20 and must be refreshed before using an
Ethereum profile as adoption evidence.

| Source | Current relevance |
|---|---|
| `https://pq.ethereum.org/` | Ethereum's post-quantum execution-layer work is framed around account abstraction, post-quantum signature verification, specialized precompiles, gradual opt-in migration, and later aggregation/proof compression. |
| `https://ethereum.org/roadmap/future-proofing/quantum-resistance/` | Ethereum's account-signature migration is described as account-abstraction based and opt-in, with draft EIP-8141 mentioned as a route for signature agility. |
| `https://eips.ethereum.org/EIPS/eip-4337` | ERC-4337 is final and supplies the no-consensus-change smart-account surface where the account defines signature validation through `validateUserOp`. |
| `https://eips.ethereum.org/EIPS/eip-7702` | EIP-7702 is final and supplies delegated EOA code execution through authorization tuples and delegation indicators. |
| `https://eips.ethereum.org/EIPS/eip-7932` | EIP-7932 is draft work for secondary signature algorithms, a registry, and a `sigrecover` precompile interface. |
| `https://eips.ethereum.org/EIPS/eip-8141` | EIP-8141 is draft frame-transaction work with validation frames and explicit post-quantum migration motivation. |

Descender profiles may target final standards directly. Draft-EIP profiles are
tracking profiles: they must isolate draft assumptions and must be revised or
retired if the upstream EIP changes.

## Common Ethereum Profile Rules

Every Ethereum profile must define:

| Field | Requirement |
|---|---|
| `ethereum_profile_id` | stable profile identifier, e.g. `descender.ethereum.erc4337.direct.v1` |
| `profile_family` | one of `erc4337_smart_account`, `eip7702_delegated_eoa`, `secondary_signature_precompile`, `frame_transaction_validation`, or `rollup_aggregate_handle` |
| `profile_version` | byte-bound profile version used in KATs and public encodings |
| `chain_id` | target chain ID or explicit chain-family rule |
| `account_namespace` | account, contract, or rollup namespace that owns admitted slots |
| `slot_state_root` | root, storage key, or witness handle for live and consumed slots |
| `slot_authorization_digest` | normalized digest checked by the direct Mode A verifier or aggregated by Mode B |
| `verifier_profile` | direct verifier, precompile, frame verifier, or aggregate verifier selected by the profile |
| `failure_mapping` | deterministic public result mapping into smart-account, precompile, frame, or rollup failure semantics |
| `resource_profile` | calldata, storage, gas, proof-size, state-read, and state-write limits |
| `review_profile` | review lanes and residual assumptions specific to the Ethereum surface |

The profile identity must be bound into the checked-core spend through either
the canonical `slot_authorization_digest`, the account state root, or an
explicit domain-separated public frame field. Two Ethereum clients or bundlers
must not be able to interpret the same authorization bytes under different
profile families.

All Ethereum profiles inherit these rules:

- direct Mode A verification must not depend on Mode B aggregation soundness;
- malformed bytes, wrong profile version, wrong chain ID, wrong account,
  stale slot root, consumed slot, duplicate slot, and unsupported profile
  family must be deterministic public failures;
- failed public verification must not mutate slot state, signer-local state, or
  account policy state unless the profile explicitly defines a public burn or
  quarantine transition;
- any wall-clock, timestamp, block-number, or finality rule must be represented
  as public policy input and covered by KATs;
- profile-specific gas and resource limits are consensus or mempool policy
  parameters, never secret signer behavior;
- profile KATs must include positive vectors, malformed-frame vectors,
  wrong-domain vectors, stale/consumed-slot vectors, and state-no-mutation
  rejection vectors.

## ERC-4337 Smart-Account Profile

Profile family: `erc4337_smart_account`

This is the first Ethereum profile to implement because it can be exercised
without a consensus change. Descender authorization is carried by the
`UserOperation.signature` field and interpreted by the account's
`validateUserOp` logic.

The profile must define a canonical `DescenderUserOperationAuthorizationV1`
envelope:

| Field | Requirement |
|---|---|
| `profile_version` | fixed profile byte string for ERC-4337 direct Mode A |
| `chain_id` | target chain ID |
| `entry_point` | EntryPoint address assumed by validation |
| `account` | smart-account address that owns the slot namespace |
| `user_operation_hash` | ERC-4337 `userOpHash` or a profile-defined digest that commits to it |
| `slot_authorization_digest` | digest consumed by the Descender spend relation |
| `slot_record` | canonical live-slot record or authenticated handle |
| `account_state_witness` | storage proof, state root handle, or local storage key needed by the account verifier |
| `checked_core_frame` | canonical direct Mode A checked-core frame bytes |
| `creation_or_refresh_evidence` | account-creation or slot-refresh evidence if the slot is not already admitted |
| `policy_context` | optional policy object handle; empty for a pure single-slot account |

The account ABI must specify how the envelope is parsed inside
`validateUserOp(PackedUserOperation userOp, bytes32 userOpHash,
uint256 missingAccountFunds)` and how public failures are encoded in returned
validation data or reverted simulation results. A helper verifier ABI may be
used, but the account profile must still define the account-level state
transition.

Required ERC-4337 evidence:

- pure-EVM or helper-contract verifier ABI;
- calldata and storage layout for live, pending, consumed, burned, and
  restore-uncertain slots;
- gas report for successful verification and each public failure class;
- bundler simulation behavior and deterministic rejection mapping;
- paymaster assumptions, including whether paymaster validation can depend on
  Descender authorization state;
- positive and negative KATs for envelope parsing, domain binding, stale slots,
  consumed slots, duplicate slots, wrong account, wrong EntryPoint, and wrong
  chain ID;
- state-no-mutation tests for every public rejection class.

Until this evidence exists, the ERC-4337 profile is specified but not adoption
ready.

## EIP-7702 Delegated-EOA Migration Profile

Profile family: `eip7702_delegated_eoa`

EIP-7702 is a migration surface, not a final post-quantum authentication
surface. It still uses secp256k1 authorization to set EOA code. A Descender
profile may use it to move an existing EOA into Descender-verifying account
logic, but long-term asset control must not continue to depend on repeated
ECDSA authorizations.

The migration profile must define these phases:

| Phase | Requirement |
|---|---|
| `delegate` | existing EOA authorizes the Descender account or validator code path |
| `initialize_slots` | delegated code admits initial Descender slot records and account policy state |
| `confirm_control` | account proves that future spend authorization is governed by Descender slots |
| `retire_or_quarantine_ecdsa` | ECDSA authority is removed, time-limited, or quarantined under explicit recovery policy |
| `recover_or_rotate` | guardian, timelock, or rotation policy uses Descender lifecycle anchors rather than long-term ECDSA reuse |

Required EIP-7702 evidence:

- delegation transaction and initialization flow;
- account storage migration rules for live and consumed slots;
- replay and duplicate-delegation behavior;
- rollback and restore behavior if initialization fails after delegation code is
  written;
- key-rotation and recovery semantics linked to `spec/policy_model.md` and
  `spec/signer_state_profile.md`;
- explicit residual risk statement for the one-time ECDSA migration step;
- KATs or integration tests showing that post-migration spends do not require
  reuse of the old ECDSA key.

## Secondary-Signature / Precompile Profile

Profile family: `secondary_signature_precompile`

This profile tracks an EIP-7932-like registry or precompile surface. It is not
allowed to treat Descender as a stateless recoverable signature unless the
selected algorithm profile explicitly defines how account state, slot state,
and consume-once semantics are represented.

A Descender secondary-signature profile must define:

| Field | Requirement |
|---|---|
| `algorithm_id` | assigned or provisional identifier; provisional IDs must not be used in deployment claims |
| `algorithm_profile` | `descender_direct_v1` or another explicit profile, never an unversioned `Descender` label |
| `signature_container` | fixed-size or length-delimited byte container compatible with the registry/precompile rules |
| `public_identity` | public key, account identity, slot namespace, or address-derivation input |
| `signing_data` | exact digest or byte string verified by the algorithm entry |
| `return_semantics` | success, failure, and empty-output behavior mapped to public verifier result classes |
| `malleability_rules` | canonical encoding and rejection rules for equivalent byte strings |

If the registry requires fixed padded signatures, Descender must either define a
fixed maximum direct Mode A container for the selected parameter/profile or
reject the registry surface until variable-length proof material has a safe
standard encoding.

Required precompile evidence:

- algorithm/profile identifier decision;
- canonical byte encoding for public identity and authorization bytes;
- verifier ABI and public return semantics;
- gas model and benchmark method;
- malleability analysis for every encoded field;
- malformed-frame, wrong-domain, unsupported-algorithm, stale-slot,
  consumed-slot, and wrong-account KATs;
- statement that Mode B claims are out of scope unless the aggregate proof
  endpoint is specified separately.

## Frame-Transaction Validation Profile

Profile family: `frame_transaction_validation`

This profile tracks an EIP-8141-like native account-abstraction surface.
Descender validation must be represented as a validation frame over the same
direct Mode A authorization object used by smart accounts, not as a separate
unchecked signature payload.

The frame profile must define:

| Field | Requirement |
|---|---|
| `validation_frame_target` | account, verifier, or precompile target for Descender validation |
| `frame_mode` | validation or `VERIFY`-equivalent mode accepted by the final frame-transaction rules |
| `signature_scheme` | Descender direct profile identifier used in the transaction signature object, if any |
| `message_binding` | canonical transaction hash or explicit digest bound to `slot_authorization_digest` |
| `state_access` | slot/account storage reads permitted during validation |
| `payment_separation` | rule proving gas payment does not add hidden authorization assumptions |
| `static_constraints` | static validation constraints required by the final EIP |
| `failure_mapping` | invalid transaction, invalid frame, validation failure, or verifier failure result |

Required frame-transaction evidence:

- payload mapping from frame transaction fields to
  `slot_authorization_digest`;
- account-state access rules for live, pending, consumed, burned, and
  restore-uncertain slots;
- deterministic failure modes suitable for mempool validation;
- gas-payment separation tests;
- positive and negative KATs for canonical transaction-hash binding, explicit
  digest binding, wrong frame target, wrong frame mode, stale slot, consumed
  slot, and wrong account;
- draft-stability ledger showing which assumptions depend on the final EIP.

## Rollup Aggregate Handle Profile

Profile family: `rollup_aggregate_handle`

This is a Mode B profile. It is for Ethereum rollups, L1 settlement contracts,
bridges, and light clients that consume a compact proof/state handle instead of
verifying every direct authorization on L1.

The aggregate handle must commit to:

| Field | Requirement |
|---|---|
| `aggregate_profile_id` | Mode B Ethereum rollup profile identifier |
| `ordered_batch_statement_hash` | canonical ordered statement over accounts, slots, messages, and state transitions |
| `slot_record_da_commitment` | calldata, blob, external DA, or proof-public-input commitment for slot records |
| `consume_update_root` | root of consumed-slot and account-state updates |
| `duplicate_rejection_root` | commitment to duplicate-slot, duplicate-account, and duplicate-message checks |
| `recursion_verifier_id` | concrete proof system and verifier profile |
| `outer_proof_identity` | whether the outer proof is hash/STARK-based, pairing-based, or another assumption class |
| `settlement_assumptions` | L1, rollup, bridge, or finality assumptions outside the cryptographic theorem |

If the final proof is wrapped in a pairing SNARK, the profile must not claim the
outer aggregate proof is post-quantum. A pairing wrapper may be used as an
interim Ethereum gas-optimization artifact only if the residual assumption is
explicitly named in the review profile and requirements table.

Required rollup evidence:

- aggregate handle byte format;
- ordered batch statement format;
- duplicate-slot, duplicate-account, duplicate-message, and duplicate-consume
  rejection tests;
- data-availability rules for slot records, policy records, and aggregate
  public inputs;
- light-client proof semantics if light clients consume the handle;
- recursion proof system, hash/challenger profile, and verifier KATs;
- bridge/settlement assumption ledger when the profile is used for bridge
  committees or cross-chain authorization.

## Non-Claims

Ethereum profiles must not claim:

- Ethereum has selected Descender;
- a draft EIP will ship unchanged;
- ERC-4337 deployment is equivalent to L1 protocol adoption;
- EIP-7702 removes the one-time ECDSA migration risk by itself;
- Descender is a native BLS replacement for Ethereum consensus;
- a precompile profile automatically provides account lifecycle semantics;
- a pairing-SNARK-wrapped rollup proof is post-quantum at the outer proof layer;
- Mode B aggregation is ready before the aggregate proof system, data
  availability rules, and light-client semantics are specified.

## Required Evidence

Tracker ownership: `SPEC-ETH-001`, `SPEC-DEPLOY-001`, `SPEC-VERIFY-001`,
`SPEC-KAT-001`, `SPEC-LIFE-001`, `SPEC-POLICY-001`, `SPEC-STATE-001`,
`ADOPT-ETH-001`, `ADOPT-ETH-AA-001`, `ADOPT-ETH-7702-001`,
`ADOPT-ETH-7932-001`, `ADOPT-ETH-8141-001`, `ADOPT-ETH-ROLLUP-001`,
`DIR-CC-001`, `DIR-LIFE-001`, `DIR-REJECT-001`, `DIR-KAT-001`,
`AGG-ROLLUP-001`, `AGG-DA-001`, `AGG-LC-001`, `POL-REQ-001`,
`POL-RECOVERY-001`, and `ADOPT-HW-STATE-001`.

Before an Ethereum profile can be marked `verified`, the repo needs:

- concrete profile object or document for the selected family;
- canonical public encodings and KATs for the profile envelope;
- positive and negative verification vectors;
- deterministic rejection and state-no-mutation tests;
- gas/resource measurements for the target execution environment;
- lifecycle, finality, rollback, restore, and recovery tests where relevant;
- profile-specific review lane in `spec/review_package.md`;
- explicit residual-assumption ledger for draft-EIP stability, precompile
  registry behavior, rollup settlement, proof-system assumptions, and any
  one-time ECDSA migration step.
