# Account-Creation Prevalidated Slot Proof Boundary Ledger

Status: Path 2 proof-boundary ledger for direct single-slot deployment mode.

This ledger separates executable evidence, proof statements, carried lifecycle
assumptions, and explicit non-claims for the Path 2 account-creation /
prevalidated-slot design.

## Boundary Classes

| id | boundary | status | evidence / next action |
|---|---|---|---|
| P2-BL-001 | canonical leaf/evidence/certificate frames | executable KAT coverage | `path2_prevalidated_slot_leaf_v1_frame_is_canonical`, frame mutation KATs |
| P2-BL-002 | non-circular creation statement | executable KAT coverage | `Path2CreationHiddenSourceStatementV1`, message/signature independence KAT |
| P2-BL-003 | direct replayable relation binding | executable skeleton, theorem pending | relation replay accepts reviewed active-lane certificate and rejects pad, statement, profile, verifier rebinding |
| P2-BL-004 | direct authorization-to-admission | executable KAT coverage | exact digest and `(account_id, state_root, authorization)` namespace admission KATs; composed direct-spend verifier consumes this path |
| P2-BL-005 | checked-core spend challenge binding | executable KAT coverage | challenge frame mutation KATs, accepting Path 2 signer KATs, and composed direct-spend verifier |
| P2-BL-006 | consume-once capability | executable local model, theorem pending | composed direct-spend KAT proves first spend inserts the exact authorization and second spend rejects; successor-state theorem still needed |
| P2-BL-007 | no-postchallenge creation timing | proof-game prose | `path2_no_postchallenge_creation_timing_game.md` |
| P2-BL-008 | lifecycle/reorg/rollback behavior | carried deployment assumption plus local controls | `state_lifecycle_closure_n32_q12289.md`; must be named in main theorem |
| P2-BL-009 | live-slot leakage/hiding | ledger stated, executable closure partial | debug no-secret KAT exists; full creation payload API theorem remains |
| P2-BL-010 | hash/QROM budget | assumption ledger stated | `path2_hash_qrom_assumption_ledger.md`; query-budget accounting remains |
| P2-BL-011 | optional aggregation | inactive theorem mode | no direct-mode dependency; future mode consumes `optional_aggregation_soundness` |
| P2-BL-012 | external review | out of internal scope | required before deployment claim, but excluded from this internal completion tracker |
| P2-BL-013 | descent/SIS creation payload commitment | transparent fail-closed payload boundary anchored, theorem pending | `path2_descent_sis_feasibility.md` fixes public quotient, committed role vectors, and lower-bound byte/q-op rows but is blocked as a public commitment shape; `path2_descent_sis_obligation_matrix.md` pins the closure rows; `Descender.Path2DescentSIS` and `path2_descent_sis_type_map.md` close the public quotient/type-map row; `path2_descent_sis_matrix_domains.md` and Rust KATs close matrix-domain regeneration; `path2_descent_sis_binding_surface.md` pins the double-opening-to-SIS-kernel binding game; `path2_descent_sis_norm_range_surface.md` identifies raw q12289 residue openings as a binding/norm bottleneck; `path2_descent_sis_rank_hiding_probe.md` shows the current public pad/projection/opening commitments are full-column-rank and reveal openings; `path2_descent_sis_hiding_redesign_options.md` pins candidate fixes; `path2_descent_sis_lattice_sigma_toy_model.md` shows the optimistic 64-repetition floor is 16416 bytes; `path2_transparent_proof_byte_model.md` now models old 4896/6744/8848-byte lower-bound rows, masked 5520/7524/9784-byte floors, and conservative range/account 6144/8304/10720-byte floors for 256/512/1024 rows; `path2_payload_proof_candidate_decision.md` selects transparent proof over the Path 2 descent relation as the active payload candidate; `path2_transparent_payload_contract.md` pins the selected proof frame, query symbols, two-view extractor/soundness surface, no-export surface, and negative controls; `PATH2_TRANSPARENT_DESCENT_*` constants, `path2_transparent_descent_transcript_digest`, `path2_transparent_descent_public_view_v1_json`, and KATs `path2_transparent_descent_transcript_binds_public_context` / `path2_transparent_descent_public_view_serializes_only_public_material` provide first executable transcript and no-export metadata coverage; `path2_direct_query_budget_v1` now marks transparent proof query symbols active while keeping `q_batch` inactive; `path2_transparent_extractor_soundness_bridge.md` pins how an accepting two-view transparent proof feeds `direct_replayable_relation_certificate_soundness_or_break` or exports a named transparent-proof, hidden-source, norm/range, QROM/domain, or no-export break; `verify_path2_direct_creation_transparent_descent_payload_boundary` and KAT `path2_transparent_descent_payload_boundary_binds_public_view_before_fail_closed` bind transparent proof kind, proof-system id, public view, nonrecursive leaf context, creation proof binding, payload digest, and query-opening digests before rejecting fail-closed; `path2_inner_commitment_relation_refinement.md` splits the 66-row target into `R_core_66` versus inner-commitment opening, account-binding, and outer-SHA layers; `path2_trace_commitment_binding_audit.md` shows current code binds trace roots through transparent transcript and payload digest, not directly inside `Path2CreationProofBindingV1`; `path2_role_commitment_link_equation_options.md` selects typed transparent trace subcommitments as the compact link-equation path and marks ordinary SHA digest handles dead for compact opening; `path2_transparent_commitment_api.md` pins the theorem interface needed from the proof family: parent root, role subroots, column profile, hiding/blinding rule, extractor guarantee, and public payload bytes; `path2_transparent_proof_family_gate.md` selects `zk_masked_trace_iop` conditionally and rejects vanilla raw trace opening for hidden columns; `path2_masked_trace_transform.md` pins the masked query view and raises the private-lane floor from 80 to 138; `path2_masked_trace_range_account_delta.md` raises the conservative lane floor to 194 before concrete range/FRI overhead; `path2_zk_masked_trace_iop_assumption.md` names the required assumption and its failback threshold; `path2_masked_trace_iop_selection_checklist.md`, `path2_masked_trace_commitment_layout.md`, `path2_masked_trace_leaf_encoding_fork.md`, and `path2_two_view_trace_theorem_interface.md` refine the current blocker to a two-view theorem: public masked leaves plus a hidden extractor view with consistency between them; `path2_two_view_linear_leakage_probe.md` passes the fresh-mask sanity check and demonstrates reused masks leak rank; `path2_two_view_column_profile_rules.md` makes unique mask domains and profile-version rejection mandatory |

The leaf/registry/state-binding rows are expanded in
`research/path2_leaf_registry_state_binding_theorem.md`. The local one-use
successor-state row is expanded in
`research/path2_one_use_successor_state_theorem.md`.

## Valid Successor State

For the direct single-slot theorem, a valid successor state is a state reached
from an admitted account state by exactly one successful consumption of the
named `slot_authorization_digest`, with these local conditions:

- the consumed authorization digest equals `SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || leaf)`;
- the leaf was admitted under the exact `(account_id, account_state_root_before)`
  namespace used by the spend challenge;
- the slot id in the checked-core signature equals the slot id in the leaf;
- the leaf live marker was `0x01` before consumption;
- the consume-once set did not already contain the authorization digest;
- the successor consume-once set contains that digest.

Executable closure for the local transition is
`verify_path2_direct_single_slot_spend_authorization`. That verifier composes
public-key identity binding, state-namespace admission, checked-core
authorization, exact checked-core/leaf slot agreement, and consume-once mutation
in one deployment-facing path. The negative slot-rebinding KAT is now part of
the proof boundary because the standalone checked-core verifier can validate a
signature whose `slot_authorization_digest` names another leaf unless the
composed verifier also enforces decoded slot equality against that leaf.

The theorem does not model arbitrary chain reorg semantics internally. A reorg,
rollback, restore, or mempool reveal that makes a consumed or revealed slot
spendable again is charged to `slot_replay_or_rollback_break` and the lifecycle
closure policy.

## Main Direct-Mode Theorem Shape

```text
account_creation_prevalidated_slot_authorization_or_break:

If direct single-slot Path 2 spend authorization accepts, then:

1. the checked-core signature is valid for the message, public key,
   account id, state root, slot id, proof commitment, and
   slot_authorization_digest;
2. slot_authorization_digest names a canonical PrevalidatedSlotLeafV1;
3. that leaf was admitted before the spend challenge under the same account
   and state-root namespace;
4. admission accepted a direct replayable relation certificate for
   CreationHiddenSourceStatementV1;
5. the exact authorization is consumed once in a valid successor state;

or the adversary exports one of:

- creation_time_hidden_source_soundness_break;
- prevalidation_leaf_binding_break;
- prevalidation_registry_binding_break;
- prevalidation_data_availability_break;
- postchallenge_slot_creation_or_rewrite_break;
- checked_core_message_binding_break;
- slot_replay_or_rollback_break;
- qrom_or_domain_separation_break;
- live_slot_leakage_or_keygen_break;
- transparentProofSoundnessBreak;
- path2_sis_norm_or_range_break.
```

## Explicit Non-Claims

- Direct mode does not consume aggregation soundness.
- Path 2 does not prove spend-only full packed-witness extraction.
- The reviewed 308-byte/Fq12 projection-carrier route remains a fail-closed
  baseline and negative-control source, not the deployment theorem.
- The descent/SIS feasibility rows are lower-bound candidate-shape estimates,
  not proof-size or hardness claims.
- Local executable KATs are not a substitute for external cryptographic review.

## Remaining Internal Closure

Risk-first order is pinned in
`research/path2_proof_risk_burn_down_plan.md`. Broad verifier implementation is
deliberately later than the theorem-risk burn-down.

1. Resolve the exact transparent relation layer split. First pass:
   `research/path2_transparent_relation_arithmetization_target.md` now treats
   the 66 rows as `R_core_66` only: masked projection, digit reconstruction, and
   digit/range checks. Refinement:
   `research/path2_inner_commitment_relation_refinement.md` requires either a
   proof that the transparent proof-family trace is linked to the proof-friendly
   role commitments named by `Path2CreationProofBindingV1`, or explicit
   algebraic opening rows with updated byte/row estimates. Trace-binding audit:
   `research/path2_trace_commitment_binding_audit.md` keeps the current two-hop
   payload/certificate binding as the preferred no-state-change route, but only
   if those link equations are stated and proved. Link-equation pass:
   `research/path2_role_commitment_link_equation_options.md` selects typed trace
   subcommitments as the compact path and rejects ordinary SHA digest handles as
   a compact opening theorem. Commitment API pass:
   `research/path2_transparent_commitment_api.md` pins the proof-family interface
   needed to keep the 66-row core compact. Proof-family gate:
   `research/path2_transparent_proof_family_gate.md` marks vanilla raw trace
   opening as dead for hidden columns and makes the masked-column transform the
   next blocker. Masking pass: `research/path2_masked_trace_transform.md`
   raises the lane floor to 138, and
   `research/path2_transparent_proof_byte_model.md` updates the planning floors.
   Range/account pass: `research/path2_masked_trace_range_account_delta.md`
   raises the conservative floor to 194 lanes and updates planning floors to
   6144/8304/10720 bytes for 256/512/1024 rows. The next blocker is formalizing
   the masked-trace IOP assumption and setting fallback thresholds; first pass:
   `research/path2_zk_masked_trace_iop_assumption.md`. The active interface is
   now a two-view trace theorem:
   `research/path2_two_view_trace_theorem_interface.md`.
   Resolution pass:
   `research/path2_top_proof_risk_resolution_options.md` rejects raw
   transparent openings and SHA-in-circuit compactness; selected theorem shape is
   a ZK transparent IOP over proof-friendly inner commitments, with outer
   SHA-256 used only for public byte binding and data availability.
2. Prove or redesign the query-opening no-export/ZK story for concrete trace
   columns before selecting final transparent proof parameters.
3. Re-run raw-centered versus base-128 digit-opening binding as theorem
   statements, then either carry a bounded-SIS/norm assumption or promote digit
   openings and update the byte model.
4. Prove the nonrecursive public binding composition from leaf/evidence/
   certificate to transparent public view and direct relation certificate.
5. Instantiate exact Fiat-Shamir/QROM query accounting after the proof family
   and query schedule are fixed.
6. Only then implement the accepting transparent verifier/KAT suite and connect
   `direct_replayable_relation_certificate_soundness_or_break` to spend
   authorization and consume-once in one composition document.
