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
| 3 | `hidden_extractor_view` | `obligation_pinned` | The extractor recovers unmasked role digits, semantic residues, masks, blinding/noise, range witnesses, and account-binding witnesses. | Extractor returns `h in Fiber(B_c, b)` satisfying `ExtractedRolesComplete`, `LocalValid_c`, and `DescentConsistent_c`. | Extractor recovers only public masked values or cannot bind extracted values to `b`. |
| 4 | `masked_or_simulated_openings` | `theorem_gate_pinned` | Public query openings for hidden classes are masked, secret-shared, or simulated and are consistent with the extracted hidden trace. | `NoExportHolds(b, view, profile)` plus `MaskedOpeningConsistencySoundness`, with no raw hidden digit/range/blinding/mask exposure. | Any raw hidden pad/projection/opening/range/blinding cell is verifier-visible, or the public masked view is not tied to the extracted trace. |
| 5 | `hybrid_base128_range` | `blocking` | Hidden role values use base-128 digit witnesses; public masked projection remains raw canonical q12289. | `UsesSelectedNormRangeMode(profile)` and `NormRangeValid(b,h)` under `hybridBase128HiddenRoles`. | Candidate relies on raw q12289 hidden-role deltas for SIS binding. |
| 6 | `blinding_noise_extractor_bounds` | `blocking` | Blinding/noise/range auxiliary coordinates are extractor-bounded for binding without becoming public hiding commitments. | `BlindingNoiseBounded(b, policy, h)` is an extractor-side property; hiding comes from the proof simulator. | Candidate tries to use bounded public blinding to hide full-rank role commitments. |
| 7 | `mask_freshness_domains` | `blocking` | Every masked opening has a unique domain binding native boundary, role, column set, row, query, and transcript. | Reuse exports `live_slot_leakage_or_keygen_break` or `qrom_or_domain_separation_break`. | Same mask can enter two independent equations for a hidden value. |
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

Next: define the minimum theorem interface for the active proof family:
`NativeBoundaryTraceKnowledgeSoundness`,
`NativeBoundaryTraceZeroKnowledge`, and
`NativeBoundaryTraceQromSchedule`.
Evidence:
`research/path2_native_boundary_hidden_extractor_view.md` pins the complete
extractor output obligation and the Lean interface now includes
`HiddenExtractorRoleCompletenessSoundness`.
`research/path2_native_boundary_knowledge_soundness_decomposition.md` splits
knowledge soundness into parent-trace extraction and trace assembly, making the
construction-dependent part explicit.
`research/path2_native_boundary_role_completeness_table.md` pins the mandatory
roles and Lean now exposes `ProfileRoleComplete`.
`research/path2_native_boundary_parent_trace_extraction_contract.md` pins the
extraction-mode gate and Lean now rejects sampled-opening-only profiles via
`SupportsFullTraceExtraction`.
`Descender/Path2NativeBoundary.lean` now also requires
`MaskedOpeningConsistencySoundness` inside `ParentTraceExtractionSoundness`, so
the public masked opening view and hidden extractor trace cannot be discharged
as two unrelated witnesses.
`research/path2_native_boundary_proof_family_extractor_classification.md`
classifies vanilla Merkle/random-query paths as dead for this gate and keeps
`zk_masked_trace_iop_native_boundary` active only if it provides full
committed-trace extraction plus simulator-safe openings.
`research/path2_native_boundary_trace_proof_family_interface.md` pins the
minimum Lean-level split for the active proof-family assumption.

## Non-Claims

- This does not choose a concrete STARK/IOP library.
- This does not prove simulator security.
- This does not update byte counts.
- This does not make Path 2 deployable.
