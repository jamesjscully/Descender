# Path 2 Native Boundary `b` Construction Tracker

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This remains a proof-design detail tracker for the native-boundary research
> line. It is not the top-level project tracker; top-level status lives in
> `../TRACKER.md`.

Detail status: supporting proof-design note, subordinate to `../TRACKER.md`.

Purpose: find or construct the public boundary object `b` so account creation
is a native descent/gluing admission problem instead of a collection of
independent public commitments that later require an expensive consistency
proof.

This tracker is proof-first. It does not select implementation parameters,
prove deployability, or replace the account-creation theorem until the boundary
map, quotient, and extractor handoff are stated precisely.

Status legend: `[ ]` open, `[~]` in progress, `[x]` complete, `[D]` dead path.

## Core Question

Choose a public boundary object `b` such that:

```text
b = B_c(h)
```

for one canonical map from one hidden representative `h` in account context
`c`, and every account-creation / spend-facing public datum is a projection of
that same boundary object.

The intended theorem shape is:

```text
AcceptAccountCreate(c, b, proof)
  =>
exists h.
  LocalValid_c(h)
  DescentConsistent_c(h)
  B_c(h) = b
```

or the adversary exports a named break.

## Working Definitions

- `c`: account/lifecycle/verifier context.
- `H_c`: hidden representative space for context `c`.
- `h`: one hidden descent representative / local-section witness.
- `~safe`: safety equivalence relation on `H_c`.
- `B_c : H_c -> Boundary_c`: canonical public boundary map.
- `b`: admitted public boundary object.
- `p_i : Boundary_c -> Field_i`: public projections consumed by leaf,
  evidence, proof payload, lifecycle state, and spend authorization.

The target consistency criterion is:

```text
field_i = p_i(B_c(h))
```

for the same `h` for all public fields.

## Native Consistency Criterion

Native consistency means we avoid this shape:

```text
b = (B_1(h_1), B_2(h_2), ..., B_k(h_k))
```

followed by a large proof that all `h_i` are compatible.

Instead, the boundary must have this shape:

```text
b = B_c(h)
```

with component fields obtained only by projection:

```text
account_commitment = p_account(b)
slot_binding       = p_slot(b)
projection_digest  = p_projection(b)
spend_anchor       = p_spend(b)
```

If two representatives produce the same boundary, they must be equivalent for
all safety-relevant behavior:

```text
B_c(h) = B_c(h')  =>  h ~safe h'
```

or else the proof must charge a named binding/descent soundness break.

## Proof Milestone Order

1. Define `H_c`.
2. Define `~safe`.
3. List the mandatory public projections out of `b`.
4. Test whether the current Path 2 / 308b objects already factor through one
   candidate `B_c`.
5. Mark non-factoring paths dead or explicitly redesign them.
6. Construct the minimal quotient boundary if the current objects do not
   factor cleanly.
7. State account-admission soundness against `b`.
8. State spend handoff soundness against the admitted fiber of `b`.
9. Reconcile the result with the two-view transparent payload theorem.
10. Update the byte/state model only after the proof shape stabilizes.

## A. Hidden Representative Space

- [x] Define `H_c`.
  Requirement: `H_c` must include exactly the hidden material whose consistency
  account creation and spend authorization depend on: local descent section
  coordinates, packed syndrome/residue coordinates, hidden source projection
  coordinates, pad/mask source material, lifecycle slot material, and the
  opening witness consumed by the spend relation.
  Current working form:

  ```text
  H_c =
    local section data
    + packed residue/source coordinates
    + pad/mask source coordinates
    + lifecycle slot coordinates
    + spend-opening coordinates
  ```

  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` defines `H_c` as
  `Section_c x PackedResidues_c x PadSource_c x OpeningTuple_c x
  ProjectionRelation_c x LifecycleSlot_c x TraceWitness_c x Gauge_c`, with
  role labels and structural/gauge/safety classification. The next open item is
  to promote this ledger into an explicit `Path2NativeBoundaryV1` theorem
  interface.

- [x] Define local validity for `H_c`.
  Requirement: state the local row/section checks independent of any concrete
  proof payload. This should separate `LocalValid_c(h)` from the public boundary
  map so the admission theorem does not smuggle validity into `b`.
  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` states the
  first-pass `LocalValid_c(h)` checklist: canonical q12289 decoding, packed
  residue reconstruction, pad-source derivation, active-lane carrier equations,
  checked-core residue compatibility, range/canonicality predicates, and no
  postchallenge dependence.

- [x] Define descent/gluing consistency for `H_c`.
  Requirement: state the equations that make the local components one descent
  object rather than unrelated role witnesses.
  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` states
  `DescentConsistent_c(h)` as all role components being restrictions of one
  `Section_c`, including packed/opening agreement, pad/opening agreement,
  pad/projection derivation, packed/projection unmasking, trace/role handle
  agreement, and lifecycle/context agreement.

## B. Safety Equivalence

- [x] Define `h ~safe h'`.
  Requirement: equivalent representatives may differ in irrelevant hidden
  coordinates, masks, gauges, or encodings, but must induce the same
  safety-relevant public behavior.
  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` defines `~safe`
  over one context `c`, lists required invariants, and separates allowed
  blinding/gauge differences from forbidden spend/replay/authorization
  differences.

- [x] Pin safety-relevant behavior.
  Requirement: if `h ~safe h'`, then both representatives must induce the same:
  account boundary, spend authorization boundary, replay/nullifier/lifecycle
  behavior, checked-core transcript semantics, and future slot consumption
  semantics.
  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` pins account
  namespace, slot identity, admission namespace, opening boundary, pad boundary,
  projection quotient, checked-core authorization, lifecycle behavior, and
  transcript behavior as `~safe` invariants.

- [x] Define the acceptable kernel of `B_c`.
  Requirement:

  ```text
  ker(B_c) subset ~safe
  ```

  The boundary may hide representative choices, but it must not identify two
  representatives that lead to different spend or replay behavior.
  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` states
  `B_c(h)=B_c(h') => h ~safe h'` or `boundary_binding_break` /
  `descent_soundness_break`.

## C. Mandatory Public Projections

- [x] Enumerate all current public fields that must factor through `b`.
  Requirement: include the leaf, evidence, certificate/payload boundary,
  lifecycle slot state, transparent public view, creation-proof binding, and
  checked-core spend handle.
  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` audits natural
  projections, semantic-decision projections, and byte-binding-only fields
  across the leaf, evidence, certificate, proof binding, transparent public
  view, and payload.

- [~] Define projection functions.
  Requirement: for each public field `field_i`, define a projection
  `p_i : Boundary_c -> Field_i` or mark the field as a redesign blocker.
  Progress:
  `research/path2_native_boundary_hc_factorization_ledger.md` names first-pass
  projections for context, lifecycle, public projection, section root, role
  profile, and role restrictions.
  `research/path2_native_boundary_v1_interface.md` drafts the exact
  `Path2NativeBoundaryV1` projection interface, including `p_context`,
  `p_section_root`, `p_role_profile`, `p_role_handles`,
  `p_public_projection`, `p_lifecycle`, `p_statement`,
  `p_slot_leaf`, and `p_transparent_public_view`. Remaining work is auditing
  older 308b/checked-core fields against the same interface and deciding
  whether to preserve compatibility names in the leaf.
  Lean anchor:
  `Descender/Path2NativeBoundary.lean` defines the corresponding proof-side
  structures and projection predicates; `lake build
  Descender.Path2NativeBoundary` passes.

- [x] Identify fields that are only byte binding.
  Requirement: outer SHA-256/digest handles may remain consensus byte binding,
  but their semantic meaning must factor through `b` or through authenticated
  data availability for `b`.
  Evidence:
  `research/path2_native_boundary_hc_factorization_ledger.md` classifies
  `slot_leaf_digest`, `slot_authorization_digest`, payload/proof byte digests,
  query-opening digests, Merkle path digests, and consistency-message digests as
  byte/data-availability binding rather than semantic roots.

## D. Factorization Audit

- [~] Audit the current Path 2 boundary fields.
  Requirement: test whether the existing Path 2 leaf/evidence/payload fields
  can be expressed as projections of one candidate `B_c(h)`.
  Progress:
  `research/path2_native_boundary_hc_factorization_ledger.md` concludes the
  design is close to factorable but not native yet: context and transparent
  public view fields fit, while opening/pad/statement/proof-binding/role
  commitment handles need to be reinterpreted as projections of one native
  boundary.

- [x] Audit the older 308b / checked-core public fields.
  Requirement: identify whether the 308b public material is a projection of the
  same boundary, a downstream projection after admission, or a dead path.
  Evidence:
  `research/path2_native_boundary_308b_checked_core_audit.md` classifies the
  252-byte checked-core signature as a downstream spend projection, the 56-byte
  carrier as superseded negative-control/fail-closed evidence, state-assisted
  opening/pad fields as compatibility projections from `b`, and the old
  64-variable generalized projection relation as dead.

- [x] Mark independent-commitment paths dead when they do not factor.
  Requirement: any path shaped like independent role commitments plus a later
  equality proof must be explicitly marked dead unless it has a clear compact
  consistency theorem.
  Evidence:
  `research/path2_native_boundary_308b_checked_core_audit.md` marks the old
  64-variable generalized projection account boundary, spend-carrier-as-native
  proof, independent role commitments plus later equality proof, and SHA digest
  role handles inside the compact q12289 relation as dead for the
  native-boundary path.

## E. Candidate Boundary Construction

- [ ] Construct the minimal quotient candidate.
  Requirement:

  ```text
  Boundary_c := H_c / ~safe
  b := [h]_safe
  ```

  Then define concrete encodings/projections only after this quotient is
  mathematically clear.
  Progress:
  `research/path2_native_boundary_hc_factorization_ledger.md` states the
  minimal quotient `Boundary_c := H_c / ~safe`; the practical V1 interface in
  `research/path2_native_boundary_v1_interface.md` is now the candidate encoded
  representative.

- [~] Construct the practical encoded boundary candidate.
  Requirement: choose a concrete representation of the quotient suitable for
  blockchain admission. Current expected shape:

  ```text
  b = (
    section_root_or_commitment,
    boundary_projection,
    lifecycle_slot_binding,
    spend_relation_handle
  )
  ```

  These entries are not independent witnesses; they must be canonical
  projections of one `B_c(h)`.
  Progress:
  `research/path2_native_boundary_v1_interface.md` proposes
  `Path2NativeBoundaryV1 = (boundary_version, context_projection,
  parent_section_root, role_profile_digest, role_restriction_handles,
  public_projection_quotient, lifecycle_authorization_projection,
  creation_statement_projection, aggregation_context_digest)` and defines
  `native_boundary_digest`.
  Lean anchor:
  `Descender.Path2NativeBoundary.Boundary` and
  `Descender.Path2NativeBoundary.TransparentViewProjectsFrom`.

- [x] Decide whether role subcommitments are projections or eliminated.
  Requirement: if role subcommitments remain, they must be authenticated
  restrictions of the same descent-section boundary. If that cannot be stated
  compactly, eliminate them from `b` and move their meaning into the quotient.
  Evidence:
  `research/path2_native_boundary_v1_interface.md` selects role subcommitments
  as typed restrictions of `parent_section_root` under `role_profile_digest`.
  If a proof system cannot supply that restriction rule, the path falls back to
  explicit algebraic opening rows or is marked dead for compact opening.

## F. Account Admission Theorem

- [x] State native-boundary account admission.
  Target:

  ```text
  AcceptAccountCreate(c, b, proof)
    =>
  exists h.
    LocalValid_c(h)
    DescentConsistent_c(h)
    B_c(h) = b
  ```

  or one named break.
  Progress:
  `research/path2_native_boundary_v1_interface.md` states the theorem
  interface, and `Descender/Path2NativeBoundary.lean` provides
  `AccountAdmissionSoundness` plus wrapper theorem
  `account_admission_soundness_or_break`. This is a Lean interface only; the
  concrete cryptographic proof remains open.

- [x] Name the break surface.
  Requirement: the theorem must distinguish at least:
  `boundary_binding_break`, `descent_soundness_break`,
  `commitment_opening_break`, `transparent_proof_soundness_break`,
  `no_export_zk_break`, and `qrom_or_domain_separation_break`.
  Evidence:
  `research/path2_native_boundary_v1_interface.md`,
  `research/path2_transparent_extractor_soundness_bridge.md`, and
  `Descender/Path2NativeBoundary.lean` name the native-boundary break surface,
  including boundary/descent, inner commitment, transparent proof, norm/range,
  no-export, QROM/domain, leaf binding, data availability, checked-core, and
  lifecycle/replay breaks.

- [x] State no-export / hiding against `b`.
  Requirement: public admission must reveal only the intended boundary class,
  not the hidden representative. In quotient language:

  ```text
  b reveals [h]_public, not h
  ```
  Evidence:
  `research/path2_native_boundary_no_export_hiding.md` states the allowed
  public material, forbidden exports, boundary-class hiding statement, mask
  freshness rule, and relation to role restrictions. Lean interface
  `Descender.Path2NativeBoundary.NoExportSoundness` and theorem
  `no_export_holds_or_break` now anchor the obligation.

## G. Spend Handoff Theorem

- [x] State spend handoff through the admitted fiber.
  Target:

  ```text
  Admitted(c, b)
  SpendAccept(c, b, sig, msg, state)
    =>
  exists h in Fiber(B_c, b).
    SpendWitnessValid_c(h, sig, msg, state)
  ```

  or a named spend/authorization break.
  Progress:
  `research/path2_native_boundary_v1_interface.md` states the theorem
  interface, and `Descender/Path2NativeBoundary.lean` provides
  `SpendHandoffSoundness` plus wrapper theorem `spend_handoff_or_break`.
  Concrete spend composition remains open.

- [ ] Prove consume-once semantics depend only on `b`.
  Requirement: no representative hidden inside the fiber may change lifecycle,
  replay, or slot-consumption behavior.

- [ ] Prove checked-core transcript binding depends only on `b`.
  Requirement: the spend challenge should bind the projected authorization
  handle from `b`, not an independent digest with separate semantics.

## H. Interaction With Two-View Transparent Payload

- [x] Reinterpret the two-view theorem through `B_c`.
  Requirement: the public masked view should be a projection of `b`, while the
  hidden extractor view should recover some `h` in the fiber of `b`.
  Evidence:
  `research/path2_two_view_trace_theorem_interface.md` now replaces
  `same_parent_context` with `same_native_boundary`, makes the public view
  fields projections from `b`, and sets the extractor target to
  `h in Fiber(B_c, b)`. `research/path2_transparent_payload_contract.md` and
  `research/path2_transparent_extractor_soundness_bridge.md` were updated to
  bind `native_boundary_digest`. Lean wrapper theorem
  `transparent_view_extracts_fiber_or_break` mirrors this surface.

- [x] Check whether the column-profile enum is still the next blocker.
  Requirement: if `b` absorbs some role consistency obligations, the
  column-profile theorem may shrink. If not, it remains the next transparent
  proof blocker.
  Progress:
  `research/path2_native_boundary_column_profile_impact.md` concludes the
  column-profile enum is still required, but its burden shrinks: it no longer
  proves cross-object context/role equality and instead defines typed
  restrictions, visibility classes, mask domains, and query-opening/no-export
  policy for one native boundary.

- [ ] Update payload byte model only after this check.
  Requirement: do not carry the current 6-10 KB planning floor as final until
  the boundary quotient shows which role-link equations survive.
  Progress:
  Do not update yet. `research/path2_native_boundary_norm_range_soundness.md`
  identifies the next theorem decision as raw centered q12289 residues versus
  base-128 digit openings versus a hybrid representation. Byte-model updates
  should wait for that choice.

## Dead-Path Criteria

Mark a path `[D]` if any of the following holds:

- it requires proving equality among independently chosen hidden witnesses
  without a compact native-boundary theorem;
- it treats a SHA digest as proof semantics rather than byte binding or data
  availability;
- it leaks hidden representative coordinates through public openings;
- it allows two representatives with the same `b` but different spend,
  lifecycle, or replay behavior;
- it requires the parked lattice-sigma route solely to connect public fields
  that could instead be projections of one boundary object.

## Immediate Next

The active proof-family assumption is now pinned:

```text
NativeBoundaryZkTraceBindingAndHiding
```

`research/path2_native_boundary_blinding_noise_decision.md` rejects public
linear role commitments with current dimensions and selects private role
linear commitments inside the ZK/two-view proof as the active theorem shape.

The proof-family checklist is now instantiated:

1. parent-root binding is interface-pinned;
2. private typed role restrictions are object-pinned;
3. hidden extractor view obligation is pinned;
4. public masked/simulated query openings remain blocking;
5. hybrid base-128 norm/range checks remain blocking;
6. extractor-side blinding/noise bounds remain blocking;
7. QROM/query schedule and symbolic query rows are pinned.
Evidence:
`research/path2_native_boundary_zk_trace_checklist.md` pins the active
pre-implementation gates and identifies the first unresolved gate as
instantiating the already defined atomic proof-family endpoint after
classifying full-trace extraction versus sampled-query-only candidates.
`research/path2_hash_qrom_assumption_ledger.md` now records the
native-boundary query symbols and schedule inputs.
`research/path2_native_boundary_hidden_extractor_view.md` pins the complete
extractor-output obligation and Lean anchor.
`research/path2_native_boundary_proof_family_extractor_classification.md`
marks sampled-query-only paths dead and keeps the masked trace IOP route active
only with full committed-trace extraction.
`research/path2_native_boundary_trace_proof_family_interface.md` splits the
active assumption into knowledge-soundness, no-export, QROM schedule,
norm/range, and blinding/noise components.
`research/path2_native_boundary_knowledge_soundness_decomposition.md` further
splits knowledge soundness into parent-trace extraction atoms, which are
proof-family dependent, and trace assembly, which is controlled by the
native-boundary
column/profile design.
`research/path2_native_boundary_parent_trace_extraction_contract.md` pins the
full-trace extraction contract and now pins the active extraction/relation pair
to `fullTraceExtractable` plus `fullWitnessRelationKnowledge`.
`research/path2_native_boundary_relation_knowledge_soundness_contract.md` adds
the separate relation-knowledge gate, rejecting sampled-row-only,
proximity-without-witness, and non-selected relation modes for the active
profile.
`research/path2_native_boundary_parent_trace_extraction_open_obligations.md`
lists the remaining proof-family obligations, with relation knowledge
soundness as the likely hardest subclaim.
`research/path2_native_boundary_relation_knowledge_route_matrix.md` compares
the remaining proof-family routes against the decomposed extraction/relation
gates.
`research/path2_native_boundary_trace_relation_v1_inventory.md` and
`research/path2_native_boundary_trace_relation_row_symbols.md` pin the active
relation inventory and symbolic row model.
`research/path2_native_boundary_open_row_estimates.md` gives the first
conservative algebraic row estimate: 160-220 rows, working estimate 176, with
statement/transcript digest checks kept outside the hidden relation.
`research/path2_native_boundary_opening_tuple_account_binding_estimate.md`,
`research/path2_native_boundary_checked_core_compat_estimate.md`, and
`research/path2_native_boundary_role_restriction_row_estimate.md` stress-test
the highest-risk row groups.
`research/path2_native_boundary_context_lifecycle_row_estimate.md` and
`research/path2_native_boundary_blinding_noise_row_estimate.md` pin the
remaining row-estimate conditions.
`research/path2_native_boundary_row_estimate_scenarios.md` records the active
176-row scenario and fallback/reject thresholds.
`research/path2_transparent_proof_byte_model.md` now includes the
native-boundary relation overlay for 160/176/220 rows; the working estimate is
still below the parked lattice-sigma floor, while the stress 1024-row overlay is
close.
`research/path2_native_boundary_proof_family_overhead_budget.md` pins the
remaining byte headroom, and
`research/path2_native_boundary_transparent_proof_family_v1.md` packages the
active transparent proof-family assumption with explicit row/overhead gates.
`research/path2_native_boundary_core_problem_reframe.md` reframes the core
problem as a zero-knowledge image-membership proof for the native descent
boundary map, and opens creative route comparison beyond the masked trace route.
`research/path2_native_descent_non_zk_alternatives.md` records the stronger
native fork: avoid treating `h` as a generic private witness when possible by
constructing a public quotient/image/obstruction boundary whose accepted fibers
are large enough to hide representatives but safety-saturated enough to fix
spend, replay, and lifecycle behavior.
`research/path2_holistic_descent_boundary_construction.md` sharpens this into a
first-principles construction program: derive the private representative,
public boundary, obstruction map, fiber privacy, and spend semantics together
instead of starting from a fixed hidden witness and searching for a proof.
`research/path2_high_level_descent_design_space.md` lifts the search one level
higher by defining the abstract objects `M_c`, `H_c`, `G_c`, `Q_c`, `O_c`,
maps `boundary_c` and `obs_c`, protocol projections, and the laws each
candidate must satisfy before concrete bytes or proof systems are selected.
`research/path2_geometric_descent_lens.md` adds a disciplined geometric lens:
`Q_c` as a moduli/public quotient object, `H_c` as local trivializations,
private randomness as gauge/torsor fibers, `obs_c` as an obstruction class,
and pullback boundaries as a way to make account/lifecycle/spend/transcript
consistency native.
`research/path2_concrete_solution_search_strategy.md` converts the abstract
frame into the active work plan: define `M_c`, generate exactly three serious
native candidates, falsify them through hard gates, and promote only a survivor
into Lean/theorem/byte models.
`research/path2_native_solution_search_todo_tracker.md` is the detailed todo
tracker for executing that plan.
`research/path2_meaning_v1.md` completes the first semantic target pass by
defining `M_c = AccountNamespace_c × SpendAuthorizationClass_c ×
LifecycleIdentity_c × TranscriptBindingSurface_c × StateTransitionBoundary_c ×
ParameterContext_c`, plus `meaning_c : H_c -> M_c` and first-pass
`~safe` as equality of semantic meaning.
`research/path2_candidate_pullback_boundary.md` opens Candidate A. It defines a
pullback boundary over `Q_context` with account/lifecycle/spend/transcript/state
component quotients. Preliminary result: strong for projection completeness and
native consistency, but effective descent is the main blocker because pullback
validity does not by itself imply an account-backed private lift.
`research/path2_candidate_obstruction_boundary.md` opens Candidate B. It defines
public boundary data `Q_c`, obstruction object `O_c`, and obstruction map
`obs_c`. Preliminary result: highest-upside candidate for replacing generic ZK,
but blocked on constructing a concrete public obstruction equation whose
vanishing proves an account-backed lift, not just abstract gluing.
`research/path2_candidate_canonical_gauge.md` opens Candidate C. It defines a
canonical public object `q`, target `canonical_lift_c : Q_c -> H_c / G_c`,
gauge space, and gauge action laws. Preliminary result: cleanest existence story
if canonical lifts exist, but high privacy risk if canonicalization collapses
the private gauge fiber.
`research/path2_native_candidate_gate_matrix.md` compares the three candidates
and selects a hybrid strategy: Candidate A supplies pullback public quotient
organization, Candidate B supplies obstruction-vanishing native validity and
effective descent, and Candidate C supplies the private gauge/fiber model. The
next concrete blocker is defining an actual `obs_c : Q_c -> O_c`, not just its
named components.
`research/path2_obstruction_map_v1_attempt.md` defines the first concrete
obstruction-map attempt: public boundary faces, obstruction components, defect
maps, public gluing equations, and the effective-descent decomposition. It
narrows the remaining blocker to a lift/gauge theorem from global public
boundary to private representative fiber.
`research/path2_gauge_lift_v1_attempt.md` defines that lift/fiber model:
`H_c(q) = LiftCore_c(q) × AuthorizationLocal_c(q) × Gauge_c(q) ×
Randomizer_c(q)`, with boundary erasing private coordinates and meaning derived
from `q`. It narrows the next blocker to classifying authorization-local spend
material: private is acceptable only if the public authorization class is fixed
by `q`.
`research/path2_authorization_local_classification.md` classifies
authorization-local material as private semantic-local signing material over a
fixed public authorization class, not pure gauge. The key law is
`auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)`. The next blocker is
stating spend acceptance directly over the native boundary `q`.
`research/path2_spend_closure_over_q.md` states that spend accepts
`SpendAccept(c,q,sig,msg,state)` and that checked-core/current spend may remain
downstream only if every semantic input is a projection of `q`. Independent
spend handles are dead unless their meaning publicly factors through `q`.
`research/path2_native_boundary_projection_audit.md` audits current field names
against the native `q` model. It classifies semantic projections, byte bindings,
authorization-local data, gauge/randomizer data, typed restrictions, and dead
independent handles. Highest-risk fields are `opening_commitment`,
`pad_seed_digest`, `slot_authorization_digest`, `parent_section_root`, and role
handles.
`research/path2_active_native_account_design_v1.md` consolidates the selected
hybrid as the active design: `Q_c` semantic faces, `M_c`, `O_c`, `obs_c`,
private fiber `H_c(q)`, public projections, account admission theorem, spend
closure theorem, field disposition, break surface, and open obligations. The
next implementation step is now started:
`Descender/Path2NativeAccountDesign.lean` defines the theorem-interface module
for the active design and builds with `lake build
Descender.Path2NativeAccountDesign`.
`research/path2_projection_surfaces_v1.md` defines the concrete semantic
projection surfaces `p_context`, `p_account`, `p_spend`, `p_lifecycle`,
`p_transcript`, and `p_state`. This is the basis for resetting byte/state
estimates around native `q` rather than the old proof-envelope model.
`research/path2_native_byte_state_model_v1.md` resets the size model around the
active native design. Current planning numbers are 244-392 bytes for account
creation public data, 145-320 bytes for account state, 128-416 bytes for signer
private state, and 0-32 bytes spend signature impact, excluding fallback proof
envelopes.
`research/path2_residual_assumption_ledger_v1.md` separates active native
structural assumptions from ordinary hash/domain assumptions, downstream
checked-core assumptions, authorization-local assumptions, and fallback
proof-system assumptions. It keeps masked trace as fallback and identifies
concrete `O_gluing` instantiation as the next active proof move.
`research/path2_oglue_instantiation_v1.md` instantiates `O_gluing` as a product
of public edge compatibility checks and separates coherent public boundary
construction from private lift nonemptiness.
`research/path2_lift_nonemptiness_v1.md` shows that unconditional
`obs_c(q)=0 => H_c(q) nonempty` is too strong as the default consensus theorem.
The active split is now safety-only public admission plus honest
wallet/keygen spendability, with spendability-certified admission kept as an
optional stronger mode.
`research/path2_honest_create_sampler_v1.md` pins the wallet/keygen sampler:
derive authorization-local material first, set `p_spend(q)` to its public
authorization class, then build all public faces of `q` from one semantic
source.
`research/path2_authorization_class_binding_proof_v1.md` shows that
authorization class binding is achievable if `auth_compatible_c` is typed over
`p_spend(q)`, and identifies spend projection completeness as the next premise
to audit.
`research/path2_spend_projection_completeness_audit_v1.md` audits that premise
and adds the missing `allowed_state_transition_class` and `aggregation_mode`
coordinates to `p_spend(q)`, edge-linked through `E_SD` and `E_ST`.
`research/path2_checked_core_frame_binding_v1.md` specifies the checked-core
native frame theorem and updates the Lean interface so the frame includes the
prechallenge proof commitment in addition to native projections, message, and
state.
`research/path2_fiber_privacy_no_export_v1.md` specifies the active native
fiber privacy/no-export theorem and updates the Lean interface with public-view
simulation hooks.
`research/path2_spend_closure_final_composition_v1.md` composes the active
proof pieces and adds the Lean wrapper `public_admission_spend_action_or_break`,
now strengthened so `ProjectionCompleteness` yields `SpendActionMatchesMeaning`
for the returned action.
`research/path2_concrete_oglue_encoding_v1.md` instantiates `E_SD`, `E_ST`, and
context/pullback as concrete public byte/equality checks. The Lean interface
now has `ConcreteGluingFields`, `ConcreteGluingChecks`,
`ConcreteGluingConsequences`, and `concrete_gluing_consequences`.
`research/path2_pstate_canonicality_v1.md` defines `state_transition_class` as
a canonical state-boundary tuple over `p_state(q)` plus context, and adds the
Lean interface `StateProjectionCanonicality`.
`research/path2_context_state_domain_binding_v1.md` extends that context to
include version, parameter set, verifier family, spend domain, chain/app domain,
fork/epoch, and aggregation mode so state classes cannot replay across
deployment contexts.
`research/path2_ocontext_wellformedness_v1.md` specifies the public validity
checks for version, parameter set, verifier family, domains, fork/epoch, and
aggregation mode; Lean now exposes `ContextWellFormedness`.
`research/path2_checked_core_frame_byte_audit_v1.md` finds that the visible
checked-core transcript helper is fallback response evidence, not the active
native challenge frame. A concrete `Path2NativeCheckedCoreFrameV1` spec is now
required.
`research/path2_native_checked_core_frame_v1_spec.md` defines that native frame
field order, domain tags, forbidden legacy semantic fields, and mutation tests;
Lean now exposes `NativeCheckedCoreFrameBinding`.
`research/path2_public_view_no_export_allowlist_v1.md` defines the active
native public-output allowlist/denylist and Lean policy hooks for no-export.
`research/path2_proof_side_closure_status_v1.md` assesses proof-side closure:
the hidden-witness extraction blocker is no longer active; remaining work is
concrete encoders, checked-core frame implementation binding, no-export KATs,
and hash/domain assumption refresh.
`research/path2_residual_assumption_ledger_v1.md` is refreshed to match that
status.
`research/path2_native_boundary_trace_assembly_map.md` pins the first-pass map
from full extracted trace roles to `H_c` components and identifies role
completeness as the next design-controlled check.

The immediate native-design question is now:

```text
can the Lean theorem interface express both
  AcceptCreate(c,q) => obs_c(q)=0 ∧ public_boundary_sound_c(q)
and
  HonestCreate_c(seed)=Some(q,h) => LocalValid_c(q,h)
without forcing consensus to extract private authorization material?
```

If yes, account creation can stay compact while honest wallet/keygen remains
spendability-sound.

Implementation bridge status:
`Path2NativeCheckedCoreFrameV1` now has a reference encoder/decoder, challenge
helper, facade exports, and KAT coverage for the golden byte layout, decode
round trip, semantic mutation sensitivity, invalid context rejection, and
legacy private-handle exclusion.

The active native public-view serialization bridge now has reference coverage:
`PublicNativeAccountViewV1` and `PublicNativeSpendViewV1` expose only the
allowlisted digest/context surface, and KATs check private/fallback denylist
absence plus digest-only proof-byte handling.

The next implementation-facing proof bridge is projection preimage encoding:
the concrete byte encoders for `p_context_digest`, `p_account_digest`,
`p_spend_digest`, `p_lifecycle_digest`, `p_transcript_digest`, and
`p_state_digest` must be specified and tested as projections of the same native
boundary object rather than independent commitments.

Projection encoder status:
`research/path2_native_projection_preimage_encoders_v1_spec.md` fixes the
domain tags, byte order, digest rule, native boundary digest composition,
no-export rule, and KAT requirements for each `p_*` projection. The reference
code now implements typed context/account/spend/lifecycle/state/transcript
projection encoders, digest helpers, boundary digest composition, facade
exports, and KATs for layout, digest mutation cascade, context rejection, and
private sentinel absence.

Single-source assembly status:
`Path2NativeProjectionBundleV1`, `Path2NativeBoundaryAssemblyV1`, and
`assemble_path2_native_boundary_v1` now derive all projection digests, the
acyclic boundary digest, transcript digest, checked-core frame, and active
native public views from one projection bundle. Implementation also exposed a
byte-model correction: `native_boundary_digest` must not include
`p_transcript_digest`, because the transcript projection binds
`native_boundary_digest`. The resulting order is child projection digests,
then boundary digest, then transcript digest, then checked-core frame/public
views.

Lean mirror status:
`Descender/Path2NativeAccountDesign.lean` and
`Descender/Path2NativeBoundary.lean` now encode the same acyclic assembly order:
child projection digests determine `nativeBoundaryDigest`, and transcript
digest then binds that boundary digest. Both modules build.

Assembly-to-frame proof status:
`Descender/Path2NativeAccountDesign.lean` now defines
`NativeCheckedCoreFrameDigestBinding` and
`native_assembled_checked_core_frame_eq`, connecting the acyclic projection
assembly to `NativeCheckedCoreFrameBinding`. The checked-core frame theorem now
expands through assembled boundary/transcript/account/spend/lifecycle/state
digests rather than allowing independent digest handles.

Public-view binding status:
`Descender/Path2NativeAccountDesign.lean` now defines
`NativePublicViewFields`, `NativePublicViewAssembly`, and
`native_public_views_assembled_eqs`. The account/spend public-view equations
expand through assembled public projection digests and public message/state/
proof digest-length handles, with no private fiber input.

Rust/Lean cross-check status:
`research/path2_native_rust_lean_assembly_crosscheck_v1.md` now maps the Rust
projection, assembly, checked-core frame, and public-view KATs to the Lean
assembly/frame/public-view theorems. It explicitly leaves SHA-256 binding,
canonical byte encoding, serializer denylist enforcement, and domain separation
outside Lean's arithmetic model.

Domain/schema registry status:
`path2_active_native_domains_and_public_schemas_are_unique` checks active
native projection domains, the boundary domain, checked-core frame domain, and
public-native account/spend schemas for namespace/version shape and pairwise
uniqueness. `production_registered_domain_schema_constants` now extends this
into a production-wide manifest covering active native Path 2, legacy Path 2,
reviewed/Fq12, deployment preflight, lifecycle, target-chain, and hidden-source
fallback surfaces; `production_domain_schema_registry_is_globally_disjoint`
checks printable versioned values, global uniqueness, required active-native
entries, and no active-native collision with non-native surfaces.

Residual ledger status:
`research/path2_residual_assumption_ledger_v1.md` now separates implemented
active-native KAT surfaces from remaining deployment assumptions. Local and
production-wide domain/schema uniqueness are KAT-covered; SHA-256 binding,
QROM modeling, and parser/serde behavior remain explicit obligations.

Consensus carrier status:
`research/path2_native_consensus_carrier_v1_spec.md` selects a hybrid admitted
`q` carrier. Validators receive a 330-byte fixed body containing the compact
native/projection digest surface plus public state/lifecycle/obstruction fields
and a data-availability digest for the expanded projection preimages. Rust now
implements `Path2NativeConsensusCarrierV1`, canonical bytes, digest, decoder,
assembly-derived constructor, facade exports, carrier KATs, verifier/preflight
recomputation, and accepted/rejected no-export scans.

QROM/hash status:
`Path2NativeHashQromReviewV1` / `path2_native_hash_qrom_review_v1` now imports
the deployment query caps from `path2_direct_query_budget_v1`, records the
production registry entry count, pins the native consensus-carrier byte counts,
names the active native public query symbols, and separates ordinary public
SHA-256/QROM byte-binding assumptions from hidden-witness obligations.
`path2_native_hash_qrom_review_tracks_public_transcript_assumptions` checks
these fields and scans the serialized review with the active native no-export
denylist. `research/path2_hash_qrom_assumption_ledger.md` records the active
native review.

Implementation preflight status:
`cargo test --manifest-path tools/parameter-search/Cargo.toml --lib` passes all
130 library tests, including active native projection/frame/view/assembly,
consensus carrier, preflight report, global domain/schema registry, and
QROM/hash review KATs. `cargo test --manifest-path
tools/production-reference/Cargo.toml --lib` passes all 33 facade tests after
exporting the active native public surfaces. `lake env lean Descender.lean`
succeeds for the current Lean theorem interface aggregate.

The active native proof-side public boundary is internally preflighted. Remaining
internal work is outside this proof-side closure slice: productionizing the full
signer/verifier implementation, constant-time hardening, and storage/lifecycle
hardening. External review remains excluded from this internal tracker.

Active native productization preflight:
`Path2NativeDeploymentPreflightV1` / `path2_native_deployment_preflight_v1`
now records the completed proof-side public boundary, byte bounds, facade/export
readiness, and exactly three internal fail-closed blockers: active native
signer/verifier pipeline, constant-time audit, and target-chain storage/lifecycle
hardening. It intentionally excludes external review from the internal readiness
calculation. KAT
`path2_native_deployment_preflight_tracks_internal_productization_blockers`
pins the report and digest.

Active native verifier pipeline contract:
`Path2NativeVerifierPipelineVerdictV1` /
`verify_path2_native_deployment_pipeline_v1` is now the fail-closed
production-facing verifier wrapper. It consumes the 330-byte carrier plus
available projection preimage data, calls active native carrier preflight,
distinguishes public-boundary acceptance from production acceptance, emits only
public report/verdict data, and rejects with
`reject_active_native_pipeline_not_productized` until the deployment preflight
blockers are closed. KAT
`path2_native_verifier_pipeline_is_public_only_and_fail_closed_until_productized`
covers accepted public-boundary input, carrier mismatch rejection, digest
generation, and JSON/debug no-export scans.

Active native signer-side pipeline contract:
`Path2NativeSignerPipelinePublicOutputV1` /
`path2_native_signer_pipeline_public_output_v1` derives the public carrier,
carrier digest, core-proof digest/length, projection-preimage DA digest, and
native boundary transcript digests from signer-held inputs. The output remains
fail-closed with `reject_active_native_signer_not_productized`, exposes no
projection bundle or private material, and is verifier-compatible with
`verify_path2_native_deployment_pipeline_v1`. KAT
`path2_native_signer_pipeline_public_output_is_public_and_verifier_compatible`
covers no-export serialization/debug scans and verifier handoff.

Active native constant-time audit scope:
`Path2NativeConstantTimeAuditScopeV1` /
`path2_native_constant_time_audit_scope_v1` now pins the constant-time blocker
to eight secret-dependent active-native operations, four public variable-length
operations, seven required audit artifacts, and explicit non-claims for public
message length, unaudited sampler schedules, and research-only APIs. KAT
`path2_native_constant_time_audit_scope_pins_secret_dependent_surface` checks
the scope and digest.

The next concrete productization move is to define the active native
storage/lifecycle hardening scope: target-chain storage, lifecycle,
restore-anchor, replay, and data-availability artifacts required before
`storage_lifecycle_hardening_complete` can become true.

The holistic construction hypothesis is:

```text
Boundary_c = descended account meaning + public obstruction certificate
H_c        = local lift of Boundary_c + private safety-irrelevant gauge
S_c        = native descent/gluing obstruction equation
Spend_c    = operation on the boundary class with private gauge authorization
```

The next tracker pass should therefore start from `Meaning_c`, not from the
current operational hidden witness layout.

The high-level search procedure is:

1. define `M_c`;
2. choose candidate public quotient `Q_c`;
3. define all consensus projections `p_i : Q_c -> Field_i`;
4. define obstruction/check map `obs_c : Q_c -> O_c`;
5. prove or falsify effective descent;
6. prove or falsify safety saturation;
7. check fiber privacy/gauge size;
8. check spend closure over `q`;
9. only then choose byte encodings, commitments, or proof systems.

The concrete strategy now evaluates three candidates first:

```text
Candidate A: pullback boundary
Candidate B: obstruction-vanishing boundary
Candidate C: canonical lift + private gauge
```

Each candidate must pass:

```text
projection completeness
safety saturation
effective descent
fiber privacy
spend closure
native verification
```

before byte modeling or proof-system selection.

## Non-Claims

- This tracker does not prove account-creation soundness.
- This tracker does not make Path 2 deployable.
- This tracker does not select final transparent proof parameters.
- This tracker does not require aggregation, but it should avoid choices that
  make future aggregation impossible.
- This tracker does not treat external review as an internal closure item.
