# JASMIN-SCOPE — Smallest Deployable Verifier Core (scope + effort estimate)

**Date:** 2026-06-09
**Type:** SCOPING + EFFORT-ESTIMATE only. **No Jasmin spike code is in this
deliverable** and none should be written yet (see §5 Sequencing).
**Unblocked by:** EC-SPIKE → **GO (qualified)**
(`formal/easycrypt/SPIKE_VERDICT.md`; `VERIFICATION_PLAN.md` Phase-1). EasyCrypt
is adopted for the reduction spine, and — decisively for this layer — Jasmin's
correctness proof is realized through the Jasmin→EasyCrypt embedding (the
Formosa / ML-KEM pipeline). That is why scoping Jasmin only becomes meaningful
now: the equivalence *target language* exists.

**Status of this layer:** not started. This document picks the first core to
implement and estimates it. It does **not** claim any constant-time property, any
proof, or any compiled artifact — those are Phase-2 work, gated per §5.

---

## 0. Anchoring numbers (from the spec, do not drift)

All from `formal/spec/descender-signature-spec.md` §1–§4 unless noted.

| Quantity | Value | Anchor |
|---|---|---|
| `q` | 12289 (14-bit prime, `FIELD_BITS=14`) | spec §1 |
| extension field | `Fq12 = F_q[X]/(X¹²+X+25)`, verified irreducible | spec §1 |
| irreducible poly (low→high) | `[25,1,0,0,0,0,0,0,0,0,0,0,1]` (monic, deg 12) | `Spike_core.fst:24` |
| SIS public rows | 12 | spec §1 |
| Public key | **172 B** = `seed[32] ‖ payload[140]` | spec §2 |
| Signature (deployed) | **314 B** = `checked_core[258] ‖ projection_carrier[56]` | spec §2 |
| `checked_core[258]` | `slot_id[4] ‖ challenge[27] ‖ active_response[52] ‖ local_openings[104] ‖ proof_commitment[32] ‖ masked_response[39]` | spec §2 |
| `projection_carrier[56]` | `projection_lanes[24] ‖ check_bytes[3] ‖ transcript_binding[29]` | spec §2 |
| syndrome packing | 12 lanes × 14 bits = 168 bits in a 32-B digest; top 256−168 = 88 bits MUST be zero (canonical) | `Spike_core.fst:18–22` |

**Known discrepancy to resolve before any code (flagged, not hidden):**
`formal/kat/kat_v0.json` declares `signature_bytes: 260` with layout
`slot_id[4] ‖ challenge[27] ‖ response[52] ‖ openings[104] ‖ linear_sigma_proof[73]`
— i.e. the **legacy linear-sigma path** that KAT v0 pins, *not* the 314-B
`checked_core[258] ‖ projection_carrier[56]` spec target. `VERIFICATION_PLAN.md`
Phase-0 already records this: KAT v0 "pins the CURRENT deployable linear-sigma
path that still wraps the LEGACY proof … MUST be regenerated when that path is
hardened." **The Jasmin core must target the 314-B spec layout, and KAT
conformance is against a regenerated KAT (v1) for that layout — not byte-for-byte
against today's 260-B `kat_v0.json`.** Conforming to the stale 260-B vectors
would bake the legacy path into verified assembly. This dependency is called out
in §4 and §5.

---

## 1. Smallest deployable verifier core (v0 of the Jasmin layer)

The first thing to write in Jasmin is the **arithmetic checked-core of VERIFY**:
the constant-time sigma-equation check plus the deserialize path that feeds it,
reusing the exact decomposition the hax spike already validated
(`formal/hax/extracted/Spike_core.fst`). This mirrors `spec` §4 Verify steps 1
and 3 (parse canonically; sigma equation `z = a + action(c)·w` over Fq12).

### In scope for Jasmin v0

1. **Byte parse / deserialize with canonical-encoding checks.**
   - Slice the 314-B signature into the §2 fields and the 172-B pk into
     `seed[32] ‖ payload[140]`. Fixed offsets, no secret-dependent indexing.
   - **Syndrome lane unpack** (`unpack_…_syndrome_digest`, `Spike_core.fst:366`):
     for each of the 12 lanes, read 14 LE bits from a 32-B digest
     (`read_bits_le`), reject if any lane `≥ q`, and reject if any of the 88
     slack high bits are set (`any_bits_set_le`). This is the canonical-encoding
     gate; it is the single most security-relevant parse check (non-canonical
     encodings are a malleability surface under the SUF-CMA / D2 goal).
   - Length check (reject wrong total length, cf. KAT `neg_noncanonical_truncated`).

2. **Fq12 arithmetic over `F_q[X]/(X¹²+X+25)`** — the four ops the hax core
   factored out, all `[u16;12]` / `Vec<u16>`-shaped, all first-order, fixed width:
   - `add_mod` / `sub_mod` mod q (`Spike_core.fst:35,43`) — field ±, result `< q`.
   - **`poly_mul_mod`** schoolbook convolution + monic reduction mod the
     irreducible poly (`Spike_core.fst:138`). This is the one nontrivial routine
     (see §2, §4); the HAX-SPIKE flagged it as "the one lemma needing real work."
   - `fq12_syndrome_add` / `fq12_syndrome_mul` / `fq12_syndrome_normalize`
     (`Spike_core.fst:312,338,359`) — the 12-coefficient wrappers.

3. **Sigma-equation evaluation + syndrome equality compare.**
   - Compute the RHS `a + action(c)·w` exactly as the target
     `reviewed_fq12_account_carried_expected_response_syndrome`
     (`Spike_core.fst:480`): unpack witness `w` and randomizer `a` from their
     digests, evaluate `fq12_syndrome_add(a, fq12_syndrome_mul(action(c), w))`.
   - **`action(c)`** is the SHA-256 hash-to-field random oracle (spec §3). As in
     the hax spike, v0 treats it as an **opaque total map `challenge → Fq12`**
     (`action_oracle`, `Spike_core.fst:457`). Hashing SHA-256 in Jasmin is a
     separate, well-trodden Formosa component and is **out of scope for v0** —
     v0 takes the 12-lane action vector as an input.
   - **Constant-time syndrome equality compare** of the 12-lane RHS against the
     parsed response: accumulate per-lane XOR into one word, branch once on the
     aggregate (no early-out, no per-lane branch).

### Out of scope for Jasmin v0 (explicitly deferred)

- **Keygen and signing** (spec §4 Keygen/Sign) — verifier-first; signing touches
  the genuine secrets and is a much larger CT surface.
- **The 56-B `projection_carrier`** (`projection_lanes ‖ check_bytes ‖
  transcript_binding`) and its binding/anti-replay checks (spec §4 Verify step 4)
  — note spec §6 records the carrier proves only a *projection relation*, not a
  full witness opening; deferring it avoids encoding an unsettled spec question.
- **Account-tree / consume-once lifecycle** (slot freshness, replay/rollback,
  the successor transition; spec §4 Verify steps 2,5; KAT `neg_replayed_slot`,
  `neg_stale_account_root`). State-machine logic, not field arithmetic.
- **`local_openings` fiber/pad consistency** (KAT `neg_mutated_response`,
  `neg_mutated_carrier_*`) — Verify step 4; layered on after the core.
- **SHA-256 / `uniform_mod_from_hash` rejection sampler** — supply `action(c)`
  as input; reuse a Formosa SHA-256 later.

v0 = "given parsed public inputs, decide the Fq12 sigma equation in constant
time, with canonical-encoding rejection." That is the smallest unit that is both
deployable-shaped and has a clean EasyCrypt equivalence target.

---

## 2. Constant-time obligations

**The verifier is almost entirely a function of PUBLIC data.** Inputs to v0 —
signature bytes, public key, challenge/action vector, the response and digests —
are all attacker-known. The forgery game gives the adversary these already, so a
timing side-channel here leaks nothing secret. Therefore for the **v0 verifier
core the CT requirement is largely NOMINAL**, not a binding secrecy boundary.

We still implement it CT, for three concrete reasons, and one of them is binding:

- **Binding (genuine):** the `action(c)·w` multiply and the lane unpack consume
  digests (`witness_commitment`, `proof_commitment`) that are *commitments to
  secret witness material*. Even though their bytes are public in the transcript,
  the deployable verifier is reused in contexts (batch verification, signer-side
  self-verification before publishing) where the same routine runs on
  not-yet-public commitments. A data-independent `poly_mul_mod` and a
  data-independent lane unpack are the cheap insurance that keeps the core safe
  under that reuse. This is the one place to treat CT as load-bearing.
- **Hygiene / D2 conservatism (nominal but required by policy):** the project's
  D2 stance (`formal/jasmin/README.md`) makes CT a first-class obligation on
  *all public-data-dependent rejection paths*. The Jasmin→EasyCrypt CT checker is
  cheap to satisfy here and gives a uniform "no secret-dependent control flow"
  certificate across the whole verifier regardless of which inputs are secret.
- **Future signer reuse:** the same `add_mod`/`sub_mod`/`poly_mul_mod` primitives
  will be lifted into the (secret-touching) signer in a later phase; building them
  CT now avoids a rewrite.

### Concrete CT rules for the implementation

- **q = 12289 modular reduction without data-dependent branches or division.**
  Use **Montgomery or Barrett** reduction on the `u32` products, matching the
  ML-KEM/Formosa idiom (Kyber's q = 3329 is the same shape; the techniques
  transfer directly). The hax extraction currently uses a literal `%! q` on `u32`
  intermediates (`Spike_core.fst:36,198,290`); that is fine as the *functional
  reference*, but the Jasmin code must replace `%` with a constant-time Barrett/
  Montgomery sequence (no hardware `div`/`rem`, whose latency is data-dependent on
  some targets). The functional-equivalence proof (§3) is exactly what licenses
  swapping `%! q` for the Barrett/Montgomery form.
- **No early-out compares.** The syndrome equality and the canonical-bit checks
  (`any_bits_set_le` short-circuits in the Rust reference) must be rewritten to
  accumulate over all lanes/bits and branch **once** on the aggregate.
- **No secret-dependent indexing.** Lane offsets `idx*14` and the schoolbook
  `degree = lhs_degree + rhs_degree` indices are all derived from public lengths
  and loop counters — already data-independent in the reference; keep them so.
- **Fixed-iteration loops.** `poly_mul_mod`'s reduction loop is `while len ≥
  modulus_len` in the reference (`Spike_core.fst:213`, the `while_loop_cf` the hax
  F\* typecheck stalled on). For Jasmin, replace it with a **fixed-bound `for`
  loop** over the known max degree (lhs/rhs are degree < 12, product degree < 23,
  reduce a fixed number of steps) so there is no data-dependent trip count. This
  also sidesteps the missing `while_loop_cf` library combinator noted in HAX-SPIKE.

No claim is made here that any of this *is* CT-proven — only that these are the
obligations the Jasmin CT checker must discharge in Phase 2.

---

## 3. Correctness obligation

The Jasmin core must be proven **functionally equivalent to the EasyCrypt/spec
model of VERIFY**, via the **Jasmin → EasyCrypt embedding** (the same
extract-to-EC-and-prove pipeline used for Formosa / ML-KEM). Concretely:

1. **Equivalence target = the EasyCrypt model of `spec` §4 Verify steps 1 & 3**
   — the canonical parse + the Fq12 sigma equation. This model is produced by
   Phase 2's EC reduction work; it is the *definition* the Jasmin code is proven
   against. EC-SPIKE returning **GO** (`formal/easycrypt/SPIKE_VERDICT.md`) is
   what makes this target real: the probability/reduction spine lives in EC, so
   the verifier predicate the reduction reasons about is *the same EC term* the
   Jasmin embedding must match. No second hand-written model, no extra
   spec-correspondence seam on the Jasmin side.
2. **Functional reference cross-check.** The hax-extracted `Spike_core.fst`
   (`formal/hax/extracted/`) is the independently-derived F\* image of the same
   arithmetic core; the Jasmin core's input/output behaviour must agree with it
   lane-for-lane (useful as a debugging oracle even before the EC proof closes).
3. **KAT conformance, byte-for-byte.** The core must reproduce the frozen KAT
   vectors byte-for-byte, including the negative vectors and their reject reasons
   (`formal/kat/kat_v0.json` today carries 3 positive round-trips + 8 negatives
   over 7 distinct reject reasons — e.g. `signature has N bytes, expected …`,
   `challenge mismatch`, `local fiber-state mismatch`, `account randomizer
   commitment mismatch`, `slot already consumed`, `slot id out of range`,
   non-canonical digest). **Caveat (§0):** `kat_v0.json` pins the **260-B legacy
   linear-sigma** layout, not the 314-B spec target. The conformance obligation
   is against a **regenerated KAT (v1) for the 314-B `checked_core ‖
   projection_carrier` layout**; this regeneration is a hard prerequisite
   (`VERIFICATION_PLAN.md` Phase-0 already flags KAT v0 must be regenerated when
   the path is hardened). KAT and EC-equivalence are complementary, not redundant
   (HAX-SPIKE rec. #3): EC proves *Jasmin ≡ spec*; KAT proves *both ≡ the
   reference runtime*.

**Honesty:** nothing here is proven yet. This section states the obligation; it
does not assert it is met.

---

## 4. Effort estimate

Rough, for one engineer fluent in Jasmin + EasyCrypt; **dominant uncertainty is
toolchain install and the `poly_mul_mod` correctness proof**, not the code volume
(the core is small — the whole hax image is ~509 lines and most is plumbing).

| Workstream | Estimate | Notes |
|---|---|---|
| **0. Toolchain bring-up** | **0.5–1 pw** | Jasmin is **NOT installed** (no `jasminc`/`jasmin` on PATH, confirmed). Install Jasmin + its EasyCrypt export; wire the `extract-to-EC` path; stand up the CT checker. EasyCrypt 2026.05 + Z3 4.16.0 already present (EC-SPIKE). |
| **1. Parse / deserialize path** | **1 pw** | Field slicing of the 314-B sig + 172-B pk; the 14-bit lane unpack + canonical-bit + lane-`<q` rejects; fixed-length check. Small, fiddly bit-twiddling; CT-clean. |
| **2. Fq12 arithmetic** | **1.5–2 pw** | `add_mod`/`sub_mod` with **Barrett/Montgomery** for q=12289 (~0.5 pw, ML-KEM template); `poly_mul_mod` schoolbook + monic reduction as a **fixed-bound loop** (~1–1.5 pw, the genuine work). |
| **3. CT proof** | **0.5–1 pw** | Run the Jasmin CT checker; close the "no secret-dependent branch/index/loop-bound" obligations. Cheap *because* most inputs are public (§2); cost is mostly making the reductions and compares branchless. |
| **4. EC functional-equivalence proof** | **2–3 pw** | Prove the embedded Jasmin program equals the EC VERIFY model (steps 1 & 3). The `poly_mul_mod` loop-invariant + monic-reduction correctness is the bulk; everything else is `add_mod`/`sub_mod`/lane bound lemmas (HAX-SPIKE judged these "Z3-trivial"). Includes proving the Barrett/Montgomery form `≡ %! q`. |
| **5. KAT conformance** | **0.5 pw** | **Plus** a prerequisite KAT-v1 regeneration for the 314-B layout (separate task, not Jasmin work; flagged §0/§3). Wiring the core to emit/accept KAT vectors and diffing is small. |

**Headline estimate: ~6–8.5 person-weeks** for the v0 verifier core (parse +
Fq12 sigma + CT + EC-equivalence + KAT), assuming the Phase-2 EC VERIFY model and
the regenerated 314-B KAT already exist. Add **~0.5–1 pw** if toolchain
bring-up hits friction (plausible given the hax-vs-dependency skew already seen
in HAX-SPIKE). **Round figure to quote: 7 person-weeks.**

### Dependency ordering (what must exist first)

1. **EC reduction model of VERIFY (Phase 2)** — defines the equivalence target.
   Without it, workstream 4 has nothing to prove against. **Hard blocker.**
2. **Jasmin toolchain installed + EC-export wired** (workstream 0) — currently
   absent. Can proceed in parallel with #1.
3. **Regenerated KAT v1 for the 314-B spec layout** — blocks workstream 5;
   independent of #1/#2 and can start anytime.
4. Then 1 → 2 → 3 → (4 needs #1) → 5. Parse (ws1) and arithmetic (ws2) can be
   written before the EC model lands, but cannot be *proven* (ws3/ws4) until it
   does.

---

## 5. Sequencing — start this AFTER the first real EC break-event reduction

This layer is **gated behind the Phase-2 EC reduction model existing**, for a
specific reason: the Jasmin correctness obligation (§3) is "equivalent to the
EasyCrypt VERIFY model." Until that EC model is written, the *equivalence target
is undefined* and the largest workstream (ws4, ~2–3 pw) cannot even begin.

EC-SPIKE was a GO but **qualified**: its verdict explicitly defers committing all
of Phase 2 until "a second narrow spike on the OB-1 coupling + **one real QROM
break-event reduction**" runs (`VERIFICATION_PLAN.md` Phase-1; `formal/easycrypt/
SPIKE_VERDICT.md`). That first *real* break-event reduction is what produces the
concrete EC VERIFY predicate the Jasmin core attaches to.

**Recommendation:** do **not** start Jasmin implementation before that first real
EC break-event reduction lands. Starting earlier risks building the verified core
against a VERIFY model that the reduction work then reshapes — re-doing the
expensive equivalence proof. The cheap, no-regret work that *can* start now is
non-blocking: (a) install the Jasmin toolchain (ws0), and (b) regenerate KAT v1
for the 314-B layout. Both are prerequisites with zero dependence on the EC model
and remove two items from the critical path.

Net: **Jasmin coding begins once (i) the first real EC break-event reduction
defines the VERIFY model and (ii) the 314-B KAT v1 exists.** Until then this
document is the deliverable, and per `VERIFICATION_PLAN.md` Phase-1 the
JASMIN-SCOPE line can be marked done while the implementation line stays gated in
Phase 2.

---

## Appendix — reuse map (hax decomposition → Jasmin routines)

| hax / `Spike_core.fst` symbol | Jasmin v0 routine | In scope |
|---|---|---|
| `add_mod` / `sub_mod` (`:35,:43`) | field ± with Barrett/Montgomery for q | yes |
| `poly_mul_mod` (`:138`) | schoolbook mul + monic reduce, fixed-bound loop | yes (main work) |
| `fq12_syndrome_add/mul/normalize` (`:312,:338,:359`) | 12-lane wrappers | yes |
| `read_bits_le` / `any_bits_set_le` (`:57,:78`) | 14-bit lane unpack + canonical-bit reject | yes |
| `unpack_…_syndrome_digest` (`:366`) | digest → `[u16;12]`, reject `≥q` / slack bits | yes |
| `reviewed_fq12_account_carried_expected_response_syndrome` (`:480`) | sigma RHS `a + action(c)·w` + CT equality compare | yes |
| `action_oracle` (`:457`) | **input** to v0 (real SHA-256 hash-to-field deferred) | no (v0) |
| projection carrier / openings / account lifecycle | — | no (v0) |
