# Account-Creation Proof Mechanism Decision

Status: selected proof mechanism for the first Path 2 theorem pass.

## Decision

Use `direct_replayable_relation_certificate` as the first account-creation proof
mechanism.

Account creation admits a live prevalidated slot only when validators can replay
a concrete creation-time relation certificate against the canonical
`PrevalidatedSlotLeafV1` bindings. The leaf commits to the transcript digest and
payload digest, but those digests are not accepted as proof by themselves.

This mechanism is selected for the proof-first phase because it is the closest
fit to the executable evidence we already have:

- active-lane hidden-source statement binding;
- state-assisted opening tuple binding;
- pad-seed opening binding;
- masked-state18 packed witness opening extraction;
- 56-byte projection-carrier lane/check/transcript binding;
- local fail-closed negative controls for semantic single-field mutations.

## Non-Decision

This is not a final byte-optimized proof system. Creation-time evidence may be
larger than the spend signature. Path 2 optimizes spend transactions by moving
hidden-source validation to slot admission; it does not require the admission
certificate to fit in the spend envelope.

## Mechanism Shape

```text
verify_prevalidated_slot_creation(
  PrevalidatedSlotLeafV1,
  AccountCreationEvidenceV1 {
    evidence_kind = direct_replayable_relation_certificate,
    payload = DirectReplayableRelationCertificateV1
  }
) -> accept/reject
```

Acceptance means:

1. the leaf frame is canonical;
2. the evidence envelope duplicates and matches every binding field in the leaf;
3. the prevalidation transcript digest is recomputed from the evidence envelope;
4. the relation certificate payload is available and replayable;
5. replay verifies the creation-time hidden-source relation;
6. the account-state admission transition records the exact leaf digest and
   authorization digest as live.

## DirectReplayableRelationCertificateV1

The first proof target certificate must include or deterministically reconstruct:

| component | role |
|---|---|
| `canonical_leaf_bytes` | exact `PrevalidatedSlotLeafV1` preimage |
| `slot_authorization_digest` | normalized spend authorization digest |
| `state_assisted_opening_tuple` | prechallenge opening tuple bound to slot/account |
| `hidden_source_statement` | creation-time active-lane public relation statement; no spend message/signature digest |
| `hidden_source_statement_digest` | must match the leaf |
| `account_witness_commitment` | witness commitment named by account state |
| `account_randomizer_commitment` | randomizer/pad commitment named by account state |
| `pad_seed_opening_digest` | must match the leaf `pad_seed_digest` |
| `account_witness_projection_opening_digest` | selected active-lane witness opening digest |
| `masked_state18_packed_witness_opening` | replayable opening/certificate for 12 checked-core residues plus six masked projection residues |
| `projection_carrier_56_bytes` | current executable carrier bytes |
| `carrier_lanes_check_transcript` | decoded lane/check/transcript binding |
| `checked_core_context_digest` | digest of the checked-core context the carrier is bound to at admission |
| `verifier_version_digest` | must match the leaf |
| `creation_profile_digest` | profile/domain binding for the creation verifier |

The first executable wrapper slice additionally defines a public binding
certificate digest over account id, public-key identity, slot id, verifier
version, state root, opening commitment, pad-seed digest, hidden-source
statement digest, lifecycle anchor, relation transcript digest, and a public
replay-payload digest. This closes only payload availability and cross-leaf
payload binding at the frame level; it does not yet replay the hidden-source
relation itself.

The next executable slice wires this public binding certificate to the existing
reviewed Fq12 active-lane verifier. `verify_path2_prevalidated_slot_creation_relation_replay`
first checks Path 2 leaf/evidence/payload bindings, then replays
`verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate`,
and finally checks that replayed slot id, statement digest, opening commitment,
and pad-seed digest equal the leaf. This is still not final proof discharge, but
it is the first concrete creation-verifier skeleton.

This executable skeleton is deliberately not the final Path 2 statement model.
The reviewed Fq12 active-lane statement frame contains spend-time
`message_digest` and `signature_digest`. A final direct replayable creation
certificate must instead bind a non-circular `CreationHiddenSourceStatementV1`
and leave message/signature binding to the spend-time checked-core challenge.
The executable Path 2 fixture now follows that split: the leaf-bound hidden
source digest is `Path2CreationHiddenSourceStatementV1`, while the reviewed Fq12
statement digest is only a direct-replay payload binding.

The certificate may store full component bytes or store a replayable reference
to them, but data availability is mandatory. If validators cannot replay the
payload, creation admission fails closed.

## What The Creation Verifier Checks

The direct verifier checks:

- `slot_leaf_digest == SHA256(PrevalidatedSlotLeafV1_bytes)`;
- `slot_authorization_digest == SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || leaf)`;
- all evidence envelope binding fields match the leaf;
- leaf `prevalidation_transcript_digest` equals the digest over the evidence
  envelope and certificate payload digest;
- opening commitment matches the state-assisted opening tuple;
- account witness/randomizer commitments match the account identity/context;
- pad seed opens to the leaf `pad_seed_digest`;
- public pad-opening statement is derived from the same pad source;
- active-lane hidden-source statement digest matches the leaf;
- masked-state18 packed witness opening repacks to the account witness
  commitment;
- projection carrier lanes/check/transcript bind the extracted opening, pad
  statement, slot id, verifier version, checked-core context, and opening
  commitment;
- duplicate slot id and duplicate pad-seed digest are rejected in the chosen
  account-state namespace.

## Fallbacks Kept Open

These mechanisms remain reserved but are not selected for the first proof pass:

| fallback | when to revisit |
|---|---|
| `state_stored_full_carrier` | if replayable certificates are too complex but storing 56 extra bytes per slot is acceptable |
| `certificate_root_and_availability_record` | if direct evidence must be pruned from state but validators still need replayable data |
| `future_compact_proof_handle` | if a compact proof system is later instantiated for the same creation relation |
| `aggregated_batch_admission_handle` | only after direct single-slot theorem is complete and `optional_aggregation_soundness` is instantiated |

## Rejected Mechanisms For First Proof Pass

- Hash-only prevalidation leaf. A digest without replayable evidence is not proof
  semantics.
- Spend-time raw `pad_seed` reveal. It changes leakage and increases spend size.
- Simple extra carrier lanes. The rank probe shows no superlinear proof leverage.
- Spend-only full-witness extraction from the 308-byte transcript. The existing
  negative control shows the current byte relation does not imply that claim.

## Proof Obligation Created By This Decision

The next theorem target becomes:

```text
direct_replayable_relation_certificate_soundness_or_break
```

If `verify_prevalidated_slot_creation` accepts a
`direct_replayable_relation_certificate`, then either:

- the active-lane creation-time hidden-source relation is valid for the leaf; or
- the adversary exports one of:
  - `creation_time_hidden_source_soundness_break`;
  - `prevalidation_leaf_binding_break`;
  - `qrom_or_domain_separation_break`;
  - `prevalidation_data_availability_break`;
  - `live_slot_leakage_or_keygen_break`.

This theorem feeds the larger
`account_creation_prevalidated_slot_authorization_or_break` theorem.

## Negative Controls Required Next

The creation-time negative-control matrix must include:

- mutate `slot_leaf_digest`;
- mutate `slot_authorization_digest`;
- mutate account id;
- mutate public-key identity;
- mutate slot id;
- mutate verifier version;
- mutate account state root;
- mutate opening commitment;
- mutate pad-seed digest;
- mutate hidden-source statement digest;
- mutate prevalidation transcript digest;
- mutate relation payload digest;
- substitute witness opening;
- substitute pad seed;
- mismatch public pad statement;
- mismatch relation projection;
- replay carrier context under another account/key/slot/state root;
- bypass precommitment with a valid relation opening not committed by the leaf;
- remove certificate payload while keeping its digest.

## Proof Tracker Closure

This decision closes "Decide the creation-time proof mechanism" at the proof
planning level. It opens the concrete relation-spec and soundness-theorem tasks
for `DirectReplayableRelationCertificateV1`.
