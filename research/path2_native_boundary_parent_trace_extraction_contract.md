# Path 2 Native Boundary Parent Trace Extraction Contract

Status: extractor contract pinned; concrete proof family not selected.

Purpose: state the exact cryptographic theorem needed for
`ParentTraceExtractionSoundness`.

This is now the main hard assumption in the active path.

## Contract

For a public native boundary `b`, transparent view `view`, and role-complete
profile `profile`, a proof family satisfies parent-trace extraction if:

```text
TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
ProfileRoleComplete(profile)
UsesSelectedTraceKnowledgeModes(profile)
  =>
exists trace.
  TransparentExtractsTrace(b, view, trace)
  TraceOpensBoundary(b, profile, trace)
  TraceRelationValid(b, profile, trace)
```

or the adversary exports one of:

- `transparentProofSoundnessBreak`;
- `inner_commitment_binding_break`;
- `boundary_binding_break`;
- `qrom_or_domain_separation_break`.

## Extracted Trace Must Include

The extracted trace is not a list of verifier-opened cells. It must contain:

| field | required meaning |
|---|---|
| `parentSectionRoot` | exactly `p_section_root(b)` |
| `roleProfileDigest` | exactly `p_role_profile(b)` |
| `roleHandles` | exactly `p_role_handles(b)` |
| `traceWitness` | complete hidden/public trace columns needed by mandatory roles |
| `constraintWitness` | quotient/constraint trace columns proving relation validity |
| `maskWitness` | masks and mask-link witnesses for opened masked values |
| `rangeWitness` | hidden range/digit witnesses for selected range mode |
| `blindingWitness` | extractor-side blinding/noise auxiliaries and bounds |

The verifier does not see these raw fields. They are extractor outputs.

## What The Proof Family May Use

The extractor theorem may rely on:

- binding of the parent trace commitment/root;
- binding of typed role restrictions under that parent root;
- low-degree/proximity/list-decoding soundness, if the proof family has it;
- Fiat-Shamir/QROM forking or rewinding with the pinned schedule;
- collision resistance/domain separation for transcript and commitment roots;
- proof-family-specific knowledge soundness.

The theorem may not rely on:

- public raw openings of hidden role cells;
- assuming the prover honestly committed to unqueried cells;
- reconstructing unqueried cells from sampled openings without a proximity or
  knowledge-extraction theorem;
- post-query choice of role handles, profile, mask domains, or boundary digest.

## Sufficient Proof Shapes

The following theorem shapes are sufficient in principle:

### Full Trace Knowledge Extraction

```text
Accept(root, relation, transcript)
  =>
exists full_trace.
  Opens(root, full_trace)
  RelationHolds(full_trace)
```

plus role restriction binding and no-export.

### Polynomial/Oracle Extraction With Reconstruction

```text
Accept(oracle_commitment, proximity_proof)
  =>
exists low_degree_oracle.
  CloseToCommittedOracle(low_degree_oracle)
  RelationHolds(low_degree_oracle)
```

provided the reconstruction uniquely fixes all mandatory role columns under the
parent root, or failure exports a list/proximity soundness break.

### MPC-in-the-Head Witness Extraction

```text
Accept(views, commitments)
  =>
exists witness.
  RelationHolds(witness)
```

provided the witness can be mapped into `ExtractedTrace` and the opened views
are simulator-safe.

## Insufficient Proof Shapes

The following are not sufficient:

| shape | why it fails |
|---|---|
| sampled Merkle openings only | unqueried private role columns are not extracted |
| local consistency at sampled rows only | no full trace or role-complete witness |
| public masked values only | masks hide the semantic role values from both verifier and extractor unless the committed trace is extracted |
| hash digest of hidden role data | digest binds bytes but gives no extractor |
| public full-rank linear role commitments | extracts/leaks role vector under current dimensions, violating no-export |
| simulator-only hidden roles with no extractor witness | cannot establish `ExtractedRolesComplete` |

## Negative Controls

A candidate proof family must reject or export a break if:

- `parentSectionRoot != p_section_root(b)`;
- `roleProfileDigest != p_role_profile(b)`;
- any extracted role handle differs from `p_role_handles(b)`;
- the accepted proof has only sampled masked openings and no full trace
  extraction theorem;
- a hidden role is opened raw in public proof bytes;
- an unqueried mandatory role is absent from the extracted trace;
- trace reconstruction admits two safety-distinct traces under one parent root;
- query indices were derived before root/profile/role handles/mask domains were
  bound.

## Current Assessment

This contract is plausible for a STARK-like IOP only if we explicitly assume or
prove a knowledge-extraction theorem for the committed trace. A generic
"FRI-style proximity proof" is not enough by itself; proximity must be tied to
the committed witness and relation in a way that extracts the full role trace.

For the active native-boundary route, the best framing is:

```text
ParentTraceExtractionSoundness
```

as a composed proof-family contract from:

```text
ParentTraceCommitmentExtractionSoundness
ParentTraceDoubleBindingSoundness
MaskedOpeningConsistencySoundness
ExtractedTraceRelationKnowledgeSoundness
```

The current preferred endpoint,
`transparent_view_full_interface_from_selected_profile_atomic_gates_or_break`,
consumes those atomic pieces through the selected active profile. Meanwhile:

```text
TraceAssemblySoundness
```

is a design-controlled native-boundary theorem.

## Next Move

Add Lean predicates distinguishing:

```text
sampledOpeningOnly
fullTraceExtractable
```

or, more generally, add a proof-family extraction mode enum so the active theorem
can explicitly reject sampled-opening-only candidates.

Done:
`Descender/Path2NativeBoundary.lean` now defines `TraceExtractionMode` and
`SupportsFullTraceExtraction`. `ParentTraceExtractionSoundness` requires
`UsesSelectedTraceKnowledgeModes profile`, so the active theorem is pinned to
`fullTraceExtractable` plus `fullWitnessRelationKnowledge` with native-boundary
relation coverage. Profiles marked `sampledOpeningOnly`, `proximityOnly`, or a
non-selected relation-knowledge mode cannot satisfy the active parent-trace
extraction theorem unless the active profile is deliberately changed.

Further split:
`research/path2_native_boundary_relation_knowledge_soundness_contract.md`
separates full-trace extraction from relation knowledge. Lean now defines
`RelationKnowledgeMode` and `SupportsRelationKnowledge`; parent-trace
extraction requires both full trace extraction and relation-knowledge support.

## Non-Claims

- This does not prove any STARK/IOP theorem.
- This does not choose a concrete commitment scheme.
- This does not update proof bytes.
