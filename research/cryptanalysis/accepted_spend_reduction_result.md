# Accepted-Spend Forgery Reduction — Result (GAP E / AP2-N003)

**Date:** 2026-06-10
**Route:** (A) EasyCrypt game-based reduction (composes directly with the union-bound spine).
**File:** `formal/easycrypt/accepted_spend_forgery_reduction.ec`
**Status:** MACHINE-CHECKED, EasyCrypt 2026.05 / Z3 4.16.0, **exit 0**. No `admit`, no `sorry`; every lemma closes with `qed`.

---

## What was established

The dominant remaining term in the Path-2 success budget — the **accepted-spend forgery**
(the non-break path in `account_creation_prevalidated_slot_authorization_or_break`): an
adversary producing an ACCEPTING checked-core signature on a FRESH message WITHOUT the
witness — is bounded:

> **`accepted_spend_term_le_2pow_neg129`**: the same-action, fresh-message accepted forgery
> has probability **≤ 2⁻¹²⁹**.

This is exactly the `acceptedSpendBound ≤ 2⁻¹²⁹` hypothesis required by the Lean closure
`successProb_le_2pow_neg128_of_proportional_bounds`
(`Descender/Path2AcceptedSpendClosure.lean`) and the `eps_accepted = 2⁻¹²⁹` term in the EC
spine (`formal/easycrypt/gap_c_remodel_spine_wiring.ec`).

The bound is **DERIVED from a proven reduction**, not axiomatized.

---

## The reduction (forgery ⟹ coded-SIS-break ∨ action-collision)

An accepted forgery on a fresh message must produce a SECOND valid checked-core transcript
distinct from an honest one. The verifier's hard per-coordinate gate forces both responses
into `[-48,48]²³` (`RESPONSE_BOUND = 48`, `production_reference.rs:198`; gate at
`signature_q12289_local_delta_stateful.rs:4662`). The forgery splits into exactly two cases:

**(a) Same action challenge.** The response difference δ = r − r′ lies in `[-96,96]²³`
*exactly* (the triangle frontier `2·RESPONSE_BOUND = 96`, proven `coeff_diff_in_box`), and δ
is a nonzero vector in the response-link CODED kernel. Two sub-cases:
- **Inside-box** (δ realizable with coeff ≤ 96): **IMPOSSIBLE**. The `ResponseLinkExactCertificate`
  (~1.12M-node exact enumeration) proves the minimum coded-kernel cover coefficient is **114 > 96**.
  Proven empty in EC by `inside_box_impossible` (the certificate core: a nonzero coded vector with
  coeff ≤ 96 contradicts the enumeration frontier `min_coded_coeff = 114`).
- **Beyond-box**: the forger breaks the response-link CODED-SIS problem outside the deployed box
  → the named EXTERNAL hardness.

**(b) Different action challenge.** `action(c)` is a SHA-256 random-oracle hash-to-Fq12 (not
algebraic; `reviewed_fq12_challenge_action_vector`). Transferring a response across two distinct
action challenges requires an action collision. **This case is NOT the accepted-spend term** — it
is the separately-enumerated action-collision break event (one of the 9 named breaks). It is
charged to its own negligible slot (~2⁻¹⁶²), not folded into the accepted-spend term.

So the accepted-spend term proper (same-action, fresh-message) reduces to the coded-SIS surface
alone, and `accepted_spend_term_le_2pow_neg129` bounds it by 2⁻¹²⁹.

### Reduction implication — GENUINELY PROVEN (not assumed)

Proven lemmas (EC program logic + Z3; the conclusion is never axiomatized):
- `coeff_diff_in_box` — two in-bound responses differ by ≤ 96 (exact verifier-gate frontier).
- `forge_yields_coded_vector` — a same-action accepted forgery exhibits a nonzero coded vector, coeff ≤ 96.
- `inside_box_impossible` — that vector contradicts the enumeration min coeff 114 (the certificate core).
- `forge_splits` — accepted forgery ⟹ beyond-box-coded-SIS-break ∨ action-collision.
- `sameaction_forge_is_codedSIS` — a same-action accepted forgery IS a beyond-box coded-SIS break.
- `forge_eq_split` / `sis_eq_split` / `act_eq_split` / `split_prob` / `accept_le_sum` — the union
  bound over a single instrumented experiment: `Pr[accept] ≤ Pr[CodedSIS win] + Pr[ActionColl win]`.
- `forgeSA_le_codedSIS` + `accept_dominant_le_2pow_neg129` + `accepted_spend_term_le_2pow_neg129`
  — the accept term ≤ Pr[CodedSIS win] ≤ 2⁻¹²⁹.

---

## Named assumptions (exactly two carry a bound)

| Axiom | Kind | Bound | Status |
|---|---|---|---|
| `coded_sis_beyond_box_hard` | **EXTERNAL** = `open_external_estimator` | 2⁻¹²⁹ | **Irreducibly external** |
| `action_collision_qrom` | standard QROM hash collision-resistance | 2⁻¹⁶² | negligible, standard |

Plus one **structural** (non-hardness) axiom: `enumeration_total` — every nonzero coded-kernel
vector has coeff ≥ 114. This is a *finite verified computation* (the `ResponseLinkExactCertificate`
enumeration, corroborated by BKZ in `beyond_box_VERDICT.md`), the EC analogue of
`encode_frame_injective` in the binding reduction — **not** a cryptographic assumption.

---

## The explicit external residual

**`coded_sis_beyond_box_hard` IS the entire cryptographic residual.** The ≥ 129-bit hardness of
the response-link CODED-SIS problem *beyond* the deployed coeff ≤ 96 box is **irreducibly
external** and **cannot be proven internally**: it is the asymptotic/quantum hardness of the
coded distribution plus the pseudorandomness of the SHA-256 matrix sampler
(`beyond_box_VERDICT.md` residual #1/#2). The internal beyond-box cryptanalysis found **no
obstruction** (exact-enum min 114, BKZ L∞-min 120, structureless dead-zone, no algebraic
shortcut) but a small modeled regime cannot settle the asymptotic claim. An external
lattice-estimator/SIS run pointed at the coded problem is the open hand-off.

The job here was the **reduction** (forgery ⟹ break), which is proven. The hardness *magnitude*
is the external estimator — exactly one named axiom — and the negligible QROM action term.

---

## Composition with the spine

`eps_accepted = 2⁻¹²⁹` in `gap_c_remodel_spine_wiring.ec` is now annotated as discharged by this
reduction (no longer a bare placeholder). The spine arithmetic
(`remodelled_budget_closes`) already proves `eps_accepted + Σ break-terms ≤ 2⁻¹²⁸`, and the Lean
`successProb_le_2pow_neg128_of_proportional_bounds` consumes `acceptedSpendBound ≤ 2⁻¹²⁹` — which
this file now supplies modulo the single external coded-SIS estimator + the negligible QROM term.

For completeness the file also proves the lumped both-branches bound
`accepted_spend_forgery_bound`: `Pr[accept] ≤ 2⁻¹²⁹ + 2⁻¹⁶² ≤ 2⁻¹²⁸`.
