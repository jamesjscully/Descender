# Lean ↔ EC Seam Discharge (GAP B + GAP D)

Status date: 2026-06-10. Authoritative state: `SECURITY_DOSSIER.md`.
This document records the GAP-B discharge in `formal/easycrypt/spike_union_bound.ec`
(the union-bound spine) and the GAP-D fidelity-bridge reviewer checklist.

All claims here were re-verified this session: EasyCrypt 2026.05 (Nix store path),
Z3 4.16.0, `exit 0`; Lean source inspected. The spine and all 9 tracked
`formal/easycrypt/*.ec` files typecheck `exit 0`. No `admit`/`sorry`/`oops`
appears as a real tactic in the spine (only inside comment prose).

---

## 1. What changed (GAP B)

### Before
`spike_union_bound.ec` carried the OB-1 event coupling as a **probability
inequality axiom**, plus a re-stated copy of it for the closure section:

```
axiom success_implies_split &m :
  Pr[G.success() @ &m : res]
  <= Pr[G.accepted() @ &m : res]
     + big predT (fun b => Pr[G.break_(b) @ &m : res]) break_univ.
axiom success_le_union_c &m :   (* same statement, restated for CLOSURE section *)
  ...
```

This smuggled the entire union bound — the success/accept/break collapse **and**
the finite additivity over the 9 breaks — into one (actually two) imported
probability inequalities. The union-bound lemma `success_le_union` was then a
one-line `exact (success_implies_split &m)`: the "proof" was the axiom.

Real axioms before: **5** — `success_implies_split`, `success_le_union_c`,
`break_univ_uniq`, `break_univ_total`, `break_card9`.

### After
The three procedures are collapsed into **one experiment**: a single
distribution `dout : outcome distr` over outcomes, with `success`, `accepted`,
and per-break events modeled as **deterministic boolean projections** of the
outcome (`success_pred`, `accepted_pred`, `break_pred o b`). This is the exact EC
image of the Lean model, where the three indicators are functions of
`observe trace` (an `Outcome`).

The only residual import is the **smallest structural fact** — a *pointwise,
probability-free* event implication on outcomes:

```
axiom structural_split (o : outcome) :
  success_pred o => accepted_pred o \/ has (break_pred o) break_univ.
```

From this, `success_le_union` is now **PROVEN** in EC:

```
mu dout success_pred
  <= mu dout (accepted_pred \/ has break)     (* mu_le, from structural_split *)
  <= mu dout accepted_pred
     + mu dout (has break)                     (* mu_or  — finite sub-additivity *)
  <= mu dout accepted_pred
     + Σ_{b∈break_univ} mu dout (break_pred · b)  (* mu_has_le — iterated union *)
```

`mu_le`, `mu_or`, and `mu_has_le` are EasyCrypt library lemmas
(`Distr.ec`). The final real-arithmetic composition is closed by `smt`.

Real axioms after: **4** — `structural_split`, `break_univ_uniq`,
`break_univ_total`, `break_card9`.

### Net effect

| | before | after |
|---|---|---|
| real axioms (column-0) | 5 | 4 |
| union bound `success_le_union` | **axiom** (`exact`) | **PROVEN** (`mu_le`+`mu_or`+`mu_has_le`) |
| coupling carrier | 2 probability inequalities | 1 pointwise predicate implication |
| probability mass in the carried axiom | yes (it *was* a Pr-bound) | **none** (pure predicate fact) |

This is a genuine reduction, not a relabel: the carried obligation moved from a
probability inequality (which is what you are trying to *prove*) down to a
pointwise predicate implication (which is a structural-soundness fact), and the
entire probabilistic layer above it — sub-additivity and the 9-fold union —
is now machine-checked by EC + Z3 rather than asserted. The duplicate
`success_le_union_c` axiom is eliminated entirely (it is now a proven lemma with
the same derivation, re-instantiated at the closure section's `dout`).

---

## 2. The Lean ↔ EC correspondence (the seam)

The single carried axiom `structural_split` is the verbatim EC image of the Lean
**conclusion** of `success_implies_accepted_or_break_of_structural`
(`Descender/Path2AcceptedSpendClosure.lean:109`):

```
-- Lean conclusion (after the three faithfulness hyps are applied):
∀ trace, fullSignatureSuccess (observe trace) = true →
  acceptedSpendForgery (observe trace) = true ∨
    ∃ b, namedBreak b (observe trace) = true
```

EC:
```
∀ o, success_pred o => accepted_pred o \/ has (break_pred o) break_univ
```

Symbol map:

| Lean | EC |
|---|---|
| `Outcome` (= `observe trace`) | `type outcome` |
| `Path2FullSpendGame.BreakEvent` (`Fintype`, card 9) | `type break_t`, `break_univ` |
| `fullSignatureSuccess o` | `success_pred o` |
| `acceptedSpendForgery o` | `accepted_pred o` |
| `namedBreak b o` | `break_pred o b` |
| `∃ b, namedBreak b (observe trace)` | `has (break_pred o) break_univ` (via `break_univ_total`) |
| `traceDist : FiniteProbability Trace` pushed through `observe` | `dout : outcome distr` |
| `G.successProb` | `mu dout success_pred` |
| `G.acceptedSpendProb` | `mu dout accepted_pred` |
| `G.breakProb b` | `mu dout (fun o => break_pred o b)` |
| `successProb_le_acceptedSpendOrBreakUnionBound` | lemma `success_le_union` (PROVEN) |
| `breakProb_eq_zero_of_never` | lemma `break_zero_of_never{,_mu}` |
| `successProb_le_target_of_bounds` | lemma `success_le_target` |
| `successProb_le_2pow_neg128_of_proportional_bounds` | lemma `success_le_2pow_neg128` |

### Why `structural_split` is irreducible in EC

In Lean, `success_implies_accepted_or_break_of_structural` is **derived** — it is
not an axiom there either. Its derivation, in turn, rests on:

1. `account_creation_prevalidated_slot_authorization_or_break`
   (`Path2FullSpendGame.lean:197`), itself proven by 4-way case analysis over the
   four `_or_break` fields of `DirectAcceptedSpendSoundness`
   (`bound_slot_or_break`, `admitted_prechallenge_or_break`,
   `checked_core_binds_or_break`, `consume_once_or_break`);
2. three faithfulness hypotheses (`hSuccess`, `hAccepted`, `hBreak`) wiring the
   game's boolean indicators to the structural predicates.

Porting the *full* derivation into EC would require re-encoding the entire
`Predicates`/`DirectAcceptedSpendSoundness` structure and the four soundness
sub-lemmas in EC — which would faithfully reproduce the Lean argument but add no
machine-checking value, because each `_or_break` field is itself a structural
assumption (charged to a named break or to the verifier's accept condition). The
honest minimal seam is therefore the *conclusion* of the Lean derivation, carried
as one pointwise EC axiom. The probabilistic layer that genuinely needs an EC
proof (sub-additivity, 9-fold union) is what was previously hidden and is now
discharged.

**This is the precise correspondence a reviewer must accept:** `structural_split`
in EC ≡ the proven Lean theorem
`success_implies_accepted_or_break_of_structural` conclusion. Lean carries the
*proof*; EC carries the *statement* and proves everything probabilistic above it.
The two-tool split is intentional and is the seam.

---

## 3. GAP D — abstract ↔ EC fidelity bridges (reviewer checklist)

GAP D names two fidelity obligations flagged prose-only in
`creation_authorization_theorem_status.md §D`. Where formalizable as EC lemmas
they are; where genuinely cross-tool, the exact correspondence + the
human-checkable item is stated.

### D.1 Registry digest-equality model
File: `formal/easycrypt/break_prevalidation_registry_binding.ec`.

- **EC model:** acceptance at key `k=(accountId, accountStateRootBefore)` is
  `dig lf = t_k` where `t_k = dig (committed k)` is the honest registry handle
  fixed at slot creation. The break forces `lf <> committed k ∧ dig lf = t_k`, a
  **second-preimage** on `t_k` (GAP-C remodel: `eps ~ q/2^256`, no birthday
  square, no query cap).
- **Abstract (Lean):** `directAdmissionContains record accountId root digest leaf`
  — a `Prop` asserting the registry entry at `(accountId, root, digest)` matches
  the leaf-fixed committed value.
- **Bridge (formalizable, modeling-level):** the EC `dig lf = t_k` is the
  functional realization of `directAdmissionContains` when the registry maps each
  key to a unique committed leaf and the digest is the lookup handle. This is a
  *definitional* correspondence (predicate ≡ digest-equality at a digest-keyed
  entry), already discharged inside the EC file by construction; it is NOT a
  probability bridge.
- **Reviewer must confirm:** the production verifier
  (`production_reference.rs`, slot-authorization lookup) recomputes the presented
  leaf's authorization digest and compares it to the leaf-fixed committed digest
  at `(accountId, root)` — i.e. the registry handle is honest-fixed *before* the
  adversary acts (confirmed in
  `research/cryptanalysis/creation_time_binding_VERDICT.md`). If the production
  lookup instead trusted an adversary-supplied digest, the model would be a free
  collision (weaker) and the second-preimage bound would not apply.

### D.2 Hidden-source operational break
File: `formal/easycrypt/break_creation_time_hidden_source.ec`.

- **EC model:** the break is the operational image `!true_creation_honest`; the
  reduction `break_implies_bridge_win` requires the `accept` flag and exports the
  `verifier_exporter_bridge` win (52 inactive hidden-source columns excluded from
  the verifier-visible active projection).
- **Abstract (Lean):** `!AccountCreationAccepted` for the hidden-source soundness
  break (`creationTimeHiddenSourceSoundnessBreak`).
- **Bridge (genuinely cross-tool, prose):** `!true_creation_honest` (EC) ↔
  `!AccountCreationAccepted` (Lean) is a modeling identification, not a formal
  cross-tool theorem. The EC file proves `break ⇒ exporter-bridge win`; the Lean
  side names the same event in the 9-break alphabet.
- **Reviewer must confirm:** the `verifier_exporter_bridge` named assumption —
  the verifier's active projection does not leak the 52 inactive hidden-source
  coordinates — has real content (probe B in the prior per-event report built an
  inactive-column-only forger that falsifies a naive exporter), and it sits atop
  the already-proven relation-validity arm. The reviewer accepts the
  EC↔Lean event identification as a modeling boundary.

### D.3 The seam itself (GAP B residual) — the primary GAP-D item
- **Reviewer must confirm:** EC `structural_split` ≡ the conclusion of proven Lean
  `success_implies_accepted_or_break_of_structural`
  (`Path2AcceptedSpendClosure.lean:109`), which is derived from
  `account_creation_prevalidated_slot_authorization_or_break`
  (`Path2FullSpendGame.lean:197`) by case analysis over the four `_or_break`
  fields of `DirectAcceptedSpendSoundness`. The reviewer checks: (i) the Lean
  theorem is `qed` with no `sorry`; (ii) the EC statement is a faithful
  transcription (success⇒accepted∨break, pointwise, no probability); (iii) the
  three faithfulness conditions (`hSuccess`/`hAccepted`/`hBreak`) are reasonable
  modeling identifications between the game indicators and the structural
  predicates. Items (i)–(ii) are mechanical; (iii) is the accepted modeling seam.

---

## 4. Axiom census (spine, after discharge)

| axiom | kind | carries probability? | justification |
|---|---|---|---|
| `structural_split` | pointwise predicate implication | **no** | EC image of proven Lean structural soundness conclusion (the seam, §2) |
| `break_univ_uniq` | `Fintype` data | no | Lean `Finset.univ` nodup |
| `break_univ_total` | `Fintype` data | no | Lean `Finset.univ` complete |
| `break_card9` | `Fintype` card data | no | Lean `card = 9` by `decide` |

No axiom in the spine carries probability mass after the discharge. The union
bound, the never-firing `=0`, the budget arithmetic, and the concrete `2^-128`
closure are all PROVEN.

---

## 5. What remains beyond this spine (unchanged by GAP B/D)

These are out of scope for the seam discharge and tracked elsewhere:
- **GAP A** — `prevalidationDataAvailability` (#22) fail-closed-unbuilt; no EC
  reduction file, no accepting public verifier.
- **GAP C** — per-event numeric ε's wired into the spine via
  `gap_c_remodel_spine_wiring.ec` (registry remodeled to second-preimage).
- **GAP E** — accepted-spend `Pr[accepted] <= 2^-129` reduced to the external
  coded-SIS assumption (`accepted_spend_forgery_reduction.ec`); irreducibly
  external concrete-security margin.

The GAP-B discharge does not weaken or strengthen these; it only removes the
union-bound coupling axiom and replaces it with the minimal pointwise structural
fact, proving the probabilistic layer.
