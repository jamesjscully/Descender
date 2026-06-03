# Direct Replayable Relation Certificate Soundness Theorem

Status: theorem target for Path 2 account creation.

This theorem is the first local soundness theorem for the selected
`direct_replayable_relation_certificate` mechanism. It is the creation-time
input to the larger `account_creation_prevalidated_slot_authorization_or_break`
theorem.

## Theorem Name

`direct_replayable_relation_certificate_soundness_or_break`

## Informal Statement

For n=32, q=12289, if `verify_prevalidated_slot_creation` accepts a canonical
`PrevalidatedSlotLeafV1` and an `AccountCreationEvidenceV1` whose evidence kind
is `direct_replayable_relation_certificate`, then either
`CreationHiddenSourceRelationV1` holds for that leaf/evidence/certificate, or
the adversary exports a named break.

## Inputs

| input | meaning |
|---|---|
| `leaf` | canonical `PrevalidatedSlotLeafV1` |
| `evidence` | `AccountCreationEvidenceV1` envelope |
| `certificate` | `DirectReplayableRelationCertificateV1` payload |
| `account_state_before` | account namespace and state root before admission |
| `creation_verifier_version` | concrete verifier/profile version |
| `hash_oracle_context` | SHA-256/RO interface used by frame digests |

## Acceptance Hypotheses

The theorem assumes the creation verifier accepts only after checking:

1. `leaf` is canonical and has `live_status_marker == 0x01`;
2. `slot_leaf_digest == SHA256(leaf)`;
3. `slot_authorization_digest == SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || leaf)`;
4. every duplicated field in `evidence` matches `leaf`;
5. `leaf.prevalidation_transcript_digest` is recomputed from the evidence
   envelope and certificate payload digest;
6. `evidence.evidence_kind == direct_replayable_relation_certificate`;
7. the certificate payload is available for replay;
8. replay checks all seven groups in `CreationHiddenSourceRelationV1`,
   including that the leaf-bound creation statement is
   message/signature-independent and that any reviewed Fq12 statement digest is
   used only as direct-replay metadata;
9. duplicate `(account_id, slot_id)` and duplicate `pad_seed_digest` are rejected
   in the chosen account namespace.

## Conclusion

At least one of the following holds:

1. `CreationHiddenSourceRelationV1(leaf, evidence, certificate)` is true.
2. `creation_time_hidden_source_soundness_break` is exported.
3. `prevalidation_leaf_binding_break` is exported.
4. `qrom_or_domain_separation_break` is exported.
5. `prevalidation_data_availability_break` is exported.
6. `live_slot_leakage_or_keygen_break` is exported.
7. `transparentProofSoundnessBreak` is exported.
8. `path2_sis_norm_or_range_break` is exported.

## Break Meanings

| break | covers |
|---|---|
| `creation_time_hidden_source_soundness_break` | creation verifier accepts a replayable certificate whose active-lane hidden-source relation is false |
| `prevalidation_leaf_binding_break` | accepted evidence binds to a different account/key/slot/state/opening/pad/statement/lifecycle value than the leaf |
| `qrom_or_domain_separation_break` | distinct frames collide or a hash/RO frame is used outside its domain contract |
| `prevalidation_data_availability_break` | creation accepts without replayable certificate data or with unavailable payload bytes |
| `live_slot_leakage_or_keygen_break` | published/admitted slot metadata violates the charged live-slot leakage or keygen assumption surface |
| `transparentProofSoundnessBreak` | selected transparent payload verifier accepts without an extractable witness trace satisfying the stated proof relation |
| `path2_sis_norm_or_range_break` | extracted Path 2 descent/SIS lanes are noncanonical, out of range, or exploit modular wrap outside the carried bound |

## Proof Skeleton

| step | claim | if false |
|---|---|---|
| `normalize_leaf` | accepted leaf has one canonical byte preimage and digest | `prevalidation_leaf_binding_break` or `qrom_or_domain_separation_break` |
| `bind_evidence_to_leaf` | evidence duplicates the same account/key/slot/state/opening/pad/statement/lifecycle values | `prevalidation_leaf_binding_break` |
| `split_creation_from_replay_statement` | the leaf-bound statement digest is `CreationHiddenSourceStatementV1`, while the reviewed Fq12 signature-dependent statement digest is only certificate replay metadata | `creation_time_hidden_source_soundness_break` or `prevalidation_leaf_binding_break` |
| `bind_public_key_identity` | `leaf.public_key_identity` equals the digest of the public key whose active-lane relation is replayed | `prevalidation_leaf_binding_break` |
| `bind_transcript_payload` | leaf transcript digest commits to the evidence envelope and certificate payload digest | `qrom_or_domain_separation_break` |
| `require_payload_availability` | certificate payload is replayable by validators | `prevalidation_data_availability_break` |
| `replay_relation_checks` | replay establishes all `CreationHiddenSourceRelationV1` check groups | `creation_time_hidden_source_soundness_break` |
| `charge_live_metadata` | any public slot metadata leakage is included in the live-slot leakage ledger | `live_slot_leakage_or_keygen_break` |

## Dependencies

- `PrevalidatedSlotLeafV1` frame spec;
- `slot_authorization_digest` spec;
- `AccountCreationEvidenceV1` spec;
- `DirectReplayableRelationCertificateV1` mechanism decision;
- `CreationHiddenSourceRelationV1` spec;
- Path 2 domain registry rows;
- creation-time negative-control matrix;
- Path 2 live-slot leakage ledger.

## Explicit Non-Claims

- This theorem does not prove spend-time authorization or consume-once behavior.
- This theorem does not consume optional aggregation assumptions.
- This theorem does not prove BLS-style signature aggregation.
- This theorem does not require a compact creation proof; replayable admission
  evidence may be larger than the spend signature.
- This theorem does not revive the rejected spend-only full-witness extraction
  claim.

## Open Items To Discharge

1. Instantiate the direct creation proof payload semantics. The transparent
   extractor/soundness bridge is now pinned in
   `research/path2_transparent_extractor_soundness_bridge.md`, and
   `verify_path2_direct_creation_transparent_descent_payload_boundary` now binds
   the transparent public view and payload digest before rejecting fail-closed.
   The current public certificate still carries `public_replay_payload_digest`
   as a binding handle rather than a concrete accepting verifier result.
2. Implement or generate byte-level negative controls for the chosen payload
   verifier. The existing wrapper KATs cover leaf/evidence/certificate binding
   and active-lane replay against test-harness inputs; the remaining controls
   must target the public proof payload boundary.
3. Connect the chosen payload verifier to the spend-time
   `account_creation_prevalidated_slot_authorization_or_break` composition.

The exact residual blocker is recorded in
`research/path2_direct_relation_certificate_payload_blocker.md`.

## Proof Tracker Closure

This document states the local direct replayable certificate soundness theorem.
It closes the theorem-statement part of "Prove account creation implies relation
validity or named break"; proof discharge remains open until the public direct
creation proof payload exists and has negative-control coverage.
