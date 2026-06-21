# Signer State, Rollback, And Fault Profile

Status: ACTIVE_NORMATIVE

This file defines the local state, rollback, crash, restore, and fault
requirements for Descender wallets, hardware signers, secure elements, HSMs,
and custody services. It refines the local signer side of
`spec/lifecycle_model.md`; it does not replace the consensus lifecycle state
machine.

The profile exists because one-use authorization is not only a verifier
property. If a wallet restores an old backup, retries after a timeout, loses a
pending release, or lets the host reset a slot cursor, the same authorization
material can be released twice even when the public verifier is otherwise
correct.

## Scope

The signer state profile covers:

- local slot state records and persistence;
- crash windows before, during, and after `authorize_spend`;
- host retry and transport timeout behavior;
- backup/restore and restore-anchor placement;
- reorg, dropped transaction, and finality transitions;
- local burn, recovery, and rotation hooks;
- fixed public state/fault result classes;
- executable evidence required for wallet, secure-element, HSM, and custody
  deployment claims.

Consensus-visible admission, spend, replay, burn, and recovery semantics remain
governed by `spec/lifecycle_model.md` and `spec/consensus_verifier.md`.
Command envelopes and public errors remain governed by `spec/signer_api.md`.
Secret-side leakage and diagnostic behavior remain governed by
`spec/leakage_profile.md`.

## Local State Records

Every signer deployment profile must define a persistent record for each
locally known slot.

| Field | Requirement |
|---|---|
| `profile_id` | deployment profile that owns the record |
| `account_id` | public account namespace or digest |
| `slot_id` | local slot identifier bound into public evidence |
| `slot_authorization_digest` | canonical digest from `spec/lifecycle_model.md` |
| `local_state` | one of the local states below |
| `lifecycle_anchor` | finality, recovery, restore, or monotonic state anchor |
| `release_record` | public digest of any released spend payload, if released |
| `state_sequence` | monotonic local sequence or equivalent rollback-resistant counter |
| `last_chain_observation` | public chain/finality observation used for state transition |
| `fault_quarantine` | whether the slot/account is locked pending recovery |

The storage backend may be a wallet database, secure element object, HSM object,
custody-service quorum record, or chain-observed account state. The deployment
profile must state which backend is authoritative and which records are
exportable.

## Local States

The local signer states are:

| Local state | Required behavior |
|---|---|
| `unused` | no spend payload has been released; `authorize_spend` may proceed if policy and consensus context permit |
| `preparing` | signer is deriving or checking a spend payload; restart must return to `unused` or `fault_quarantine` without release |
| `pending_release` | state is durably marked before spend bytes leave the signer boundary |
| `released_pending_finality` | spend bytes were released; the slot must not authorize again |
| `confirmed_consumed` | target-chain policy says the spend is accepted/final enough |
| `locally_burned` | signer refuses reuse regardless of current chain state |
| `restore_uncertain` | backup/restore, crash, reorg, or database recovery lost enough context that reuse is unsafe |
| `fault_quarantine` | local fault, dependency failure, or storage uncertainty requires recovery or operator review |

Only `unused` may authorize a new spend. Every other state must reject
`authorize_spend` with a fixed public state/fault result class.

## Spend Release Atomicity

`authorize_spend` must be treated as a transactional state update. A deployment
must prove or test one of these release disciplines:

1. **mark-before-release:** persist `pending_release` or
   `released_pending_finality` before spend bytes leave the signer boundary;
2. **atomic enclave release:** use a reviewed hardware/HSM primitive that
   atomically records the release with the output;
3. **custody quorum release:** require quorum state to record the release before
   any signer share or final spend payload is returned;
4. **chain-mediated release:** emit only through a chain or rollup transaction
   primitive that atomically consumes the slot under the target deployment
   profile.

If none of these applies, the signer must fail closed. A host retry, transport
timeout, process crash, or dependency exception must not cause a second spend
release for the same `slot_authorization_digest`.

## Restore Anchors

Restore safety requires a rollback-resistant anchor outside ordinary restorable
wallet state.

Allowed anchor classes:

| Anchor class | Requirement |
|---|---|
| `chain_observed_anchor` | trusted finalized or deployment-defined chain observation |
| `hardware_monotonic_counter` | secure element, HSM, or platform monotonic state not restorable from ordinary backups |
| `custody_quorum_anchor` | quorum-controlled service state with audit trail and rollback policy |
| `account_recovery_root` | policy-controlled recovery or rotation root that can burn or replace uncertain slots |

Forbidden anchor placement:

- storing the only trusted restore anchor inside the same restorable wallet
  backup as the slot cursor;
- trusting a host-supplied snapshot sequence without monotonic validation;
- allowing backup import to move `released_pending_finality`,
  `confirmed_consumed`, `locally_burned`, or `restore_uncertain` back to
  `unused`;
- treating dropped mempool transactions as proof that a released slot is safe to
  reuse.

## Reorg And Finality Handling

Every deployment profile must define signer behavior for:

- local release before mempool broadcast;
- mempool broadcast with no inclusion;
- inclusion before finality;
- inclusion followed by reorg;
- finality followed by disputed or reverted settlement;
- account recovery after uncertain release;
- burn or rotation of released slots.

The conservative default is:

1. released slots remain unavailable locally after mempool broadcast;
2. reorged or dropped released slots move to `restore_uncertain` or
   `locally_burned`, not `unused`;
3. account recovery or rotation may admit replacement slots but must not reuse
   the old digest;
4. finality evidence may move `released_pending_finality` to
   `confirmed_consumed`.

## Fixed Public State/Fault Results

Deployment signer APIs must use fixed public results for state and fault
failures:

| Result | Meaning |
|---|---|
| `reject_slot_not_unused` | slot exists but is not eligible for release |
| `reject_restore_uncertain` | local restore or rollback state is unsafe |
| `reject_pending_release` | prior release is pending or finality is unknown |
| `reject_local_burned` | local or policy burn blocks the slot |
| `reject_fault_quarantine` | local fault requires recovery or review |
| `reject_storage_not_durable` | required persistent state could not be written before release |
| `reject_finality_profile_mismatch` | host supplied finality/reorg evidence is not valid for the deployment profile |

These may map to coarser `spec/signer_api.md` result classes, but deployment
builds must not expose raw database errors, rollback counters, host exception
strings, dependency callback text, or secret-bearing diagnostics.

## Evidence Matrix

Before a signer-state claim can support deployment readiness, the target profile
must provide evidence for:

| Scenario | Required evidence |
|---|---|
| clean admit/spend/confirm | slot moves `unused -> released_pending_finality -> confirmed_consumed` |
| crash before release | no spend bytes are returned, or slot remains unavailable until checked |
| crash after release before host acknowledgement | slot remains unavailable after restart |
| host retry after timeout | retry returns prior public release handle or fixed rejection, never a second spend |
| stale backup restore | released or consumed slot cannot become `unused` |
| restore anchor rollback | stale or co-restored anchor is rejected |
| mempool drop | released slot remains unavailable or moves to recovery/burn |
| chain reorg | released slot remains unavailable until profile-defined recovery/burn |
| storage write failure | no spend bytes are released, or slot enters `fault_quarantine` |
| dependency/callback failure | fixed public rejection with no state rollback |
| account recovery | replacement slots may be admitted without resurrecting consumed/released digests |

## Required Evidence

Tracker ownership: `SPEC-STATE-001`, `ADOPT-HW-STATE-001`, `SPEC-LIFE-001`,
`DIR-LIFE-001`, `SPEC-SIGNER-001`, `ADOPT-HW-API-001`, `SPEC-DEPLOY-001`,
`DIR-REJECT-001`, `DIR-KAT-001`, `SPEC-KAT-001`, `SPEC-LEAKAGE-001`,
`POL-RECOVERY-001`, and `DIR-REVIEW-001`.

Before the signer state/fault surface can be marked `verified`, the repo needs:

- a concrete persistent state schema for at least one deployment profile;
- executable state-machine tests for every scenario in the evidence matrix;
- KATs or regression vectors for public state/fault result classes;
- crash/restart tests around `authorize_spend`;
- stale-backup and restore-anchor negative tests;
- host retry and transport-timeout tests;
- chain reorg/finality profile tests;
- storage failure and dependency/callback failure tests;
- account recovery/rotation tests proving old digests are not resurrected;
- target wallet, secure-element, HSM, or custody review for any claim naming
  that environment.

Until those artifacts exist, `ADOPT-HW-STATE-001` remains below `verified`, and
hardware/custody adoption surfaces remain below `ready`.
