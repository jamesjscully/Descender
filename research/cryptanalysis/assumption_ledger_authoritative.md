# Authoritative Assumption Ledger (WS1)

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This ledger is useful cryptanalysis history, but its "authoritative" role was
> scoped to the WS1 campaign. Current assumptions and non-claims live in
> `../../spec/security_model.md`; top-level work status lives in `../../TRACKER.md`.

Created: 2026-06-09 as the WS1 assumption-ledger artifact for the retired
cryptanalysis-readiness campaign. Built from **direct reading of the Lean
source**, not from summaries. Current assumption authority lives in
`../../spec/security_model.md`; current work ownership lives in `../../TRACKER.md`.

Each entry: what it unfolds to (ground truth), exact location, concrete
parameters, honest classification, and WS2 reduction status + next action.

Classification key: **STD** = named studied problem · **STD-NS** = studied
problem at nonstandard parameters/metric · **NOVEL** = genuinely novel/structured
· **GAP** = assumption gap (named break / left open) · **DEFECT** = formalized
statement does not capture the intended claim.

---

## A. The core problem template

### A1 — `P_TND_Search` / `P_TND_Decision` — STD-NS (template), reduction NOT done
- `Descender/PTND.lean:16` (search), `:167` (decision), `:240` (`Distinguishes`).
- **Unfolds to:** find/distinguish a planted secret `s` with
  `centeredNormLinf q (target − eval s) ≤ eta` (search), or distinguish
  `realSample = eval s + noise` from `nullSample` (decision). `eval`, `noise`,
  `nullSample` are **arbitrary functions** — the structure is a generalized
  **q-ary Bounded-Distance Decoding / LWE** template, *not* yet a concrete hard
  problem. "Projected Target Nearest Distance" and "Projected Coboundary" are
  **project coinages**; no standard-literature problem by those names.
- **Honest note:** `PTND.lean` proves only elementary structural lemmas
  (unique-solution-from-separation, rounding-noise composition). Its own
  docstring states it "does not claim average-case reductions, KEM security, or
  distributional theorems." So **hardness is not established here**; it is only
  as hard as the eventual instantiation of `eval`.
- **WS2 status / next action:** the reduction-to-standard is the *instantiation*
  step — show the deployed `eval` is a module-linear map and `noise` the
  standard error distribution, then cite Module-LWE/Ring-LWE. **Not present in
  PTND.lean.** WS2-a must locate where `eval` is instantiated and prove (or
  flag) the linearity that makes A1 ≡ Module-LWE at the deployed params.

### A2 — `ProjectedCoboundaryFamily` — STD-NS (worst-case family), reduction NOT done
- `Descender/PTND.lean:121`.
- **Unfolds to:** a predicate `instances : P_TND_Search → Prop` plus a worst-case
  `Solver`; "no solver solves the family." A worst-case BDD/LWE family.
- **WS2:** standard average-case → worst-case lattice machinery is the target;
  none is proven (the file disclaims it). Flag as STD-NS pending the A1
  instantiation.

## B. The lattice layer

### B1 — `QaryPrimalLatticePromise` — STD (= SIS), at nonstandard L1 metric
- `Descender/QLatticeHardness.lean:79`; short vector `:68`.
- **Unfolds to:** no nonzero `z` with `residueVec q z ∈ image(A)` and
  `intL1Norm z < vectorBound`. This **is the Short Integer Solution (SIS)**
  problem (short nonzero vector in a q-ary lattice).
- **Nonstandard:** metric is **L1** (`intL1Norm`), not the L2 used by the lattice
  estimator literature; small dims (see params). Stated as an **assumed
  promise**, not reduced to Module-SIS.
- **WS2:** map L1 bound → L2/∞ and relate to Module-SIS at deployed params; this
  is the tractable case (genuine SIS). Feeds WS3 estimator.

### B2 — `QaryDualLatticePromise` / `AmplitudeVulnerableQaryDualRelation` — STD (= dual-LWE distinguishing), nonstandard
- `Descender/QLatticeHardness.lean:87`, `:106`, `:296` (`CenteredNoiseAntiConcentrated`).
- **Unfolds to:** no nonzero short dual relation `z` whose centered-noise pairing
  stays in a proper interval mod q (the dual distinguisher for decision-LWE).
- **WS2:** standard dual-attack framing; relate the centered-L∞/amplitude
  condition to the L2 dual-attack cost model. Feeds WS3.

### B3 — SIS seed / full-response openings — STD (= SIS), nonstandard, bit-packed
- `Descender/LocalDeltaByteCommitment.lean` (`SISStyleSeedOpeningMap` ~`:2434`;
  full-response reduction further down). **WS1-b must pin exact lines.**
- **Unfolds to:** SIS-style short-preimage opening for the packed commitment
  (seed: 18 residues; full-response: 23 residues, 12 public rows), bounded norm,
  bit-packed to SHA256-width.
- **WS2:** same as B1/B2 — relate to Module-SIS at the packed parameters.

## C. Novel / structured assumptions (the load-bearing risk)

### C1 — Fq12 action collision resistance — NOVEL
- `Descender/LocalDeltaByteCommitment.lean:2056-2063` (`action`,
  `badActionCollision`); floor `actionCapacityFloorBits = 163`;
  `Descender/Path2HashBudget.lean:139` (`Fq12ActionCollisionBudgetPriced`).
- **Field:** `F_12289[X]/(X^12 + X + 25)` (degree-12 extension).
- **Unfolds to:** `badActionCollision c₀ c₁ ↔ action c₀ = action c₁`; security =
  hard to find a collision of `action`, asserted floor 163 bits. **`action` is an
  abstract certificate field here** — the concrete map and its collision bound
  live in the Rust reference / generated data.
- **Two distinct issues:**
  1. **Checkable-but-assumed facts:** `extensionPolynomialIrreducible`,
     `rabinXQ12MinusXZero`, gcd-degree-zero are carried as `_holds` Prop fields
     (assumed) for a **concrete** polynomial — these are decidable finite
     computations and should be *proven* (`decide`/Rabin test in Lean), not
     assumed. (WS2-a sub-task; removes an unnecessary assumption.)
  2. **The genuine assumption:** collision-resistance of `action` with a 163-bit
     floor. Whether this is a generic birthday bound over the action space or
     hides algebraic collisions depends entirely on how `action` is defined.
- **WS2 status / next action:** locate the concrete `action` definition (Rust /
  generated). If it is hash-derived, the floor is a generic collision bound
  (standard-ish, WS3 prices it). If it is an algebraic map, **WS4-d must attack
  it structurally** — this is the single highest-risk reduction question.

### C2 — Role-root binding — NOVEL (reduces to commitment binding)
- `Descender/Path2RoleRootBinding.lean:78` (`…PinsNativeRoleRoot`), `:111`
  (`…PinsFullOpeningRoles`).
- **Unfolds to:** over the abstract `Predicates` model — two *valid* role-lift
  certificates for the same accepted payload either agree on
  `nativeRoleRootCommitment` (and on the full opening coordinates), **or** a
  named break is exported. This is a **commitment-binding** statement
  specialized to the role-root structure.
- **WS2:** reduce to **collision-resistance of the underlying digest** (standard,
  if `nativeRoleRootCommitment` is a hash). Next action: confirm the commitment
  is a concrete collision-resistant hash and replace the abstract `Prop` field
  with a reduction to its collision-resistance. Likely tractable; lowers C2 from
  NOVEL toward STD.

### C3 — Active-lane nullity / no-survivor — **RESOLVED 2026-06-09** (was DEFECT)
- `Descender/HiddenSourceActiveSubspace.lean` (def + theorem),
  `Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean` (literal data),
  `scripts/research/generate_hidden_source_active_subspace_lean.py` (generator).
- **Trace result (WS1-d step 1):** the old `ActiveLaneNullityZero` /
  `active_relation_only_zero` had **zero downstream consumers** in Lean (grep
  across `Descender/` excluding the two defining files is empty) and **no Rust
  gate inspects this file**. So it was **not a live soundness hole** — a
  misleading vacuous companion. The substantive bounded-survivor content lives
  in `ResponseLinkExactCertificate.excludesBoundedNonzeroKernel`
  (`responseDifferenceBound < minimumCoverCoeffBound`), a real searched-bound
  inequality, which the generated module already references.
- **What was wrong:** `ActiveLaneNullityZero := ∀ x, (∀ lane, x lane = 0) → x = 0`
  is a tautology (proved by `funext`) carrying no information about the
  projection.
- **Fix applied:** replaced with the faithful, **non-vacuous**
  `ActiveLaneReadoutDetects` — "if every active lane equation
  `coefficientModQ * x sourceColumn` reads zero, then every active source
  coordinate is zero" — which is *false* if any coefficient ≡ 0 mod q. Proved by
  `activeLaneReadoutDetects_of_coefficientsNonzero` via field cancellation
  (`mul_eq_zero` over `ZMod 12289`, using a new `Fact (Nat.Prime 12289)`
  instance), with the non-vacuity witness `coefficientsNonzeroModQ` discharged by
  `native_decide` over the 12 literal coefficients. Base + generated module +
  generator script all updated; full `lake build` (3358) and the axiom audit are
  green. The Lean fact now matches the intent of the Python replay for the
  active-lane readout (the *bounded-kernel* no-survivor part remains, as before,
  in `ResponseLinkExactCertificate`).
- **Residual:** this proves the active readout is non-degenerate; it does **not**
  by itself prove the wider hidden-source no-survivor claim (that is the
  bounded-kernel certificate's job). Keep both linked in WS4 when probing the
  hand-searched kernel.

## D. Named break events (assumption gaps — priced/proven per WS7, D2)

- `Descender/Path2FullSpendGame.lean:62-73` — 9 events:
  `creationTimeHiddenSourceSoundnessBreak`, `prevalidationLeafBindingBreak`,
  `prevalidationRegistryBindingBreak`, `prevalidationDataAvailabilityBreak`,
  `postchallengeSlotCreationOrRewriteBreak`, `checkedCoreMessageBindingBreak`,
  `slotReplayOrRollbackBreak`, `qromOrDomainSeparationBreak`,
  `liveSlotLeakageOrKeygenBreak`.
- `Descender/Path2NativeBoundary.lean:410-426` — 14 boundary events (overlapping;
  dedupe in this ledger): boundary/descent/inner-commitment/transparent-proof
  soundness, norm/range, no-export, QROM/domain-sep, reviewed-Fq12 payload
  binding / hidden-source soundness / domain-sep.
- **Per D2 (conservative):** replay/rollback and domain-separation breaks should
  be **proven** (WS7-b), not left as named assumptions; QROM/hash and
  keygen-leakage may remain named assumptions but must be standard (QROM, not
  ROM) with margin. D2 decision pending on exactly which stay named.

---

## Summary table

| ID | Assumption | Loc | Class | WS2 reduction status |
|---|---|---|---|---|
| A1 | P_TND search/decision template | PTND.lean:16/167 | STD-NS | instantiation→Module-LWE **not done** |
| A2 | Projected coboundary family | PTND.lean:121 | STD-NS | worst-case machinery **not done** |
| B1 | Primal q-ary no-short-vector | QLatticeHardness.lean:79 | STD (SIS) | L1→L2 map to Module-SIS pending |
| B2 | Dual q-ary distinguisher | QLatticeHardness.lean:87/296 | STD (dual-LWE) | cost-model map pending |
| B3 | SIS seed/response openings | LocalDeltaByteCommitment ~2434 | STD (SIS) | params→Module-SIS pending |
| C1 | Fq12 action collision | LocalDeltaByteCommitment.lean:2056 | NOVEL | **highest risk**; checkable facts assumed; action map to be located |
| C2 | Role-root binding | Path2RoleRootBinding.lean:78/111 | NOVEL→STD? | reduce to digest collision-resistance |
| C3 | Active-lane nullity | HiddenSourceActiveSubspace.lean | **RESOLVED** | re-formalized as non-vacuous readout-detection; proven |
| D | 23 named break events | FullSpendGame:62 / NativeBoundary:410 | GAP | prove replay/domain-sep (D2); keep QROM/keygen named-but-standard |

## WS2 reduction verdicts (2026-06-09)

Reproducible artifacts: `scripts/research/verify_fq12_irreducibility.py`,
`scripts/research/cryptanalysis_concrete_security_estimate.py`.

- **C1 (Fq12 action collision) → REDUCES TO STANDARD (QROM hash collision).**
  The deployed action `reviewed_fq12_challenge_action_vector`
  (`production_reference.rs:36992`) sets each of the 12 residues to
  `uniform_mod_from_hash(domain, challenge, idx, Q)` — a SHA256-derived
  hash-to-`F_q^12` map. An action collision is therefore a **generic hash
  collision** over `12289^12 = 2^163.0`, *not* an algebraic shortcut. The Fq12
  field (`X^12+X+25`, **verified irreducible over F_12289** by the Rabin script)
  is used only for the multiplication `z = r + action(c)·w` enabling forking
  extraction (nonzero difference invertible). **Reduction:** collision-resistance
  of the SHA256-derived map in the QROM (standard).
  **Quantitative (corrected 2026-06-09):** the action-collision soundness term is
  `~2^-162` **per fork** (negligible; adequate for 128-bit). An earlier draft
  modeled it as a multi-target `T^2/2^163 = 2^-99` term — that was WRONG: a
  precomputed collision yields no forgery (the response still needs `w`, and FS
  cannot be cheaply steered onto a colliding challenge), so the floor matters only
  for the forking extractor (per-fork `2^-162`). C1 imposes **no** `2^-128`
  obstruction; composed-bound closure is WS7 reduction-wiring, not a C1 gap.
  *Lean debt:* irreducibility + Rabin facts are still assumed `_holds` fields
  though now externally verified-true; discharging them in Lean (Rabin via
  `native_decide` on `(ZMod 12289)[X]` modular exponentiation) is laborious but
  feasible — tracked, not a soundness risk.

- **C2 (role-root binding) → REDUCES TO digest collision-resistance.** The
  `…PinsNativeRoleRoot`/`…PinsFullOpeningRoles` defs say two *valid* role-lift
  certificates agree on `nativeRoleRootCommitment` (and coordinates) or a break
  fires — i.e. the commitment binds. If `nativeRoleRootCommitment` is a SHA256
  digest (it is, in the deployed frame), this is collision-resistance of that
  digest (standard). Reduction is structurally clear; the Lean form keeps it an
  abstract `Prop` field. *Tag:* reduced-to-standard (pending the Lean rewrite
  that replaces the abstract field with a CR reduction).

- **C3 (active-lane nullity) → RESOLVED** (see entry above): re-formalized as the
  non-vacuous, proven `ActiveLaneReadoutDetects`.

- **B1/B2/B3 (q-ary primal/dual, SIS openings) → STANDARD (SIS / dual-LWE),
  metric mapping pending.** These are genuine SIS / decision-LWE in an L1 /
  centered-L∞ metric. The deployed instance is *small* (≈23 response coords, 12
  syndrome rows, q=12289, centered coord bound 48): for this size the
  **authoritative short-vector statement is the in-repo exact enumeration**,
  `ResponseLinkExactCertificate.excludesBoundedNonzeroKernel` proving
  `responseDifferenceBound 96 < minimumCoverCoeffBound 114` (combined kernel dim
  9) — i.e. NO nonzero kernel vector inside the response box. The asymptotic
  core-SVP figure is only relevant to the larger keygen/public-view SIS, whose
  exact instance dimension must be extracted before any bit-security number is
  asserted (WS3 remainder). *Tag:* reduced-to-standard family; concrete margin
  is WS3.

**DoD-2 status:** C1 reduced (QROM CR) · C2 reduced (digest CR) · C3 resolved ·
B1/B2/B3 standard family. **No assumption remains classified as irreducibly
novel.** Residual is quantitative (WS3 margins, WS7 composed bound) and Lean-debt
(discharge C1 irreducibility, rewrite C2 as an explicit CR reduction), not a new
hardness problem.

## WS1-b/c finalization (2026-06-09)

- **B3 SIS openings (lines pinned):** `Descender/LocalDeltaByteCommitment.lean`
  `SISStyleSeedOpeningMap` `:2434`; `PackedSISFullResponseOpeningReduction`
  `:3409`.
- **Break-event dedupe (WS1-b):** the 14 boundary events
  (`Path2NativeBoundary.lean:412-425`) and 9 spend events
  (`Path2FullSpendGame.lean:64-72`) overlap on: `qromOrDomainSeparationBreak`,
  `prevalidationLeafBindingBreak`, `prevalidationDataAvailabilityBreak`,
  `checkedCoreMessageBindingBreak`, `slotReplayOrRollbackBreak`. Distinct
  boundary-only: boundaryBinding, descentSoundness, innerCommitmentBinding,
  transparentProofSoundness, normOrRange, noExport, reviewedFq12PayloadBinding,
  reviewedFq12HiddenSourceSoundness, reviewedFq12DomainSeparation. Distinct
  spend-only: creationTimeHiddenSourceSoundness, prevalidationRegistryBinding,
  postchallengeSlotCreationOrRewrite, liveSlotLeakageOrKeygen. **Union = 18
  distinct named assumptions.**
- **WS1-c classification settled:** A1/A2 confirmed **project coinages for q-ary
  BDD/LWE** (no standard problem is mis-named); B1/B2 are SIS/dual-LWE; the L1 /
  centered-L∞ → L2 metric mapping is the only open WS1-c arithmetic, folded into
  WS3.

## WS1 completion checklist (DoD-1)

- [~] WS1-a: load-bearing assumptions read + classified from source (this doc).
- [x] WS1-b: B3 lines pinned (2434/3409); break events deduped → **18 distinct**.
- [x] WS1-c: A1/A2 confirmed coinages for q-ary BDD/LWE; B1/B2 = SIS/dual-LWE;
  only open item is the L1→L2 metric mapping, folded into WS3.
- [x] WS1-d: **C3 defect RESOLVED** — traced (no consumers; not a soundness
  hole) and re-formalized as a non-vacuous, proven readout-detection property
  (base + generated + generator script). Build + audit green.
- [x] WS1-e: every premise classified; **no irreducibly-novel assumption
  remains** (DoD-1 met at classification level; residual is quantitative/Lean-debt,
  tracked under WS3/WS7).
