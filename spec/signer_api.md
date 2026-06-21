# Signer Public API And Wallet/HSM Boundary

Status: ACTIVE_NORMATIVE

This file defines the public signer API boundary for wallets, secure elements,
HSMs, and custody services. It turns the signer-safety profile into an
implementable command/result contract without selecting a target transport such
as APDU, JSON-RPC, FFI, or smart-account calldata.

The API exists to keep Descender's signer side auditable. A compact
post-quantum authorization system is not adoption-ready if hosts, wallets, or
custody services can accidentally expose hidden witnesses, retry diagnostics,
rollback-sensitive state, or platform-dependent behavior through their public
interfaces.

## Scope

The API model covers:

- public command classes sent to a signer, wallet, HSM, secure element, or
  custody service;
- public response classes and fixed public errors;
- diagnostic-build separation;
- signer-local lifecycle state around pending, consumed, burned, and uncertain
  slots;
- public API KAT and transport-profile requirements.

It does not define secret-bearing algorithms. Keygen, sampler, witness handling,
constant-time rules, entropy, and fault behavior remain governed by
`spec/signer_profile.md`, `spec/entropy_profile.md`, and
`spec/leakage_profile.md`. Consensus-visible verifier behavior remains governed
by `spec/consensus_verifier.md`, `spec/wire_formats.md`, and
`spec/lifecycle_model.md`; signer-local crash, retry, rollback, restore, and
fault-state behavior is governed by `spec/signer_state_profile.md`.

## API Identity

Every deployment-facing signer API profile must define:

| Field | Requirement |
|---|---|
| `api_domain` | versioned domain separator for signer API messages |
| `api_version` | explicit integer or semantic version |
| `profile_id` | deployment profile or generic direct-mode profile |
| `transport_profile` | APDU, ABI, FFI, RPC, HSM message, or custody-service binding |
| `command_set` | supported command names and result classes |
| `encoding` | canonical request and response byte encoding |
| `resource_limits` | request, response, chunk, storage, and timeout limits |
| `diagnostic_mode` | `disabled` for deployment builds; separate profile for diagnostic builds |

The `api_domain`, `api_version`, and `profile_id` must be bound into every
request and response, either directly in the encoded object or through a
domain-separated digest already covered by the transport profile.

## Public Command Set

The direct Mode A command set is:

| Command | Purpose | Public output |
|---|---|---|
| `get_profile` | report API/profile/version/resource support | profile descriptor and fixed limits |
| `create_or_refresh_slot` | create or refresh one prevalidated slot under a policy and deployment profile | public slot record or fixed public error |
| `export_slot_record` | return the current public record for an existing slot | public slot record and local public slot state |
| `authorize_spend` | produce one checked-core spend authorization for a live slot and message/context | checked-core spend bytes or fixed public error |
| `mark_spend_result` | update local state after accepted, finalized, dropped, reverted, or rejected spend evidence | public local-state result |
| `burn_slot` | make a slot locally unavailable and request/record policy burn where applicable | public burn result |
| `recover_or_rotate` | start account recovery or slot rotation under the active policy | public recovery or rotation handle |

Additional commands require a versioned profile update. A deployment must not
smuggle extra behavior through untyped extension fields.

## Request Envelope

Every command request must have a canonical envelope containing only public or
policy-authorized fields:

| Field | Requirement |
|---|---|
| `api_domain` | matches the selected signer API profile |
| `api_version` | exact supported version |
| `profile_id` | deployment or generic direct-mode profile |
| `command` | one command from the supported command set |
| `request_id` | public anti-confusion identifier; not used as secret randomness |
| `account_id` | public account namespace identifier or digest |
| `slot_authorization_digest` | present for slot-specific commands |
| `message_digest` | present for spend authorization |
| `state_context_digest` | chain/application state context bound by the spend |
| `policy_digest` | public account-policy digest where policy is in scope |
| `lifecycle_anchor` | public lifecycle/finality anchor where required |
| `public_payload` | command-specific public bytes under a fixed schema |

The envelope must not contain hidden witnesses, pad seeds, projection masks,
sampler randomness, private openings, raw callback diagnostics, or retry counts.

## Public Result Classes

Signer APIs must expose only fixed public result classes in deployment builds.

| Class | Meaning | Local state effect |
|---|---|---|
| `ok_profile` | profile query succeeded | none |
| `ok_slot_prepared` | slot creation or refresh produced a public slot record | records slot as `unused` or profile-defined live candidate |
| `ok_slot_record` | public slot record was exported | none |
| `ok_spend_authorization` | spend authorization was released | marks the slot `pending` unless the profile proves a stronger atomic update |
| `ok_slot_marked_consumed` | accepted/finalized spend evidence was recorded locally | `pending -> confirmed_consumed` |
| `ok_slot_burned` | slot is locally unavailable or burned by policy | `unused/pending/restore_uncertain -> locally_burned` where policy permits |
| `ok_recovery_or_rotation_started` | recovery or rotation flow produced a public handle | profile-defined recovery state |
| `reject_malformed_request` | request bytes, length, domain, version, command, or canonical encoding are invalid | none |
| `reject_unsupported_version` | version is recognized but unsupported by this signer profile | none |
| `reject_unsupported_profile` | profile id, transport, or command set is unsupported | none |
| `reject_policy_denied` | public policy state denies the command | none |
| `reject_lifecycle_state` | slot is consumed, burned, pending, restore-uncertain, absent, stale, or otherwise not usable | none |
| `reject_state_context_mismatch` | account, slot, lifecycle anchor, deployment domain, or state context does not match local state | none |
| `reject_entropy_unavailable` | required deployment entropy is unavailable or failed health checks | none; may burn or quarantine according to profile |
| `reject_resource_limit` | public request exceeds deterministic size, chunking, timeout, storage, or rate limits | none |
| `reject_operation_busy` | signer cannot safely interleave the requested operation with an in-flight operation | none |
| `internal_error_fail_closed` | implementation cannot safely complete the command | none; may quarantine affected slots |

These classes may be mapped to coarser transport errors, but the mapping must be
fixed by the transport profile and covered by KATs. Transport errors must not
include hidden witness material, callback strings, internal retry diagnostics,
or platform exception text.

## Transactional State Discipline

`authorize_spend` is a state-changing command from the signer's perspective.
Before returning spend bytes, a deployment signer must either:

1. persistently mark the slot `pending`; or
2. use a reviewed profile-specific mechanism proving that a crash, retry, host
   replay, or transport timeout cannot cause a second spend release.

The host cannot force a pending, consumed, burned, or restore-uncertain slot
back to `unused`. `mark_spend_result` may move a slot from `pending` to
`confirmed_consumed`, `locally_burned`, or `restore_uncertain` according to the
deployment profile. It must not make a released slot reusable merely because a
transaction was dropped or a local database was restored.

Signer-local rollback protection must be tied to `spec/lifecycle_model.md`.
Deployment profiles must state whether they use monotonic counters, secure
storage, account recovery roots, lifecycle anchors, or custody-service quorum
state to prevent unsafe reuse after crash or restore.

## Diagnostic Builds

Deployment builds must expose only the public result classes above.

Diagnostic builds that expose hidden witnesses, retry counts, raw proof
callbacks, trace dumps, or internal error strings must use a separate
`api_domain`, separate `profile_id`, and separate binary or compile-time
configuration. Diagnostic builds must not emit authorizations accepted by a
deployment profile unless a later review explicitly approves that profile.

## Transport Profiles

The generic API model does not choose one wire transport. Each transport profile
must specify:

- canonical request and response encoding;
- byte order and field-size rules;
- APDU, ABI, FFI, RPC, HSM-message, or custody-service command identifiers;
- chunking and transcript-binding rules for large proofs or public records;
- timeout, retry, and idempotency behavior;
- deterministic resource limits and public error mapping;
- whether host-side batching is allowed;
- how public API KATs are represented for the transport.

A transport profile may be target-specific, but it must not change the command
semantics or secret/public boundary.

## Required Evidence

Tracker ownership: `ADOPT-HW-API-001`, `SPEC-SIGNER-001`, `DIR-REJECT-001`,
`DIR-KAT-001`, `SPEC-KAT-001`, `SPEC-VERIFY-001`, `SPEC-DEPLOY-001`,
`DIR-LIFE-001`, `ADOPT-HW-STATE-001`, `ADOPT-HW-ENT-001`, and
`ADOPT-HW-CT-001`.

Before the wallet/HSM API surface can be marked `verified`, the repo needs:

- canonical request and response encodings for at least one deployment
  transport profile;
- public result enum implemented by the active signer API;
- positive and negative signer public API KATs for every command;
- malformed-request, unsupported-version, unsupported-profile, lifecycle,
  policy-denial, entropy-failure, resource-limit, busy, and fail-closed vectors;
- diagnostic-build separation tests proving deployment builds cannot emit hidden
  witnesses or internal diagnostics;
- crash/retry/timeout tests proving `authorize_spend` cannot release two spends
  for one slot;
- restore and rollback tests tied to `spec/lifecycle_model.md`;
- size, chunking, memory, storage, and timeout limits for each transport;
- hardware wallet, secure-element, HSM, or custody-service review evidence for
  any deployment claim naming that environment.

Until those artifacts exist, the API model is only `partial` adoption evidence:
the public boundary is specified, but no target wallet, HSM, or custody
integration is deployment-ready.
