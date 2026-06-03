# PrevalidatedSlotLeafV1 Frame

Status: proof-frame specification for Path 2.

This document closes the first proof-object vocabulary item for the
account-creation / prevalidated-slot path: the canonical slot leaf frame and the
normalized slot authorization digest.

## Design Rule

The leaf is a fixed-width binary preimage. Variable-sized protocol objects are
represented by 32-byte digests before they enter the leaf. This gives the proof,
Rust implementation, Lean constants, account-state roots, and optional
aggregation paths one unambiguous object to bind.

The first implementation may store direct slot records rather than Merkle or
accumulator leaves, but every direct record must be able to derive this exact
leaf.

## Domains

Reserved ASCII domains:

| id | bytes | length |
|---|---|---:|
| slot leaf | `DESCENDER_SLOT_LEAF_V1` | `22` |
| slot authorization | `DESCENDER_SLOT_AUTHORIZATION_V1` | `31` |

These are planning domains. They still need to be added to the executable global
domain registry before implementation promotion.

## Leaf Byte Frame

`PrevalidatedSlotLeafV1` is serialized in this exact order:

| offset | field | bytes | encoding | proof role |
|---:|---|---:|---|---|
| 0 | `domain_separator` | 22 | ASCII `DESCENDER_SLOT_LEAF_V1` | separates leaf preimages from every other hash frame |
| 22 | `frame_version` | 2 | little-endian `u16`, value `1` | versioned frame guard |
| 24 | `profile_version_digest` | 32 | SHA-256 digest | binds `Descender.Signature.q12289.local_delta.stateful.v1` or successor profile |
| 56 | `account_id` | 32 | canonical account namespace digest | prevents cross-account leaf reuse |
| 88 | `public_key_identity` | 32 | canonical public-key digest | binds the slot to the verifying key |
| 120 | `slot_id` | 8 | little-endian `u64` | embeds current `u32` slot ids with high bits zero |
| 128 | `verifier_version_digest` | 32 | SHA-256 digest | prevents old verifier/prevalidation replay |
| 160 | `account_state_root_before` | 32 | state-root digest | fixes the admission state context |
| 192 | `opening_commitment` | 32 | digest/commitment bytes | binds the state-assisted hidden-source opening |
| 224 | `pad_seed_digest` | 32 | digest bytes | binds the hidden pad seed without revealing it |
| 256 | `hidden_source_statement_digest` | 32 | digest bytes | binds the creation-time hidden-source statement |
| 288 | `prevalidation_transcript_digest` | 32 | digest bytes | binds the creation verifier transcript/certificate |
| 320 | `lifecycle_anchor` | 32 | digest bytes | binds restore/rollback lifecycle context |
| 352 | `live_status_marker` | 1 | byte, `0x01` for live prevalidated | prevents spent/reserved records from sharing live authorization |

Total leaf preimage size: `353` bytes.

## Digest Definitions

```text
prevalidated_slot_leaf_digest =
  SHA256(PrevalidatedSlotLeafV1_bytes)

slot_authorization_digest =
  SHA256(
    ASCII("DESCENDER_SLOT_AUTHORIZATION_V1") ||
    PrevalidatedSlotLeafV1_bytes
  )
```

The checked-core spend challenge must bind `slot_authorization_digest`, not the
concrete representation used to validate it. Direct records, inclusion proofs,
and future aggregated proof handles all normalize to the same digest.

`prevalidation_transcript_digest` is computed before the final leaf digest. It
binds the evidence envelope and duplicated leaf binding fields directly; it must
not include `prevalidated_slot_leaf_digest`, because that digest hashes the full
leaf including `prevalidation_transcript_digest`.

## Mapping From Current State-Assisted Slot Records

The current executable slot record already supplies:

| leaf field | current source |
|---|---|
| `slot_id` | `ProductionReviewedFq12StateAssistedAccountSlotRecord.slot_id` |
| `opening_commitment` | `hidden_source_opening_commitment` |
| `pad_seed_digest` | `pad_seed_digest` |
| `lifecycle_anchor` | `restore_anchor_digest` |
| `live_status_marker` | `unused == true` maps to `0x01`; consumed/spent records are not live leaves |

The Path 2 prevalidation layer must add or derive:

| leaf field | required source |
|---|---|
| `profile_version_digest` | digest of the selected deployment profile string |
| `account_id` | canonical account namespace digest |
| `public_key_identity` | canonical public-key digest |
| `verifier_version_digest` | digest of the account-creation verifier/profile version |
| `account_state_root_before` | state root before slot admission or account-refresh update |
| `hidden_source_statement_digest` | digest of the creation-time hidden-source statement |
| `prevalidation_transcript_digest` | digest/root of the creation-time validation transcript or proof certificate |

## Binding Requirements

An accepted spend may use a leaf only if all of these are true:

- the leaf digest was admitted by the account-creation verifier;
- the leaf account id matches the account namespace being spent;
- the public-key identity matches the checked-core verifier key;
- the slot id matches the checked-core challenge frame;
- the verifier version digest matches the verifier used at account creation;
- the opening commitment and pad-seed digest match the admitted hidden-source
  relation;
- the hidden-source statement digest matches the admitted relation;
- the prevalidation transcript digest names a real creation-time validation
  transcript or certificate, not a bare unchecked hash;
- the lifecycle anchor is current for the state transition;
- `live_status_marker == 0x01`.

## Required Negative Controls

The first byte-level tests for this frame should mutate each field independently
and require authorization failure:

- profile version digest;
- account id;
- public-key identity;
- slot id;
- verifier version digest;
- account state root;
- opening commitment;
- pad-seed digest;
- hidden-source statement digest;
- prevalidation transcript digest;
- lifecycle anchor;
- live-status marker;
- slot leaf domain;
- slot authorization domain.

## Rust And Lean Constant Plan

Rust constants to add when implementation starts:

```text
DESCENDER_SLOT_LEAF_DOMAIN: &[u8] = b"DESCENDER_SLOT_LEAF_V1"
DESCENDER_SLOT_AUTHORIZATION_DOMAIN: &[u8] = b"DESCENDER_SLOT_AUTHORIZATION_V1"
PREVALIDATED_SLOT_LEAF_V1_BYTES: usize = 353
PREVALIDATED_SLOT_LEAF_FRAME_VERSION: u16 = 1
```

Lean constants should mirror:

```text
slotLeafDomainLen = 22
slotAuthorizationDomainLen = 31
prevalidatedSlotLeafV1Bytes = 353
prevalidatedSlotLeafFrameVersion = 1
```

## Proof Tracker Closure

This spec closes the proof vocabulary item "Define `PrevalidatedSlotLeafV1`
byte/field frame" at the planning/specification level. It also closes the
paired item "Define `slot_authorization_digest`" at the same level. Executable
closure remains open until Rust constants, domain-registry rows, byte-level KATs,
and Lean/Rust constant checks exist.
