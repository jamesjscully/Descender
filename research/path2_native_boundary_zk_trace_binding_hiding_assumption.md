# Path 2 Native Boundary ZK Trace Binding/Hiding Assumption

Status: active proof-family assumption pinned; not proved.

Purpose: replace the rejected public-linear-role-commitment path with the
native-boundary two-view proof-family assumption that the current theorem
surface actually needs.

## Assumption Name

```text
NativeBoundaryZkTraceBindingAndHiding
```

This is the native-boundary refinement of the older
`zkMaskedTraceIOPBindingAndHiding` assumption.

## Scope

For a native boundary `b`, transparent public view `view`, and compatible
column profile `profile`, an accepted proof satisfies:

```text
TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)
```

and either proves all of:

1. role handles authenticate typed private role restrictions under
   `p_section_root(b)`;
2. the extractor obtains `h in Fiber(B_c, b)`;
3. `ExtractedRolesComplete(b, profile, h)`;
4. `LocalValid_c(h)`;
5. `DescentConsistent_c(h)`;
6. `NormRangeValid(b, h)` under the selected `hybridBase128HiddenRoles`
   range-policy certificate;
7. `BlindingNoiseBounded(b, policy, h)` as an extractor-side bound;
8. `NoExportHolds(b, view, profile)`;
9. `RoleAuthenticationMaterialSimulatable(b, view, profile)`;
10. `MaskFreshnessHolds(b, view, profile)`;

or exports a named break.

## Public Values

The public proof may serialize:

- native boundary digest / compatibility `creation_proof_binding_digest`;
- leaf context digest;
- parent section root;
- role handles as typed subcommitment handles;
- masked projection;
- matrix-domain transcript digest;
- proof system id and relation id;
- query/opening/path/consistency digests or simulator-safe values.

It must not serialize:

- standalone public `C_role = A_role * v_role mod q` commitments for private
  roles under the rejected full-rank dimensions;
- raw hidden role columns;
- raw digit/range witnesses;
- raw blinding/noise/mask values;
- reject-path private diagnostics.

## Break Surface

| break | meaning |
|---|---|
| `transparentProofSoundnessBreak` | accepted proof has no extractable parent trace / hidden representative |
| `inner_commitment_binding_break` | typed role restriction authenticates two safety-distinct role vectors |
| `boundary_binding_break` | accepted public projections do not agree with one native boundary |
| `descent_soundness_break` | extracted local role data does not glue to one hidden representative |
| `path2_sis_norm_or_range_break` | extracted digit/range/blinding coordinates violate selected bounds |
| `live_slot_leakage_or_keygen_break` | public transcript leaks hidden representative coordinates |
| `qrom_or_domain_separation_break` | transcript/query domains fail binding or separation |

## Relationship To Rejected Public Linear Commitments

The rejected public object:

```text
C_role = A_role * v_role mod q
```

is not the active boundary object. It may appear only as:

- a private internal relation inside the proof trace;
- an extractor-side equation;
- a negative-control baseline.

If a concrete proof family requires publishing those role commitments with the
current full-column-rank dimensions, that family fails this assumption.

## Lean Anchor

The legacy composed Lean theorem for the monolithic assumption is:

```text
Descender.Path2NativeBoundary.transparent_view_full_interface_or_break
```

The active closure theorem preferred by the proof-family interface is:

```text
Descender.Path2NativeBoundary.transparent_view_full_interface_from_selected_profile_atomic_gates_or_break
```

It composes:

- `NativeBoundaryTraceKnowledgeSoundness`, namely:
- `ParentRootRoleBindingSoundness`;
- `RoleRestrictionSoundness`;
- `ParentTraceCommitmentExtractionSoundness`;
- `ExtractedTraceRelationKnowledgeSoundness`;
- `MaskedOpeningConsistencySoundness`;
- `ParentTraceExtractionSoundness`;
- `UsesSelectedTraceKnowledgeModes(profile)` for that extraction contract;
- `TraceAssemblySoundness`;
- `NativeBoundaryTraceZeroKnowledge`, namely:
- `NoExportSoundness`;
- `RoleAuthenticationMaterialNoExportSoundness`;
- `MaskFreshnessSoundness`;
- `NativeBoundaryTraceQromScheduleSoundness`;
- `NormRangeSoundness`;
- `UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)` for that
  norm/range contract;
- or, equivalently for the selected route, `HybridBase128RangeRowSoundness`
  plus `HybridBase128RangeRowsToNormRangeSoundness`;
- `TraceRangeRowsBindRepresentativeSoundness` for same-trace range-row
  binding;
- `BlindingNoiseBoundSoundness`.
- or, equivalently for the selected route, `BlindingNoiseBoundRowSoundness`
  plus `BlindingNoiseRowsToBoundSoundness`.
- `TraceBlindingRowsBoundRepresentativeSoundness` for same-trace
  blinding/noise row binding.

For the active endpoint, the same-trace range/blinding row gates are consumed
on the extracted trace and assembled hidden representative used in the final
interface conclusion.

## Active Candidate

The active candidate remains a ZK masked/two-view transparent IOP, but now with
native-boundary semantics:

```text
zk_masked_trace_iop_native_boundary
```

The proof family must provide typed private role restrictions, not public
full-rank linear role commitments.

## Fallbacks

| fallback | trigger |
|---|---|
| `small_noise_computational_hiding` | if private role restrictions cannot be instantiated but a coherent LWE/leftover-hash hiding commitment can |
| `mpc_in_the_head_hash_zk` | if transparent masked trace cannot supply no-export/query simulation |
| `base16_hidden_roles` | if base-128 digit-coordinate SIS sanity fails |
| `dual_commitment_hiding_plus_binding` | if one-object proof becomes too assumption-heavy |

## Non-Claims

- This does not instantiate a concrete proof system.
- This does not prove simulator security.
- This does not prove SIS/module-SIS binding.
- This does not update byte counts.
