# Creation-Time Negative Control Matrix

Status: proof matrix for Path 2 account creation.

This matrix enumerates the byte-level and relation-level mutations that must
reject before a prevalidated slot can become live. It lifts the existing
active-lane hidden-relation negative controls into the account-creation setting
and adds the Path 2 leaf/evidence/registry checks.

## Scope

Verifier under test:

```text
verify_prevalidated_slot_creation(
  PrevalidatedSlotLeafV1,
  AccountCreationEvidenceV1 {
    evidence_kind = direct_replayable_relation_certificate,
    payload = DirectReplayableRelationCertificateV1
  }
)
```

Acceptance is permitted only if `CreationHiddenSourceRelationV1` holds and the
leaf can be admitted as a live slot in the account namespace.

## Matrix

| id | mutation | must reject at | named break if accepts | evidence status |
|---|---|---|---|---|
| `leaf_digest_mutation` | mutate `slot_leaf_digest` while keeping leaf bytes fixed | evidence-to-leaf binding | `prevalidation_leaf_binding_break` or `qrom_or_domain_separation_break` | spec-only |
| `authorization_digest_mutation` | mutate `slot_authorization_digest` while keeping leaf bytes fixed | authorization digest recomputation | `prevalidation_leaf_binding_break` or `qrom_or_domain_separation_break` | spec-only |
| `account_id_mutation` | bind evidence to one account id and leaf to another | leaf/evidence duplicate-field equality | `prevalidation_leaf_binding_break` | spec-only |
| `public_key_identity_mutation` | bind certificate to a different public key identity than the leaf | leaf/evidence duplicate-field equality | `prevalidation_leaf_binding_break` | spec-only |
| `slot_id_mutation` | use different slot ids in leaf, opening tuple, carrier transcript, or evidence | leaf/evidence/opening/carrier binding | `prevalidation_leaf_binding_break` or `creation_time_hidden_source_soundness_break` | spec-only |
| `verifier_version_mutation` | replay evidence under a verifier version not named by the leaf | verifier/profile binding | `prevalidation_leaf_binding_break` or `qrom_or_domain_separation_break` | spec-only |
| `account_state_root_mutation` | admit a certificate generated for a different state root | state-root binding | `prevalidation_registry_binding_break` | spec-only |
| `opening_commitment_mutation` | keep certificate payload valid but change leaf opening commitment | state-assisted opening commitment check | `prevalidation_leaf_binding_break` or `creation_time_hidden_source_soundness_break` | covered-by-existing-relation-pattern |
| `pad_seed_digest_mutation` | keep pad proof/certificate fixed but change leaf pad-seed digest | pad-seed opening check | `prevalidation_leaf_binding_break` or `creation_time_hidden_source_soundness_break` | covered-by-existing-relation-pattern |
| `hidden_source_statement_digest_mutation` | change the leaf statement digest while replaying the old statement | statement digest recomputation | `prevalidation_leaf_binding_break` or `creation_time_hidden_source_soundness_break` | covered-by-existing-relation-pattern |
| `prevalidation_transcript_digest_mutation` | change leaf transcript digest without changing evidence payload | evidence transcript digest recomputation | `qrom_or_domain_separation_break` | spec-only |
| `relation_payload_digest_mutation` | change payload digest while keeping payload bytes fixed | evidence transcript digest recomputation | `qrom_or_domain_separation_break` | spec-only |
| `missing_payload_with_digest` | provide only payload digest/certificate root but no replayable payload | data availability gate | `prevalidation_data_availability_break` | spec-only |
| `witness_opening_substitution` | substitute witness opening while holding account witness commitment and carrier shape fixed | masked-state18 packed opening / repack check | `creation_time_hidden_source_soundness_break` or account commitment binding break | covered-by-existing-rust-negative-control |
| `pad_seed_substitution` | substitute pad seed while holding pad digest/public pad statement fixed | pad-seed opening and pad statement derivation | `creation_time_hidden_source_soundness_break` or pad/PRF break | covered-by-existing-rust-negative-control |
| `public_pad_statement_mismatch` | derive or supply public pad statement inconsistent with pad seed | pad opening statement check | `creation_time_hidden_source_soundness_break` | covered-by-existing-rust-negative-control |
| `active_lane_projection_mismatch` | splice witness projection and pad projection from different statements | active-lane hidden-source relation check | `creation_time_hidden_source_soundness_break` | covered-by-existing-rust-negative-control |
| `carrier_account_context_replay` | replay valid carrier under another account id or public key identity | carrier transcript binding | `creation_time_hidden_source_soundness_break` or `prevalidation_leaf_binding_break` | spec-extension-needed |
| `carrier_slot_context_replay` | replay valid carrier under another slot id | carrier transcript binding | `creation_time_hidden_source_soundness_break` or `prevalidation_leaf_binding_break` | covered-by-existing-relation-pattern |
| `carrier_state_root_context_replay` | replay valid carrier under another account state root | carrier transcript / leaf binding | `prevalidation_registry_binding_break` | spec-extension-needed |
| `carrier_verifier_context_replay` | replay valid carrier under another verifier version/profile | carrier transcript / verifier binding | `qrom_or_domain_separation_break` or `creation_time_hidden_source_soundness_break` | spec-extension-needed |
| `precommitment_bypass` | use a relation-valid opening not committed by leaf opening commitment | state-assisted opening commitment check | `creation_time_hidden_source_soundness_break` or opening binding break | covered-by-existing-rust-negative-control |
| `false_statement_acceptance` | construct a syntactically valid certificate for a false active-lane statement | hidden-source relation replay | `creation_time_hidden_source_soundness_break` | covered-by-existing-rust-negative-control |
| `duplicate_slot_id_admission` | admit two live leaves with same `(account_id, slot_id)` | admission namespace check | `prevalidation_registry_binding_break` or `slot_replay_or_rollback_break` | lifecycle-pattern-existing |
| `duplicate_pad_seed_digest_admission` | admit two live leaves with same `pad_seed_digest` in uniqueness domain | pad-seed uniqueness check | `live_slot_leakage_or_keygen_break` or lifecycle break | lifecycle-pattern-existing |
| `non_live_status_marker` | try to admit or authorize leaf with `live_status_marker != 0x01` | leaf canonicality / admission status | `prevalidation_leaf_binding_break` or lifecycle break | spec-only |
| `slot_leaf_domain_swap` | use a non-Path-2 domain in the leaf preimage | leaf canonicality | `qrom_or_domain_separation_break` | domain-registry-spec |
| `authorization_domain_swap` | compute authorization digest under another domain | authorization digest recomputation | `qrom_or_domain_separation_break` | domain-registry-spec |
| `single_batch_domain_confusion` | admit direct evidence under a batch prevalidation domain | evidence domain check | `qrom_or_domain_separation_break` or `optional_aggregation_soundness_break` | domain-registry-spec |

## Coverage Classes

| status | meaning |
|---|---|
| `covered-by-existing-rust-negative-control` | already represented by the selected hidden-relation negative-control artifact |
| `covered-by-existing-relation-pattern` | existing Rust candidate tests exercise the same class, but Path 2 creation-specific framing still needs KATs |
| `lifecycle-pattern-existing` | existing lifecycle tests cover the policy class; creation-admission KATs still needed |
| `domain-registry-spec` | planning domains are registered with duplicate/prefix checks; mutation KATs still needed |
| `spec-extension-needed` | new Path 2 context binding not fully covered by existing 308-byte spend-carrier tests |
| `spec-only` | currently specified here with no executable KAT |

## Minimum Executable KAT Set

The first implementation should prioritize these KATs:

1. leaf digest mutation;
2. authorization digest mutation;
3. account id mutation;
4. public-key identity mutation;
5. slot id mutation;
6. verifier version mutation;
7. account state root mutation;
8. opening commitment mutation;
9. pad-seed digest mutation;
10. hidden-source statement digest mutation;
11. missing payload with retained digest;
12. witness opening substitution;
13. pad seed substitution;
14. public pad statement mismatch;
15. active-lane projection mismatch;
16. carrier account/key context replay;
17. carrier state-root context replay;
18. duplicate slot id admission;
19. duplicate pad-seed digest admission;
20. direct-vs-batch domain confusion.

## Theorem Use

This matrix supports two proof obligations:

- `direct_replayable_relation_certificate_soundness_or_break`: every accepted
  single-fault mutation must map to relation validity or a named break.
- `account_creation_prevalidated_slot_authorization_or_break`: any spend using a
  malformed or misbound prevalidation record must fail before authorization or
  export a named break.

## Current Gap

This matrix is a proof-planning artifact. It does not claim that all Path 2 KATs
exist yet. The largest new gaps are account/public-key context replay,
state-root context replay, evidence payload availability, and direct/batch domain
confusion.
