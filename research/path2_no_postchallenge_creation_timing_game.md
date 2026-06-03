# Path 2 No-Postchallenge Creation Timing Game

Status: proof-game rule for Path 2 spend authorization.

## Rule

A Path 2 slot is spend-authorizable only if its creation leaf was fixed and
admitted before the spend challenge for that slot was derived.

The adversary must not be able to:

- choose or rewrite `PrevalidatedSlotLeafV1` after seeing the spend message;
- choose or rewrite `CreationHiddenSourceStatementV1` after seeing the spend
  message;
- choose or rewrite the direct replay certificate after seeing the spend
  challenge;
- replace `account_state_root_before` with a state root that was not the root
  used for admission;
- consume a slot through an authorization digest not already admitted under the
  exact account/state namespace.

If any of those actions are needed for an accepting spend, the proof exports
`postchallenge_slot_creation_or_rewrite_break`.

## Timing Order

The deployment proof uses this order:

```text
1. account/key/slot namespace is fixed
2. CreationHiddenSourceStatementV1 is computed
3. DirectReplayableRelationCertificateV1 is checked
4. PrevalidatedSlotLeafV1 is committed into account state
5. account_state_root_before / admission root is fixed
6. slot_authorization_digest is derived from the fixed leaf
7. spend message is chosen
8. checked-core proof commitment is fixed
9. Path 2 checked-core challenge is derived
10. checked-core responses/openings are encoded
11. exact slot authorization is consumed once
```

Steps 1 through 6 must precede step 9. Step 7 may happen after admission; that
is the point of Path 2. The spend message is bound by the checked-core challenge
without entering the creation-time hidden-source statement.

## State-Root Timing Lemma

If `verify_path2_checked_core_authorization` accepts for
`account_state_root_before = R`, then the checked-core challenge bytes include
`R`, and direct authorization must find the exact
`(account_id, R, slot_authorization_digest)` tuple in the admitted namespace.

Therefore a spend accepted under a different state root requires one of:

- a checked-core challenge binding break;
- a direct registry/state-namespace binding break;
- a hash/domain collision in `slot_authorization_digest`;
- a lifecycle rollback/reorg break outside the local signature theorem.

## Executable Hooks

Current Rust hooks matching this game:

- `Path2CreationHiddenSourceStatementV1` excludes spend message and signature
  bytes, so the creation statement is fixed before spend-time challenge
  derivation.
- `path2_checked_core_authorization_challenge_frame` binds message,
  `account_state_root_before`, `slot_authorization_digest`, and proof
  commitment.
- `validate_path2_direct_slot_authorization_state_namespace` accepts only an
  admitted `(account_id, account_state_root_before, slot_authorization_digest)`
  tuple.
- `verify_path2_prevalidated_slot_consume_once` rejects a second consumption of
  the same authorization digest in the local consume-once model.

## Open Proof Boundary

The local theorem can charge ordinary reorg/rollback behavior to the lifecycle
model, but it must name the assumption. The remaining composition theorem should
state:

```text
accepted Path 2 spend
  -> admitted prechallenge leaf under the same account/state namespace
  -> exact authorization consumed once in a valid successor state
  or checked-core / registry / lifecycle / hash break
```
