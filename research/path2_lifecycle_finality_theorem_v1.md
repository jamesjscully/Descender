# Path2LifecycleFinalityTheoremV1

> Classification: `ACTIVE_SUPPORT`. Supports the active Path 2 lifecycle
> (`ADR-0002`, tracker row `DIR-LIFE-001`, normative
> `spec/lifecycle_model.md`). It closes the proof-relevant slice of the
> consume-once / reorg-finality obligation (local consume-once + named profiled
> finality assumption) and routes per-target-chain finality to the deployment
> profile decision. Not normative by itself.

Status: consume-once lifecycle theorem with profiled reorg/finality assumption.

## Obligation and the proof/deployment split

`DIR-LIFE-001` now pins the ADR-0010 D2 finality profile as executable evidence:
records at or below the finalized head survive, while records in the reorg-able
window are freed by `reorg_to(new_head)`. Per the risk ordering,
lifecycle/finality is **lower proof-family-collapse risk** — it becomes
proof-critical only if reorg/replay semantics can invalidate the consume-once
assumption used by the direct theorem. So the obligation factors:

```text
#9 = LOCAL-CONSUME-ONCE (proof-relevant: the direct theorem's "consumed once in a
                          valid successor state" hypothesis holds locally)
   ∧ REORG-FINALITY-PROFILE (deployment decision: per-target-chain finality
                             source/depth and disputed-state policy)
```

This document closes the first at paper level (composing the existing
successor-state theorem) and identifies the second as a per-profile deployment
decision (`SPEC-DEPLOY-001`). The D2 finalized-head rule is now instantiated by
`Path2D2FinalityProfileState` and the
`path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized`
regression, but the full lifecycle transition profile is not yet verified.

## Lemma C — Local consume-once

> **Lemma C.** In the local successor-state model, an authorization digest
> admitted under `(account_id, account_state_root_before, slot_authorization_digest)`
> can be consumed at most once: `verify_path2_prevalidated_slot_consume_once`
> rejects any second consumption of the same digest, transitioning the slot to a
> valid spent successor state. A second accepted spend for a consumed digest
> exports `slot_replay_or_rollback_break`.

**Proof.** This is the content of
`research/path2_one_use_successor_state_theorem.md` and the
`Path2DirectLifecycleBoundaryReportV1` local model (consume-once validator,
spent-slot ledger, named replay/persistence tests): the spent-slot ledger records
the consumed digest, and the successor-state validator requires the digest be
absent from the spent set before accepting and present after — so a replay finds
it present and rejects. ∎

## Theorem (lifecycle composition with profiled finality)

> **Theorem.** An accepted direct Path 2 spend consumes the exact admitted
> authorization once in a valid successor state satisfying `spec/lifecycle_model.md`,
> OR exports a named lifecycle break
> (`slot_replay_or_rollback_break`,
> `postchallenge_slot_creation_or_rewrite_break`,
> `prevalidation_registry_binding_break`), under the deployment profile's
> finality assumption `A_finality(profile)`.

**Proof.** By Lemma C the consumption is once-only in the local model. By the
no-postchallenge timing theorem (#7, Lemma T) the admission preceded the
challenge under the exact namespace, excluding
`postchallenge_slot_creation_or_rewrite_break` and
`prevalidation_registry_binding_break` except as named breaks. The remaining
failure mode is a chain-level reorg/rollback that reverts a presumed-final spend
or resurrects a consumed digest; this is exactly `A_finality(profile)` —
discharged by the deployment profile, not by the local signature theorem. ∎

So the direct theorem's "consumed once in a valid successor state" hypothesis
holds locally and unconditionally; the only added assumption is the named,
explicit `A_finality(profile)` — a charged assumption, not a silent gap.

## `A_finality(profile)` — the per-target-chain deployment decision

Per `spec/lifecycle_model.md` § Reorg And Finality Policy, each deployment
profile MUST define:

- finality depth or finality source;
- whether mempool-visible / locally-signed spends move the signer to `pending`;
- what happens when a spend is dropped before finality;
- what happens when a finalized / presumed-final spend is reverted;
- how recovery, slot burns, and slot refreshes interact with disputed state;
- which data a wallet/HSM must persist before returning a spend payload.

**Safe default (baseline `A_finality`):** after a signer releases a spend
payload, the slot stays locally unavailable until the deployment has positive
evidence the spend cannot be replayed or that account policy has burned/replaced
the slot. This conservative default is sound for any chain (it never re-enables a
slot without finality evidence); a profile may relax it only with an explicit
finality source. This is a product/deployment decision (`SPEC-DEPLOY-001`), not a
theorem to discharge here.

## Adversarial Self-Check

- **Replay a consumed digest.** Rejected by Lemma C (spent-slot ledger); a second
  accept is `slot_replay_or_rollback_break`.
- **Reorg reverts a final spend.** This is `A_finality(profile)` — under the safe
  default the slot was never re-enabled without finality evidence, so no second
  spend was released. A profile that relaxes the default owns the residual.
- **Recovery resurrects a consumed digest.** `slot_replay_or_rollback_break` per
  the break mapping; account recovery must not resurrect consumed digests.
- **Honest residual.** (1) Target-chain/deployment profile review tying the
  digest-pinned local transition corpus to an adopter profile; the boundary now
  pins admit→spend→replay→burn→recovery→restore/D2 local vectors, but not a
  deployment-reviewed profile. (2) Deployment profile review for any chain that
  relaxes the D2 safe default. (3) External review.
  `Path2DirectLifecycleBoundaryReportV1` sets
  `target_chain_finality_profile_instantiated = true` for the D2 finalized-head
  rule, while keeping `lifecycle_transition_profile_verified` and
  `deployment_ready` false.

## What This Closes And Hands Off

- **Closes (proof-relevant):** local consume-once (Lemma C) + the lifecycle
  composition charging reorg/finality as a named profiled assumption, so the
  direct theorem's consume-once hypothesis carries no hidden proof gap — the
  `R_namespace`/lifecycle factor of `R_create` reduces to `A_finality(profile)`.
- **Hands off (deployment decision):** target-chain lifecycle profile review
  (`SPEC-DEPLOY-001`), including recovery/restore policy review; external
  review.
