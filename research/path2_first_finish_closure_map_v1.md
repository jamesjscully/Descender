# Path 2 First Finish Closure Map V1

> MIGRATION STATUS (2026-07-09): `ACTIVE_SUPPORT`.
> This is a subordinate proof-finish map, not the project tracker.
> Top-level status lives in `../TRACKER.md`; normative blocker ordering lives in
> `../spec/security_model.md`.

Status: active support for first proof-finish triage.

## Purpose

This note exists to stop proof work from expanding sideways. It maps the first
finish line to the small set of fail-closed switches that still block the direct
Path 2 theorem and account-creation proof boundary. It does not change tracker
status, does not claim deployment readiness, and does not replace the residual
matrix in `../spec/security_model.md`.

For this map, "first finish" means an internally coherent direct-mode proof
package with every locally closable rank-1/rank-2 obligation either discharged
or reduced to a named external-review hypothesis. It does not mean the production
gate opens. `DIR-REVIEW-001` remains the external third-party cryptanalysis and
review gate.

## Current Fail-Closed Switches

The direct theorem boundary is still intentionally fail-closed:

| Artifact | Blocking field | Current value | First-finish interpretation |
|---|---:|---:|---|
| `formal/kat/path2_direct_theorem_boundary_v1.json` | `composition_capstone_under_explicit_named_assumptions_imported` | `true` | final local product theorem exists under exact named proof/review assumptions |
| `formal/kat/path2_direct_theorem_boundary_v1.json` | `composition_theorem_instantiated` | `false` | no final admission/spend/lifecycle composition theorem is instantiated |
| `formal/kat/path2_direct_theorem_boundary_v1.json` | `account_creation_proof_boundary_verified` | `false` | the direct theorem cannot consume the account-creation proof boundary as closed |
| `formal/kat/path2_direct_theorem_boundary_v1.json` | `qrom_and_action_bounds_linked_to_theorem` | `false` | local theorem-linkage support exists, but reviewed QROM/action theorem acceptance is still open |
| `formal/kat/path2_account_creation_proof_boundary_v1.json` | `simulator_theorem_instantiated` | `false` | no selected-payload simulator/no-export theorem is instantiated |
| `formal/kat/path2_account_creation_proof_boundary_v1.json` | `fri_aok_extractor_theorem_instantiated` | `false` | no selected Plonky3/q12289 FRI/AoK extractor theorem is instantiated |
| `formal/kat/path2_account_creation_proof_boundary_v1.json` | `fri_qrom_residual_priced` | `false` | FRI/QROM transcript and residual pricing are not closed against the selected profile |
| `formal/kat/path2_account_creation_proof_boundary_v1.json` | `production_gate_open` | `false` | production admission remains fail-closed pending review and maintainer acceptance |
| `formal/kat/path2_account_creation_proof_boundary_v1.json` | `external_review_complete` | `false` | third-party review is not complete and must not be simulated by local evidence |

## Work Classification

| Work item | Tracker owners | Local action | First-finish handling |
|---|---|---|---|
| Direct theorem composition | `DIR-SEC-001`, `DIR-RISK-001`, `DIR-CREATE-001` | maintain the explicit-assumption capstone over accepted account creation, spend, lifecycle, QROM/action, and named-break evidence | local theorem shape closed; assumption acceptance remains external/profile work |
| QROM/action reviewed theorem acceptance | `DIR-QROM-001`, `DIR-ACTION-001`, `DIR-SEC-001` | keep the local theorem-linkage reduction pinned and obtain/record the reviewed QROM/action theorem input it explicitly names | external-review hypothesis before deployment |
| Direct leaf canonical/live | `DIR-SEC-001`, `DIR-CREATE-001`, `DIR-LIFE-001` | maintain the accepted leaf hook, frozen frame corpus, and verified lifecycle-profile binding | theorem-scoped row closed; broader parser/deployment evidence remains open |
| Direct evidence/leaf binding | `DIR-SEC-001`, `DIR-CREATE-001` | maintain the accepted tuple binding hook and frozen malformed-field frame corpus | theorem-scoped row closed; broader parser/deployment evidence remains open |
| Direct admission namespace and selected-target DA | `DIR-SEC-001`, `DIR-CREATE-001`, `DIR-HANDLE-001` | maintain the exact state key, full canonical record, digest rechecks, D2 reorg behavior, Lean import, and generated admission boundary | theorem-scoped row closed for the selected sovereign state model; target database integration and external storage review remain deployment work |
| Account-creation proof-boundary consumption | `DIR-CREATE-001`, `DIR-PROOF-001`, `DIR-PROOF-002` | replace the current open residual package with closed evidence or an explicit assumption package accepted by the direct theorem | must close at the boundary level |
| Simulator/no-export theorem | `DIR-CREATE-001`, `DIR-PROOF-001`, `DIR-CT-001` | use the existing no-export allowlist, simulator-input allowlist, and reduction hooks as the theorem interface | either prove locally or carry as a named proof-family review hypothesis |
| FRI/AoK extractor theorem | `DIR-STARK-001`, `DIR-PROOF-001`, `DIR-PROOF-002` | bind the selected Plonky3/q12289 extractor statement to `Path2AccountStarkFriAokBoundaryReportV1` | normally external-review hypothesis unless a local theorem is added |
| FRI/QROM residual pricing | `DIR-QROM-001`, `DIR-STARK-001`, `DIR-REVIEW-001` | the selected-profile conditional BCS arithmetic is machine-checked; finish the Plonky3 challenger-output mapping, RBR theorem import, QROM constant review, and composition | local arithmetic closed; protocol theorem plus external review remain |
| Frozen payload and rejection corpus | `DIR-KAT-001`, `DIR-REJECT-001`, `SPEC-KAT-001` | keep current corpus gates green; add only KAT rows required by rank-1/rank-2 closure | maintain, do not expand unless a proof-boundary row requires it |
| Production gate and external review | `DIR-REVIEW-001`, `SPEC-REVIEW-001` | keep fail-closed, package review inputs, and record explicit acceptance when obtained | external/human gate, not local proof work |

## Work To Stop For Now

The following work can continue only when it directly unblocks a row above:

- additional signer leakage battery rows beyond the required proof-boundary
  evidence;
- hardware/HSM target-profile polish;
- native-boundary expansion not consumed by the direct theorem boundary;
- policy, recovery, threshold, and adaptor features;
- Mode B/rollup polish beyond the economic-viability assumptions already
  imported by the tracker.

## Next Proof Task

The direct theorem's local composition shape is now closed under exact named
assumptions. The next local task should move the deployable profile rather than
add another evidence row:

1. Complete the `ADR-0011` rate-7/capacity-9 Fiat-Shamir migration in the
   recursive verifier; the account proof is migrated, and the remaining work is
   integrating its native-fidelity/two-squeeze gadget into every recursive
   transcript block and re-pinning the recursive proof KATs.
2. Map the exact Plonky3 single-leaf transcript to the published RBR/BCS game,
   including effective `kappa >= 247`, and obtain an explicit reviewed QROM
   lift constant within the pinned `2^32` allowance.
3. Compose the selected adaptive bound with FRI/AoK, AIR, MMCS, hiding,
   Poseidon2, and
   recursion terms; keep every unavailable term as a named hypothesis.
4. Leave `production_gate_open` and `external_review_complete` false until
   `DIR-REVIEW-001` is actually satisfied.

This is the route that moves the proof toward finish without pretending the
external review gate has been completed.

## Trace Finding: QROM/Action Linkage

The numeric part of the direct QROM/action budget already exists in Lean:

- `Descender/Path2HashBudget.lean` proves
  `path2_direct_query_budget_priced`,
  `path2_direct_query_budget_activeDirectQuerySymbolsPriced`,
  `path2_direct_query_budget_auxiliary_fq12_challenge_shape_complete`, and
  `path2_direct_query_budget_fq12_action_collision_budget_priced`.
- `active_direct_budget_native_boundary_query_priced` adapts the executable
  direct budget into the native hash-budget boundary while keeping checked-core
  domain separation, hidden-source domain separation, and multi-target loss
  pricing explicit.

The local bridge and theorem-linkage support pieces are now present:
`selectedRustDirectQromActionBudgetBridge_v1` packages those budget facts,
`selectedRustDirectQromActionTheoremLinkageReduction_v1` reduces the checked-core
QROM/action theorem link to an explicit `QromActionReviewed` relation,
`Path2DirectQromActionBoundaryReportV1` records
`direct_qrom_action_budget_bridge_bound = true`, and
`Path2DirectTheoremBoundaryReportV1` records
`qrom_action_budget_bridge_imported = true`,
`qrom_action_theorem_linkage_reduction_imported = true`, and
`qrom_action_review_hypothesis_explicit = true`. This is direct theorem support,
not final QROM/action closure.

The remaining piece is reviewed theorem acceptance, not arithmetic or local
linkage shape. In
`tools/parameter-search/src/production_reference.rs`,
`path2_direct_qrom_action_boundary_report_v1` keeps
`theorem_imports_budget`, `external_qrom_review_complete`, and
`qrom_and_action_bounds_linked_to_theorem` false, and
`validate_path2_direct_qrom_action_boundary_report_v1` rejects any local flip.
`path2_direct_theorem_closure_dependency_support_open_v1` then keeps
`direct_dependency_checked_core_acceptance` open until that QROM/action report
imports the remaining theorem/review assumptions.

The selected-Rust product composition reduction is now present as
`selectedRustDirectModeAuthorizationTheoremReduction_v1`. It composes every
local positive acceptance component and exposes proof-family review,
QROM/action review, deterministic public rejection, signer/keygen review, and
external coded-SIS/lattice-core review as named hypotheses. The direct boundary
report records `composition_theorem_reduction_imported = true` and
`composition_external_hypotheses_explicit = true` while correctly leaving
`composition_theorem_instantiated = false`.
The follow-on capstone
`selectedRustDirectModeAuthorizationClosedUnderExplicitAssumptions_v1` packages
the five accepted-boundary residual reductions, explicit PQ target, explicit
selected-state nullifier closure contract used to derive F2, and the named
QROM/action, signer/keygen, and external core assumptions. The report records the capstone import as true
without changing composition instantiation or readiness flags.

The public-rejection rollback half is now imported through
`selectedRustDirectPublicRejectionStatePreserving_v1`. The direct boundary
report source-pins the Lean wrapper model to the Rust error and panic rollback
branches, total public classifier fallback, and eleven-class result table.
The centralized Rust rollback helper now has an exhaustive class-level KAT over
all nine rejection classes, an injected-panic fail-closed case, and a success
mutation control. `PATH2_DIRECT_PUBLIC_OBJECT_MUTATION_KAT_CASES_V1` and
`path2_direct_all_consensus_visible_public_object_mutations_are_panic_free`
cover all nine typed public request/state objects through the real wrapper.
`PATH2_SOVEREIGN_DIRECT_CLIENT_MAPPING_ROWS_V1` and its exhaustive KAT preserve
all eleven public classes under the selected ADR-0009/ADR-0010 sovereign client,
with every rejection mapped to no mutation. Lean packages both properties in
`selectedRustDirectDeterministicPublicRejectionClosed_v1`, and
`selectedRustDirectModeAuthorizationWithClosedPublicRejectionReduction_v1`
discharges that composition hypothesis. The theorem-scoped public-rejection row
is therefore closed; broader byte-parser and adoption-profile readiness remains
outside this closure claim.

The direct boundary now has two open rows and seven closed rows. The remaining
proof work is the account-creation proof-family/prechallenge package and the
checked-core QROM/action reviewed theorem input. External review and deployment
readiness remain fail-closed.

## Trace Finding: Leaf Canonical/Live Support

The local accepted-leaf half of `direct_obligation_leaf_canonical_live` is now
imported from the accepted-spend predicate:

- `Descender/Path2FullSpendConcrete.lean` proves
  `selectedRustDirectLeafCanonicalLiveSupport_of_concreteSpendAccepts`,
  deriving `PrevalidatedSlotLeafCanonicalAndLive` from
  `ConcreteSpendVerificationAccepts`.
- `Path2DirectTheoremBoundaryReportV1` records
  `direct_leaf_canonical_live_imported = true`,
  names that Lean theorem, and records
  `direct_leaf_canonical_live_lifecycle_and_kat_hypothesis_explicit = true`.
- `path2_direct_theorem_obligation_vectors` and the direct boundary manifest
  carry those fields through the generated evidence package.

The current frame corpus now freezes one honest case and seventeen mutation
rows, including the leaf frame-version and live-marker surfaces. Combined with
the verified ADR-0010 D2 lifecycle profile, Lean's
`SelectedRustDirectLeafAndEvidenceClosure` and
`selectedRustDirectLeafAndEvidenceClosure_of_concreteSpendAccepts` close this
direct theorem row. Broader byte-parser coverage, external review, and
deployment readiness remain fail-closed.

## Trace Finding: Evidence/Leaf Binding

The local binding half of `direct_obligation_evidence_leaf_bindings` is now
imported from the accepted-spend predicate:

- `Descender/Path2FullSpendConcrete.lean` proves
  `selectedRustDirectEvidenceLeafBindingSupport_of_concreteSpendAccepts`,
  deriving `LeafEvidenceCertificateBinding` from
  `ConcreteSpendVerificationAccepts`.
- `Path2DirectTheoremBoundaryReportV1` records
  `direct_evidence_leaf_binding_imported = true`,
  names that Lean theorem, and records
  `direct_evidence_leaf_binding_proof_boundary_hypothesis_explicit = true`.
- `path2_direct_theorem_obligation_vectors` and the direct boundary manifest
  carry those fields through the generated evidence package.

The same frozen frame corpus covers all fifteen duplicated leaf/evidence
binding fields and maps every mutation row to a public rejection class. The
shared Lean closure theorem therefore closes this direct theorem row. The
account-creation proof family, broader parser coverage, external review, and
deployment readiness remain fail-closed.

## Trace Finding: Admission Namespace And Data Availability

`direct_obligation_admission_namespace` is now theorem-scoped closed for the
selected ADR-0009 sovereign rollup and ADR-0010 D2 lifecycle profile:

- `Descender/Path2FullSpendConcrete.lean` proves
  `selectedRustDirectAdmissionNamespaceMembershipSupport_of_concreteSpendAccepts`,
  deriving `DirectAdmissionContains` for the accepted account/state-root/slot
  authorization tuple directly from `ConcreteSpendVerificationAccepts`.
- `Path2DirectTheoremBoundaryReportV1` records
  `direct_admission_namespace_membership_imported = true`,
  `direct_admission_namespace_and_data_availability_imported = true`, and
  `direct_admission_namespace_closed_by_sovereign_state = true`.
- `Path2SovereignDirectAdmissionStateV1` stores a full canonical leaf record
  under the exact `(account_id, account_state_root_before,
  slot_authorization_digest)` key. The selected account-STARK verifier also
  supplies an opaque proof-bound nullifier candidate; admission atomically
  rejects duplicate tuple or raw nullifier keys across a whole candidate block.
  Spend rejects missing records and fails closed on record corruption after
  rechecking canonical leaf bytes, leaf digest, evidence transcript digest, and
  nullifier-index consistency.
- The executable state implements D2 finalized-head reorg behavior: records
  above finality may be removed by reorg and their nullifier entries rebuilt
  from retained records, while reorg below finalized head is rejected.
- Lean proves the reduction
  `selectedRustSovereignPadSeedReorgSafeUniqueness_of_verifiedNullifierRegistry_v1`,
  deriving F2 from deterministic verified nullifiers, the global registry, and
  D2 lifecycle facts supplied through an explicit selected-state closure
  contract. The report does not claim a cross-language instantiation proof.
- Lean proves
  `selectedRustDirectAdmissionNamespaceAndDataAvailabilityClosure_of_concreteSpendAccepts`,
  and the selected product reduction imports that closure directly.
- `Path2DirectAdmissionBoundaryReportV1` and
  `formal/kat/path2_direct_admission_boundary_v1.json` pin the state schema,
  record schema, key/value fields, endpoints, tests, Lean theorem, and report
  digest.
- `path2_direct_theorem_obligation_vectors` and the direct boundary manifest
  now carry those fields through the generated evidence package.

This instantiates theorem-scoped data availability because the canonical record
is directly present in the selected consensus state read by spend verification.
It does not claim a production database implementation: target-specific chain
database binding, restore-anchor placement review, external storage review,
final composition, external cryptographic review, and deployment readiness
remain fail-closed.
