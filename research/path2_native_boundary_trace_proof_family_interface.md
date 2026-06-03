# Path 2 Native Boundary Trace Proof-Family Interface

Status: minimum active proof-family interface pinned in Lean and Rust;
construction not selected.

Purpose: split `NativeBoundaryZkTraceBindingAndHiding` into the three proof
interfaces that must be supplied by any concrete transparent account-creation
proof family.

## Interface Split

The active assumption is now decomposed as:

```text
NativeBoundaryZkTraceBindingAndHiding
  =
NativeBoundaryTraceKnowledgeSoundness
  + NativeBoundaryTraceZeroKnowledge
  + NativeBoundaryTraceQromScheduleSoundness
  + NormRangeSoundness
  + BlindingNoiseBoundSoundness
```

This split is proof-side only. It does not select bytes, verifier code, or a
library.

## Knowledge Soundness

`NativeBoundaryTraceKnowledgeSoundness` requires:

```text
ParentRootRoleBindingSoundness
RoleRestrictionSoundness
ParentTraceCommitmentExtractionSoundness
ExtractedTraceRelationKnowledgeSoundness
MaskedOpeningConsistencySoundness
ParentTraceExtractionSoundness
TraceAssemblySoundness
```

`ParentTraceExtractionSoundness` is active-path specific: Lean now requires
`UsesSelectedTraceKnowledgeModes(profile)`, selecting `fullTraceExtractable`
and `fullWitnessRelationKnowledge` with full native-boundary relation coverage.
`proximityWithWitnessBridge` remains a supported redesign shape, not the
currently selected Rust profile.

Meaning:

- one parent section root binds the full trace;
- role handles are typed restrictions of that root;
- an accepting proof extracts a full parent trace opening that root;
- the profile declares `fullTraceExtractable` or `fullWitnessExtractable`, not
  sampled-opening-only extraction;
- the profile declares relation knowledge support, not sampled-row-only checks
  or proximity without witness reconstruction;
- the profile's relation inventory covers mandatory native-boundary classes,
  including opening-tuple account-binding and checked-core residue
  compatibility;
- the full trace assembles into the native hidden representative;
- an accepting proof extracts `h in Fiber(B_c, b)`;
- the extracted `h` covers every private role in the column profile.

This is the highest-risk part. A proof that only checks sampled query openings
does not satisfy this interface.

## Zero Knowledge / No Export

`NativeBoundaryTraceZeroKnowledge` is currently:

```text
NoExportSoundness
RoleAuthenticationMaterialNoExportSoundness
MaskFreshnessSoundness
```

Meaning verifier-visible material must be simulatable from public boundary
projections and masked public values. Raw hidden role vectors, digit/range
witnesses, masks, pad sources, blinding/noise coordinates, and account-binding
witnesses must not be public openings.

The split is intentional:

- `NoExportSoundness` says the public transcript exports only allowed material.
- `RoleAuthenticationMaterialNoExportSoundness` says public authentication
  paths, simulator messages, and extractor-only handles are simulatable and do
  not leak raw private role vectors.
- `MaskFreshnessSoundness` says every masked opening uses a fresh
  native-boundary/role/column/row/query/transcript domain or exports a named
  leakage/domain break.

## QROM Schedule

`NativeBoundaryTraceQromScheduleSoundness` requires:

```text
QromScheduleBound(b, view, profile)
```

or a named break. It means the transcript binds all native-boundary public
values, role handles, authentication modes, range/no-export/blinding policies,
proof-system ids, relation ids, and mask-domain material before deriving query
indices or challenges.

The symbolic query rows are recorded in
`research/path2_hash_qrom_assumption_ledger.md`.

## Range And Blinding

The proof family must also supply:

```text
NormRangeSoundness
BlindingNoiseBoundSoundness
```

under selected mode:

```text
hybridBase128HiddenRoles
UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)
```

This keeps the public masked projection in canonical q12289 form while using
base-128 hidden-role witnesses for SIS/range binding. The selected range
certificate fixes semantic range policy id `1`, witness policy id `2`, private
role rows `64`, digit counts `12/12/16`, reconstruction rows `20`, range rows
`40`, range witnesses `40`, and digit maximum `127`.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now names:

- `NativeBoundaryTraceKnowledgeSoundness`
- `NativeBoundaryTraceZeroKnowledge`
- `NativeBoundaryTraceQromScheduleSoundness`
- `NativeBoundaryZkTraceBindingAndHiding`
- `native_boundary_trace_qrom_schedule_bound_or_break`
- `RoleAuthenticationMaterialNoExportSoundness`
- `MaskFreshnessSoundness`

and `transparent_view_full_interface_or_break` now includes:

```text
UsesSelectedTraceKnowledgeModes(profile)
P.extractedRolesComplete b profile h
P.qromScheduleBound b view profile
```

The final interface now routes through `ParentTraceExtractionSoundness` and
`TraceAssemblySoundness`; the older two-view extractor surface is no longer an
independent shortcut for the active endpoint.

The deeper knowledge-soundness split is recorded in
`research/path2_native_boundary_knowledge_soundness_decomposition.md`.

The Rust active proof-family interface mirror is
`Path2NativeBoundaryTraceProofFamilyInterfaceV1`. Its digest is bound into the
transparent proof-system id through
`path2_transparent_proof_family_capability_v1_digest`, and it now requires:

```text
opening_tuple_account_binding_rows_required = true
checked_core_residue_compatibility_rows_required = true
role_authentication_material_simulatable_required = true
mask_freshness_required = true
blinding_noise_policy_id = 1
blinding_noise_centered_bound = 2048
sis_role_matrix_rows = 64
parent_trace_commitment_extractable_required = true
extracted_trace_relation_knowledge_required = true
masked_opening_consistency_required = true
parent_trace_extraction_composition_required = true
qrom_extractor_rewind_bound_required = true
double_trace_binding_break_exported = true
```

Negative controls reject clearing any of these flags or drifting either
blinding/noise policy field before the transparent verifier handoff.

## Closure Criterion

The proof side can treat this assumption as closed only after we decide one of:

1. A concrete proof family supplies all five components above with acceptable
   assumptions.
2. We intentionally keep `NativeBoundaryZkTraceBindingAndHiding` as one named
   cryptographic assumption and document that the deployability claim depends
   on it.
3. The active transparent route fails, and we move to the MPC-in-the-head or
   parked lattice-sigma fallback.

## Immediate Next

Attack the highest-risk component first:

```text
NativeBoundaryTraceKnowledgeSoundness
```

The specific question is whether a compact transparent masked trace proof can
provide full committed-trace extraction without publishing raw hidden role
cells.

## Non-Claims

- This does not prove the proof-family assumption.
- This does not choose STARK/FRI/MPC internals.
- This does not update account-creation bytes.
