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
  + NativeBoundaryTraceQromExtractorRewindSoundness
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
ParentTraceDoubleBindingSoundness
ExtractedTraceRelationKnowledgeSoundness
MaskedOpeningConsistencySoundness
ParentTraceExtractionSoundness
TraceAssemblySoundness
```

`ParentTraceExtractionSoundness` is active-path specific: Lean now requires
`UsesSelectedTraceKnowledgeModes(profile)`, selecting `fullTraceExtractable`
and `fullWitnessRelationKnowledge` with full native-boundary relation coverage.
For the selected active profile, the same predicate also requires the
native-boundary relation row floor `176`, matching the Rust
`PATH2_NATIVE_BOUNDARY_TRACE_RELATION_ROWS_WORKING` constant, plus selected
per-class relation floors so surplus rows cannot all sit in irrelevant classes.
`proximityWithWitnessBridge` remains a supported redesign shape, not the
currently selected Rust profile.

Meaning:

- one parent section root binds the full trace;
- role handles are typed restrictions of that root;
- an accepting proof extracts a full parent trace opening that root;
- two full traces opening that same accepted parent trace are safety-equivalent
  or export a named trace-binding break;
- the active extraction combiner consumes the double-trace theorem before
  returning `ParentTraceExtractionSoundness`;
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

`NativeBoundaryTraceQromExtractorRewindSoundness` is separate. It requires:

```text
QromExtractorRewindBound(b, view, profile)
```

or a named break. Schedule binding fixes what is hashed before challenges are
drawn; extractor-rewind soundness accounts for the measure-and-reprogram or
forking loss paid by the full-trace extractor for that exact schedule.

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
The Lean route now decomposes this into `HybridBase128RangeRowSoundness` plus
`HybridBase128RangeRowsToNormRangeSoundness`, which compose back into
`NormRangeSoundness`. `TraceRangeRowsBindRepresentativeSoundness` additionally
ties those row facts to the same extracted trace that assembles the
representative.
The blinding/noise side has the parallel route:
`BlindingNoiseBoundRowSoundness` plus `BlindingNoiseRowsToBoundSoundness`
compose back into `BlindingNoiseBoundSoundness`, with
`TraceBlindingRowsBoundRepresentativeSoundness` tying those bound rows to the
same extracted trace.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now names:

- `NativeBoundaryTraceKnowledgeSoundness`
- `NativeBoundaryTraceZeroKnowledge`
- `NativeBoundaryTraceQromScheduleSoundness`
- `NativeBoundaryTraceQromExtractorRewindSoundness`
- `NativeBoundaryZkTraceBindingAndHiding`
- `native_boundary_trace_qrom_schedule_bound_or_break`
- `native_boundary_trace_qrom_extractor_rewind_bound_or_break`
- `RoleAuthenticationMaterialNoExportSoundness`
- `MaskFreshnessSoundness`
- `AccountCreateSelectedTransparentViewSoundness`
- `selected_transparent_view_atomic_gates_imply_account_admission`

and `transparent_view_full_interface_or_break` now includes:

```text
UsesSelectedTraceKnowledgeModes(profile)
P.extractedRolesComplete b profile h
P.qromScheduleBound b view profile
P.qromExtractorRewindBound b view profile
```

The preferred active closure adapter is now:

```text
transparent_view_full_interface_from_selected_profile_atomic_gates_or_break
```

It derives the same endpoint from the selected active Rust profile map,
parent-trace commitment extraction, double-trace binding, masked-opening
consistency, extracted-trace relation knowledge, decomposed trace assembly,
same-trace hybrid range-row binding, hybrid range-row soundness, same-trace
blinding/noise row binding, and blinding/noise bound-row obligations. The older
`transparent_view_full_interface_from_atomic_gates_or_break` remains the
general atomic adapter; the selected-profile adapter derives role completeness
and selected trace-knowledge modes from `SelectedActiveColumnProfile`.

The final interface now routes through `ParentTraceExtractionSoundness` and
`TraceAssemblySoundness`; the older two-view extractor surface is no longer an
independent shortcut for the active endpoint.
`TraceAssemblySoundness` is now decomposed into representative assembly
existence, role completeness, boundary-fiber projection, local validity, and
descent consistency.
`AccountCreateSelectedTransparentViewSoundness` is the account-admission bridge:
accepted account creation must expose the accepted transparent view, selected
profile, selected range policy, and selected blinding policy. The theorem
`selected_transparent_view_atomic_gates_imply_account_admission` then turns the
selected-profile transparent endpoint into `AccountAdmissionSoundness`.

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
native_boundary_relation_row_floor = 176
native_boundary_relation_class_floors_required = true
parent_trace_commitment_extractable_required = true
extracted_trace_relation_knowledge_required = true
masked_opening_consistency_required = true
parent_trace_extraction_composition_required = true
trace_assembly_existence_required = true
trace_assembly_roles_complete_required = true
trace_assembly_derived_roles_required = true
trace_assembly_fiber_required = true
trace_assembly_local_valid_required = true
trace_assembly_descent_consistent_required = true
same_trace_range_rows_required = true
range_row_soundness_required = true
range_rows_to_norm_required = true
same_trace_blinding_rows_required = true
blinding_row_soundness_required = true
blinding_rows_to_bound_required = true
lean_atomic_full_interface_endpoint_present = true
lean_selected_profile_atomic_endpoint_present = true
lean_account_admission_adapter_endpoint_present = true
qrom_extractor_rewind_bound_required = true
double_trace_binding_break_exported = true
```

Negative controls reject clearing any of these flags or drifting either
blinding/noise policy field before the transparent verifier handoff. Clearing
or lowering the native-boundary relation row floor rejects because the active
profile must use the working 176-row native-boundary relation scale rather than
a one-row-per-class placeholder. Clearing
`native_boundary_relation_class_floors_required` rejects because the Lean
selected profile now requires conservative floors for each mandatory relation
class. Clearing
any decomposed assembly/range/blinding row flag rejects because the atomic Lean
endpoint consumes those subclaims directly, including the same-trace gates that
tie row facts to the extracted trace used for representative assembly. Clearing
the derived-role assembly flag rejects because lifecycle-slot and
statement-source derivation are now named inside `TraceAssemblySoundness`.
`lean_atomic_full_interface_endpoint_present` rejects because the active Rust
interface must be bound to the narrow Lean theorem
`transparent_view_full_interface_from_atomic_gates_or_break`.
Clearing `lean_selected_profile_atomic_endpoint_present` rejects because the
active closure target is the selected-profile theorem
`transparent_view_full_interface_from_selected_profile_atomic_gates_or_break`,
not the looser atomic adapter alone.
Clearing `lean_account_admission_adapter_endpoint_present` rejects because the
active route must bridge account creation into the selected transparent
view/profile/policy tuple and derive `AccountAdmissionSoundness`, not merely a
view-level theorem.
Rust now exposes the corresponding pre-verifier bridge as
`verify_path2_direct_creation_transparent_descent_selected_view_bridge`. It
reuses the direct creation proof payload binding checks, validates the accepted
transparent public view, requires the selected column profile/range/blinding
interface, checks the leaf context and aggregation context against the creation
binding, and recomputes the transparent payload digest. It deliberately stops
before transparent proof verification; the full
`verify_path2_direct_creation_transparent_descent_payload_boundary` still returns
the proof-verifier-not-implemented blocker after this bridge succeeds.
Clearing `qrom_extractor_rewind_bound_required` rejects because the active Lean
endpoint now consumes `NativeBoundaryTraceQromExtractorRewindSoundness` as a
distinct proof-family obligation, not as part of the schedule predicate.

## Closure Criterion

The proof side can treat this assumption as closed only after we decide one of:

1. A concrete proof family supplies all six components above with acceptable
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
