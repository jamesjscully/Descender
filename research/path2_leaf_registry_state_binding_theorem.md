# Path 2 Leaf, Registry, And State Binding Theorem

Status: internal direct-mode theorem slice for account-creation prevalidated
slot deployment.

## Statement

For direct single-slot Path 2, if a spend verifier accepts a tuple

```text
leaf,
account_creation_evidence,
direct_replayable_relation_certificate,
public_key,
message,
account_state,
checked_core_signature,
admitted_state_authorizations,
consumed_authorizations
```

through `verify_path2_direct_single_slot_spend_authorization`, then one of the
following holds:

1. the accepted spend names exactly one canonical `PrevalidatedSlotLeafV1`;
2. the evidence and certificate duplicate the same account id, public-key
   identity, slot id, verifier version, account state root, opening
   commitment, pad-seed digest, hidden-source statement digest, relation
   transcript, and lifecycle anchor as that leaf;
3. the leaf's `slot_authorization_digest` was admitted under the exact
   `(account_id, account_state_root_before, slot_authorization_digest)`
   namespace used by the checked-core challenge;
4. the direct certificate payload was available to the validator and matched
   the evidence payload digest;
5. no duplicate `(account_id, slot_id)` or duplicate `pad_seed_digest` was
   accepted in the admission batch modeled by
   `verify_path2_prevalidated_slot_admission_namespace`;
6. the checked-core signature's decoded slot id equals the leaf slot id before
   any consume-once mutation;
7. the exact authorization digest is inserted into the consume-once set once;

or the adversary exports one of:

- `prevalidation_leaf_binding_break`;
- `prevalidation_registry_binding_break`;
- `prevalidation_data_availability_break`;
- `duplicate_prevalidation_admission_break`;
- `checked_core_message_binding_break`;
- `slot_replay_or_rollback_break`;
- `qrom_or_domain_separation_break`.

## Proof Boundary

The theorem is a composition of deterministic verifier checks and named hash or
state assumptions.

### Leaf Binding

`verify_path2_prevalidated_slot_creation_frame_bindings` recomputes:

- `slot_leaf_digest = H(PrevalidatedSlotLeafV1)`;
- `slot_authorization_digest = H(DESCENDER_SLOT_AUTHORIZATION_V1, leaf)`;
- `prevalidation_transcript_digest = H(Path2AccountCreationEvidenceV1)`.

It rejects if evidence disagrees with the leaf on account id, public-key
identity, slot id, verifier version, state root, opening commitment, pad-seed
digest, hidden-source statement digest, lifecycle anchor, leaf digest,
authorization digest, or transcript digest. Therefore two accepted meanings for
one leaf require equal duplicated fields or a collision/domain-separation break.

Executable evidence:

- `path2_prevalidated_slot_leaf_v1_frame_is_canonical`;
- `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations`;
- `path2_prevalidation_transcript_digest_is_not_circular`;
- `path2_creation_hidden_source_statement_v1_is_canonical_and_non_circular`.

### Certificate Payload Availability

`verify_path2_prevalidated_slot_creation_payload_bindings` is fail-closed if
the direct replayable certificate is absent. If present, its canonical digest
must equal `evidence.evidence_payload_digest`, and the certificate must duplicate
the leaf/evidence fields listed above.

Executable evidence:

- `path2_prevalidated_slot_creation_payload_kats_reject_missing_or_replayed_payloads`;
- `path2_public_certificate_debug_excludes_private_opening_material`;
- `path2_public_admission_view_serializes_only_public_material`.

### Registry Binding

`validate_path2_direct_slot_authorization_state_namespace` validates the leaf,
evidence, and certificate payload, recomputes `slot_authorization_digest`, and
requires membership of the exact triple:

```text
(leaf.account_id, leaf.account_state_root_before, slot_authorization_digest)
```

in the admitted state-authorization set. A spend cannot move an authorization to
a different account or state root without rejection or a registry/hash break.

Executable evidence:

- `path2_direct_authorization_requires_exact_admitted_leaf`;
- `path2_direct_authorization_requires_state_namespace_admission`;
- `path2_direct_single_slot_spend_composes_admission_signature_and_consume_once`.

### Duplicate Prevention

`verify_path2_prevalidated_slot_admission_namespace` rejects duplicate
`(account_id, slot_id)` and duplicate `pad_seed_digest` values in the modeled
admission batch. The theorem assumes production admission uses this uniqueness
domain, or else any duplicate accepted outside it is charged to
`duplicate_prevalidation_admission_break` or `slot_replay_or_rollback_break`.

Executable evidence:

- `path2_prevalidated_slot_admission_namespace_rejects_duplicates`.

### Exact Slot And One-Use Transition

`verify_path2_direct_single_slot_spend_authorization` runs checked-core
verification before consumption, then rejects unless the decoded checked-core
slot id equals `leaf.slot_id`. Only after that exact-slot check does it call the
consume-once transition.

Executable evidence:

- `path2_checked_core_authorization_accepts_signed_path2_fixture`;
- `path2_checked_core_authorization_signer_accepts_nonzero_responses`;
- `path2_direct_single_slot_spend_rejects_leaf_slot_rebinding`;
- `path2_prevalidated_slot_consume_once_rejects_reauthorization`.

## Non-Claims

This theorem does not prove the creation-time hidden-source relation itself.
That remains the job of
`direct_replayable_relation_certificate_soundness_or_break`.

This theorem also does not model arbitrary chain reorg policy internally. A
rollback that makes an already consumed authorization spendable again is charged
to `slot_replay_or_rollback_break` and the lifecycle closure policy.
