# Path2AccountStarkTranscriptQromTheoremV1

> Classification: `ACTIVE_SUPPORT`. Supports the active Poseidon2/Plonky3
> account-creation path (`ADR-0004`, tracker rows `DIR-QROM-001`,
> `DIR-PROOF-001`). It is the Fiat–Shamir / QROM transcript-soundness theorem for
> the account-STARK, composing the FRI/AoK extractor
> (`research/cryptanalysis/fri_aok_extractor_instantiation.md` §9) into the
> non-interactive setting. Not normative by itself; the residual authority is
> `spec/security_model.md` "Selected Account-STARK Residual Matrix".

Status: account-STARK transcript Fiat–Shamir / QROM soundness theorem and
concrete pricing.

## Obligation

Residual matrix rows: *"account-STARK FRI transcript QROM theorem"* (kept false
by `Path2AccountStarkFriQromResidualReportV1`) and *"Residual-assumption ledger
prices FRI queries, field size, the account-STARK Poseidon2/FRI transcript
theorem, recursion if used, and multi-target/QROM terms"* (FRI-soundness-margin
row). Named break: `fri_soundness_margin_break`. This document supplies the
transcript theorem and the classical+quantum price; the FRI residual-pricing half
(query/field/MMCS terms) was computed in the extractor §9 re-pricing.

## Inputs (already established)

- **Interactive AoK (#2, §9):** an accepting interactive account-STARK transcript
  yields, via the FRI/DEEP extractor, a trace `T ⊨ AIR` with error
  `κ_IOP ≈ 2⁻¹²⁴` (query phase `2⁻¹²⁸`, field additive `2⁻²¹⁵⁻²²⁴`, Merkle/sponge
  floor `2⁻¹²⁴`).
- **Arithmetization soundness (#1):** `T ⊨ AIR ⟹` witness `⊨ R_air`.
- **FS hash:** the transcript is made non-interactive by the Poseidon2
  `DuplexChallenger` (`config.rs` `Challenger = DuplexChallenger<Val,Perm,16,8>`)
  — the SAME width-16 KoalaBear Poseidon2 sponge analyzed in
  `research/cryptanalysis/frame_prf_poseidon2_ro_reduction.md`.

## Standard results used

- **R1 (sponge indifferentiability, classical):** BDPV08 — a sponge over an ideal
  permutation `P` is `(t,q_P,ε)`-indifferentiable from an RO with
  `ε ≤ N²/2^{c·log₂p}`, delivering `min(c/2, output)` bits. Here `c=8`,
  `log₂p=30.989` ⇒ floor `c/2 ≈ 123.95` bits. Reduces to **A_poseidon2_perm_ideal**
  (Poseidon2 ≈ ideal permutation).
- **R2 (sponge indifferentiability, quantum):** Czajkowski–Bruinderink–Hülsing–
  Schaffner–Unruh, *Quantum Indistinguishability of Random Sponges* (CRYPTO 2019)
  — the sponge over an ideal permutation is indifferentiable from a
  quantum-accessible RO; the `c/2` capacity floor is unchanged (Grover/quantum
  walk does not beat the generic capacity-collision floor). Needs the *quantum*
  form of A_poseidon2_perm_ideal.
- **R3 (Fiat–Shamir in (Q)ROM for FRI/STARK):** Block–Garreta–Katz–Thaler–Tiwari–
  Zhang 2023/1071 (RBR-FS soundness of FRI); Don–Fehr–Majenz–Schaffner 2019 and
  Liu–Zhandry 2019 (QROM-FS for round-by-round / Σ-protocols) — an FS-compiled
  round-by-round-sound IOP is knowledge-sound in the (Q)ROM up to a polynomial
  query-extraction factor and the per-round grinding cost.

## Main Theorem

> **Theorem (account-STARK transcript FS/QROM soundness).** Model the Poseidon2
> `DuplexChallenger` as a random oracle via R1 (classical) / R2 (quantum). Then
> the FS-compiled (non-interactive) account-STARK is knowledge-sound: any prover
> producing an accepting proof `π` for statement `pv` with probability above the
> error `κ_FS` below yields (by the §9 extractor run on the RO-reconstructed
> transcript) a trace `T ⊨ AIR`, hence (by #1) a witness `⊨ R_air`. Concretely,
> with `s=56`, `½·log_blowup=2`, query PoW `g=16`, `c=8`:
>
> ```text
>   Classical:  κ_FS ≈ max( 2^{-(56·2 + 16)},  ε_sponge,  Q_FS · κ_IOP )
>                    ≈ 2^{-124}        (binding term: the c/2 ≈ 124-bit sponge floor)
> ```
>
> **CORRECTION (red-team 2026-06-27 — the earlier quantum line here was WRONG).**
> The prior draft halved only the explicit 16-bit PoW (`56·2 + g/2 ⇒ 2^{-120}`).
> That is incorrect for a *non-interactive* FS proof: the prover commits its δ-far
> oracle ONCE and then grinds the FS nonce, which re-derives all 56 query
> positions each attempt. So the ENTIRE classical budget `s·(½log_blowup) + g =
> 128` bits is a single grinding search of marked-density `2^{-128}`, and Grover
> solves it in `2^{64}`:
> ```text
>   Quantum:    κ_FS^Q ≈ max( 2^{-(56·2 + 16)/2},  ε_collision^Q )
>                     ≈ max( 2^{-64},  2^{-(82..99)} )
>                     ≈ 2^{-64}        (FS-grind-dominated; the query phase is
>                                       FULLY grindable in FS, not just the PoW)
> ```
> Two consequences the prior draft missed: (i) raising `PROD_QUERY_POW_BITS`
> 16→32 reaches only `(112+32)/2 = 2^{-72}`, NOT 2^{-128} — it is INSUFFICIENT;
> 128-bit *quantum* soundness needs the classical budget ≈ 256 bits, i.e. ≈ 120
> queries (`s≈120`, ~2× FRI proof size) or an infeasible PoW. (ii) The quantum
> *collision* resistance of the 8-element (≈248-bit) Merkle/sponge digest is
> ≈ `2^{-82..-99}` (BHT / parallel-Grover), separate from and BELOW the `2^{-124}`
> indifferentiability floor — for a post-quantum binding claim this is the honest
> figure, not 124. See §"Quantum pricing" below.

### Proof sketch

1. **RO replacement.** By R1/R2 replace the `DuplexChallenger` with a (quantum-
   accessible) RO, paying `ε_sponge ≈ 2⁻¹²⁴` (R1) / `ε_sponge^Q ≈ 2⁻¹²⁴` (R2).
   After replacement the FS challenges are RO outputs.
2. **FS extraction.** By R3, an accepting non-interactive `π` with the RO
   challenges corresponds, up to the QROM-FS query factor `poly(Q_FS)` (DFMS/LZ),
   to an accepting interactive transcript whose challenges are uniform. **In the
   non-interactive setting the prover commits its oracles BEFORE the query
   challenge, then grinds the FS nonce — so the query-soundness term
   `(√ρ)^s = 2^{-112}` is grindable, not just the explicit PoW `2^{-g}`.** The
   combined classical grinding cost is `2^{-(s·(½log_blowup)+g)} = 2^{-128}`;
   Grover over the nonce search solves this marked-density-`2^{-128}` predicate in
   `2^{64}`. (R3/RBR makes FS *provably sound*; it does NOT exempt the grindable
   query budget from Grover.)
3. **Interactive extraction.** Run the §9 extractor on the reconstructed
   interactive transcript to obtain `T ⊨ AIR` with `κ_IOP ≈ 2⁻¹²⁴`.
4. **Relation.** By #1, `T` decodes to a witness `⊨ R_air`.
5. **Compose the errors.** Classically `κ_FS = ε_sponge + poly(Q_FS)·κ_IOP +
   2^{-(s·2+g)} ≈ 2⁻¹²⁴` (sponge floor binds). **Quantumly** the FS grind
   dominates at `2^{-(s·2+g)/2} = 2⁻⁶⁴`, with the quantum Merkle-collision floor
   `2^{-(82..99)}` next; the indifferentiability floor `2⁻¹²⁴` is NOT the quantum
   binding term. ∎

## Concrete pricing and the actionable knob (CORRECTED 2026-06-27)

| regime | FS grind (query+PoW) | quantum digest collision | composed `κ_FS` |
|---|---|---|---|
| classical | `2⁻¹²⁸` (`56·2+16`) | `2⁻¹²⁴` (sponge/Merkle) | `2⁻¹²⁴` (sponge floor) |
| quantum (g=16, s=56) | `2⁻⁶⁴` (Grover on full budget) | `2⁻⁸²⁻⁹⁹` (BHT) | **`2⁻⁶⁴`** |
| quantum (g=32, s=56) | `2⁻⁷²` | `2⁻⁸²⁻⁹⁹` | **`2⁻⁷²`** |
| quantum (g=16, s≈120) | `2⁻¹²⁸` (classical budget ≈256) | `2⁻⁸²⁻⁹⁹` | `2⁻⁸²⁻⁹⁹` (now Merkle-bound) |

**Actionable — and the earlier "g=32 fixes it" claim was WRONG:**

- Raising `PROD_QUERY_POW_BITS` 16→32 reaches only `2⁻⁷²` quantum, **not** 2⁻¹²⁸.
  The whole query+PoW budget is Grover-able in FS, so quantum security is HALF the
  classical bits.
- 128-bit *quantum* FS-grind soundness needs the classical budget ≈ 256 bits, i.e.
  **≈ 120 queries** (`s≈120, g=16 → 240+16=256 → /2=128`) — roughly **2× the FRI
  proof size**. This is a real design decision (proof-size vs PQ margin), James's
  call.
- Even after that, the **quantum binding floor is the 8-element Merkle/sponge
  digest collision ≈ `2⁻⁸²⁻⁹⁹`** (BHT/parallel-Grover), NOT 124. A hard quantum-128
  binding claim needs a wider digest / capacity `c=9` AND `s≈120`. The classical
  headline stays `2⁻¹²⁴` (sole sponge cap, `ADR-0006`).

## Composition with the full statement-correctness chain

```text
accepting NON-INTERACTIVE account-STARK proof π for pv
   ⟹ (R1/R2 RO replacement + R3 FS extraction, κ_FS ≈ 2⁻¹²⁴ classical / 2⁻⁶⁴ quantum at deployed params)
      an accepting interactive transcript
   ⟹ (§9 FRI/DEEP extractor, κ_IOP ≈ 2⁻¹²⁴)  a trace T ⊨ AIR
   ⟹ (#1 R_air arithmetization soundness)  a witness ⊨ R_air.
```

Together with #3 (the bundle is simulatable / no-export) this is the full
in-circuit statement-correctness + knowledge + privacy chain for the account-STARK
at the honest `2⁻¹²⁴` (classical) level, modulo the out-of-AIR `R_wiring`/`R_namespace`
clauses (#5/#9).

## Honest residual (not flipped)

- **A_poseidon2_perm_ideal (classical + quantum):** the Poseidon2 permutation
  modeled as ideal — named-standard primitive assumption, ASSUMED not proven.
- **Machine-checked QROM bound:** R2/R3 are paper here; the EC/Lean O2H /
  compressed-oracle formalization is the same carried GAP-B/D seam as the dossier
  action-map QROM line (`frame_prf_ro_discharge.md` §5). Future work.
- **Recursion pricing:** the recursive (outer-AIR) FS challenge is sampled from
  the same field/sponge — the floor compounds at the outer layer and must be
  re-budgeted (extractor §6, `AGG-ROLLUP-001`). Not on the direct-mode path.
- **External QROM review** and the 124-vs-128 relabel / `c=9` decision remain.

`Path2AccountStarkFriQromResidualReportV1` keeps `account-STARK transcript QROM
theorem`, recursion pricing, external review, profile acceptance, and
deployment-ready flags **false**; this document reduces the obligation to the
named assumptions + the actionable `g=32` knob + external review, it does not
flip those flags.
