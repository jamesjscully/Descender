# Recursive-FRI knowledge-extractor argument — `descender-recursion`

**Date:** 2026-06-19. **Status:** the recursion's central remaining PROOF-RISK item,
ARGUED as OUR mathematics and REDUCED to named seams (cited standard results, instantiated
for OUR validated gadget REFERENCES + pinned params, with honest residuals). NOT "discharged":
this is a paper sketch OVER UNBUILT AIR CONSTRAINTS — the five gadgets are plain-Rust
reference logic (zero `impl Air<AB>` exist yet; no `recursive_air`), so the soundness content
attributed to them below is what the constraints WILL enforce once lifted, not what is
enforced today. Builds on the LEAF extractor
(`fri_aok_extractor_instantiation.md` §3, "§L" below) and replaces its §6 "future work"
stub with a concrete argument now that (a) the challenge field is deg-8 (commit 4c06f6b,
the field-compounding flagged in §6(i) is resolved) and (b) all five verifier-AIR gadgets
are referenced + soundness-tested (commits 8bc6a97…29fb310).

This is a paper instantiation, not a Lean/EC machine proof — same composition-seam posture
(GAP-B/D) as the SUF-CMA spine. It is honest about the one accounting cost that does NOT
vanish: the N-fold extraction union over a block (a `log₂ N`-bit erosion of the per-leaf
floor).

---

## 0. What is being proven

`descender-recursion` is a hand-written verifier-AIR `R` that re-verifies one (Phase 1) or
two (Phase 2/3 fold) inner `descender-account-stark` proofs in-circuit and re-exposes their
public values. **Claim (recursive knowledge soundness):** there is a PPT extractor `E_rec`
that, given black-box rewinding access to any prover `P*` making the OUTER verifier accept an
`R`-proof with probability `ε > κ_rec`, outputs a valid account-creation witness for EVERY
leaf aggregated under that proof, in expected time `poly(|L_outer|)·N/(ε − κ_rec)` for a block
of `N` leaves. `κ_rec` is computed in §4.

The outer AIR `R` is itself a STARK over the SAME pinned stack (KoalaBear, deg-8 Challenge,
Poseidon2-16, conservative-Johnson FRI: `PROD_NUM_QUERIES=56`, `PROD_LOG_BLOWUP=4`,
`PROD_QUERY_POW_BITS=16`). So §L applies to `R` verbatim at the outer layer.

---

## 1. The outer AIR as an extractable relation (what the gadgets buy us)

`R` will accept iff its trace satisfies the five gadget relations. The table gives each
gadget's INTENDED role once its reference logic is lifted to `Air<AB>` constraints — what
makes the embedded inner transcript usable by the extractor. **Caveat (load-bearing):** none
of these is yet an AIR constraint; each cell states what the constraint WILL establish, and
several depend on DEFERRED pieces (noted per row). Today only the plain-Rust reference logic
is validated (golden + forgery tests).

| Gadget | Reference validates TODAY | Intended extraction role once lifted (+ deferred dependency) |
|---|---|---|
| 1 challenger | sponge-wiring fidelity to native `DuplexChallenger` (golden, element-for-element) + length-tag binding + §4 wiring constraints | WILL reproduce the inner FS challenges faithfully so the transcript is Fiat–Shamir-bound. NOTE: "no prover-chosen challenges" additionally needs the gadget-2 Poseidon2 permutation sub-AIR (perm correctness is deferred, `challenger.rs:212`) AND the cross-gadget β/eval bus binding challenges to opening values (deferred, `fri_fold.rs:22`) — NOT established by this reference alone. |
| 2 merkle | binary path-fold authenticates a leaf digest to the root (forgery-tested) | WILL bind inner opened values to committed inner roots (§L Step 1 commit-fixing, chained). DEFERRED: leaf-row hashing incl. hiding salt (value→leaf-digest) + multi-height injection (`merkle.rs:13`) — the value→leaf binding is exactly the deferred part. |
| 3 fri_fold | the fold IDENTITY `fold(f(s),f(-s)) = g(s²)` (algebraic correctness) + β/eval divergence | WILL enforce inner FRI commit-phase fold CORRECTNESS per query/round. Proximity (δ-closeness to RS) comes from BCIKS (§L Step 2), NOT from this gadget — this gadget gives fold correctness only. |
| 4 deep | the DEEP-ALI relation `C(ζ)=Z_H(ζ)·Q(ζ)` given `C(ζ),Q(ζ)` (forgery-tested) | WILL enforce the OOD quotient link. DEFERRED: `C(ζ)` itself = the full leaf-AIR constraint-poly eval, and quotient-chunk recomposition (`deep.rs:14`) — i.e. the part that actually ties to the leaf constraints is deferred. |
| 5 public_io | element-wise statement-preservation equality (substitution/length/empty forgery-tested) | WILL bind the re-exposed statement to the inner one ⇒ extraction targets the CLAIMED statement. (Fully covered by the reference.) |

So once lifted + assembled, a satisfying `R`-trace WILL BE an inner accepting transcript plus
its statement — the object §L's leaf extractor consumes — with every Fiat–Shamir and
commitment check enforced as outer AIR constraints. Today that is the validated reference
logic, not yet the constraints.

---

## 2. The recursive extractor `E_rec` (construction)

Depth-`t` balanced fold tree, `N = 2^t` leaves; each internal node verifies its ≤2 children.

1. **Extract the root trace.** Apply §L (Steps 1–4) to the OUTER proof `R`-proof: rewind `P*`
   over fresh outer FS query randomness, harvest accepting outer query-openings, list-decode
   the outer trace columns inside the Johnson radius, DEEP-select the unique codeword,
   interpolate → outer trace `T_root ∈ Val^{H_out×w_out}`. Succeeds when `ε > κ_outer`.
2. **Read the embedded children.** By the gadget-1/2 bindings, `T_root` contains, for each
   child, its Merkle roots + opened values + the in-AIR-recomputed FS transcript + its public
   values (gadget 5). These are, by §1, a child accepting transcript — NOT re-proven, but
   READ OFF the extracted outer trace.
3. **Recurse.** Treat each embedded child transcript as a §L instance and apply §L to it. For
   an internal child, this yields ITS embedded grandchildren (chain down). For a LEAF child,
   §L Step 4 reads off the account-creation witness (`pad_seed`, `account_projection`,
   `masked_projection`, `syndrome`, `packed_witness_commitment`).
4. **Output** the witness of every leaf. ∎

Commit-fixing chains because the outer AIR binds the inner roots in-circuit (gadget 2): once
`T_root` is fixed (Step 1), every descendant root is a FIXED function of it, up to
Poseidon2-Merkle collision — the named-standard CR term, charged once per node in §4.

---

## 3. Round-by-round composition (why it does not blow up)

The Fiat–Shamir compilation of each layer is round-by-round (RBR) knowledge sound
(Block–Garreta–Katz–Thaler–Tiwari–Zhang, eprint 2023/1071): each FS challenge is a separate
RBR round whose state-advancing error is the §L per-layer error. RBR composition is ADDITIVE,
not multiplicative — re-using a challenge in-circuit (gadget 1 reproduces the inner FS draw)
does not square the error; it inherits the inner per-round error plus the outer round that
binds it. Hence over the tree:

```
    κ_rec  ≤  Σ_{nodes v in the fold tree} κ_v ,
```

with `κ_v` = the §L knowledge-soundness error of node `v`'s STARK (§4). A balanced binary
fold of `N` leaves has `2N − 1` nodes, so `κ_rec ≤ (2N−1)·max_v κ_v ≈ 2N·κ_node` (for N=1
this loosens — a single proof is the un-unioned `κ_node`, §4). The additive-per-round result
is eprint 2023/1071 (FS security of FRI, RBR knowledge soundness); the lift to a recursion
TREE is ethSTARK-recursion folklore (a clean stated theorem for the tree case is not in 1071
— labelled as such, not over-cited). There is no exponential blow-up, but there IS an `O(N)`
union (see §4, the honest cost).

---

## 4. Concrete `κ_rec` for OUR params (deg-8) — and the honest N-union cost

Per node, §L with the deg-8 field gives (re-running §L.4 with `|F| = p⁸ = 2^247.9`):

- **Query phase:** `(√ρ)^s · 2^{−g} = (1/4)^{56} · 2^{−16} = 2^{−128}` (conservative Johnson) — unchanged by the field.
- **FRI commit/fold + DEEP additive:** `≈ |L_node|·log|L_node| / |F|`. The OUTER trace embeds the inner verifier (~400–600 Poseidon2 perms + the gadget columns); take `|L_node| ≈ 2^{20…24}` ⇒ additive `≈ 2^{−220…−224}` — DEEP in the noise (this is exactly what deg-8 bought: the §6(i) field-compounding is gone).
- **Merkle-CR + sponge floors:** `≈ 2^{−124}` (the accepted creation ceiling — `DIGEST_ELEMS=8`, capacity 8; same as the leaf, NOT improved by deg-8, per the 2026-06-19 decision).

So per node `κ_node ≈ 2^{−124}` (floor-bound, as accepted). Composing over the tree:

```
    κ_rec  ≈  2N · 2^{−124}  =  2^{−(124 − 1 − log₂N)} .
```

| Block size N | nodes (2N−1) | `κ_rec` (= nodes · 2⁻¹²⁴) |
|---|---|---|
| 1 (single)   | 1     | 2⁻¹²⁴ (un-unioned per-node floor; the `2N` form is a loose upper bound that over-charges here) |
| 1024 (2¹⁰)   | ~2¹¹   | **2⁻¹¹³** |
| 4096 (2¹²)   | ~2¹³   | **2⁻¹¹¹** |
| 2²⁰          | ~2²¹   | 2⁻¹⁰³ |

**HONEST COST (the one term that does not vanish):** the recursion's aggregated knowledge
soundness for a block of `N` leaves is `≈ 2^{−(123 − log₂N)}` — i.e. the per-leaf 124-bit
floor erodes by `~log₂N + 1` bits from the N-fold extraction union. At the production batch
sizes this is **~2⁻¹¹¹…⁻¹¹³**, still a strong PQ level but below the per-leaf 124. This is
inherent to aggregating N independent statements under one proof (every recursive STARK pays
it) and is the recursive analogue of the multi-target term already carried in the SUF-CMA
spine. It is NOT fixable by the field/digest (those set the per-node floor); only a smaller
block or a different aggregation shape changes the `log₂N`.

---

## 5. Resolved vs residual (the §6 open items, revisited)

- **§6(i) in-circuit FS field-compounding — RESOLVED.** The worry was the per-round challenge
  union eroding 128 over the deg-4 (~124-bit) field. With deg-8 (2^247.9) the per-round FS
  collision/union term is `≈ (#rounds)/2^{247.9}`, negligible; the binding floor is now the
  Merkle-CR/sponge 124 (accepted), not the challenge field. Gadget 1 reproduces the inner FS
  draw faithfully (golden-tested element-for-element), so the in-circuit transcript is the
  real one — no extra in-circuit-FS soundness loss beyond the standard QROM-FS seam (§5 next).
- **§6(ii) HidingFriPcs hiding under recursion — CONDITIONAL, identified.** The inner proof is
  ZK (salted-leaf `MerkleTreeHidingMmcs` + random codewords). The recursive extractor reads
  the inner opened values from `T_outer`; for the recursion to remain ZK (not leak the inner
  witness through the outer proof), the outer AIR must treat the inner opened values as PRIVATE
  outer-witness columns (never outer-public) and the outer proof must itself be ZK over them.
  This is a constructible property (the outer HidingFriPcs hides the outer trace, which
  contains the inner openings) but is an OBLIGATION on the `recursive_air` assembly, not yet
  built — flag for the assembly step. Extraction soundness (this doc) does not depend on it;
  witness-hiding does.
- **§6(iii) Lean/EC mechanization — STILL A SEAM.** The RBR composition (§3) and the §L leaf
  extractor are paper proofs; porting the additive-composition seam to EasyCrypt is the same
  GAP-B/D mechanization debt the dossier already carries. Named, not closed.

Additional carried items: the QROM-FS loss (Don–Fehr–Majenz–Schaffner 2019 / Liu–Zhandry
2019) applies once per layer to each FS-derived challenge; under RBR it composes additively
into `κ_rec` as a polynomial query factor (already a NAMED-STANDARD seam for the leaf). The
Poseidon2-permutation-ideal assumption (`A_poseidon2_perm_ideal`) underlies every in-circuit
hash, charged once.

---

## 6. What this argument establishes (and what it does not)

- The recursion's **knowledge-extractor argument** is CONSTRUCTED (§2) and its error computed
  for OUR params (§4), reducing recursive knowledge soundness to: the LEAF extractor (§L, OUR
  math) + RBR composition (cited, eprint 2023/1071 + tree folklore) + the INTENDED gadget
  bindings (§1, once lifted) + the named-standard Poseidon2-CR/QROM-FS seams. The high-proof-
  risk recursion item moves from "open/unknown" to "a concrete argument with an explicit
  budget over named seams" — but it is NOT closed: it is a paper sketch whose gadget bindings
  are validated as references, not yet as AIR constraints (see §1 caveat).
- **Honest residuals:** (1) `κ_rec ≈ 2^{−(123−log₂N)}` — the N-fold union erosion (≈2⁻¹¹¹…⁻¹¹³
  at production batches), inherent and documented, NOT a per-node weakness; (2) the §5(ii)
  outer-ZK obligation on the (unbuilt) `recursive_air` assembly — extraction-independent but
  required for witness-hiding; (3) the EC mechanization seam (§5(iii)); (4) this presumes the
  mechanical AIR build (gadget `Air<AB>` lifts + the deferred leaf-eval / chunk-recomposition)
  faithfully encodes the validated reference logic — the references are tested, the encoding
  is pending.

---

## 7. Citations

- A. Block, A. Garreta, J. Katz, J. Thaler, P. R. Tiwari, M. Zhang. *Fiat–Shamir Security of
  FRI and Related SNARKs.* eprint 2023/1071. — RBR knowledge soundness; additive recursive
  composition.
- Ben-Sasson, Carmon, Ishai, Kopparty, Saraf. *Proximity Gaps for RS Codes.* eprint 2020/654.
- Ben-Sasson, Goldberg, Kopparty, Saraf. *DEEP-FRI.* eprint 2019/336.
- StarkWare. *ethSTARK Documentation* v1.1, eprint 2021/582, §5 + recursion.
- Don, Fehr, Majenz, Schaffner. *Security of the Fiat–Shamir Transformation in the QROM.*
  CRYPTO 2019; Liu, Zhandry, *Revisiting … QROM*, CRYPTO 2019. — QROM-FS loss.
- Cross-ref: `fri_aok_extractor_instantiation.md` (leaf extractor §L), the validated gadgets
  (`descender-recursion/src/{challenger,merkle,fri_fold,deep,public_io}.rs`).
