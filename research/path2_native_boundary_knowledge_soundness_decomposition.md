# Path 2 Native Boundary Knowledge-Soundness Decomposition

Status: proof decomposition pinned; concrete proof family not proved.

Purpose: refine `NativeBoundaryTraceKnowledgeSoundness` into the minimum
subclaims needed to turn an accepting transparent proof into a complete hidden
representative.

## Decomposed Shape

The knowledge-soundness part of the active proof-family assumption is now:

```text
NativeBoundaryTraceKnowledgeSoundness
  =
ParentRootRoleBindingSoundness
  + RoleRestrictionSoundness
  + ParentTraceCommitmentExtractionSoundness
  + ExtractedTraceRelationKnowledgeSoundness
  + MaskedOpeningConsistencySoundness
  + ParentTraceExtractionSoundness
  + TraceAssemblySoundness
```

The first two claims bind the public handles. The next three split the proof
family's hard extractor obligation into commitment extraction, relation
knowledge, and masked-opening consistency. `ParentTraceExtractionSoundness` is
the composed contract produced from those subclaims. `TraceAssemblySoundness`
is now itself a composed design-controlled endpoint; the active endpoint no
longer accepts an independent two-view extractor shortcut.

## ParentTraceExtractionSoundness

Target:

```text
TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
UsesSelectedTraceKnowledgeModes(profile)
  =>
exists trace.
  TransparentExtractsTrace(b, view, trace)
  TraceOpensBoundary(b, profile, trace)
  TraceRelationValid(b, profile, trace)
```

or a named break.

Meaning:

- the proof-family extractor recovers a full parent trace, not just sampled
  cells;
- the trace opens `p_section_root(b)`;
- the trace contains all role columns needed by `profile`;
- the trace satisfies the local algebraic relation over hidden and public
  columns.

This is the highest-risk cryptographic proof obligation.

## TraceAssemblySoundness

Target:

```text
TransparentExtractsTrace(b, view, trace)
TraceOpensBoundary(b, profile, trace)
TraceRelationValid(b, profile, trace)
  =>
exists h.
  TraceAssemblesRepresentative(trace, h)
  TransparentExtracts(b, view, h)
  ExtractedRolesComplete(b, profile, h)
  h in Fiber(B_c, b)
  LocalValid_c(h)
  DescentConsistent_c(h)
```

or a named break.

Meaning:

- this is mostly our native-boundary/descent bookkeeping, not a generic proof
  system theorem;
- once a full trace is available, the role columns must assemble into one
  hidden representative;
- the assembly proof is split into representative existence, role completeness,
  boundary fiber projection, local validity, and descent consistency;
- failures are charged to boundary binding, descent soundness, norm/range, or
  trace relation soundness.

This is more achievable than parent-trace extraction because we control the
column profile and the `B_c` projection design.

## Why This Split Matters

Without this split, the phrase "the extractor obtains `h`" hides two very
different requirements:

1. cryptographic proof-family extraction of a committed trace;
2. native-boundary assembly of that trace into the descent representative.

The first is construction-dependent and may force us toward a STARK-like IOP,
MPC-in-the-head, or the parked sigma fallback. The second is the part we can
make true by construction through the column profile, role handles, and
boundary projections.

## Candidate Impact

| candidate | parent-trace extraction | trace assembly | disposition |
|---|---|---|---|
| vanilla Merkle random queries | no | not reached | dead |
| Merkle/FRI proximity without knowledge extractor | not enough | not reached | dead unless supplemented |
| STARK-style IOP with knowledge extraction | plausible assumption | under our control | active if ZK/no-export works |
| masked trace IOP native-boundary | intended assumption | under our control | active |
| MPC-in-the-head | full witness extraction is natural | needs role mapping | fallback |
| lattice sigma | full witness extraction is natural | needs larger payload | fallback floor |

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now defines:

- `ExtractedTrace`
- `Predicates.transparentExtractsTrace`
- `Predicates.traceOpensBoundary`
- `Predicates.traceRelationValid`
- `Predicates.traceAssemblesRepresentative`
- `Predicates.traceAssemblyRolesComplete`
- `Predicates.traceAssemblyInFiber`
- `Predicates.traceAssemblyLocalValid`
- `Predicates.traceAssemblyDescentConsistent`
- `ParentTraceCommitmentExtractionSoundness`
- `ExtractedTraceRelationKnowledgeSoundness`
- `MaskedOpeningConsistencySoundness`
- `UsesSelectedTraceKnowledgeModes`
- `ParentTraceExtractionSoundness`
- `TraceRepresentativeAssemblyExistenceSoundness`
- `TraceAssemblyRolesCompleteSoundness`
- `TraceAssemblyFiberSoundness`
- `TraceAssemblyLocalValiditySoundness`
- `TraceAssemblyDescentConsistencySoundness`
- `decomposed_trace_assembly_implies_trace_assembly_soundness`
- `TraceAssemblySoundness`
- `trace_extraction_implies_hidden_representative_or_break`
- `transparent_view_full_interface_from_decomposed_or_break`
- `transparent_view_full_interface_from_atomic_gates_or_break`

## Next Proof Move

Work on `TraceAssemblySoundness` first, because it is design-controlled. The
task is now to instantiate the five decomposed subclaims for the columns of
`ExtractedTrace` that assemble into:

```text
Section_c
PackedResidues_c
PadSource_c
OpeningTuple_c
ProjectionRelation_c
LifecycleSlot_c
TraceWitness_c
Gauge_c
```

and to verify that every required projection of `b` is determined by those
columns.

First-pass result:
`research/path2_native_boundary_trace_assembly_map.md` pins the map from full
trace roles to `H_c` components and records the selected active role column-set
map.
`research/path2_native_boundary_role_completeness_table.md` pins mandatory
role coverage, and the Lean interface now requires `ProfileRoleComplete` for
trace assembly and hidden extractor role completeness. Lean now also exposes
`SelectedActiveColumnProfile` and proves
`selected_active_column_profile_role_complete`, so the active Rust
masked-base128 role/column-set map discharges `ProfileRoleComplete` directly.
`research/path2_native_boundary_parent_trace_extraction_contract.md` pins the
parent-trace extraction contract. The preferred full-interface endpoint now
uses `transparent_view_full_interface_from_atomic_gates_or_break`, so concrete
proof work can target the parent-trace commitment extraction, masked-opening
consistency, relation-knowledge, decomposed assembly, same-trace range-row,
range-row soundness, same-trace blinding-row, and blinding-row subclaims
directly. The endpoint now runs the same-trace range/blinding obligations on
the extracted trace and assembled representative that appear in the final
interface conclusion.
`research/path2_native_boundary_relation_knowledge_soundness_contract.md` pins
the separate relation-knowledge gate, and Lean now requires
`SupportsRelationKnowledge profile` as well.

If trace assembly fails, the active route is structurally wrong. If trace
assembly succeeds, the remaining hard assumption is cleanly isolated as
parent-trace extractability plus no-export.

## Non-Claims

- This does not prove STARK/IOP knowledge soundness.
- This does not prove zero knowledge.
- This does not update proof sizes.
