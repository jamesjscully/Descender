# AccountCreationEvidenceV1 Spec

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting interface spec note, not a normative spec. Current
> normative authority starts at `../spec/README.md`.

Status: proof-interface specification for Path 2.

This document defines the creation-time evidence object consumed by
`verify_prevalidated_slot_creation`. It does not by itself decide the final proof
mechanism. Its purpose is to make the theorem interface explicit and prevent the
prevalidation leaf from degenerating into a bare unchecked digest.

## Design Rule

`PrevalidatedSlotLeafV1.prevalidation_transcript_digest` is not proof
semantics. It is only a binding handle.

The proof obligation lives in:

```text
verify_prevalidated_slot_creation(slot_leaf, account_creation_evidence)
```

If that verifier accepts, then the creation-time hidden-source relation must be
valid for the slot, or a named break must be exported.

## Evidence Envelope

`AccountCreationEvidenceV1` is a proof-level envelope with these fields:

| field | role |
|---|---|
| `evidence_domain` | ASCII `DESCENDER_SLOT_PREVALIDATE_SINGLE_V1` for direct single-slot admission |
| `evidence_version` | `u16`, value `1` |
| `evidence_kind` | proof mechanism selector |
| `slot_leaf_digest` | SHA-256 digest of `PrevalidatedSlotLeafV1_bytes` |
| `slot_authorization_digest` | normalized spend authorization digest |
| `creation_verifier_version_digest` | digest of the verifier that checks this evidence |
| `hidden_source_statement_digest` | must equal the digest in the leaf |
| `opening_commitment` | must equal the commitment in the leaf |
| `pad_seed_digest` | must equal the digest in the leaf |
| `account_identity_digest` | must equal the leaf account id |
| `public_key_identity` | must equal the leaf public-key identity |
| `slot_id` | must equal the leaf slot id |
| `state_root_before` | must equal the leaf state root before admission |
| `lifecycle_anchor` | must equal the leaf lifecycle anchor |
| `relation_transcript_digest` | digest of the checked creation-time relation transcript |
| `evidence_payload_digest` | digest of the concrete proof/certificate payload |

The leaf field `prevalidation_transcript_digest` must equal a domain-separated
digest over at least:

```text
evidence_domain
evidence_version
evidence_kind
creation_verifier_version_digest
hidden_source_statement_digest
opening_commitment
pad_seed_digest
account_identity_digest
public_key_identity
slot_id
state_root_before
lifecycle_anchor
relation_transcript_digest
evidence_payload_digest
```

It must not include `slot_leaf_digest`, because `slot_leaf_digest` hashes the
full leaf, including `prevalidation_transcript_digest`. Including it would make
the evidence transcript definition circular. The final `slot_leaf_digest` is
checked after the transcript digest has been computed and inserted into the
leaf.

## Evidence Kinds

The proof interface reserves these evidence kinds:

| kind | status | meaning |
|---|---|---|
| `direct_replayable_relation_certificate` | preferred first proof target | validators can replay a creation-time relation certificate against public commitments and proof material |
| `state_stored_full_carrier` | fallback | the account state stores the full 56-byte carrier and the verifier checks it at creation time |
| `certificate_root_and_availability_record` | fallback | the leaf binds a certificate root, and data availability gives validators the replayable certificate |
| `future_compact_proof_handle` | reserved | a later compact proof system proves the same creation relation |
| `aggregated_batch_admission_handle` | optional | only valid under `optional_aggregation_soundness` and the batch-domain theorem mode |

Only `direct_replayable_relation_certificate` is the preferred proof target for
the next proof step. The others remain reserved so the leaf and authorization
frames do not need redesign if the mechanism changes.

## Creation Verifier Checks

For direct single-slot admission, `verify_prevalidated_slot_creation` must check:

- the evidence domain is `DESCENDER_SLOT_PREVALIDATE_SINGLE_V1`;
- the evidence kind is one enabled by the direct theorem mode;
- `slot_leaf_digest` matches the supplied `PrevalidatedSlotLeafV1`;
- `slot_authorization_digest` matches the supplied leaf;
- every duplicated binding field matches the leaf exactly;
- `prevalidation_transcript_digest` in the leaf matches the evidence transcript
  binding rule;
- the concrete evidence payload verifies the creation-time hidden-source
  relation for the statement/opening/pad/account commitments named by the leaf;
- failure returns a public named rejection/break class, not secret material.

## Relation Payload For Preferred First Target

For `direct_replayable_relation_certificate`, the payload must bind:

- state-assisted opening tuple commitment;
- account witness commitment;
- account randomizer commitment;
- pad-seed opening commitment/digest;
- public pad-opening statement digest;
- creation-time active-lane hidden-source statement digest;
- reviewed Fq12 active-lane statement digest, if the direct replay payload uses
  the older signature-dependent verifier as a negative-control source;
- account-witness projection-opening digest;
- carrier/check/transcript material or a replayable certificate of the same
  active-lane relation;
- verifier version and profile;
- account id, public-key identity, slot id, and state root before admission.

The payload may be large at account creation. Path 2 optimizes spend size; it
does not require creation evidence to fit inside the spend envelope.

The first executable frame-binding slice models this payload as
`Path2DirectReplayableRelationCertificateV1`, whose digest is the
`evidence_payload_digest`. That certificate duplicates account id, public-key
identity, slot id, verifier version, state root, opening commitment,
`pad_seed_digest`, creation hidden-source statement digest, reviewed active-lane
statement digest, lifecycle anchor, relation transcript digest, and a public
replay-payload digest. The creation statement digest is what enters the leaf;
the reviewed active-lane statement digest is only replay metadata for the
temporary/direct certificate path.

## What Is Committed vs Checked

Checked by the creation verifier:

- hidden-source relation validity for the chosen evidence kind;
- all binding equalities between evidence and leaf;
- verifier/profile/domain agreement;
- live-slot admission preconditions;
- duplicate slot/pad-seed policy where the state model exposes it.

Committed by the leaf:

- digest of the checked transcript;
- digest of the concrete evidence payload;
- account, key, slot, state, opening, pad, statement, and lifecycle bindings.

Not sufficient:

- a standalone `prevalidation_transcript_digest`;
- a standalone certificate root with no data availability rule;
- a replayable carrier that is not bound to account id, key identity, verifier
  version, state root, opening commitment, and pad-seed digest.

## Named Breaks

If creation admission accepts invalid evidence, the theorem exports one of:

- `creation_time_hidden_source_soundness_break`;
- `prevalidation_leaf_binding_break`;
- `qrom_or_domain_separation_break`;
- `prevalidation_data_availability_break`;
- `live_slot_leakage_or_keygen_break`.

## Proof Tracker Closure

This spec closes the proof vocabulary item "Define creation-time evidence
object" at the interface level. The next open proof item is choosing and
formalizing the concrete creation-time proof mechanism, with
`direct_replayable_relation_certificate` as the current preferred target.
