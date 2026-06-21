# CreationHiddenSourceRelationV1 Spec

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting relation spec note, not a normative spec. Current
> normative authority starts at `../spec/README.md`.

Status: proof relation specification for Path 2 account creation.

`CreationHiddenSourceRelationV1` is the relation checked by the selected
`direct_replayable_relation_certificate` mechanism before a slot becomes live.
It is the account-creation version of the active-lane, state-assisted,
masked-projection relation, with one critical split from the older spend-carried
308-byte/Fq12 path: the creation-time statement must be independent of the
future spend message and signature.

## Non-Claim

This relation does not prove a full hidden-source witness over every historical
64-variable projection coordinate. That spend-only claim is already rejected by
negative control evidence. The creation relation proves the selected active-lane
relation that the account state later authorizes for one spend.

## Public Inputs

The relation consumes these public inputs:

| input | source |
|---|---|
| `leaf` | canonical `PrevalidatedSlotLeafV1` |
| `slot_authorization_digest` | normalized digest derived from `leaf` |
| `creation_verifier_version_digest` | verifier/profile version for admission |
| `account_id` | account namespace digest from `leaf` |
| `public_key_identity` | public-key digest from `leaf` |
| `slot_id` | slot id from `leaf` |
| `account_state_root_before` | admission state root from `leaf` |
| `opening_commitment` | state-assisted hidden-source opening commitment from `leaf` |
| `pad_seed_digest` | hidden pad seed digest from `leaf` |
| `hidden_source_statement_digest` | creation-time active-lane statement digest from `leaf` |
| `prevalidation_transcript_digest` | transcript binding digest from `leaf` |
| `lifecycle_anchor` | restore/state lifecycle anchor from `leaf` |
| `checked_core_context_digest` | creation-time digest of checked-core context authorized by the slot |

The checked-core context digest is included only if it is fixed before the leaf
is admitted. The existing reviewed Fq12 statement frame includes spend-time
`message_digest` and `signature_digest`; that object is not a valid final Path 2
creation statement because it creates a circular dependency through
`slot_authorization_digest`.

## Certificate Witness / Replay Payload

The direct replayable certificate supplies enough data to replay these hidden
and public checks:

| component | hidden/public | purpose |
|---|:---:|---|
| `state_assisted_opening_tuple` | hidden/private opening material committed by digest | binds witness opening, pad seed, restore anchor, and carrier handle material before admission |
| `account_witness_commitment` | public commitment | account witness commitment named by account state |
| `account_randomizer_commitment` | public commitment | pad/randomizer commitment named by account state |
| `pad_seed` or replayable pad opening proof | hidden/private unless proof hides it | opens `pad_seed_digest` and derives public pad-opening statement |
| `public_pad_opening_statement` | public | verifier-owned pad-opening statement for the selected profile |
| `masked_state18_packed_witness_opening` | hidden/private unless proof hides it | opens the 32-byte packed witness state to 12 checked-core residues and six masked projection residues |
| `account_witness_projection_opening_frame` | hidden/private unless proof hides it | selected six account projection residues and digest frame |
| `hidden_source_statement` | public | canonical creation-time active-lane relation statement; no spend message or signature digest |
| `projection_carrier_56_bytes` | public/replayable | 12 lanes, check bytes, and transcript binding |
| `carrier_decoding` | public/replayable | decoded lane/check/transcript view |

If a later compact proof hides these witnesses, the certificate may carry proof
bytes instead of raw openings. For the first proof pass, the relation is defined
by replayability: validators can check the relation directly from available
certificate data.

## Relation Checks

`CreationHiddenSourceRelationV1(leaf, evidence)` holds iff all checks below hold.

### 1. Leaf And Evidence Binding

- `leaf.domain_separator == DESCENDER_SLOT_LEAF_V1`;
- `leaf.live_status_marker == 0x01`;
- `slot_authorization_digest == SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || leaf)`;
- `evidence.slot_leaf_digest == SHA256(leaf)`;
- all duplicated account/key/slot/state/opening/pad/statement/lifecycle fields
  in `AccountCreationEvidenceV1` match `leaf`;
- `leaf.prevalidation_transcript_digest` matches the domain-separated evidence
  transcript digest rule.

### 2. State-Assisted Opening Commitment

- the opening tuple slot id equals `leaf.slot_id`;
- the opening tuple account witness commitment equals the account witness
  commitment used by the relation;
- the opening tuple account randomizer commitment equals the account randomizer
  commitment used by the relation;
- the opening tuple restore anchor equals `leaf.lifecycle_anchor`;
- the opening tuple pad-seed digest equals `leaf.pad_seed_digest`;
- the opening tuple commitment digest equals `leaf.opening_commitment`;
- the opening tuple is fixed before admission and before any later spend
  challenge for this slot.

### 3. Pad-Seed Opening

- the pad seed or pad proof opens `leaf.pad_seed_digest`;
- the same pad source derives the public pad-opening statement;
- the public pad-opening statement digest is included in the hidden-source
  statement digest;
- a substituted pad seed, substituted digest, or mismatched public pad statement
  rejects or exports a named break.

### 4. Masked-State18 Packed Witness Opening

- the packed witness opening decodes the 32-byte account witness commitment into
  18 q12289 residues;
- residues 0..11 are the checked-core Fq12 syndrome residues;
- residues 12..17 are the six masked active-lane projection residues;
- projection masks are derived from the pad seed, slot id, and
  `reviewed_fq12_packed_witness_state18_projection_mask` domain;
- unmasking yields the six account projection residues;
- repacking the opened residues equals `account_witness_commitment`.

### 5. Active-Lane Hidden-Source Statement

- the hidden-source statement is recomputed canonically from account identity,
  public-key identity, slot id, state/opening/pad commitments, public
  pad-opening statement, selected active-lane profile, verifier version,
  lifecycle anchor, and any pre-admission checked-core context digest;
- the hidden-source statement does not include spend-time `message_digest`,
  `signature_digest`, spend proof commitment, or `slot_authorization_digest`;
- its digest equals `leaf.hidden_source_statement_digest`;
- the statement names exactly the 12 carrier-visible lanes: six account
  projection lanes and six pad-source lanes.

### 6. Projection Carrier Binding

- `projection_carrier_56_bytes` decodes to 12 canonical q12289 lanes, three
  check bytes, and 29 transcript-binding bytes;
- account carrier lanes equal the six unmasked account projection residues;
- pad carrier lanes equal the six pad-source projection residues;
- check bytes match the active-lane relation;
- transcript-binding bytes bind the same opening commitment, statement digest,
  checked-core context digest, verifier version, account id, public-key
  identity, and slot id;
- replaying the carrier under a different account/key/slot/state/verifier
  context rejects or exports a carrier/transcript binding break.

### 7. Admission Namespace

- the account namespace has no already-live slot with the same `(account_id,
  slot_id)`;
- the selected uniqueness domain has no live slot with the same
  `pad_seed_digest`;
- the admission state root and lifecycle anchor match the account-state
  transition that makes this leaf live.

## Soundness Claim For This Relation

If the direct replayable creation verifier accepts, then either
`CreationHiddenSourceRelationV1` holds for the leaf and evidence, or the
adversary exports one of:

- `creation_time_hidden_source_soundness_break`;
- `prevalidation_leaf_binding_break`;
- `qrom_or_domain_separation_break`;
- `prevalidation_data_availability_break`;
- `live_slot_leakage_or_keygen_break`.

This relation feeds the spend theorem only through the admitted leaf and
`slot_authorization_digest`. Spend-time authorization does not re-open the
hidden witness; it checks that the consumed slot was admitted under this
relation.

## Required Negative Controls

The creation-time tests/proof cases must reject or name a break for:

- mutated leaf digest;
- mutated authorization digest;
- wrong account id;
- wrong public-key identity;
- wrong slot id;
- wrong verifier version;
- wrong account state root;
- wrong opening commitment;
- wrong pad-seed digest;
- wrong hidden-source statement digest;
- wrong prevalidation transcript digest;
- missing certificate payload with retained digest;
- substituted witness opening;
- substituted pad seed;
- public pad statement mismatch;
- active-lane projection mismatch;
- carrier replay under another account/key/slot/state root/verifier version;
- precommitment bypass with a relation-valid opening not committed by the leaf;
- duplicate slot id;
- duplicate pad-seed digest.

## Mapping To Existing Evidence

| relation check | existing evidence |
|---|---|
| state-assisted opening tuple | `verify_reviewed_fq12_state_assisted_slot_opening` and state-assisted opening tuple code |
| pad-seed opening | `verify_reviewed_fq12_state_assisted_pad_seed_opening` |
| masked-state18 packed opening | `verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening` |
| projection carrier binding | `verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple` |
| candidate local certificate | `verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate` |
| semantic negative controls | `hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289` |

## First Executable Path 2 Replay Hook

`tools/parameter-search/src/production_reference.rs` now includes
`verify_path2_prevalidated_slot_creation_relation_replay`. It checks the Path 2
leaf/evidence/payload bindings, calls the reviewed Fq12 active-lane masked
projection soundness certificate verifier, and binds the replayed slot id,
statement digest, opening commitment, and pad-seed digest back to the
`PrevalidatedSlotLeafV1` values.

This hook currently reuses the reviewed Fq12 statement as a temporary executable
skeleton. That reuse is intentionally provisional. The reviewed statement frame
includes `message_digest` and `signature_digest`, so it cannot be the final
Path 2 creation leaf statement. The required replacement is a canonical
`CreationHiddenSourceStatementV1` frame that is fixed before the spend challenge
and is bound to spend-time authorization only through the leaf and
`slot_authorization_digest`. First executable progress now defines
`Path2CreationHiddenSourceStatementV1` bytes and a KAT proving message/signature
independence. The replay fixture now binds that digest into the
leaf/evidence/certificate replay path and keeps the reviewed Fq12 statement
digest as separate direct-replay metadata.

Current KATs prove:

- a reviewed active-lane certificate accepts through the Path 2 wrapper;
- wrong pad seed rejects through the active-lane verifier;
- a Path 2 leaf bound to a different statement digest rejects after replay.
- verifier-version and profile rebinding reject at the creation-statement digest
  layer, even when the old reviewed replay payload remains locally available.

This hook is an executable skeleton for the creation verifier, not the final
cryptographic theorem.

## Proof Tracker Closure

This spec closes "State the creation-time hidden-source relation" at the proof
planning level. Executable closure remains open until a direct creation verifier
and relation-specific negative-control matrix are implemented.
