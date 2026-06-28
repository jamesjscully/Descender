# Assumption-Boundary Stress Test (2026-06-27)

> Classification: `ACTIVE_RESEARCH`. Red-team of the deployable proof family's
> full assumption boundary, hunting (a) assumptions that look standard but aren't
> well-established at our parameters, and (b) composition seams where two sound
> assumptions meet and leave an exploitable gap. Authority for current
> assumptions remains `../../spec/security_model.md`.

Four pillars probed adversarially: (A) lattice/coded-SIS core, (B) Poseidon2
primitive + 124-bit floor, (C) FRI-AoK + FS/QROM at degree-8, (D) cross-assumption
seams. **The boundary is NOT clean — it has one quantum shortfall and one real
composition seam that must be addressed before the named assumptions can be
carried safely.** No runnable forgery exists against the fail-closed build today.

## Genuine findings (ranked)

### F1 — 🔴 Quantum FS-grinding soundness ≈ 2⁻⁶⁴, not ≈2⁻¹²⁰ (corrected a prior error)

The earlier `transcript_qrom_theorem` / `fri_aok §9` drafts priced quantum
soundness by Grover-halving only the 16-bit PoW (`56·2 + 8 ⇒ 2⁻¹²⁰`). **Wrong for
a non-interactive FS proof.** The prover commits its δ-far oracle once, then grinds
the FS nonce, which re-derives all 56 query positions per attempt. The whole
classical budget `s·(½log_blowup)+g = 128` bits is one grinding search of
marked-density `2⁻¹²⁸`; Grover solves it in **2⁶⁴**. Consequences:

- Quantum FS soundness at deployed params (s=56, g=16) ≈ **2⁻⁶⁴**.
- The proposed `g:16→32` fix reaches only `2⁻⁷²` — **insufficient**.
- 128-bit *quantum* needs classical budget ≈ 256 bits ≈ **s≈120 queries (~2× FRI
  proof size)** or an infeasible PoW.

Corrected in `research/path2_account_stark_transcript_qrom_theorem_v1.md` and
cross-noted in `fri_aok_extractor_instantiation.md` §9. **This is a real PQ
shortfall, not just a label.** Classical soundness is unaffected (2⁻¹²⁴).

### F2 — 🔴 mask-PRF ↔ lifecycle pad-seed freshness seam (hiding break under reorg)

`masked_projection` hides `account_projection` only if `(pad_seed, slot_id)` is
single-use (mask freshness). The in-code guard is a **per-account, in-memory
`BTreeSet`** at construction (`production_reference.rs:58449-58468`) — there is NO
global, persistent, reorg-safe uniqueness invariant; the spent-ledger check fires
at spend, not admission. Under a chain reorg / backup-restore that re-admits the
same `(pad_seed, slot_id)`, an observer computes
`masked_projection − masked_projection′ = account_projection − account_projection′`
from two pre-spend packed-witness commitments — **key-free, no spend required**.
`spec/lifecycle_model.md` explicitly defers the reorg case to an uninstantiated
target-chain profile (rank-6 open), and `deployable_pad_hiding_VERDICT.md` carries
this as open item #7. **Reachable on any reorg-capable chain before that profile +
item #7 close.** A hiding/privacy break, not a forgery — but concrete and today-
reachable on deployment.

**Partial fix landed (2026-06-27, machine-checked):**
`Descender/Path2FullSpendConcrete.lean` adds `GlobalPadSeedRegistry`,
`padSeed_reuse_excluded_of_globalUnique`, and `padSeed_globally_injective_of_registry`
— no-`sorry` Lean proofs that a GLOBAL injective pad-seed admission registry (one
leaf per digest, cross-account) makes the mask-reuse precondition unsatisfiable AT
ADMISSION: any two admitted leaves sharing a pad-seed digest are the SAME leaf
(the full "no two live slots share a pad seed" invariant the per-account in-memory
guard lacks). This closes the cross-account duplicate-admission half of F2. The **reorg-freeing** half (what happens to a
pad-seed when its admission is reverted) remains coupled to the per-chain finality
profile (`DIR-LIFE-001`, D2) and is NOT closed here. So F2 reduces to: implement
the global registry in the consensus admission path (engineering) + define the
reorg/free policy (deployment decision D2).

UPDATE (2026-06-27, q128 worktree): the global reorg-safe registry is IMPLEMENTED and tested as a
deployable reference (`descender-account-stark` module `pad_seed_registry`, `PadSeedRegistry`): an
injective `admitted_for: pad_seed → (leaf, height)` enforced on the canonical chain (rejects any
canonical re-admission of a pad-seed — the cross-account/persistent guard), PLUS the reorg-freeing
half the Lean defers — `reorg_to(new_head)` frees pad-seeds admitted in now-orphaned blocks (above
`new_head`) while keeping surviving canonical admissions blocked, with strictly-monotone admission
heights. 5 tests pass: canonical reuse rejection, non-monotone-height rejection, reorg frees
orphaned/keeps canonical, deep-reorg-then-fresh, and the canonical-chain-pad-seed-injective property.
So both halves of F2 are now realized in code; what remains is wiring this registry into the live
consensus admission path and pinning the reorg/free policy to the chosen finality profile (D2,
deployment decision) — not a missing-mechanism gap.

UPDATE (2026-06-27, q128 worktree — the core-AIR nullifier): the registry's KEY is now a real
proof-bound public value. The leaf relation exposes a new CLASS A-2 public output
`nullifier = Poseidon2(DOMAIN ‖ pad_seed ‖ slot)`, computed IN-AIR over the SAME pad_seed lanes that
feed C1 (seam-bound to C2) and the SAME slot lanes that feed C2 (context-bound to the public slot),
its squeezed digest constrained EQUAL to the public `nullifier` (`air.rs` CLASS A-2 block). Crucially
it is NONCE-FREE: unlike `pad_source_commitment` (C1), which is blinded by `nonce_c1` and so differs
across honest re-proofs of one pad_seed (making it useless as a reuse key), two proofs reusing one
pad_seed for one slot yield the IDENTICAL nullifier — exactly the single-use signal the registry
needs. This closes the gap flagged earlier ("`pad_source_commitment` is nonce-blinded, unsuitable; F2
needs a new public AIR output"): the registry can now key on a value the proof itself binds, so a
reused pad_seed cannot present a fresh-looking admission. Implemented + tested at production q128 knobs
(`air.rs` tests `nullifier_matches_recompute_and_is_nonce_free`, `forged_nullifier_violates_constraints`,
`nullifier_domain_lane_mismatch_violates_constraints`; threaded through `AccountCreationPublic`, the
batch digest chain, the recursion public-IO mirror, and the deployed verifier's transport wire; frozen
q128 KAT re-pinned). What remains for F2 is purely the consensus wiring + D2 policy — no missing
in-circuit mechanism.

### F3 — 🟠 Quantum Merkle/sponge collision floor ≈ 2⁻⁸²⁻⁹⁹ (not itemized)

The 8-element (~248-bit) Merkle/sponge digest has classical collision resistance
≈2⁻¹²⁴ but **quantum collision** ≈ `2⁻⁸²⁻⁹⁹` (BHT / parallel-Grover). Commitment
binding (hence AoK and salted-leaf hiding) rests on collision resistance, so the
honest post-quantum *binding* floor is ~82–99 bits, below the 124 indifferentiability
floor — and the dossier's quantum section addressed only indifferentiability/Grover
(2⁻¹²⁴), omitting this. For any PQ binding claim, ~82–99 is the number on the page.

## Hardening / honesty items (no break, fix before external review)

- **H1 — mask PRF has no domain tag.** `c2_data` absorbs bare `pad_seed‖slot_id`
  (40 lanes), no separator. Separation from the FS challenger is *length-incidental*
  (FS inputs are `domain‖frame` > 40 lanes), not capacity-IV based. Add an explicit
  mask domain prefix; length-prefix the FS domains (prefix-freeness is currently
  unenforced — `production_domain_schema_registry_is_globally_disjoint` checks only
  string uniqueness, not frame injectivity).
- **H2 — loaded gun in-tree (risk DOWNGRADED 2026-06-27 after verification).**
  `tools/parameter-search/src/poseidon2_q12289.rs` is a q12289 Poseidon2 with a
  **self-chosen, unvetted internal diagonal** ("deep security DEFERRED"), used by a
  retired q12289-native research family in `parameter-search`. VERIFIED: the
  deployed crates (`descender-account-stark`, `descender-mask-prf`,
  `descender-account-verifier`) have NO Cargo dependency on `parameter-search` and
  never reference this permutation — the Cargo.toml comments explicitly enforce the
  separation ("parameter-search must not depend on Plonky3", "signer-side builds
  must stay Plonky3-free"). So it is **crate-isolated** and cannot be silently
  wired into the deployed path; the residual risk is only a future refactor that
  crosses the crate boundary. Action: keep the crate boundary (already enforced);
  a non-deployment framing marker on the file is sufficient — do NOT code-gate (it
  would break the research family build).
- **H3 — EC endpoint composed on paper.** The 2⁻¹²⁸ spine is matched by hand across
  three `.ec` files with no shared `require` import; the multi-target union rests on
  `axiom sha256_multi_target_spr`. Link them by a shared import so a dropped/double-
  counted term can't hide (GAP-B/D seam).

## Framing corrections (not weaknesses, but the boundary was mis-described)

- **Coded-SIS underlies SPEND, not account CREATION.** On the active account-STARK
  path, creation forgery resistance = **STARK knowledge-soundness + Poseidon2
  CR/PRF**, not a lattice problem; the q12289 syndrome is a committed, range-checked
  *public echo*. The "114>96 in-box impossible" result is solid but narrow (per-
  matrix combinatorial non-existence over a dim-9 lattice, spend-side) and is one
  dropped verifier-check from its negative control (the bare-lattice coeff-68 ≤96
  forging vector) — keep a regression test pinning the `coord0≡coord3≡0 mod q`
  coded constraint.
- **The 124-bit sponge capacity is now the SOLE scheme-wide classical floor** (the
  degree-8 raise removed the field co-cap). Headline must be 2⁻¹²⁴, not 2⁻¹²⁸.
- **Poseidon2 α=3 / 8+20 rounds** is the vetted Plonky3 default-16 (byte-identical,
  clean) but the **thinnest algebraic margin in the family** (lowest S-box degree →
  slowest degree growth) — correctly labelled ASSUMED; an active-research assumption,
  no attack today.

## Verdict on the assumption boundary

The *named-standard* assumptions (FRI-AoK at degree-8, Poseidon2-as-ideal-
permutation, BCIKS/DEEP) are **well-established and correctly stated at our
parameters — classically.** The boundary's weak points are NOT the individual
crypto assumptions; they are:

1. **a quantum shortfall** (F1: FS-grind → ~2⁶⁴ quantum; F3: ~2⁸²⁻⁹⁹ quantum
   binding) — the scheme is ~124-bit classical but materially less post-quantum
   than advertised; and
2. **a composition seam** (F2: pad-seed freshness ↔ lifecycle) that breaks hiding
   under reorg because freshness is a carried assumption with an in-code
   enforcement gap, not an invariant.

Carrying FRI-AoK / Poseidon2 / coded-SIS is fine. Carrying them **without** (a) an
honest PQ headline + a query-count decision, and (b) a global reorg-safe pad-seed
single-use invariant, is NOT — those two are the exploitable gaps.

## Decisions surfaced for the maintainer (James's call)

- **D1 — PQ target.** Accept the honest classical-124 / quantum-~64 headline, OR
  raise to s≈120 queries (~2× FRI proof size) for quantum-128 FS-grind soundness
  (and widen the digest / capacity c=9 for quantum-128 binding). Proof-size vs PQ
  margin.
- **D2 — pad-seed uniqueness.** Promote `(pad_seed, slot_id)` single-use to a
  global, persistent, reorg-safe invariant (close lifecycle open item #7 + the
  rank-6 reorg profile) before any reorg-capable deployment.
- **D3 — headline relabel + hardening.** Relabel 2⁻¹²⁴ (sole sponge cap); land H1
  (mask domain tag), H2 (gate the q12289 Poseidon2), H3 (link EC files).

### F2 — admission-seam design: the pad-seed NULLIFIER (2026-06-27)

Implementing the F2 admission path surfaced a concrete design requirement: the global registry
(`descender-account-stark::pad_seed_registry`) is keyed on a 32-byte digest, but NO existing public
field is a faithful key. `pad_source_commitment` (C1 = `Poseidon2(pad_seed ‖ c1_nonce)`) is
NONCE-BLINDED — the same `pad_seed` under two `c1_nonce`s yields two different commitments, so keying
on it would NOT detect reuse. The masks/`masked_projection` are public but entangled with
`account_projection`. So F2's single-use guard needs a dedicated **pad-seed nullifier**: a public
value DETERMINISTIC in `(pad_seed, slot_id)` and independent of all blinding nonces.

Concrete spec (the next build, in-scope engineering — no external review, no maintainer decision):
- Add a public output `pad_nullifier = Poseidon2_sponge(DOMAIN ‖ pad_seed ‖ slot_id)` to the
  account-creation relation, with `DOMAIN` a fixed domain-separation tag distinct from the C1/C2/C6
  domains. Reuse the existing in-AIR Poseidon2 sponge gadget (same vetted width-16 permutation).
- Constrain it in-AIR from the SAME witnessed `pad_seed` lanes the C1/C2 gadgets bind (so the
  nullifier cannot be forged independent of the pad-seed actually used), and expose it in
  `AccountCreationPublic`.
- Admission path: `verify_account_creation(public, proof)` then
  `PadSeedRegistry::admit(public.pad_nullifier, leaf_id, height)` — the registry already accepts an
  arbitrary 32-byte key, so only the nullifier OUTPUT is missing.

Ripple (why it is a bounded build, not a one-liner): adding a public output changes the public
statement, the FRI transcript, the payload byte model, and every digest-pinned KAT (including the new
frozen q128 leaf KAT) — all must regenerate. So it is the same class as the q128 re-param: an AIR +
public-statement change with KAT/byte-model regen, done atomically. The registry and reorg-safe
semantics are already built and tested; this nullifier output + admission compose is the remaining
F2 engineering.
