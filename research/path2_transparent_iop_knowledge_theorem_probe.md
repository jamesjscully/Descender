# Path 2 Transparent IOP Knowledge Theorem Probe

Status: active proof-family theorem probe; no concrete IOP selected.

Purpose: state the exact theorem a transparent committed-trace proof family
must supply before the active Path 2 native-boundary route can be treated as
cryptographically closed.

## Active Endpoint

The active Lean closure theorem is:

```text
transparent_view_full_interface_from_selected_profile_atomic_gates_or_break
```

It targets the selected Path 2 native-boundary account-creation profile only.
It is not a generic transparent proof-system statement and it does not apply to
inactive signature paths.

The account-admission wrapper is:

```text
AccountAdmissionSoundness
account_admission_soundness_or_break
AccountCreateSelectedTransparentViewSoundness
selected_transparent_view_atomic_gates_imply_account_admission
```

so the final bridge must start from `P.accountCreateAccepts c b`, recover the
accepted transparent view and selected profile consumed by the endpoint, and
then return the account-admission representative or a named break.

## Existing Lean Obligations

The concrete proof family must instantiate the following proof-family-facing
obligations from `Descender/Path2NativeBoundary.lean`:

```text
ParentTraceCommitmentExtractionSoundness
ParentTraceDoubleBindingSoundness
MaskedOpeningConsistencySoundness
ExtractedTraceRelationKnowledgeSoundness
NoExportSoundness
RoleAuthenticationMaterialNoExportSoundness
MaskFreshnessSoundness
NativeBoundaryTraceQromScheduleSoundness
NativeBoundaryTraceQromExtractorRewindSoundness
HybridBase128RangeRowSoundness
HybridBase128RangeRowsToNormRangeSoundness
TraceRangeRowsBindRepresentativeSoundness
BlindingNoiseBoundRowSoundness
BlindingNoiseRowsToBoundSoundness
TraceBlindingRowsBoundRepresentativeSoundness
```

The endpoint then composes these with the native trace-assembly obligations:

```text
TraceRepresentativeAssemblyExistenceSoundness
TraceAssemblyRolesCompleteSoundness
TraceAssemblyDerivedRolesSoundness
TraceAssemblyFiberSoundness
TraceAssemblyLocalValiditySoundness
TraceAssemblyDescentConsistencySoundness
```

## Selected Profile Gates

The selected profile must satisfy:

```text
SelectedActiveColumnProfile(profile)
UsesSelectedTraceKnowledgeModes(profile)
UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)
UsesSelectedBlindingNoisePolicy(policy)
```

For the active profile, `UsesSelectedTraceKnowledgeModes` pins:

```text
fullTraceExtractable
fullWitnessRelationKnowledge
TraceRelationMeetsSelectedNativeBoundaryRowFloor
TraceRelationMeetsSelectedNativeBoundaryClassFloors
```

with selected row floor:

```text
NativeBoundaryTraceRelationV1 rows >= 176
```

and selected per-class floors for q-field rows, base-128 range rows,
mask-consistency rows, role-restriction rows, opening-tuple account binding,
checked-core residue compatibility, lifecycle/context, and proof-system
bookkeeping.

## Minimal Concrete Theorem

A concrete transparent proof family should supply one theorem of this shape:

```text
accountCreateAccepts(c, b)
  =>
c = b.context
  and exists view profile rangePolicy policy.
    transparentAccepts(b, view)
    and TransparentViewProjectsFrom(b, view)
    and ColumnProfileCompatibleWithBoundary(b, view, profile)
    and SelectedActiveColumnProfile(profile)
    and UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)
    and UsesSelectedBlindingNoisePolicy(policy)
```

plus the committed-oracle argument-of-knowledge theorem:

```text
TransparentIopAokAccepts(b, view, profile, proof)
  and SelectedActiveColumnProfile(profile)
  =>
    exists trace.
      transparentExtractsTrace(b, view, trace)
      and traceOpensBoundary(b, profile, trace)
      and maskedOpeningsConsistent(b, view, profile, trace)
      and traceRelationValid(b, profile, trace)
      and traceRangeRowsBindRepresentative(b, profile, trace)
      and traceBlindingRowsBoundRepresentative(b, profile, trace)
  or
    boundaryBreakFor(b.context, b, breakEvent)
```

with side theorems:

```text
same accepted parent root + two extracted traces
  =>
tracesSafetyEquivalent(b, profile, traceA, traceB)
  or trace binding break

verifier transcript
  =>
noExportHolds(b, view, profile)
  and roleAuthenticationMaterialSimulatable(b, view, profile)
  and maskFreshnessHolds(b, view, profile)
  or no-export/domain break

Fiat-Shamir schedule
  =>
qromScheduleBound(b, view, profile)
  and qromExtractorRewindBound(b, view, profile)
  or QROM break
```

The theorem must be stronger than sampled-row soundness. The extractor must
recover one full hidden native-boundary trace whose role handles, masked query
openings, range rows, blinding rows, opening-tuple rows, checked-core rows, and
lifecycle/context rows all refer to the same accepted boundary.

Lean now names the bridge and final adapter:

```text
AccountCreateSelectedTransparentViewSoundness(P)
selected_transparent_view_atomic_gates_imply_account_admission(P)
```

`AccountCreateSelectedTransparentViewSoundness` is not optional. Without it,
the proof family only proves a transparent-view theorem and does not close
account admission.

The Rust mirror is now named
`verify_path2_direct_creation_transparent_descent_selected_view_bridge`. It is a
pre-verifier adapter for this Lean obligation: from a direct account-creation
payload plus transparent payload it checks the selected proof-system id, public
view validity, selected column profile, query schedule, interface policy flags,
leaf-context binding, aggregation-context binding, commitment API, and payload
digest recomputation. This does not claim transparent proof soundness; the
account-create-to-view bridge is only enough to hand a concrete proof verifier
the selected view/profile tuple that `AccountCreateSelectedTransparentViewSoundness`
expects.

## False Implication To Reject

The following implication is invalid:

```text
Merkle root + sampled openings + proximity check
  =>
full hidden native-boundary trace extraction
```

It remains invalid even if every sampled opening is locally consistent. The
active theorem needs reconstruction or argument-of-knowledge soundness for
unqueried hidden columns and witnesses, plus binding between the reconstructed
trace and the verifier-visible masked openings.

## Go / No-Go

Continue the transparent route if the concrete proof family can state and
defend the theorem above while staying within the byte headroom in
`research/path2_native_boundary_proof_family_overhead_budget.md`.

Pivot if any of these are required:

- raw public openings of hidden role vectors, masks, range witnesses, pad
  source, opening tuple, blinding/noise, or account-binding witnesses;
- hidden SHA preimage rows inside `NativeBoundaryTraceRelationV1`;
- relation knowledge only for sampled rows or proximity without witness
  reconstruction;
- untyped digest labels in place of extractable role subcommitments;
- concrete overhead above the 16416-byte parked sigma comparator for the
  selected security row.
- no explicit bridge from `accountCreateAccepts` to the selected accepted
  transparent view, profile, range policy, and blinding policy.

## Tracker Impact

This probe should be treated as the next proof-family closure item after the
parallel route synthesis. It does not alter the active theorem target; it only
states the concrete theorem that a deployable transparent proof family must
provide.
