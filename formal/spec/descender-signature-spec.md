# Descender Signature Legacy Formal Specification (v0)

> MIGRATION STATUS (2026-06-20): `LEGACY_FORMAL`.
> This file is no longer the active product specification. It records the
> legacy 314-byte projection-carrier formal target and remains useful as
> historical formal context and as a negative-control source. Current product
> authority starts at `spec/README.md`, then append-only ADRs in `decisions/`,
> then the consolidated `TRACKER.md`.

Legacy status: v0 draft, formerly normative for the formal stack. At that stage,
each tool model (Lean, EasyCrypt, Jasmin, hax-extracted Rust) attached to
**this** document. Byte sizes
are the legacy/current-at-the-time post-AP2-004 constants (cross-checked against
`tools/parameter-search/src/production_reference.rs` by
`scripts/research/check_estimator_param_sync.py`). Section numbers are the
citation anchors tool formalizations must reference.

## 1. Parameters

| Symbol | Value | Notes |
|---|---|---|
| `n` | 32 | code/lattice dimension |
| `q` | 12289 | prime, 14-bit (`FIELD_BITS=14`) |
| `Fq12` | `F_q[X]/(X¹²+X+25)` | degree-12 extension; **verified irreducible** (`scripts/research/verify_fq12_irreducibility.py`) |
| SIS public rows | 12 | `SIS_FULL_RESPONSE_PUBLIC_ROWS` |
| response coords | ~23 | packed in 39–41 B |
| `RESPONSE_BOUND` | 48 | centered L∞ bound per response coord |
| response-difference bound | 96 | `2·RESPONSE_BOUND`; bounded-kernel search bound |
| challenge | 27 B | 23 algebraic + 4 hardening; **162-bit** entropy |
| repetitions | 26 | `REPS` |

## 2. Byte layouts (the conformance surface)

- **Public key — 172 B:** `seed[32] ‖ payload[140]` (systematic public view).
- **Account slot (state) — 32 B:** witness-commitment digest, consume-once.
  Per-spend account-link binding = `witness_commit[32] ‖ randomizer_commit[32] ‖ response_link_code[4]` (68 B).
- **Signature — 314 B (legacy formal target, not active product):**
  `checked_core[258] ‖ projection_carrier[56]` where
  - `checked_core[258]` = `slot_id[4] ‖ challenge[27] ‖ active_response[52] ‖ local_openings[104] ‖ proof_commitment[32] ‖ masked_response[39]`
  - `projection_carrier[56]` = `projection_lanes[24] (12 × q-residue) ‖ check_bytes[3] ‖ transcript_binding[29]`

## 3. The relation and the Fq12 action

The hard relation is a q-ary **SIS / response-link-coded** instance: a short
witness `w` (centered coords) whose syndrome under the public matrix equals the
account witness commitment, *and* that satisfies the response-link code (coords
0–3 constraints) which excludes the otherwise-short bare-quotient kernel vectors
(see `research/cryptanalysis/.../quotient_sis_lattice_attack_review`). The exact
bounded-kernel certificate proves no admissible short vector exists inside the
96-box (`ResponseLinkExactCertificate`, 96 < 114).

The **action** is `action : challenge → Fq12 (≅ F_q¹²)`, defined as
`action(c)[i] = uniform_mod_from_hash(DOMAIN, c, i, q)` for `i ∈ 0..11` — i.e. a
SHA-256-derived hash-to-field map. It is a random oracle, not algebraic; the
field structure is used only so a nonzero `action(c₀)−action(c₁)` is invertible
for forking extraction.

## 4. Algorithms (abstract)

**Keygen() → (pk, sk, account):** sample short witness material; derive the
systematic public view `pk (172 B)`; initialize `account` with consume-once
slots, each committing a hidden witness as a 32-B digest.

**Sign(sk, account, slot_id, msg) → (sig, account'):**
1. draw randomizer `r`; `proof_commitment a = L(r)`.
2. `challenge c = H(pk, msg, slot_id, witness_commit, a)` (FS frame; 162-bit).
3. `response z = r + action(c) · w` over the Fq12 syndrome structure.
4. assemble `local_openings`, `masked_response`, and the 56-B `projection_carrier`
   (masked projection of the witness; witness-hiding).
5. **consume** the slot (one-use lifecycle transition); return `sig (314 B)`, `account'`.

**Verify(pk, msg, account, sig) → (accept?, account'):**
1. parse the 314-B envelope canonically (reject non-canonical).
2. slot/witness/randomizer/freshness checks; recompute `c`.
3. **sigma equation** `z = a + action(c) · witness_syndrome` over Fq12.
4. local fiber + pad-opening consistency; projection-carrier binding +
   transcript binding (anti-replay across pk/msg/body/slot).
5. consume-once successor transition; all rejection is public-data-dependent and
   fail-closed.

## 5. Legacy Security Target (what the v0 proofs needed to establish)

- **Goal:** SUF-CMA in the **QROM** (conservative, per decision D2). Strong
  unforgeability (anti-malleability), quantum random oracle.
- **Bound:** adversary advantage `≤ 2⁻¹²⁸` at query budget `T = 2³²`.
- **Reduction:** forgery → (a) response-link-coded **SIS** solver, or (b) a named
  break reducing to **hash collision-resistance / domain separation (QROM)**, or
  (c) a **state-machine** violation (replay/rollback — proven impossible by the
  consume-once lifecycle). Per-event sorting in `research/cryptanalysis/ws7_reduction_scoping.md`.
- **Standing external assumption:** hardness of the response-link-coded SIS
  instance at these parameters (`open_external_estimator`). No tool closes this.

## 6. Legacy Open Spec Questions

These were unresolved questions for the v0 projection-carrier lineage. They are
not active product tasks unless re-imported by `spec/`, ADRs, or `TRACKER.md`.

- **Full witness opening vs projection-only.** The 56-B carrier proves a
  projection *relation*, not a full opening of the account-witness commitment
  (regression `projection_carrier_tuple_does_not_prove_full_witness_opening`).
  v1 must either add a full-opening proof or formally downgrade the claim to a
  proved projection-only relation with stated ZK/soundness — this is a spec-level
  decision, not just a proof gap.
- **Aggregation** is out of scope for v1 (deferred to v2 per D1); the spec keeps
  verification canonical/deterministic so aggregation stays possible later.
