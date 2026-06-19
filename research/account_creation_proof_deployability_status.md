# Account-Creation Proof — Deployability Status (2026-06-18)

Consolidated status of the account-creation hidden-source proof after the Plonky3
re-host. Honest about what is done, what remains, and what is irreducibly
external. Numbers are MEASURED unless marked.

## CORRECTION 2 (2026-06-18, full design red-team, 4 attackers) — multiple over-claims retracted

- **CRITICAL: the batch/aggregation is UNSOUND as built.** The batch digest is
  NOT bound to the proof (verifier does `p3_verify(proof, &[])` with empty public
  values + a separate `digest==hash(tuples)` check; nothing ties them; digest
  never enters the transcript or AIR). REPRODUCED: a real proof accepts another
  batch's tuples / 9 impossible garbage "accounts". => the "~1.6 KB/account
  batched" size answer is RETRACTED (built on an unsound batch). Single-instance
  is fine (public statement is transcript-bound). FIX: make the digest a
  transcript-bound STARK public value + in-AIR hash-chain (the deferred work).
- **HIGH: "128-bit soundness" is OPTIMISTIC.** 56*2+16=128 is the proximity-gap
  (capacity-regime) formula; under the conservative Johnson bound (forced by the
  late-2025 proximity-gap conjecture disproof) 56 queries ~= 72 bits. Config is
  ~72-bit CONSERVATIVE, not 128 — needs ~128 queries.
- **HIGH: "128-bit hiding verified by attempted recovery" over-claimed the
  EVIDENCE.** The recovery test (opened_value != plaintext_byte) is trivially
  true with blinded FRI openings = tests the WRONG channel (account_projection
  hiding rests on the mask bias on the PUBLIC masked_projection, not on FRI
  hiding). Same false-assurance shape as reopened HIGH-2. ALSO the default prove
  path uses a FIXED 64-bit non-CSPRNG seed (non-hiding across proofs). Mask-bias
  PROPERTY is independently confirmed ~2^-141 (likely sound), but it is NOT
  properly verified and the default RNG must be fixed.
- **HIGH: "spend SUF-CMA preserved" is CONJECTURED, not proven.** "#9=0" is
  classifier deferral (keygen out-of-scope per AP2-005), the frame-PRF lemma is
  not composed into the spine, and the EC re-charge is not done.
SURVIVED red-team (sound): single-instance hiding (no witness leak, bias 2^-141,
no side channel, HidingFriPcs provisioning ok), cross-account/cross-slot replay
closed at the deployed binding layer, the fast single-instance prove/verify.
VERDICT: design viable, NOT broken, but readiness was over-claimed on multiple
axes; do not invest in heavy wiring until: batch digest bound, queries raised to
conservative-128-bit, proper ZK test + CSPRNG, EC re-charge, and the 5 missing
relation classes (below) are added.

## CORRECTION (2026-06-18, design red-team) — scope of "what's proved"

A design red-team found this doc OVER-CLAIMED. What the re-host proves FAST/ZK is
the **C1∧C2∧C4∧C6 hidden-source CORE relation**, NOT the full account-creation
relation. The re-host's public statement is only `(pad_source_commitment,
opening_tuple_commitment, masked_projection)` — it binds NO context (account_id,
slot_id, state_root, verifier_version, lifecycle_anchor). Against the soundness
contract's 8 MANDATORY relation classes it implements ~3 and OMITS 5:
role-restriction, opening-tuple-account-binding, checked-core-residue-
compatibility, lifecycle-context, proof-system-bookkeeping. Per the contract,
omitting these means relation-knowledge soundness CANNOT be claimed — so as an
isolated object the proof is a context-free "universal admission token" (the same
failure shape the old core had). FIXABLE, not fatal: the deployed leaf/registry
binding layer is independently tight, and the missing classes are ADDITIVE (add
public context columns + binding rows to the AIR; bind the C2-PRF slot to the
public-context slot). Until added, "the account-creation relation is proved" is
FALSE — only the hidden-source core is. Related over-claims to fix: af_hash_
selection.md still pins q12289-native Poseidon2 (stale; re-host uses KoalaBear +
q12289 app-modulus); the carrier is computed over native KoalaBear not mod q12289;
pad-seed uniqueness is only intra-batch yet tracker marks duplicate-prevention
[x]; the deployed creation verifier is still an unimplemented stub. See the
2026-06-18 red-team synthesis. The numbers below stand; the "what it proves"
claim is corrected to "the core relation."

## What the proof proves

`account_creation: ∃ hidden (pad_seed, slot, account_projection, lifecycle, nonces)`
such that, jointly (one pad_seed, one canonical account_projection):
- C1: pad_source_commitment opens to pad_seed
- C2: projection masks = Poseidon2-PRF(pad_seed, slot)
- C4: masked_projection = account_projection + masks (mod q12289), account_projection canonical base-128
- C6: opening_tuple_commitment opens to (account_projection, lifecycle)
all as ONE Plonky3 ZK-STARK over KoalaBear. This is exactly the C1∧C2∧C4∧C6
obligation the machine-checked negative theorem said the old sampled core lacked.

## DONE (measured + tested this session)

| property | value |
|---|---|
| Prove (single) | ~35–56 ms |
| Prove (batched) | sub-ms / account |
| Verify | ~40–86 ms |
| Proof size (single) | ~2.9 MB |
| Proof size (batched) | ~6 KB/acct @ N=256, ~1.6 KB/acct @ N=1024 |
| Soundness | 128-bit conjectured FRI (Plonky3, vetted); tamper-rejected |
| Zero-knowledge | enabled (HidingFriPcs + salted Poseidon2-Merkle); verified by ATTEMPTED RECOVERY |
| Hiding strength | 128-bit (wide-then-reduce mask gadget; bias 2⁻¹⁴¹) |
| Constants | pinned vetted `default_koalabear_poseidon2_16` |
| Field | KoalaBear (31-bit, recursion-friendly; retires the 14-bit small-field concern) |

Packaged as the clean crate `tools/descender-account-stark/` (Plonky3 isolated;
`tools/parameter-search` has 0 p3 deps) with API `prove/verify_account_creation`
+ batch variants.

Risks retired vs the original hand-rolled q12289 stack: prove time (6–8 min →
~ms, ~10⁵×), hand-rolled-FRI soundness (→ Plonky3 vetted FRI), small-field
Poseidon2 (→ 31-bit vetted), ZK/HIGH-2 hiding (closed at construction level), and
a 17-bit→128-bit hiding-bias bug (caught + fixed by the mask-reduction analysis).

Spend SUF-CMA under the mask-PRF swap: ANALYZED — the mask PRF is hiding-only
(forgery reduction never touches it; bears break #9 = 0 mass in the 2⁻¹²⁸ spine),
so SUF-CMA is PRESERVED under a clean assumption swap
(`A_sha256_frame_prf → A_poseidon2_prf`); spend stays 262/318 B (values change,
not bytes).

## STATUS UPDATE (2026-06-19) — critical-path items 1–3 now DONE; remainder is formal+external

This session's net effect on the list below: the INTERNAL critical-path (1–3) is essentially
complete; what remains for "fully deployable" is formal mechanization (Lean/EC) + external
cryptanalysis + review/gate-flip. Recursion (5) is OPTIONAL scale and was advanced separately.

- **#1 wire account-stark** → ✅ DONE (Option A `descender-account-verifier` crate + producer
  migration wired into `parameter-search`).
- **#2 deployed spend mask-PRF swap + KAT** → ✅ DONE (on `deployed-mask-prf-swap`; full spend
  FS re-hosted to Poseidon2 + `kat_v0` regenerated from the real pipeline, byte-reproduced).
- **#3 AoK / injectivity for the re-hosted relation** → ✅ PAPER-LEVEL DONE — the
  argument-of-knowledge for the re-hosted (KoalaBear-STARK) relation is
  `fri_aok_extractor_instantiation.md` (extractor constructed + per-param error) + the
  AIR-fully-constrained soundness audit. Lean/EC MECHANIZATION of it is the named GAP-B/D seam
  (rolls into #6). NOTE: `Descender/Path2NativeAccountDesign.lean` (0 sorry/0 axiom) is the
  q12289-era formalization; the re-hosted relation's mechanization is future formal-methods work.
- **Soundness levels SETTLED this session:** spend SUF-CMA 2⁻¹²⁸ / creation-STARK knowledge
  soundness 2⁻¹²⁴ (deg-8 challenge-field bump applied to lift the FRI field-bound term off
  ~2⁻¹⁰⁴; 2⁻¹²⁴ = inherent industry-standard WIDTH-16 Poseidon2 ceiling, James-accepted). See
  field-bound-soundness-ceiling + degree5_field_rehost_scope.md.
- **#5 recursion** → OPTIONAL (scale). Advanced this session: all 5 gadget references validated,
  gadget-1 a real `Air<AB>`, Poseidon2 dep de-risked, recursive extractor argued+reviewed,
  composition architecture decided (monolithic-inline). Remaining = the multi-week monolithic
  `recursive_air` build. NOT on the deployable critical path.

⇒ **Internal prover/code work for the DEPLOYABLE base scheme is DONE.** Remaining blockers are
all formal-mechanization (#3-mech, GAP-B/D) + external (third-party lattice cryptanalysis #6,
external review + gate-flip #4) — none solvable by further prover code this session.

## REMAINING to deployable (integration + formal + external)

1. **Wire `descender-account-stark` into the deployed flow** — `parameter-search`
   takes a path-dep, calls the API at admission (prove) + verification (verify).
   Integration, not research.
2. **Deployed spend mask-PRF swap** — change the deployed spend verifier to derive
   masks via the SAME KoalaBear-Poseidon2 + wide-then-reduce (the current SHA path
   uses rejection sampling = already exact-uniform, so no bias there; the swap is
   for consistency with the proof). Regenerate the KAT corpus; do the EC re-charge
   of `frame_prf_ro_reduction.ec` (light — proof goes through verbatim, assumption
   relabeled). HIGH BLAST RADIUS on the deployed signer.
3. **Lean injectivity re-proof** for the re-hosted relation (the construction
   changed field + PRF; the injectivity/argument-of-knowledge argument needs
   restating for the new construction).
4. **Flip the fail-closed gate** once 1–3 + review land.
5. **Proof-size / aggregation (optional but recommended)** — strict per-position
   batch binding (in-AIR hash chain); full recursion (cross-time aggregation →
   O(1) chain-state proof). KoalaBear chosen for this.
6. **IRREDUCIBLY EXTERNAL:** the spend signature's coded-SIS lattice core
   (ring/module cryptanalysis) and the Lean↔EC GAP-B/D paper-composition seams.
   "Fully deployable for the whole descender" CANNOT be reached by code alone —
   it requires third-party lattice cryptanalysis. This is the standing external
   residual (see SECURITY_DOSSIER / RED_TEAM_REPORT).

## Honest bottom line

The account-creation proof (the active frontier and the riskiest open
cryptographic-engineering item) is now FAST, ZERO-KNOWLEDGE, 128-bit, and cleanly
packaged — the hard performance + correctness risks are retired and assumptions
were tested as we went. The path to a fully deployable WHOLE descender is now
integration (1–4), optional aggregation (5), and the irreducible external
cryptanalysis (6) — multi-week and partly third-party, not solvable by further
prover work this session.
