# Creation-Time Negative Control KAT Plan

Status: executable-test plan for Path 2 account-creation admission. No new
verifier is claimed until these KATs exist and pass.

Implementation progress: the first narrow Rust slice exists in
`tools/parameter-search/src/production_reference.rs`. It implements
`Path2PrevalidatedSlotLeafV1`, `Path2AccountCreationEvidenceV1`,
`Path2DirectReplayableRelationCertificateV1`, canonical 353-byte leaf
serialization, `path2_slot_authorization_digest`,
`verify_path2_prevalidated_slot_creation_frame_bindings`,
`verify_path2_prevalidated_slot_creation_payload_bindings`,
`verify_path2_prevalidated_slot_admission_namespace`,
`verify_path2_prevalidated_slot_consume_once`,
`verify_path2_prevalidated_slot_creation_relation_replay`,
`validate_path2_direct_slot_authorization_admission`,
`require_path2_checked_core_authorization_challenge_binding`,
`verify_path2_checked_core_authorization`,
`verify_path2_direct_single_slot_spend_authorization`, and current focused
KATs including:

- `path2_prevalidated_slot_leaf_v1_frame_is_canonical`;
- `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations`;
- `path2_prevalidation_transcript_digest_is_not_circular`;
- `path2_prevalidated_slot_creation_payload_kats_reject_missing_or_replayed_payloads`;
- `path2_prevalidated_slot_admission_namespace_rejects_duplicates`;
- `path2_public_certificate_debug_excludes_private_opening_material`;
- `path2_prevalidated_slot_consume_once_rejects_reauthorization`;
- `path2_creation_relation_replay_accepts_reviewed_active_lane_certificate`;
- `path2_creation_relation_replay_rejects_mutated_relation_inputs`;
- `path2_creation_relation_replay_rejects_profile_and_verifier_rebinding`;
- `path2_direct_authorization_requires_exact_admitted_leaf`;
- `path2_direct_authorization_requires_state_namespace_admission`;
- `path2_direct_single_slot_spend_composes_admission_signature_and_consume_once`;
- `path2_direct_single_slot_spend_rejects_leaf_slot_rebinding`;
- `path2_checked_core_authorization_challenge_binds_slot_authorization`;
- `path2_checked_core_authorization_verifier_uses_path2_challenge_frame`;
- `path2_checked_core_authorization_accepts_signed_path2_fixture`;
- `path2_checked_core_authorization_signer_accepts_nonzero_responses`.

These tests close only the first frame/evidence binding slice. They do not
implement hidden-source relation replay, payload availability, lifecycle
admission, or spend authorization.

The first implementation also found and fixed two proof-frame issues before they
became harder to unwind:

- `prevalidation_transcript_digest` must not include `slot_leaf_digest`, because
  the final leaf digest already includes `prevalidation_transcript_digest`;
- `slot_id` must be duplicated inside the evidence transcript, or cross-slot
  replay is under-specified.

## Target

The tests target the future direct-mode verifier:

```text
verify_prevalidated_slot_creation(
  PrevalidatedSlotLeafV1,
  AccountCreationEvidenceV1 {
    evidence_kind = direct_replayable_relation_certificate,
    payload = DirectReplayableRelationCertificateV1
  }
)
```

The verifier must fail closed for every single-field mutation listed in
`research/creation_time_negative_control_matrix.md`.

## Existing Executable Coverage To Reuse

The existing Rust source already exercises the active-lane core that Path 2 will
reuse at creation time.

| Path 2 control | existing test/pattern | source |
|---|---|---|
| `witness_opening_substitution` | `reviewed_fq12_packed_witness_rejects_malformed_opening_fields` and `masked_projection_malformed_openings_reject_without_material` | `tools/parameter-search/src/production_reference.rs` |
| `pad_seed_substitution` | `reviewed_fq12_packed_witness_rejects_wrong_pad_seed_unmask`, `selected_projection_reject_pad_seed_rebinding`, and `masked_projection_wrong_pad_seed_rejects_without_partial_opening` | `tools/parameter-search/src/production_reference.rs` |
| `public_pad_statement_mismatch` | pad/statement mismatch checks through `verify_reviewed_fq12_state_assisted_pad_seed_opening` and masked projection tuple tests | `tools/parameter-search/src/production_reference.rs` |
| `active_lane_projection_mismatch` | `reviewed_fq12_packed_witness_rejects_selected_lane_mutation` and `selected_projection_reject_carrier_lane_mutation` | `tools/parameter-search/src/production_reference.rs` |
| `opening_commitment_mutation` | `selected_projection_reject_account_state_rebinding` and slot-opening commitment checks | `tools/parameter-search/src/production_reference.rs` |
| `hidden_source_statement_digest_mutation` | `masked_projection_malformed_openings_reject_without_material` and `masked_projection_carrier_rebinding_rejects_without_export` | `tools/parameter-search/src/production_reference.rs` |
| `carrier_slot_context_replay` | statement slot rebinding in `masked_projection_carrier_rebinding_rejects_without_export` | `tools/parameter-search/src/production_reference.rs` |
| no private material on rejection | `assert_no_selected_projection_material` family and public-state digest-only tests | `tools/parameter-search/src/production_reference.rs` |
| one-use lifecycle pattern | lifecycle manager and spent-ledger replay tests | `tools/parameter-search/src/production_reference.rs` |

These tests are not sufficient by themselves because they verify the older
308-byte/Fq12 candidate path, not the Path 2 leaf/evidence envelope.

## New Fixture Required

Add a local test fixture when Path 2 implementation starts:

```text
PrevalidatedSlotCreationFixture {
  leaf: PrevalidatedSlotLeafV1,
  evidence: AccountCreationEvidenceV1,
  certificate: DirectReplayableRelationCertificateV1,
  slot_authorization_digest,
  account_state_before,
  account_state_after,
}
```

The fixture should be built from the existing reviewed Fq12 packed-witness
fixture, then wrapped in the Path 2 leaf/evidence envelope. The first accepting
test must prove that the wrapper does not alter the underlying active-lane
acceptance conditions.

Current blocker: the existing reviewed Fq12 hidden-source statement frame is
signature-dependent. The final accepting Path 2 spend fixture must wait for a
non-circular `CreationHiddenSourceStatementV1` that excludes spend-time message
and signature digests. Until then, reviewed Fq12 statement replay is a
negative-control source and executable skeleton only.

## New KATs By Implementation Layer

### Leaf And Evidence Binding

| test id | mutation | expected rejection |
|---|---|---|
| `path2_kat_leaf_digest_mutation` | mutate `evidence.slot_leaf_digest` only | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_authorization_digest_mutation` | mutate `evidence.slot_authorization_digest` only | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_account_id_mutation` | mutate evidence account id while leaf is fixed | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_public_key_identity_mutation` | mutate evidence public-key identity while leaf is fixed | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_slot_id_mutation` | mutate evidence or leaf slot id without recomputing certificate | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` and `path2_prevalidation_transcript_digest_is_not_circular` |
| `path2_kat_verifier_version_mutation` | mutate verifier version digest in evidence | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_account_state_root_mutation` | mutate `state_root_before` | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_prevalidation_transcript_digest_mutation` | mutate leaf transcript digest only | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_relation_payload_digest_mutation` | mutate evidence payload digest only | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |
| `path2_kat_non_live_status_marker` | set marker to any value other than `0x01` | implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations` |

### Payload Availability

| test id | mutation | expected rejection |
|---|---|---|
| `path2_kat_missing_payload_with_digest` | retain payload digest but remove payload bytes/reference | implemented in `path2_prevalidated_slot_creation_payload_kats_reject_missing_or_replayed_payloads` |
| `path2_kat_payload_digest_cross_leaf_replay` | use valid payload digest from another leaf | implemented as payload digest mismatch / transcript-bound replay rejection |
| `path2_kat_payload_payload_cross_leaf_replay` | use valid payload bytes from another account/key/slot/state root | implemented for account-id, public-key, slot-id, and state-root cross-leaf replay in `path2_prevalidated_slot_creation_payload_kats_reject_missing_or_replayed_payloads` |

### Context Replay

| test id | mutation | expected rejection |
|---|---|---|
| `path2_kat_carrier_account_context_replay` | replay carrier/certificate under another account id | `prevalidation_leaf_binding_break` |
| `path2_kat_carrier_public_key_context_replay` | replay carrier/certificate under another public-key identity | implemented in `path2_creation_relation_replay_rejects_profile_and_verifier_rebinding` |
| `path2_kat_carrier_state_root_context_replay` | replay carrier/certificate under another account state root | `prevalidation_registry_binding_break` |
| `path2_kat_carrier_verifier_context_replay` | replay carrier/certificate under another verifier version | implemented in `path2_creation_relation_replay_rejects_profile_and_verifier_rebinding` |
| `path2_kat_relation_replay_acceptance` | replay the active-lane reviewed Fq12 relation through the Path 2 wrapper | implemented in `path2_creation_relation_replay_accepts_reviewed_active_lane_certificate` |
| `path2_kat_relation_replay_mutated_pad_seed` | replay with wrong pad seed after wrapper binding succeeds | implemented in `path2_creation_relation_replay_rejects_mutated_relation_inputs` |
| `path2_kat_relation_replay_statement_mismatch` | bind wrapper to a statement digest different from the replayed active-lane statement | implemented in `path2_creation_relation_replay_rejects_mutated_relation_inputs` |

### Domains

| test id | mutation | expected rejection |
|---|---|---|
| `path2_kat_slot_leaf_domain_swap` | leaf uses any domain except `DESCENDER_SLOT_LEAF_V1` | `qrom_or_domain_separation_break` |
| `path2_kat_authorization_domain_swap` | authorization digest uses wrong domain | `qrom_or_domain_separation_break` |
| `path2_kat_single_batch_domain_confusion` | direct evidence uses batch domain or batch evidence uses direct domain | direct-mode batch-domain rejection implemented in `path2_prevalidated_slot_creation_frame_binding_kats_reject_mutations`; batch-mode tests remain future optional-aggregation work |

### Lifecycle And Uniqueness

| test id | mutation | expected rejection |
|---|---|---|
| `path2_kat_duplicate_slot_id_admission` | admit two live leaves for same `(account_id, slot_id)` | implemented in `path2_prevalidated_slot_admission_namespace_rejects_duplicates` |
| `path2_kat_duplicate_pad_seed_digest_admission` | admit two live leaves with same `pad_seed_digest` in uniqueness domain | implemented in `path2_prevalidated_slot_admission_namespace_rejects_duplicates` |
| `path2_kat_spent_leaf_reauthorization` | consume a leaf, then try to authorize it again | implemented in `path2_prevalidated_slot_consume_once_rejects_reauthorization` |

### Leakage Guard KATs

| test id | check |
|---|---|
| `path2_kat_public_leaf_contains_no_raw_pad_seed` | implemented for debug output in `path2_public_certificate_debug_excludes_private_opening_material`; serde serialization remains future work if these structs become serializable |
| `path2_kat_public_evidence_contains_no_projection_masks` | implemented for debug output in `path2_public_certificate_debug_excludes_private_opening_material` |
| `path2_kat_public_evidence_contains_no_unmasked_projection` | implemented for debug output in `path2_public_certificate_debug_excludes_private_opening_material` |
| `path2_kat_reject_paths_emit_no_private_material` | every rejection reason/debug output passes the existing `assert_no_selected_projection_material` filter |

## Suggested File Targets

Initial implementation should keep tests near the existing reviewed Fq12
negative controls:

```text
tools/parameter-search/src/production_reference.rs
```

Once the Path 2 verifier grows large enough, move the admission structs and KATs
into a dedicated module or crate-facing test file, but preserve the existing
fixture reuse so the old 308-byte path remains a negative-control baseline.

## Minimum Promotion Gate

Do not mark `direct_replayable_relation_certificate_soundness_or_break`
implemented until all of these are true:

- accepting Path 2 wrapper fixture passes;
- creation statement digest is fixed before the spend message/signature and has
  a KAT rejecting accidental `message_digest`/`signature_digest` reintroduction;
- every leaf/evidence mutation rejects;
- payload availability failure rejects;
- account/key/slot/state/verifier replay rejects;
- direct/batch domain confusion rejects;
- duplicate slot and duplicate pad-seed policies reject;
- public serialization/debug output excludes raw pad seed, projection masks, and
  unmasked selected account projection residues;
- regenerated preflight/status artifacts still report deployment fail-closed
  until the theorem and verifier are complete.

## Tracker Impact

This plan turns the matrix from a proof-only checklist into executable work.
It does not close the negative-control task. Closure requires Rust KATs and, for
the leaf/evidence byte frames, matching Lean or byte-frame constants if Lean is
used for the proof object.
