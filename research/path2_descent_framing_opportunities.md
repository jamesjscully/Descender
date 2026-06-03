# Path 2 Descent-Framing Opportunities

Status: proof-organization note.

## Purpose

Path 2 has one primary descent/SIS target:

```text
Path2DescentSisCommitmentV1
```

This note records where the same descent framing should also be used elsewhere
in the procedure, and where it should not be forced.

The useful pattern is:

```text
public quotient / handle
  -> local representative or proof object
  -> glues to canonical hidden/account/state meaning
  -> or exports a named break
```

## Strong Opportunities

### 1. Account-Creation Proof Payload

This is the main target.

Public object:

```text
Path2CreationProofBindingV1
Path2DirectCreationProofPayloadV1
PrevalidatedSlotLeafV2.creation_proof_binding_digest
```

Local representative:

```text
committed pad lanes
committed account projection lanes
committed opening tuple scalars
packed masked projection residues
```

Gluing claim:

```text
accepted local q12289 relation descends to a valid hidden account-opening
relation for the leaf, or exports SIS binding, hiding, link-equation,
public-projection, sampler, or QROM break.
```

This should be the first concrete instantiation.

### 2. Direct/Batch/Aggregated Admission Equivalence

Optional aggregation is naturally a descent problem.

Public quotient:

```text
slot_authorization_digest
```

Representatives:

```text
DirectRecord(slot_leaf)
InclusionProof(slot_leaf, account_root, path)
AggregatedProofHandle(slot_leaf, batch_root, proof_ref)
```

Gluing claim:

```text
any accepted representation glues to the same canonical leaf meaning, or
exports optional_aggregation_soundness, data-availability, ordering, duplicate,
or inclusion-binding break.
```

This is inactive for direct deployment, but the theorem mode switch should use
this framing when aggregation is enabled.

### 3. Live-Slot Leakage And Public-Projection Guard

The old quotient-link research showed that public link values can condition the
hidden sampler. The same idea applies to Path 2 public admission bytes.

Public quotient:

```text
leaf + creation_proof_binding_digest + public commitments + proof transcript
```

Forbidden representatives:

```text
raw pad seed
projection masks
unmasked account projection residues
private opening frames
public linear link rows that condition the hidden sampler
```

Gluing/hiding claim:

```text
the public view is either simulatable from the allowed quotient view, or exports
publicProjectionBreak / samplerHidingBreak / live_slot_leakage_or_keygen_break.
```

This should be folded into the Path 2 payload no-export and leakage tests once
the concrete descent/SIS commitment is chosen.

## Moderate Opportunities

### 4. Creation Evidence Data Availability

Data availability can be phrased as descent from a digest/root to a concrete
representative:

```text
public_replay_payload_digest -> available payload bytes -> verified relation
```

This is useful for batch/rollup modes. In direct mode, current fail-closed
binding checks are probably enough; descent language should only be added if it
clarifies a future root/inclusion proof.

### 5. Consume-Once State Transition

A spend consumes a public capability:

```text
slot_authorization_digest
```

and must glue to exactly one state transition:

```text
unconsumed live leaf -> consumed successor state
```

This can be modeled as a small descent-style state transition theorem, but it
is less urgent because the current local consume-once model is already simple
and executable. Use descent framing only if rollback/reorg proofs become more
abstract.

## Weak Or Non-Opportunities

### Checked-Core Message Binding

The checked-core verifier is already a linear sigma transcript with a concrete
challenge frame. It has related structure, but the immediate theorem work is
challenge binding and equation verification, not hidden quotient gluing.

Do not refactor this unless we later formalize the whole checked-core verifier
through `CommittedSigma`.

### Canonical Leaf And Domain Binding

Canonical leaf bytes, transcript digests, and domain separation are ordinary
deterministic binding/hash claims. Calling them descent would obscure the
simpler invariant:

```text
one canonical byte frame -> one public meaning, unless hash/domain binding
breaks.
```

Keep these as frame-binding KATs and hash/QROM ledger items.

### Production Lifecycle Engineering

Restore anchors, local storage locks, canonical DB integration, and reorg
policy are deployment engineering and state-machine assumptions. They can feed
a gluing theorem, but they are not themselves the cryptographic descent
commitment problem.

## Recommended Use

Use descent framing for:

- Path 2 hidden-source creation payload;
- optional direct/batch/aggregate admission equivalence;
- public-projection/sampler-hiding leakage guard;
- future data-availability roots if direct payload bytes are not stored.

Avoid descent framing for:

- byte-frame canonicalization;
- simple SHA/domain binding;
- already-concrete checked-core equation checks;
- storage/lifecycle implementation details.

## Tracker Impact

The immediate proof tracker should keep `Path2DescentSisCommitmentV1` as the
top blocker. Other descent opportunities are secondary organization work. They
should not expand the direct-mode deployment scope or activate aggregation
assumptions.
