# Red-Team Vector 3 — State-Machine / Lifecycle Attack

**Date:** 2026-06-10 · **Target:** `tools/parameter-search/src/production_reference.rs`
account-state transition / consume-once / barrier code.
**Probe harness:** `tools/parameter-search/tests/redteam_state_machine_probe.rs`
(6 attack tests, all RUN — `cargo test --test redteam_state_machine_probe`).

## Verdict

**NO production double-spend / replay / rollback break found.** The consume-once
invariant holds across every production store flow. The dossier claims #5
(`postchallengeSlotCreationOrRewrite =0`) and #7 (`slotReplayOrRollback =0`) are
NOT refuted at the code level — the actual Rust state machine enforces them via a
**persistent spent-slot ledger** that is independent of the lifecycle snapshot.

Two real **weaknesses** (sharp edges, not breaks) were found and are reported below.

## What the production state machine actually does

The two consume-once driver families:

1. **In-memory lifecycle manager** (`ProductionAccountLifecycleManager`,
   `ProductionReviewedFq12StateAssistedAccountLifecycleManager`): the *bare*
   `verify_and_consume` / `verify_expanded_opening_frame_and_consume` methods consult
   **only the in-memory slot `unused` flag** — no ledger.
2. **Store / DB flows** (`verify_consume_record_and_save`,
   `apply_selected_projection_consensus_opening_frame`,
   `ProductionSelectedProjectionTargetChainDbAdapter::apply_opening_frame_and_commit`,
   `ProductionMockBlockchain::finalize_block`): these ALWAYS load a **persistent
   `ProductionSpentSlotLedger`** and call `ledger.require_unspent(public_key, slot_id)`
   plus `require_pad_seed_digest_unspent(pad)` BEFORE consuming. The ledger is a
   **separate file/KV namespace** from the lifecycle snapshot, so reverting the
   snapshot does not revert the ledger.

Defenses verified load-bearing:
- **Replay / double-spend:** `ledger.require_unspent((sha256(pk), slot_id))` +
  global `require_pad_seed_digest_unspent`.
- **Rollback:** `from_snapshot_with_restore_anchor` enforces
  `snapshot.snapshot_sequence >= anchor.minimum_snapshot_sequence`; the anchor is
  updated **monotonically** (`update_monotonic` takes `.max()`). Even with a
  permissive anchor, the persistent ledger catches the re-spend.
- **Post-challenge / commit binding:** `validate_selected_projection_target_chain_commit`
  cross-checks the spent-record `public_key_sha256` and forces
  `trusted_anchor.minimum_snapshot_sequence == next_snapshot.snapshot_sequence`.

## Attacks attempted (all RUN)

| # | Attack | Result | Guard that fired |
|---|---|---|---|
| A | Cross-public-key double-spend (same snapshot+frame under pk_a then pk_b) | REJECTED | global pad-seed-digest uniqueness |
| B | Rollback via no-anchor `from_snapshot` + ledger-free consume | "succeeds" on **non-production** ledger-free API only | n/a (no ledger on that path) |
| C | Snapshot rollback, persistent ledger carried forward (consensus path) | REJECTED | `require_unspent` ("slot is already in spent ledger") |
| D | Two slots sharing one pad-seed-digest | REJECTED | account constructor (pad seed must be unique per account) |
| E | Forged spent-ledger argument (key moved off slot 0, pad stripped) | "succeeds" on the **bare transition fn** that trusts its argument | n/a (trusted input) |
| F | **END-TO-END** DB-adapter double-spend (two `apply_opening_frame_and_commit` of slot 0) | REJECTED | persistent `require_unspent` |

Attack F is the strongest test (full deployment model: trusted chain DB holds
snapshot+ledger+anchor; attacker supplies only the spend tx). The second spend of
the same slot is rejected by the persistent ledger. **Consume-once holds.**

## Weakness 1 — pad-seed-digest is the SOLE cross-public-key replay guard

The spent-ledger key is `(sha256(public_key), slot_id)`. The opening-frame
verifier `verify_hidden_source_fallback_expanded_opening_state_slot_frame` does NOT
bind `public_key` — `public_key` is used *only* to key the ledger. In Attack A the
cross-key double-spend was stopped **not** by any public-key binding but by the
*global* `require_pad_seed_digest_unspent` check (the same pad-seed-digest collided
in the ledger regardless of public key).

Implication: the per-`(pk, slot_id)` ledger key does nothing for cross-key replay;
the entire cross-key defense rests on the global pad-seed uniqueness invariant. If
that invariant were ever weakened (e.g. a future path that omits the pad-seed
check, or accounts that legitimately share a pad-seed-digest), cross-public-key
replay opens up. Recommend binding `public_key` into the opening-frame relation
itself, not only into the ledger key.

## Weakness 2 — bare transition functions trust their snapshot/ledger inputs

- `from_snapshot` (no anchor) and `load_manager` (no anchor) accept *any* older
  snapshot and resurrect consumed slots (Attack B). No production store consumes
  via the resulting ledger-free path, so this is not a production break, but the
  snapshot alone is **not** a replay guard.
- `apply_selected_projection_consensus_opening_frame` trusts its
  `spent_slot_records: Vec<...>` argument verbatim (Attack E): a record whose key is
  moved off the target slot and whose `pad_seed_digest` is `None` silences both
  guards. The production DB adapters load these records from an authenticated KV
  store and `validate_selected_projection_target_chain_commit` re-checks them, so
  this does not lift to the adapter — but the bare function is a footgun for any
  caller that forwards attacker-influenced records.

Both are "trusted-input" sharp edges: the authentication of snapshot/ledger/anchor
is delegated to the storage/consensus layer, and the in-memory manager + bare
consensus fn assume their inputs are authentic.

## Residual for a better-resourced adversary

1. **Concurrency / TOCTOU on the file stores.** The file stores use a lockfile
   (`try_lock` / `create_new` + `Drop`-remove). Worth fuzzing: stale-lock breaking
   (`break_stale_lock`) under a clock-skew attacker, and whether two processes can
   interleave load→consume→save around a crash between the ledger `save_ledger` and
   the lifecycle `save_manager` (non-atomic across the two files — a crash there
   could leave the ledger updated but the snapshot reverted, or vice-versa).
2. **Pad-seed-digest second-preimage / forced collision.** Weakness 1 makes the
   pad-seed-digest the cross-key replay lynchpin. A 256-bit SHA-256 pre-image is out
   of reach, but any path that lets an attacker *choose* a slot's pad-seed-digest to
   equal a victim's would enable a cross-key replay; audit all account-creation
   entry points for attacker control of `pad_seed_digest`.
3. **Cross-shard / per-namespace ledger isolation.** If a deployment shards the
   spent ledger by anything finer than `sha256(public_key)` (e.g. per-chain), the
   same `(pk, slot)` could be unspent in a second shard — examine multi-chain
   deployment wiring for ledger-namespace gaps.
