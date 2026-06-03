# Deployable Signature Security Tracker

Scope: internal work required before claiming Descender is a deployable secure signature system. External third-party review is intentionally excluded. Items are ordered from hardest and most likely to expose a real security issue to more mechanical deployment hardening.

Status legend: `[ ]` open, `[~]` in progress, `[x]` complete.

## Critical Cryptographic Closure

- [x] Define the final deployed signature target unambiguously.
  Acceptance criteria: one profile is selected as the deployable target, with exact byte size, account-state payload, verifier entrypoint, proof format, Fiat-Shamir frame, and rejection behavior. The selected target must not mix the 254-byte full-response Lean route, 250-byte reviewed account-carried route, 252-byte Fq12 route, and 308-byte projection-carrier route without a written equivalence theorem and executable conformance tests.
  Evidence: `research/deployable_signature_target.md` pins the selected target as the fail-closed 308-byte Fq12 projection-carrier envelope; `python3 scripts/research/verify_selected_deployment_terminology.py`, `cargo test` in `tools/production-reference`, and `cargo test --lib` in `tools/parameter-search` pass.

- [x] Resolve the 284-byte vs 308-byte selected-target divergence.
  Acceptance criteria: the production-reference README, Rust constants, preflight blocker text, target specification, verifier entrypoints, and final theorem surface name the same deployed target. Current code points at the 308-byte Fq12 projection-carrier envelope, while the production-reference README still describes a 284-byte hidden-source-proof envelope.
  Evidence: `tools/production-reference/README.md` now describes 284 B as archived and 308 B as selected; the selected-deployment terminology verifier passes.

- [x] Keep the public representative forgery class blocked.
  Acceptance criteria: no final verifier may accept a public post-challenge representative reconstructed only from public key bytes, account digest, slot id, and message. Regression tests must preserve the old public-forgery probe as a negative control against any replacement verifier.
  Evidence: `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs` now has the explicit regression test `public_postchallenge_representative_forgery_remains_negative_control`. It verifies both public forgery probes use no private key/account hidden witness material, records that the old public paths are forgeable, and asserts production preflight still blocks the deployment verifier. `cargo test --bin signature_q12289_local_delta_stateful public_postchallenge_representative_forgery_remains_negative_control` passes.

- [~] Replace the fail-closed selected deployment verifier with a real accepting verifier.
  Acceptance criteria: `verify_reviewed_fq12_account_carried_deployment_verifier` or its successor accepts exactly the selected deployed proof when all cryptographic checks pass, and still rejects malformed, replayed, stale, inconsistent, and noncanonical inputs.
  Current state: the selected 308-byte/Fq12 path now has an executable state-assisted masked-projection candidate verifier, `verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier`. It decodes the 308-byte projection-carrier envelope, verifies the embedded 252-byte Fq12 checked core against the state-assisted account slot, derives and binds the canonical hidden-source statement, verifies the masked 18-residue projection-carrier tuple against the pad seed, and extracts a deterministic full packed-witness opening for the 12 checked-core residues plus six unmasked projection residues. `verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate` now packages the local soundness certificate from candidate acceptance: checked-core challenge/response syndrome, packed opening, pad-seed digest, carrier lanes, carrier check bytes, and transcript binding. `state_assisted_masked_projection_candidate_verifier_accepts_executable_tuple` covers honest acceptance, certificate extraction, and statement-mutation rejection. Production preflight no longer emits `PRODUCTION_DEPLOYMENT_BLOCKER_FINAL_VERIFIER`; the primary internal blocker is now `PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS`. Remaining work is to compose this local certificate into deployable proof semantics: global unforgeability, zero-knowledge/witness-hiding, and mask-PRF theorem integration.

- [x] Eliminate the inactive hidden-source projection survivor for the selected active-lane relation.
  Acceptance criteria: the exact infeasibility replay for the selected 308 B projection relation reports no bounded survivor under the verifier rows consumed by the accepting verifier. If this cannot be achieved with the fixed 56-byte carrier, update the selected target rather than weakening the verifier.
  Evidence: `scripts/research/hidden_source_fallback_projection_active_subspace_exact_replay.py` regenerates `hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.{json,md}` with `survivor_in_active_subspace = false`, active projection nullity `0`, and zero open replay rows. `scripts/research/generate_hidden_source_active_subspace_lean.py` regenerates `Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean`; `lake env lean Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean` passes.

- [x] Decide active-lane relation vs larger carrier for hidden-source projection.
  Acceptance criteria: either the final hidden-source relation is explicitly redefined to the twelve carrier-visible projection lanes and fresh exact replay/cryptanalysis closes for that relation, or the selected target changes to carry enough information to bind the wider hidden-source projection space. Do not accept a verifier that reasons over 64 projection variables while carrying only 12 public lane residues.
  Evidence: the selected path is the active-lane relation. The wider 64-variable relation remains rejected for the fixed 56-byte carrier because it admits inactive hidden-source coordinates not bound by carrier lanes.

- [~] Promote the active-lane relation into the deployment preflight and theorem surface.
  Acceptance criteria: production preflight, public blocker text, Lean theorem imports, Rust verifier naming, and target documentation all consume the active-lane relation as the selected hidden-source statement. No deployment-facing component may still treat the 64-variable projection model as the accepted relation.
  Progress: production preflight exposes `reviewed_fq12_active_lane_relation_certificate_present` and now also records that the state-assisted masked-projection candidate verifier is present. `Descender.lean` imports the generated active-subspace certificate and `lake env lean Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean` passes. `production_reference_gap_report.py`, `third_party_review_status_index.py`, and `verify_production_preflight_certificate.py` have been rerun and agree on the new 4-blocker preflight certificate. Remaining work is to connect the active-lane candidate relation to the final deployable theorem statement and clean up any remaining stale non-gating research summaries.

- [x] Implement pad-opening witness binding for the selected path.
  Acceptance criteria: an accepting signature binds the public pad-opening statement to the committed hidden pad seed or an explicitly modeled binding break. The verifier must check the concrete relation, not only local consistency of revealed openings. Negative tests must mutate pad seed, opening statement, observed blocks, local openings, challenge, and account commitments independently.
  Evidence: `reviewed_fq12_pad_seed_opening_digest`, `verify_reviewed_fq12_pad_seed_opening`, and `verify_reviewed_fq12_state_assisted_pad_seed_opening` bind the opened pad seed to the prechallenge state-assisted opening commitment and to the verifier-owned public pad-opening statement. The production preflight now exposes `reviewed_fq12_pad_opening_witness_binding_present` and no longer emits `PRODUCTION_DEPLOYMENT_BLOCKER_PAD_OPENING_BINDING`. `state_assisted_pad_seed_opening_binds_public_statement` mutates the seed, public statement, and committed opening and rejects each. `cargo test --lib` in `tools/parameter-search` and `cargo test` in `tools/production-reference` pass.

- [~] Implement hidden-source proof semantics.
  Acceptance criteria: the hidden-source proof verifier enforces the actual relation it is meant to prove: account witness commitment opening, pad seed opening, transcript binding, soundness relation, and zero-knowledge/witness-hiding semantics as applicable. The current callback/fail-closed placeholder is not sufficient.
  Progress: `reviewed_fq12_hidden_source_proof_acceptance_contract` now uses the `partial_executable_relation` profile and marks four executable relation obligations as implemented: statement binding, packed witness commitment opening, pad-seed opening, and transcript binding. `reviewed_fq12_masked_state18_packed_witness_opening` and `verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening` extract the full packed state from the 32-byte witness commitment under the pad seed and slot id: 12 checked-core syndrome residues, six masked projection residues, six masks, six unmasked account-projection residues, and a repacked commitment that must equal the account commitment. `verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate` composes candidate acceptance into a deterministic local soundness certificate. `hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289` now audits source evidence for seven semantic fail-closed negative controls against the selected 308-byte candidate verifier and soundness-certificate wrapper: witness digest substitution, pad seed substitution, public pad-statement mismatch, relation projection mismatch, carrier context replay, false statement acceptance, and precommitment bypass. `hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289` states the selected-path global theorem target with two implemented local hops and five open global hops. Its mask-PRF leakage obligation consumes `pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289`, which composes the simulator plus four child fail-closed contracts into one theorem target, `pre_signature_projection_mask_prf_leakage_or_named_break`.
  Path decision: the selected planning direction is now account-creation / prevalidated-slot authorization. The prior spend-only theorem target, "accepting tuple/carrier implies full hidden-source witness opening or named break," remains a negative-control baseline but is no longer the intended deployment architecture. The new theorem target is: account/slot creation validates and commits the hidden-source relation before a slot becomes live; spend-time verification proves authorization to consume that prevalidated one-use slot plus checked-core message binding, or exports a named creation-soundness, registry-binding, lifecycle, checked-core, QROM, or leakage break. See `research/account_creation_prevalidated_slot_plan.md` and `research/selected_hidden_source_path_decision.md`.

- [x] Unify the account-witness projection-opening commitment model.
  Acceptance criteria: the selected deployment has one commitment model for the account-witness opening. The current code has a direct expanded projection-opening slot commitment path and a state-assisted tuple commitment path; the deployable verifier must either prove these are equivalent under the selected relation or delete one path from the deployment surface. The final model must prevent pairing an arbitrary projection-opening frame with an unrelated account witness commitment.
  Decision: select the state-assisted opening tuple and archive the direct expanded-opening state-slot commitment path. The direct path remains as a reference predicate/negative-control source only. See `research/selected_hidden_source_path_decision.md`.
  Evidence: production preflight exposes `reviewed_fq12_direct_expanded_opening_path_archived`, and the direct expanded-opening state-slot helpers are commented as archival non-deployment helpers. The active path is now state-assisted and masked-18-residue based: the projection-opening frame is derived from the packed witness field and the candidate verifier rejects hidden-source statement rebinding. This closes model selection and direct-path ambiguity. The separate remaining blocker is the proof semantics theorem for what that selected state-assisted relation proves.

- [ ] Prove or refute the projection-carrier security claim.
  Acceptance criteria: the 56-byte carrier route has a complete semantic predicate that either accepts a valid hidden-source relation or exports a concrete break. The leakage accounting must prove that every carrier byte is either public binding/context or charged leakage. If the carrier leaks too much, the route must be abandoned or redesigned.
  Current selected construction: pack the 12 checked-core Fq12 syndrome residues plus six masked selected account-witness projection coordinates into the existing 32-byte witness field. The unmasked variant was rejected as a deployable default because prepublishing small centered projection coordinates is trivially distinguishable across live slots (`small_centered_fraction = 1.0` versus about `0.00198` for uniform rows). Option A is now the active path: `reviewed_fq12_pack_masked_witness_state18_from_parts` packs `raw_projection_i + mask_i mod q`, with masks derived from the pad seed, slot id, and `REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN`. The Fq12 state-assisted path has an 18-residue-aware commitment validator, `verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple`, and a composed candidate verifier for the 308-byte envelope.
  Evidence and remaining gate: `masked_witness_state18_unmasks_projection_with_pad_seed` confirms correct pad seed unmasking, wrong-pad rejection, slot-id mask separation, slot-bound prechallenge opening fixation, alternate-witness replay rejection, rebinding rejection, and full packed-opening extraction/repack validation. The statistical probe shows the pad-seed-masked centered model matches the uniform marginal test (`small_centered_fraction` about `0.001973` versus `0.001977` for uniform). `pre_signature_projection_mask_independence_probe.py` checks 50,000 pad seeds against the existing pad PRF projection lanes: max centered Pearson correlation is about `0.004880`, total mask-pad equality fraction is about `0.000097`, and adjacent slot six-lane mask equality is `0.0`. `projection_mask_prf_frame_is_disjoint_from_pad_prf_frame` enforces the SHA-256 frame boundary. `hidden_source_fallback_projection_carrier_lane_rank_probe_n32_q12289` rejects simple extra carrier lanes as a compact proof replacement: each added lane buys one linear constraint, does not change the checked-core kernel, and all inactive lanes still miss the crude constraint target. `pre_signature_projection_mask_prf_leakage_theorem_composition.py` now composes the real/ideal simulator, RO reduction, lifecycle same-game lift, spend-time leakage budget, and reject-equivalence contract into a final fail-closed bound with 13 named breaks, five carried assumptions, and 20 open proof/test items. Under the selected Path 2 architecture, this work feeds the account-creation verifier and prevalidation certificate rather than a spend-only extraction claim.

- [ ] Design the account-creation prevalidated-slot verifier.
  Acceptance criteria: a slot becomes live only after a creation-time verifier validates the hidden-source relation and commits the result to account state. The committed prevalidation record must bind public-key identity, slot id, verifier version, opening commitment, pad-seed digest, hidden-source statement digest, account state root, and creation-time validation transcript or certificate root.
  Current state: Path 2 is selected in `research/selected_hidden_source_path_decision.md`; `research/account_creation_prevalidated_slot_plan.md` records the planning target, state-layout candidates, theorem target, aggregation implications, non-selected paths, and immediate planning tasks. `research/account_creation_prevalidated_slot_proof_tracker.md` is now the proof-first checklist for the account-creation theorem, proof boundary ledger, and optional aggregation mode. The next implementation artifact is `account_creation_prevalidated_slot_authorization_theorem`.

- [ ] Design the spend-time prevalidated-slot authorization verifier.
  Acceptance criteria: spend-time verification accepts only when the checked-core Fq12 signature is valid for the message/public key/slot challenge frame, the named slot is live and prevalidated under current account state, the prevalidation handle binds the same identity/slot/version/opening/pad digest, and the state transition consumes the exact slot. Missing, stale, wrong-key, wrong-slot, wrong-version, wrong-root, or already-consumed prevalidation records must fail closed.
  Target byte shape: `252 B` checked-core spend body when the prevalidation authorization record is consensus-visible, plus any target-chain inclusion proof if direct state lookup is unavailable.

- [ ] Add aggregation-ready account/slot frames without requiring aggregation.
  Acceptance criteria: direct single-slot deployment uses canonical prevalidated-slot leaf bytes, a normalized slot authorization digest, and reserved direct/batch/root/consume domains. Aggregated slot admission, accumulator-backed roots, and rollup/block proofs remain optional theorem modes guarded by an inactive `optional_aggregation_soundness` assumption. Enabling aggregation must not change the checked-core spend signature frame; it may only change how the same slot authorization digest is validated.
  Current state: `research/account_creation_aggregation_readiness_contract.md` defines the canonical leaf fields, admission modes, authorization handle representations, reserved domain names, canonical batch semantics, optional aggregation assumption class, negative controls, and theorem modes.

- [ ] Close quotient witness hiding as an actual probability statement.
  Acceptance criteria: replace proposition-only sampler claims with a concrete distributional theorem or verified exhaustive/probabilistic certificate for the selected parameter set. The proof must cover canonical packing, fiber-state equality, response bounds, active pad-output exclusion, and conditioning under the actual verifier law.

- [ ] Validate that the "statistical distance zero" claim is really true for the executable sampler.
  Acceptance criteria: the executable signer’s masked-response distribution is shown identical to the modeled conditioned fiber distribution, or a nonzero distance bound is computed and propagated into security loss. Tests must catch bias from rejection sampling, canonical packing, truncation, and per-slot single-sample behavior.

- [ ] Prove extraction soundness for the concrete byte verifier, not only for abstract certificate fields.
  Acceptance criteria: an accepting pair/fork for the deployed verifier either yields the required witness/secret/representative or produces a named algebraic/SIS/hash break. The proof must consume exactly the deployed parser, challenge bytes, proof commitment, account digest, response bytes, and local openings.

- [ ] Close the algebraic binding/SIS reduction for the selected quotient map.
  Acceptance criteria: `extracts_or_break`, `noAlgebraicBindingBreak`, matrix rank/surjectivity, kernel claims, right inverse, and packing injectivity are instantiated from reproducible concrete artifacts rather than assumed fields.

- [ ] Prove challenge-action soundness for the deployed challenge action.
  Acceptance criteria: forks with same commitment and non-colliding challenge action extract correctly. Challenge-action collisions must have a quantified bound or a concrete break reduction. Tests must cover equal challenge bytes, different bytes with same action, malformed high bits, and domain mismatches.

- [ ] Close the QROM/Fiat-Shamir fork-generation ledger.
  Acceptance criteria: the measure-and-reprogram/forking loss is quantified for the exact challenge space, query budget, signature target, and stateful slot model. The ledger must feed into the final advantage bound rather than existing as an uninterpreted proposition.

- [ ] Prove Fiat-Shamir frame agreement for the executable verifier.
  Acceptance criteria: the Lean frame proposition and Rust `challenge_bytes` frame are shown equivalent byte-for-byte for the selected target. Include generated test vectors for public key, message, slot id, witness commitment, proof commitment, and all challenge bytes.

- [ ] Prove response-link bounded-kernel certificate reproducibility.
  Acceptance criteria: the response-link kernel infeasibility certificate can be regenerated from checked inputs, and the executable verifier checks the same matrix/domain/coordinate/bound tuple consumed by Lean.

- [ ] Decide whether SHA-256 is being used as random oracle, PRF, collision-resistant hash, or all three.
  Acceptance criteria: each use has a named game, domain separation, input frame, query budget, and reduction accounting. Pad expansion, commitments, challenge hashing, matrix generation, transcript binding, and proof transcript hashes must not share ambiguous assumptions.

- [ ] Close SHA-256 pad PRF assumptions for the selected signer.
  Acceptance criteria: pad-generation indistinguishability is modeled with exact adversary interface, query cap, domain separation, and leakage. If this remains an assumption, it must be a clear final security assumption consumed by the theorem, not a placeholder for missing implementation.

- [ ] Prove account-carried commitment binding for witness and randomizer commitments.
  Acceptance criteria: the account state commitment format binds the committed witness/randomizer values used by the verifier. Any hash collision or alternate opening must be explicitly modeled as a break event with budget.

- [ ] Close the public-key/keygen hardness connection.
  Acceptance criteria: the final keygen distribution and public key encoding are connected to the assumed hard problem. The current public view and quotient maps must not introduce a distinguisher or short relation outside the modeled assumptions.

- [ ] Re-run structural cryptanalysis against the final selected target.
  Acceptance criteria: direct dual, reduced-basis, q-ary L1, moment/Lee, active-subspace, response-link, and transcript-malleability probes run against the final parameters and exact deployed byte maps. Any successful distinguisher must block deployment.

## Verifier And Signer Correctness

- [ ] Build an end-to-end honest signer for the selected deployed proof.
  Acceptance criteria: given a valid secret key and unused account slot, the signer emits a signature accepted by the final verifier without test-only shortcuts or callbacks that fabricate proof objects.

- [ ] Add deterministic test vectors for full signing and verification.
  Acceptance criteria: vectors include keypair, account state, message, slot id, challenge frame, challenge bytes, responses, openings, proof bytes, and post-verification account state. Lean/Rust conformance checks consume the same vectors.

- [ ] Make malformed-signature rejection complete.
  Acceptance criteria: every byte field has tests for length errors, noncanonical q values, high challenge bits, response bound violations, invalid slot id, reused slot, stale account state, wrong proof commitment, wrong witness commitment, wrong public key, wrong message, and wrong domain.

- [ ] Prove no transcript malleability under account-carried state.
  Acceptance criteria: modifying any signature byte, account slot byte, public key byte, message byte, or proof byte either preserves an identical verified statement or rejects. There must be no alternate accepted transcript for the same account slot unless it is covered by the formal relation.

- [ ] Verify active block coverage and extraction conditions under the real challenge distribution.
  Acceptance criteria: the probability of insufficient active coverage is calculated for the exact repetition count and challenge encoding, and the extractor theorem consumes that bound.

- [ ] Ensure rejection sampling cannot leak through abort patterns.
  Acceptance criteria: signer aborts, retry counts, rejection loops, and callback errors are either constant-shape or excluded from the adversary interface by an enforceable API boundary.

- [ ] Remove or isolate test-only unchecked constructors from deployment-facing paths.
  Acceptance criteria: unchecked constructors are crate-private or test-only where possible. Public unchecked helpers must be impossible to call from deployment wrappers.

## Constant-Time And Side-Channel Closure

- [ ] Constant-time audit the verifier.
  Acceptance criteria: secret-independent control flow and memory access are documented for all verifier code. Public rejection paths may vary only on public data. Timing tests should include malformed signatures with equalized public rejection classes where relevant.

- [ ] Constant-time audit the signer and sampler.
  Acceptance criteria: secret-dependent rejection loops, table lookups, hash inputs, branch behavior, and memory access are removed or bounded/equalized. Any unavoidable leakage is included in the security model.

- [ ] Constant-time audit keygen and account-state generation.
  Acceptance criteria: keygen, commitment generation, randomizer creation, pad seed derivation, and account-slot creation do not leak secret material through timing, allocation shape, panic messages, or serialized state.

- [ ] Remove secret-bearing errors and debug output from deployment paths.
  Acceptance criteria: sanitized wrappers are mandatory for deployment APIs, and tests assert that secret sentinels never appear in errors, logs, serialized account state, rejection reports, or preflight artifacts.

- [ ] Check panic freedom for all deployment-facing APIs.
  Acceptance criteria: malformed external inputs return typed errors, not panics. `unwrap`/`expect` are allowed only after locally proven length/canonicality checks or inside tests.

## Stateful Account And Deployment Plumbing

- [~] Make account lifecycle atomic for real storage.
  Acceptance criteria: slot reservation, signing, verification, consumption, ledger recording, and persistence are atomic under crashes and concurrent writers. Recovery must burn ambiguous reserved slots and prevent stale wallet replay.
  Progress: lifecycle is now treated as part of the security boundary for the state-assisted Fq12 path. The persisted account slot record carries public `pad_seed_digest`; construction, checked record export, snapshot restore, and slot opening reject zero, duplicate, or mismatching digests. Selected-projection spent records also persist the digest, and target-chain commit validation rejects zero, mismatching, or duplicate selected-projection pad-seed digests. Existing lifecycle tests cover reservation, consume, stale-wallet replay, restore-anchor rejection, reorg burn, file-store locking, selected-projection spend recording, failed atomic commits, transactional KV/file-backed KV persistence, SQLite-backed canonical-node conformance, and duplicate-digest target-chain commit rejection. Remaining work is deployment-database review under the actual target chain and final theorem integration.

- [ ] Finalize consensus/storage semantics for account state.
  Acceptance criteria: the deployment model specifies what lives on-chain or in consensus state, what is local wallet state, and what restore anchors are trusted to protect. The verifier must not rely on unverifiable local state.

- [ ] Prove replay resistance across forks/reorgs/backups.
  Acceptance criteria: a signature accepted once cannot be accepted again against any valid successor state. Backup restore, stale wallet, chain reorg, and concurrent signing scenarios must have executable tests.

- [ ] Define key/account rotation and exhaustion behavior.
  Acceptance criteria: account slots cannot silently wrap, be reused, or be regenerated incompatibly. Exhausted accounts fail closed and expose safe migration behavior.

- [ ] Lock down domain separation registry.
  Acceptance criteria: every hash domain string is listed once, has a single purpose, includes profile/version where needed, and has tests preventing accidental cross-use or truncation.

- [ ] Stabilize serialization formats.
  Acceptance criteria: public key, account state, signature, proof, preflight certificate, spent-slot ledger, and restore-anchor formats are versioned, canonical, and reject duplicate or ambiguous encodings.

## Formal Integration And Theorem Surface

- [ ] State the final deployable security theorem.
  Acceptance criteria: a single theorem names the deployed scheme, adversary class, assumptions, advantage bound, stateful replay model, and accepted signature relation. It must not be only a deterministic-core or partial verifier theorem.

- [ ] Eliminate placeholder propositions from the final theorem path.
  Acceptance criteria: remaining propositions are either proven in Lean, generated from checked certificates, or explicitly listed as final cryptographic assumptions. No `*_holds` field should hide an implementation obligation.

- [ ] Connect generated certificates to reproducible generators.
  Acceptance criteria: every generated Lean certificate includes source inputs, generator command, hash of generator, hash of output, and Rust/Lean agreement tests.

- [ ] Add Lean/Rust cross-checks for constants.
  Acceptance criteria: q, dimensions, byte lengths, field bit widths, domain tags, challenge masks, repetition counts, and selected coordinates match across Lean and Rust automatically.

- [ ] Add final theorem regression tests.
  Acceptance criteria: `lake build`, Rust tests, certificate regeneration checks, and conformance vectors run in one command. CI fails if the deployed theorem no longer imports or if executable constants diverge.

## Release Gate

- [ ] Production preflight has zero internal blockers.
  Acceptance criteria: the preflight report is generated by code, not hand-edited, and reports deployable only when selected verifier, hidden-source semantics, constant-time closure, storage semantics, and theorem integration are complete. External review remains outside this tracker by request.

- [ ] Mark unsafe research APIs as non-deployment.
  Acceptance criteria: crates, modules, and docs distinguish research probes from deployment APIs. The deployment crate exposes only the finalized signer, verifier, account lifecycle, serialization, and preflight interfaces.

- [ ] Produce a final internal security packet.
  Acceptance criteria: contains the final theorem statement, assumptions, parameter set, byte formats, test vectors, cryptanalysis outputs, side-channel report, lifecycle model, and known limitations. This is internal evidence, not an external review substitute.
