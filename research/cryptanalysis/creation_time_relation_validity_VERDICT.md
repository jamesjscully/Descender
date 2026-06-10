# RISK-FIRST VERDICT — Creation-Time Hidden-Source RELATION-VALIDITY

**Date:** 2026-06-09 · **n=32, q=12289, Fq12** · **Role:** prove-OR-break of `creation_time_hidden_source_soundness_break`
**Predecessor:** `creation_time_binding_VERDICT.md` (binding = SOUND; risk relocated to relation-validity + data-availability)
**Target verifier:** `verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate`
(`production_reference.rs:38517`; candidate verifier `:38453`)

---

## VERDICT: the soundness certificate REJECTS the relation-invalid opening.
## Classification: **SOUND (modulo named assumptions) for the relation arm, with one CONDITIONAL-GAP residual: data-availability.**

The decisive attack — a creation certificate whose account-witness opening is UNIQUE and repacks
consistently to the leaf-fixed `account_witness_commitment` (PASSES binding) but is **relation-INVALID**
— is **REJECTED** by the real verifier, including the strong form where the forger recomputes a
**self-consistent carrier binding** for the forged projection lanes. There is **no
`creation_time_hidden_source_soundness_break`**. No redesign of the relation arm is required.

The remaining open item is **NOT** relation soundness. It is **data-availability**: the public,
harness-free creation-acceptance payload (`verify_path2_direct_creation_proof_payload_v1`) is
**fully fail-closed**; only the **harness-input replay** path (`…relation_replay`) enforces the
relation today. That is exactly the project's intended fail-closed posture, and it is the precise
piece that must be built to make creation accepting from public data alone.

---

## THE ATTACK — construction, commands, output

**Probe file:** `tools/parameter-search/tests/creation_time_relation_validity_probe.rs`
**Command:** `cargo test --test creation_time_relation_validity_probe` (run from `tools/parameter-search/`)
**Result:** `4 passed; 0 failed`.

### Why no free relation-invalid value exists (the mechanism, line-checked)

The 6-lane `account_projection` is **NOT a prover-supplied free value**. The verifier **derives** it
from the leaf-fixed commitment:

- `reviewed_fq12_masked_state18_packed_witness_opening` (`:6528`):
  `account_projection[i] = (masked_projection[i] − mask[i]) mod q`, where `masked_projection` is read
  out of the leaf-fixed commitment residues (`unpack_reviewed_fq12_packed_witness_state18`) and
  `mask` is a PRF of `(pad_seed, slot_id)`. It then **repacks and asserts `repacked == commitment`,
  fail-closed** (`:6556`).
- The relation predicate `verify_hidden_source_fallback_opening_projection_or_export` (`:5740`)
  requires **per lane**: `decoded.lanes[lane] == claimed == recomputed`, where `recomputed` comes
  from `hidden_source_fallback_projection_values_from_hidden_material(account_projection, pad_seed)`
  — the SAME derived value. Any mismatch routes to `classify_…_opening_break` → **Export**, which the
  carrier-tuple wrapper (`:7020`) converts to a hard **Err** ("…exported a hidden-material break").

So a forger has no slot to inject a relation-invalid projection: it is fixed by the commitment.

### Construction A — claimed-lane mutation (`relation_invalid_claimed_lane_mutation_is_rejected`)
Mutate one CLAIMED carrier projection lane away from the relation-derived value, commitment/opening
unchanged (binding still holds). → **REJECTED.**

### Construction B — the decisive one: internally-consistent relation-invalid carrier
(`relation_invalid_internally_consistent_carrier_is_rejected`)
Forge claimed lanes (`derived[0]+1`) AND **recompute a self-consistent carrier binding**
(`reviewed_fq12_state_assisted_projection_carrier_binding`) for the forged lanes, bound to the
**unchanged** `opening_commitment` + opening + statement. This **defeats the carrier check-bytes /
transcript-binding guard** (those now match the forged lanes) while the claimed projection is
relation-INVALID. It PASSES binding and PASSES carrier-binding. → **REJECTED** at the
`claimed == recomputed` lane-equality predicate, before the carrier-binding check is even reached.
- Exact rejection reason (captured `--nocapture`):
  `reason="reject_fail_closed_invalid_projection_carrier" slot_id=Some(0)`.

### Honest control (`honest_fixture_soundness_certificate_accepts`)
The honest fixture is **ACCEPTED**; the certificate's `packed_witness_opening.account_projection`
equals the relation-derived value `[1,2,3,4,5,6]` — confirming the projection on the certificate is
the derived (not prover-chosen) quantity.

---

## (A) COVERAGE — verified: active lane relation IS enforced; 52 inactive coords cannot carry a forgery

- The soundness certificate runs the **checked-core** (`verify_reviewed_fq12_state_assisted_checked_core`,
  `:38196`) over the **12 active syndrome lanes** via the sigma equation
  `actual_response_syndrome == expected_response_syndrome(challenge, witness_commitment, proof_commitment)`
  (`:38243`). The 12 active lanes ARE the checked-core `witness_syndrome` (the first
  `REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS` residues of the same leaf-fixed commitment) — bound,
  not free.
- Cross-check of the active-subspace replay JSON
  (`signature_readiness_20260516/hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json`):
  `active_projection_rank = 12`, **`active_projection_nullity = 0`**, `bridge_semantics_closed = true`,
  `exact_certificate_complete_for_active_subspace = true`. The prior bounded survivor is
  `classification = invalid_for_active_subspace_relation` / `in_active_subspace = false` — it
  "mutates an inactive hidden-source coordinate that is not verifier-visible". Status:
  `active_subspace_exact_replay_closed_inactive_survivor_excluded_fail_closed`.
- **Assumption that remains (named, not novel):** the **verifier-exporter bridge** — that the 52
  inactive source columns are genuinely non-public hidden material excluded from the
  verifier-visible active projection — is asserted by the replay artifact
  (`bridge_semantics_closed`), not re-derived in this probe. Nullity-0 over the 12 active lanes is
  the C3 fix (re-formalized + proven per the assumption ledger); the 52-column exclusion rests on
  that bridge. This is the residual to keep in the dossier; it is not a binding/relation survivor.

---

## (B) DATA-AVAILABILITY — CONDITIONAL-GAP, precisely characterized

Two creation-relation paths exist:

1. **WORKS (harness-input replay):** `verify_path2_prevalidated_slot_creation_relation_replay`
   (`:26360`) re-runs the soundness certificate with the **full hidden harness inputs**
   (`public_key, message, account, signature_with_carrier, opening, statement, pad_seed`) and binds
   its outputs (`opening_commitment`, `pad_seed_digest`, `statement_digest`, `slot_id`) to the
   prevalidated leaf. Tests `path2_creation_relation_replay_accepts_…` /
   `…rejects_mutated_relation_inputs` pass. **This enforces the relation — but only when the hidden
   inputs are available to the replayer.**

2. **FAIL-CLOSED (public, harness-free payload):** `verify_path2_direct_creation_proof_payload_v1`
   (`:26475`) is the public-data dispatcher (binding + payload digests only, NO harness inputs).
   Every branch returns an error:
   - `PATH2_DIRECT_CREATION_PROOF_KIND_UNINSTANTIATED` → "verifier is not implemented"
   - `PATH2_DIRECT_CREATION_PROOF_KIND_TRANSPARENT_DESCENT` → "verifier is not implemented"
   - otherwise → "payload kind is unsupported"

   `verify_path2_prevalidated_slot_creation_payload_bindings` (`:26283`) additionally **requires** a
   `Some(certificate)` (else "Path 2 evidence payload unavailable") and checks the certificate digest
   + 9 leaf-context bindings — i.e. the frame bindings are implemented and tight, but they bind a
   certificate **digest**, they do not by themselves replay the relation.

3. **PRESENT-BUT-NOT-WIRED (toy):** `verify_path2_direct_creation_toy_q_linear_proof_v1` (`:26653`)
   IS a real public-data Sigma verifier: it checks, from `binding` + `payload` + a public `proof`,
   the masked-projection relation `pad_lane[i] + projection_lane[i] == challenge · masked_projection[i] (mod q)`
   for all 6 lanes plus two commitment equations. **But (a) it is explicitly a TOY q-linear relation,
   not the real hidden-source relation, and (b) it is referenced ONLY from tests (`:52412`+), never
   from the production dispatcher `verify_path2_direct_creation_proof_payload_v1`.**

**What must be built to make creation ACCEPTING from public data:** a public, harness-free creation
proof (instantiate `PATH2_DIRECT_CREATION_PROOF_KIND` with a real, non-toy verifier wired into the
`:26475` dispatcher) that proves the same lane-equality relation the harness-replay enforces, over
public payload data, and is bound to the leaf via the existing frame/digest bindings (already
implemented). Until then, creation acceptance is intentionally fail-closed and depends on holding the
hidden harness inputs at replay time. This is `prevalidation_data_availability_break` — a build item,
**not** a soundness break.

---

## MANDATORY REGRESSION — landed and PASSING

`opening_commitment_changes_under_syndrome_only_mutation` (in the same probe file) asserts (not just
prints) that a **syndrome-only** witness mutation changes `opening_commitment` — guarding the proven
binding against a refactor that drops `account_witness_commitment` from the opening frame (which would
reopen the 12-dim syndrome-lane nullity survivor). PASSES. (The pre-existing
`creation_time_binding_rank_probe.rs` `probe3` exercised the commitment-distinctness but only at the
witness-commitment level; this adds the explicit syndrome-only / opening_commitment-frame assertion
the binding verdict called mandatory.)

---

## SUMMARY

| Item | Result |
|---|---|
| Relation-invalid opening (claimed-lane) | **REJECTED** |
| Relation-invalid opening (self-consistent carrier, passes binding + carrier-binding) | **REJECTED** (`reject_fail_closed_invalid_projection_carrier`) |
| Honest fixture | ACCEPTED; certificate projection = derived value |
| `creation_time_hidden_source_soundness_break` | **NOT REAL** — relation arm SOUND modulo named assumptions |
| (A) Active-lane (12) relation enforced | YES (checked-core sigma + nullity-0 replay) |
| (A) 52 inactive cols | Excluded via verifier-exporter bridge (named assumption, `bridge_semantics_closed`) |
| (B) Harness-replay relation path | IMPLEMENTED + tested |
| (B) Public harness-free creation payload | **FAIL-CLOSED** (UNINSTANTIATED/TRANSPARENT not implemented; toy verifier present but unwired) |
| Mandatory binding regression | LANDED + PASSING |

**Single biggest finding:** the relation is **not** carried by any prover-chosen value — the 6-lane
`account_projection` is *recomputed from the leaf-fixed commitment* and checked `claimed == recomputed`
per lane; even a forger who recomputes a self-consistent carrier binding for forged lanes is rejected.
The risk that remains is purely **data-availability** (build the public creation payload), not soundness.

**Next concrete action:** implement and wire a **non-toy** public creation-proof verifier into
`verify_path2_direct_creation_proof_payload_v1` (`:26475`) that replays the masked-projection
lane-equality relation over public payload data (the toy q-linear verifier at `:26653` is the shape
to harden + de-toy + wire), then prove it equivalent to the harness-replay relation. Secondary:
formalize the verifier-exporter bridge (52-inactive-column exclusion) as a named, checked assumption
in the dossier.

**DO NOT git commit — left for maintainer review.**
