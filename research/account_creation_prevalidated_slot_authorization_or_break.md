# Account-Creation Prevalidated Slot Authorization Or Break

Status: final direct-mode theorem statement for Path 2 proof composition.

## Theorem

`account_creation_prevalidated_slot_authorization_or_break`

For n=32, q=12289, in direct single-slot Path 2 mode, if
`verify_path2_spend_authorization` accepts a spend tuple, then either the spend
consumes a previously admitted prevalidated slot in a valid successor state, or
the adversary exports a named break.

## Accepted Spend Tuple

The accepted tuple contains:

- public key;
- message;
- account id;
- account state root before spend;
- slot id;
- checked-core signature body;
- `PrevalidatedSlotLeafV1`;
- `AccountCreationEvidenceV1`;
- `DirectReplayableRelationCertificateV1`;
- direct admission namespace record;
- consume-once state before and after spend.

## Acceptance Hypotheses

The verifier accepts only if all checks below pass.

1. The leaf is canonical and live.
2. Evidence and certificate payload bindings match the leaf.
3. `slot_authorization_digest` equals
   `SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || PrevalidatedSlotLeafV1_bytes)`.
4. The direct admission namespace contains the exact
   `(account_id, account_state_root_before, slot_authorization_digest)` tuple.
5. The creation verifier accepted the direct replayable relation certificate
   before the spend challenge was derived.
6. The leaf-bound hidden-source statement is
   `CreationHiddenSourceStatementV1`, not the reviewed Fq12
   signature-dependent statement.
7. The checked-core challenge frame binds public key, message, account id,
   account state root, slot id, checked-core witness commitment,
   `slot_authorization_digest`, and proof commitment.
8. The reviewed Fq12 checked-core equation and public local/pad consistency
   checks accept under that Path 2 challenge.
9. The consume-once transition inserts the exact authorization digest into a
   valid successor state.

## Conclusion

At least one of these is true:

1. The spend consumes the exact admitted leaf named by
   `slot_authorization_digest` once, under the same account/state namespace, in
   a valid successor state.
2. The adversary exports one of the named breaks below.

## Named Breaks

| break | meaning |
|---|---|
| `creation_time_hidden_source_soundness_break` | creation admission accepted a direct certificate whose `CreationHiddenSourceRelationV1` is false |
| `prevalidation_leaf_binding_break` | leaf/evidence/certificate bind different account, key, slot, state, opening, pad, statement, lifecycle, or transcript values |
| `prevalidation_registry_binding_break` | spend accepted without an exact admitted `(account_id, state_root, slot_authorization_digest)` namespace record |
| `prevalidation_data_availability_break` | admission/spend accepted without replayable direct certificate data |
| `postchallenge_slot_creation_or_rewrite_break` | leaf, creation statement, or certificate was created or rewritten after the spend challenge/message timing boundary |
| `checked_core_message_binding_break` | checked-core challenge does not bind the accepted message, account, state root, slot, authorization digest, or proof commitment |
| `slot_replay_or_rollback_break` | a spent, revealed, burned, or rollback-ambiguous slot becomes spendable again |
| `qrom_or_domain_separation_break` | a digest/frame collision or domain-confusion event is needed for acceptance |
| `live_slot_leakage_or_keygen_break` | public live-slot metadata violates the charged leakage/keygen assumption surface |

## Proof Composition

| step | uses | conclusion | if false |
|---|---|---|---|
| canonicalize leaf | `PrevalidatedSlotLeafV1` frame | one leaf byte meaning | `prevalidation_leaf_binding_break` or `qrom_or_domain_separation_break` |
| bind evidence/certificate | payload binding checks | evidence and certificate name the same creation object | `prevalidation_leaf_binding_break` |
| replay creation relation | `direct_replayable_relation_certificate_soundness_or_break` | creation relation accepted before spend | `creation_time_hidden_source_soundness_break` or data-availability break |
| enforce timing | no-postchallenge timing game | creation object predates spend challenge | `postchallenge_slot_creation_or_rewrite_break` |
| bind admission namespace | direct state namespace registry | leaf was admitted under same account/state root | `prevalidation_registry_binding_break` |
| verify checked core | Path 2 challenge frame and reviewed Fq12 equation | message and authorization are bound to checked-core acceptance | `checked_core_message_binding_break` |
| consume exact slot | consume-once transition | exact authorization is consumed once | `slot_replay_or_rollback_break` |
| charge public metadata | live-slot leakage ledger | public Path 2 metadata is in the leakage model | `live_slot_leakage_or_keygen_break` |

## Direct Mode

This theorem is for direct single-slot admission. It does not consume optional
aggregation assumptions. Direct mode validates the concrete leaf/evidence/
certificate tuple and direct namespace record.

## Batch/Aggregation Mode

Batch or aggregated admission is a separate theorem mode. It must additionally
consume `optional_aggregation_soundness`, canonical batch ordering, duplicate
rejection, inclusion/data-availability binding, and direct-vs-batch leaf
equivalence. None of those assumptions are required for the direct theorem.

## External Review Boundary

This theorem statement is an internal proof target. It does not by itself claim
deployment readiness. Cryptographic hardness, QROM budget, side-channel,
constant-time, lifecycle integration, and external review gates remain separate
deployment boundaries.
