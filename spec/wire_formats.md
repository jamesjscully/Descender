# Wire Formats And Public Objects

Status: ACTIVE_NORMATIVE_FOR_AUTHORITY_MAP

This file names the active public objects and records the consensus-facing byte
frames that are stable enough to be treated as current authority. Supporting
research notes may contain additional mechanism details, but they do not
supersede this file.

## Active Objects

| Object | Status | Detailed source |
|---|---|---|
| `PrevalidatedSlotLeafV1` | active direct-mode leaf | `research/prevalidated_slot_leaf_v1_frame.md` |
| `slot_authorization_digest` | active normalized spend handle | `research/prevalidated_slot_leaf_v1_frame.md` |
| `AccountCreationEvidenceV1` | active creation evidence envelope | `research/account_creation_evidence_v1_spec.md` |
| `SpendAuthorizationRelationV1` | active spend relation | `research/spend_authorization_relation_v1_spec.md` |
| `Path2NativeCheckedCoreFrameV1` | active checked-core challenge frame | `tools/parameter-search/src/production_reference.rs` |
| `AccountCreationPublic` | active Plonky3 public statement | `tools/descender-account-stark/src/lib.rs` |
| `DeployedCreationAnchors` | active STARK-to-leaf anchor boundary | `tools/descender-account-verifier/src/lib.rs` |

## Domain Constants

| Constant | Value | Bytes | Status |
|---|---|---:|---|
| `PATH2_PREVALIDATED_SLOT_LEAF_DOMAIN` | `DESCENDER_SLOT_LEAF_V1` | 22 | active direct-mode leaf |
| `PATH2_SLOT_AUTHORIZATION_DOMAIN` | `DESCENDER_SLOT_AUTHORIZATION_V1` | 31 | active direct-mode authorization digest |
| `PATH2_SLOT_PREVALIDATE_SINGLE_DOMAIN` | `DESCENDER_SLOT_PREVALIDATE_SINGLE_V1` | 36 | active single-slot evidence domain |
| `PATH2_NATIVE_CHECKED_CORE_FRAME_DOMAIN` | `Descender.Path2.NativeCheckedCoreFrame.v1` | 41 | active Poseidon2 checked-core challenge frame |

`PATH2_PREVALIDATED_SLOT_LEAF_V2_DOMAIN` and
`PATH2_SLOT_AUTHORIZATION_V2_DOMAIN` exist in the implementation as an
extension point with an added `creation_proof_binding_digest`, but V2 is not yet
the normative direct-mode leaf. Promoting V2 requires an ADR or an explicit spec
update.

## PrevalidatedSlotLeafV1

`PrevalidatedSlotLeafV1` is a fixed 353-byte preimage. All multibyte integers
are little-endian. All digest fields are 32 bytes.

| Offset | Field | Bytes | Encoding |
|---:|---|---:|---|
| 0 | `domain_separator` | 22 | ASCII `DESCENDER_SLOT_LEAF_V1` |
| 22 | `frame_version` | 2 | `u16`, value `1` |
| 24 | `profile_version_digest` | 32 | digest |
| 56 | `account_id` | 32 | digest |
| 88 | `public_key_identity` | 32 | digest |
| 120 | `slot_id` | 8 | `u64` |
| 128 | `verifier_version_digest` | 32 | digest |
| 160 | `account_state_root_before` | 32 | digest |
| 192 | `opening_commitment` | 32 | digest or commitment bytes |
| 224 | `pad_seed_digest` | 32 | digest |
| 256 | `hidden_source_statement_digest` | 32 | digest |
| 288 | `prevalidation_transcript_digest` | 32 | digest |
| 320 | `lifecycle_anchor` | 32 | digest |
| 352 | `live_status_marker` | 1 | `0x01` for live |

The active direct-mode implementation rejects a leaf unless:

- `frame_version == 1`;
- `domain_separator == DESCENDER_SLOT_LEAF_V1`;
- `live_status_marker == 0x01`.

The leaf digest is:

```text
prevalidated_slot_leaf_digest =
  SHA256(PrevalidatedSlotLeafV1_bytes)
```

## Slot Authorization Digest

Direct-mode spend authorization binds this normalized digest, not a chain-local
record encoding:

```text
slot_authorization_digest =
  SHA256(
    ASCII("DESCENDER_SLOT_AUTHORIZATION_V1") ||
    PrevalidatedSlotLeafV1_bytes
  )
```

Every direct record, inclusion proof, or future aggregate handle must normalize
to the same `slot_authorization_digest` before checked-core spend verification.

## AccountCreationEvidenceV1

`AccountCreationEvidenceV1` is a proof-interface envelope. Its fields are not
currently a compact consensus byte object; they define what the account-creation
verifier must bind before a leaf can be admitted.

| Field | Type | Required relation to leaf |
|---|---|---|
| `evidence_domain` | string | `DESCENDER_SLOT_PREVALIDATE_SINGLE_V1` for direct mode |
| `evidence_version` | `u16` | `1` |
| `evidence_kind` | `u16` | direct replayable relation certificate is kind `1` |
| `slot_leaf_digest` | digest | `SHA256(PrevalidatedSlotLeafV1_bytes)` |
| `slot_authorization_digest` | digest | digest defined above |
| `creation_verifier_version_digest` | digest | equals `leaf.verifier_version_digest` |
| `hidden_source_statement_digest` | digest | equals leaf field |
| `opening_commitment` | digest | equals leaf field |
| `pad_seed_digest` | digest | equals leaf field |
| `account_identity_digest` | digest | equals `leaf.account_id` |
| `public_key_identity` | digest | equals leaf field |
| `slot_id` | `u64` | equals leaf field |
| `state_root_before` | digest | equals `leaf.account_state_root_before` |
| `lifecycle_anchor` | digest | equals leaf field |
| `relation_transcript_digest` | digest | checked creation-time relation transcript |
| `evidence_payload_digest` | digest | concrete proof or certificate payload |

`prevalidation_transcript_digest` in the leaf is the hash of the evidence
transcript fields, excluding `slot_leaf_digest` to avoid circularity. The
creation verifier must reject if any duplicated evidence field disagrees with
the leaf.

## Path2NativeCheckedCoreFrameV1

The active checked-core frame is the Poseidon2-friendly challenge frame consumed
by the Path 2 native spend path. The byte encoding is:

```text
ASCII("Descender.Path2.NativeCheckedCoreFrame.v1") ||
native_boundary_version:u16 ||
parameter_set_id:u16 ||
verifier_family_id:u16 ||
account_creation_domain_digest[32] ||
spend_verifier_domain_digest[32] ||
chain_or_application_domain_digest[32] ||
fork_or_epoch_scope_digest[32] ||
aggregation_mode:u8 ||
native_boundary_digest[32] ||
p_account_digest[32] ||
p_spend_digest[32] ||
p_lifecycle_digest[32] ||
p_transcript_digest[32] ||
p_state_digest[32] ||
message_digest[32] ||
current_state_digest[32] ||
proof_commitment[32]
```

The current size is `464` bytes: 41 bytes of domain plus a 423-byte body.

Current fixed values:

| Field | Value | Meaning |
|---|---:|---|
| `native_boundary_version` | `1` | checked-core frame version |
| `parameter_set_id` | `1` | q12289 parameter set |
| `verifier_family_id` | `1` | Fq12 checked-core verifier family |
| `aggregation_mode` | `0` or `1` | direct or aggregate-compatible; other values reserved |

The checked-core challenge is derived from this frame through the active
Poseidon2 checked-core challenge path, not from the legacy SHA-256 transaction
body.

## SpendAuthorizationRelationV1

Spend authorization accepts only after handle validation produces the normalized
leaf fields and `slot_authorization_digest`, the checked-core verifier accepts a
challenge frame bound to those values, and the lifecycle transition consumes the
exact live slot once under `spec/lifecycle_model.md`.

Direct mode must not require any raw pad seed, projection masks, unmasked
account projection residues, or private opening frames at spend time.

## Legacy Objects

| Object | Current status |
|---|---|
| 314-byte `checked_core || projection_carrier` envelope | legacy fail-closed lineage, retained as evidence/negative controls |
| 308-byte projection-carrier labels | stale pre-widening labels, not active product target |
| 260-byte linear-sigma/KAT v0 path | conformance and formal-history artifact, not deployable security target |

## Rule For New Wire Formats

Tracker ownership: `DIR-LEAF-001`, `DIR-EVID-001`, `DIR-CC-001`,
`DIR-KAT-001`, and `DIR-REJECT-001`.

Any new consensus-visible or wallet-visible wire format must update:

1. this file;
2. `TRACKER.md`;
3. an ADR if the format changes product direction, security assumptions, or
   compatibility;
4. KATs that mutate every semantic field independently.
