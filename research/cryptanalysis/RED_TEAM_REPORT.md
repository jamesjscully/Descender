# Descender Red-Team Report

**Scheme:** Descender post-quantum lattice signature. q=12289, n=32, Fq12 = F_q[X]/(X^12+X+25).
**Engagement:** Authorized internal red-team of the maintainer's own scheme prior to external submission. Objective: BREAK it.
**Date:** 2026-06-10.
**Verdict in one line:** THE SCHEME DOES NOT PASS AS-IS. Two HIGH-severity breaks on the deployable path (signature malleability / SUF-CMA violation, and key-free full-witness recovery) plus a HIGH-severity proof-budget break (multi-target second-preimage on the 214-bit checked core) must be fixed before external submission. The core lattice hardness, the box frontier, and the consume-once lifecycle survived hard attacks.

---

## 1. DID ANYTHING CRACK? — YES. (3 HIGH findings; 0 critical-with-shipped-default)

No finding is rated *critical* only because each leaky/forgeable path is gated AVAILABLE/NON-DEFAULT behind a fail-closed production preflight, OR (Vector 5) is a closure-budget breach rather than a runnable forgery today. But all three directly REFUTE headline claims in `SECURITY_DOSSIER.md` and each is a true defect that must be remediated. Listed by severity.

### HIGH-1 — Signature malleability / SUF-CMA violation on the deployed local-delta verifier (Vector 2) — **BROKEN**

The per-rep relation check (`verify_stateful_signature_with_hidden_preimage_relation`, `signature_q12289_local_delta_stateful.rs:3724`; and `verify_local_delta_relation_with_masked_witness`, `:4053`, used by the deployable proof-carrying path) accepts ANY `mask_view` with `local_state(mask_view)==fiber_state` and `opening==mask_view+pad`, where `local_state(m)=m[0]+50*m[1] mod 12289`. It does NOT pin `mask_view` to the signer's canonical choice. The local relation `a+50*b ≡ fiber` has coded-kernel vector `(50,-1)`, L-inf 50, inside the response box `[-48,48]`. For the reference fiber there are exactly TWO in-box solutions per rep — confirmed:

```
python3 -c "Q=12289;S=50;B=48;f=(12+S*(Q-12))%Q;print([(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a+S*b)%Q==f])"
-> [(-38, -11), (12, -12)]
```

The Fiat-Shamir challenge binds `(pk,msg,slot,digest[,proof_commitment])` but NOT the response/opening bytes. An attacker with NO secret swaps `(12,-12)->(-38,-11)`, recomputes `openings = mask_view+pad`, and the verifier accepts a byte-distinct signature for the same message. The 26 reps are checked independently → up to 2^26 (~6.7e7) distinct accepting signatures per message. Directly refutes the dossier SUF-CMA claim. The 114>96 coded-SIS certificate does NOT cover this per-rep opening relation — relation mismatch between the proven object and the deployed check.

**Reproduction (independently re-run, PASS):**
```
cd /home/jack/code/Descender/tools/parameter-search
cargo test --bin signature_q12289_local_delta_stateful redteam_response_opening
-> 2 passed (redteam_response_opening_malleability_distinct_accepting_signature [quotient path],
            redteam_response_opening_malleability_deployable_proof_path [deployable proof path])
```
Report: `research/cryptanalysis/redteam_implementation_malleability.md`.

### HIGH-2 — Key-free FULL-WITNESS recovery from the historical masked-state18 proof object (Vector 4) — **BROKEN OBJECT; ACCEPTOR HARD-GATED**

Status update (2026-06-11): `verify_path2_direct_creation_masked_state18_proof_v1` and the
proof-carrying dispatcher now reject this proof kind with
`PATH2_MASKED_STATE18_PUBLIC_PROOF_DISABLED_ERROR`. The object remains leaky if serialized.

`Path2MaskedState18CreationProofV1` (kind `PATH2_DIRECT_CREATION_PROOF_KIND_MASKED_STATE18=3`) is `#[derive(Serialize)]` and carries `pad_seed` IN CLEAR — confirmed at `production_reference.rs:1484-1487`:
```rust
pub struct Path2MaskedState18CreationProofV1 {
    pub pad_seed: [u8; REVIEWED_FQ12_PAD_SEED_BYTES],
    pub account_projection: [u16; HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES],
}
```
`pad_seed` is the master witness secret: it deterministically derives the projection masks (`reviewed_fq12_packed_witness_state18_projection_masks`), the per-block secret pad residue table (`reviewed_fq12_pad_residues_from_seed_bounded`), and the hidden-source projection VALUES (`hidden_source_fallback_pad_prf_projection_values_from_seed`). Any observer of the serialized object recovers the entire hidden source with ZERO key material. The "PRF-fresh mask" hides nothing because its seed ships next to it. Witness-hiding = 0 bits. This refutes the historical masked-state18 public proof as a hiding design; the current transparent-descent ZK path is the active hiding candidate.

**Reproduction (independently re-run, PASS):**
```
cd /home/jack/code/Descender/tools/parameter-search
cargo test --release --lib redteam_masked_state18_creation_proof_leaks_full_witness_key_free
-> 1 passed
```
Test: accepts honest proof; re-parses JSON wire → reconstructs `pad_seed` byte-for-byte; from public `pad_seed` alone recovers full hidden-source projection + all 16 secret pad blocks + per-lane masks (all == ground truth). Severity HIGH not critical because `production_deployment_preflight()` stays fail-closed (path AVAILABLE/NON-DEFAULT, not shipped).

### HIGH-3 — Multi-target second-preimage on the 214-bit checked-core challenge (Vector 5) — **partial-weakness / closure-budget BREACH**

The GAP-C re-model (`break_checked_core_message_binding.ec`, `gap_c_remodel_spine_wiring.ec:47,52`) treats event #6 as SINGLE-target second-preimage with `eps = q/2^214 = 2^-150` at q=2^64, claiming "NO query cap." But the SUF-CMA experiment (`descender-signature-spec.md:77`, T=2^32) hands the adversary T honest pinned challenges c_1..c_T; a forgery wins by colliding the 214-bit challenge against ANY one → multi-target SPR with advantage **T·q/2^214**. The union-bound spine (`spike_union_bound.ec:109,131`) unions over the 9 event TYPES only and does NOT quantify over the T honest queries; the T factor vanishes at the per-event-reduction ↔ experiment seam.

**Quantified (independently re-run):**
```
T=2^32 q=2^64 n=214: adv=2^-118   <- breaches 2^-128 by ~10 bits
T=2^33 q=2^64 n=214: adv=2^-117
T=2^32 q=2^80 n=214: adv=2^-102
T=2^32 q=2^64 n=246: adv=2^-150   <- cap-free fix: widen to >=246 bits
safe envelope: log2 T + log2 q <= 86  (spec point 32+64=96 -> breach)
```
A signing-query cap T<=2^22 (at q=2^64) is therefore LOAD-BEARING — directly refuting the dossier "closes at q=2^64 with NO query cap" headline. The project's own `per_event_epsilon_budget_closure.md:147-154` already admits the cap is load-bearing. The 214-bit core is `22*8+6 + 32 = 182+32`. Leaf/registry (n=256) survive (T·q/2^256=2^-160); only the narrow 214-bit core fails.
Report: `research/cryptanalysis/redteam_boundary_composition.md`.

---

## 2. WHAT SURVIVED — strongest attack per surviving vector (evidence-based residual confidence)

### Vector 1 — Lattice assault on the coded-SIS residual — **no break (severity none)**
Four independent methods agree the shortest forgeable coded vector has L-inf=114>96:
1. **Native fpylll Enumeration** over the dim-9 coded sublattice (kernel of the 12×23 q=12289 matrix ∩ coord[0]=coord[3]=0 mod q) inside the full L2 cover ball radius^2=23·96^2=211968 (provably contains every L-inf<=96 vector): `Enumeration(G,nr_solutions=200000).enumerate(0,23,211968,0)` on BKZ-55 basis → 49728 coded vectors, **min L-inf=114, ZERO with L-inf<=96**.
2. **BKZ blocksize 20-60** full-dim SVP → shortest L2 vector L-inf=139.
3. **8.5M random small-combination L-inf minimizations** → best 138.
4. **lattice-estimator (Sage installed via nix)**: `SIS.estimate(n=12,q=12289,length_bound=460.4,m=23)` → rop=2^inf.

The bare (non-coded) lattice has a coeff-68 vector inside the box, but its coords[0],[3] are nonzero mod q so the response-link code rejects it — the intentionally-weak keygen is NOT relied on. Sampler statistically clean (chi2=34.3 df=31, full rank, gcd 1). In-repo cert `response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json` replays status=verified. **Tool limit:** generic integer/q-ary lattice model only; the actual Fq12 ring/module structure was NOT attacked.

### Vector 3 — State-machine / lifecycle (double-spend / replay / rollback) — **no break (severity low)**
End-to-end DB-adapter double-spend (Attack F): two `apply_opening_frame_and_commit` calls with the same valid frame → second REJECTED "slot is already in spent ledger." The persistent `ProductionSpentSlotLedger` (separate KV namespace, keyed (sha256(pk),slot_id) + global pad-seed-digest uniqueness) is the load-bearing consume-once guard; snapshot revert does not revert it, and the monotonic restore anchor blocks stale-snapshot rollback. `cargo test --test redteam_state_machine_probe` → 6 passed. Two demonstrated WEAKNESSES (not breaks): cross-pk replay is stopped ONLY by global pad-seed-digest uniqueness (no pk binding in the opening relation); bare transition fns trust inputs (ledger-free non-production API).

---

## 3. RESIDUAL ATTACK SURFACE — prioritized for the external team (seeds the cryptanalysis-package scope)

**P0 — fix-and-attack the three breaks' neighborhoods:**
1. After binding response/opening bytes into Fiat-Shamir to kill HIGH-1, re-audit `production_reference.rs` `verify_path2_*` (esp. the public creation verifier) for the SAME non-canonical local-delta relation, and check whether non-reference fiber_states admit up to ~3 in-box mask_views (slope 50 vs box 48). Probe whether any consensus/dedup/mempool layer keys replay on signature bytes (2^26 malleated variants = front-running vector pre-slot-consumption).
2. HIGH-2 fix is the transparent-descent ZK creation proof (currently unimplemented). Once it lands, attack ITS soundness/ZK: query_count/column_profile masked-column hiding, the SHA-256 frame-PRF assumption `A_sha256_projection_mask_frame_prf`. Also verify `ProductionReviewedFq12MaskedProjectionSoundnessCertificate` (full opening in clear) never escapes the verifier boundary — second full-witness-disclosure object.
3. HIGH-3: realize the multi-target SPR as a Hellman-Oechslin/rainbow precomputation over T=2^32 honest frames against `truncate_and_mask(SHA256)` (turns the 2^-118 closure breach into a ~2^107-class runnable forgery); run a Wagner k-tree over (honest challenge XOR adversary nibbles) exploiting the 0x3f-masked last byte; quantify quantum multi-target (BHT/Grover). Re-state the spine per-event term as T·eps_spr(n) and re-derive the (T,q) envelope → forces width >=246 bits OR enforced T<=2^22.

**P1 — the lattice ring structure (Vector 1's one untouched model):**
4. Attack the actual RING: Fq12=F_q[X]/(X^12+X+25). If the 23 columns encode ring elements, a module-lattice / ring-homomorphism attack could expose structure invisible to generic integer-lattice reduction. Run the lattice-estimator with a MODULE model on the coded problem specifically (not bare q-ary SIS), plus tuned g6k sieving. These are the documented external open items.
5. Structured pre-image/correlation attack on the SHA-256 sampler (beyond statistical tests).

**P2 — lifecycle hardening (Vector 3 weaknesses):**
6. Bind public_key into the opening-frame RELATION (not only the ledger key) to remove cross-key replay's sole dependence on pad-seed uniqueness.
7. TOCTOU/crash-atomicity across the two files (save_ledger + save_manager not atomic; fuzz interleaved processes around the lockfile / break_stale_lock clock skew).
8. Multi-chain/sharded-ledger wiring: a spent-ledger namespace finer than sha256(pk) could leave the same (pk,slot) unspent in a second shard.

---

## 4. NET — does the red-team pass change the security posture?

**YES — the posture changes from "claimed-secure" to "secure on the lattice core and lifecycle, but with three deployable-path defects that must be fixed before external submission."**

- The headline is NOT the lattice. The lattice/coded-SIS core (114>96 box frontier) and the consume-once lifecycle SURVIVED hard, multi-method attacks and the dossier's evidence base for them is STRENGTHENED. We did not declare them unbreakable — we declare them *survived these specific attacks at these resource levels* (exhaustive in-box enumeration + BKZ-60 + estimator rop=2^inf for the lattice; end-to-end DB double-spend rejected for the lifecycle).
- The headline IS the implementation/proof seam. Three claims in `SECURITY_DOSSIER.md` are REFUTED by passing reproductions: SUF-CMA (HIGH-1, malleability), witness-hiding of the deployable creation proof (HIGH-2, key-free recovery), and "no query cap at q=2^64" (HIGH-3, multi-target SPR breaching 2^-128 by ~10 bits).
- **Required before external submission:** (1) bind response/opening bytes into Fiat-Shamir; (2) implement the transparent-descent ZK creation proof and remove/gate the pad_seed-in-clear path; (3) either widen the checked core to >=246 bits or enforce a T<=2^22 signing-query cap and restate the spine per-event term as T·eps_spr(n).
- These breaks are practical/algebraic (not statistical distinguishers): HIGH-1 and HIGH-2 reproduce as passing Rust tests with r=1.0 exactness; HIGH-3 is an exact budget arithmetic. None is a "we didn't try" residual.

**Honest bottom line:** the scheme's hard cryptographic core held; its deployable wiring and one proof-budget term did not. Fix the three HIGH findings, then re-run Vectors 2/4/5 to confirm closure and hand Vector 1's ring/module model + HIGH-3's runnable rainbow/Wagner realization to the external team.
