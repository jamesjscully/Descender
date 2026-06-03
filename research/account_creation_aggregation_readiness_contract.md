# Account-Creation Aggregation Readiness Contract

Status: optional aggregation model contract for the Path 2 planning stage.

## Purpose

This contract bakes aggregation-compatible assumptions into the account-creation
model without making aggregation required for the first deployable verifier.

The rule is simple: every single-slot verifier must remain complete and secure
on its own, but every accepted single-slot object must also have a canonical
leaf form that can later be admitted, included, consumed, or proven inside a
batch/rollup/accumulator transcript.

## Non-Requirement

Aggregation is not a deployment prerequisite.

The baseline deployment mode is:

```text
single slot creation -> direct consensus-visible slot record
single slot spend    -> direct state lookup and slot consumption
```

The optional aggregation mode is:

```text
batch slot creation -> root/proof over canonical slot leaves
batch slot spend    -> aggregated inclusion/consume-once proof
```

The single-slot theorem must not depend on the aggregation proof system. The
aggregation theorem imports the single-slot theorem and adds accumulator,
batch-proof, or rollup-soundness assumptions only when those features are
enabled.

## Canonical Slot Leaf

Every prevalidated slot is represented by a canonical leaf. The fixed byte frame
is specified in `research/prevalidated_slot_leaf_v1_frame.md`; the field list is:

```text
PrevalidatedSlotLeafV1 =
  domain_separator
  profile_version
  account_id
  public_key_identity
  slot_id
  verifier_version
  account_state_root_before
  opening_commitment
  pad_seed_digest
  hidden_source_statement_digest
  prevalidation_transcript_digest
  lifecycle_anchor
  live_status_marker
```

The exact byte frame remains to be fixed, but the field list is mandatory unless
a later theorem explicitly removes a field and charges the resulting assumption.

## Admission Modes

There are two admissible ways for a slot to become live.

### Direct Admission

```text
verify_prevalidated_slot_creation(slot_leaf, creation_evidence) -> accept
```

This is the required baseline. Validators can replay the slot creation evidence
or check a consensus-visible record that was produced by this verifier.

### Batch Admission

```text
verify_prevalidated_slot_batch(root_update, ordered_slot_leaves, batch_proof)
  -> accept
```

This is optional. It is valid only if every admitted leaf has the same canonical
meaning as direct admission would have given it.

Batch admission cannot weaken the single-slot relation. It can only replace many
direct checks with one proof or one root update under an explicitly named
aggregation assumption.

## Authorization Handle

Spend-time verification consumes a normalized authorization digest:

```text
slot_authorization_digest =
  H(DESCENDER_SLOT_AUTHORIZATION_V1 || canonical_slot_leaf)
```

The checked-core spend frame binds this digest, not the concrete representation
used to prove it.

Allowed representations:

- `DirectRecord(slot_leaf)`;
- `InclusionProof(slot_leaf, account_root, path)`;
- `AggregatedProofHandle(slot_leaf, batch_root, proof_ref)`.

The spend theorem is representation-independent after the handle is validated.

## Domain Separation

Reserve these domains before implementation:

```text
DESCENDER_SLOT_LEAF_V1
DESCENDER_SLOT_AUTHORIZATION_V1
DESCENDER_SLOT_PREVALIDATE_SINGLE_V1
DESCENDER_SLOT_PREVALIDATE_BATCH_V1
DESCENDER_SLOT_ROOT_UPDATE_V1
DESCENDER_SLOT_CONSUME_SINGLE_V1
DESCENDER_SLOT_CONSUME_BATCH_V1
```

No domain may be reused between direct and batch admission unless the byte frame
is identical and the theorem explicitly states the equivalence.

## Canonical Batch Semantics

Batch admission must be deterministic:

- sort leaves by `(account_id, slot_id)`;
- reject duplicate `(account_id, slot_id)`;
- reject duplicate `pad_seed_digest` within the checked uniqueness domain;
- reject duplicate prevalidation transcript digests unless explicitly allowed by
  a collision/binding theorem;
- reject cross-account inclusion unless the leaf account id matches the account
  root namespace;
- produce a deterministic root update;
- bind the ordered leaf digest list into the batch transcript;
- make reordering either byte-identical after canonical sorting or rejected.

## Optional Assumption Class

Add a carried but inactive assumption class:

```text
optional_aggregation_soundness
```

It is inactive in direct single-slot deployment.

It becomes active only when one of these is enabled:

- batch slot-creation admission;
- accumulator-backed account roots;
- rollup/block proof of many slot creations;
- aggregated spend inclusion or consume-once proof.

When active, the final theorem must name the concrete proof system or
accumulator and add its soundness, binding, data-availability, and transcript
domain assumptions.

## Required Negative Controls

The aggregation-ready model must include negative controls for:

- wrong account root;
- wrong batch root;
- wrong slot id;
- wrong public-key identity;
- wrong verifier version;
- wrong opening commitment;
- wrong `pad_seed_digest`;
- wrong hidden-source statement digest;
- stale leaf;
- missing leaf;
- duplicate leaf;
- duplicate `pad_seed_digest`;
- reordered batch with noncanonical transcript;
- cross-account inclusion;
- direct-record acceptance with only a batch-domain transcript;
- batch-record acceptance with only a single-slot-domain transcript;
- spend challenge bound to a different `slot_authorization_digest`.

## Theorem Modes

The deployable theorem has two modes.

### Mode A: Direct Single-Slot

Required for first deployment.

Assumptions:

- account-creation hidden-source soundness;
- direct prevalidation registry binding;
- checked-core message binding;
- lifecycle consume-once semantics;
- hash/domain assumptions already charged by the base scheme.

No aggregation assumption is consumed.

### Mode B: Aggregated Admission Or Consumption

Optional.

Assumptions:

- all Mode A assumptions;
- `optional_aggregation_soundness` instantiated by a concrete accumulator,
  rollup proof, or batch proof system;
- data availability for any prevalidation records not directly stored in
  consensus-visible state;
- canonical batch transcript and ordering soundness.

## Planning Consequence

The first implementation should build direct single-slot creation and direct
single-slot spend, but its byte frames should already use the canonical leaf,
authorization digest, and domain names above. That prevents a later aggregation
upgrade from requiring a signature-format redesign.
