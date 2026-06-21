# Path 2 Native Solution Search Todo Tracker

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This remains a detailed research todo list for native Path 2 design search.
> It is subordinate to the consolidated top-level tracker in `../TRACKER.md`.

Detail status: supporting research todo list, subordinate to `../TRACKER.md`.

Purpose: drive the search for a concrete native account-creation design using
the high-level descent/geometric strategy. This tracker is for internal proof
and design work only. It does not include external review.

Status legend:

- `[ ]` open
- `[~]` in progress
- `[x]` complete
- `[D]` dead path

## Goal

Identify and promote a concrete native Path 2 account design such that account
creation is primarily a structural descent-boundary verification problem, not a
generic hidden-witness ZK problem.

Target shape:

```text
M_c      semantic account meaning
Q_c      public boundary/moduli quotient
O_c      obstruction/check object
obs_c    Q_c -> O_c
H_c      private local representatives
G_c      private gauge/randomizer action
```

with:

```text
obs_c(q)=0
  => exists h. boundary_c(h)=q ∧ LocalValid_c(h)

boundary_c(h)=boundary_c(h') => h ~safe h'
```

and all consensus-facing fields factoring through `q : Q_c`.

## Phase 0: Ground Rules

- [x] Reframe the problem away from fixed hidden witness proof search.
  Evidence:
  `research/path2_native_boundary_core_problem_reframe.md`,
  `research/path2_native_descent_non_zk_alternatives.md`, and
  `research/path2_holistic_descent_boundary_construction.md`.

- [x] Define the high-level abstract search space.
  Evidence:
  `research/path2_high_level_descent_design_space.md`.

- [x] Add geometric design lens.
  Evidence:
  `research/path2_geometric_descent_lens.md`.

- [x] Define concrete search strategy.
  Evidence:
  `research/path2_concrete_solution_search_strategy.md`.

- [x] Keep older masked-trace/STARK-like route as measured fallback only.
  Requirement: do not delete or abandon the existing route, but do not let it
  determine the native design unless all native candidates fail.
  Evidence:
  `research/path2_meaning_v1.md` explicitly treats the masked-trace route as a
  fallback proof strategy rather than the definition of account meaning.

## Phase 1: Define `M_c` First

- [x] Create `research/path2_meaning_v1.md`.
  Requirement: define the minimal semantic account meaning fixed by account
  creation before choosing public boundary bytes or private witness layout.
  Evidence:
  `research/path2_meaning_v1.md`.

- [x] Define account namespace component.
  Requirement: identify exactly what distinguishes one account from another at
  consensus level.
  Evidence:
  `research/path2_meaning_v1.md` defines `AccountNamespace_c`.

- [x] Define spend authorization class.
  Requirement: specify the public meaning that a future spend must bind to,
  independent of representative-specific private choices.
  Evidence:
  `research/path2_meaning_v1.md` defines `SpendAuthorizationClass_c`.

- [x] Define lifecycle/nullifier identity.
  Requirement: state what account creation fixes so consume-once/replay
  behavior cannot vary within a public boundary fiber.
  Evidence:
  `research/path2_meaning_v1.md` defines `LifecycleIdentity_c`.

- [x] Define checked-core transcript binding surface.
  Requirement: identify which public account meaning must be bound into the
  checked-core challenge/transcript frame.
  Evidence:
  `research/path2_meaning_v1.md` defines `TranscriptBindingSurface_c`.

- [x] Define state-transition boundary.
  Requirement: specify what the admitted account authorizes or constrains for
  future state transitions.
  Evidence:
  `research/path2_meaning_v1.md` defines `StateTransitionBoundary_c`.

- [x] Define context/version/domain binding.
  Requirement: make verifier version, parameter context, and domain separation
  part of the semantic object where needed.
  Evidence:
  `research/path2_meaning_v1.md` defines `ParameterContext_c`.

- [x] State `meaning_c : H_c -> M_c` abstractly.
  Requirement: this can be provisional, but it must name the semantic projection
  every candidate must preserve.
  Evidence:
  `research/path2_meaning_v1.md` states the abstract representative projection.

- [x] Define semantic safety equivalence from `M_c`.
  Target:

  ```text
  h ~safe h' iff meaning_c(h)=meaning_c(h')
  ```

  or document the extra safety fields if equality of `M_c` is not sufficient.
  Evidence:
  `research/path2_meaning_v1.md` defines first-pass `~safe` by equality of
  `meaning_c`.

## Phase 2: Candidate A - Pullback Boundary

- [x] Create `research/path2_candidate_pullback_boundary.md`.
  Evidence:
  `research/path2_candidate_pullback_boundary.md`.

- [x] Define component quotients.
  Candidate shape:

  ```text
  Q_c =
    Q_account
      ×_{Q_context}
    Q_lifecycle
      ×_{Q_context}
    Q_spend
      ×_{Q_context}
    Q_transcript
  ```
  Evidence:
  `research/path2_candidate_pullback_boundary.md` defines `Q_context`,
  `Q_account`, `Q_lifecycle`, `Q_spend`, `Q_transcript`, and `Q_state`.

- [x] Define the shared base/context object.
  Requirement: identify exactly what all components must agree on by
  construction.
  Evidence:
  `research/path2_candidate_pullback_boundary.md` defines `Q_context`.

- [x] Define public projections from `Q_c`.
  Required projections:

  ```text
  p_leaf
  p_spend
  p_lifecycle
  p_transcript
  p_state
  ```
  Evidence:
  `research/path2_candidate_pullback_boundary.md` defines first-pass
  projections.

- [x] State the pullback validity condition.
  Requirement: validity should be native agreement over the shared context, not
  equality proofs between independent commitments.
  Evidence:
  `research/path2_candidate_pullback_boundary.md` defines
  `PullbackValid_c(q)`.

- [~] Gate A1: projection completeness.
  Target:

  ```text
  every public/consensus field = p_i(q)
  ```
  Progress:
  First pass is likely pass by construction, pending audit of older checked-core
  carrier compatibility names, creation proof binding digest, transparent view
  fields, and role subcommitment handles.

- [~] Gate A2: safety saturation.
  Target:

  ```text
  boundary_c(h)=boundary_c(h') => h ~safe h'
  ```
  Progress:
  Plausible if `meaning_c` ignores gauge/randomizer coordinates; still blocked
  on whether `OpeningLocal_c` contains semantic spend authority.

- [~] Gate A3: effective descent.
  Target:

  ```text
  q in pullback-valid boundary => exists account-backed local lift h
  ```
  Progress:
  Identified as the main pullback blocker: pullback consistency does not by
  itself imply existence of a private account-backed lift.

- [~] Gate A4: fiber privacy.
  Requirement: pullback public data must not become injective in signer-local
  pads, masks, openings, or gauges.
  Progress:
  Plausible if component quotients exclude local/gauge coordinates; needs audit.

- [~] Gate A5: spend closure.
  Requirement: future spend must be expressible over `q` plus signer-held local
  authorization data.
  Progress:
  Plausible, pending checked-core binding audit against `p_transcript(q)`.

- [~] Gate A6: native verification.
  Requirement: core validity must not immediately require a generic hidden
  witness proof.
  Progress:
  Pullback membership is native for consistency; lift existence remains unclear.

- [~] Decide pullback candidate disposition.
  Values: `[active]`, `[demote]`, `[D]`.
  Progress:
  Preliminary disposition is `[active-but-incomplete]`: likely useful as the
  outer semantic boundary, but probably not sufficient alone for effective
  descent.

## Phase 3: Candidate B - Obstruction-Vanishing Boundary

- [x] Create `research/path2_candidate_obstruction_boundary.md`.
  Evidence:
  `research/path2_candidate_obstruction_boundary.md`.

- [x] Define public boundary data `Q_c`.
  Requirement: specify the public descent boundary whose validity is checked.
  Evidence:
  Candidate B defines `Q_c = ContextBoundary_c × MeaningBoundary_c ×
  LocalRestrictionBoundary_c × OverlapBoundary_c × SpendStateBoundary_c`.

- [x] Define obstruction/check object `O_c`.
  Requirement: determine whether this is a module, syndrome space, cocycle
  target, rank/check space, or product of smaller obstruction spaces.
  Evidence:
  Candidate B defines first-pass `O_c = O_context × O_gluing × O_meaning ×
  O_spend_state × O_canonicality`.

- [x] Define obstruction map.
  Target:

  ```text
  obs_c : Q_c -> O_c
  ```
  Evidence:
  Candidate B defines `obs_c(q)` as context, gluing, meaning,
  spend-state, and canonicality defects.

- [x] Define public validity.
  Target:

  ```text
  obs_c(q)=0
  ```
  Evidence:
  Candidate B defines public validity as obstruction vanishing.

- [x] State effective descent theorem candidate.
  Target:

  ```text
  obs_c(q)=0
    => exists h. boundary_c(h)=q ∧ LocalValid_c(h)
  ```
  Evidence:
  Candidate B states `obstruction_effective_descent`.

- [~] Bind obstruction vanishing to account meaning.
  Requirement: prove or state how `obs_c(q)=0` implies the lift is
  account-backed, not just some abstract glued object.
  Progress:
  Candidate B identifies this as the main blocker and introduces `O_meaning`
  and `O_spend_state` as required obstruction components.

- [~] Gate B1: projection completeness.
  Progress:
  Plausible because `MeaningBoundary_c` and `SpendStateBoundary_c` are explicit,
  pending checked-core and role-handle audit.

- [~] Gate B2: safety saturation.
  Progress:
  Strong if `O_meaning` is correct; still needs test that `OpeningLocal_c` and
  gauge choices cannot change spend authority.

- [~] Gate B3: effective descent.
  Progress:
  Highest-upside candidate for this gate, but blocked on concrete `O_c` and
  `obs_c`.

- [~] Gate B4: fiber privacy.
  Progress:
  Plausible if `Q_c` contains boundary-level restrictions only, not private
  witness coordinates.

- [~] Gate B5: spend closure.
  Progress:
  Plausible if `O_spend_state` binds spend/lifecycle/transcript/state surfaces.

- [~] Gate B6: native verification.
  Progress:
  Pass only if `obs_c(q)=0` becomes an explicit compact public algebraic check.

- [~] Decide obstruction candidate disposition.
  Values: `[active]`, `[demote]`, `[D]`.
  Progress:
  Preliminary disposition is `[active-highest-upside]`; main blocker is
  constructing a concrete account-backed obstruction theorem.

## Phase 4: Candidate C - Canonical Lift + Private Gauge

- [x] Create `research/path2_candidate_canonical_gauge.md`.
  Evidence:
  `research/path2_candidate_canonical_gauge.md`.

- [x] Define canonical public object `q : Q_c`.
  Requirement: public validity should mostly be canonicality/well-formedness of
  `q`.
  Evidence:
  Candidate C defines `Q_c` as canonical meaning/spend/lifecycle/transcript/state
  boundaries plus canonicality certificate.

- [x] Define canonical lift.
  Target:

  ```text
  canonical_lift_c : Q_c -> H_c / G_c
  ```
  Evidence:
  Candidate C states the target `canonical_lift_c`.

- [x] Define private gauge/randomizer space `G_c`.
  Requirement: identify which private choices are intended to be pure gauge:
  pads, masks, local section coordinates, opening-local data, randomizers, or
  other signer state.
  Evidence:
  Candidate C defines first-pass `G_c = PadGauge_c × MaskGauge_c ×
  RandomizerGauge_c × LocalSectionGauge_c × OpeningLocalGauge_c`.

- [x] Define gauge action on private representatives.
  Target:

  ```text
  G_c acts on Fiber(boundary_c, q)
  ```
  Evidence:
  Candidate C states `act_c : G_c -> H_c -> H_c` and boundary/meaning
  invariance laws.

- [~] Prove or falsify gauge invariance.
  Target:

  ```text
  meaning_c(g.h)=meaning_c(h)
  ```
  Progress:
  Candidate C identifies this as the core safety theorem; concrete gauge
  classification remains open.

- [~] Determine whether spend authority is gauge or semantic.
  Requirement: if private gauge changes spend authority, the candidate fails or
  must be redesigned.
  Progress:
  Candidate C identifies three acceptable outcomes and marks authority-changing
  gauge as dead.

- [~] Gate C1: projection completeness.
  Progress:
  Plausible because `Q_c` includes canonical semantic boundaries; checked-core
  and role-handle audit remains.

- [~] Gate C2: safety saturation.
  Progress:
  Strong if gauge invariance holds; blocked on concrete gauge classification.

- [~] Gate C3: effective descent.
  Progress:
  Strong if `canonical_lift_c(q)` exists without exporting private material.

- [~] Gate C4: fiber privacy.
  Progress:
  Central blocker: canonicalization must not collapse the private gauge fiber.

- [~] Gate C5: spend closure.
  Progress:
  Unclear until spend authority is classified as semantic versus gauge-local.

- [~] Gate C6: native verification.
  Progress:
  Pass if canonicality is public; fails if canonical lift existence requires
  generic hidden-witness proof.

- [~] Decide canonical-gauge candidate disposition.
  Values: `[active]`, `[demote]`, `[D]`.
  Progress:
  Preliminary disposition is `[active-high-risk-high-reward]`.

## Phase 5: Gate Matrix And Selection

- [x] Create `research/path2_native_candidate_gate_matrix.md`.
  Evidence:
  `research/path2_native_candidate_gate_matrix.md`.

- [x] Compare all candidates on projection completeness.
  Evidence:
  Gate matrix marks all three candidates `[likely-pass]`, with audits pending.

- [x] Compare all candidates on safety saturation.
  Evidence:
  Gate matrix identifies Candidate B as strongest if `O_meaning` is real,
  Candidate C blocked on gauge classification, and Candidate A unclear.

- [x] Compare all candidates on effective descent.
  Evidence:
  Gate matrix marks Candidate A blocked, Candidate B active core, and Candidate
  C dependent on canonical lift.

- [x] Compare all candidates on fiber privacy.
  Evidence:
  Gate matrix identifies Candidate C fiber-collapse risk and Candidate B's need
  to keep `Q_c` boundary-level.

- [x] Compare all candidates on spend closure.
  Evidence:
  Gate matrix marks spend closure unclear for all candidates pending
  spend/gauge/obstruction details.

- [x] Compare all candidates on native verification.
  Evidence:
  Gate matrix distinguishes Candidate A's native consistency check, Candidate
  B's concrete `obs_c` blocker, and Candidate C's canonicality/lift issue.

- [x] Identify residual proof burden for each survivor.
  Requirement: distinguish structural theorem obligations from external proof
  system obligations.
  Evidence:
  Gate matrix identifies residual burdens and recommends hybrid A+B+C.

- [x] Choose active candidate or hybrid.
  Requirement: write a clear disposition:

  ```text
  selected:
  parked:
  dead:
  reason:
  next theorem:
  ```
  Evidence:
  Gate matrix selects hybrid A+B+C, with Candidate B obstruction map
  construction as priority.

## Phase 5A: Obstruction Map Refinement

- [x] Create `research/path2_obstruction_map_v1_attempt.md`.
  Evidence:
  `research/path2_obstruction_map_v1_attempt.md`.

- [x] Define first concrete `Q_c` faces.
  Evidence:
  `path2_obstruction_map_v1_attempt.md` defines `ContextFace_c`,
  `AccountFace_c`, `SpendFace_c`, `LifecycleFace_c`, `TranscriptFace_c`,
  `StateFace_c`, `OverlapFace_c`, and `CanonicalityFace_c`.

- [x] Define first concrete `O_c` components.
  Evidence:
  `path2_obstruction_map_v1_attempt.md` defines `O_context`, `O_pullback`,
  `O_gluing`, `O_meaning`, `O_spend_state`, and `O_canonicality`.

- [x] Define first concrete `obs_c` component map.
  Evidence:
  `path2_obstruction_map_v1_attempt.md` defines `context_defect`,
  `pullback_defect`, `gluing_defect`, `meaning_defect`,
  `spend_state_defect`, and `canonicality_defect`.

- [x] Define local restrictions.
  Evidence:
  V1 local restrictions are public semantic faces:
  `AccountFace_c`, `SpendFace_c`, `LifecycleFace_c`, `TranscriptFace_c`, and
  `StateFace_c`.

- [x] Define public gluing equation classes.
  Evidence:
  V1 public gluing equations include same-context, account-spend,
  account-lifecycle, spend-transcript, lifecycle-state, state-account, and
  canonical meaning agreement.

- [x] Identify obstruction-map residual blocker.
  Evidence:
  `path2_obstruction_map_v1_attempt.md` concludes that `obs_c=0` gets from
  public faces to a global public boundary, but still needs a lift/gauge theorem
  from global public boundary to private representative fiber.

- [x] Create `research/path2_gauge_lift_v1_attempt.md`.
  Requirement: define `Lift_c(q)`, `Gauge_c`, `OpeningLocal_c`,
  `Randomizer_c`, and classify private coordinates as pure gauge,
  authorization-local, or semantic.
  Progress:
  Completed as `research/path2_gauge_lift_v1_attempt.md`; see Phase 5B below.

## Phase 5B: Gauge/Lift Refinement

- [x] Create `research/path2_gauge_lift_v1_attempt.md`.
  Evidence:
  `research/path2_gauge_lift_v1_attempt.md`.

- [x] Define private fiber shape over `q`.
  Evidence:
  V1 defines `H_c(q) = LiftCore_c(q) × AuthorizationLocal_c(q) ×
  Gauge_c(q) × Randomizer_c(q)`.

- [x] Define boundary map over private fiber.
  Evidence:
  V1 defines `boundary_c(q,lift,auth,gauge,randomizer)=q`.

- [x] Define semantic map over private fiber.
  Evidence:
  V1 defines `meaning_c(q,lift,auth,gauge,randomizer)=
  meaning_of_boundary_c(q)` when authorization is compatible.

- [x] Define lift existence theorem target.
  Evidence:
  V1 states `lift_exists_v1`.

- [x] Define fiber privacy theorem target.
  Evidence:
  V1 states `fiber_privacy_v1` and sufficient conditions.

- [x] Define gauge invariance theorem target.
  Evidence:
  V1 states `gauge_invariance_v1`.

- [x] Classify first-pass private coordinates.
  Evidence:
  V1 classifies account/lifecycle/transcript/state as public meaning, checked-core
  private opening local as `AuthorizationLocal_c(q)`, pads/masks as
  gauge/randomizer, and blinding/noise as randomizer.

- [x] Create `research/path2_authorization_local_classification.md`.
  Requirement: decide whether authorization-local data changes only spend
  evidence or changes public spend class.
  Evidence:
  `research/path2_authorization_local_classification.md`.

## Phase 5C: Authorization-Local Classification

- [x] Classify authorization-local data.
  Evidence:
  `path2_authorization_local_classification.md` classifies it as private
  semantic-local signing material over a fixed public authorization class, not
  pure gauge.

- [x] State public spend class law.
  Evidence:
  `path2_authorization_local_classification.md` states:
  `auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)`.

- [x] State dead condition for multiple spend classes in one boundary fiber.
  Evidence:
  The classification rejects any `q` with two compatible authorization locals
  producing different `AuthClass_c`.

- [x] Define authorization-local break surface.
  Evidence:
  Named breaks include `authorization_class_binding_break`,
  `authorization_local_sampling_break`, `authorization_local_hiding_break`,
  `spend_closure_break`, `lifecycle_authorization_mismatch_break`, and
  `transcript_authorization_binding_break`.

- [x] Create `research/path2_spend_closure_over_q.md`.
  Requirement: state `SpendAccept(c,q,sig,msg,state)` and audit whether current
  checked-core/current spend verification can consume projections of `q` without
  independent semantic handles.
  Evidence:
  `research/path2_spend_closure_over_q.md`.

## Phase 5D: Spend Closure Over `q`

- [x] State `SpendAccept(c,q,sig,msg,state)`.
  Evidence:
  `path2_spend_closure_over_q.md`.

- [x] Define public spend verifier inputs as projections of `q`.
  Evidence:
  V1 requires spend verification to use `p_spend(q)`, `p_lifecycle(q)`,
  `p_transcript(q)`, `p_state(q)`, message, current state, and signature bytes.

- [x] State spend closure theorem.
  Evidence:
  V1 states `spend_closure_over_q_v1`.

- [x] State production soundness over authorization-local data.
  Evidence:
  V1 states `spend_production_soundness_v1`.

- [x] State checked-core interpretation over `q`.
  Evidence:
  V1 defines `checked_core_frame(q,msg,state)` and allowed/disallowed inputs.

- [x] Mark independent spend handles as dead unless factoring through `q`.
  Evidence:
  V1 marks `SpendAccept(c, handle, sig, msg, state)` dead if
  `handle != p_spend(q)` and no public factoring proof exists.

- [x] State lifecycle/transcript/state closure targets.
  Evidence:
  V1 states lifecycle, transcript, and state closure over `q`.

- [x] Create `research/path2_native_boundary_projection_audit.md`.
  Requirement: audit current public fields and classify each as projection of
  `q`, byte/data-availability binding for `q`, private authorization-local,
  pure gauge/randomizer, or dead independent semantic handle.
  Evidence:
  `research/path2_native_boundary_projection_audit.md`.

## Phase 5E: Projection Audit

- [x] Classify top-level native boundary fields.
  Evidence:
  Projection audit classifies version/context/account/lifecycle/state fields as
  projections of `q`, and `native_boundary_digest` as byte binding for `q`.

- [x] Classify section/role fields.
  Evidence:
  Projection audit classifies role handles as typed restrictions of `q` or dead
  if independent SHA/preimage handles.

- [x] Classify spend-facing fields.
  Evidence:
  Projection audit classifies slot authorization, opening commitment, pad seed,
  checked-core frame, and checked-core proof bytes under the native `q` model.

- [x] Classify transparent/proof-family fields.
  Evidence:
  Projection audit classifies trace roots/columns/query openings as proof
  metadata/evidence unless semantically projected from `q`.

- [x] Identify high-risk independent-handle fields.
  Evidence:
  Projection audit lists `opening_commitment`, `pad_seed_digest`,
  `slot_authorization_digest`, `parent_section_root`, and role handles.

- [x] Define next consolidation step.
  Evidence:
  Projection audit identifies `path2_active_native_account_design_v1.md` as the
  next required document.

## Phase 6: Promote Active Native Design

- [x] Create `research/path2_active_native_account_design_v1.md`.
  Evidence:
  `research/path2_active_native_account_design_v1.md`.

- [x] Define concrete `M_c`, `Q_c`, `O_c`, `H_c`, `G_c`.
  Evidence:
  Active design defines `M_c`, first active `Q_c`, `O_c`, and private
  `H_c(q) = LiftCore_c(q) × AuthorizationLocal_c(q) × Gauge_c(q) ×
  Randomizer_c(q)`.

- [x] Define maps:

  ```text
  meaning_c
  boundary_c
  obs_c
  p_leaf
  p_spend
  p_lifecycle
  p_transcript
  p_state
  ```
  Evidence:
  Active design defines `meaning_of_boundary_c`, `obs_c`, boundary/semantic
  projections, and public projections.

- [x] State projection completeness theorem.
  Evidence:
  Active design states public projections and field disposition; independent
  handles are dead unless projected/byte-bound.

- [x] State safety saturation theorem.
  Evidence:
  Active design defines `~safe` via `meaning_c` and requires `H_c(q)` meaning
  to derive from `q`.

- [x] State effective descent theorem.
  Evidence:
  Active design states account admission and effective descent decomposition.

- [x] State fiber privacy / no-export theorem.
  Evidence:
  Active design states private/gauge/randomizer field disposition and
  `fiber_privacy_break` surface; concrete theorem remains an open obligation.

- [x] State spend closure theorem.
  Evidence:
  Active design states `SpendAccept(c,q,sig,msg,state)` and `spend_closure_v1`.

- [x] State named break surface.
  Minimum breaks:

  ```text
  effective_descent_break
  safety_saturation_break
  projection_factorization_break
  gauge_privacy_break
  spend_closure_break
  domain_binding_break
  ```
  Evidence:
  Active design lists the minimum active break surface.

- [x] Create `Descender/Path2NativeAccountDesign.lean`.
  Requirement: theorem-interface level first; no concrete cryptographic
  instantiation until structural laws are stable.
  Evidence:
  `Descender/Path2NativeAccountDesign.lean` defines the active native design
  interface: `Meaning`, `Boundary`, `Obstruction`, private fiber predicates,
  public projections, account admission, effective descent, authorization class
  binding, fiber privacy, spend closure, checked-core frame binding, and named
  breaks.

- [x] Add import to `Descender.lean` only after the Lean file builds.
  Evidence:
  `Descender.lean` imports `Descender.Path2NativeAccountDesign`.

- [x] Run:

  ```text
  lake build Descender.Path2NativeAccountDesign
  lake env lean Descender.lean
  ```
  Progress:
  `lake build Descender.Path2NativeAccountDesign` passes.
  `lake env lean Descender.lean` passes.

## Phase 7: Byte, State, And Proof-System Reconciliation

- [x] Reset byte model for selected native design.
  Requirement: do not inherit 6-16 KB proof-family estimates unless the
  selected design still needs that proof layer.
  Progress:
  `research/path2_native_byte_state_model_v1.md` resets planning around
  semantic projections of `q` and separates fallback proof envelopes.

- [x] Define concrete projection surfaces.
  Evidence:
  `research/path2_projection_surfaces_v1.md` defines `p_context`, `p_account`,
  `p_spend`, `p_lifecycle`, `p_transcript`, and `p_state` contents plus
  compatibility field mapping.

- [x] Estimate account creation public bytes.
  Evidence:
  `path2_native_byte_state_model_v1.md` estimates 244-392 bytes for the active
  native target before residual proof systems.

- [x] Estimate account state bytes.
  Evidence:
  `path2_native_byte_state_model_v1.md` estimates 145-320 bytes.

- [x] Estimate signer private state bytes.
  Evidence:
  `path2_native_byte_state_model_v1.md` estimates 128-416 bytes.

- [x] Estimate spend signature bytes impact.
  Evidence:
  `path2_native_byte_state_model_v1.md` estimates 0-32 bytes.

- [x] Estimate verifier compute.
  Evidence:
  `path2_native_byte_state_model_v1.md` identifies hashes/equality checks plus
  open `O_gluing` q/module checks as the only potentially nontrivial native
  verifier compute.

- [x] Identify residual cryptographic assumptions.
  Requirement: classify assumptions as native structural, SIS/module-SIS,
  hash/QROM, proof-system, or implementation/state-machine.
  Progress:
  `research/path2_residual_assumption_ledger_v1.md` separates active native
  structural assumptions, ordinary hash/domain assumptions, downstream
  checked-core assumptions, authorization-local assumptions, and fallback
  proof-system assumptions.

- [x] Decide whether masked-trace fallback remains necessary.
  Progress:
  Keep fallback until concrete `O_gluing`, lift nonemptiness, authorization
  class binding, spend closure, and fiber privacy are solved structurally.

- [x] Create `research/path2_native_byte_state_model_v1.md`.
  Requirement: reset byte/state estimates around semantic projections of `q`,
  not the old proof-family envelope.
  Evidence:
  `research/path2_native_byte_state_model_v1.md`.

- [x] Create `research/path2_residual_assumption_ledger_v1.md`.
  Requirement: separate active native structural assumptions, ordinary
  hash/QROM/domain assumptions, checked-core downstream assumptions, and
  fallback proof-system assumptions.
  Evidence:
  `research/path2_residual_assumption_ledger_v1.md`.

- [x] Create `research/path2_oglue_instantiation_v1.md`.
  Requirement: instantiate concrete `O_gluing` as the first active native proof
  move.
  Evidence:
  `research/path2_oglue_instantiation_v1.md`.

- [x] Define concrete `O_gluing` edge modules.
  Evidence:
  `O_gluing = E_AS × E_AL × E_ST × E_LT × E_LD × E_AD × E_SD`.

- [x] Define concrete `gluing_defect(q)`.
  Evidence:
  `gluing_defect(q)` is the tuple of account-spend, account-lifecycle,
  spend-transcript, lifecycle-transcript, lifecycle-state, account-state, and
  spend-state edge defects.

- [x] Select first carrier for `O_gluing`.
  Evidence:
  Product of equality/digest checks selected as `active_first_instantiation`;
  q12289 syndrome and Cech/coboundary carriers are deferred refinements.

- [x] Separate public boundary gluing from private lift nonemptiness.
  Evidence:
  `path2_oglue_instantiation_v1.md` states public boundary theorem target and
  identifies lift nonemptiness as the next blocker.

- [x] Create `research/path2_lift_nonemptiness_v1.md`.
  Requirement: prove or state `obs_c(q)=0 => H_c(q) nonempty` under the split
  public gluing then private fiber construction model.
  Evidence:
  `research/path2_lift_nonemptiness_v1.md` shows that unconditional lift
  nonemptiness is too strong as the default consensus theorem. It splits
  safety admission from honest wallet/keygen spendability and keeps
  spendability-certified admission as an optional stronger mode.

- [x] Update `Descender/Path2NativeAccountDesign.lean` with the safety vs
  spendability theorem split.
  Requirement: add public admission soundness and honest creation soundness
  interfaces, while keeping full lift extraction as an optional stronger
  certificate.
  Evidence:
  `Descender/Path2NativeAccountDesign.lean` now defines
  `PublicAdmissionSoundness`, `HonestCreationSoundness`,
  `NativePublicAccountDesignCertificate`, and theorem wrappers
  `public_admission_or_break`, `public_boundary_sound_of_zero_obstruction`, and
  `honest_create_local_valid`. `PublicAdmissionSoundness` now returns
  `CheckedObstructionZero`, which exposes explicit context, pullback, gluing,
  meaning, spend-state, and canonicality component checks instead of a bare
  no-obstruction flag. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Create `research/path2_honest_create_sampler_v1.md`.
  Requirement: specify `HonestCreate_c(seed)=Some(q,h)` and prove that it emits
  `obs_c(q)=0`, `LocalValid_c(q,h)`, and
  `AuthClass_c(h.auth)=p_spend(q)` without exposing `h` to consensus.
  Evidence:
  `research/path2_honest_create_sampler_v1.md` pins the private-to-public
  construction order: sample authorization-local material first, derive its
  public authorization class, then set `p_spend(q)` to that class and build
  coherent public faces from one semantic source.

- [x] Create `research/path2_authorization_class_binding_proof_v1.md`.
  Requirement: prove the highest-risk remaining theorem:
  `auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)` for the active
  honest-create construction order, or identify exactly which semantic class
  must move into `q`.
  Evidence:
  `research/path2_authorization_class_binding_proof_v1.md` shows the theorem
  is achievable if `auth_compatible_c` is typed over the public class
  `p_spend(q)`, and lists every spend-semantic coordinate that must move into
  `q` if binding fails.

- [x] Add Lean predicate for authorization compatibility over public class.
  Evidence:
  `Descender/Path2NativeAccountDesign.lean` now defines
  `AuthorizationCompatibleOverClass` plus theorem wrappers
  `auth_compatible_over_class` and
  `honest_create_authorization_over_class`. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Create `research/path2_spend_projection_completeness_audit_v1.md`.
  Requirement: audit whether `p_spend(q)` and `AuthClass_c(auth)` include every
  public spend-semantic coordinate consumed by checked-core, state transition,
  replay, lifecycle, and aggregation verification.
  Evidence:
  `research/path2_spend_projection_completeness_audit_v1.md` found two
  projection gaps: `allowed_state_transition_class` and `aggregation_mode`.
  `research/path2_projection_surfaces_v1.md` now includes both in `p_spend(q)`,
  and `research/path2_oglue_instantiation_v1.md` now edge-links them through
  `E_SD` and `E_ST`.

- [x] Create `research/path2_checked_core_frame_binding_v1.md`.
  Requirement: prove or specify that checked-core spend verification binds the
  frame
  `frame(p_spend(q), p_lifecycle(q), p_transcript(q), p_state(q), msg, state,
  p_context(q))` and no independent semantic handle.
  Evidence:
  `research/path2_checked_core_frame_binding_v1.md` specifies the native frame
  theorem and disallows old independent opening/pad/carrier handles as semantic
  inputs. `Descender/Path2NativeAccountDesign.lean` now gives
  `CheckedCoreFrameBinding.frame` an explicit proof-commitment input and an
  ordered symbolic `OrderedFrameEncoder` field-list binding, then adds
  `checked_core_frame_eq`. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Create `research/path2_fiber_privacy_no_export_v1.md`.
  Requirement: prove or specify that authorization-local, gauge, and randomizer
  fields are not exported by public account creation, spend verification, error
  paths, helper APIs, or checked-core frame construction.
  Evidence:
  `research/path2_fiber_privacy_no_export_v1.md` defines the allowed public
  view, forbidden private exports, component-level privacy rules, checked-core
  interaction, error-path/helper-API rules, and theorem target. The Lean
  interface now includes `PublicViewPredicates`, `FiberNoExportSoundness`,
  `NativePublicAccountDesignCertificate.fiberNoExportSoundness`, and
  `public_view_simulatable_or_break`. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Create `research/path2_spend_closure_final_composition_v1.md`.
  Requirement: compose public admission, honest creation, authorization class
  binding, spend projection completeness, checked-core frame binding, and
  fiber no-export into the final active Path 2 proof shape.
  Evidence:
  `research/path2_spend_closure_final_composition_v1.md` states the active
  final theorem and remaining concrete instantiation gaps. Lean now includes
  `public_admission_spend_action_or_break`, composing `PublicAdmissionSoundness`
  with `ProjectionCompleteness` and `CheckedCoreSpendReduction`; the reduction
  derives `SpendClosure` only after checked-core evidence, frame projection
  binding, authorization-class binding, lifecycle binding, state-transition
  binding, and replay-scope binding are supplied. The wrapper returns
  `SpendActionMatchesMeaning` so action authorization, lifecycle, and state
  fields agree with the corresponding components of `meaningOfBoundary(q)`.
  Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Refresh `research/path2_native_byte_state_model_v1.md`.
  Requirement: account for the strengthened `p_spend(q)` fields
  `allowed_state_transition_class` and `aggregation_mode`, and confirm whether
  the current account creation/state byte ranges still hold.
  Evidence:
  `research/path2_native_byte_state_model_v1.md` now records preferred compact
  ranges unchanged at account creation `244-392` bytes and account state
  `145-320` bytes if `allowed_state_transition_class` is edge-linked through
  `p_state(q)`. Conservative stress ranges are account creation `244-424` bytes
  and account state `145-352` bytes if a separate state-transition-class digest
  is persisted.

- [x] Create `research/path2_projection_encoding_instantiation_v1.md`.
  Requirement: decide concrete encodings for `p_spend(q)` and `p_state(q)`,
  especially whether `allowed_state_transition_class` is an edge-linked field
  of `p_state(q)` or a separate explicit digest.
  Evidence:
  `research/path2_projection_encoding_instantiation_v1.md` selects the
  edge-linked encoding: `allowed_state_transition_class` is derived from
  `p_state(q)` and enforced by `E_SD`; `aggregation_mode` is packed in
  mode/context bytes and edge-linked through context/transcript checks.
  `research/path2_native_byte_state_model_v1.md` now marks the preferred
  compact target as active and the +32-byte range as stress fallback only.

- [x] Create `research/path2_concrete_oglue_encoding_v1.md`.
  Requirement: instantiate concrete byte/equality checks for `E_SD`, `E_ST`,
  and context pullback under the edge-linked encoding decision.
  Evidence:
  `research/path2_concrete_oglue_encoding_v1.md` defines public deterministic
  checks for `E_SD`, `E_ST`, and context/pullback consistency. Lean now includes
  `ConcreteGluingFields`, `ConcreteGluingChecks`,
  `ConcreteGluingConsequences`, and `concrete_gluing_consequences`. Verified
  with `lake build Descender.Path2NativeAccountDesign` and `lake env lean
  Descender.lean`.

- [x] Create `research/path2_pstate_canonicality_v1.md`.
  Requirement: prove or specify that `p_state(q)` uniquely determines
  `state_transition_class`, so the edge-linked `E_SD` encoding does not need a
  separate 32-byte state-transition-class digest.
  Evidence:
  `research/path2_pstate_canonicality_v1.md` defines
  `state_transition_class` as a canonical domain-separated tuple over the full
  `p_state(q)` boundary plus context, not merely `account_state_root_before`.
  `research/path2_projection_surfaces_v1.md` now states this derived class.
  Lean now includes `StateProjectionFields`, `StateProjectionCanonicality`, and
  `pstate_transition_class_unique`. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Create `research/path2_context_state_domain_binding_v1.md`.
  Requirement: prove or specify that the context fields used by
  `StateTransitionClass_c(q)` are uniquely bound to `p_context(q)` and cannot
  be replayed across chain/app domain, fork/epoch scope, or aggregation mode.
  Evidence:
  `research/path2_context_state_domain_binding_v1.md` extends the
  state-transition class frame with native boundary version, parameter set,
  verifier family, spend domain, chain/app domain, fork/epoch scope, and
  aggregation mode. `research/path2_pstate_canonicality_v1.md` and
  `research/path2_projection_surfaces_v1.md` now list the full context. Lean
  `StateProjectionFields` and `StateProjectionCanonicality` now include the
  full context components. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Create `research/path2_ocontext_wellformedness_v1.md`.
  Requirement: specify `O_context` well-formedness for version/domain fields,
  allowed aggregation modes, reserved-mode activation, and parameter/verifier
  family validity.
  Evidence:
  `research/path2_ocontext_wellformedness_v1.md` specifies validity checks for
  native boundary version, parameter set, verifier family, domains, fork/epoch
  scope, and aggregation mode. Lean now includes `ContextFields`,
  `ContextPredicates`, `ContextWellFormedness`, and `context_wellformed`.
  Verified with `lake build Descender.Path2NativeAccountDesign` and `lake env
  lean Descender.lean`.

- [x] Create `research/path2_checked_core_frame_byte_audit_v1.md`.
  Requirement: audit the concrete checked-core frame byte fields against the
  native projection frame, confirming no independent slot/opening/pad/carrier
  handle has semantic authority.
  Evidence:
  `research/path2_checked_core_frame_byte_audit_v1.md` finds that the visible
  `encode_hidden_source_fallback_checked_core_transcript_frame` helper encodes
  fallback response-transcript evidence, not the active native checked-core
  challenge frame. It requires a new `Path2NativeCheckedCoreFrameV1` or wrapper
  proving the checked-core challenge preimage includes the native projection
  frame.

- [x] Create `research/path2_native_checked_core_frame_v1_spec.md`.
  Requirement: specify exact field order, domain tags, canonical encodings, and
  mutation tests for the native checked-core challenge frame.
  Evidence:
  `research/path2_native_checked_core_frame_v1_spec.md` defines
  `Path2NativeCheckedCoreFrameV1`, exact field order, forbidden legacy semantic
  fields, wrapper rule, and mutation tests. Lean now includes
  `NativeCheckedCoreFrameFields`, `NativeCheckedCoreFrameBinding`, and
  `native_checked_core_frame_eq`. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake env lean Descender.lean`.

- [x] Create `research/path2_public_view_no_export_allowlist_v1.md`.
  Requirement: define the active native public-view serializer allowlist and
  private denylist so no authorization-local, gauge, randomizer, opening, pad,
  or raw representative coordinate is exported by account creation, spend
  verification, errors, or helper APIs.
  Evidence:
  `research/path2_public_view_no_export_allowlist_v1.md` defines active native
  account/spend public views, a private denylist, conditional fallback-only
  fields, error/helper API rules, and KAT requirements. Lean now includes
  `PublicOutputPolicy`, `PublicOutputNoExport`, and `public_output_no_export`.
  Verified with `lake build Descender.Path2NativeAccountDesign` and `lake env
  lean Descender.lean`.

- [x] Create `research/path2_proof_side_closure_status_v1.md`.
  Requirement: summarize the current proof-side closure status, distinguish
  closed theorem-interface blockers from remaining concrete instantiation and
  implementation-facing gates, and identify the next highest-risk proof item.
  Evidence:
  `research/path2_proof_side_closure_status_v1.md` states that no unresolved
  hidden-witness extraction blocker remains in the active native proof shape;
  remaining gates are concrete encoders, checked-core frame implementation
  binding, serializer no-export KATs, and hash/domain assumption refresh.

- [x] Refresh `research/path2_residual_assumption_ledger_v1.md`.
  Requirement: update stale native assumption statuses after proof-side closure
  work.
  Evidence:
  `research/path2_residual_assumption_ledger_v1.md` now distinguishes active
  theorem obligations, remaining native instantiation gates, ordinary
  hash/domain assumptions, checked-core downstream assumptions,
  authorization-local assumptions, fallback assumptions, retired paths, and
  deployability closure criteria.

- [x] Create `research/path2_native_checked_core_frame_kat_plan_v1.md`.
  Requirement: define implementation-facing encoder tests and mutation KATs for
  `Path2NativeCheckedCoreFrameV1`.
  Evidence:
  `research/path2_native_checked_core_frame_kat_plan_v1.md` defines golden
  vector, decode/offset, mutation, rejection, legacy-handle, cross-projection,
  timing, and no-export KATs for the native checked-core frame encoder.

- [x] Implement `Path2NativeCheckedCoreFrameV1` encoder/decoder and KATs in the
  reference code.
  Requirement: add the concrete frame type and tests matching
  `research/path2_native_checked_core_frame_v1_spec.md` and
  `research/path2_native_checked_core_frame_kat_plan_v1.md`.
  Evidence:
  `tools/parameter-search/src/production_reference.rs` now defines
  `Path2NativeCheckedCoreFrameV1`, validates version/parameter/verifier/
  aggregation mode, encodes/decodes the canonical 13-digest frame, and derives
  the challenge from that exact byte preimage. KATs cover golden layout,
  round-trip decoding, semantic field mutation, invalid context rejection, and
  exclusion of legacy opening/pad/hidden-source/carrier handles. The public
  facade re-exports the type, constants, encoder, decoder, challenge helper,
  and validator.

## Phase 8: Dead-Path Criteria

Mark a candidate `[D]` if:

- [ ] any consensus field cannot factor through `q`;
- [ ] two representatives over one `q` can differ in spend/replay/lifecycle
  behavior;
- [ ] public validity does not imply existence of an account-backed local lift;
- [ ] public data is injective in private representative-specific material;
- [ ] spend cannot be closed over `q` plus signer-held local authorization;
- [ ] the core validity condition immediately requires a generic hidden-witness
  ZK proof;
- [ ] the candidate recreates independent role commitments plus a later
  equality proof.

## Immediate Next Item

- [x] Implement active native public-view serializer no-export KATs.
  Requirement: add first-class reference serializers/tests for the allowlisted
  account/spend public views in
  `research/path2_public_view_no_export_allowlist_v1.md`, proving private
  representative, gauge, pad, opening, and fallback carrier material cannot
  enter account creation or spend verification outputs.
  Evidence:
  `tools/parameter-search/src/production_reference.rs` now defines
  `PublicNativeAccountViewV1` and `PublicNativeSpendViewV1` as narrow
  active-native serializers. The account view exposes only context IDs,
  aggregation mode, native boundary/projection digests, lifecycle marker,
  public state root, and obstruction digest. The spend view exposes only the
  native boundary digest, checked-core frame digest, message/state digests,
  proof commitment, and checked-core proof byte digest/length. KATs assert
  allowlisted field presence, denylisted private/fallback names absence, raw
  proof bytes exclusion, mutation sensitivity for proof bytes, and rejection of
  invalid native context.

## Next Proof/Implementation Bridge

- [x] Add native projection preimage encoder specs and KATs.
  Requirement: define the byte encoders that produce `p_context_digest`,
  `p_account_digest`, `p_spend_digest`, `p_lifecycle_digest`,
  `p_transcript_digest`, and `p_state_digest` from the native boundary object,
  then add KATs proving each digest changes under the corresponding public
  projection mutation and does not depend on private representative/gauge
  material.
  Evidence:
  `research/path2_native_projection_preimage_encoders_v1_spec.md` now fixes the
  domain tags, field order, byte encodings, digest rule, native boundary digest
  composition, no-export rule, and KAT requirements for all six projection
  preimages. `tools/parameter-search/src/production_reference.rs` now defines
  typed context/account/spend/lifecycle/state/transcript projection structs,
  canonical byte encoders, digest helpers, and native boundary digest
  composition. KATs cover stable layouts, digest equality to `SHA256(bytes)`,
  transcript/boundary mutation cascade, context rejection, spend verifier
  rejection, and private sentinel absence. The production facade re-exports the
  projection types, constants, encoders, digest helpers, and boundary digest
  helper.

## Completed Bridge

- [x] Add a native boundary assembly helper and KATs.
  Requirement: build one typed helper that takes the six public projection
  structs plus spend message/state/proof commitment, derives every `p_*_digest`,
  computes `native_boundary_digest`, constructs `Path2NativeCheckedCoreFrameV1`,
  and emits `PublicNativeAccountViewV1` / `PublicNativeSpendViewV1`. KATs must
  prove all emitted digests come from the same projection bundle and that
  mutating any projection changes the boundary/frame/view consistently.
  Evidence:
  `tools/parameter-search/src/production_reference.rs` now defines
  `Path2NativeProjectionBundleV1`, `Path2NativeBoundaryAssemblyV1`, and
  `assemble_path2_native_boundary_v1`. The helper derives all child projection
  digests, computes an acyclic `native_boundary_digest` over context/account/
  spend/lifecycle/state digests, derives the transcript digest after the
  boundary digest, builds `Path2NativeCheckedCoreFrameV1`, and emits the active
  native account/spend public views. KATs prove one consistent public surface,
  reject mismatched public-key identity and aggregation mode, and show
  projection mutation cascades through boundary digest, checked-core frame, and
  spend view.

## Immediate Next Item

- [x] Add Lean/interface mirror for the acyclic assembly rule.
  Requirement: update the proof-side model so `nativeBoundaryDigest` is a
  function of the non-transcript projection digests, while `p_transcript(q)`
  binds that boundary digest. This closes the byte-level circularity discovered
  during implementation and gives the theorem side the same assembly order as
  the reference code.
  Evidence:
  `Descender/Path2NativeAccountDesign.lean` now defines
  `NativeProjectionDigestFields`, `NativeBoundaryDigestAssembly`,
  `NativeTranscriptDigestAssembly`, `NativeAcyclicAssembly`, and theorems
  `native_boundary_digest_acyclic_eq`,
  `native_transcript_digest_binds_boundary_eq`, and
  `native_acyclic_assembly_eqs`. `Descender/Path2NativeBoundary.lean` mirrors
  the byte implementation with `ProjectionDigestFields`,
  `assembledNativeBoundaryDigest`, `assembledTranscriptDigest`,
  `AcyclicProjectionDigestAssembly`, and theorem wrappers for boundary and
  transcript assembly. Verified with `lake build
  Descender.Path2NativeAccountDesign` and `lake build
  Descender.Path2NativeBoundary`.

## Immediate Next Item

- [x] Add a proof-side assembly-to-checked-core binding theorem.
  Requirement: connect `NativeAcyclicAssembly` / `AcyclicProjectionDigestAssembly`
  to `NativeCheckedCoreFrameBinding`, proving that a checked-core frame built
  from an assembled boundary necessarily uses the derived boundary,
  transcript, account, spend, lifecycle, and state digests rather than
  independently supplied digest handles.
  Evidence:
  `Descender/Path2NativeAccountDesign.lean` now defines
  `NativeCheckedCoreFrameDigestBinding` and theorem
  `native_assembled_checked_core_frame_eq`, tying checked-core frame digest
  fields to the assembled projection digest fields. The theorem expands the
  acyclic boundary digest and transcript digest as ordered symbolic frame
  encodings inside the checked-core frame equation, so the proof interface
  cannot satisfy `NativeCheckedCoreFrameBinding` with independently chosen
  digest handles or order-insensitive arithmetic. Verified with `lake build
  Descender.Path2NativeAccountDesign`.

## Immediate Next Item

- [x] Add a proof-side public-view binding theorem.
  Requirement: connect the assembled projection digests to
  `PublicOutputNoExport` / public-view predicates, proving that the active
  native account/spend public views are simulatable from the assembled public
  boundary, message/state digests, and proof digest/length, not from private
  representative material.
  Evidence:
  `Descender/Path2NativeAccountDesign.lean` now defines
  `NativePublicViewFields`, `NativePublicViewAssembly`, and theorem
  `native_public_views_assembled_eqs`. The theorem expands account and spend
  public views through acyclic assembled projection digests plus public
  message/state/proof digest and length handles, with no private fiber input.
  Verified with `lake build Descender.Path2NativeAccountDesign`.

## Immediate Next Item

- [x] Add Rust/Lean cross-check KAT notes for assembly theorem coverage.
  Requirement: document the correspondence between
  `assemble_path2_native_boundary_v1` KATs and Lean theorems
  `native_assembled_checked_core_frame_eq` /
  `native_public_views_assembled_eqs`, including the exact assumptions that
  remain outside Lean's arithmetic model: SHA-256 collision resistance,
  canonical byte encoding, and serializer denylist enforcement.
  Evidence:
  `research/path2_native_rust_lean_assembly_crosscheck_v1.md` maps Rust KATs
  for projection preimages, boundary assembly, checked-core frames, and
  public-view no-export to Lean interfaces/theorems. It also names the
  remaining outside-Lean assumptions: SHA-256 binding/collision resistance,
  canonical byte encoding, serializer denylist enforcement, and domain
  separation.

## Immediate Next Item

- [x] Add active native domain/schema registry check.
  Requirement: verify active native projection domains, boundary/transcript/
  checked-core frame domains, and public-view schemas are unique and that the
  active native public serializers are the intended validator-facing surfaces.
  This should become a focused Rust KAT or small reference helper, and should
  update the residual assumption ledger for domain separation.
  Evidence:
  `tools/parameter-search/src/production_reference.rs` now includes
  `path2_active_native_domains_and_public_schemas_are_unique`, covering active
  native projection domains, boundary domain, checked-core frame domain, and
  public-native account/spend schemas. The KAT checks uniqueness, Path 2
  namespace prefixes, explicit version suffixes, and no domain/schema
  collisions.

## Immediate Next Item

- [x] Refresh `research/path2_residual_assumption_ledger_v1.md`.
  Requirement: record that active native domain/schema uniqueness now has a
  Rust KAT, while SHA-256 collision resistance, QROM modeling, serde/parser
  behavior, and denylist completeness remain assumptions or implementation
  obligations.
  Evidence:
  `research/path2_residual_assumption_ledger_v1.md` now distinguishes
  implemented active-native projection/frame/view/assembly/carrier/preflight
  KATs from remaining global domain registry audit, deployment QROM/hash
  assumption review, and implementation preflight. It also separates
  local active-native domain/schema uniqueness, covered by
  `path2_active_native_domains_and_public_schemas_are_unique`, from broader
  cryptographic domain-separation and SHA/QROM assumptions.

## Immediate Next Item

- [x] Define the final consensus carrier encoder for admitted `q`.
  Requirement: decide and specify the byte carrier that validators/store nodes
  actually receive for the admitted native boundary object. It must be derived
  from the projection bundle/assembly, avoid private material, and make clear
  whether it carries expanded projection preimages, only digests plus data
  availability references, or a hybrid.
  Evidence:
  Selected hybrid carrier documented in
  `research/path2_native_consensus_carrier_v1_spec.md`: a 330-byte fixed body
  carrying version/context fields, lifecycle marker, native/projection digests,
  public state root, obstruction digest, and a DA digest for expanded
  projection preimages. `tools/parameter-search/src/production_reference.rs`
  now defines `Path2NativeConsensusCarrierV1`, encoder/decoder/digest helpers,
  assembly-derived constructor, validation, facade exports, and KATs for
  canonical layout, round trip, digest mutation, context rejection, domain
  rejection, and no private/message/proof fields in the account-creation
  carrier.

## Immediate Next Item

- [x] Wire consensus carrier into verifier/preflight assembly.
  Requirement: add reference helper(s) that take the admitted carrier plus
  available projection preimage bundle, recompute `assemble_path2_native_boundary_v1`,
  and verify that every carrier digest matches the recomputed assembly. This is
  the executable bridge from consensus bytes to the native proof/checking
  surface.
  Evidence:
  `tools/parameter-search/src/production_reference.rs` now includes
  `verify_path2_native_consensus_carrier_v1_against_bundle`, which validates
  the carrier, recomputes assembly from the projection bundle and checked-core
  proof bytes, rebuilds the expected carrier with the supplied projection
  preimage DA digest, and rejects unless the consensus carrier matches exactly.
  KATs cover successful recomputation and rejection for carrier digest mutation,
  projection bundle mutation, and DA digest mismatch. The helper is re-exported
  through the production facade.

## Immediate Next Item

- [x] Add accepted/rejected verifier-output no-export scan.
  Requirement: create a focused preflight/verifier output KAT that exercises
  accepted carrier verification and representative rejection cases, then scans
  returned errors/reports/debug JSON for private fiber, pad, opening, gauge,
  fallback carrier, raw proof, and decoded private-lane material.
  Evidence:
  `Path2NativeConsensusCarrierPreflightReportV1` is now the validator-facing
  preflight result surface. It contains only public carrier/projection digests,
  optional carrier/recomputed-boundary digests, acceptance status, and coarse
  rejection codes. The KAT
  `path2_native_consensus_carrier_preflight_reports_do_not_export_private_material`
  covers accepted verification plus invalid carrier context, carrier/bundle
  mismatch, inconsistent projection bundle, and DA digest mismatch, scanning
  JSON and debug renderings for private fiber, pad, opening, gauge, raw proof,
  and decoded private-lane markers.

## Immediate Next Item

- [x] Audit the global active domain/schema registry.
  Requirement: extend the current local active-native domain/schema uniqueness
  KAT into a broader registry check or documented manifest so that Path 2
  native domains cannot collide with legacy reviewed/Fq12, deployment
  preflight, lifecycle, target-chain, or hidden-source fallback domains.
  Evidence:
  `production_registered_domain_schema_constants` now records the production
  domain/schema/profile manifest across active native Path 2, legacy Path 2,
  reviewed/Fq12, deployment preflight, lifecycle, target-chain, and hidden-source
  fallback surfaces. The KAT
  `production_domain_schema_registry_is_globally_disjoint` verifies printable
  versioned values, global uniqueness, required active-native entries, and no
  active-native collision with non-native surfaces.

## Immediate Next Item

- [x] Refresh the QROM/hash assumption review for deployment query bounds.
  Requirement: tie the active native domain/schema manifest and 330-byte
  consensus carrier transcript to the deployment query-budget model, making
  explicit which assumptions remain ordinary SHA-256/QROM assumptions rather
  than hidden-witness proof obligations.
  Evidence:
  `Path2NativeHashQromReviewV1` / `path2_native_hash_qrom_review_v1` now imports
  the deployment query caps from `path2_direct_query_budget_v1`, records the
  production registry entry count, pins the native consensus-carrier byte
  counts, names the active native public query symbols, and separates ordinary
  public SHA-256/QROM byte-binding assumptions from hidden-witness obligations.
  KAT `path2_native_hash_qrom_review_tracks_public_transcript_assumptions`
  checks these fields and scans the serialized review with the active native
  no-export denylist. `research/path2_hash_qrom_assumption_ledger.md` records
  the active native review.

## Immediate Next Item

- [x] Run the implementation preflight for the active native proof-side surface.
  Requirement: run the broad relevant Rust/Lean checks and record whether the
  active native carrier, projection encoders, preflight report, domain registry,
  and QROM review are all exported through the facade and consistent with the
  Lean theorem interfaces.
  Evidence:
  `cargo test --manifest-path tools/parameter-search/Cargo.toml --lib` passes
  all 130 library tests, including active native projection/frame/view/assembly,
  consensus carrier, preflight report, global domain/schema registry, and
  QROM/hash review KATs. `cargo test --manifest-path
  tools/production-reference/Cargo.toml --lib` passes all 33 facade tests after
  exporting the active native public surfaces. `lake env lean Descender.lean`
  succeeds for the current Lean theorem interface aggregate.

## Current Status

The active native proof-side public boundary is now internally preflighted:
canonical projection encoders, checked-core frame, public views, consensus
carrier, validator-facing preflight report, global domain/schema manifest, and
QROM/hash review are implemented, tested, documented, and facade-exported.

Remaining internal work is outside this proof-side closure slice:
productionizing the full signer/verifier implementation, constant-time
hardening, and storage/lifecycle hardening. External review remains excluded
from this internal tracker.

## Productization Plan

- [x] Add active native internal deployment preflight.
  Requirement: generate the post-proof-side readiness state from code, with the
  active native proof-side public boundary marked complete while production
  signer/verifier, constant-time, and storage/lifecycle work remain fail-closed.
  Evidence:
  `Path2NativeDeploymentPreflightV1` / `path2_native_deployment_preflight_v1`
  records the completed proof-side surfaces, byte bounds, facade/export
  readiness, and exactly three internal blockers:
  active native signer/verifier pipeline, constant-time audit, and target-chain
  storage/lifecycle hardening. External review is intentionally not counted as
  an internal blocker. KAT
  `path2_native_deployment_preflight_tracks_internal_productization_blockers`
  pins the report and digest.

## Immediate Next Item

- [x] Define the active native signer/verifier pipeline contract.
  Requirement: specify and implement the production-facing entrypoints that
  consume the 330-byte active native carrier plus available projection preimage
  data, call carrier preflight, bind the checked-core proof frame, and return
  only the public preflight/verdict surface. This should not reopen the proof
  theorem; it should wire the already-preflighted public boundary into the
  deployment API.
  Evidence:
  `Path2NativeVerifierPipelineVerdictV1` /
  `verify_path2_native_deployment_pipeline_v1` is now the fail-closed
  production-facing verifier wrapper. It runs active native carrier preflight,
  distinguishes public-boundary acceptance from production acceptance, emits
  only public report/verdict data, and rejects with
  `reject_active_native_pipeline_not_productized` until the deployment preflight
  blockers are closed. KAT
  `path2_native_verifier_pipeline_is_public_only_and_fail_closed_until_productized`
  covers accepted public-boundary input, carrier mismatch rejection, digest
  generation, and JSON/debug no-export scans.

## Immediate Next Item

- [x] Define the active native signer-side pipeline contract.
  Requirement: specify the production signer boundary that constructs the
  projection bundle, proof commitment, carrier, and projection-preimage DA
  digest without exporting private material, then hands only public carrier and
  checked-core proof bytes to the verifier pipeline. This should remain
  fail-closed until concrete constant-time sampling/keygen are audited.
  Evidence:
  `Path2NativeSignerPipelinePublicOutputV1` /
  `path2_native_signer_pipeline_public_output_v1` derives the public carrier,
  carrier digest, core-proof digest/length, projection-preimage DA digest, and
  native boundary transcript digests from signer-held inputs. The output remains
  fail-closed with `reject_active_native_signer_not_productized`, exposes no
  projection bundle or private material, and is verifier-compatible with
  `verify_path2_native_deployment_pipeline_v1`. KAT
  `path2_native_signer_pipeline_public_output_is_public_and_verifier_compatible`
  covers no-export serialization/debug scans and verifier handoff.

## Immediate Next Item

- [x] Define the concrete constant-time audit scope for active native signing.
  Requirement: enumerate the signer/keygen/sampler/verifier operations that are
  secret-dependent in the active native pipeline, separate public-length hashing
  from secret timing claims, and pin the test/audit artifacts required before
  `constant_time_audit_complete` can become true.
  Evidence:
  `Path2NativeConstantTimeAuditScopeV1` /
  `path2_native_constant_time_audit_scope_v1` now pins the constant-time blocker
  to eight secret-dependent active-native operations, four public variable-length
  operations, seven required audit artifacts, and explicit non-claims for public
  message length, unaudited sampler schedules, and research-only APIs. KAT
  `path2_native_constant_time_audit_scope_pins_secret_dependent_surface` checks
  the scope and digest.

## Immediate Next Item

- [x] Define the active native storage/lifecycle hardening scope.
  Requirement: enumerate the target-chain storage, lifecycle, restore-anchor,
  replay, and data-availability artifacts required before
  `storage_lifecycle_hardening_complete` can become true.
  Evidence:
  `Path2NativeStorageLifecycleHardeningScopeV1` /
  `path2_native_storage_lifecycle_hardening_scope_v1` now pins the storage
  blocker to target-chain storage artifacts, lifecycle safety artifacts,
  data-availability storage/recompute requirements, restore/reorg policy
  artifacts, seven required harnesses, and explicit non-claims for reference
  in-memory stores, account-restorable anchors, missing external storage review,
  and cross-chain replay outside declared context domains. KAT
  `path2_native_storage_lifecycle_hardening_scope_pins_consensus_storage_surface`
  checks the scope and digest.

## Cryptoanalytic Red-Team Pass

Status: paused productization to attack the active native proof shape before
more implementation work accumulates. External review remains excluded; this is
internal cryptanalysis.

- [x] Boundary sufficiency attack: same `q`, different semantics.
  Requirement: try to construct or rule out two private/local representatives
  `h1`, `h2` with the same public carrier/boundary `q` but different spend,
  lifecycle, replay, state-transition, or authorization behavior. A failure
  must name the missing projection/obstruction field or theorem obligation.
  Evidence:
  `research/path2_native_red_team_boundary_sufficiency_v1.md` records the first
  red-team result. No concrete private same-`q` representative attack was found
  in the public API surface, because signer-local/private material is not an
  input to active native projection encoders, carrier, preflight report, or
  public signer/verifier wrappers. The pass did find and fix a public gluing
  weakness: assembly now rejects spend-domain, spend-state transition, and
  lifecycle/state live-marker mismatches. KAT
  `path2_native_boundary_assembly_rejects_independent_digest_inputs` covers the
  new rejection edges.

- [x] False obstruction acceptance.
  Requirement: try to construct `q` with `obs_c(q)=0` and passing public checks
  but no valid usable account boundary. A failure must identify whether the gap
  is projection completeness, gluing soundness, native verification, or data
  availability.
  Evidence:
  `research/path2_native_red_team_false_obstruction_acceptance_v1.md` records
  the second red-team result. The pass found a concrete implementation gap:
  arbitrary `obstruction_flags_digest` values could pass carrier preflight.
  The fix adds `PATH2_NATIVE_NO_OBSTRUCTION_FLAGS_DOMAIN`,
  `path2_native_no_obstruction_flags_digest()`, production registry/facade
  exports, and an assembly rejection unless obstruction flags equal the
  canonical no-obstruction digest. KAT
  `path2_native_boundary_assembly_rejects_independent_digest_inputs` now covers
  nonzero/mutated obstruction flags. The Lean interface now separately requires
  `CheckedObstructionZero`, preventing the canonical digest from being modeled
  as a self-attested substitute for context, pullback, gluing, meaning,
  spend-state, and canonicality checks.

- [x] Authorization-class confusion.
  Requirement: try to make `AuthClass_c(h)=p_spend(q)` for an authorization
  that should not be valid for the account, context, lifecycle, message, or
  state transition.
  Evidence:
  `research/path2_native_red_team_authorization_class_confusion_v1.md` records
  the findings. The spend projection carried `allowed_authorization_local_mode`
  but accepted unknown modes. The fix adds
  `PATH2_NATIVE_AUTHORIZATION_LOCAL_MODE_DIRECT`, validates that active native
  spend projections use that mode, exports it through the facade, and extends
  `path2_native_boundary_assembly_rejects_independent_digest_inputs` to reject
  unsupported modes. A second fix adds
  `PATH2_NATIVE_AUTHORIZATION_CLASS_DOMAIN` and
  `path2_native_spend_authorization_class_digest_v1`, making
  `spend_authorization_class_digest` derive from the complete public spend
  semantic tuple rather than accepting an independent label. A third pass after
  checked-core equation wiring found that `public_key_identity` and
  `checked_core_verifier_handle_digest` were still independent. The fix adds
  `PATH2_NATIVE_PUBLIC_KEY_IDENTITY_DOMAIN` and
  `path2_native_public_key_identity_digest_v1`, and assembly now rejects unless
  account/spend public-key identity is derived from the checked-core verifier
  handle. This forces the spend class, account identity, and checked-core proof
  equation through the same public authorization coordinate.

- [x] DA digest / projection-preimage ambiguity.
  Requirement: try to bind the 330-byte carrier to one public semantic surface
  while projection-preimage DA material admits a different interpretation.
  Evidence:
  `research/path2_native_red_team_da_projection_preimage_ambiguity_v1.md`
  records the result. The pass found a concrete consensus-binding gap:
  `projection_preimage_da_digest` was externally supplied to carrier preflight,
  so preflight only checked consistency with the caller-selected digest rather
  than deriving it from the canonical projection bundle. The fix adds
  `PATH2_NATIVE_PROJECTION_PREIMAGE_DA_DOMAIN`,
  `path2_native_projection_preimage_da_v1_bytes`, and
  `path2_native_projection_preimage_da_digest_v1`; verifier preflight and
  deployment pipeline now recompute the digest from the bundle, and signer
  public output derives it before carrier construction. Residual DA availability
  guarantees are target-chain storage/lifecycle hardening, not a compact carrier
  ambiguity.

- [x] No-export leakage red team.
  Requirement: test whether carrier fields, preflight reports, lifecycle
  markers, obstruction flags, or projection digests distinguish private fibers
  beyond the intended public class.
  Evidence:
  `research/path2_native_red_team_no_export_leakage_v1.md` records the pass.
  No raw private-field export was found in the active native carrier, account
  view, spend view, preflight report, verifier verdict, or signer public output.
  The pass did find an avoidable leakage-shape issue: public spend views exposed
  `checked_core_proof_byte_len` while accepting arbitrary proof lengths. The fix
  adds `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES` and makes
  `public_native_spend_view_v1` reject any non-fixed checked-core proof length,
  which also gates assembly, carrier preflight, verifier pipeline, and signer
  public output. Residual hiding of the proof bytes themselves remains a
  checked-core proof-family obligation.

- [x] QROM/domain multi-target replay review.
  Requirement: check whether active native domains, digest fanout, or
  deployment query bounds leave a multi-target, cross-context, or replay surface
  not charged in the current QROM/hash review.
  Evidence:
  `research/path2_native_red_team_qrom_domain_replay_v1.md` records the pass.
  The active domain registry was globally disjoint and the DA digest was already
  charged, but the red-team fixes had introduced two registered hash surfaces
  that were not explicit in `Path2NativeHashQromReviewV1`: authorization-class
  digest and no-obstruction digest. The fix adds
  `q_native_authorization_class` and `q_native_no_obstruction_flags` to the
  active native query symbols and extends ordinary hash assumptions to name
  those tuple/flag bindings.

- [x] Checked-core composition review.
  Requirement: verify that checked-core equation soundness, message binding,
  proof-commitment timing, and native boundary binding compose without a
  gap or stale legacy-frame dependency.
  Evidence:
  `research/path2_native_red_team_checked_core_composition_v1.md` records the
  result and fix. Native frame binding, field mutation, legacy-handle
  exclusion, assembly-derived frame construction, and fixed proof length are
  covered. The implementation now adds `path2_native_checked_core_accept_v1`
  and wires it into `assemble_path2_native_boundary_v1`, so carrier preflight
  cannot accept unless the reviewed Fq12 proof commitment matches the native
  frame, the challenge is derived from the native frame, and the Fq12 response
  syndrome equation matches the spend `checked_core_verifier_handle_digest`.
  `checked_core_composition_review_complete` is now true; remaining blockers are
  signer/verifier productization, constant-time audit, and storage/lifecycle
  hardening.

## Phase 9: Lean/Rust Instantiation Bridge

Purpose: after the core algorithm red-team patches, prevent the proof from
closing over abstract names that are not yet tied to concrete byte-level
relations.

Evidence:
`research/path2_lean_rust_instantiation_bridge_checklist_v1.md`.

- [~] Checked-core spend reduction bridge.
  Requirement: instantiate Lean `CheckedCoreSpendReduction` with the Rust
  relation accepted by `path2_native_checked_core_accept_v1` over the
  `Path2NativeCheckedCoreFrameV1` assembled by
  `assemble_path2_native_boundary_v1`.
  Current evidence:
  native frame construction, checked-core challenge derivation, proof-commitment
  equality, response-syndrome equation, and mutation KATs exist. Remaining work
  is the explicit bridge statement and assumption export for downstream Fq12
  equation soundness.
  Evidence:
  `research/path2_checked_core_spend_reduction_bridge_v1.md` defines the
  concrete bridge candidate, maps acceptance checks to Lean fields, and names
  remaining hash/parser/equation assumptions.

- [~] Checked obstruction-zero component bridge.
  Requirement: map `CheckedObstructionZero` component predicates to concrete
  Rust checks for context, pullback, gluing, meaning, spend-state, and
  canonicality. The canonical no-obstruction digest is not sufficient by
  itself; it is accepted only after the component checks are identified.
  Evidence:
  `research/path2_checked_obstruction_zero_bridge_v1.md` maps each component
  predicate to current Rust anchors and identifies three dependent blockers:
  projection completeness, lifecycle consume-once/storage semantics, and
  ordered encoder/hash-binding.

- [~] Projection completeness bridge.
  Requirement: define the concrete `q` object represented by the native
  projection bundle plus derived boundary/transcript/frame fields, then link
  each `ProjectionCompleteness` field to the exact projection encoder and
  public output derivation.
  Evidence:
  `research/path2_projection_completeness_bridge_v1.md` defines concrete `q`
  as `Path2NativeProjectionBundleV1` plus successful
  `Path2NativeBoundaryAssemblyV1`, maps `p_spend`, `p_lifecycle`,
  `p_transcript`, and `p_state` to Rust encoders, and classifies the compact
  carrier as a consensus view rather than the whole proof object.

- [~] Private fiber meaning bridge.
  Requirement: name the honest signer/keygen private state type that
  instantiates `PrivateFiber`, name the constructor emitting `(q,h)` or the
  equivalent public/private pair, and show private gauge/randomizer changes do
  not change `meaningOfFiber`.
  Evidence:
  `research/path2_private_fiber_meaning_bridge_v1.md` defines the intended
  private fiber decomposition, maps current public anchors, and marks the
  honest constructor/productized signer pipeline as the remaining blocker.
  Rust now has `Path2NativeHonestConstructorScopeV1` and KAT
  `path2_native_honest_constructor_scope_pins_private_fiber_bridge`, which pin
  the required private-fiber components, public outputs, invariance laws,
  artifacts, and negative-control harnesses.
  Progress:
  Added draft executable constructor objects
  `Path2NativePrivateFiberStateV1`,
  `Path2NativeHonestConstructorSecretMaterialV1`,
  `Path2NativeBoundaryMeaningDigestV1`, and
  `Path2NativeHonestConstructorDraftV1`, plus
  `path2_native_honest_constructor_draft_v1` and
  `path2_native_honest_constructor_from_secret_material_v1`. The constructor
  now derives private-fiber digests from explicit secret material, checks that
  the proof randomizer syndrome matches the public proof commitment, generates
  checked-core proof bytes via `path2_native_checked_core_proof_from_bundle_v1`,
  using the same native frame derivation as `assemble_path2_native_boundary_v1`,
  and hands the resulting public boundary to verifier preflight. KATs cover
  private-fiber binding to public `q`, same-`q` gauge/randomizer invariance,
  authorization-class mismatch rejection, generated-proof verifier
  compatibility, assembly/proof-generation frame-derivation equality, local
  seed-buffer zeroization, and proof-randomizer mismatch rejection. Still open:
  replace the bridge constructor with a productized signer/keygen constructor
  with production entropy separation, real secret ownership, full zeroization,
  and constant-time coverage.
  New productization scope:
  `Path2NativeSignerKeygenProductizationScopeV1` /
  `path2_native_signer_keygen_productization_scope_v1` pins the remaining
  signer/keygen blocker to entropy streams, secret ownership rules,
  constructor contracts, zeroization requirements, and required harnesses. KAT
  `path2_native_signer_keygen_productization_scope_pins_secret_ownership_blocker`
  checks the scope and production registry includes the new schema.
  Ownership hardening:
  `Path2NativeHonestConstructorSecretMaterialV1` is now non-`Copy`, has no
  `Debug`/`Serialize` surface, and explicitly zeroizes owned seeds plus the
  proof-randomizer syndrome. KAT
  `path2_native_secret_material_zeroize_clears_all_owned_buffers` covers this
  bridge behavior. Remaining zeroization work is production integration over
  real signer/keygen buffers and rejection paths.
  Entropy bridge:
  `Path2NativeSignerEntropyRootV1` /
  `path2_native_secret_material_from_entropy_root_v1` derives authorization,
  gauge, randomizer, recovery, and proof-randomizer material from an owned root
  under `PATH2_NATIVE_SIGNER_ENTROPY_DERIVATION_DOMAIN` and the public `q`
  binding. KATs
  `path2_native_entropy_root_derives_domain_separated_secret_material`,
  `path2_native_entropy_root_binding_mutation_changes_secret_material`, and
  `path2_native_entropy_root_zeroize_clears_owned_buffers` cover stream
  separation, public-binding sensitivity, canonical field elements, constructor
  compatibility, and root zeroization.
  OS entropy and constructor bridge:
  `path2_native_signer_entropy_root_from_os_v1` wires `getrandom` OS entropy
  into the entropy-root type; `path2_native_honest_constructor_from_os_entropy_v1`
  derives the proof commitment internally and emits verifier-compatible
  fail-closed public output. KATs
  `path2_native_os_entropy_root_is_nonzero_and_domain_separable`,
  `path2_native_entropy_q_sampler_uses_fixed_attempt_budget`,
  `path2_native_entropy_constructor_derives_commitment_and_verifier_compatible_output`,
  and `path2_native_os_entropy_constructor_keeps_public_output_fail_closed`
  cover this surface. Remaining signer/keygen work is now whole-signer
  constant-time and rejection-cleanup evidence; the constructor bridge itself is
  no longer missing.

- [x] Add a sanitized signer execution report for the entropy-root constructor.
  Requirement: exercise the signer-facing constructor path through a public
  report that exposes only accept/reject booleans, coarse rejection codes,
  public digests, and the fail-closed deployment preflight. Constructor,
  entropy, or verifier failures must not serialize root entropy, secret
  material, private fiber fields, checked-core proof bytes, or internal error
  strings.
  Result:
  `Path2NativeSignerExecutionReportV1`,
  `path2_native_signer_execution_report_from_entropy_root_v1`,
  `path2_native_signer_execution_report_from_os_entropy_v1`, and
  `path2_native_signer_execution_report_v1_digest` now wrap the constructor
  surface in a digest-only report. Success remains fail-closed under
  `reject_active_native_signer_not_productized`; constructor failures collapse
  to `reject_active_native_signer_constructor_failed`; OS entropy failures
  collapse to `reject_active_native_signer_entropy_failed`. KATs
  `path2_native_signer_execution_report_success_is_digest_only_and_fail_closed`
  and
  `path2_native_signer_execution_report_rejection_is_sanitized_and_digest_only`
  cover the positive and rejection surfaces. This closes the public
  error-serialization part of the blocker; it does not close the compiler,
  runtime, memory-access, or full cleanup constant-time audit.

- [x] Pin the full signer fixed-stage public audit shape.
  Requirement: define the stage schedule that a production signer/keygen must
  make secret-independent, and compare success/rejection reports through a
  public, digest-only artifact. The report must not claim constant-time
  completion until branch, memory-access, timing, and cleanup evidence exists.
  Result:
  `Path2NativeSignerConstantTimeStageReportV1`,
  `path2_native_signer_constant_time_stage_labels_v1`,
  `path2_native_signer_constant_time_stage_report_from_reports_v1`, and
  `path2_native_signer_constant_time_stage_report_v1_digest` now pin an
  8-stage public schedule: deployment preflight snapshot, entropy root
  acquisition, secret derivation schedule, commitment derivation, private-fiber
  binding check, checked-core proof generation, public-boundary verification,
  and public report sanitization. KAT
  `path2_native_signer_constant_time_stage_report_pins_fixed_public_shape`
  verifies success and rejection reports share this fixed stage shape, reject
  codes are public-only, report digests differ, private material is absent, and
  `audit_complete` remains false. Remaining CT work is now narrowed to proving
  the concrete implementation follows this schedule without secret-dependent
  branches, secret-dependent memory access, or rejection cleanup leakage.

- [x] Remove the entropy-to-field sampler exhaustion rejection from the signer
  path.
  Requirement: the q-syndrome derivation must not expose a rare rejection whose
  occurrence depends on entropy-derived candidate values.
  Result:
  `path2_native_signer_entropy_q_v1` now always executes
  `PATH2_NATIVE_SIGNER_ENTROPY_Q_FIXED_ATTEMPTS = 4` candidate derivations and
  returns the first under-threshold reduced value, or a deterministic reduced
  fallback if no candidate is under threshold. The function still returns a
  `Result` for API compatibility, but the fixed-attempt sampler itself no
  longer emits `sampler exhausted` as a signer rejection surface. KAT
  `path2_native_entropy_q_sampler_has_no_secret_dependent_failure_surface`
  checks repeated roots return canonical field elements and that the CT stage
  report has no sampler-exhaustion public cleanup policy. Remaining CT work:
  compiler/backend audit of comparison, masking, SHA-256, and memory access
  behavior.

- [x] Remove runtime modulo/division from the signer entropy q sampler.
  Requirement: the fixed-attempt signer sampler must not rely on `% Q` or
  division-like runtime reduction in the entropy-to-field path.
  Result:
  `path2_native_signer_entropy_q_v1` now draws a 14-bit candidate, accepts it
  directly when `< Q`, and otherwise uses mask/subtract reduction only for the
  deterministic fallback path. The constant-time audit scope now names
  `sampler_no_runtime_modulo_or_division`, and KAT
  `path2_native_entropy_q_sampler_uses_14_bit_reduction_without_runtime_modulo`
  covers canonical outputs plus the required audit artifact. This closes the
  sampler modulo/division design concern; remaining evidence is compiler/output
  review for the comparisons and masks themselves.

- [x] Add a source-shape CT audit for the signer q sampler.
  Requirement: comparison/mask and fixed-memory-access claims for the signer q
  sampler must be pinned by executable source evidence before any backend
  timing claim is considered.
  Result:
  `Path2NativeSignerCtSourceAuditV1`,
  `path2_native_signer_ct_source_audit_v1`, and
  `path2_native_signer_ct_source_audit_v1_digest` now bind the source window
  for `path2_native_signer_entropy_q_v1`. The audit checks the fixed-attempt
  loop marker, deterministic fallback marker, mask-select marker, absence of
  runtime modulo/division tokens, absence of sampler-exhaustion errors, and
  fixed digest byte loads (`digest[0]`, `digest[1]`) instead of secret-indexed
  digest access. KAT
  `path2_native_signer_ct_source_audit_pins_sampler_source_shape` covers this
  surface and keeps `backend_audit_complete = false`. Remaining CT work is now
  backend/assembly and timing evidence for the audited source shape plus SHA-256
  and cleanup behavior.

- [x] Add repeatable release-assembly audit for the signer q sampler.
  Requirement: the source-shape CT claim must be backed by an optimizer-level
  artifact for the sampler function, not only Rust source inspection.
  Result:
  `path2_native_signer_entropy_q_v1` is now marked `#[inline(never)]` so release
  assembly exposes a stable sampler body. Script
  `scripts/research/path2_native_signer_q_sampler_backend_audit.sh` runs
  `cargo rustc --release --lib -- --emit=asm`, extracts the sampler function,
  and checks exactly four fixed calls to `path2_native_signer_entropy_digest_v1`,
  zero division instructions, zero conditional branch instructions,
  conditional-move lowering, and fixed digest byte loads. Current audit output
  digest:
  `38a641831c385e02d6bedc39cc6b89c09d04bd17ffc6347c978c39cc5d6a877a`.
  This closes the current sampler backend-shape evidence for x86_64 release
  builds. Remaining CT work is whole-signer timing evidence, SHA-256 behavior,
  OS entropy boundary review, and rejection cleanup outside this sampler body.

- [x] Add executable signer rejection-cleanup public-surface audit.
  Requirement: success, constructor rejection, and entropy rejection reports
  must have a single sanitized public error surface before any whole-signer
  cleanup claim is considered.
  Result:
  `path2_native_signer_execution_failure_report_v1` now centralizes sanitized
  signer failure reports. `Path2NativeSignerRejectionCleanupAuditV1`,
  `path2_native_signer_rejection_cleanup_audit_v1`, and
  `path2_native_signer_rejection_cleanup_audit_v1_digest` compare success,
  constructor-failure, and entropy-failure report surfaces. KAT
  `path2_native_signer_rejection_cleanup_audit_pins_public_failure_surface`
  verifies rejection reports carry no public-output/proof digests, use only
  public rejection codes, serialize without private-material sentinels, and
  keep `cleanup_zeroization_claim_complete = false`. This closes the current
  public-error serialization gap; remaining cleanup work is allocator/runtime
  zeroization evidence and whole-signer timing evidence.

- [x] Add signer SHA-256 source-boundary audit.
  Requirement: before treating SHA-256 as a remaining implementation blocker,
  separate source-level secret-dependent input-shape risk from the backend
  constant-time/library question.
  Result:
  `Path2NativeSignerSha256BoundaryAuditV1`,
  `path2_native_signer_sha256_boundary_audit_v1`, and
  `path2_native_signer_sha256_boundary_audit_v1_digest` now pin the signer
  entropy-derivation SHA boundary. KAT
  `path2_native_signer_sha256_boundary_audit_pins_source_boundary` checks the
  domain, label, public binding, owned entropy, and counter frames; confirms the
  public binding is exactly six digest words; confirms derivation labels are
  static public call sites; and confirms the q sampler invokes a fixed digest
  schedule. The audit keeps `sha256_backend_constant_time_claim_complete =
  false`, so the remaining SHA item is narrowed to backend/library timing
  evidence, not a secret-dependent source input-length issue.

- [x] Add valid-private-path SHA-256 call-schedule audit.
  Requirement: after public-input prevalidation, the deployable signer claim
  needs the signer-owned SHA schedule pinned for the valid private path, not
  just the frame shape of individual digest calls.
  Result:
  `Path2NativeSignerSha256CallScheduleAuditV1`,
  `path2_native_signer_sha256_call_schedule_audit_v1`, and
  KAT `path2_native_signer_sha256_call_schedule_audit_pins_valid_private_path`
  pin the active schedule: 4 fixed seed-derivation SHA calls, 12 q-sampler
  invocations, 4 digest attempts per q-sampler invocation, 48 q-sampler SHA
  calls, 52 total entropy-derivation SHA calls, and 4 fixed secret-component
  SHA calls. The audit also checks static labels, fixed public loop bounds, and
  seed zeroization after secret-component digesting. It keeps
  `sha256_backend_constant_time_claim_complete = false`, so the remaining SHA
  issue is now specifically the `sha2` compression/backend timing behavior and
  not signer-owned call-count variability.

- [x] Add SHA-256 backend dependency inventory.
  Requirement: the remaining SHA-256 backend timing assumption must be tied to
  a concrete dependency/backend surface, not a generic "hash function" note.
  Result:
  `scripts/research/path2_native_signer_sha256_backend_dependency_audit.sh`
  pins the parameter-search signer dependency to `sha2 0.10.9`, with lockfile
  dependencies `cfg-if`, `cpufeatures`, and `digest`, and confirms no
  `sha2-asm` dependency is present in the current lockfile. It now also checks
  that the active crate enables the `sha2/force-soft` feature, so runtime
  x86/SHA-NI dispatch is not selected for signer private hashing. Current lock block
  digest:
  `a9e07d75c2662606741f669bfa7fe7cb9683adef27d58df6ce7d8f2ec349b6cc`.
  This does not prove SHA-256 backend constant-time behavior; it narrows the
  remaining review target to the RustCrypto portable `sha2`/`digest` backend
  selected by this build.

- [x] Add SHA-256 selected-backend source audit.
  Requirement: the remaining SHA-256 backend timing assumption must identify
  the actual source-level backend selected by the locked build and avoid
  environment-dependent CPU dispatch for private signer hashing.
  Result:
  `scripts/research/path2_native_signer_sha256_source_backend_audit.sh` uses
  locked `cargo metadata` to locate `sha2 0.10.9` and pins the SHA-256 backend
  source shape: no active `sha2-asm` dependency, active `force-soft` feature,
  runtime x86 dispatch not selected, soft backend selected, and the soft
  backend's public block loop present. Current source digests:
  `sha256.rs`
  `6ced9112d4ceb4eff20b56d93a063f72ecef4c6ec3b215f08696a853b09083c3`;
  `sha256/soft.rs`
  `98e765a8e8dfa0af31f2b76570f212e6b3099522bf300e1554cbbd9fd5d02960`.
  This removes the runtime dispatch/SHA-NI selection surface from the active
  path. The remaining SHA risk is portable backend constant-time behavior and
  internal state lifetime; this is still not a constant-time proof.

- [x] Add SHA-256 selected-backend release assembly audit.
  Requirement: source-level SHA backend selection must be backed by compiled
  release evidence for the actual locked `sha2` object used by the signer.
  Result:
  `scripts/research/path2_native_signer_sha256_release_backend_audit.sh`
  extracts the compiled `libsha2` rlib and audits the object containing
  `sha2::sha256::compress256`. Current force-soft audit: `compress256` symbol
  present, force-soft release backend selected, x86 SHA-NI `digest_blocks`
  absent, `shani_cpuid` storage absent, public CPU-feature dispatch absent,
  zero allocator/memory-routine calls in the audited compression body,
  SHA-NI instruction counts all zero, and public block-count guard present.
  Assembly digest: `compress256`
  `4cc8aa6cfc8f7e9d861e66624f3f71b4f561bd20200deb5754239f046b43348f`.
  This still keeps `constant_time_claim_complete = 0`; the remaining SHA risk
  is portable backend timing semantics and internal compression-state lifetime,
  not runtime dispatch or unknown release backend selection.

- [x] Add portable SHA-256 soft-backend CT-shape audit.
  Requirement: after forcing the portable SHA-256 backend, separately inspect
  the soft compression source and release assembly for data-dependent control
  flow markers rather than relying only on backend-selection evidence.
  Evidence:
  `scripts/research/path2_native_signer_sha256_soft_ct_shape_audit.sh` pins
  `sha2 0.10.9` soft backend source shape and release assembly for
  `sha2::sha256::compress256`. Current audit: public block loop present,
  fixed 4-byte chunk loop present, no source-level `if`/`match`/`while`, two
  conditional branches and one unconditional jump in release assembly
  attributable to the public block-count loop, zero calls, zero cmovs, zero
  setcc, zero variable shifts, zero allocator/memory/SHA-NI/CPU-dispatch
  markers, `sha256/soft.rs` digest
  `98e765a8e8dfa0af31f2b76570f212e6b3099522bf300e1554cbbd9fd5d02960`,
  and `compress256` assembly digest
  `4cc8aa6cfc8f7e9d861e66624f3f71b4f561bd20200deb5754239f046b43348f`.
  This is strong implementation evidence for the portable backend shape, but
  it still leaves the formal portable-backend timing and internal-state
  lifetime claim open.

- [x] Pin the active in-tree private SHA-256 CT shape.
  Requirement: after moving private signer hashing off upstream
  `Sha256::digest(frame.as_slice())`, do not rely only on CT evidence for the
  inactive upstream compression path.
  Evidence:
  `scripts/research/path2_native_signer_private_sha256_ct_shape_audit.sh`
  audits `path2_native_private_sha256_digest_v1` directly. It pins source-level
  fixed block/schedule/round loops, release assembly with nine public loop
  conditional branches, one unconditional jump, three calls total, zero
  public-length memory routine calls, three zeroizer calls, no cmov/setcc,
  no variable shifts, and no allocator/SHA-NI/CPU-dispatch markers. The CT
  evidence bundle now includes this active-path audit, and the Rust ledger pins
  it as `private_sha256_ct_shape_script_pinned = true`. The formal constant-time
  claim remains open because target CPU timing and residual register/spill
  evidence still need deployment evidence.

- [x] Replace upstream private SHA-256 hashing with an in-tree zeroizing path.
  Requirement: do not leave "internal hash-state lifetime" as an abstract
  residual. Identify the concrete state owned by the active backend and whether
  it is explicitly scrubbed.
  Evidence:
  `scripts/research/path2_native_signer_sha256_internal_state_lifetime_audit.sh`
  now audits the active private signer hash path: both private digest wrappers
  call `path2_native_private_sha256_digest_v1`, no longer call upstream
  `Sha256::digest(frame.as_slice())`, and the in-tree digest owns fixed
  workspace/schedule/state buffers with explicit byte/u32 volatile zeroizers.
  The release audit confirms one byte-workspace zeroizer call, two u32-zeroizer
  calls, output written to caller-owned digest storage before internal cleanup,
  fixed stack offsets for workspace/schedule/state cleanup, zero post-cleanup
  `mov` instructions before return, no allocator path, and no SHA-NI/CPU-dispatch
  markers. KAT `path2_native_private_sha256_matches_sha2_and_zeroizes_workspace`
  checks the private implementation against `sha2` over representative lengths.
  The Rust ledger pins this as `sha256_private_hash_lifetime_audit_pinned = true`.
  Remaining blocker: post-cleanup register lifetime and formal backend timing
  are still open; this closes the old upstream private hash-state cleanup gap
  and the private SHA internal stack-lifetime gap, not the whole constant-time
  claim.

- [x] Pin the post-cleanup register residual boundary.
  Requirement: distinguish stack-lifetime closure from register-lifetime
  closure. Do not claim that optimized release registers are scrubbed unless
  the assembly actually enforces it.
  Evidence:
  `scripts/research/path2_native_signer_register_residual_audit.sh` extracts
  the private SHA callee plus both digest wrappers. It pins that private SHA
  writes caller-owned output before cleanup, has zero post-cleanup `mov`
  instructions on the success epilogue, and that both wrappers copy digest
  output before cleanup, scrub their local digest copies, and have zero
  post-cleanup `mov` instructions on their success epilogues. It also records
  the observed XMM digest-copy registers and explicitly emits
  `general_register_scrub_complete=0`, `simd_register_scrub_complete=0`, and
  `register_residual_lifetime_claim_complete=0`. This turns the remaining
  register issue into a named target/toolchain policy question rather than a
  vague internal-lifetime blocker.

- [x] Add target/toolchain register-scrub policy gate.
  Requirement: once the static residual audit has isolated register state as
  the remaining lifetime issue, the project needs an explicit closure policy
  rather than an implicit hope that Rust source zeroization covers registers.
  Evidence:
  `scripts/research/path2_native_signer_register_scrub_policy.sh` consumes the
  register-residual audit and emits
  `Descender.Path2.NativeSignerRegisterScrubPolicy.v1`. It pins that internal
  stack lifetime and digest-wrapper stack lifetime are closed, while
  `general_register_scrub_complete=0`, `simd_register_scrub_complete=0`, and
  `register_scrub_policy_complete=0` remain open. Accepted closure options are:
  an audited target assembly epilogue that scrubs GPR/SIMD before return; an
  audited compiler/toolchain register-zeroing flag with release assembly
  evidence; or a deployment process-isolation policy with no cross-secret reuse
  claim. It explicitly rejects source-level zeroization alone as a register
  scrub closure.

- [x] Add signer OS entropy boundary audit and sanitize raw entropy errors.
  Requirement: OS entropy acquisition must not leave partially initialized
  entropy roots on failure or export backend-specific error detail through the
  active signer boundary.
  Result:
  `path2_native_signer_entropy_root_from_os_v1` now returns static failure
  strings instead of formatting raw `getrandom` backend errors. It still
  zero-initializes both owned entropy buffers and zeroizes the whole root on
  either OS entropy failure. `Path2NativeSignerOsEntropyBoundaryAuditV1`,
  `path2_native_signer_os_entropy_boundary_audit_v1`, and
  `path2_native_signer_os_entropy_boundary_audit_v1_digest` pin the source
  boundary: exactly two OS entropy calls, root-before-nonce order, zeroization
  after either failure, no partial-entropy success, no backend error detail
  export, and sanitized public entropy-failure reporting. KAT
  `path2_native_os_entropy_boundary_audit_pins_sanitized_source_boundary`
  covers the audit. The remaining OS item is only backend/operating-system RNG
  health and platform support review, not signer boundary leakage.

- [x] Add signer runtime-cleanup source audit and explicit early-error scrub.
  Requirement: cleanup evidence must cover the owned signer root, owned
  constructor material, and private digest frames on both success and
  constructor-error paths before any deployable cleanup claim is considered.
  Result:
  `path2_native_private_fiber_from_secret_material_v1` now explicitly
  zeroizes owned constructor material before returning from proof-commitment
  packing failures or proof-commitment mismatch failures, instead of relying
  only on `Drop`. `Path2NativeSignerRuntimeCleanupAuditV1`,
  `path2_native_signer_runtime_cleanup_audit_v1`, and
  `path2_native_signer_runtime_cleanup_audit_v1_digest` pin the source cleanup
  shape: owned root drop, owned material drop, root success zeroization, OS
  failure zeroization, per-seed zeroization after digesting, fixed stack-backed
  signer entropy digest frames, fixed stack-backed constructor digest frames,
  fixed digest-frame zeroization, private-fiber success zeroization,
  private-fiber error zeroization, and digest-only public failure reports. KAT
  `path2_native_signer_digest_frame_zeroize_clears_fixed_buffer` covers the
  fixed signer digest frame scrub helper, and KAT
  `path2_native_runtime_cleanup_audit_pins_owned_source_cleanup` covers the
  audit. The previous allocator-residue concern for private SHA input frames is
  removed by construction: those frames no longer allocate.

- [x] Harden signer-owned zeroization against dead-store elimination and add a
  release-assembly audit.
  Requirement: source-level scrub calls must not rely on ordinary stores that
  the optimizer may remove when buffers are dead after cleanup.
  Result:
  signer-owned byte and u16 secret buffers now use
  `path2_native_zeroize_bytes_v1` and `path2_native_zeroize_u16s_v1`, and
  signer-owned fixed digest frames use
  `path2_native_zeroize_signer_digest_frame_v1`; each performs volatile zero
  writes followed by `compiler_fence(Ordering::SeqCst)`. The runtime-cleanup
  audit now checks for volatile byte zeroization, volatile u16 zeroization,
  volatile fixed-frame zeroization, and compiler fences. Script
  `scripts/research/path2_native_signer_zeroize_backend_audit.sh` runs release
  assembly extraction and checks that `path2_native_zeroize_bytes_v1` contains
  32 explicit byte zero stores plus a compiler fence marker, that
  `path2_native_zeroize_u16s_v1` contains 12 explicit u16 zero stores plus a
  compiler fence marker, that `path2_native_zeroize_signer_digest_frame_v1`
  exposes eight unrolled byte-zero stores under a fixed 448-byte loop bound
  plus a compiler fence marker, and that none of these helpers lowers to
  external `memset`/`bzero`-style routines. Current
  audit digests:
  byte helper
  `c35f1a3615fb1d54fe74a3d98ab96d87a7e2b53d61729f2e5887abe6c4685d68`;
  u16 helper
  `226007c0ff85096a393f59747f5ede75e34e6adcbca1d03c9fd72340798791f4`;
  digest-frame helper
  `5172ac7a8de0ac68ca0695dd7d9b0f4282f73011948345f52a8f1850da0a5652`.
  Remaining cleanup evidence is whole-execution stack/register lifetime review,
  not heap allocator residue for private digest frames and not compiler
  dead-store resistance for the signer-owned scrub helpers.

- [x] Add release-assembly audit for signer private digest-frame constructors.
  Requirement: after moving private SHA input frames off heap, release evidence
  must show the entropy and secret-component digest constructors actually remain
  stack-backed, avoid allocator paths, and call the fixed-frame scrub helper
  after hashing.
  Result:
  `scripts/research/path2_native_signer_digest_frame_backend_audit.sh` extracts
  release assembly for `path2_native_signer_entropy_digest_v1` and
  `path2_native_digest_secret_component_v1`. Current audit: both constructors
  reserve fixed `504`-byte stack frames, expose zero allocator calls, call
  `path2_native_zeroize_signer_digest_frame_v1` exactly once after the private
  SHA call, and do not call upstream `Digest::digest`. The entropy wrapper also
  calls the byte zeroizer once for its local digest copy; the secret-component
  wrapper calls the byte zeroizer twice, covering both the consumed seed and its
  local digest copy. Remaining cleanup evidence is target-specific register
  lifetime and broader whole-signer timing, not heap allocation or missing scrub
  calls in these private digest-frame constructors.

- [!] Add release whole-signer timing smoke probe; it exposes a rejection
  short-path blocker.
  Requirement: before the signer can claim a fixed public execution schedule,
  success and sanitized rejection paths need repeatable release timing evidence.
  Result:
  `Path2NativeSignerTimingProbeV1` and
  `scripts/research/path2_native_signer_timing_probe.sh` now run the active
  native signer execution report path in release mode for success and
  constructor-rejection cases, emit sanitized digest-only report evidence, and
  explicitly keep `timing_claim_complete = false`. Current 64-iteration probe:
  success average `158031 ns`, constructor-rejection average `31108 ns`,
  max/min ratio `5.080075x`. This is a real fixed-stage implementation gap:
  the current constructor rejection path short-circuits before doing work
  comparable to the successful constructor path. Because the rejection is driven
  by public invalid account/projection input, this result is not by itself a
  hidden-witness leak, but it blocks any deployable whole-signer constant-time
  claim. Next required fix: introduce a constant-work rejection wrapper or
  prevalidated-public-input boundary so production signing never mixes the
  full private constructor path with early public-shape rejection timing.

- [x] Add prevalidated public-input boundary for the signer private path.
  Requirement: malformed public projection/account input must be rejected
  before entropy acquisition and before private signing work, so the private
  signer path is not responsible for equalizing invalid public-input failures
  against successful signing.
  Evidence:
  `Path2NativeSignerPrevalidatedPublicInputV1` now carries the public projection
  bundle plus canonical context/account/spend/lifecycle/state/transcript,
  native-boundary, and projection-preimage DA digests. Function
  `path2_native_signer_prevalidated_public_input_v1` performs public-only
  digesting and pullback/cross-field checks and maps all failures to
  `reject_active_native_signer_public_prevalidation_failed`. Function
  `path2_native_signer_execution_report_from_prevalidated_entropy_root_v1`
  accepts only the prevalidated wrapper for the private execution report path.
  Tests
  `path2_native_signer_public_prevalidation_accepts_valid_projection`,
  `path2_native_signer_public_prevalidation_rejects_invalid_projection_before_entropy`,
  and
  `path2_native_prevalidated_signer_execution_report_keeps_public_reject_outside_private_path`
  pin the boundary. `path2_native_signer_execution_report_from_public_input_os_entropy_v1`
  is now the production-style report entrypoint: it prevalidates first, then
  requests OS entropy only for a valid public bundle, and returns the sanitized
  prevalidation rejection code otherwise. Remaining work: rerun release timing
  evidence on only valid/prevalidated private signing cases.

- [x] Add signer production-routing source audit.
  Requirement: the deployable signer claim needs an auditable production-style
  entrypoint, not just lower-level helper functions. The legacy direct
  entropy-root/OS-entropy report path may remain for deterministic tests and
  negative controls, but it must not be the intended production route.
  Evidence:
  `Path2NativeSignerProductionRoutingAuditV1` checks that
  `path2_native_signer_execution_report_from_public_input_os_entropy_v1`
  calls `path2_native_signer_prevalidated_public_input_v1` first, does not call
  OS entropy directly, then routes only prevalidated input to the OS/private
  signer entrypoint. KAT
  `path2_native_signer_production_routing_audit_pins_prevalidation_entrypoint`
  pins this source shape and records digest evidence. `audit_complete` remains
  false because source routing is not whole-signer timing or backend evidence.

- [x] Add release-assembly audit for signer production routing.
  Requirement: the source-level production-routing claim must survive release
  lowering. The public-input signer entrypoint must call public prevalidation
  before the prevalidated OS/private path, and must not call OS entropy directly.
  Evidence:
  `scripts/research/path2_native_signer_production_routing_backend_audit.sh`
  extracts release assembly and checks
  `path2_native_signer_execution_report_from_public_input_os_entropy_v1`.
  Current audit: public prevalidation call line `13`, prevalidated OS call line
  `18`, public direct entropy calls `0`, prevalidated OS entropy calls `1`.
  Assembly digests: public entrypoint
  `03e640c32dd63eca4ee50ebe7c84888924d20e48b4ac95b09c5fba93afe0999d`;
  prevalidated OS entrypoint
  `f1acc81616b791239b91f12a6027b3ed318743512b75ca3a833b682dda921e64`.

- [~] Add valid/prevalidated private-signer timing probe.
  Requirement: after removing malformed public input from the private signer
  schedule, timing evidence must compare valid prevalidated signing runs rather
  than success against invalid-public-input rejection.
  Evidence:
  `Path2NativeSignerPrevalidatedTimingProbeV1` and
  `scripts/research/path2_native_signer_prevalidated_timing_probe.sh` run two
  valid prevalidated signer report paths in release mode, emit sanitized
  digest-only report evidence, and explicitly keep
  `timing_claim_complete = false`. Current 32-iteration probe: first valid
  average `124875 ns`, second valid average `125733 ns`, max/min ratio
  `1.006871x` within the latest stability run's first trial. This is smoke evidence only; it narrows the next timing question
  to variance among valid private signing executions and
  backend/microarchitectural review.

- [~] Add multi-trial valid/prevalidated timing stability probe.
  Requirement: a single wall-clock timing run is too weak to support the signer
  claim, so repeatable stability evidence should aggregate several valid
  prevalidated release probes without claiming a complete constant-time proof.
  Evidence:
  `scripts/research/path2_native_signer_prevalidated_timing_stability_probe.sh`
  runs the valid/prevalidated timing probe across multiple trials, checks that
  all reports remain sanitized and that `timing_claim_complete = false`, then
  emits min/max/mean ratio evidence. Current smoke run with 3 trials and 32
  iterations per trial: max ratio `1.012484x`, mean ratio `1.005355x`, first
  valid average range `124875..126124 ns`, second valid average range
  `125733..126434 ns`, payload digest
  `64613975a4828d5075c02e5bcc73f30f3692662ce3584734a51fe2bbd459adaa`.

- [x] Add first-class constant-time evidence ledger.
  Requirement: the Rust-side deployability gate must distinguish completed
  source/backend pinning from the still-open constant-time claim, instead of
  carrying one opaque "CT incomplete" flag.
  Evidence:
  `Path2NativeSignerCtEvidenceLedgerV1` now aggregates the current source
  audits and pins the backend audit scripts for the q sampler, zeroize helpers,
  fixed private digest frames, SHA-256 dependency/source/release backend,
  active private SHA-256 CT shape/lifetime, register-residual boundary,
  production routing, prevalidated timing stability probe, and preserved-output
  environment manifest. KAT
  `path2_native_signer_ct_evidence_ledger_separates_pinned_evidence_from_open_claims`
  verifies that the ledger marks source evidence and backend script pinning
  true while keeping `constant_time_audit_complete = false`. The explicitly
  remaining blockers are private SHA-256 backend constant-time and
  post-cleanup register lifetime assumptions, whole-signer microarchitectural
  timing beyond smoke probes, target/toolchain register-scrub policy, and
  target-host promotion of the archived release audit outputs.

- [x] Add hash-addressed CT evidence bundle runner.
  Requirement: the current release/source audit scripts must be runnable as one
  reproducible evidence bundle, with individual output digests and an aggregate
  payload digest, while still refusing to claim constant-time completion.
  Evidence:
  `scripts/research/path2_native_signer_ct_evidence_bundle.sh` executes the
  current signer CT/backend probes and emits
  `Descender.Path2.NativeSignerCtEvidenceBundle.v1`,
  per-audit output SHA-256 values, `constant_time_claim_complete=0`, and a
  `bundle_payload_sha256`. The Rust CT evidence ledger pins this bundle script
  through `ct_evidence_bundle_script_pinned`.

- [x] Add preserved-output mode for CT evidence bundles.
  Requirement: a target-host evidence run must preserve the child audit outputs
  that the manifest hashes; stdout-only hashes are not enough for later
  inspection.
  Evidence:
  `scripts/research/path2_native_signer_ct_evidence_bundle.sh` now honors
  `PATH2_NATIVE_SIGNER_CT_EVIDENCE_OUT_DIR`. When set, it writes
  `ct_evidence_bundle.manifest`, `ct_evidence_bundle.payload`,
  `ct_evidence_bundle.environment`, and every `*.out` child audit transcript
  into the requested directory, and emits `audit_output_path`,
  `environment_manifest_path`, `environment_manifest_sha256`, and
  `manifest_path`. The environment manifest records kernel/arch, Rust/Cargo
  versions, rustc host/commit, CPU model, CPU flags digest, audit count, and
  timing harness parameters. The Rust ledger pins this behavior with
  `ct_evidence_bundle_preserves_outputs_when_requested = true`, while keeping
  `target_host_evidence_bundle_execution_complete = false` until a real
  target-host bundle run is promoted as a release artifact.

- [x] Add target-host CT/timing campaign wrapper.
  Requirement: the preserved bundle is useful only if a target-host run can be
  archived as one campaign artifact with the environment, timing stability
  output, register-scrub policy output, and bundle hashes tied together.
  Evidence:
  `scripts/research/path2_native_signer_target_host_timing_campaign.sh` runs
  the preserved-output CT evidence bundle under
  `PATH2_NATIVE_SIGNER_TARGET_HOST_CAMPAIGN_DIR`, then runs the register-scrub
  policy gate and writes
  `target_host_timing_campaign.manifest`. It pins the bundle manifest/payload,
  environment manifest, timing-stability output, and register-policy output by
  SHA-256, and emits `target_host_bundle_archived=1` while keeping
  `register_scrub_policy_complete=0`,
  `microarchitectural_timing_claim_complete=0`, and
  `constant_time_claim_complete=0`. Smoke run:
  `/tmp/path2_target_campaign_smoke`, one timing iteration/trial, passed.

- [x] Add target-host campaign promotion gate.
  Requirement: archived target-host evidence must not be promoted accidentally
  from smoke output. Promotion needs a separate gate that verifies artifact
  hashes, enforces timing iteration/trial floors, and still refuses to close the
  deployable CT claim while register scrub and microarchitectural timing remain
  open.
  Evidence:
  `scripts/research/path2_native_signer_target_host_campaign_promotion_gate.sh`
  consumes an archived campaign named by
  `PATH2_NATIVE_SIGNER_TARGET_HOST_CAMPAIGN_DIR`, defaults the promotion floor
  to 256 timing iterations and 5 stability trials, verifies the campaign
  manifest hashes against the preserved bundle/environment/timing/register
  outputs, and emits
  `Descender.Path2.NativeSignerTargetHostCampaignPromotionGate.v1`. Even when
  thresholds and hashes pass, it emits `register_scrub_policy_complete=0`,
  `microarchitectural_timing_claim_complete=0`,
  `release_promotion_claim_complete=0`, and
  `constant_time_claim_complete=0`. The Rust CT evidence ledger pins this gate
  through `target_host_campaign_promotion_gate_script_pinned`.

- [~] Ordered frame encoder/hash-binding bridge.
  Requirement: separate byte encoder canonicality from SHA-256/QROM digest
  binding for every active native frame/projection/public-view digest. Raw byte
  equality may use parser injectivity; digest equality must export a named
  `hash_binding_or_qrom_break`.
  Evidence:
  `research/path2_ordered_encoder_hash_binding_bridge_v1.md` separates
  fixed-width parser-injective encoders, length-prefixed DA frames, JSON public
  serializers, and digest-bound surfaces. The concrete DA frame gap is now
  covered by KAT `path2_native_projection_preimage_da_frames_roundtrip`; the
  remaining work is to keep digest-equality steps charged to named
  hash/QROM/data-availability assumptions.
