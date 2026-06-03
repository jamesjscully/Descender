# Path 2 Native Boundary Relation Knowledge-Soundness Contract

Status: relation-knowledge contract pinned; proof not supplied.

Purpose: separate "the extractor obtains a full trace" from "the extracted
trace satisfies the native-boundary relation."

Both are required. A full trace that is not known to satisfy the relation is not
enough for account creation.

## Relation Knowledge Modes

The Lean profile now distinguishes:

```text
RelationKnowledgeMode =
  sampledRowChecksOnly
  | proximityWithoutWitness
  | proximityWithWitnessBridge
  | directRelationKnowledge
  | fullWitnessRelationKnowledge
```

The active theorem requires:

```text
UsesSelectedTraceKnowledgeModes(profile)
```

which pins the active profile to:

- `TraceExtractionMode.fullTraceExtractable`;
- `RelationKnowledgeMode.fullWitnessRelationKnowledge`;
- full `TraceRelationCoversNativeBoundary` coverage;
- `TraceRelationMeetsSelectedNativeBoundaryRowFloor`, with selected floor
  `176` rows.

The broader helper predicate `SupportsRelationKnowledge(profile)` accepts:

- `proximityWithWitnessBridge`;
- `directRelationKnowledge`;
- `fullWitnessRelationKnowledge`.

It rejects:

- `sampledRowChecksOnly`;
- `proximityWithoutWitness`.

It also now requires the profile's `TraceRelationProfile` to cover every
mandatory native-boundary constraint class:

```text
qFieldRow
base128RangeRow
maskConsistencyRow
roleRestrictionRow
openingTupleAccountBindingRow
checkedCoreResidueCompatibilityRow
lifecycleContextRow
proofSystemBookkeepingRow
```

This prevents relation-knowledge soundness from being claimed for only the old
66-row semantic core or for a relation profile that omits masked-opening,
role-restriction, opening-tuple account-binding, checked-core compatibility,
lifecycle/context, or transcript/bookkeeping obligations.

## Contract

For an accepted proof with a full extracted trace:

```text
TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
UsesSelectedTraceKnowledgeModes(profile)
```

the proof family must establish:

```text
TraceRelationValid(b, profile, trace)
```

for the extracted `trace`, not for a post-hoc witness and not only for sampled
rows.

The parent extraction theorem now also requires:

```text
MaskedOpeningsConsistent(b, view, profile, trace)
```

before relation validity closes. This is intentionally separate from
`TraceRelationValid`: the public masked/simulated openings must be tied to the
same hidden extracted cells and mask witnesses used by the relation, so a proof
cannot pair a simulator-safe public view with an unrelated relation witness.

## Required Relation Content

`TraceRelationValid(b, profile, trace)` must cover:

1. public context columns match `p_context(b)`;
2. parent root/profile/role handles match `p_section_root(b)`,
   `p_role_profile(b)`, and `p_role_handles(b)`;
3. base-128 hidden-role digits reconstruct canonical q12289 semantic values;
4. packed residue coordinates reconstruct the account witness state;
5. pad source derives pad lanes and mask domains;
6. projection relation links account projection, pad projection, and
   `p_public_projection(b)`;
7. opening tuple binds account witness, randomizer, lifecycle anchor, and
   spend-opening coordinates;
8. range witnesses satisfy `hybridBase128HiddenRoles`;
9. blinding/noise auxiliaries satisfy extractor-side bounds;
10. no relation column depends on post-admission spend challenge material.

## STARK-Like Theorem Shape

A STARK-like route is acceptable only with a theorem of this shape:

```text
AcceptSTARK(public_inputs, commitments, openings, fri_proof)
  =>
exists trace.
  Opens(parent_root, trace)
  LowDegreeOrValidEncoding(trace)
  ConstraintSystemSatisfied(trace, public_inputs)
  NativeBoundaryRelation(trace, b, profile)
```

or a named break.

The theorem must not stop at:

```text
sampled rows satisfy queried constraints
```

unless it also proves that sampled acceptance implies a reconstructed witness
trace satisfying the relation except with the claimed soundness error.

## Proximity Bridge

If a future profile uses proximity testing, the bridge must show:

```text
CommittedOracle close to low_degree_oracle
Constraint checks accept
  =>
exists trace.
  TraceRelationValid(b, profile, trace)
```

The bridge must also show that the reconstructed trace opens the same parent
root/role handles or exports `inner_commitment_binding_break` /
`transparentProofSoundnessBreak`.

Bare proximity to some low-degree object is insufficient if that object is not
bound to the role-complete native-boundary trace.

## Failure Modes

Reject or export a named break if:

- constraints are checked only on sampled rows;
- a reconstructed low-degree object is not tied to `p_section_root(b)`;
- role subtraces are not tied to `p_role_handles(b)`;
- the relation omits hidden range/digit reconstruction;
- masked public openings are not linked to hidden unmasked cells;
- the trace can satisfy public checks while omitting a mandatory private role;
- the proof family cannot state a concrete soundness error for relation
  knowledge.

## Current Assessment

This is the remaining nontrivial cryptographic assumption for the active
transparent route. It is plausible as a named STARK/IOP-style proof-family
assumption, but it is not something the native-boundary design itself proves.

The design work has reduced the problem to a clean yes/no:

```text
Can the chosen proof family provide relation knowledge soundness for the
role-complete native-boundary trace without raw hidden openings?
```

If yes, continue Path 2. If no, switch to MPC-in-the-head or the parked
lattice-sigma fallback.

Relation inventory:
`research/path2_native_boundary_trace_relation_v1_inventory.md` defines
`NativeBoundaryTraceRelationV1` as the active relation object. It shows the old
66-row core is now only a comparator; the masked/native-boundary lower bound is
at least 124 rows before role, lifecycle, account-binding, checked-core, and
bookkeeping rows.
`research/path2_native_boundary_trace_relation_row_symbols.md` pins symbolic
row variables for the remaining open groups.
`research/path2_native_boundary_open_row_estimates.md` gives a first
conservative range of 160-220 rows, with working estimate 176, assuming SHA
digest and transcript schedule checks remain outside the hidden relation.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now defines:

- `RelationKnowledgeMode`
- `SupportsRelationKnowledge`
- `TraceRelationProfile`
- `TraceRelationCoversNativeBoundary`
- `TraceRelationMeetsSelectedNativeBoundaryRowFloor`

and `ParentTraceExtractionSoundness` requires both:

```text
UsesSelectedTraceKnowledgeModes(profile)
```

## Non-Claims

- This does not prove STARK/IOP soundness.
- This does not choose a concrete AIR/constraint system.
- This does not update byte counts.
