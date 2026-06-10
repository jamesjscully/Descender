# Cryptanalysis-First Readiness — Master Tracker

Created: 2026-06-09. Living master; supersedes no existing doc but **indexes and
consolidates** them (see §7). This is the single instrument for measuring
progress toward the immediate goal.

> **Tooling architecture (2026-06-09):** the project adopted a multi-tool formal
> stack — Lean+Mathlib (math + reduction skeleton), EasyCrypt (security
> reduction), Jasmin (deployable CT implementation), hax (Rust-reference bridge),
> KAT (cross-tool conformance). The spine + seams + phased roadmap live in
> `formal/README.md` and `formal/VERIFICATION_PLAN.md`, with the canonical spec
> in `formal/spec/`. The cryptanalysis workstreams below are the **math/security
> content**; the formal plan governs **which tool proves what**. Immediate next:
> Phase-0 KAT corpus + the EC-SPIKE and HAX-SPIKE decision gates.

## 0. Mission and definition of done

Goal: a post-quantum, blockchain-efficient signature (target: aggregation-capable)
whose security has been cryptanalyzed internally as hard as a competent third
party would, **before** any external submission.

Recalibrated bar (the honest one — see §2): the destination of this phase is
**not** "100% confident the scheme is secure." That is epistemically
unreachable by self-cryptanalysis: a proof reduces forgery to assumptions and
says nothing about whether the assumptions hold, and a designer can only attack
the weaknesses they can imagine. The achievable — and required — internal bar is:

- **DoD-1 (crisp):** every hardness assumption the active proof bottoms out in is
  stated as a standalone, machine-checked Lean `Prop`, with an informal
  one-sentence statement, concrete parameters, and a verified classification.
  Zero unclassified premises.
- **DoD-2 (standard-related):** every novel assumption is either reduced to a
  named, studied problem (Module-SIS / Ring-LWE / SVP-family) in Lean, **or**
  explicitly flagged as load-bearing-and-novel with the residual risk localized.
- **DoD-3 (no known attack):** a verified concrete-security estimator shows the
  cheapest *known* attack at the chosen parameters costs ≥ 2^128 (quantum), and
  the empirical small-scale cryptanalysis has found no exploitable structure.
- **DoD-4 (documented):** a cryptanalysis dossier records every attack class
  tried, its cost, and why it fails — the artifact that makes eventual external
  review fast and conclusive.

DoD-1..4 are **necessary, not sufficient.** Surviving time-under-attack by
hostile outsiders is irreducible and is the *next* phase, not this one. Reaching
"all internal gates green" is a different destination from "deployable"; do not
let the former be mistaken for the latter.

## 1. Risk posture (agreed)

- The existential risk lives in the protocol/assumption design, not the
  implementation. The proof's role is to **map forgery to the assumptions** —
  i.e., to tell us exactly what to cryptanalyze.
- Proof and implementation may co-evolve; we are not required to prove the
  theorem independently of the implementation. We will formalize the
  *assumption* and the *concrete-security estimate*, not only the reduction.
- A 260-byte PQ signature at 128 bits would beat Falcon; results that good are
  exactly where hidden structure tends to bite. Two prior paths already died to
  our own cryptanalysis (compact six-replay; 2^-4.55 compact path). Internal
  cryptanalysis killing this scheme is **success**, not failure.

## 2. Process change (new — this is the point of this tracker)

| # | Old behavior | New rule | Why |
|---|---|---|---|
| P1 | "Expose AP2-N003 …" micro-commits surfacing projections of open obligations | **Stop.** Proof effort goes to the *real* reduction (AP2-N003 item 1 = a concrete priced numeric certificate with real per-event bounds) and to the assumption work below. | Marginal value of "Expose" commits is ~0; they optimize internal text-agreement, not security. |
| P2 | Confidence implied by green Rust source-inspection gates | The **deliverable is the dossier (§0 DoD-4)**, not green gates. Gates attest text co-presence; only the elaborator + cryptanalysis attest substance. | The 2026-06-02→06-09 broken-build window proved gates can be green while the tree doesn't even compile. |
| P3 | Assumption statements implicit inside large certificate records | Every assumption must be a standalone Lean `Prop`, **legible to a hostile outsider**, and tied to a named problem where possible. | Guards against the closed-loop trap: impl→assumption→proof→what-we-cryptanalyze can be internally consistent and externally false. |
| P4 | Cryptanalysis attacks the model the proof already guards | Cryptanalysis must attack the **assumptions** by means *outside* the proof's model (lattice reduction, structure, statistics), at the small parameters where we have empirical reach. | A valid proof over an easy assumption is valid and useless. |
| P5 | No compile/axiom enforcement | CI enforces `lake build`, axiom/sorry audit, and `cargo check --all-targets` on every push (landed — §6). | A non-compiling tree can no longer carry a "compiles" verdict. |

## 3. Workstreams and milestones

Status legend: `[ ]` open · `[~]` in progress · `[x]` done · `[!]` blocked/needs decision.

### WS1 — Assumption ledger: enumerate, formalize, classify (→ DoD-1)

Objective: one authoritative, machine-cross-checked ledger of every premise the
active reduction bottoms out in. Consolidates the existing ledgers (§7).

Provisional inventory (from 2026-06-09 recon — **classifications are CLAIMED and
must be verified in WS1-c**; do not trust them yet):

| Assumption (Lean) | File | Claimed class | Notes / skepticism |
|---|---|---|---|
| `P_TND_Search` / `ProjectedCoboundaryFamily` | `Descender/PTND.lean` | named? | "Projected Target Nearest Distance" may be a project coinage, **not** a literature-standard name. Verify against published problems before claiming "studied". |
| `QaryPrimalLatticePromise` / `ShortQaryPrimalVector` | `Descender/QLatticeHardness.lean` | SIS, nonstandard params | Standard SIS core but L1/centered metric and small dims are nonstandard. |
| `QaryDualLatticePromise` / `ShortQaryDualRelation` | `Descender/QLatticeHardness.lean` | dual-LWE, nonstandard | centered-norm + amplitude framing is deployment-specific. |
| `CenteredNoiseAntiConcentrated` | `Descender/QLatticeHardness.lean` | LWE-style, nonstandard | finite-support dual-oracle anti-concentration. |
| SIS seed/full-response opening | `Descender/LocalDeltaByteCommitment.lean` | SIS, nonstandard | n=18 seed / n=23,m=12 response, bounded norms, bit-packed. |
| `Action.badActionCollision` / `Fq12ActionCollisionBudgetPriced` | `LocalDeltaByteCommitment.lean`, `Path2HashBudget.lean` | **NOVEL** | the load-bearing new algebraic assumption; primary WS2/WS4 target. |
| `ReviewedFq12…PinsNativeRoleRoot` / `…PinsFullOpeningRoles` | `Descender/Path2RoleRootBinding.lean` | **NOVEL** binding-gap | custom role-root binding; assert two distinct valid certs ⇒ digest collision. |
| `ActiveLaneNullityZero` / `ActiveSubspaceCertificate` | `Descender/HiddenSourceActiveSubspace.lean` | structural | finite linear-independence claim; appears `decide`-discharged — verify it is, and that it is the *deployed* lane set. |
| 9 named break events | `Descender/Path2FullSpendGame.lean:62-73` | assumption gaps | QROM/hash, domain-sep, keygen leakage, replay/rollback, etc. — each is an assumption. |
| 14 boundary break events | `Descender/Path2NativeBoundary.lean:410-426` | assumption gaps | overlaps the 9; dedupe in ledger. |

Milestones (authoritative ledger: `research/cryptanalysis/assumption_ledger_authoritative.md`):
- [~] **WS1-a** Load-bearing assumptions read from source + classified (done in the ledger; remaining: emit any not-yet-standalone premises as `Prop`s and cross-check vs `audit/lean_theorem_statements.txt`).
- [ ] **WS1-b** Pin exact `file:line` for B3 (SIS openings) and dedupe the 9+14 break-event lists.
- [ ] **WS1-c** Literature check: confirmed A1/A2 (`P_TND`/"coboundary") are **project coinages for q-ary BDD/LWE**, no standard problem mis-named; settle B1/B2 L1→L2 metric mapping.
- [x] **WS1-d** **DEFECT (C3) RESOLVED 2026-06-09:** traced — the vacuous `ActiveLaneNullityZero` had zero Lean consumers and no Rust gate inspected it (not a live soundness hole); the real bounded-survivor content was already in `ResponseLinkExactCertificate.excludesBoundedNonzeroKernel`. Replaced the tautology with the non-vacuous `ActiveLaneReadoutDetects` (false if any coefficient ≡0 mod q), proven via field cancellation over `ZMod 12289` with a new `Fact (Nat.Prime 12289)` instance; non-vacuity witness `coefficientsNonzeroModQ` by `native_decide`. Updated base + generated module + generator script; `lake build` (3358) and axiom audit green.
- [ ] **WS1-e** 0 unclassified premises; ledger signed off (DoD-1).

Key WS1 findings (2026-06-09): the lattice layer (A1/A2/B1/B2/B3) has the right
*shape* (q-ary BDD/LWE, SIS primal/dual) but everything is an **assumed promise**
— no reduction to Module-SIS/Ring-LWE is proven, and the metric is L1/centered-L∞
not the estimator's L2. The novel risk concentrates in **C1 (Fq12 action
collision)**. **C3 (vacuous nullity claim) is RESOLVED** — re-formalized as a
non-vacuous, proven readout-detection property; it was not a live soundness hole
(no consumers).

### WS2 — Reduce novel assumptions to standard ones, or fail loudly (→ DoD-2)

Objective: for each NOVEL assumption, prove `standard-problem → novel-problem`
in Lean (novel is ≥ as hard as a studied problem), collapsing risk onto studied
ground. **Inability to reduce is the signal that the novelty is load-bearing.**

Milestones (per the ledger's WS2 columns):
- [ ] **WS2-pre** A1 instantiation: locate where `eval` is instantiated and prove it is module-linear with standard noise, making the P_TND template ≡ Module-LWE. *Prerequisite for A1/A2 meaning anything.*
- [ ] **WS2-a** `Fq12ActionCollision` (C1): (i) prove the checkable facts (irreducibility / Rabin / gcd-degree) by `decide` instead of assuming them; (ii) locate the concrete `action` map; (iii) decide whether collision is a generic birthday bound (→ WS3 prices it) or algebraic (→ WS4-d attacks it). Exit: Lean reduction or written irreducible-novelty note.
- [ ] **WS2-b** Role-root binding (C2): reduce to collision-resistance of the underlying digest (likely lowers C2 NOVEL→STD), or flag standalone.
- [ ] **WS2-c** B1/B2/B3: map L1/centered metric → L2 and relate to Module-SIS/Ring-LWE at deployed params (tractable; these are genuine SIS/LWE).
- [ ] **WS2-d** Summary map: each assumption tagged `reduced-to-standard` | `standalone-novel-flagged`. Exit: DoD-2.

### WS3 — Verified concrete-security estimator (→ DoD-3, parameter justification)

Objective: machine-checked cost of *known* attacks as a function of parameters.
Output: "cheapest known attack ≥ 2^128 quantum at the deployed parameters."

Milestones:
- [ ] **WS3-a** Model primal/dual lattice attacks (BKZ blocksize → core-SVP cost) over the deployed q-ary/SIS instances, with verified arithmetic.
- [ ] **WS3-b** Model combinatorial / meet-in-the-middle / hybrid attacks and the structure-specific algebraic attacks on the Fq12 action.
- [ ] **WS3-c** Tie the estimator parameters to the actual deployed constants (no divergence between estimator and `production_reference` constants — a CI check).
- [ ] **WS3-d** Stated caveat made unmissable: estimator bounds **modeled** attacks only. Exit: verified ≥ 2^128 margin + caveat (DoD-3 part 1).

### WS4 — Empirical structural cryptanalysis at small scale (→ DoD-3)

Objective: exploit n=32, q=12289 smallness offensively — the same smallness that
enables our verification enables direct probing for hidden structure.

Milestones:
- [ ] **WS4-a** Compute lattice invariants / hunt for unexpected sublattices in the deployed instances; specifically interrogate the **hand-searched bounded-kernel response-link** (searched artifacts hide structure).
- [ ] **WS4-b** Statistical leakage tests on signature/transcript distributions (bias, correlation, the `3×2` repeated local-block structure called out in the existing plan).
- [ ] **WS4-c** Full break of toy parameters; fit the cost curve and check it extrapolates above target at deployed size.
- [ ] **WS4-d** Direct structural attack on the Fq12 action collision floor (does the claimed 163-bit floor survive a structure-aware search?).
- [ ] **WS4-e** No exploitable structure found, or finding documented and design revised. Exit: DoD-3 part 2.

### WS5 — Adversarial red-team attack catalog = the dossier (→ DoD-4)

Objective: a living catalog; every attack class with status `tried / cost /
why-it-fails`. This *is* the third-party hand-off artifact. Builds on the
existing red-team notes and cryptanalysis plan (§7).

Milestones:
- [ ] **WS5-a** Catalog skeleton with every class: lattice reduction, hybrid, combinatorial, algebraic/structure, statistical/leakage, multi-target/QROM, malleability/noncanonical-encoding, replay/rollback/state-reuse, and (when WS6 lands) rogue-key/related-key.
- [ ] **WS5-b** Each class linked to its WS3/WS4 evidence and to negative-control regression tests.
- [ ] **WS5-c** Dossier reviewed for completeness ("what modality did we *not* run?"). Exit: DoD-4.

### WS6 — Aggregation: decide the model now, fold into proof (→ enables the aggregation goal)

Objective (per D1 = deferred): aggregation is **out of scope for v1**. WS6 is demoted
to a standing guardrail: keep the base scheme aggregation-capable-later without building
it now.

Milestones:
- [x] **WS6-a** D1 resolved: defer aggregation to v2; single-signature v1.
- [ ] **WS6-b** (guardrail) Record the "don't foreclose aggregation" constraints — canonical/deterministic verification, low-complexity algebraic relation, re-derivable randomness — as a checklist applied when the base verification equation changes. Low effort, no active design work.
- [ ] **WS6-c** (v2, parked) When/if aggregation is pursued: resolve scaling-vs-custody, then design the outer succinct-proof layer (scaling) or evaluate native lattice multi-sig (custody) as a separable component.

### WS7 — The real reduction (AP2-N003 item 1) — the proof half that gates the rest

Objective: replace the fail-closed / uninhabited certificate with a concrete
`PricedAcceptedSpendNumericCertificate` carrying *real* per-event probability
bounds. This is where the proof commits to which break events are **proven** vs
**named assumptions** — the most important sorting in the project, and the input
to WS1/WS5.

Milestones:
- [ ] **WS7-a** Construct the concrete trace distribution + accepted-spend event model (no longer uninhabited).
- [ ] **WS7-b** Per-event bounds: for each of the 9 breaks, either prove the bound (forking lemma vs the 182-bit challenge space; collision floors) or record it as a named assumption in WS1.
- [ ] **WS7-c** Final budget sum ≤ 2^-128 via `norm_num` over fixed rationals; flip the evidence bits + retire the impossibility theorems (synchronized Lean+Rust).
- [ ] **WS7-d** Replace `ConcreteSpendVerificationAccepts` (currently definitionally the conclusion) with a non-tautological acceptor so `concreteFullSignatureExecutionSource_instantiation_v1` is a real reduction.

## 4. Changes already landed (2026-06-09)

- [x] **Lean build fixed**: `Descender/CommittedSigma.lean` byte-frontier moved from the stale 181-byte base to 187 after the AP2-004 challenge widening; `lake build` green (3358 jobs).
- [x] **Rust build fixed**: `signature_q12289_local_delta_stateful` widened from 21- to the shared 27-byte challenge; constants now sourced from `production_reference`; stability vectors regenerated; `cargo check --all-targets` green; 26/26 bin tests pass.
- [x] **CI**: `.github/workflows/ci.yml` runs `lake build`, the semantic theorem audit, `cargo check --all-targets`, and the q12289 bin tests on every push. `scripts/check_build.sh` mirrors it locally.
- [x] **Semantic audit**: `scripts/audit_theorems.lean` fails on disallowed axioms / `sorry` in the whole Descender closure and dumps elaborated gate-endpoint statements to `audit/lean_theorem_statements.txt` (CI-diffed). Currently: 0 disallowed axioms, 208 native_decide trust instances reported.
- [x] **Tracker corrections**: `audit/active_path_signature_tracker.md` now carries the compile-claim correction and the closure-language correction (AP2-001/002/007 "Closed" = conditional plumbing, not unconditional).

### 4b. Phase-0/Phase-1 increment (2026-06-09, parallel-stream round)

Three adversarially-verified streams landed (all left UNSTAGED per the no-commit
rule; `.github/` + `formal/` were UNTRACKED at session end — maintainer must
`git add` to land CI/KAT):

- **KAT corpus v0 (Phase-0, DONE).** `formal/kat/kat_v0.json` — 3 positive
  round-trips + 8 negative vectors, 7 distinct reject reasons, all generated by
  the REAL `production_keygen→production_sign→production_verify` pipeline (not
  hand-typed; negatives are genuine single-byte mutations of a real signature).
  `--kat-check` reproduces byte-for-byte (exit 1 + first-divergence diff on drift),
  2 `#[test]`s pass (28/28 bin tests green), wired into `.github/workflows/ci.yml`.
  **Honest caveats:** anchors the Rust reference to ITSELF only (no second impl
  yet); pins the CURRENT deployable linear-sigma path that still wraps the LEGACY
  proof — a byte anchor, NOT a soundness claim — and an adapted public
  conditioned-fiber sigma forgery still verifies on that path, so the corpus must
  be regenerated when it is hardened.
- **EC-SPIKE + HAX-SPIKE (Phase-1, PREPARED, BLOCKED ON TOOL INSTALL — NOT done).**
  Paper-specs only, honestly labeled not-machine-checked. EC: `formal/easycrypt/
  spike_union_bound.ec` mirrors the proven Lean union-bound theorems line-for-line
  (correspondence table verified — every named Lean theorem is real and sorry-free),
  OB-1..OB-4 remain `admit`/axiom; EasyCrypt + opam ABSENT. HAX: target
  `reviewed_fq12_account_carried_expected_response_syndrome`
  (`production_reference.rs:37008`) + obligation written; `hax` ABSENT (`cargo hax`
  → no such command). Note `fstar.exe` (2026.03.24) + `z3` ARE present, so HAX is
  gated only on the hax frontend. **No go/no-go verdict issued — gates NOT green.**
- **Lean trace-G increment (WS7-a partial).** `Descender/Path2TraceModel.lean`: a
  faithful concrete `AcceptedSpendNumericGame` whose 3 events are `decide` of the
  REAL concrete verifier predicates, soundness field DERIVED from the proven
  structural theorem (not assumed), `traceDist`/`observe` left as INPUTS (cannot
  vacuously inhabit). `lake build` green; `#print axioms` on the 4 key theorems =
  `[propext, Classical.choice, Quot.sound]` only. 2 named breaks proven prob 0 via
  real never-proofs. **STILL OPEN (multi-week / EC-blocked):** accepted-spend SIS
  bound (≤2⁻¹²⁹) + 7 cryptographic break bounds (≤2⁻¹³³) are explicit UNPROVEN
  hypotheses; `traceDist`/`observe` not yet a real production sampler. This is the
  WS7-a "no-longer-uninhabited" step, NOT WS7-b/c/d.

## 5. Decisions

### D1 (WS6-a) — RESOLVED: defer aggregation to v2; single-signature v1

Decision (2026-06-09): ship a **single-signature v1 with no aggregation**, revisit
aggregation as v2. Rationale: keeps the base hardness assumption and the proof as simple
as possible while we are still closing and cryptanalyzing them; avoids stacking
aggregation novelty (pairing-free PQ aggregation is genuinely hard — BLS's pairing trick
is quantum-broken, lattice sigs don't natively aggregate, native multi-sig has a delicate
proof history). **Standing guardrail only:** keep the base design *aggregation-capable
later* by not foreclosing it — canonical/deterministic verification, a low-complexity
algebraic verification relation (the Fq12 equation is a promising start), and
re-derivable randomness. WS6 is therefore demoted to a guardrail check, not an active
build. If v2 aggregation is pursued, resolve the scaling (compress independent sigs, via
an outer succinct-proof layer) vs custody (M-of-N native multi-sig) fork then.

### D2 (WS7-b) — RESOLVED: conservative side of industry standard

Concrete targets (fold into DoD-3 and WS7):
- **SUF-CMA** (strong unforgeability — blocks malleability), not just EUF-CMA.
- **QROM** Fiat–Shamir proof, not just classical ROM.
- Reduce everything reducible to **Module-SIS / Ring-LWE at standard parameters**;
  minimize surviving novel assumptions (each is a conservatism cost).
- **Margin beyond core-SVP**: discount the BKZ cost model and keep margin (≥ the 53-bit
  slack as a floor); estimators only bound known attacks.
- Security level: **Cat 1 with documented margin; evaluate Cat 3 feasibility** (raise if
  the size budget tolerates it).
- **Stateful-scheme conservatism**: treat slot reuse / rollback as first-class
  catastrophic events — **prove** the replay/rollback breaks (WS7-b), do not assume them
  away; pair with strong operational state guards.

### D3 — RESOLVED: staging ladder (affects sequencing, not the cryptography)

1. **Testnet** (valueless): builds + functions + no obvious break.
2. **Value-capped mainnet** (bounded funds at risk): internal DoD-1..4 complete **and**
   external review started.
3. **Uncapped mainnet**: requires external cryptanalysis with real time-under-attack —
   **never reachable on self-review alone** (DoD is necessary, not sufficient).

Internal DoD gets us to value-capped; uncapped is the external phase.

## 6. Cleanup ledger

Safe-now (non-gated): keep `audit/lean_theorem_statements.txt` committed and current (CI diffs it).

Deferred (gated files — require **synchronized Lean+Rust marker edits**, do not
touch piecemeal): duplicated `evidenceReportBound`/`evidenceReportBoundProof`
fields; dead `let _h… :=` bindings in `acceptedSpendForgeryProb_le_bound` and
`toNativeFullSignatureSecurityBoundary`; the `nonTautologicalExecutionSource := true`
report-bit naming. Clean these opportunistically when the corresponding Rust
marker gate is next touched (tracked also in `active_path_signature_tracker.md`).

## 7. Consolidation map (existing docs → workstream)

These already exist and are **inputs**, not to be duplicated:

- `research/path2_residual_assumption_ledger_v1.md`, `research/path2_hash_qrom_assumption_ledger.md`, `research/cryptanalysis/review_readiness_20260519/{assumption_ledger,assumption_review,dependency_audit}.md` → **WS1**.
- `research/path2_descent_sis_hardness_sanity.md`, `path2_descent_sis_feasibility.*`, `path2_descent_sis_lattice_sigma_toy_model.*` → **WS2/WS3**.
- `research/cryptanalysis/review_readiness_20260519/cryptanalysis_plan.md`, `research/path2_core_algorithm_red_team_v1.md`, `audit/active_path_signature_independent_red_team_20260602.md`, `research/cryptanalysis/signature_readiness_20260516/*` → **WS5** (fold into the catalog).
- `research/account_creation_aggregation_readiness_contract.md` → **WS6**.
- `research/deployable_signature_security_tracker.md` → productization (next phase); keep, do not merge.

Action: WS1-a and WS5-a should begin by *absorbing* the above, reconciling
overlaps/contradictions, and superseding them with single authoritative
documents linked from here.

## 8. Scorecard (update each working session)

| DoD | Metric | Now (2026-06-09, end of session) |
|---|---|---|
| Build/CI health | green on push | ✅ green (3358 jobs; axiom audit clean) |
| DoD-1 | premises classified | ✅ **met** — 18 distinct break events + 9 assumption classes ledgered; WS1-b/c/d/e done; **no irreducibly-novel assumption remains** |
| DoD-2 | novel assumptions reduced-or-flagged | ✅ **met (classification)** — C1→QROM hash CR, C2→digest CR, C3 resolved, B1/B2/B3 standard SIS/LWE family. Residual is quantitative + Lean-debt, not new hardness |
| DoD-3 | verified known-attack margin ≥ 2^128 | ⚠️ **partial** — exact numbers verified (action floor 2^163, per-fork extractor term ~2^-162, FS 162-bit, box 96<114). Two earlier "gaps" RESOLVED: C1 imposes no 2^-128 obstruction (per-fork, not multi-target); keygen SIS is intentionally LLL-weak → exact-enumeration regime (certified), no asymptotic margin to extract. Genuinely open: WS7 reduction wiring + beyond-box hardness (`open_external_estimator`, EXTERNAL) |
| DoD-3 | empirical structure probes | ⚠️ **partial** — algebraic-shortcut class ruled out, field verified irreducible, response box exhaustively clean; statistical-leakage + keygen-SIS reduction probes OPEN (`ws4_empirical_probes.md` OPEN) |
| DoD-4 | dossier attack classes documented | ✅ **met** — `ws5_attack_catalog.md` consolidates all sources, 12 classes with status + completeness critic |
| WS7 | concrete priced certificate inhabited | ⚠️ **closure skeleton PROVEN + concrete game `G` now FED in** (`Path2AcceptedSpendClosure.lean` 5 sorry-free theorems; `Path2TraceModel.lean` 2026-06-09: faithful concrete `AcceptedSpendNumericGame`, events=`decide` of real verifier, soundness DERIVED from structural theorem, `traceDist`/`observe` as inputs, 2 breaks proven =0; `#print axioms`=propext/Classical.choice/Quot.sound). This is WS7-a. **STILL OPEN (multi-week/EC-blocked):** accepted-spend SIS bound ≤2⁻¹²⁹ + 7 cryptographic break bounds ≤2⁻¹³³ are UNPROVEN hypotheses (WS7-b); real execution-distribution sampler not instantiated. No longer uninhabited, but NOT a closed reduction |

Workstream docs: `assumption_ledger_authoritative.md` (WS1/WS2),
`ws3_concrete_security_findings.md` + `scripts/research/cryptanalysis_concrete_security_estimate.py` (WS3),
`ws4_empirical_probes.md` + `scripts/research/verify_fq12_irreducibility.py` (WS4),
`ws5_attack_catalog.md` (WS5), `ws7_reduction_scoping.md` (WS7).

**Honest bottom line:** the foundational/analysis program (WS1, WS2, WS5, and
the verified WS3/WS4 sub-results) is complete and turned the load-bearing novel
risk (C1) into a standard QROM hash-collision assumption. The remaining work is
genuinely security-critical and NOT done: closing the composed numeric bound
(the action term is 2^-99 < the 2^-128 target at T=2^32), extracting the
keygen-SIS asymptotic margin, running the statistical-leakage battery, and the
maintainer-led concrete reduction. These are tracked as open follow-on tasks —
not marked complete, because security is not yet established.

## 9. Formal-toolchain phase milestone (2026-06-09, later session)

Drove the `formal/` multi-tool plan (`formal/VERIFICATION_PLAN.md`) through
**Phase 0 + all of Phase 1**, installing the real toolchains in-environment and
converting both decision gates from paper-spikes to machine-checked verdicts:

- **Phase 0 — KAT v0:** DONE. `formal/kat/kat_v0.json` (3 positive + 8 negative)
  generated from the real reference via `--kat-check`, wired to CI. *Caveat:* pins
  the 260-B legacy linear-sigma path, NOT the 314-B spec target — byte anchor only,
  regen required on path-hardening.
- **Phase 1 EC-SPIKE → GO.** Installed EasyCrypt 2026.05 + Z3 4.16.0 (nix).
  `formal/easycrypt/spike_union_bound.ec` typechecks **exit 0, zero `admit.`** —
  all 5 lemmas `qed` (union-bound assembly + Pr[never]=0 `byphoare` + concrete
  2⁻¹²⁸ budget, Z3-discharged), faithfully mirroring the proven Lean closure
  theorems. Cryptographic seam (OB-1) is a named axiom on both sides. **Decision:
  reduction/numeric-closure spine → EasyCrypt; Lean keeps field/lattice math.**
  `formal/easycrypt/SPIKE_VERDICT.md`.
- **Phase 1 HAX-SPIKE → GO (conditional).** Installed `cargo-hax` 0.3.7 (nix);
  `fstar.exe` 2026.03.24 already present. Extracted the REAL Fq12 verifier
  (`reviewed_fq12_account_carried_expected_response_syndrome`) to a 509-line
  `formal/hax/extracted/Spike_core.fst` — no Rust construct defeated the frontend;
  blocks only on one F* library combinator (`while_loop_cf`) + a `rusqlite`
  toolchain skew (isolated via `formal/hax/spike_core/`). No F* lemma proved yet.
  **Decision: adopt hax to retire the marker gates; keep KAT + marker gate until
  the F* lemmas discharge.** `formal/hax/SPIKE_VERDICT.md`.
- **Phase 1 JASMIN-SCOPE → DONE.** `formal/jasmin/JASMIN_SCOPE.md` — ~7 person-weeks
  for the CT verifier core, gated behind the first real EC break-event reduction.

**What this milestone did and did NOT do:** it resolved the *tooling* questions
(which tool owns which layer) with real runs, and landed the cross-tool byte
anchor. It did NOT prove any security bound — the per-event SIS/QROM reductions
(WS7-b, now EC-targeted) and spec §6 path-hardening remain the genuine multi-week
work, and Phase 3 external cryptanalysis stays irreducible. Tooling effort
optimizes formalization cost + credibility, never security.

**Phase-2 first increment (same session):** the EC-SPIKE GO follow-up landed —
`formal/easycrypt/break_slot_replay_or_rollback.ec` (typechecks exit 0; verified
ZERO `admit./sorry.` tactics, ZERO `axiom` decls, 6 `qed` lemmas). This is the
FIRST of the 7 cryptographically-charged break events given a REAL discharged EC
reduction: `slotReplayOrRollback` is proven **Pr[break] = 0%r** via an operational
consume-once lifecycle module (a `consumed` digest-ledger + monotone barrier +
guarded one-use `spend` transition; unbounded adaptive adversary over a `while`
schedule). Crucially NON-vacuous: it DERIVES the never-firing postcondition from
the state machine (break flag = live read `accept ∧ (is_replay ∨ is_rollback)`,
provably never set because `accept ⟹ ¬is_replay ∧ ¬is_rollback`, Z3-closed) —
unlike the Lean `ConcreteBreakFor = False`-by-construction and unlike the spike
which TOOK never-firing as a hypothesis. No residual hardness assumption: this
event is structurally impossible, now machine-proven. 6 of 7 charged breaks
remain (recommended order in `formal/easycrypt/break_reduction_status.md`: next
`postchallengeSlotCreationOrRewrite` — same template, expected =0; then a CR-game
harness on `checkedCoreMessageBinding` amortized across the 4 binding/availability
events; the accepted-spend SIS bound is the deepest, last). Security still NOT
established — 6 reductions + the SIS bound + spec §6 path-hardening + external
review all remain.

**Risk-first pivot (2026-06-09, later session — James's standing riskiest-part-first
policy):** attacked the highest-redesign-risk obstruction first — does the creation-
time certificate bind the FULL account witness, or does the spend-only two-witnesses
survivor (`projection_carrier_tuple_does_not_prove_full_witness_opening`, lib.rs:2205)
reappear at creation? Prove-AND-break, 3 independent angles, UNANIMOUS high-confidence,
independently re-verified (survivor search `creation_survivor_found=false`; 5/5 rank
probe; ground-truth tests pass). **RESULT: WITNESS-BINDING is SOUND** — the certificate
makes the opening map injective; the survivor is killed because the leaf fixes a single
`account_witness_commitment` (`opening_commitment` SHA256 frame includes it,
`production_reference.rs:44266`) and the 18-residue packing is an unconditional bijection.
Reduces to SHA256 CR (named `prevalidation_leaf_binding`/`qrom_or_domain_sep` break),
NOT an open algebraic gap. **Correction:** the closer is the `opening_commitment` leaf-
binding, NOT the `masked_state18` repack alone (an algebraic identity; decision-doc line
119 mislocates it). **Risk frontier MOVED** binding → relation-validity
(`creation_time_hidden_source_soundness_break`): binding gives uniqueness, not relation-
VALIDITY. Next risk-first target (task #21): negative control feeding a unique-but-
relation-INVALID opening that still repacks to the leaf-fixed commitment — does the
soundness certificate REJECT it? Accept ⟹ redesign now. Artifacts:
`creation_time_binding_{VERDICT,rank_analysis,soundness_characterization}.md`,
`creation_time_witness_binding_findings.md`, survivor-search script+JSON,
`tools/parameter-search/tests/creation_time_binding_rank_probe.rs`.

**Relation-validity (the moved frontier) — also SOUND (2026-06-09→10, verified 4/4
probe tests).** The soundness certificate `verify_reviewed_fq12_state_assisted_masked_
projection_soundness_certificate` (production_reference.rs:38517) REJECTS relation-
invalid openings including the strong forged-carrier form (→ `reject_fail_closed_
invalid_projection_carrier`). No `creation_time_hidden_source_soundness_break`, no
relation-arm redesign. Mechanism: the 6-lane `account_projection` is RECOMPUTED from
the leaf-fixed commitment and the predicate enforces claimed==recomputed per lane — a
forger has no slot to inject an invalid projection. So BOTH creation-time soundness
questions (binding + relation-validity) are SOUND modulo named non-novel assumptions
(SHA256 CR + the verifier-exporter bridge for the 52 inactive columns, task #23). The
remaining creation-side gaps are NOT redesign risk: (build, task #22) the public
harness-free creation-proof dispatcher (:26475) is fail-closed — wire+de-toy a non-toy
public verifier (:26653) and prove it equivalent to the tested harness-replay relation
(:26360); this is what flips creation fail-closed→accepting. Net risk-first status:
the two highest-redesign-risk obstructions HELD; next risk-first target is #20 (SIS/
response-link ≥2¹²⁸ margin, partly external) + the #23 bridge assumption. Artifacts:
`creation_time_relation_validity_VERDICT.md`, `creation_time_relation_validity_probe.rs`.
