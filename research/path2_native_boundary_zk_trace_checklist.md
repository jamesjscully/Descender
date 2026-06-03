# Path 2 Native Boundary ZK Trace Checklist

Status: concrete proof-family checklist pinned.

Purpose: instantiate the active `NativeBoundaryZkTraceBindingAndHiding`
assumption as a pre-implementation gate. No accepting verifier work should
start until all blocking rows are either satisfied by a concrete proof family or
the path is abandoned.

## Blocking Gates

| rank | id | status | requirement | accept | reject |
|---:|---|---|---|---|---|
| 1 | `parent_root_binding` | `interface_pinned` | One parent section/trace root binds all role restrictions and public masked-view columns for the native boundary. | Extractor theorem outputs one parent trace under `p_section_root(b)` or exports `transparentProofSoundnessBreak`. | Proof only extracts sampled openings or an after-the-fact satisfying tuple. |
| 2 | `private_typed_role_restrictions` | `theorem_and_profile_gate_pinned` | Role handles are typed private restrictions under `p_section_root(b)` and `p_role_profile(b)`, not public raw hidden leaves or public full-rank linear commitments. | Role handles authenticate pad/projection/opening/packed-witness columns with `inner_commitment_binding_break` for double openings, and private roles use only masked, simulator-only, or extractor-only authentication modes. | Candidate publishes standalone `C_role = A_role*v_role mod q` for private roles under current dimensions, or authenticates private roles through raw hidden leaves. |
| 3 | `hidden_extractor_view` | `selected_profile_atomic_endpoint_pinned_parent_extraction_open` | The extractor recovers unmasked role digits, semantic residues, masks, blinding/noise, range witnesses, and account-binding witnesses. | `transparent_view_full_interface_from_selected_profile_atomic_gates_or_break` composes the selected active profile, commitment extraction, masked-opening consistency, relation knowledge, and decomposed trace assembly into one role-complete representative in `Fiber(B_c,b)`. | Extractor recovers only public masked values, cannot extract one full trace, or cannot bind assembled values to `b`. |
| 4 | `masked_or_simulated_openings` | `theorem_gate_pinned` | Public query openings for hidden classes are masked, secret-shared, or simulated and are consistent with the extracted hidden trace. | `NoExportHolds(b, view, profile)`, `RoleAuthenticationMaterialNoExportSoundness`, `MaskFreshnessSoundness`, plus `MaskedOpeningConsistencySoundness`, with no raw hidden digit/range/blinding/mask exposure. | Any raw hidden pad/projection/opening/range/blinding cell is verifier-visible, role-authentication material is not simulatable, masks are reused across safety-distinct domains, or the public masked view is not tied to the extracted trace. |
| 5 | `hybrid_base128_range` | `same_trace_decomposed_route_pinned` | Hidden role values use base-128 digit witnesses; public masked projection remains raw canonical q12289. | `UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)` pins semantic policy `1`, witness policy `2`, rows `64`, digit counts `12/12/16`, range rows `40`, reconstruction rows `20`, extractor-hidden semantic range entries, and range-private witness visibility; `TraceRangeRowsBindRepresentativeSoundness`, `HybridBase128RangeRowSoundness`, and `HybridBase128RangeRowsToNormRangeSoundness` close `NormRangeSoundness`. | Candidate relies on raw q12289 hidden-role deltas for SIS binding, mutates selected range-policy ids/layout, or cannot bind accepted range rows from the same extracted trace to the representative. |
| 6 | `blinding_noise_extractor_bounds` | `same_trace_decomposed_route_pinned` | Blinding/noise/range auxiliary coordinates are extractor-bounded for binding without becoming public hiding commitments. | `UsesSelectedBlindingNoisePolicy(policy)` pins policy id `1` and bound `2048`; `TraceBlindingRowsBoundRepresentativeSoundness`, `BlindingNoiseBoundRowSoundness`, and `BlindingNoiseRowsToBoundSoundness` close `BlindingNoiseBoundSoundness`. Hiding still comes from the proof simulator/no-export route. | Candidate tries to use bounded public blinding to hide full-rank role commitments, mutates the selected bound, or cannot bind accepted bound rows from the same extracted trace to the representative. |
| 7 | `mask_freshness_domains` | `theorem_and_profile_gate_pinned` | Every masked opening has a unique domain binding native boundary, role, column set, row, query, and transcript. | `MaskFreshnessSoundness` plus Rust proof-family interface/capability flags; reuse exports `live_slot_leakage_or_keygen_break` or `qrom_or_domain_separation_break`. | Same mask can enter two independent equations for a hidden value. |
| 8 | `fiat_shamir_qrom_schedule` | `ledger_pinned` | Queries/challenges are derived after binding native boundary digest, profile, roots, role handles, matrix transcript, aggregation context, and proof-system id. | Transcript order is explicit and feeds the QROM ledger. | Any role handle, root, or context projection can be chosen after query derivation. |
| 9 | `byte_threshold` | `decision_gate` | Concrete proof estimate remains below fallback threshold or has better assumptions. | Stays materially below 16416-byte parked lattice-sigma floor or justifies higher bytes. | Approaches/exceeds fallback without better theorem quality. |

## Active Candidate

```text
zk_masked_trace_iop_native_boundary
```

This is not vanilla Merkle/FRI. The proof family must provide ZK/simulator
semantics for query openings and private role restrictions.

## Current Dispositions

| path | disposition |
|---|---|
| `vanilla_merkle_fri_trace` | dead negative control |
| `public_linear_role_commitments_current_dimensions` | dead for private roles |
| `zk_masked_trace_iop_native_boundary` | active |
| `small_noise_computational_hiding` | fallback if private role restrictions cannot be instantiated |
| `mpc_in_the_head_hash_zk` | fallback if transparent query simulation fails |
| `base16_hidden_roles` | fallback if base-128 digit-bound sanity fails |

## Immediate Blocker

The first unresolved gate is:

```text
hidden_extractor_view
```

The parent-root binding interface is pinned in
`research/path2_native_boundary_parent_root_role_binding.md`, and the role
restriction object is pinned in
`research/path2_native_boundary_private_role_restriction_v1.md`. The allowed
role authentication modes are mapped in
`research/path2_native_boundary_role_authentication_mode_map.md`; Lean now
requires `ColumnProfilePrivateAuthModesSafe` through
`ColumnProfileCompatibleWithBoundary`, and the Rust active column-profile digest
binds `Path2TwoViewColumnProfileEntryV1.authentication_mode` with negative
controls for raw hidden-leaf and public-linear downgrades. The QROM
schedule is pinned in
`research/path2_native_boundary_zk_trace_qrom_schedule.md`, and the symbolic
native-boundary query rows are now recorded in
`research/path2_hash_qrom_assumption_ledger.md`.

The minimum theorem interface for the active proof family is now defined and
digest-bound. The remaining blocker is no longer naming the interface; it is
constructing or selecting a proof family that satisfies the atomic extraction
triple and the same-trace row obligations consumed by
`transparent_view_full_interface_from_selected_profile_atomic_gates_or_break`.
Evidence:
`research/path2_native_boundary_hidden_extractor_view.md` pins the complete
extractor output obligation and the Lean endpoint now derives role-complete
hidden representatives through `ParentTraceExtractionSoundness` plus
`TraceAssemblySoundness`.
`research/path2_native_boundary_knowledge_soundness_decomposition.md` splits
knowledge soundness into parent-trace commitment extraction, extracted-trace
relation knowledge, masked-opening consistency, the composed parent-trace
extraction contract, and trace assembly, making the construction-dependent
part explicit.
`research/path2_native_boundary_role_completeness_table.md` pins the mandatory
roles and Lean now exposes `ProfileRoleComplete`.
`research/path2_native_boundary_trace_relation_v1_inventory.md`,
`research/path2_native_boundary_opening_tuple_account_binding_estimate.md`, and
`research/path2_native_boundary_checked_core_compat_estimate.md` pin the
mandatory opening-tuple account-binding and checked-core residue-compatibility
row classes; Lean now requires those classes inside
`TraceRelationCoversNativeBoundary`, so a proof profile cannot close with only
the masked 66-row core plus generic lifecycle/bookkeeping rows.
`research/path2_native_boundary_parent_trace_extraction_contract.md` pins the
extraction-mode gate and Lean now rejects sampled-opening-only profiles via
`SupportsFullTraceExtraction`.
`Descender/Path2NativeBoundary.lean` now also requires
`MaskedOpeningConsistencySoundness` inside `ParentTraceExtractionSoundness`, so
the public masked opening view and hidden extractor trace cannot be discharged
as two unrelated witnesses.
`NativeBoundaryTraceZeroKnowledge` now also splits out
`RoleAuthenticationMaterialNoExportSoundness` and `MaskFreshnessSoundness`, and
the Rust proof-family interface/capability digests reject missing
role-authentication simulator support or missing mask-freshness support before
the transparent verifier handoff.
`research/path2_native_boundary_proof_family_extractor_classification.md`
classifies vanilla Merkle/random-query paths as dead for this gate and keeps
`zk_masked_trace_iop_native_boundary` active only if it provides full
committed-trace extraction plus simulator-safe openings.
`research/path2_native_boundary_trace_proof_family_interface.md` pins the
minimum Lean-level split for the active proof-family assumption.
The preferred Lean endpoint now consumes the same-trace range-row and
blinding-row gates directly, so accepted range/blinding facts cannot be supplied
from a trace other than the extracted trace that assembles the hidden
representative.

## Non-Claims

- This does not choose a concrete STARK/IOP library.
- This does not prove simulator security.
- This does not update byte counts.
- This does not make Path 2 deployable.
