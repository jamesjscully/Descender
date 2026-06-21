# Slot Lifecycle And State Model

Status: ACTIVE_NORMATIVE

This file defines the lifecycle contract for Path 2 prevalidated slots. The
cryptographic theorem can prove one-use authorization only if the chain and the
signer both enforce a compatible state machine.

## Scope

The lifecycle model covers:

- Mode A direct single-slot admission and spend;
- the consensus-visible status of a `slot_authorization_digest`;
- local signer and wallet custody behavior around pending spends, reorgs,
  crashes, backups, and restores;
- the state invariants that Mode B aggregation must preserve.

It does not define validator finality, bridge finality, or rollup settlement by
itself. Each target chain must instantiate those policies through
`spec/deployment_profiles.md` before claiming deployment readiness. The
deployment-facing wallet/HSM persistence, restore-anchor, crash/retry, and
fault-state evidence model lives in `spec/signer_state_profile.md`.

## State Objects

The lifecycle key is:

```text
slot_authorization_digest =
  SHA256("DESCENDER_SLOT_AUTHORIZATION_V1" || PrevalidatedSlotLeafV1)
```

The consensus state must be able to decide, for the account namespace and
state-root scope, whether this digest is:

| State | Meaning |
|---|---|
| `absent` | no admitted live authorization exists for this digest |
| `live` | the digest names an admitted slot eligible for one spend |
| `consumed` | a finalized or currently accepted transition spent this exact digest |
| `burned` | account policy permanently disabled this digest without a spend |
| `recoverable` | target-chain policy marks the slot uncertain after rollback, restore, or disputed finality |

Only `live` is spendable. `absent`, `consumed`, `burned`, and `recoverable`
must reject direct spends.

`recoverable` is intentionally not a permission to reuse the same authorization.
It is a policy hook for account recovery, evidence inspection, or slot
replacement.

## Local Custody States

Wallets, hardware signers, HSMs, and custody systems need a stricter local view
than consensus state. A signer may mark a slot as:

| Local state | Required behavior |
|---|---|
| `unused` | no spend payload has been released for this slot |
| `pending` | a spend payload was released but finality is not yet known |
| `confirmed_consumed` | the target chain finalized or accepted the spend under its policy |
| `locally_burned` | the signer refuses future use regardless of chain state |
| `restore_uncertain` | backup/restore, reorg, crash, or database recovery lost enough context that reuse is unsafe |

Local `pending`, `confirmed_consumed`, `locally_burned`, and
`restore_uncertain` slots must not be reused for a new spend. A target-chain
reorg may make a chain record look live again, but it must not make a signer
release a second spend under the same `slot_authorization_digest`.

## Direct-Mode State Transitions

Mode A has the following normative transitions.

| Operation | Precondition | Consensus transition | Required rejection |
|---|---|---|---|
| Admit slot | digest is `absent`; creation evidence verifies; leaf is live and account-bound | `absent -> live` | reject duplicate admission or mismatched evidence |
| Spend slot | digest is `live`; checked-core spend verifies; account/policy permits spend | `live -> consumed` atomically with accepted spend | reject if digest is not exactly `live` |
| Replay spend | digest is `consumed` | no transition | reject deterministically |
| Burn slot | digest is `live` or `recoverable`; account policy authorizes burn | `live/recoverable -> burned` | reject unauthorized burn |
| Recover account | policy authorizes recovery or rotation | create new admitted slot or mark old slot `burned/recoverable` | never resurrect a consumed digest as live |
| Roll back state | target-chain reorg policy rewinds an accepted transition | chain-specific state update | local signers must mark released slots `pending`, `locally_burned`, or `restore_uncertain` until resolved |

Admission and spend must be atomic with their state updates. A verifier must not
expose an accepted spend result without also recording the consume-once marker in
the same consensus transition.

## Invariants

Every direct-mode deployment must maintain these invariants.

1. A `slot_authorization_digest` can be accepted by at most one finalized valid
   spend transition for a target chain history.
2. A checked-core spend challenge binds the account id, state-root or state
   scope, slot id, lifecycle anchor, public-key identity, message digest, and
   `slot_authorization_digest`.
3. Once a digest reaches `consumed` in a finalized chain history, no later valid
   transition may return that same digest to `live`.
4. Account recovery and key rotation may admit replacement slots, but they must
   not reuse the old digest.
5. Reorg and rollback handling must be explicit. If two histories both finalize
   spends of the same digest, the deployment exports
   `slot_replay_or_rollback_break`.
6. Direct mode must not import Mode B aggregation assumptions for lifecycle
   safety.

## Reorg And Finality Policy

Each target-chain deployment profile must define:

- finality depth or finality source;
- whether mempool-visible or locally signed spends put the signer into
  `pending`;
- what happens when a spend is dropped before finality;
- what happens when a finalized or presumed-final spend is reverted;
- how account recovery, slot burns, and slot refreshes interact with disputed
  state;
- which data a wallet or HSM must persist before returning a spend payload.

The safe default is conservative: after a signer releases a spend payload, the
slot remains unavailable locally until the deployment has positive evidence that
the spend cannot be replayed or that account policy has burned/replaced the
slot.

## Break Mapping

Lifecycle failures map to named breaks rather than silent non-claims.

| Failure | Break class |
|---|---|
| accepted duplicate spend for a consumed digest | `slot_replay_or_rollback_break` |
| admission after challenge generation rewrites the live slot | `postchallenge_slot_creation_or_rewrite_break` |
| accepted spend uses a digest not admitted under the account namespace | `prevalidation_registry_binding_break` |
| two canonical leaves collide into one digest | `qrom_or_domain_separation_break` or canonical-frame collision |
| local signer releases a second spend after rollback/restore | `live_slot_leakage_or_keygen_break` or `slot_replay_or_rollback_break`, depending on deployment boundary |
| account recovery resurrects a consumed digest | `slot_replay_or_rollback_break` |

## Required Evidence

Tracker ownership: `DIR-LIFE-001`, `SPEC-DEPLOY-001`, `SPEC-STATE-001`,
`ADOPT-HW-STATE-001`, `AGG-ORDER-001`, and `AGG-DA-001`.

Before a lifecycle claim can be marked `verified`, the repo needs executable
evidence for:

- admit then spend then replay rejection;
- duplicate admission rejection;
- wrong account, wrong state root, wrong slot id, and stale leaf rejection;
- burn then spend rejection;
- recovery creating a new slot without resurrecting the old digest;
- local pending-state crash and restart behavior;
- backup/restore marking released slots unavailable;
- target-chain reorg behavior for dropped, reverted, and finalized spends;
- aggregate or rollup duplicate-prevention behavior before Mode B adoption.

`research/path2_one_use_successor_state_theorem.md` is the current supporting
research theorem slice for the direct local successor-state property. This file
is the normative product contract.
