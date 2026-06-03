# Path 2 Live-Slot Leakage Ledger

Status: direct-mode leakage ledger for account-creation / prevalidated-slot
authorization. The direct public admission view is executable; hidden-source
relation soundness remains a separate fail-closed proof boundary.

## Scope

This ledger charges the public view created when a prevalidated slot becomes
live. The charge applies even if the slot is never spent.

Path 2 moves hidden-source validation from spend time to account creation. It
does not make hidden-source material free. Any value published at admission must
be either:

- ordinary public metadata;
- a binding digest or commitment;
- masked live-slot leakage covered by the mask-PRF theorem;
- a zero-knowledge or hiding proof/certificate view; or
- explicitly charged as public opening leakage, which changes the theorem.

## Public At Slot Admission

The canonical public admission object is `PrevalidatedSlotLeafV1` or a direct
record/root that lets validators reconstruct the same leaf. The full leaf
preimage is 353 bytes; an implementation may store only its digest/root, but the
theorem charges the fields as public when the chain makes them consensus-visible.

| value | size | charge | rule |
|---|---:|---|---|
| `profile_version_digest` | 32 B | public version metadata | binds the selected deployment profile |
| `account_id` | 32 B | public account namespace | prevents cross-account reuse |
| `public_key_identity` | 32 B | public key binding | binds the slot to the verifying key |
| `slot_id` | 8 B | public slot metadata | one-use capability index |
| `verifier_version_digest` | 32 B | public version metadata | prevents old-verifier replay |
| `account_state_root_before` | 32 B | public state context | fixes the admission state |
| `opening_commitment` | 32 B | public commitment | binds the state-assisted opening tuple |
| `pad_seed_digest` | 32 B | public digest only | binds the pad seed without revealing it |
| `hidden_source_statement_digest` | 32 B | public relation binding | binds the active-lane statement |
| `prevalidation_transcript_digest` | 32 B | public transcript binding | binds checked evidence, not proof semantics by itself |
| `lifecycle_anchor` | 32 B | public lifecycle metadata | binds restore/rollback context |
| `live_status_marker` | 1 B | public state bit | distinguishes live leaves from reserved/spent records |
| `slot_authorization_digest` | 32 B | public authorization binding | value consumed by the checked-core spend frame |

The leaf domain and frame version are public constants. Their byte cost is part
of the 353-byte preimage but not counted as secret leakage.

## Public Evidence Metadata

`AccountCreationEvidenceV1` duplicates binding fields and adds:

| value | charge | rule |
|---|---|---|
| `evidence_domain` and `evidence_version` | public domain/version metadata | must be `DESCENDER_SLOT_PREVALIDATE_SINGLE_V1` in direct mode |
| `evidence_kind` | public mechanism selector | direct theorem enables `direct_replayable_relation_certificate` |
| `relation_transcript_digest` | public digest | binds relation replay transcript |
| `evidence_payload_digest` | public digest | binds the replayable payload or proof bytes |

A digest or root is not proof semantics. Creation admission is valid only when
the verifier can check the payload or an explicitly named proof system for the
same relation.

## Acceptable Replay Payload Views

The selected first proof mechanism is
`direct_replayable_relation_certificate`. For leakage accounting, this phrase
must not be read as permission to publish arbitrary private openings.

Acceptable public payload material:

- canonical public hidden-source statement bytes;
- public account witness and randomizer commitments;
- public pad-opening statement or its digest, if the statement is already
  classified as public for the chosen profile;
- projection carrier bytes only when they are a public relation statement or a
  hiding certificate view, not raw secret openings;
- check bytes, transcript bindings, verifier/profile metadata, and domain tags;
- zero-knowledge or hiding proof bytes for pad/witness openings;
- availability references for the same public proof/certificate bytes.

Forbidden public payload material before spend unless the theorem is changed:

- raw `pad_seed`;
- projection-mask residues;
- six unmasked selected account projection residues;
- `account_witness_projection_opening_frame` carrying unmasked residues;
- raw `masked_state18_packed_witness_opening` if it exposes account projection
  residues or mask residues;
- any state-assisted opening tuple field that reveals witness secret material
  rather than only a commitment/proof view.

If the first implementation of `direct_replayable_relation_certificate` exposes
any forbidden value, the live-slot theorem must switch from
`pre_spend_hiding_preserved` to `public_creation_opening_model`, and the system
cannot claim the previous live-slot hiding budget.

## Hidden Before Spend

These values remain hidden from the public live-slot view:

- raw pad seed;
- pad PRF output not derivable from public data;
- projection-mask residues;
- unmasked selected account projection residues;
- private account witness and randomizer openings;
- private witness-opening randomness;
- private proof witnesses used by any compact or hiding creation proof.

The ledger therefore carries the same mask-PRF intuition as the earlier
308-byte route: public masked projection state may be simulated as uniform under
the projection-mask PRF assumption, but unmasked projection residues are not
allowed in the pre-spend public view.

## Public At Spend

Spend-time verification may reveal:

- checked-core Fq12 signature body;
- message/challenge/public-key binding material;
- `slot_authorization_digest`;
- slot id and account-state transition metadata;
- live-to-consumed or burn status.

Path 2 spend verification does not re-open the hidden-source witness. It checks
that the consumed slot was already admitted. Raw pad seed and unmasked account
projection residues remain forbidden in the spend transaction unless a later
theorem explicitly spends an opening budget.

## Never-Spent Slots

Never-spent live slots still reveal:

- all consensus-visible leaf metadata;
- all public creation-evidence metadata;
- all public certificate/proof bytes made available for data availability;
- lifecycle and state-root placement.

They do not reveal:

- the raw pad seed;
- unmasked selected account projection residues;
- private opening frames;
- private proof witnesses.

This is the main difference from spend-only accounting: leakage is charged at
admission, not only on successful spend.

## Hash, PRF, And Binding Assumptions Charged Here

| assumption | role |
|---|---|
| SHA-256 collision/preimage resistance for `prevalidated_slot_leaf_digest` | prevents two leaf meanings for one digest |
| SHA-256 binding for `slot_authorization_digest` | binds the checked-core spend frame to one canonical leaf |
| SHA-256 binding for `prevalidation_transcript_digest` | binds the evidence envelope and payload digest |
| SHA-256 binding for `hidden_source_statement_digest` | binds the active-lane public relation statement |
| SHA-256 preimage resistance for `pad_seed_digest` | hides raw pad seed from digest-only live slots |
| projection-mask PRF assumption | hides selected projection residues behind masked live-slot state |
| pad PRF/domain separation | prevents reuse or cross-domain derivation of pad-source material |
| domain separation across single/batch/admit/consume frames | prevents direct/batch and creation/spend transcript confusion |

Query budgets are not assigned in this ledger. They belong in the separate
Path 2 hash/QROM assumption ledger.

## Optional Aggregation View

Aggregation does not change what a leaf means. It may change what observers see:

- direct mode may expose full leaf fields or a direct consensus-visible record;
- inclusion mode may expose the leaf plus an account-root path;
- aggregated mode may expose a batch root, proof handle, and availability
  reference.

If aggregation hides individual leaf fields from ordinary observers but
validators can recover or verify them, the direct theorem still charges the leaf
as public to validators. Any stronger privacy claim for batch roots is a
separate theorem and consumes `optional_aggregation_soundness` plus data
availability assumptions.

## Required Negative Controls

The creation-time and spend-time KAT plan must include controls that prove:

- no public leaf or evidence field contains raw `pad_seed`;
- no public leaf or evidence field contains projection-mask residues;
- no public leaf or evidence field contains unmasked selected account projection
  residues;
- missing certificate payload fails closed even when its digest is present;
- a payload digest for one account/key/slot/state root cannot authorize another;
- direct-domain evidence is not accepted as batch evidence, and batch-domain
  evidence is not accepted as direct evidence;
- rejecting verifier paths do not emit private opening material in diagnostics.

## Current Conclusion

Path 2 preserves the intended pre-spend hiding claim only under a constrained
certificate API: public replay must be a public relation/proof view, not a raw
private-opening dump.

## Executable Public-View Closure

`tools/parameter-search/src/production_reference.rs` now exposes
`path2_public_admission_view_v1` and `path2_public_admission_view_v1_json` as
the public direct-admission serialization surface. The API first verifies
leaf/evidence/certificate payload bindings, then serializes only:

- the public schema label;
- `slot_leaf_digest`;
- `slot_authorization_digest`;
- `PrevalidatedSlotLeafV1`;
- `AccountCreationEvidenceV1`;
- `Path2DirectReplayableRelationCertificateV1`.

The certificate object contains digest and public replay-handle fields only,
including `public_replay_payload_digest` and
`reviewed_active_lane_statement_digest`; it does not serialize raw pad seed,
projection masks, unmasked account projection residues, private opening frames,
or raw replay payload bytes.

KAT `path2_public_admission_view_serializes_only_public_material` proves:

- the public view includes expected digest/public handle fields;
- selected-projection private material markers are absent from JSON output;
- serialization fails closed if evidence payload bindings are mutated.

KAT `path2_creation_relation_replay_rejections_do_not_export_private_material`
proves representative Path 2 relation-replay rejection paths keep raw pad seed,
projection masks, unmasked projection residues, and private opening-frame
markers out of errors/debug output.

The immediate proof consequence is:

```text
pre_spend_hiding_preserved_or_public_creation_opening_break
```

If creation admission accepts and the public admission view contains no
forbidden payload material, then the live-slot leakage is bounded by the public
leaf/evidence metadata, digest bindings, and mask-PRF leakage terms listed
above. If forbidden material is public, the theorem must either explicitly
charge that disclosure or reject the design as not deployable under the current
Path 2 hiding model.

For the current direct public view, the standard pre-spend hiding claim is
preserved at the serialization/error boundary. This does not by itself prove
`direct_replayable_relation_certificate_soundness_or_break`; it only proves the
public Path 2 object and representative reject paths do not expand the leakage
mode beyond the ledger.
