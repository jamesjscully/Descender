# FRI Argument-of-Knowledge extractor — instantiation for `descender-account-stark`

**Status:** discharges the `p2_sis_extractor` / "FRI-AoK extractor" obligation as
OUR mathematics: standard results (cited), instantiated for OUR pinned params, with
the concrete knowledge-soundness error and an HONEST residual on what binds.
**Date:** 2026-06-18. **Scope:** the deployed account-creation leaf STARK
(`tools/descender-account-stark`), single-instance and batch modes. Recursive
(round-by-round under an outer AIR) extractor is flagged future work in §6.

This is a paper instantiation, not a Lean/EC machine proof. It is honest about the
binding constraint (§4): under the conservative Johnson regime the **query-phase**
soundness reaches 128 bits, but the FRI **commit/fold + DEEP additive** term over
the ~124-bit challenge field is the binding constraint and does NOT clear 2⁻¹²⁸ at
realistic trace degrees. The fix is degree-5 extension (or a 2-adic 128-bit field).

---

## 0. Pinned parameters (from `src/config.rs`, verified 2026-06-18)

| Symbol | Value | Source |
|---|---|---|
| Base field `Val` | KoalaBear, `p = 2³¹ − 2²⁴ + 1` | `config.rs:102` |
| `log₂ p` | 30.9887 | computed |
| Challenge field `Challenge` | `BinomialExtensionField<Val,4>` (degree-4 ext) | `config.rs:104` |
| `\|F\| = p⁴` | `log₂ = 4·30.9887 = 123.955` bits | computed |
| `PROD_LOG_BLOWUP` | 4 ⇒ rate `ρ = 1/2⁴ = 1/16` | `config.rs:51` |
| `√ρ` | `1/4` ⇒ `−log₂√ρ = 2` | computed |
| `PROD_NUM_QUERIES` | `s = 56` | `config.rs:53` |
| `PROD_QUERY_POW_BITS` | `g = 16` | `config.rs:55` |
| `PROD_COMMIT_POW_BITS` | `0` | `config.rs:57` |
| FRI fold arity | 2 (`HidingFriPcs`, Plonky3 default) | type alias |
| Hiding/ZK | `HidingFriPcs` + `MerkleTreeHidingMmcs`, ≥2 blowup | `config.rs:21,115` |

The config comment is explicit and CORRECT in posture: the late-2025
up-to-capacity proximity-gap conjecture is DISPROVEN, so we use the conservative
Johnson decoding radius `δ_J = 1 − √ρ` (here `3/4`) and the conservative
`½·log_blowup = 2` bits/query, NOT the optimistic `log_blowup = 4` bits/query.

---

## 1. The committed objects and the relation (what the extractor must output)

The leaf AIR (`src/air.rs`) proves the Path-2 creation relation
`CreationHiddenSourceRelationV1`: there exist private trace columns — `pad_seed`,
the six `account_projection` field-lanes, the PRF-derived `masks[6]`, the
`masked_projection[6] = (account_projection + masks) mod q12289` carrier, the
12 `syndrome` residues, and the Poseidon2 commitment columns (`pad_source_commitment`,
`opening_tuple_commitment`, `role_root`, `packed_witness_commitment`) — such that
the in-AIR constraints (C1, C2-PRF, C4 mod-q, C6 preimage-order, range/canonicity,
public-binding) all vanish on the trace domain, AND the public values
(`account_state_root`, `verifier_version`, `syndrome[12]`,
`packed_witness_commitment[32]`, role-root) match.

The witness the extractor must produce is exactly this trace `T ∈ Val^{H×w}`
(`H` = trace height = power of two ≥ 2, `w` = trace width), from which the
relation-level witness (notably the canonical `account_projection`,
`masked_projection`, and the binding `packed_witness_commitment`) is read off the
fixed columns. Because the AIR is FULLY CONSTRAINED (audited 2026-06-18,
`path2_account_stark_air_soundness_audit.md`: all 8 relation classes wired into the
constraint polynomial, ranges/canonicity/PRF/commitments all real), a trace that
satisfies the constraints IS a valid relation witness. So the AoK reduces to:

> **Extract a low-degree trace `T` consistent with the prover's Merkle commitments
> and the verifier's accepting FRI/DEEP-ALI transcript.**

This is precisely the FRI/DEEP-ALI knowledge-soundness statement.

---

## 2. Theorem statements (standard results, cited)

We use the DEEP-ALI + FRI pipeline as deployed by Plonky3 `uni-stark` /
`HidingFriPcs`, whose soundness is the ethSTARK / DEEP-ALI analysis layered on the
BCIKS proximity-gap and FRI-soundness theorems.

### 2.1 Proximity gaps for RS codes (BCIKS) — the core combinatorial fact

**Theorem (Ben-Sasson, Carmon, Ishai, Kopparty, Saraf — eprint 2020/654, "Proximity
Gaps for Reed–Solomon Codes").** Let `RS[F, L, ρ]` be the RS code of rate `ρ` over
evaluation domain `L ⊆ F`. For a proximity parameter `δ < 1 − √ρ` (the Johnson
bound), an affine/linear space `U` of words is either (i) ALL of `U` is within
relative Hamming distance `δ` of the code, or (ii) all but a
`≤ err = poly(\|L\|)/\|F\|` fraction of `U` are at distance `> δ`. Concretely, for
the correlated-agreement / batched form used by FRI folding, in the Johnson regime
`δ = 1 − √ρ − η` (any `η > 0`),

```
    err_prox(δ) ≤ (m+½)⁷ · |L| / (3 · ρ^{3/2} · |F|),   with m = 1/(2η√ρ) (Johnson list-size param)
```

(BCIKS Thm 1.2 / 4.1, Johnson-bound version). The list size at the Johnson radius is
`L_J ≤ m+½ = 1/(2η√ρ) + ½`, finite for any `η > 0` — this is what bounds the
extractor's branching (§3).

### 2.2 FRI soundness / round-by-round knowledge soundness

**Theorem (BBHR18 FRI; refined RBR soundness — Block–Garreta–Katz–Thaler–Tiwari–Zhang
et al., eprint 2023/1071 "Fiat–Shamir Security of FRI and Related SNARKs", and the
ethSTARK report v1.1 §5).** For the commit-and-query FRI protocol on `RS[F,L,ρ]`
with `r = log₂(d)` folding rounds (arity 2) and `s` independent query repetitions,
if the committed oracle is `δ`-far from the code (`δ < 1 − √ρ`, Johnson) then a
cheating prover is rejected except with probability

```
    err_FRI ≤ err_commit + err_query,
    err_commit ≤ (r + ½) · err_prox(δ)        (folding/consistency, per round, additive)
    err_query ≤ (1 − δ + η)^s = (√ρ + η)^s    (s independent queries each catch a far point w.p. ≥ δ)
```

Round-by-round knowledge soundness (eprint 2023/1071) makes this Fiat–Shamir–safe:
each FS challenge is a separate RBR round whose state-advancing error is bounded by
the above terms, so the non-interactive (FS-compiled) protocol inherits the bound up
to the per-round transcript-grinding cost, which we cover with the query-phase
proof-of-work `g = 16` bits.

### 2.3 DEEP-ALI (quotienting the AIR into the RS proximity test)

**Theorem (ethSTARK / DEEP-FRI — Ben-Sasson, Goldberg, Kopparty, Saraf, eprint
2019/336 "DEEP-FRI", and the ethSTARK report).** The AIR-to-FRI reduction (DEEP
algebraic linking) adds a soundness term for the out-of-domain DEEP point `z ∈ F`
and the constraint-batching randomness:

```
    err_DEEP ≤ (D_quot + 1) / |F|   +   (#columns/quotients batched) / |F|
```

where `D_quot` is the degree of the quotient polynomial (constraint degree × trace
degree). If FRI accepts a word `δ`-close to `RS` AND the DEEP/quotient test passes,
then the unique decoded codeword is the evaluation of polynomials satisfying the AIR
constraints (DEEP soundness: a false statement forces the quotient to be a non-
polynomial / high degree, caught by FRI except `err_DEEP`).

---

## 3. The extractor construction (OUR instantiation)

We build a probabilistic polynomial-time extractor `E` that, given black-box rewinding
access to any prover `P*` that makes the Plonky3/`uni-stark` verifier accept with
probability `ε > κ` (`κ` = §4 knowledge-soundness error), outputs a valid trace
witness `T` for `CreationHiddenSourceRelationV1`, in expected time `poly(\|L\|)/(ε−κ)`.

**Step 1 — fix the commitments.** Run `P*` to the first accepting transcript. This
pins the Merkle roots of (a) the doubled-domain trace commitment (the `HidingFriPcs`
salted-leaf MMCS, `config.rs:115`), (b) the quotient/DEEP commitment, and (c) every
FRI fold-layer root. By the binding (collision-resistance) of the Poseidon2-Merkle
MMCS, these roots fix functions `f₀ : L → Val^w` (trace) and the fold words
`f₁,…,f_r` on the FRI domains, up to negligible Merkle-collision probability
`err_MMCS ≤ (#opened leaves)·Q_H² / 2^{8·31}` (8-elem Poseidon2 digest;
collision-resistance is the named-standard `SHA-256/Poseidon2 CR` class in the dossier).

**Step 2 — harvest queries by rewinding.** Rewind `P*` over fresh FS query
randomness, collecting accepting query-openings. Each accepting query reveals
consistent (trace, fold) column values at a uniformly random `x ∈ L` together with
its Merkle authentication path. Collect `O(\|L\|/(ε−κ))` accepting openings; by a
standard heavy-row / averaging argument (the openings are to the COMMITTED `f₀`
from Step 1, which is fixed), with overwhelming probability the harvested points
cover `> δ` fraction of `L` whenever `f₀` were `δ`-far — i.e. acceptance with
`ε > κ` certifies `f₀` is `δ`-CLOSE to `RS[F,L,ρ]^w` (column-wise), by the
contrapositive of §2.2 `err_query = (√ρ+η)^s`.

**Step 3 — decode to the unique low-degree codeword.** Since `f₀` is within the
Johnson radius `δ < 1 − √ρ` of the code, list-decode each trace column:
Guruswami–Sudan / Johnson-bound list decoding returns a list of size
`≤ L_J = 1/(2η√ρ) + ½` candidate low-degree polynomials in `poly(\|L\|)` time. The
DEEP out-of-domain consistency check (§2.3) — already part of the accepting
transcript — selects, except with `err_DEEP`, the UNIQUE codeword whose
out-of-domain evaluation matches the prover's claimed DEEP answer. Interpolate to get
the trace polynomials `\hat T₁,…,\hat T_w` of degree `< H`.

**Step 4 — read off the witness.** Evaluate `\hat T_j` on the trace domain (size
`H`) to recover `T ∈ Val^{H×w}`. Because the AIR is fully constrained (audit
2026-06-18) and the DEEP/quotient test passed, `T` satisfies every relation
constraint and its public columns equal the statement's. Read the relation witness
off the fixed columns: canonical `account_projection[6]`, `masked_projection[6]`,
`syndrome[12]`, and the binding `packed_witness_commitment[32]`. Output `T`. ∎

The list-decoding branching (`L_J`) is finite because we are STRICTLY inside the
Johnson radius (`η>0`); this is exactly why we cannot rely on the disproven
up-to-capacity conjecture — beyond Johnson the list size is unbounded and Step 3
fails to be polynomial.

---

## 4. Concrete knowledge-soundness error for OUR params

The extractor succeeds whenever `P*` accepts with probability above the total
knowledge-soundness error `κ = err_query + err_commit + err_DEEP + err_MMCS`. We
compute each over `\|F\| = 2^{123.955}`, `ρ = 1/16`, `√ρ = 1/4`, `s = 56`, `g = 16`.

### 4a. Query-phase term (the part the config sizes to 128)

```
    err_query = (√ρ)^s · 2^{−g}
              = (1/4)^{56} · 2^{−16}
              = 2^{−112} · 2^{−16}
              = 2^{−128}.
```

`−log₂ err_query = 56·2 + 16 = 128 bits.` ✓ This matches `PROD_SOUNDNESS_BITS`
(`config.rs:78`). Under the conservative Johnson `2 bits/query`, the query phase
EXACTLY hits the 128-bit target. (Optimistic capacity would be `56·4+16 = 240`; we
do NOT claim it.)

### 4b. Commit/fold + DEEP additive term (the BINDING constraint)

Let `D` = quotient/codeword degree bound, `\|L\| = D/ρ` the FRI domain, `r = log₂ D`
the number of fold rounds. The additive error over the field is, conservatively (§2.2/2.3,
leading term, taking the Johnson list param `m≈O(1)` for a clean comparison):

```
    err_commit + err_DEEP  ≈  (r · |L| + D) / |F|.
```

The deployed leaf trace is SHORT but WIDE (single-instance = 2 rows; batch = a few
hundred rows), so the codeword degree `D` is driven by **constraint degree × trace
length**, dominated by the Poseidon2 S-box (degree `KOALABEAR_S_BOX_DEGREE = 3`,
chained across full/partial rounds) and the digest chains. Realistic `D` lands in
`2^{16}…2^{24}` depending on batch size and the unrolled Poseidon2 rounds. Evaluating
the additive term:

| `log₂ D` | `\|L\| = 2^{logD+4}` | `r = log₂ D` | `err_commit+err_DEEP` over `2^{123.955}` |
|---|---|---|---|
| 16 | `2²⁰` | 16 | `≈ 2^{−99.9}` |
| 20 | `2²⁴` | 20 | `≈ 2^{−95.6}` |
| 24 | `2²⁸` | 24 | `≈ 2^{−91.4}` |

**This is the binding constraint.** The additive commit/DEEP error is
`≈ 2^{−92} … 2^{−100}`, FAR above `2⁻¹²⁸`. So although the query phase reaches 128
bits, the COMPOSED knowledge-soundness error is

```
    κ = err_query + (err_commit + err_DEEP) + err_MMCS
      ≈ 2^{−128} + 2^{−92..−100} + (negligible)
      ≈ 2^{−92 … −100}      (the additive field term dominates).
```

The ROOT CAUSE is §0: the additive terms scale as `\|L\|/\|F\|` and `\|F\| = 2^{123.955}`
is only ~124 bits. No amount of queries or PoW fixes an additive `1/\|F\|` term — it
is set by the challenge-field size and the codeword degree.

---

## 5. Does the composed soundness clear 2⁻¹²⁸? — HONEST verdict

**No, not at the deployed (degree-4 / ~124-bit) challenge field.**

- The **query count (56) and PoW (16) are NOT the binding constraint** — they
  deliver exactly 128 bits in the conservative Johnson regime (§4a).
- The **~124-bit challenge field IS the binding constraint.** The FRI commit/fold
  consistency and DEEP out-of-domain additive terms are `≈ \|L\|·log\|L\| / \|F\|`,
  which at `\|F\| = 2^{123.955}` and realistic codeword degree `2^{16..24}` sit at
  `≈ 2^{−92 … −100}`. The whole protocol's knowledge-soundness error is no better
  than this. It does NOT clear `2⁻¹²⁸`; honest headline is **≈ 92–100 bits of
  knowledge soundness**, not 128.

### Fixes (in order of preference)

1. **Degree-5 extension (`BinomialExtensionField<Val,5>`):** `\|F\| = p⁵ = 2^{154.9}`.
   The additive term drops to `≈ 2^{−123 … −131}`; combined with the 128-bit query
   phase the composed `κ ≈ 2^{−123 … −128}` clears (or nearly clears) the target.
   Verify KoalaBear admits an irreducible degree-5 binomial (Plonky3 support) and
   re-cost the Poseidon2/FRI arithmetic (extension mults get ~1.5× more expensive).
   This is the surgical fix — it does not touch the query schedule.
2. **Two-extension / 128-bit-secure field (e.g. degree-8 ext or a dedicated
   2-adic 128-bit field):** `\|F\| ≥ 2^{248}` makes additive terms vanish entirely;
   heavier arithmetic and a field re-host (tracked in the STARK-field-rehost memo).
3. **More queries does NOT help** the binding term (additive `1/\|F\|`), so raising
   `s` past 56 is wasted against this constraint — only the field size helps.

Recommendation: adopt the **degree-5 extension** (option 1) and re-run §4b with
`\|F\| = 2^{154.9}`; keep `s = 56`, `g = 16` unchanged. Until then, the leaf STARK's
honest knowledge-soundness label is **≈ 92–100 bits (NAMED-STANDARD, field-bound)**,
NOT 128, and the extractor of §3 is valid but with error `κ ≈ 2^{−92..−100}`.

This corrects the `PROD_SOUNDNESS_BITS = 128` label: 128 is the QUERY-phase figure
only; the COMPOSED AoK figure is field-bound and lower. The config's conservative-
Johnson posture is right; the missing piece is charging the additive field term.

---

## 6. Recursive extractor (round-by-round under an outer AIR) — future work

`descender-recursion` aggregates leaf proofs under an outer AIR. The composed
extractor is the standard recursive/round-by-round construction (eprint 2023/1071
RBR knowledge soundness composes; ethSTARK recursion analyses):

- The outer verifier-circuit's AIR re-derives each inner FS challenge; RBR soundness
  lets the inner-FRI knowledge error compose ADDITIVELY across the recursion tree of
  depth `t`: `κ_rec ≤ κ_outer + Σ_leaves κ_leaf` (no exponential blow-up under RBR).
- The recursive EXTRACTOR runs §3 at the outer layer to recover the outer trace
  (which embeds the inner Merkle roots + the inner accepting transcripts as witness),
  then runs §3 on each embedded inner transcript. Because the outer AIR binds the
  inner roots in-circuit, Step 1 (commit-fixing) chains down the tree.
- **Open items (future work):** (i) the inner-challenge-soundness penalty of
  simulating FS *inside* a finite-field circuit (challenge sampled from the SAME
  ~124-bit field — the §5 field constraint compounds at the outer layer and MUST be
  re-budgeted); (ii) the simulator/hiding interaction with `HidingFriPcs` under
  recursion (the salted leaves must remain hiding when re-committed by the outer
  layer); (iii) a Lean/EC mechanization of the RBR composition seam (currently the
  same paper-composition GAP-B/D seam the dossier carries for the SUF-CMA spine).

---

## 7. Citations

- E. Ben-Sasson, D. Carmon, Y. Ishai, S. Kopparty, S. Saraf. *Proximity Gaps for
  Reed–Solomon Codes.* eprint 2020/654 (FOCS 2020). — proximity-gap, Johnson-bound
  correlated agreement, list-size `m`.
- E. Ben-Sasson, I. Bentov, Y. Horesh, M. Riabzev. *Fast Reed–Solomon Interactive
  Oracle Proofs of Proximity (FRI).* ICALP 2018 / eprint 2017/1196. — FRI soundness.
- E. Ben-Sasson, L. Goldberg, S. Kopparty, S. Saraf. *DEEP-FRI: Sampling Outside the
  Box Improves Soundness.* eprint 2019/336. — DEEP out-of-domain term.
- StarkWare. *ethSTARK Documentation* (v1.1, eprint 2021/582), §5 soundness. —
  conservative Johnson `½·log_blowup` bits/query; commit + query decomposition.
- A. Block, A. Garreta, J. Katz, J. Thaler, P. R. Tiwari, M. Zhang. *Fiat–Shamir
  Security of FRI and Related SNARKs.* eprint 2023/1071. — round-by-round knowledge
  soundness, FS-compiled FRI/STARK, recursive composition.
- (Context) Late-2025 disproof of the up-to-capacity proximity-gap CONJECTURE ⇒ we
  use the Johnson bound `δ < 1 − √ρ` and `½·log_blowup` bits/query, NOT capacity.

---

## 8. What this discharges, precisely

- `p2_sis_extractor` / "FRI-AoK extractor": **DISCHARGED as OUR math** at the leaf —
  the extractor of §3 is constructed and its error computed for OUR params (§4),
  reducing the obligation to NAMED-STANDARD primitives (BCIKS proximity gap,
  FRI/RBR soundness, DEEP) + Poseidon2-Merkle collision-resistance.
- **HONEST RESIDUAL (do not over-claim):** (1) the composed AoK error is
  `κ ≈ 2^{−92..−100}`, field-bound — it does NOT clear `2⁻¹²⁸` at the deployed
  degree-4 extension; the fix is the degree-5 extension (§5). (2) This is a paper
  instantiation, not a Lean/EC machine proof — same composition-seam posture as the
  dossier's GAP-B/D. (3) The recursive extractor (§6) is future work and inherits
  the §5 field constraint at the outer layer.
