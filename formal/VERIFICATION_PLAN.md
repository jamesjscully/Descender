# Descender Formal Verification — Phased Plan

Governing principle: **prove cheap-and-decisive things first; gate each tool
commitment on a spike, not optimism.** Tool effort optimizes formalization cost
and credibility, not security (external cryptanalysis of the SIS/QROM
assumptions stays irreducible — Phase 0 below makes that explicit).

Status legend: `[ ]` open · `[~]` in progress · `[x]` done · `[!]` decision gate.

## Phase 0 — Foundations (largely DONE, keep current)

- [x] Canonical spec spine (`spec/descender-signature-spec.md`).
- [x] Lean math + reduction skeleton: assumption ledger, C1→QROM-hash reduction,
  verified concrete-security estimator, irreducibility check, param-sync CI
  guard, and the **closure skeleton** (`Descender/Path2AcceptedSpendClosure.lean`).
- [x] Build + axiom audit + cryptanalysis CI green.
- [x] **KAT corpus v0** (`kat/`): frozen reference test vectors generated from the
  REAL Rust signing pipeline (keygen/sign/verify) — `formal/kat/kat_v0.json`:
  3 positive round-trips + 8 negative vectors with 7 distinct reject reasons
  captured from the real verifier. Reproducing `--kat-check` checker (exit 1 +
  first-divergence diff on drift) + 2 `#[test]`s, wired into `.github/workflows/ci.yml`.
  *Caveats (honest):* (a) anchors the Rust reference to ITSELF only — no second
  implementation exists yet to cross-check; cross-tool value is realized later.
  (b) Pins the CURRENT deployable linear-sigma path that still wraps the LEGACY
  proof (NOT the sound final target; an adapted public conditioned-fiber sigma
  forgery still verifies) — KAT v0 is a byte anchor, NOT a soundness claim, and
  MUST be regenerated when that path is hardened. (c) `.github/` + `formal/` were
  UNTRACKED at session end; maintainer must `git add` them for CI to take effect.

## Phase 1 — Decisive spikes (do these BEFORE committing to EC/Jasmin/hax)

Each spike is a few days, low commitment, and produces a go/no-go signal.

- [x] **EC-SPIKE — EasyCrypt reduction spike. VERDICT: GO (qualified).**
  *(RUN 2026-06-09 against installed EasyCrypt 2026.05 + Z3 4.16.0 — machine-checked,
  exit 0, independently re-typechecked.)*
  Formalized ONE game hop in
  EasyCrypt — the union-bound step + a single break-event reduction — and compared
  effort/clarity head-to-head against `Path2AcceptedSpendClosure.lean`.
  *Go signal:* game hops are markedly cleaner in EC and someone will maintain it.
  *No-go:* the missing Mathlib math (field/lattice/real-analysis) makes it a slog →
  keep the reduction in Lean. Success criterion written before starting.
  **STATUS: GO (qualified) — `formal/easycrypt/spike_union_bound.ec` typechecks
  exit 0 with ZERO `admit.` tactics** (verified: the only `admit` strings are in
  the discharge-ledger comments). All 5 lemmas close with `qed`: `break_zero_of_never`
  (full `byphoare` reduction of Pr[never]=0), `success_le_target` (union-bound
  assembly via `Bigreal.ler_sum`+`ler_add`, real arith closed by Z3), `sum_break_le_card`
  (`Bigreal.sumr_const`), `success_le_2pow_neg128` (concrete 2⁻¹²⁸ budget, Z3-closed).
  The cryptographic seam (OB-1/OB-2 single-experiment coupling) is a NAMED AXIOM —
  exactly the explicit hypothesis the Lean file also leaves open; EC does not make
  it cheaper (identical work both sides). Head-to-head: EC ~92 LoC vs Lean ~118;
  EC wins on the recurring probability-bookkeeping (where Phase-2's repeated cost
  lives), Lean wins on symbolic-power arithmetic (`norm_num` vs explicit expansions).
  Full rationale: `formal/easycrypt/SPIKE_VERDICT.md`.
  *Qualification:* before committing ALL of Phase 2, run a second narrow spike on
  the OB-1 coupling + one real QROM break-event reduction. **Decision: the
  probability-assembly / numeric-closure spine migrates to EasyCrypt; Lean retains
  the field/lattice math (seam 1 governs the spec correspondence).**
- [x] **HAX-SPIKE — hax extraction spike. VERDICT: GO (conditional).**
  *(RUN 2026-06-09 with installed `cargo-hax` 0.3.7 + `fstar.exe` 2026.03.24 + Z3 4.16.0.)*
  Ran `hax` on the self-contained Fq12 sigma-equation verifier and extracted to F*.
  *Go signal:* extraction is clean and the F* obligation is tractable → adopt hax
  to retire the marker gates. *No-go:* the Rust uses constructs hax can't handle →
  fall back to Cryptol+SAW or a Lean executable model + KAT.
  **STATUS: GO (conditional) — hax extracted the REAL target to a 509-line
  `formal/hax/extracted/Spike_core.fst`.** No Rust construct defeated the frontend:
  `Result<_,String>`+`?`, `Vec<u16>` push/pop/from_elem, `format!`/`String` errors,
  `assert_eq!`, `let-else`, nested `for…enumerate` all extracted cleanly. Required
  ONE genuine hax-frontend fixup (`iter_mut().enumerate()`→index loop; issue #420)
  plus 4 source-idiom rewrites; F* typecheck resolved all heavy machinery
  (`fold_range`, Vec/Result/Option models) and blocks on ONE missing library
  combinator (`while_loop_cf`, the `poly_mul_mod` loop). **Two one-time prerequisites
  before a hard GO:** (a) separate the verifier arithmetic from the `rusqlite`→
  `libsqlite3-sys` dep (whose `cfg_select!` build script post-dates hax's pinned
  nightly — a toolchain skew, NOT a hax capability gap; done for the spike via the
  isolated `formal/hax/spike_core/` crate), and (b) top up the 5 missing F* proof-lib
  stubs (or pin a hax build whose libs match `fstar.exe` 2026.03.24).
  **Honesty:** no F* lemma was PROVED — reached a near-clean name-resolution/typecheck;
  the spec-§3 obligation is structurally tractable (only non-trivial lemma is
  `poly_mul_mod` schoolbook-mul correctness) but not discharged. Full rationale:
  `formal/hax/SPIKE_VERDICT.md`. **Decision: adopt hax for the bridge layer to
  retire the source-inspection marker gates; keep `kat/` as independent cross-check
  and keep the marker gate until the F* lemmas actually discharge.**
- [x] **JASMIN-SCOPE. DONE** (`formal/jasmin/JASMIN_SCOPE.md`, 2026-06-09). Smallest
  v0 core = constant-time Fq12 sigma check (`z = a + action(c)·w`) + canonical parse
  of sig/pk, reusing the hax `Spike_core.fst` decomposition; keygen/signing/carrier/
  account-tree out of scope. CT is largely NOMINAL (verifier is public-data; binding
  case = reuse on not-yet-public commitments). **Headline: ~7 person-weeks** (toolchain
  0.5–1, parse 1, Fq12 arith 1.5–2, CT proof 0.5–1, EC-equivalence 2–3, KAT 0.5).
  Gated behind the first real EC break-event reduction (the equivalence target must
  exist). Flags: Jasmin NOT installed; and the conformance target must be a regenerated
  **KAT v1 for the 314-B layout** — `kat_v0.json` pins the 260-B legacy linear-sigma
  path, NOT the 314-B `checked_core[258]‖carrier[56]` spec target (see §6 open question).

## Phase 2 — Build the chosen stack (post-spike)

- [~] **Security (EC or Lean per EC-SPIKE):** the full game-based reduction —
  EUF/SUF-CMA → response-link-coded SIS + QROM hash, advantage ≤ 2⁻¹²⁸. Reuses
  the spec §5 per-event sorting. In Lean this means: concrete trace model `G`
  feeding the existing closure skeleton + per-event bounds. In EC: the standard
  game sequence.
  *Increment landed (2026-06-09, `Descender/Path2TraceModel.lean`):* a faithful
  concrete `AcceptedSpendNumericGame` whose events are `decide` of the REAL
  concrete verifier predicates and whose soundness field is DERIVED (not assumed)
  from the proven structural theorem via the closure bridge; `traceDist`/`observe`
  left as INPUTS so the certificate cannot be vacuously inhabited. `lake build`
  green; `#print axioms` on the 4 key theorems = only `[propext, Classical.choice,
  Quot.sound]` (zero `native_decide`/`sorry`). GENUINELY DISCHARGED: 2 named breaks
  (QROM/domain-sep, live-slot-leakage) proven probability 0 via real never-proofs.
  *EC reduction — first increment landed (2026-06-09, `formal/easycrypt/break_slot_replay_or_rollback.ec`):*
  per the EC-SPIKE GO, the reduction work moved to EasyCrypt. The FIRST of the 7
  charged break events is now a REAL discharged reduction: `slotReplayOrRollback`
  proven **Pr[break] = 0%r** (typechecks exit 0; verified ZERO `admit./sorry.`,
  ZERO `axiom` decls, 6 `qed`) via an operational consume-once lifecycle module —
  NON-vacuous (derives never-firing from the guarded state machine, not assumed),
  NO residual hardness assumption. Order for the rest in
  `formal/easycrypt/break_reduction_status.md`.
  *STILL OPEN (multi-week, NOT done):* 6 of 7 charged break bounds remain (next
  `postchallengeSlotCreationOrRewrite`, expected =0; then a CR-game harness over
  the 4 binding/availability events), the accepted-spend SIS/response-link bound
  (≤2⁻¹²⁹, the deepest — needs the real forgery→SIS reduction), and instantiating
  the EUF-CMA execution distribution. Security NOT established.
- [ ] **Implementation:** Jasmin verifier core, proven CT + functionally-correct
  vs the EasyCrypt/spec model; reproduces `kat/`.
- [ ] **Bridge:** hax-extract the remaining Rust reference; retire
  `production_reference.rs` source-inspection gates in favor of extracted-proof +
  KAT conformance.
- [ ] **Resolve spec §6 open questions** (full-witness-opening vs projection-only)
  before the reduction can claim more than projection soundness.

## Phase 3 — Composition + external

- [ ] Cross-tool conformance: all implementations pass `kat/`; reviewer checklist
  closes seam 1 (Lean↔EC spec correspondence).
- [ ] Package for **external cryptanalysis** of the SIS/QROM assumptions
  (`open_external_estimator`) — the irreducible step. The formal artifacts make
  this review fast and precise; they do not replace it.

## Decision gates summary

| Gate | Question | Resolves |
|---|---|---|
| EC-SPIKE | EasyCrypt or Lean for the reduction? | Phase 2 security tool |
| HAX-SPIKE | hax, or Cryptol/SAW, or Lean-exec+KAT for the bridge? | Phase 2 bridge tool |
| spec §6 | full opening vs projection-only claim? | what the reduction proves |

## Sequencing note

Phase 0 KAT + the two Phase-1 spikes are the immediate work and are independent
(parallelizable). Do **not** start Phase-2 build on any tool before its gate is
green — that is exactly the over-commitment this plan exists to prevent.
