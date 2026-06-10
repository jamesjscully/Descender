# Signature-Path Change Investigation (READ-ONLY reconstruction)

Status: investigation note, 2026-06-09. Reconstructs *why* the deployed/KAT path
is the 260-byte "linear-sigma" representative signature while the canonical spec
(`descender-signature-spec.md` §2) targets a 314-byte
`checked_core[258] ‖ projection_carrier[56]` envelope. Written to answer a
maintainer question; it proposes nothing unilaterally and flags open decisions as
questions, not reversals.

Bottom line up front:
- The 260-B linear-sigma path is **the executable baseline / byte anchor**, not
  the security target. It deliberately **wraps the legacy representative proof**.
- The 314-B Fq12 projection-carrier path is **the selected spec target but is
  fail-closed** (its deployment verifier rejects everything today).
- The public conditioned-fiber sigma forgery on the 260-B path is a
  **deliberately documented, test-asserted critical limitation** of the baseline
  — *not* a latent/accidental bug. (High confidence.)
- The actual long-term direction has moved a third time, to **account-creation /
  prevalidated-slot authorization** (a.k.a. "Path 2 prevalidation"), which is why
  neither 260-B nor 314-B is being hardened head-on right now.

---

## 1. Timeline (commit evidence)

All hashes from `git log` on `master`. Dates are author dates.

| When | Commit | What it established |
|---|---|---|
| (spec spine) | `b5a1d98` *feat(formal): multi-tool verification scaffold (spec spine + phased plan)* — Jun 9 22:57 | Introduced the canonical spec with the **314-B** `checked_core[258] ‖ projection_carrier[56]` layout as the normative target, and §6 already lists "Full witness opening vs projection-only" as an **open spec question**, citing regression `projection_carrier_tuple_does_not_prove_full_witness_opening`. So 314-B was authored *already knowing* the carrier proves a projection, not a full opening. |
| Jun 2 20:13 | `137f412` *Track active path implementation source* | First and only commit to introduce **all** of `LINEAR_SIGMA_*`, `verify_public_quotient_proof_signature`, `checked_core`, `projection_carrier`, `conditioned_fiber`, `full_witness_opening` (via `git log -S`). This is the import of the active-path Rust reference + Lean Path2*.lean stack. Both the 260-B linear-sigma representative path and the 314-B Fq12 carrier path arrive together here. |
| Jun 2 20:19–20:27 | `dac8f2a`, `dc09dcf` *Track research/audit notes* | Imported the decision notes that record the rationale: `deployable_signature_target.md`, `hidden_source_projection_design_fork.md`, `selected_hidden_source_path_decision.md`, `accepting_308b_verifier_blocker.md`, etc. |
| Jun 9 22:56 | `82faa37` *fix(lean): repair AP2-004 byte-constant drift…* | The **AP2-004 challenge widening** (algebraic challenge 21→27 B, mask `0x03`→`0x3f`). This is the "widening that broke the build" referenced in the KAT README. It is what moved the pre-widening 252+56 = **308**-B figure to 258+56 = **314**-B for the spec target, and 187+73 = **260**-B for the linear-sigma path. |
| Jun 9 22:56 | `2a18dca` *feat(kat): KAT v0 corpus from the reference…* | Froze KAT v0. Commit message is explicit: *"pins the current 260B linear-sigma path (wraps legacy proof) -- byte anchor, not a soundness claim; regenerate on path-hardening."* |

The hundreds of `AP2-N003 …` commits at HEAD are downstream Lean proof-plumbing of
the *full-signature reduction* obligations (`signatureForgerySuccessProb_le_*`),
not format changes.

### Why "260", "308", and "314" all appear
- `260` = `STATEFUL_TRANSACTION_SIGNATURE_BYTES (187) + LINEAR_SIGMA_TARGET_PROOF_BYTES (73)`
  where 187 = `slot_id[4] + challenge[27] + active_response[52] + openings[104]` and
  73 = `proof_commitment[32] + masked_witness_response[41]`
  (`production_reference.rs:205–212`). This is the **deployed/KAT** path.
- `314` = `REVIEWED_FQ12_STATE_ASSISTED_OPENING_FRAME_BYTES (258) +
  HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES (56)`
  (`production_reference.rs:544–550`,
  const `REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES`). This is the
  **spec target** path.
- `308` = the *pre-AP2-004* value of the 314-B target (252+56). Several Rust strings
  and the fixture name `selected_projection_valid_308_byte_fixture_accepts` are
  flagged as **stale labels** in `research/deployable_signature_target.md:8–13`.

---

## 2. The two paths in code

### 2a. 260-B linear-sigma (deployed, KAT-anchored, ACCEPTS)
- Sign: `production_sign` → `prodref::sign_with_secret_key` → `reference_sign_with_proof`
  (`signature_q12289_local_delta_stateful.rs:4787, 4678`). Serializes
  `slot_id ‖ challenge ‖ responses ‖ openings ‖ linear_sigma_target_proof`,
  length `LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES` (260).
- Verify: `production_verify` → `prodref::verify_with_account_binding`, whose inner
  closure calls `legacy_account_from_production(...)` then
  `reference_verify_with_proof` = `verify_public_quotient_proof_signature`
  (`…:4853–4874, 4757–4764`). **This is literally "wraps the legacy proof"**: the
  production binding layer adds account/slot/freshness checks around the legacy
  representative quotient verifier.
- What it verifies: a *public quotient representative* relation — the response `z`
  is treated as a representative that solves the public 12×23 q-syndrome equations
  plus a 2-byte fiber-state constraint and the local-opening predicate. It does
  **not** require a prechallenge witness-bound sigma commitment.
- This path **accepts** (KAT v0 has 3 positive round-trips).

### 2b. 314-B Fq12 projection carrier (spec target, FAIL-CLOSED, REJECTS)
- Constants/types: `REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES`,
  `HiddenSourceFallbackProjectionCarrier`, the state-assisted opening tuple, and the
  composing verifier `verify_reviewed_fq12_state_assisted_projection_carrier_tuple`
  (`tools/production-reference/src/lib.rs`).
- Disposition: **fail-closed by design.**
  `production_reference.rs:22` —
  `PRODUCTION_DEPLOYMENT_BLOCKER_SELECTED_VERIFIER = "selected 308-byte/Fq12
  active-lane projection-carrier deployment verifier is fail-closed until
  active-lane preflight promotion and hidden-source semantics are implemented"`.
  `…:556` — the carrier exporter is `…_UNIMPLEMENTED` / fail-closed. The
  preflight asserts `reviewed_fq12_deployment_verifier_present == false`
  (`…stateful.rs:8746–8747`).
- Status: **partially implemented, intentionally not accepting.** The parser,
  carrier codec, statement/pad/transcript bindings, and the composing tuple
  verifier exist and are exercised, but the verifier is wired to **reject** until
  the account-witness opening + soundness + ZK obligations close. So 314-B is
  "implemented as a fail-closed skeleton + aspirational acceptance," not a live
  accepting verifier.

### 2c. Lean
- `137f412` imported `Descender/Path2*.lean` (Path2FullOpening*, Path2NativeBoundary,
  Path2FullSpendGame, etc.). The full-signature reduction shape lives in
  `NativeFullSignatureSecurityBoundary` and is being driven to
  `signatureForgerySuccessProb_le_active128BitTarget` by the AP2-N003 commits.
- `audit/q12289_full_signature_obligation_burndown.md:39–44, 56` records that the
  **q12289 FS-core endpoint is explicitly NOT the deployment verifier** (QFS-007
  "redesign trigger fired"); the active theorem target is the Path 2 / reviewed
  Fq12 relation. The q12289 path is kept as a reference/negative-control shape.

---

## 3. The forgery: exactly what it demonstrates

Two probes in `signature_q12289_local_delta_stateful.rs`, both **asserted in
tests** and both flagged in the security audit:

1. `public_proof_forgery_probe()` (`…:2711`): the old fixed-commitment
   representative attack. It **accepts** under the legacy representative verifier
   (`verifier_accepts == true`, asserted `…:7673, 8716`) but is **rejected** by the
   strict sigma-equation verifier (`sigma_equation_verifier_accepts == false`).
2. `public_sigma_equation_forgery_probe()` (`…:2809`): the *adapted* attack. After
   the challenge is known, the attacker chooses a proof commitment, computes
   `a + c·digest`, and **publicly solves the post-challenge quotient equations**
   with a bounded fiber state. This forgery **accepts even under the strict sigma
   verifier** (`verifier_accepts == true`, asserted `…:7697, 8738`,
   `accepted_reason == "ok-stateful-local-delta-public-quotient-sigma-equation"`).

Both probes set `uses_private_key_material = false` and
`account_hidden_witness_available = false`, and the test asserts those are false.
So an adapted public conditioned-fiber sigma forgery **verifies without any private
witness or hidden account material**.

**Is this a documented limitation or a latent bug? Documented limitation. (High
confidence.)** Evidence:
- It is **asserted to accept** in two tests, one named
  `public_postchallenge_representative_forgery_remains_negative_control`
  (`…:8708`). A latent bug would not be pinned as a passing negative control.
- It is filed in the internal audit as a `severity: "critical"` finding with a
  `required_closure` (`…:6452–6457`): *"make acceptance depend on a prechallenge
  witness-bound sigma commitment or another hidden-witness proof."*
- The review-gate string says it outright (`…:6428`): *"the current local-opening
  relation is still not sound: an adapted public conditioned-fiber sigma-equation
  forgery verifies without private witness material."*
- The KAT README and the `2a18dca` commit message both state the 260-B path is a
  *byte anchor, not a soundness claim*.

So the 260-B path is **knowingly unsound as a standalone spend proof.** It is
shipped as a conformance/byte anchor, with the soundness burden explicitly deferred.

Distinct second weakness (do not conflate): the **314-B** path has its own
documented gap — `projection_carrier_tuple_does_not_prove_full_witness_opening`
(`tools/production-reference/src/lib.rs:2205`). That test builds two **different**
account-witness commitments (`witness_a` vs `witness_b`) that **both accept**
against the same six-value projection-opening frame. That proves the carrier proves
a *projection relation*, not a *full opening* of the account-witness commitment.
This is also a **documented limitation** (spec §6, `deployable_signature_target.md:104–119`),
not a bug. The 314-B path stays fail-closed precisely because of it.

---

## 4. Why the path is 260-B linear-sigma — reconstructed rationale

The 260-B linear-sigma path was **not** chosen as the secure spend format. The
evidence shows a deliberate layering decision plus a strategic pivot:

1. **It is the byte anchor / executable baseline.** The whole point of KAT v0 is
   cross-tool byte conformance across Rust/Jasmin/Lean/hax where proofs can't span
   tools. The 260-B linear-sigma path is the only **fully executable, accepting**
   sign→verify pipeline today, so it is what KAT pins. (`kat_v0.md:31–34`,
   `2a18dca` message.) The 314-B target can't anchor a corpus because it never
   accepts.
2. **The 314-B "secure" target is intentionally fail-closed** until the
   active-lane relation is promoted through preflight and the
   account-witness-opening / soundness / ZK obligations close
   (`production_reference.rs:22`, `accepting_308b_verifier_blocker.md`). You cannot
   ship an all-rejecting verifier as the conformance reference, so the baseline
   wraps the legacy representative proof and is labeled non-normative.
3. **A separate, earlier 260-B route was explicitly *rejected*.**
   `deployable_signature_target.md:54–63` archives a *"`260 B` full-response public
   quotient route: rejected as a deployable route because the public representative
   relation admitted a public forgery class."* The current deployed 260-B
   linear-sigma path is the **same representative family**, retained as a baseline
   *with the forgery documented*, not promoted to deployable.
4. **The real direction has moved past both.** The current selected architecture is
   **account-creation / prevalidated-slot authorization** (Path 2 prevalidation):
   validate the hidden-source relation at slot creation, commit the result into
   account state, and make spend a one-use authorization consume. Rationale
   (`hidden_source_projection_design_fork.md:53–60`,
   `selected_hidden_source_path_decision.md:32–62`): a rank probe showed extra
   carrier lanes buy only one linear constraint each (no compression), and moving
   the proof to creation-time is **more aggregation-friendly** and avoids ~100
   transient proof bytes per spend. So neither 260-B nor 314-B is being hardened
   head-on; both are kept as executable baseline + negative-control sources.

One-paragraph reconstruction: *The deployed path is 260-B linear-sigma because it
is the only fully-executable accepting sign/verify pipeline and therefore the only
thing KAT v0 can pin for cross-tool byte conformance; the secure 314-B Fq12
projection-carrier target is deliberately fail-closed (it rejects everything) until
its account-witness-opening/soundness/ZK obligations close, and the genuinely
"selected" long-term direction has actually pivoted again to account-creation
prevalidation. The 260-B path knowingly wraps the legacy representative ("public
quotient") proof, whose public-conditioned-fiber forgery is a deliberately
test-asserted, audit-filed critical limitation — the maintainers shipped it as a
byte anchor explicitly labeled "not a soundness claim," not as a sound spend
proof.*

This is **intentional**, not an unintended regression. The one thing that looks
like an oversight (a question for the maintainer, below) is the **stale "308"
labels** and the spec-vs-KAT byte-size divergence.

---

## 5. Decision options — cost/risk for the maintainer

### Option A — Harden to the 314-B full-witness-opening path
Make the 314-B Fq12 projection-carrier verifier accepting, with a real
account-witness opening proof closing
`projection_carrier_tuple_does_not_prove_full_witness_opening` and a prechallenge
witness-bound sigma commitment closing the §3 forgery.
- **Pros:** matches the canonical spec §2 as written; gives a single
  full-strength spend proof; closes both documented gaps at the spend layer.
- **Cost:** highest. Requires (per `deployable_signature_target.md:70–101`):
  active-lane preflight promotion, a full account-witness opening proof, ZK/soundness
  semantics, and Lean theorem integration. The §6 spec question ("add a full-opening
  proof") is unresolved precisely because it's hard.
- **Risk:** the design notes argue this direction *does not compress* (extra lanes
  buy one constraint each) and adds ~100 bytes/spend. You'd be hardening a path the
  team has already, on paper, decided to step away from. **Question for maintainer:
  is 314-B still the intended end state, or is the canonical spec §2 now describing
  a target the prevalidation pivot has superseded?**

### Option B — Formally scope v1 to projection-only soundness
Keep the carrier as a proved **projection relation** and downgrade the deployment
claim accordingly (spec §6 explicitly offers this: "formally downgrade the claim to
a proved projection-only relation with stated ZK/soundness").
- **Pros:** much cheaper; aligns with what the carrier actually proves; doesn't
  fight the prevalidation direction. The projection-only relation is already
  largely executable.
- **Cost:** medium. Requires writing the projection-only soundness + ZK statement,
  and — critically — making sure the **260-B baseline forgery is out of the
  deployable claim**, because projection-only soundness still does not rescue the
  representative spend proof.
- **Risk:** projection-only is a **strictly weaker** security claim. If downstream
  consumers assume full-witness binding, that is a real semantic gap. The §3 forgery
  shows the spend layer needs a prechallenge witness-bound commitment regardless;
  projection-only soundness on the carrier does not by itself fix spend
  unforgeability. **Question for maintainer: does the prevalidation architecture
  intend to carry the spend-unforgeability burden at creation time, making
  projection-only acceptable at spend? The notes (selected_hidden_source_path_decision.md:106–123)
  say yes — but that theorem is not yet closed.**

### Headline recommendation (CORRECTED 2026-06-09 — supersedes the earlier projection-only lean)

**The earlier draft of this section recommended "scope v1 to projection-only." That
was wrong and is retracted.** It conflated two different things: (i) *projection-only
*at spend*, with the full hidden-source soundness discharged at account creation* —
which IS the selected design — and (ii) *projection-only as the entire security
claim, dropping full-witness binding* — which is exactly the underconstraint the
whole Fq12 redesign exists to remove. Picking (ii) would re-enshrine the forgeability
documented in §3.

Corrected picture: **fail-closed is the project's scaffolding strategy, not evidence
of abandonment.** The 314-B/Fq12 path is held fail-closed (`PRODUCTION_DEPLOYMENT_
BLOCKER_SELECTED_VERIFIER`) *by design* — stand up the parser/exporter/verifier
structure rejecting everything, then discharge the internal obligations and only then
flip to accepting. The hidden-source soundness burden is **relocated** to account
creation by the Path-2 prevalidation pivot (for byte cost + aggregation), **not
dropped**: `selected_hidden_source_path_decision.md:64–123` lists the creation-time
proof obligations explicitly, and the selected theorem is *accepted-spend ⟹ the slot
was creation-validated for the same identity/slot/opening/pad/version/root, or a
named break*. Projection-only *at spend* is acceptable **only because** creation is
required to carry full validation — a theorem that is **not yet closed** and is the
real Phase-2 target.

So the live question is **not** "full-opening vs settle-for-projection-only." It is:
finalize the **account-state / prevalidation layout** and **close the creation-time
hidden-source validation + prevalidated-slot-authorization theorem** (which the
landed `Path2TraceModel.lean` + the EC `slotReplayOrRollback` consume-once reduction
already target). The 260-B linear-sigma path stays a byte anchor + negative control
and must never carry a deployable soundness claim (§3 forgery). The maintainer owns
the account-state-layout choice; this investigation does not assert it.

### Things that look like they *might* be mistakes (flagged as questions, not reversals)
1. **Spec §2 vs KAT divergence.** The canonical spec normatively states 314-B, but
   the frozen KAT and the only accepting pipeline are 260-B. This is internally
   consistent *if* everyone reads "spec = aspirational target, KAT = current
   baseline," but a reviewer could reasonably read it as drift. *Question: should the
   spec explicitly mark §2's 314-B as "target, not yet conformance-anchored" and add
   the 260-B baseline as a labeled non-normative appendix?*
2. **Stale "308" labels.** `research/deployable_signature_target.md:8–13` already
   flags that Rust strings and the **source-gate-inspected** fixture name
   `selected_projection_valid_308_byte_fixture_accepts` say 308 while the computed
   size is 314. Because a source-gate inspects that name, a coordinated relabel is
   needed and is non-trivial. *Question: schedule the 308→314 relabel?*
3. **`PRODUCTION_DEPLOYMENT_BLOCKER_SELECTED_VERIFIER` still says "308-byte"** while
   the computed target is 314-B — same stale-label class.

None of the above were changed by this investigation (read-only).
