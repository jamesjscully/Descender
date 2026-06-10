# Red-team VECTOR 5 — Boundary / Composition / Query-model attack

**Date:** 2026-06-10 · **Attacker:** red-team (authorized defensive cryptanalysis)
**Target claims:** the exact 96<114 box frontier; the 214-bit checked-core
second-preimage closure with "NO query cap"; the union-bound composition; the
2^33 query-cap dependence.

**HEADLINE RESULT — PARTIAL BREAK (high severity, quantitative):**
The GAP-C re-model's central claim — that the 214-bit checked-core message-binding
term (#6) closes at a generic `q = 2^64` with **NO query cap** — is **WRONG**. It
silently uses a **single-target** second-preimage bound `q/2^214 = 2^-150` where the
actual SUF-CMA experiment is **multi-target** in the number of honest signing
queries `T`. Restoring the multi-target factor gives the per-event term
`T·q/2^214`, which at the spec's own SUF-CMA budget `T = 2^32` and a generic
`q = 2^64` is **2^-118 ≫ 2^-128** — i.e. it breaches the stated SUF-CMA target by
~10 bits, and the security claim is again **load-bearing on a query cap** (it needs
`log2 T + log2 q ≤ 86`, e.g. `T ≤ 2^22` at `q = 2^64`, NOT `T = 2^32`). This
directly refutes the dossier's §1 statement that "the budget closes at a generic
classical query budget q = 2^64 with NO query cap."

---

## (a) THE 96<114 FRONTIER — VERIFIED SOUND, no break found.

I independently reproduced the entire exact-box certificate from first principles
(exact rational arithmetic, not the Lean `native_decide`):

- **Lattice membership / completeness.** Extracted `reducedBasis` (23×23) from
  `Descender/Generated/ResponseLinkExactCertificateN32Q12289Coords03B96.lean`.
  Confirmed every basis row satisfies the coded-kernel relation
  (`M·v ≡ 0 mod q` for the 12×23 `responseMatrix`, and `v[0]≡v[3]≡0 mod q`).
  Confirmed `|det(reducedBasis)| = q^14 = 1791561079655532333095965150039192629866166308243408855041`
  exactly, and that the constraint system has rank 14 mod q — so the basis spans
  the **full** coded-kernel lattice, not a sublattice that could miss short vectors.
- **Gram-Schmidt soundness.** Recomputed the exact GS `‖b*_i‖²` diagonal and
  compared element-by-element to the file's `gramSchmidtDiagonal` — **exact match
  on all 23 entries** (e.g. 80192, 6535992391/80192, …, 151019521). So the
  enumeration's Fincke–Pohst pruning intervals are sound; no "fudged GS → skipped
  vectors" attack exists. (Note: there is *no* Lean theorem asserting the GS data
  is correct for `reducedBasis`; it is consumed as data. I closed that gap by
  external recomputation — it holds.)
- **Independent enumeration.** Re-implemented the full L2-ball enumeration
  (radius² = 23·96², the ball that contains the entire `[-96,96]^23` box). My run
  reproduces the certificate **exactly**: nodes = 1,123,628, leaves = 99,459,
  nonzero-in-cover = 99,458, **min L∞ = 114**. No nonzero coded-kernel vector has
  L∞ ≤ 96; the minimum in the gap is exactly 114.
- **Centering / sign convention.** `hidden_source_fallback_centered_mod_q`
  (`production_reference.rs:5683`) maps `[0,6144]→[0,6144]`, `[6145,12288]→[-6144,-1]`;
  the box check is `centered.abs() > 96 → reject`. No off-by-one, no `Q/2` boundary
  slack (6144 ≫ 96), no coordinate that can absorb >48 — the [-96,96]^23 box is
  enforced exactly on the residue field.

**Verdict (a): the exact-box claim is correct.** No (96,114] boundary forgery
exists in this lattice. Reproduction scripts: `/tmp/kwork/run_enum.py` (re-derives
1123628/99459/99458/min-114 from `reducedBasis.json` in <10 min, exact `Fraction`
arithmetic).

---

## (b)+(c) THE MULTI-TARGET BREAK ON THE 214-BIT CHECKED CORE — the real finding.

### The construction.
214-bit checked-core challenge =
`truncate_and_mask(SHA256(frame))` where `frame` (an injective encoding) includes
`message_digest`, slot/state digests, and `proof_commitment`
(`ALGEBRAIC_CHALLENGE_BYTES=23` last-byte-masked `0x3f` → 22·8+6 = 182 bits, plus
`CHALLENGE_HARDENING_BYTES=4` → 32 bits, total **214**). Event #6
(`checkedCoreMessageBinding`) is the claim that an adversary cannot transfer an
honest response to a different message by colliding this 214-bit challenge.

### The GAP-C re-model and exactly where it is wrong.
`formal/easycrypt/break_checked_core_message_binding.ec` (lines 20–55) re-models #6
from a free birthday collision (`q²/2^214 = 2^-86` at `q=2^64`, which "BLOWS the
budget") to a **single-target second-preimage** against ONE honest frame `f0`:
the EC module `SPR` is initialised with a single `f0`, the reduction `R` fixes
`R.honest_tuple = h` (a single honest tuple), `break_le_spr` bounds
`Pr[Exp(A).run(h)]` for that single `h`, and the named axiom
`sha256_second_preimage_resistance` bounds the SPR game **for a single fixed `f0`**
by `q/2^214 = 2^-150`. `gap_c_remodel_spine_wiring.ec:47,52` then plugs
`eps_checked = q/2^214` (single-target, `q=2^64`) into the spine, concluding
`2^-150 ≤ 2^-133` with "NO query cap."

**The error:** the SUF-CMA experiment the spec actually defines
(`descender-signature-spec.md:77`: "advantage ≤ 2^-128 at query budget T = 2^32")
gives the adversary **`T` signing queries**, hence **`T` distinct honest
checked-core challenges** `c_1,…,c_T`. A SUF-CMA forgery wins by producing ANY
accepted `(m', σ')` not previously queried — in particular by finding a message
`m'` whose frame hashes to **any one** of the `T` pinned honest challenges `c_i`
(then the honest response for query `i`, which is bound to `c_i` + that query's
`proof_commitment`/handle, transfers to `m'`). That is a **multi-target second
preimage** over `T` targets, whose generic advantage is

> **`T · q / 2^214`**,  not  `q / 2^214`.

The union-bound spine (`spike_union_bound.ec`) is a union over the **9 break-event
TYPES** only (`break_card9`, `break_univ` has 9 elements). It does **NOT** quantify
over the `T` honest signing queries. So the per-event mass
`mu dout (break_pred o checked_core)` against the real experiment distribution
`dout` (which contains `T` honest challenges) is `≈ T·q/2^214`, while the value
plugged in is the single-`h` `q/2^214`. **The `T` factor is dropped at the seam
between the per-event single-target reduction and the experiment.** This is exactly
the composition/boundary gap vector 5(c) targets: each per-event reduction is
individually sound, but the budget composes single-target bounds against a
multi-target experiment.

### Quantification (the break).

| scenario | `T` (signing) | `q` (hash) | `T·q/2^214` | vs 2^-128 |
|---|---:|---:|---:|:--:|
| dossier's claim (single-target, "no cap") | — | 2^64 | 2^-150 | (claimed safe) |
| **spec SUF-CMA budget** | **2^32** | **2^64** | **2^-118** | **BREACH (+10 bits)** |
| ledger deployment cap | 2^33 | 2^64 | 2^-117 | **BREACH (+11 bits)** |
| spec budget, stronger hash adv | 2^32 | 2^80 | 2^-102 | **BREACH (+26 bits)** |
| genuinely-safe regime | 2^33 | 2^33 | 2^-148 | ok |

To restore `2^-128` one needs `log2 T + log2 q ≤ 86`. At a generic `q = 2^64`
this forces `T ≤ 2^22` — **below the spec's own `T = 2^32`**. At the spec's
`T = 2^32` it forces `q ≤ 2^54`. **Either way a query/signature cap is
load-bearing**, contradicting the GAP-C "NO query cap" headline. The project's own
`per_event_epsilon_budget_closure.md:147-154` already states "the cap is
load-bearing"; the GAP-C re-model's removal of that dependence is the specific step
that fails, because it conflates single-target SPR with the multi-target SUF-CMA
game.

### Why the structural arguments do NOT save it.
- The "response is secret-key-bound, so one side is pinned first" argument (EC
  lines 36–52) is correct **per honest query** — but the adversary holds `T` such
  pinned targets and needs to beat only one. Pinning per-query does not reduce the
  target count below `T`.
- The consume-once lifecycle (#5/#7, proven =0) blocks **slot replay**, a different
  event. The multi-target transfer here uses a **fresh slot/state with a colliding
  214-bit challenge**, so it is not caught by the lifecycle guard — it is caught
  only by the (insufficient) 214-bit binding.

**Severity: HIGH.** It does not yet exhibit a concrete sub-2^128 forgery one can
run today (it is a tightness/closure break of the *proof's* claimed bound, sized
in the 2^-118…2^-102 range at deployable parameters), but it falsifies the headline
security statement and re-introduces a load-bearing cap the dossier claims was
removed. A reviewer accepting `T = 2^32` (the spec's number) gets `2^-118`, i.e.
the SUF-CMA advantage bound the proofs establish is **2^-118, not 2^-128**.

---

## (d) QUERY-CAP — what breaks if the chain does not enforce 2^33.

Given (b)/(c): the safe regime needs `log2 T + log2 q ≤ 86`. If the deployment does
NOT enforce a signing-query cap `T`, then `T` can grow to the adversary's hash
budget and the checked-core term degrades toward the free-birthday `q²/2^214`
(`2^-86` at `q=2^64`) the project already flags as budget-blowing. The cap that
must be enforced is on **`T` (honest signatures issued per key / per scope)**, and
it must be tighter than `2^32` at a `2^64` hash adversary (need `T ≤ 2^22`). The
quantum case is worse: BHT/Grover-style multi-target second-preimage scales as
`(T·q_q)`-style with a cube-root improvement on the `2^214`, pushing the safe `T`
even lower; the ledger's `q_q = 2^16` quantum cap is doing more work than the
single-target re-model acknowledges.

---

## RESIDUAL / hand-off to a better-resourced adversary.
1. **Concrete multi-target SPR realization.** Build the actual `T·q` second-preimage
   search against the real `truncate_and_mask∘SHA256` 214-bit map over `T = 2^32`
   honest frames; a memory-hard rainbow/precomputation (Hellman–Oechslin against a
   214-bit truncated output) plausibly beats `2^-118` and turns the closure break
   into a runnable forgery near `2^107`-class effort. Worth `g6k`/GPU time.
2. **Generalized-birthday across honest challenges.** The `T = 2^33` honest 214-bit
   challenges among themselves collide at `T²/2^214 = 2^-148` — not directly useful,
   but a Wagner k-tree over (honest challenge ⊕ adversary-controlled frame nibbles)
   may shave the multi-target exponent further; quantify against the masked
   last-byte structure (`0x3f`) which reduces effective entropy in one byte.
3. **Re-do the union bound with the `T` factor explicit.** The honest fix is to
   re-state the spine per-event term as `T·eps_spr(n)` and re-derive the required
   `(T,q)` envelope; this will show the 214-bit width must grow (or `T` shrink) for
   a genuinely cap-free `2^-128`. The leaf/registry events (n=256) survive
   multi-target (`T·q/2^256 = 2^32·2^64/2^256 = 2^-160`), so only the narrow
   214-bit checked core is the problem — a width increase to ≥246 bits would make
   it cap-free at `T=2^32,q=2^64`.

## Files / reproduction.
- Independent box re-derivation: `/tmp/kwork/run_enum.py` (+ `reducedBasis.json`,
  `responseMatrix.json` extracted from the generated Lean cert).
- Target source: `formal/easycrypt/break_checked_core_message_binding.ec`
  (single-target SPR), `formal/easycrypt/gap_c_remodel_spine_wiring.ec:47,52`
  (single-target `eps_spr` plug-in), `spike_union_bound.ec:109,131` (union over 9
  event types, no `T`), `formal/spec/descender-signature-spec.md:77` (`T = 2^32`
  SUF-CMA budget), `research/cryptanalysis/per_event_epsilon_budget_closure.md:147`
  (project's own "cap is load-bearing" admission, contradicted by GAP-C).
