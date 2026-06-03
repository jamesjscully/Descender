# Path 2 One-Use Successor-State Theorem

Status: internal direct-mode lifecycle theorem slice.

## Statement

For direct single-slot Path 2, an authorization digest

```text
auth = H(DESCENDER_SLOT_AUTHORIZATION_V1, PrevalidatedSlotLeafV1)
```

can appear in at most one accepted spend transition from a valid predecessor
state to a valid successor state, unless the adversary exports
`slot_replay_or_rollback_break`.

## Valid Predecessor State

A predecessor state is valid for direct spend if:

- the leaf was admitted under
  `(leaf.account_id, leaf.account_state_root_before, auth)`;
- the leaf has live status marker `0x01`;
- the consume-once set does not contain `auth`;
- the account state slot named by the checked-core signature is unused;
- the lifecycle policy has not burned, restored, or rolled back the slot into a
  contradictory state.

## Valid Successor State

The successor state is valid if:

- `verify_path2_direct_single_slot_spend_authorization` accepted;
- the decoded checked-core slot id equals `leaf.slot_id`;
- the checked-core signature verified under the Path 2 authorization challenge;
- `auth` was inserted into the consume-once set;
- the account lifecycle marks the slot consumed or otherwise unavailable for
  future signing;
- any later state transition preserves that consumed marker unless a reorg,
  rollback, restore, or local database recovery rule explicitly burns the slot.

## Local Executable Model

`verify_path2_direct_single_slot_spend_authorization` delays mutation until all
public checks have succeeded:

1. public-key identity matches the leaf;
2. leaf/evidence/certificate bindings validate;
3. `auth` is present in the admitted state namespace;
4. checked-core verification succeeds;
5. decoded checked-core slot id equals `leaf.slot_id`;
6. `verify_path2_prevalidated_slot_consume_once` inserts `auth`.

If insertion fails, the verifier rejects with
`Path 2 slot authorization already consumed`.

Executable evidence:

- `path2_prevalidated_slot_consume_once_rejects_reauthorization`;
- `path2_direct_single_slot_spend_composes_admission_signature_and_consume_once`;
- `path2_direct_single_slot_spend_rejects_leaf_slot_rebinding`;
- `lifecycle_manager_verify_and_consume_blocks_replay`;
- `spent_slot_ledger_blocks_stale_wallet_replay`;
- `chain_state_file_stores_verify_record_and_persist_together`.

## Break Boundary

Two accepted spends for the same `auth` imply at least one of:

- the verifier did not use the modeled consume-once set;
- two different canonical leaves produced the same `auth`;
- the admitted state namespace was rolled back or forked after consumption;
- a local lifecycle recovery restored a consumed or revealed slot as live;
- a chain reorg policy accepted both spends as final.

The first case is an implementation bug outside this theorem. The second case
is charged to `qrom_or_domain_separation_break` or a canonical-frame collision.
The remaining cases are charged to `slot_replay_or_rollback_break`.

## Non-Claims

The theorem does not require global consensus finality. It states the local
valid-successor-state property needed by the Path 2 proof; production deployment
must instantiate the reorg/finality policy that decides when predecessor states
are considered final enough for account updates.
