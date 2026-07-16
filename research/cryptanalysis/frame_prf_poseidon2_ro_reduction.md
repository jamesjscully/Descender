# Frame-PRF → Random-Oracle reduction for the KoalaBear-Poseidon2 sponge

**Scope.** Replaces the "SHA-256-is-an-RO" idealization for BOTH (a) the
projection-mask PRF (`descender-mask-prf`, `tools/descender-mask-prf/src/mask.rs`)
and (b) the checked-core Fiat–Shamir challenge derivation
(`hash_frame_to_challenge_bytes_poseidon2` / `poseidon2_hash_bytes`, same file
lines 156–168), now that both are native KoalaBear-Poseidon2 sponges.

**Status one-liner.** The RO replacement reduces to ONE named permutation
assumption + the standard sponge indifferentiability theorem. The asymptotic
generic floor is **c/2 ≈ 123.95 bits**, but the selected theorem is explicitly
bounded to at most `2^54` total permutation queries; at that horizon the loose
classical indifferentiability term is below `2^-139`. The concrete pinned
permutation/QROM import and external review remain open.

---

## 1. The construction (exact, as deployed)

### 1.1 Permutation
`P : F_p^16 → F_p^16` is the width-16 KoalaBear Poseidon2 permutation, pinned
byte-identical to Plonky3 `default_koalabear_poseidon2_16()`
(`tools/descender-mask-prf/src/poseidon2.rs`):

- Field `F_p`, p = 2^31 − 2^24 + 1 = 0x7f000001, so **log2 p = 30.98868…**
  (NOT 31 — this matters for the bound below).
- Structure: initial external MDS-light layer; 4 initial full (external) rounds;
  20 partial (internal) rounds with the pinned diagonal V; 4 terminal full
  rounds. S-box x ↦ x^3 (alpha = 3). All round constants copied verbatim from
  `p3-koala-bear-0.6.1`.

### 1.2 Sponge
Standard sponge over P with **WIDTH = 16, RATE r = 8, CAPACITY c = 8** field
elements (`mask.rs:21`). Absorb is capacity-preserving (first permutation seeds
the r rate lanes; subsequent absorbs ADD input into the rate lanes and carry the
c capacity lanes through P, `run_sponge`, `mask.rs:110–146`). Padding: a single
ONE element at index n_input, zero-fill to a multiple of r (`build_elems`,
`mask.rs:99–106`) — i.e. injective "10*" style domain padding. Squeeze reads
rate lanes from post-permutation states (`output_source`).

Two domain-separated instances of the SAME sponge:

- **Mask PRF (C2).** Absorb the canonical field lane `MASK = 0x4D41534B`, then
  `pad_seed[32] ‖ slot_id.to_le_bytes()[8]` as byte lanes. Squeeze
  NUM_SQUEEZE = 36 field lanes, then the
  arithmetic wide-then-reduce `mask_j = (Σ_i lane_{j,i} · p^i) mod q12289`
  (6 lanes/mask, Horner, `wide_reduce_masks`) → 6 masks in Z_q12289. The explicit
  first lane separates C2 from the `NULL` nullifier and other account sponges.
- **FS challenge.** `poseidon2_hash_bytes::<OUT>(domain, input)` absorbs
  `domain ‖ input` one byte per lane, squeezes OUT lanes, emits the low byte of
  each. Domain prefix makes the two sponges non-colliding by construction.

We model the abstract object
`H : {0,1}* → F_p^{NUM_SQUEEZE}` (resp. byte string) as a random oracle and ask
how far the Poseidon2 sponge is from H.

---

## 2. Indifferentiability of the sponge from an RO

### 2.1 Theorem cited (classical)
**Bertoni–Daemen–Peeters–Van Assche, "On the indifferentiability of the sponge
construction", EUROCRYPT 2008.** For a sponge with capacity c built on an *ideal*
permutation P (random invertible permutation oracle), the sponge is
(t, q_P, ε)-indifferentiable from a random oracle, with

    ε  ≤  N(N+1) / 2^c   ≈  N^2 / 2^c ,

where N = q_P is the total number of queries the distinguisher makes to P (and
P^{-1}). Equivalently the differentiating advantage stays negligible until
N ≈ 2^{c/2}, i.e. the sponge delivers **min(c/2, output-length) bits** of generic
RO security. (The injective "10*" padding in `build_elems` is what licenses the
standard indifferentiability simulator: distinct messages map to distinct
absorbed inputs, so the simulator can keep a consistent path graph.)

### 2.2 Concrete instantiation over KoalaBear
Capacity is **c = 8 field elements**, each carrying log2 p = 30.98868 bits:

    c_bits = 8 × 30.98868  =  247.9095 bits.

The BDPVA bound is N^2 / 2^{c_bits}, with N counting permutation queries (here a
query is one evaluation of the width-16 Poseidon2 permutation). Hence

    ε_indiff(N)  ≤  N^2 / 2^{247.9095},

and the generic security floor is

    **c_bits / 2  =  123.9547 bits.**

### 2.3 Concrete check against the 2^-128 target

    123.9547  <  128.    ⇒  FLAG: the sponge does NOT clear 128-bit RO security.

Honest reading: to drive ε_indiff below 2^-128 you need N^2 ≤ 2^{247.9095−128},
i.e. N ≤ 2^{59.95}. So the sponge is a sound RO replacement only for adversaries
making fewer than ≈ 2^{60} permutation evaluations; pushed to N ≈ 2^{62} the
indifferentiability advantage is ≈ 2^{124−128}-ish = 2^{−123.9}, i.e. it tops
out at the c/2 ≈ 124-bit ceiling. This is the SAME 124-bit ceiling that the
degree-4 KoalaBear challenge-extension field already sits at (config.rs:103,
`Challenge = BinomialExtensionField<Val,4>`, "~124 bits"), so the sponge is NOT
the unique weakest link — but neither is it a clean 128.

---

## 3. The permutation assumption (replaces "SHA-256 is an RO")

The indifferentiability theorem of §2 is *conditional on P being ideal*. That
condition is the named assumption that replaces the old SHA-256-RO idealization:

> **Assumption A_poseidon2_perm_ideal.** The pinned width-16 KoalaBear Poseidon2
> permutation P (RF = 8 full + 20 partial rounds, alpha = 3, pinned constants)
> is indistinguishable from a uniformly random invertible permutation on F_p^16,
> for all adversaries within the scheme's query/time budget (N < 2^60 perm
> queries). Equivalently: there is no structural distinguisher (invariant
> subspace, algebraic/Gröbner, rebound/differential, integral) on P with
> advantage above the generic 2^{-c/2} ≈ 2^{-124}.

**Cited vs assumed:**

- CITED (theorem, no assumption): the sponge→RO reduction itself (BDPVA 2008),
  given an ideal P. The injective padding licensing the simulator is CITED.
- CITED (design-level evidence, not a proof): Poseidon2 round count and the
  external/internal layer design follow Grassi–Khovratovich–Schofnegger
  (Poseidon2, CHES 2023) and the original Poseidon (USENIX Sec 2021) security
  argument vs. the listed algebraic/statistical attacks for the 31-bit field
  family; KoalaBear is the same field class Plonky3 ships (NOT the 14-bit
  small-field concern called out in dossier §3(b)).
- ASSUMED: A_poseidon2_perm_ideal itself — that the concrete pinned P has no
  better-than-generic distinguisher. This is the irreducible primitive
  assumption, exactly analogous to "SHA-256 compression behaves ideally," and is
  what third-party cryptanalysis must vet. It is NAMED-STANDARD, not PROVEN.

Net effect on the assumption ledger: the old
`A_sha256_projection_mask_frame_prf` (SHA-256-as-RO) is RETIRED and replaced by
the pair { A_poseidon2_perm_ideal (primitive) ; BDPVA sponge-indiff (theorem,
discharged) }. The Lean/EC abstract RO model (`frame_prf_ro_reduction.ec`) is
re-pointed: its `dmask`/`ROsample_fresh_uniform` axioms now denote the
sponge-instantiated RO under A_poseidon2_perm_ideal rather than SHA-256-as-RO.
No EC proof body changes — only the meaning of the abstract RO does.

---

## 4. How the §3 RO feeds the two uses (theorem statements)

Let H be the RO that the sponge instantiates (advantage ε_indiff(N) ≤ N^2/2^{c}
away from a true RO, §2). The downstream claims are stated against H and then
the ε_indiff term is added once.

### 4.1 Mask hiding (re-host of frame_prf_ro_reduction.ec)
The EC file proves, for the *abstract* RO H, that the production mask
mask = wide_reduce(H(MASK‖pad_seed‖slot)) is
(qQ·2^{−Hseed})-indistinguishable from a
fresh uniform mask, where Hseed = 8·32 = 256 (pad_seed min-entropy). Composing
with §2 and the wide-then-reduce uniformity gap:

> **Theorem (mask hiding, Poseidon2-instantiated).** For any distinguisher
> making ≤ qQ RO/permutation queries with N total permutation queries,
>
>   Adv_hide ≤ qQ·2^{−256}
>            + 6·2^32·12289/(2130706433^6)
>            + N(N+1)/(2130706433^8)
>            + Adv_poseidon2_permutation.
>
> Terms: (i) qQ·2^{−256} is the seed-query term; (ii) the second term charges
> six q12289 masks across the selected 2^32 account horizon; (iii) the third is
> the loose sponge indifferentiability term; and (iv) the final term is the
> irreducible concrete-permutation assumption. At `qQ,N ≤ 2^54`, the first
> three terms are dominated by the reduction term and together remain below
> roughly `2^-137`; the concrete permutation/QROM term is still open.

### 4.2 FS challenge soundness
The checked-core Fiat–Shamir transform is sound in the RO model when the RO
output entropy ≥ the required challenge entropy. The deployed FS challenge is
**246 bits** (`total_challenge_bits_after_mask = 246`,
production_reference.rs:44613; CHALLENGE_HARDENING_BYTES = 8). The sponge supplies
that many bits with collision/preimage resistance bounded by §2.

> **Theorem (FS soundness, Poseidon2-instantiated).** Under A_poseidon2_perm_ideal,
> the FS challenge derived by `poseidon2_hash_bytes` is indistinguishable from a
> 246-bit RO challenge up to N^2·2^{−247.91} (indiff) plus the per-byte reduction
> bias (≈ OUT·2^{−23}, immaterial for a challenge: only collision/preimage
> resistance is needed, not exact uniformity — `mask.rs:154–155`). The
> rewinding/extraction soundness then proceeds exactly as in the SHA-256-RO
> analysis with the challenge RO replaced by H.

---

## 5. The honest residual on the concrete permutation/QROM bound

The asymptotic generic floor remains **c/2 = 123.95 bits**, so an unbounded
"128-bit RO" label would be false. The selected theorem avoids that claim and
uses the explicit `2^54` query horizon, where the concrete arithmetic clears
q128. The remaining deficiency is a reviewed quantum indifferentiability and
concrete Poseidon2 permutation theorem at that bounded horizon. Options, in
increasing cost:

1. **Accept the 124-bit ceiling, re-state the headline target as 2^{−124}.**
   Defensible because the challenge-extension field already pins the scheme's
   algebraic-soundness ceiling at the same ~124 bits (config.rs:103) and the FRI
   soundness is sized to 128 *conservatively* (config.rs:78) — so 124 is the
   pre-existing scheme-wide floor, and the sponge does not make it worse. This is
   the cheapest honest move: relabel the global target 2^{−124}, not 2^{−128}.
2. **Widen capacity to c = 9 field elements** (RATE 8 → 7, or WIDTH 16 → 18 with
   RATE 8 cap 10). c = 9 ⇒ c_bits = 278.9 ⇒ c/2 = 139.4 bits > 128. Cost: a
   permutation/sponge re-parameterization that breaks byte-identity with the
   in-circuit AIR and the Plonky3 default-16 instance — i.e. a re-host of the
   account-creation STARK permutation. Heavy; touches the AIR, Merkle, and FRI
   transcript. (Cross-ref MEMORY: stark-field-rehost.)
3. **Use a wider-permutation Poseidon2 (WIDTH 24, RATE 16, cap 8 → cap stays 8)
   does NOT help** — capacity, not width, sets the floor. Only raising c helps.

**Recommendation:** Option 1 (relabel to 2^{−124}, with the sponge floor
explicitly itemized) is the honest and lowest-risk choice; Option 2 only if a
hard 128 is contractually required. Either way the sponge floor must be written
into the dossier as an explicit 2^{−124} line item, not hidden inside a 2^{−128}
claim.

---

## 6. QROM / post-quantum caveat (honest)

- **Sponge indifferentiability survives in the QROM**, but the bound and the
  proof technique change. CITED: Czajkowski, Bruinderink, Hülsing, Schaffner,
  Unruh, "Quantum Indistinguishability of Random Sponges" (CRYPTO 2019) and
  Czajkowski et al. / Unruh's quantum-lazy-sampling line — the sponge over an
  ideal *random* permutation is indifferentiable from a quantum-accessible RO,
  but the proof needs the one-way-to-hiding (O2H) / compressed-oracle machinery
  and the *quantum* indifferentiability bound degrades the query exponent. The
  EC discharge note (`frame_prf_ro_discharge.md` §5) already carries the matching
  classical→quantum substitution Pr[bad] ≤ q·2^{−H} → q^2·2^{−H} via O2H.
- **Carried loss / what is assumed quantumly:**
  - A_poseidon2_perm_ideal must hold against a *quantum* distinguisher (ideal
    *quantum* permutation oracle). This is the post-quantum strengthening of the
    primitive assumption; it is ASSUMED, not proven for the concrete P.
  - The indifferentiability ceiling is unchanged at c/2 (Grover/quantum-walk on
    a permutation does not beat the c/2 capacity collision floor for the generic
    bound), so §2.3's 123.95-bit flag stands identically in the QROM.
  - Fiat–Shamir-in-QROM: the FS soundness of §4.2 inherits the standard QROM-FS
    loss (a polynomial query-extraction factor; Don–Fehr–Majenz–Schaffner 2019,
    Liu–Zhandry 2019), already noted as a NAMED-STANDARD seam in the dossier
    (action-map QROM line). No additional loss beyond what was carried for the
    SHA-256-RO version, except that the RO is now the sponge, so the loss is
    routed through the quantum sponge-indiff bound rather than SHA-256.

**QROM bottom line:** no step is hand-waved, but two things are honestly
ASSUMED, not proven: (i) A_poseidon2_perm_ideal against quantum adversaries, and
(ii) the concrete EC formalization of the quantum sponge-indiff bound is NOT yet
machine-checked — the EC file models an abstract RO and the QROM substitution is
argued on paper (discharge note §5). Porting it to a machine-checked O2H bound is
named future work, identical in status to the carried action-map QROM seam.
