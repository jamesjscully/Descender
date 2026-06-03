# Path 2 Native Boundary Transparent Proof Family V1

Status: deployability-facing proof-family assumption pinned; not proved.

Purpose: package the active transparent route as one named assumption with
explicit sub-obligations, row model, and overhead budget.

## Name

```text
NativeBoundaryTransparentProofFamilyV1
```

This is the concrete-facing version of:

```text
NativeBoundaryZkTraceBindingAndHiding
```

for the masked trace / STARK-like route.

## Required Sub-Assumptions

The proof family must provide:

```text
ParentTraceCommitmentExtractionSoundness
ParentTraceDoubleBindingSoundness
ExtractedTraceRelationKnowledgeSoundness
MaskedOpeningConsistencySoundness
ParentTraceExtractionSoundness
TraceAssemblySoundness
NoExportSoundness
RoleAuthenticationMaterialNoExportSoundness
MaskFreshnessSoundness
NativeBoundaryTraceQromScheduleSoundness
NormRangeSoundness
BlindingNoiseBoundSoundness
```

The first two are proof-family cryptographic assumptions. `TraceAssemblySoundness`
is mostly native-boundary bookkeeping once a full trace is extracted, and Lean
now decomposes it into representative assembly existence, role completeness,
derived lifecycle/statement roles, boundary-fiber projection, local validity,
and descent consistency.
The current preferred closure theorem is
`transparent_view_full_interface_from_selected_profile_atomic_gates_or_break`,
which composes the selected active profile, parent-trace commitment extraction,
double-trace binding, masked-opening consistency, relation-knowledge,
decomposed assembly, and decomposed range/blinding row soundness.

## Required Profile Modes

The active profile must satisfy:

```text
ProfileRoleComplete(profile)
UsesSelectedTraceKnowledgeModes(profile)
UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)
UsesSelectedBlindingNoisePolicy(policy)
selectedSisRoleMatrixRows = 64
```

Rejected modes:

- sampled openings only;
- sampled row checks only;
- proximity without witness reconstruction;
- public raw hidden openings;
- public full-rank linear role commitments for private roles.

## Relation Model

The hidden relation is:

```text
NativeBoundaryTraceRelationV1
```

Current row estimates:

```text
optimistic: 160
working:    176
stress:     220
```

The active planning lane floor is:

```text
194 lanes
```

## Overhead Budget

Comparator:

```text
parked_lattice_sigma_floor = 16416 bytes
```

Current native-boundary overlay:

| trace rows | working floor | stress floor | working headroom | stress headroom |
|---:|---:|---:|---:|---:|
| 256 | 8704 | 9728 | 7712 | 6688 |
| 512 | 11504 | 12784 | 4912 | 3632 |
| 1024 | 14560 | 16096 | 1856 | 320 |

## Decision

`NativeBoundaryTransparentProofFamilyV1` remains the active path if:

- the concrete proof-family overhead fits inside the selected row's headroom;
- no hidden SHA preimage relation is introduced;
- the proof theorem gives full trace extraction and relation knowledge;
- no-export is simulator-backed and does not require raw hidden openings.
- role-authentication material is simulator-backed and masked openings have
  fresh native-boundary/query domains.

Pivot if:

- the 1024-row case is required and concrete overhead exceeds 1856 bytes;
- the stress row is required and any nontrivial overhead is unavoidable;
- relation knowledge cannot be stated for the full trace;
- typed subcommitments are unavailable;
- no-export fails.

## Relation To Fallbacks

If this path fails by theorem quality, prefer:

1. MPC-in-the-head / VOLE-in-the-head full-witness proof;
2. lattice/SIS ZK / parked lattice-sigma route;
3. SHA-heavy generic proof only as a last-resort baseline.

If this path only fails by bytes at the high-security row, compare directly
against MPC-in-the-head and lattice/SIS ZK using the same account-creation
state model.

## Non-Claims

- This does not prove the proof-family assumption.
- This does not instantiate STARK/FRI internals.
- This does not choose final security rows.
- This does not make Path 2 deployable.
