# Creation-Authorization Theorem Status

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This is an older theorem-status snapshot with useful evidence and explicit
> supersession notes. It is not top-level status; current top-level status
> belongs in `TRACKER.md` and the active security claim shape in
> `spec/security_model.md`.

> **SUPERSEDED (2026-06-10) — see `SECURITY_DOSSIER.md` for the authoritative state.**
> Early-2026-06-10 snapshot. Since written: the 3 binding breaks were re-modeled from
> free-birthday COLLISION-resistance to the tighter SECOND-PREIMAGE resistance (GAP C,
> `gap_c_remodel_spine_wiring.ec`; budget closes at q=2⁶⁴ with no query cap); GAP C is
> WIRED; GAP A is BUILT (`verify_path2_direct_creation_masked_state18_proof_v1`, proven
> equivalent to the harness relation, production still fail-closed); GAP E (accepted-spend
> ≤2⁻¹²⁹) is reduced to the external coded-SIS assumption. Where this doc says
> "collision-resistance", "GAP A unbuilt", or "GAP C unwired", the dossier + code win.

Target theorem: `account_creation_prevalidated_slot_authorization_or_break`
(spec: `research/account_creation_prevalidated_slot_authorization_or_break.md`).

Status date: 2026-06-10. All claims below were **independently re-verified** in
this assessment (EasyCrypt 2026.05 via Nix store path, Z3 4.16.0; Lean source
inspection). This document supersedes prior per-event reports only as a summary;
the per-file proof obligations live in the `.ec` files cited.

---

## TL;DR

The full theorem is **NOT yet closed**, but the gap is narrow and named.

- **8 of 9** named breaks are accounted for (5 reduced-to-named-assumption or
  proven-impossible cryptographic events + 2 structural `=0` + 2 classifier-`False`).
- The **single remaining gap** is `prevalidation_data_availability_break`
  (= build-item #22): it is **fail-closed-but-unbuilt**. There is no EC reduction
  file and no accepting public verifier; the active path's data-availability arm
  is held uninhabited by design, not proven secure.
- Additionally, the union-bound spine `spike_union_bound.ec` carries the
  event-coupling step (OB-1, `success_implies_split`) as an **axiom**, not a
  proven implication, and the **per-event numeric bounds are not yet wired** from
  the EC reduction files into the spine. So even granting #22, the end-to-end
  composition is assembled but not fully discharged.

**Honest verdict: theorem holds MODULO the named standard assumptions ONLY for
the 8 covered breaks. It is NOT closed end-to-end because (a) data-availability
break #22 is unbuilt/fail-closed and (b) the spine's per-event numeric inputs and
OB-1 coupling are still imports, not proven from the per-event EC files.**

---

## (1) Events genuinely reduced (proven implication + named standard axiom, verify = PASS)

Independently re-confirmed: all four files typecheck `exit 0`; every `admit`/
`sorry`/`oops` hit is inside comment prose (no real tactic); axiom census matches
the reports. The named hardness axiom in each case bounds the **game-win flag**,
not the break flag, and sits **after** the proven `break => game-win` reduction
lemma in source order, so the reduction cannot depend on the hardness axiom.

| event | EC file | proven implication | named standard assumption | structural (non-hardness) axioms |
|---|---|---|---|---|
| `checkedCoreMessageBinding` | `formal/easycrypt/break_checked_core_message_binding.ec` | `break_yields_collision` (qed) → `break_le_cr` (byequiv, qed) | `sha256_collision_resistance` (truncated+masked checked-core width); SHA-256 CR, NIST FIPS 180-4 | `encode_frame_injective` (byte-framing injectivity) |
| `prevalidationLeafBinding` | `formal/easycrypt/break_prevalidation_leaf_binding.ec` | `break_implies_collision` (hoare, qed) → `break_le_cr` | `sha256_collision_resistance` on `sha256(leaf_frame l)` | `leaf_frame_injective` (fixed-width offset encoder) |
| `prevalidationRegistryBinding` | `formal/easycrypt/break_prevalidation_registry_binding.ec` | `run_break_implies_collision` → `B_wins_when_break` → `break_le_cr` (byequiv) | `sha256_collision_resistance` on slot-authorization digest | `eps_cr_ge0` (advantage non-negativity) |
| `creationTimeHiddenSource` | `formal/easycrypt/break_creation_time_hidden_source.ec` | `break_implies_bridge_win` (needs `accept`; admit appears only if `accept` dropped) | `verifier_exporter_bridge` (52 inactive hidden-source columns excluded from verifier-visible active projection) | `q_prime`, `oldCols_val`, `activeLanes_val`, `col_range`, `coef_nonzero` (all checkable certificate DATA) |

Verification commands re-run this session (all from `/home/jack/code/Descender`):

```
EC=$(ls /nix/store/*easycrypt*/bin/easycrypt|head -1)   # easycrypt-2026.05
"$EC" formal/easycrypt/break_checked_core_message_binding.ec     -> EXIT 0
"$EC" formal/easycrypt/break_creation_time_hidden_source.ec      -> EXIT 0
"$EC" formal/easycrypt/break_prevalidation_leaf_binding.ec       -> EXIT 0
"$EC" formal/easycrypt/break_prevalidation_registry_binding.ec   -> EXIT 0
```

These four are **genuine family-(b) game-based reductions, not faked, not vacuous**.
The adversarial mutation tests in the prior per-event reports (tamper of the CR
win-condition / dropping the collision submit / dropping `accept`) each break the
reduction proof, demonstrating the reductions are load-bearing. Re-confirmed
consistent here. Caveats stand and are disclosed in the files themselves:

- `prevalidationRegistryBinding`: the fidelity between the abstract Lean
  `DirectAdmissionContains` (a `Prop`) and the EC "acceptance = digest-equality at
  a digest-keyed registry entry" model is argued in comments, not formally bridged.
- `creationTimeHiddenSource`: the EC break is the operational image
  `!true_creation_honest`; the link to Lean `!AccountCreationAccepted` is modeling
  prose, not a cross-tool bridge.
- `verifier_exporter_bridge` is a *named standard assumption* (exporter does not
  leak verifier-invisible coordinates), shown to have real content (probe B in the
  prior report constructed an inactive-column-only forger that falsifies it), and
  it sits atop the already-proven relation-validity arm.

---

## (2) Coverage of the 9 named breaks

The theorem's break alphabet is `Path2FullSpendGame.BreakEvent` (9 constructors;
`Fintype` card = 9). Mapping to the active concrete classifier
`Path2FullSpendConcrete.ConcreteBreakFor`:

| # | named break | family | status |
|---|---|---|---|
| 1 | `creationTimeHiddenSourceSoundness` | (b) reduction | **REDUCED** — `verifier_exporter_bridge` |
| 2 | `prevalidationLeafBinding` | (b) reduction | **REDUCED** — SHA-256 CR |
| 3 | `prevalidationRegistryBinding` | (b) reduction | **REDUCED** — SHA-256 CR |
| 4 | `checkedCoreMessageBinding` | (b) reduction | **REDUCED** — SHA-256 CR (checked-core width) |
| 5 | `slotReplayOrRollback` | (c) structural | **=0 PROVEN** — `break_slot_replay_or_rollback.ec`, 0 axioms |
| 6 | `postchallengeSlotCreationOrRewrite` | (c) structural | **=0 PROVEN** — `break_postchallenge_slot_creation_or_rewrite.ec`, 0 axioms |
| 7 | `qromOrDomainSeparation` | classifier | **=0** — `ConcreteBreakFor … := False` (`Path2FullSpendConcrete.lean:454`); never-fires → spine Goal 2 |
| 8 | `liveSlotLeakageOrKeygen` | classifier | **=0** — `ConcreteBreakFor … := False` (`Path2FullSpendConcrete.lean:456`) |
| 9 | `prevalidationDataAvailability` | (b)/availability | **OPEN** — build-item #22, fail-closed-unbuilt; no EC file |

So **8 of 9 are covered** (5 cryptographic events at #1–6 plus the two
classifier-`False` events #7–8). The structural `=0` files carry **zero axioms**
(re-confirmed: `grep -c '^\s*axiom'` = 0 for both).

Note on #7/#8: their concrete break predicate is literally `False`, i.e. the
active concrete path does not model these as reachable; their probability is `0`
via the spine's `break_zero_of_never` (Goal 2). This is honest *for the active
concrete path* but means the QROM / domain-separation and live-slot-leakage
hardness surfaces are **discharged by the classifier choice, not by a reduction**.
Anyone reading "QROM is a named standing assumption" should know it is currently
borne at the modeling boundary (classifier sets the event to `False`), backed by
the assumption ledger (`research/path2_hash_qrom_assumption_ledger.md`), not by an
EC QROM reduction.

---

## Named standing assumptions the theorem would rest on (modulo which it closes for the 8 covered events)

1. **SHA-256 collision-resistance** (NIST FIPS 180-4) — bears events #2, #3, #4
   (truncated+masked to checked-core width for #4). Single named hardness axiom in
   each EC file; the `break => CR-win` implication is proven.
2. **Verifier-exporter bridge** (`bridge_semantics_closed`) — bears #1; the 52
   inactive hidden-source projection columns cannot carry an accepted forgery.
   Sits atop the already-proven relation-validity arm.
3. **QROM / domain-separation hardness** — *nominally* bears #7, but in the active
   concrete path #7 is `False`; the assumption is carried at the modeling boundary
   + assumption ledger, **not** by an EC reduction.
4. **SHA-256 frame-PRF / mask-PRF** — the projection-mask and pad-source PRF
   assumptions in the ledger; relevant to #8 (`liveSlotLeakage`), which is also
   `False` in the active classifier.
5. **External coded-SIS / lattice estimator** (`open_external_estimator`) —
   irreducibly external; the keygen-SIS / response-link concrete-security margin.
   This is NOT inside any EC break file; it is the standing external-cryptanalysis
   assumption for forgery hardness of the underlying scheme.

Structural (non-hardness) axioms — byte-encoder injectivity and checkable
certificate DATA (`encode_frame_injective`, `leaf_frame_injective`, `q_prime`,
dimension/`coef_nonzero` facts, `eps_cr_ge0`) — are legitimate and do not carry
probability mass.

---

## (3) What EXACTLY remains before the theorem is closed (each gap named)

**GAP A — `prevalidation_data_availability_break` is fail-closed-unbuilt (build-item #22).**
This is the decisive gap. `ConcreteBreakFor … prevalidationDataAvailabilityBreak
:= ¬ ReplayableCertificateDataAvailable` (`Path2FullSpendConcrete.lean:447`), and
`ReplayableCertificateDataAvailable` requires `directCertificate.dataAvailable ∧
admissionRecord.canonicalAt`. There is **no** EC reduction file for this event and
**no** accepting public creation-proof verifier — the dispatcher at
`production_reference.rs:26475` is fail-closed (rejects all). Per the tracker
(`account_creation_prevalidated_slot_proof_tracker.md:762`, master tracker line
473), build-item #22 = wire + de-toy a non-toy public verifier (`:26653`) and
prove it equivalent to the tested harness-replay relation (`:26360`). Until that
ships, this break is neither reduced nor proven-impossible; it is held uninhabited
by the fail-closed scaffolding. **The theorem cannot be claimed closed while #22
is unbuilt.**

**GAP B — the union-bound spine's event-coupling (OB-1) is an axiom, not proven.**
`spike_union_bound.ec` imports `axiom success_implies_split` (the
`success => accepted ∨ ⋁ break` coupling, EC line ~129). In Lean this IS derived
(`success_implies_accepted_or_break_of_structural`), but the EC spine keeps
`success`/`accepted`/`break_` as three separate procedures, so the coupling is
out of scope there and carried as an axiom (also `break_univ_uniq`,
`break_univ_total`, `break_card9`). The union-bound *assembly* (Goals 1/3) and the
never-firing `=0` (Goal 2) are proven; the coupling that feeds it is not, in EC.

**GAP C — per-event numeric bounds are not yet wired into the spine.**
The four reduced EC files prove `Pr[break] <= eps` for their event, but
`spike_union_bound.ec` treats each `Pr[break b]` abstractly and composes them via
`successProb_le_2pow_neg128_of_proportional_bounds` (each break `<= 2^-133`,
accept `<= 2^-129` ⟹ `<= 2^-128`). The concrete `eps_cr` / bridge bounds from the
per-event files are **not substituted** into the spine, and there is no proof that
`sha256_collision_bound <= 2^-133` at the deployed challenge width. So the
numeric closure to `2^-128` is assembled symbolically but the per-event ε's are
not yet plugged with their reduction-file values.

**GAP D — abstract-vs-EC fidelity bridges (disclosed, not blocking-but-not-closed).**
The registry-binding model (`acceptance = digest-equality`) and the
hidden-source operational break model (`!true_creation_honest`) are linked to
their Lean predicates by prose, not by a formal cross-tool bridge. This is the
standard EC-modeling seam and is disclosed in-file, but it is a genuine fidelity
obligation that an external reviewer would want discharged or accepted explicitly.

**GAP E — active deployment path remains fail-closed overall.**
Per project memory, `SelectedRust*Certificate` types are provably uninhabited
until the AP2-N003 accepted-spend numeric bound (`PricedAcceptedSpendNumericCertificate`)
is supplied. The accepted-spend probability term `Pr[accepted] <= 2^-129` that the
spine consumes is the WS7 numeric obligation, still external, not yet
discharged with the real EUF-CMA trace model.

---

## Bottom line

- **Genuinely reduced (proven implication + single named standard axiom, verify =
  PASS, not faked):** `checkedCoreMessageBinding`, `prevalidationLeafBinding`,
  `prevalidationRegistryBinding`, `creationTimeHiddenSource`.
- **Proven structural `=0` (zero axioms):** `slotReplayOrRollback`,
  `postchallengeSlotCreationOrRewrite`.
- **`=0` by classifier (`False`), assumption borne at modeling boundary:**
  `qromOrDomainSeparation`, `liveSlotLeakageOrKeygen`.
- **OPEN / fail-closed-unbuilt:** `prevalidationDataAvailability` (build-item #22).

The set {2 structural =0} ∪ {4 reduced binding/QROM} ∪ {2 classifier-False} covers
**8 of 9** named breaks. It does **NOT** cover all 9, because
`prevalidationDataAvailability` (#22) is still fail-closed-unbuilt. Therefore the
full theorem `account_creation_prevalidated_slot_authorization_or_break` is **NOT
closed modulo standard assumptions**. The honest claim today is:

> Modulo {SHA-256 CR, verifier-exporter bridge, QROM/domain-separation,
> SHA-256/mask PRF, external coded-SIS estimator}, the spend authorization
> implication holds for 8 of the 9 named breaks. It is NOT closed end-to-end:
> (A) the data-availability break is fail-closed-unbuilt (build-item #22),
> (B) the EC spine's event-coupling OB-1 is still an axiom, and
> (C) the per-event numeric ε's from the reduction files are not yet wired into
> the union-bound spine's `2^-128` target.

Do not represent the theorem as closed until at minimum GAP A (#22) ships an
accepting, proven-equivalent public verifier and GAPs B–C wire the proven
per-event bounds into the spine.
