# Research Archive Index

Status: ACTIVE_ARCHIVE_INDEX

This file labels high-confusion research and formal-history documents during the
project pivot to a chain-native PQ authorization stack.

Archive status does not mean "delete". It means "do not cite as the active
product target unless a current spec or ADR explicitly says so."

## Status Labels

| Label | Meaning |
|---|---|
| `ACTIVE_SUPPORT` | supports the current spec spine |
| `ACTIVE_RESEARCH` | relevant research, not normative |
| `NEGATIVE_CONTROL` | retained because it records an attack, failed route, or regression case |
| `LEGACY_FORMAL` | formal/KAT lineage retained for history or tool conformance |
| `ARCHIVED_TARGET` | old product target, no longer selected |
| `SUPERSEDED_TRACKER` | old tracker replaced by `TRACKER.md` |
| `SUPERSEDED_DECISION` | old decision note replaced by append-only ADRs |
| `MIXED_STATUS` | useful but internally historical/corrective; cite carefully |

## Retention Policy

Classification is also the cleanup disposition. A row in this archive index
does not by itself keep stale material forever; it records why the material is
kept now and what must be true before it can be removed or relocated.

| Label | Default disposition | Removal rule |
|---|---|---|
| `ACTIVE_SUPPORT` | keep as cited support for the current spec spine | remove only after the importing spec, ADR, or `TRACKER.md` row is updated to no longer depend on it |
| `ACTIVE_RESEARCH` | keep as non-normative research evidence | remove or relocate after no current spec, ADR, tracker row, selected support row, or active package entrypoint imports it |
| `NEGATIVE_CONTROL` | keep as attack, failed-route, or regression evidence | remove only after the negative case is captured in an active regression/KAT/audit artifact or explicitly retired by ADR or tracker row |
| `LEGACY_FORMAL` | keep as legacy formal/KAT/conformance history | remove or relocate after current formal authority and regression evidence replace the retained value, with tracker or ADR coverage |
| `ARCHIVED_TARGET` | archive only as old product-target history | remove or relocate after dependencies are gone and a tracker row or ADR records the retirement |
| `SUPERSEDED_TRACKER` | archive only until all live work has been copied into `TRACKER.md` | remove or relocate after `TRACKER.md` owns all remaining live items and no active support row imports the old tracker |
| `SUPERSEDED_DECISION` | archive only as pre-ADR decision history | remove or relocate after the relevant ADRs cover the history and no active support row imports the old decision note |
| `MIXED_STATUS` | keep temporarily while useful evidence is split or routed | split into active support, negative-control, or archived target material before removal, or retire by tracker row or ADR |

## High-Confusion Files

| File | Label | Current interpretation |
|---|---|---|
| `formal/spec/descender-signature-spec.md` | `LEGACY_FORMAL` | v0 formal spine for the projection-carrier lineage, not the current product spec |
| `formal/spec/path_change_investigation.md` | `ACTIVE_RESEARCH` | useful design-history evidence explaining 260/308/314 divergence |
| `formal/README.md` | `ACTIVE_SUPPORT` | formal-verification package entrypoint, subordinate to the current spec spine |
| `formal/jasmin/README.md` | `ACTIVE_RESEARCH` | Jasmin deployable-implementation layer entrypoint, not product authority by itself |
| `formal/kat/README.md` | `LEGACY_FORMAL` | KAT v0 package entrypoint for the frozen legacy byte-conformance corpus |
| `formal/lean/README.md` | `ACTIVE_SUPPORT` | Lean math and reduction-skeleton layer entrypoint for current Path 2 closure work |
| `AGENTS.md` | `ACTIVE_SUPPORT` | repo handoff entrypoint for coding and research agents, subordinate to the current spec spine |
| `legal/README.md` | `ACTIVE_RESEARCH` | legal/commercialization package entrypoint, not technical product authority |
| `legal/go_to_market_plan.md` | `ACTIVE_RESEARCH` | legal/commercialization go-to-market plan, not technical product authority |
| `papers/descent_based_cryptography.md` | `ACTIVE_RESEARCH` | theoretical descent-cryptography exposition, not a Descender algorithm specification |
| `research/account_creation_proof_deployability_status.md` | `ACTIVE_RESEARCH` | historical red-team deployability status note retained as research evidence; current status belongs in `TRACKER.md` |
| `research/path2_signature_statement_circularity_blocker.md` | `ACTIVE_RESEARCH` | split-statement blocker history retained as research evidence; current proof status lives in the current specs and tracker |
| `research/path2_stark_route_af_hash_selection.md` | `ACTIVE_RESEARCH` | Poseidon2/STARK route history retained as research evidence; current authority remains the Plonky3/KoalaBear path in `spec/` and ADRs |
| `research/path2_zk_masked_trace_iop_assumption.md` | `ACTIVE_RESEARCH` | superseded masked-trace IOP assumption note retained as research history; current authority is the native-boundary assumption surface |
| `tools/README.md` | `ACTIVE_SUPPORT` | tool package entrypoint; routes implementation readers back to the current spec spine and tracker |
| `tools/production-reference/README.md` | `LEGACY_FORMAL` | production-reference lineage documentation, not current product spec |
| `tools/cryptanalysis/README.md` | `ACTIVE_RESEARCH` | external cryptanalysis harness entrypoint, not a security claim by itself |
| `tools/parameter-search/README.md` | `ACTIVE_RESEARCH` | Rust parameter-search engine entrypoint; generated results are research evidence |
| `scripts/parameter_search/README.md` | `ACTIVE_RESEARCH` | Python parameter-search reference harness entrypoint |
| `formal/kat/kat_v0.md` | `LEGACY_FORMAL` | KAT v0 pins legacy/current-at-the-time linear-sigma path, not final Path 2 product |
| `formal/jasmin/JASMIN_SCOPE.md` | `ACTIVE_RESEARCH` | Jasmin verifier-core implementation research; not product authority or current byte-layout authority |
| `research/accepting_308b_verifier_blocker.md` | `NEGATIVE_CONTROL` | records why old projection-carrier verifier must remain fail-closed |
| `research/cryptanalysis/creation_authorization_theorem_status.md` | `ACTIVE_RESEARCH` | older theorem-status snapshot retained as research evidence with supersession notes |
| `research/cryptanalysis/deployable_pad_hiding_VERDICT.md` | `ACTIVE_RESEARCH` | spend-path hiding evidence retained as research evidence with creation-proof caveats |
| `research/cryptanalysis/high2_zk_inclear_blind_leak_VERDICT.md` | `NEGATIVE_CONTROL` | red-team hiding failure and regression closure record |
| `research/cryptanalysis/high3_challenge_widening_PLAN.md` | `ACTIVE_RESEARCH` | older hard-128 remediation plan retained as research history; current policy is ADR-0006 |
| `research/cryptanalysis/leakage_battery_VERDICT.md` | `ACTIVE_RESEARCH` | leakage evidence and scaffold-signer negative-control history retained as research evidence |
| `audit/README.md` | `ACTIVE_RESEARCH` | audit package entrypoint; routes audit readers back to the current spec spine and tracker |
| `audit/active_path_signature_independent_red_team_20260602.md` | `ACTIVE_RESEARCH` | independent red-team audit evidence for the historical active-path signature surface |
| `audit/active_path_wide_challenge_action_algebraic_soundness.md` | `ACTIVE_RESEARCH` | challenge-action widening evidence superseded by current Poseidon2/QROM budget artifacts |
| `audit/path2_native_full_signature_obligation_burndown.md` | `ACTIVE_RESEARCH` | proof-obligation evidence retained as audit/research history, not top-level project status |
| `audit/q12289_full_signature_obligation_burndown.md` | `ACTIVE_RESEARCH` | q12289 fallback/reference proof evidence, not the current deployable target |
| `research/pre_signature_projection_leakage_audit.md` | `ACTIVE_RESEARCH` | legacy projection-carrier leakage analysis retained as Path 2 leakage-history evidence |
| `research/cryptanalysis/public_creation_verifier_build_result.md` | `NEGATIVE_CONTROL` | historical public masked-state18 verifier build record; current implementation is hard-gated for leakage |
| `research/account_creation_deployed_integration_runbook.md` | `ACTIVE_RESEARCH` | older deployed-integration runbook retained for implementation history, not current deployment authority |
| `research/cryptanalysis/README.md` | `ACTIVE_RESEARCH` | cryptanalysis package entrypoint; routes security readers back to the current spec spine |
| `research/cryptanalysis/RED_TEAM_REPORT.md` | `NEGATIVE_CONTROL` | internal break-and-fix campaign retained as adversarial evidence and regression seed material |
| `research/cryptanalysis/SECURITY_DOSSIER.md` | `ACTIVE_RESEARCH` | high-value external-review package retained as research evidence; current security authority is `spec/security_model.md` |
| `research/cryptanalysis/accepted_spend_reduction_result.md` | `ACTIVE_RESEARCH` | accepted-spend reduction evidence supporting the current security model |
| `research/cryptanalysis/assumption_ledger_authoritative.md` | `ACTIVE_RESEARCH` | old WS1 assumption ledger retained as research evidence; current assumption authority is `spec/security_model.md` |
| `research/cryptanalysis/beyond_box_kernel_structure_hunt.md` | `ACTIVE_RESEARCH` | beyond-box coded-SIS structure-hunt evidence |
| `research/cryptanalysis/coded_sis_hardness_reframe.md` | `ACTIVE_RESEARCH` | coded-SIS hardness-framing note for current external-review planning |
| `research/cryptanalysis/creation_time_binding_rank_analysis.md` | `ACTIVE_RESEARCH` | creation-time binding rank-analysis evidence |
| `research/cryptanalysis/degree5_field_rehost_scope.md` | `ACTIVE_RESEARCH` | challenge-field rehost scope analysis for the account-creation STARK |
| `research/cryptanalysis/frame_prf_poseidon2_ro_reduction.md` | `ACTIVE_RESEARCH` | Poseidon2 frame-PRF random-oracle reduction note |
| `research/cryptanalysis/frame_prf_ro_discharge.md` | `ACTIVE_RESEARCH` | frame-PRF RO/QROM discharge note |
| `research/cryptanalysis/fri_aok_extractor_instantiation.md` | `ACTIVE_RESEARCH` | FRI argument-of-knowledge extractor instantiation for account creation |
| `research/cryptanalysis/assumption_boundary_stress_test_20260627.md` | `ACTIVE_RESEARCH` | adversarial stress test of the deployable assumption boundary (quantum FS-grind shortfall, pad-seed freshness seam, quantum Merkle-collision floor) |
| `research/cryptanalysis/gap_c_remodel_result.md` | `ACTIVE_RESEARCH` | second-preimage remodeling result for binding-break budget closure |
| `research/cryptanalysis/hiding_sampler_discipline_audit.md` | `ACTIVE_RESEARCH` | sampler-discipline audit evidence for witness hiding |
| `research/cryptanalysis/high2_transparent_descent_zk_build.md` | `ACTIVE_RESEARCH` | historical transparent-descent ZK build result retained as research evidence, not current product path authority |
| `research/cryptanalysis/high2_transparent_descent_zk_design.md` | `ACTIVE_RESEARCH` | historical transparent-descent ZK design spec retained as research evidence, not normative product spec |
| `research/cryptanalysis/high3_challenge_widening_fix.md` | `ACTIVE_RESEARCH` | challenge-widening fix evidence for the older checked-core byte lineage |
| `research/cryptanalysis/leakage_battery_findings.md` | `NEGATIVE_CONTROL` | empirical leakage finding retained as regression and review seed material |
| `research/cryptanalysis/lean_ec_seam_discharge.md` | `ACTIVE_RESEARCH` | Lean/EasyCrypt seam-discharge evidence and reviewer checklist |
| `research/cryptanalysis/per_event_epsilon_budget_closure.md` | `ACTIVE_RESEARCH` | historical free-birthday budget analysis and later correction record retained as research evidence |
| `research/cryptanalysis/public_sis_lattice_reduction_findings.md` | `ACTIVE_RESEARCH` | public-SIS lattice-reduction findings for external review |
| `research/cryptanalysis/recursive_fri_extractor_argument.md` | `ACTIVE_RESEARCH` | recursive-FRI extractor argument for optional aggregation mode |
| `research/cryptanalysis/redteam_boundary_composition.md` | `ACTIVE_RESEARCH` | red-team boundary/composition evidence |
| `research/cryptanalysis/redteam_implementation_malleability.md` | `ACTIVE_RESEARCH` | red-team malleability evidence and fix context |
| `research/cryptanalysis/redteam_lattice_assault.md` | `ACTIVE_RESEARCH` | red-team lattice assault evidence |
| `research/cryptanalysis/redteam_leakage_keyrecovery.md` | `ACTIVE_RESEARCH` | red-team leakage/key-recovery evidence for historical masked-state paths |
| `research/cryptanalysis/redteam_state_machine.md` | `ACTIVE_RESEARCH` | red-team state-machine/lifecycle evidence |
| `research/cryptanalysis/sponge_capacity_c9_hardening_scope.md` | `ACTIVE_RESEARCH` | hard-128 sponge-capacity scope analysis retained for policy history |
| `research/cryptanalysis/ws3_concrete_security_findings.md` | `ACTIVE_RESEARCH` | concrete-security findings and arithmetic evidence |
| `research/cryptanalysis/ws4_empirical_probes.md` | `ACTIVE_RESEARCH` | empirical structural cryptanalysis probe evidence |
| `research/cryptanalysis/ws7_reduction_scoping.md` | `ACTIVE_RESEARCH` | concrete reduction scoping evidence |

## High-Confusion Packages

| Package | Label | Current interpretation |
|---|---|---|
| `formal/easycrypt/**` | `ACTIVE_RESEARCH` | EasyCrypt reduction evidence and formal-methods context, not product authority by itself |
| `formal/hax/**` | `ACTIVE_RESEARCH` | hax bridge-spike evidence and implementation-proof context, not product authority by itself |
| `research/cryptanalysis/signature_readiness_20260516/**` | `ACTIVE_RESEARCH` | May 2026 signature-readiness campaign retained as research evidence, not current status |
| `research/cryptanalysis/n32_m1_32_structure/**` | `NEGATIVE_CONTROL` | structural attack evidence for older n=32, m1=32 parameter families |
| `research/cryptanalysis/overnight_20260515_diag_certified/**` | `ACTIVE_RESEARCH` | external-tool export package for older certified diagnostic rows |
| `research/parameter_search/**` | `ACTIVE_RESEARCH` | generated parameter-search and validation reports; evidence only unless imported by current specs or ADRs |
| `tools/external/lattice-estimator/**` | `ACTIVE_RESEARCH` | vendored external estimator documentation and templates |

## Active Support Files

| File | Label | Current role |
|---|---|---|
| `research/CLEANUP_QUEUE.md` | `ACTIVE_SUPPORT` | exact cleanup-disposition queue for every non-active archive row, subordinate to `TRACKER.md` |
| `research/account_creation_prevalidated_slot_plan.md` | `ACTIVE_SUPPORT` | selected Path 2 prevalidated-slot planning note |
| `research/account_creation_prevalidated_slot_proof_tracker.md` | `ACTIVE_SUPPORT` | detailed proof-side Path 2 checklist, subordinate to `TRACKER.md` |
| `research/account_creation_prevalidated_slot_authorization_or_break.md` | `ACTIVE_SUPPORT` | direct-mode authorization-or-break theorem statement supporting `spec/security_model.md` |
| `research/account_creation_prevalidated_slot_proof_boundary_ledger.md` | `ACTIVE_SUPPORT` | proof-boundary ledger for direct single-slot deployment mode |
| `research/account_creation_step5_verifier_wiring_status.md` | `ACTIVE_SUPPORT` | status note for deployed account-creation STARK verifier wiring |
| `research/direct_replayable_relation_certificate_soundness_theorem.md` | `ACTIVE_SUPPORT` | replayable relation certificate theorem target for account creation |
| `research/creation_time_negative_control_matrix.md` | `ACTIVE_SUPPORT` | active negative-control matrix for Path 2 account creation |
| `research/hidden_source_projection_design_fork.md` | `ACTIVE_SUPPORT` | design history supporting the Path 2 pivot and projection-carrier archival decision |
| `research/prevalidated_slot_leaf_v1_frame.md` | `ACTIVE_SUPPORT` | current leaf and authorization digest frame |
| `research/checked_core_fs_poseidon2_rehost.md` | `ACTIVE_SUPPORT` | Poseidon2 checked-core Fiat-Shamir re-host evidence supporting ADR-0004 and `spec/wire_formats.md` |
| `research/path2_account_stark_air_soundness_audit.md` | `ACTIVE_SUPPORT` | account-creation STARK soundness audit supporting the active Poseidon2/Plonky3 account-creation path |
| `research/path2_account_stark_arithmetization_soundness_theorem_v1.md` | `ACTIVE_SUPPORT` | positive clause-by-clause arithmetization-soundness theorem (trace ⊨ AIR ⟹ R_air) supporting `DIR-STARK-001` and the active Poseidon2/Plonky3 account-creation path |
| `research/path2_account_stark_simulator_no_export_theorem_v1.md` | `ACTIVE_SUPPORT` | simulator / no-export theorem for `Path2AccountStarkCreationPayloadV1` supporting `DIR-PAYLOAD-001`/`DIR-CREATE-001` and the no-export boundary target |
| `research/path2_account_stark_transcript_qrom_theorem_v1.md` | `ACTIVE_SUPPORT` | account-STARK Fiat–Shamir/QROM transcript-soundness theorem and classical+quantum pricing supporting `DIR-QROM-001`/`DIR-PROOF-001` |
| `research/path2_account_stark_replay_anchor_binding_theorem_v1.md` | `ACTIVE_SUPPORT` | theorem-level replay-payload + deployed-anchor binding (`R_wiring`) supporting `DIR-BIND-001` |
| `research/path2_authorization_class_binding_theorem_v1.md` | `ACTIVE_SUPPORT` | authorization-class binding theorem (typed compatibility + field-completeness) supporting `DIR-AUTHCLASS-001` |
| `research/path2_honest_create_sampler_timing_theorem_v1.md` | `ACTIVE_SUPPORT` | honest-create sampler soundness + no-postchallenge timing (anti-circularity) theorem supporting `DIR-CREATE-001` |
| `research/path2_account_stark_payload_injectivity_rejection_completeness_v1.md` | `ACTIVE_SUPPORT` | payload byte-model parser-injectivity + rejection-class completeness supporting `DIR-STARK-001`/`DIR-REJECT-001` |
| `research/path2_lifecycle_finality_theorem_v1.md` | `ACTIVE_SUPPORT` | consume-once lifecycle theorem with profiled reorg/finality assumption supporting `DIR-LIFE-001` |
| `research/path2_economic_deployment_architecture_v1.md` | `ACTIVE_SUPPORT` | economic deployment-architecture decision analysis (4.75 MB creation proof ⇒ PQ validity rollup + native STARK recursion) supporting `AGG-ROLLUP-001`/`ADOPT-ETH-ROLLUP-001` |
| `research/path2_q128_build_status_and_architecture_v1.md` | `ACTIVE_SUPPORT` | consolidated q128 implementation status and architecture map supporting `AGG-ROLLUP-001` |
| `research/path2_quantum128_parameterization_v1.md` | `ACTIVE_SUPPORT` | exact atomic quantum-128 parameter set (queries 120, rate 7/cap 9, digest 13) implementing `ADR-0008`, supporting `DIR-QROM-001`/`DIR-STARK-001`/`ADOPT-128-001` |
| `research/slot_authorization_handle_v1_spec.md` | `ACTIVE_SUPPORT` | normalized spend-time authorization handle interface |
| `research/account_creation_evidence_v1_spec.md` | `ACTIVE_SUPPORT` | current creation evidence envelope |
| `research/creation_hidden_source_relation_v1_spec.md` | `ACTIVE_SUPPORT` | account-creation hidden-source relation for Path 2 |
| `research/spend_authorization_relation_v1_spec.md` | `ACTIVE_SUPPORT` | current spend relation spec |
| `research/path2_one_use_successor_state_theorem.md` | `ACTIVE_SUPPORT` | direct-mode one-use successor-state theorem slice supporting `spec/lifecycle_model.md` |
| `research/creation_time_negative_control_kat_plan.md` | `ACTIVE_SUPPORT` | negative-control KAT planning for creation-time Path 2 failures |
| `research/path2_native_checked_core_frame_v1_spec.md` | `ACTIVE_SUPPORT` | active checked-core challenge frame design note |
| `research/path2_native_checked_core_frame_kat_plan_v1.md` | `ACTIVE_SUPPORT` | KAT plan for the active checked-core frame |
| `research/path2_checked_core_frame_binding_v1.md` | `ACTIVE_SUPPORT` | checked-core frame binding theorem target |
| `research/path2_checked_core_frame_byte_audit_v1.md` | `ACTIVE_SUPPORT` | byte-level audit for the native checked-core frame |
| `research/path2_checked_core_spend_reduction_bridge_v1.md` | `ACTIVE_SUPPORT` | concrete bridge for checked-core spend reduction |
| `research/path2_checked_obstruction_zero_bridge_v1.md` | `ACTIVE_SUPPORT` | concrete bridge target for obstruction-zero checked conditions |
| `research/path2_native_consensus_carrier_v1_spec.md` | `ACTIVE_SUPPORT` | selected first native consensus carrier note |
| `research/path2_native_projection_preimage_encoders_v1_spec.md` | `ACTIVE_SUPPORT` | native projection preimage encoder spec for Path 2 |
| `research/account_creation_proof_mechanism_decision.md` | `ACTIVE_SUPPORT` | proof-mechanism selection note for the first Path 2 theorem pass |
| `research/account_creation_aggregation_readiness_contract.md` | `ACTIVE_SUPPORT` | optional aggregation mode contract |
| `research/path2_proof_side_closure_status_v1.md` | `ACTIVE_SUPPORT` | proof-side status of active native Path 2 shape |
| `research/path2_payload_proof_candidate_decision.md` | `ACTIVE_SUPPORT` | active account-creation payload proof candidate note |
| `research/path2_descent_native_vs_stark_creation_verdict.md` | `ACTIVE_SUPPORT` | analysis supporting the STARK/recursion account-creation route |
| `research/path2_active_native_account_design_v1.md` | `ACTIVE_SUPPORT` | active native-account semantic boundary design note |
| `research/path2_concrete_solution_search_strategy.md` | `ACTIVE_SUPPORT` | active strategy for selecting/falsifying native account design candidates |
| `research/path2_real_commitment_options.md` | `ACTIVE_SUPPORT` | proof-family design gate for choosing PQ account-creation commitments |
| `research/path2_core_algorithm_red_team_v1.md` | `ACTIVE_SUPPORT` | adversarial review of the active Path 2 native-account theorem surface |
| `research/path2_transparent_relation_arithmetization_target.md` | `ACTIVE_SUPPORT` | transparent relation arithmetization target for account creation |
| `research/path2_transparent_iop_knowledge_theorem_probe.md` | `ACTIVE_SUPPORT` | transparent proof-family theorem obligations for Path 2 account creation |
| `research/path2_transparent_payload_contract.md` | `ACTIVE_SUPPORT` | transparent masked-trace payload contract for account-creation proof bytes |
| `research/path2_transparent_extractor_soundness_bridge.md` | `ACTIVE_SUPPORT` | extractor/soundness bridge from transparent payload to direct account-admission certificate |
| `research/path2_masked_trace_iop_selection_checklist.md` | `ACTIVE_SUPPORT` | masked-trace IOP selection checklist for Path 2 |
| `research/path2_native_boundary_zk_trace_checklist.md` | `ACTIVE_SUPPORT` | native-boundary ZK trace checklist |
| `research/path2_native_boundary_zk_trace_binding_hiding_assumption.md` | `ACTIVE_SUPPORT` | active native-boundary trace binding/hiding assumption surface |
| `research/path2_native_boundary_parent_trace_extraction_open_obligations.md` | `ACTIVE_SUPPORT` | open-obligation checklist for parent-trace extraction soundness |
| `research/path2_native_boundary_blinding_noise_decision.md` | `ACTIVE_SUPPORT` | blinding-noise decision note for the native boundary |
| `research/path2_native_boundary_308b_checked_core_audit.md` | `ACTIVE_SUPPORT` | compatibility audit between the native boundary and older 308-byte checked-core evidence |
| `research/path2_native_boundary_blinding_noise_bound.md` | `ACTIVE_SUPPORT` | blinding/noise bound analysis for the native boundary |
| `research/path2_native_boundary_blinding_noise_row_estimate.md` | `ACTIVE_SUPPORT` | row-cost estimate for native-boundary blinding/noise choices |
| `research/path2_native_boundary_checked_core_compat_estimate.md` | `ACTIVE_SUPPORT` | checked-core compatibility estimate for the native boundary |
| `research/path2_native_boundary_column_profile_impact.md` | `ACTIVE_SUPPORT` | column-profile impact analysis after the native-boundary refactor |
| `research/path2_native_boundary_column_profile_v1.md` | `ACTIVE_SUPPORT` | native-boundary column-profile design note |
| `research/path2_native_boundary_context_lifecycle_row_estimate.md` | `ACTIVE_SUPPORT` | context/lifecycle row estimate for the native boundary |
| `research/path2_native_boundary_core_problem_reframe.md` | `ACTIVE_SUPPORT` | core problem reframe for the active native-boundary route |
| `research/path2_native_boundary_creative_route_comparison.md` | `ACTIVE_RESEARCH` | route comparison for native-boundary proof options, not a selected proof decision by itself |
| `research/path2_native_boundary_hc_factorization_ledger.md` | `ACTIVE_SUPPORT` | hidden-commitment factorization ledger for native-boundary proof work |
| `research/path2_native_boundary_hidden_extractor_view.md` | `ACTIVE_SUPPORT` | hidden-extractor view for native-boundary soundness |
| `research/path2_native_boundary_hybrid_base128_range_policy.md` | `ACTIVE_SUPPORT` | hybrid base-128 range policy supporting native-boundary arithmetization |
| `research/path2_native_boundary_hybrid_base128_sis_sanity.md` | `ACTIVE_RESEARCH` | SIS sanity check for hybrid base-128 native-boundary surfaces |
| `research/path2_native_boundary_knowledge_soundness_decomposition.md` | `ACTIVE_SUPPORT` | knowledge-soundness decomposition for native-boundary proof work |
| `research/path2_native_boundary_no_export_hiding.md` | `ACTIVE_SUPPORT` | no-export hiding analysis for native-boundary public views |
| `research/path2_native_boundary_norm_range_selection.md` | `ACTIVE_SUPPORT` | norm/range selection surface for native-boundary proofs |
| `research/path2_native_boundary_norm_range_soundness.md` | `ACTIVE_SUPPORT` | norm/range soundness argument for native-boundary proofs |
| `research/path2_native_boundary_open_row_estimates.md` | `ACTIVE_SUPPORT` | open row estimates for native-boundary trace designs |
| `research/path2_native_boundary_opening_tuple_account_binding_estimate.md` | `ACTIVE_SUPPORT` | opening-tuple account-binding row estimate |
| `research/path2_native_boundary_parent_root_role_binding.md` | `ACTIVE_SUPPORT` | parent-root role-binding analysis for native-boundary traces |
| `research/path2_native_boundary_parent_trace_extraction_contract.md` | `ACTIVE_SUPPORT` | parent-trace extraction contract for native-boundary proof systems |
| `research/path2_native_boundary_private_role_restriction_v1.md` | `ACTIVE_SUPPORT` | private role-restriction design for native-boundary traces |
| `research/path2_native_boundary_projection_audit.md` | `ACTIVE_SUPPORT` | projection audit for native-boundary public surfaces |
| `research/path2_native_boundary_proof_family_extractor_classification.md` | `ACTIVE_SUPPORT` | extractor classification for candidate native-boundary proof families |
| `research/path2_native_boundary_proof_family_overhead_budget.md` | `ACTIVE_SUPPORT` | overhead budget for native-boundary proof-family selection |
| `research/path2_native_boundary_relation_knowledge_route_matrix.md` | `ACTIVE_SUPPORT` | route matrix for native-boundary relation knowledge soundness |
| `research/path2_native_boundary_relation_knowledge_soundness_contract.md` | `ACTIVE_SUPPORT` | knowledge-soundness contract for the native-boundary relation |
| `research/path2_native_boundary_role_authentication_mode_map.md` | `ACTIVE_SUPPORT` | role-authentication mode map for native-boundary traces |
| `research/path2_native_boundary_role_completeness_table.md` | `ACTIVE_SUPPORT` | role-completeness table for native-boundary trace duties |
| `research/path2_native_boundary_role_restriction_row_estimate.md` | `ACTIVE_SUPPORT` | row estimate for native-boundary role restrictions |
| `research/path2_native_boundary_role_restriction_soundness.md` | `ACTIVE_SUPPORT` | soundness note for native-boundary role restrictions |
| `research/path2_native_boundary_row_estimate_scenarios.md` | `ACTIVE_SUPPORT` | native-boundary row-estimate scenarios |
| `research/path2_native_boundary_trace_assembly_map.md` | `ACTIVE_SUPPORT` | trace assembly map for the native-boundary proof route |
| `research/path2_native_boundary_trace_proof_family_interface.md` | `ACTIVE_SUPPORT` | trace proof-family interface for native-boundary proof systems |
| `research/path2_native_boundary_trace_relation_row_symbols.md` | `ACTIVE_SUPPORT` | row-symbol inventory for the native-boundary trace relation |
| `research/path2_native_boundary_trace_relation_v1_inventory.md` | `ACTIVE_SUPPORT` | v1 inventory for the native-boundary trace relation |
| `research/path2_native_boundary_transparent_proof_family_v1.md` | `ACTIVE_SUPPORT` | transparent proof-family shape for the native boundary |
| `research/path2_native_boundary_v1_interface.md` | `ACTIVE_SUPPORT` | v1 native-boundary interface note |
| `research/path2_native_boundary_zk_trace_qrom_schedule.md` | `ACTIVE_SUPPORT` | QROM schedule for native-boundary ZK trace proof work |
| `research/path2_authorization_class_binding_proof_v1.md` | `ACTIVE_SUPPORT` | authorization-class binding proof attempt for the active native boundary |
| `research/path2_authorization_local_classification.md` | `ACTIVE_SUPPORT` | authorization-local classification pass supporting spend closure |
| `research/path2_context_state_domain_binding_v1.md` | `ACTIVE_SUPPORT` | context/state domain-binding theorem target |
| `research/path2_hash_qrom_assumption_ledger.md` | `ACTIVE_SUPPORT` | direct-mode hash and QROM assumption ledger |
| `research/path2_leaf_registry_state_binding_theorem.md` | `ACTIVE_SUPPORT` | leaf, registry, and state binding theorem slice |
| `research/path2_no_postchallenge_creation_timing_game.md` | `ACTIVE_SUPPORT` | no-postchallenge creation timing rule for spend authorization |
| `research/path2_ordered_encoder_hash_binding_bridge_v1.md` | `ACTIVE_SUPPORT` | ordered-encoder/hash-binding bridge target |
| `research/path2_residual_assumption_ledger_v1.md` | `ACTIVE_SUPPORT` | residual assumption ledger for native proof-side closure |
| `research/path2_spend_closure_final_composition_v1.md` | `ACTIVE_SUPPORT` | final spend-closure composition target for the active native proof side |
| `research/path2_spend_closure_over_q.md` | `ACTIVE_SUPPORT` | spend-closure contract over native boundary `q` |
| `research/path2_spend_projection_completeness_audit_v1.md` | `ACTIVE_SUPPORT` | spend projection completeness audit for authorization-class binding |
| `research/path2_descent_sis_feasibility.md` | `ACTIVE_RESEARCH` | descent/SIS feasibility and role-vector analysis, not a deployable proof-size claim |
| `research/path2_descent_sis_norm_range_surface.md` | `ACTIVE_SUPPORT` | active hybrid-base128 norm/range policy surface |
| `research/path2_descent_sis_hardness_sanity.md` | `ACTIVE_RESEARCH` | executable sanity blocker for descent/SIS role surfaces, not a hardness estimate |
| `research/path2_candidate_pullback_boundary.md` | `ACTIVE_RESEARCH` | pullback-boundary candidate sheet, not selected proof authority by itself |
| `research/path2_candidate_obstruction_boundary.md` | `ACTIVE_RESEARCH` | obstruction-boundary candidate sheet, not selected proof authority by itself |
| `research/path2_candidate_canonical_gauge.md` | `ACTIVE_RESEARCH` | canonical-gauge candidate sheet, not selected proof authority by itself |
| `research/path2_concrete_oglue_encoding_v1.md` | `ACTIVE_SUPPORT` | concrete `O_gluing` encoding support note for edge-linked Path 2 |
| `research/path2_current_core_negative_theorem.md` | `ACTIVE_RESEARCH` | negative theorem for the current executable masked-share core |
| `research/path2_current_view_binding_negative_theorem.md` | `ACTIVE_RESEARCH` | negative theorem for the current executable transparent-view binding shape |
| `research/path2_descent_framing_opportunities.md` | `ACTIVE_RESEARCH` | descent-framing opportunity note, not a selected requirement by itself |
| `research/path2_descent_sis_binding_surface.md` | `ACTIVE_SUPPORT` | binding game and role-surface note for descent/SIS commitments |
| `research/path2_descent_sis_commitment_viability.md` | `ACTIVE_RESEARCH` | viable descent/SIS research path, not yet deployable commitment authority |
| `research/path2_descent_sis_hiding_redesign_options.md` | `ACTIVE_RESEARCH` | redesign options for descent/SIS hiding blockers, not selected by itself |
| `research/path2_descent_sis_lattice_sigma_toy_model.md` | `ACTIVE_RESEARCH` | toy transcript model for descent/SIS proof exploration |
| `research/path2_descent_sis_matrix_domains.md` | `ACTIVE_SUPPORT` | executable matrix-domain closure note for descent/SIS role surfaces |
| `research/path2_descent_sis_obligation_matrix.md` | `ACTIVE_SUPPORT` | obligation matrix for descent/SIS closure work |
| `research/path2_descent_sis_private_zk_no_export_contract.md` | `ACTIVE_SUPPORT` | private-ZK no-export contract for descent/SIS route analysis |
| `research/path2_descent_sis_private_zk_payload_surface.md` | `ACTIVE_RESEARCH` | private-ZK payload surface for descent/SIS route exploration |
| `research/path2_descent_sis_private_zk_proof_family_gate.md` | `ACTIVE_SUPPORT` | proof-family gate for descent/SIS private-ZK route |
| `research/path2_descent_sis_private_zk_qrom_frame.md` | `ACTIVE_SUPPORT` | QROM frame contract for descent/SIS private-ZK route |
| `research/path2_descent_sis_rank_hiding_probe.md` | `ACTIVE_RESEARCH` | rank/hiding probe for descent/SIS blocker analysis |
| `research/path2_descent_sis_type_map.md` | `ACTIVE_SUPPORT` | type-map support note for descent/SIS obligation closure |
| `research/path2_direct_creation_hiding_payload_cost_model.md` | `ACTIVE_SUPPORT` | cost model for direct-creation hiding payload choices |
| `research/path2_direct_relation_certificate_payload_blocker.md` | `ACTIVE_SUPPORT` | direct relation-certificate payload blocker and closure note |
| `research/path2_fiber_privacy_no_export_v1.md` | `ACTIVE_SUPPORT` | fiber privacy and no-export theorem target |
| `research/path2_full_relation_backend_sufficiency_theorem.md` | `ACTIVE_SUPPORT` | full-relation backend sufficiency theorem target |
| `research/path2_gauge_lift_v1_attempt.md` | `ACTIVE_RESEARCH` | first gauge/lift attempt, not selected proof authority by itself |
| `research/path2_geometric_descent_lens.md` | `ACTIVE_RESEARCH` | geometric descent design lens, not a new assumption set |
| `research/path2_high_level_descent_design_space.md` | `ACTIVE_RESEARCH` | high-level descent design-space scaffold |
| `research/path2_holistic_descent_boundary_construction.md` | `ACTIVE_RESEARCH` | first-principles descent-boundary construction frame |
| `research/path2_inner_commitment_relation_refinement.md` | `ACTIVE_RESEARCH` | inner-commitment relation refinement and route probe |
| `research/path2_lift_nonemptiness_v1.md` | `ACTIVE_SUPPORT` | lift/nonemptiness decomposition supporting Path 2 proof split |
| `research/path2_meaning_v1.md` | `ACTIVE_SUPPORT` | semantic account meaning target for native solution search |
| `research/path2_obstruction_map_v1_attempt.md` | `ACTIVE_RESEARCH` | first obstruction-map attempt, not selected proof authority by itself |
| `research/path2_ocontext_wellformedness_v1.md` | `ACTIVE_SUPPORT` | `O_context` well-formedness theorem target |
| `research/path2_oglue_instantiation_v1.md` | `ACTIVE_SUPPORT` | `O_glue` instantiation support note |
| `research/path2_parallel_proof_route_synthesis.md` | `ACTIVE_RESEARCH` | synthesis of parallel proof-route probes, no final proof family selected |
| `research/path2_private_fiber_meaning_bridge_v1.md` | `ACTIVE_SUPPORT` | private-fiber meaning bridge for native proof closure |
| `research/path2_projection_completeness_bridge_v1.md` | `ACTIVE_SUPPORT` | projection-completeness bridge for native projection bundle |
| `research/path2_projection_encoding_instantiation_v1.md` | `ACTIVE_SUPPORT` | concrete projection-encoding instantiation note |
| `research/path2_projection_surfaces_v1.md` | `ACTIVE_SUPPORT` | concrete native projection-surface draft |
| `research/path2_honest_create_sampler_v1.md` | `ACTIVE_SUPPORT` | honest-create sampler and wallet/keygen spendability theorem target |
| `research/path2_live_slot_leakage_ledger.md` | `ACTIVE_SUPPORT` | direct-mode leakage ledger for prevalidated live slots |
| `research/path2_masked_trace_commitment_layout.md` | `ACTIVE_SUPPORT` | masked-trace commitment layout for proof-family work |
| `research/path2_masked_trace_leaf_encoding_fork.md` | `ACTIVE_SUPPORT` | masked-trace leaf-encoding fork note |
| `research/path2_masked_trace_range_account_delta.md` | `ACTIVE_SUPPORT` | conservative range/account delta note for masked-trace proof work |
| `research/path2_masked_trace_transform.md` | `ACTIVE_SUPPORT` | masked-trace transform support note |
| `research/path2_native_byte_state_model_v1.md` | `ACTIVE_SUPPORT` | native byte/state model for current Path 2 projections |
| `research/path2_native_candidate_gate_matrix.md` | `ACTIVE_RESEARCH` | native candidate comparison matrix, not selected proof authority by itself |
| `research/path2_native_descent_non_zk_alternatives.md` | `ACTIVE_RESEARCH` | conceptual non-ZK native descent alternatives |
| `research/path2_native_red_team_authorization_class_confusion_v1.md` | `ACTIVE_RESEARCH` | closed red-team evidence for authorization-class confusion |
| `research/path2_native_red_team_boundary_sufficiency_v1.md` | `ACTIVE_RESEARCH` | closed red-team evidence for boundary sufficiency |
| `research/path2_native_red_team_checked_core_composition_v1.md` | `ACTIVE_RESEARCH` | closed red-team evidence for checked-core composition |
| `research/path2_native_red_team_da_projection_preimage_ambiguity_v1.md` | `ACTIVE_RESEARCH` | closed red-team evidence for DA/projection-preimage ambiguity |
| `research/path2_native_red_team_false_obstruction_acceptance_v1.md` | `ACTIVE_RESEARCH` | closed red-team evidence for false-obstruction acceptance |
| `research/path2_native_red_team_no_export_leakage_v1.md` | `ACTIVE_RESEARCH` | closed red-team evidence for no-export leakage |
| `research/path2_native_red_team_qrom_domain_replay_v1.md` | `ACTIVE_RESEARCH` | closed red-team evidence for QROM/domain replay |
| `research/path2_native_rust_lean_assembly_crosscheck_v1.md` | `ACTIVE_SUPPORT` | Rust/Lean assembly cross-check for native boundary closure |
| `research/path2_private_zk_vs_transparent_gate.md` | `ACTIVE_RESEARCH` | private-ZK versus transparent proof-family comparison gate |
| `research/path2_proof_family_route_clarity.md` | `ACTIVE_SUPPORT` | proof-family route clarity note supporting the transparent route |
| `research/path2_proof_friendly_inner_commitment_sketch.md` | `ACTIVE_RESEARCH` | proof-friendly inner-commitment sketch for hiding payload work |
| `research/path2_proof_friendly_state_delta_model.md` | `ACTIVE_SUPPORT` | proof-friendly state delta byte/state model |
| `research/path2_pstate_canonicality_v1.md` | `ACTIVE_SUPPORT` | `p_state(q)` canonicality theorem target |
| `research/path2_public_view_no_export_allowlist_v1.md` | `ACTIVE_SUPPORT` | public-view no-export allowlist for active native path |
| `research/path2_role_commitment_link_equation_options.md` | `ACTIVE_RESEARCH` | role-commitment link-equation option note, not instantiated |
| `research/path2_stark_route_c1c2_sha_binding_bridge_risk.md` | `ACTIVE_SUPPORT` | risk-first analysis of the selected transparent STARK/FRI route binding bridge |
| `research/path2_theorem_mode_switch.md` | `ACTIVE_SUPPORT` | theorem-mode contract for direct and optional aggregation modes |
| `research/path2_top_proof_risk_resolution_options.md` | `ACTIVE_RESEARCH` | top proof-risk resolution options, not yet proved |
| `research/path2_toy_q_linear_creation_payload_trial.md` | `ACTIVE_RESEARCH` | executable proof-friendly payload trial, not deployable |
| `research/path2_trace_commitment_binding_audit.md` | `ACTIVE_SUPPORT` | trace-commitment binding audit for transparent proof route |
| `research/path2_transparent_commitment_api.md` | `ACTIVE_SUPPORT` | transparent commitment API theorem interface |
| `research/path2_transparent_proof_byte_model.md` | `ACTIVE_RESEARCH` | transparent proof byte model, not final proof-size authority |
| `research/path2_transparent_proof_family_gate.md` | `ACTIVE_SUPPORT` | transparent proof-family gate for masked-trace candidate |
| `research/path2_two_view_column_profile_rules.md` | `ACTIVE_SUPPORT` | two-view column-profile rules for masked-trace proof work |
| `research/path2_two_view_consistency_obligations.md` | `ACTIVE_SUPPORT` | two-view consistency obligations for masked-trace proof work |
| `research/path2_two_view_linear_leakage_probe.md` | `ACTIVE_RESEARCH` | two-view linear leakage probe evidence |
| `research/path2_two_view_trace_theorem_interface.md` | `ACTIVE_SUPPORT` | two-view trace theorem interface for native-boundary proof work |
| `research/path2_creation_time_mask_prf_leakage_lift.md` | `ACTIVE_SUPPORT` | lifts legacy projection mask-PRF leakage reasoning into Path 2 account creation |
| `research/path2_lean_rust_instantiation_bridge_checklist_v1.md` | `ACTIVE_SUPPORT` | Lean/Rust instantiation bridge checklist |
| `research/spend_fs_full_poseidon2_rehost_plan.md` | `ACTIVE_SUPPORT` | spend Fiat-Shamir Poseidon2 rehost plan |
| `research/ethereum_account_abstraction_positioning.md` | `ACTIVE_SUPPORT` | Ethereum/account-abstraction adoption positioning note |
| `formal/lean/trace_model_status.md` | `ACTIVE_SUPPORT` | current Lean trace-model status for Path 2 closure work |
| `tools/descender-recursion/PHASE1_GADGET1_CHALLENGER_SPEC.md` | `ACTIVE_SUPPORT` | recursion challenger gadget specification |
| `legal/implementation_plan.md` | `ACTIVE_RESEARCH` | legal/commercialization checklist, not cryptographic product authority |
| `research/cryptanalysis/beyond_box_VERDICT.md` | `ACTIVE_RESEARCH` | current-use beyond-box cryptanalysis evidence, not normative status |
| `research/cryptanalysis/creation_time_binding_VERDICT.md` | `ACTIVE_RESEARCH` | creation-time binding evidence for active Path 2 analysis |
| `research/cryptanalysis/creation_time_relation_validity_VERDICT.md` | `ACTIVE_RESEARCH` | creation-time relation-validity evidence for active Path 2 analysis |
| `research/cryptanalysis/creation_time_binding_soundness_characterization.md` | `ACTIVE_RESEARCH` | creation-time binding soundness characterization evidence |
| `research/cryptanalysis/creation_time_witness_binding_findings.md` | `ACTIVE_RESEARCH` | empirical creation-time witness-binding survivor search findings |
| `research/cryptanalysis/high1_malleability_fix.md` | `ACTIVE_RESEARCH` | cryptanalysis evidence for the HIGH-1 malleability fix |
| `research/cryptanalysis/high2_descent_native_hiding_exploration.md` | `ACTIVE_RESEARCH` | read-only exploration of descent-native hiding feasibility |
| `research/cryptanalysis/ws5_attack_catalog.md` | `ACTIVE_RESEARCH` | active attack catalog evidence for cryptanalysis review |
| `research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.verify.md` | `ACTIVE_RESEARCH` | parameter-search sidecar verification artifact |
| `research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2049_rows_per_group4.verify.md` | `ACTIVE_RESEARCH` | parameter-search sidecar verification artifact |
| `research/path2_proof_family_aggregation_recursion_analysis.md` | `ACTIVE_SUPPORT` | Mode B aggregation/recursion design analysis |
| `research/path2_recursion_air_composition_architecture.md` | `ACTIVE_SUPPORT` | current recursion architecture decision |
| `research/path2_recursion_circuit_concrete_design.md` | `ACTIVE_SUPPORT` | concrete as-built design note for account-creation plus spend recursion circuit planning |
| `research/path2_native_boundary_b_construction_tracker.md` | `ACTIVE_SUPPORT` | detailed native-boundary proof-design tracker, subordinate to `TRACKER.md` |
| `research/path2_native_solution_search_todo_tracker.md` | `ACTIVE_SUPPORT` | detailed native Path 2 search todo, subordinate to `TRACKER.md` |
| `research/path2_mith_full_witness_probe.md` | `ACTIVE_RESEARCH` | fallback proof-family probe, not the active selected endpoint |
| `legal/patent_licensing_commercialization_strategy.md` | `ACTIVE_RESEARCH` | legal/commercialization strategy, not cryptographic product authority |

## Cleanup Rule

When touching an old research document, add a banner if it can be mistaken for a
current product target or current tracker.
