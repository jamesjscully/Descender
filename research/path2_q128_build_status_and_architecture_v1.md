# Path 2 quantum-128 Build — Status & Architecture (v1)

> MIGRATION STATUS (2026-06-28): `ACTIVE_SUPPORT`.
> This is a status/architecture map of the q128 implementation effort, not a normative spec.
> Current normative authority starts at `../spec/README.md`. Settlement and finality decisions
> live in `../decisions/ADR-0009-native-sovereign-pq-rollup-settlement.md` and
> `../decisions/ADR-0010-registry-lifecycle-consensus-wiring-and-finality.md`; production readiness
> still remains external-review gated.
> This detail note is subordinate to `../TRACKER.md`, which remains the top-level
> implementation, proof, process, and adoption tracker.

## 0. Purpose & scope

This note consolidates, for a maintainer/reviewer, the state of the quantum-128 (q128) re-parameterized
build originally staged on the isolated `q128-reparam` worktree and merged into the active repo: what
is implemented and tested, where each piece lives, the pinned KATs, the economic story, and the exact
remaining gates. It claims only what the code and tests support; everything else is listed as a gate.
Nothing here flips the external-review-gated production gate (ADR-0008).

The original objective driving this work: *an economically viable quantum-128 level deployable standard
(everything except external analysis).*

## 1. q128 parameterization (the security floor)

`tools/descender-account-stark/src/config.rs` is the single source of truth:

| Parameter | Value | Note |
|---|---|---|
| `DIGEST_ELEMS` | 13 | digest width (raised 8→13 for collision/BHT margin) |
| `PROD_NUM_QUERIES` | 120 | FRI queries (raised 56→120; FS-grind/Grover floor) |
| `PROD_LOG_BLOWUP` | 4 | rate ρ = 1/16 |
| `PROD_QUERY_POW_BITS` | 16 | query proof-of-work |
| `Challenge` | `BinomialExtensionField<Val, 8>` | degree-8 ext (|F| = p^8 ≈ 2^248) |
| field / perm | KoalaBear / Poseidon2-16 | Plonky3 0.6.1 |

The conservative q128 floors (FS-grind→Grover, sponge indifferentiability c/2, digest-13 BHT
collision) are recorded in `research/path2_quantum128_parameterization_v1.md`. The aggregate STARK and
every leaf/recursion AIR read these `PROD_*` constants, so a re-parameterization cannot silently
desync any layer.

## 2. Layers (what's built, where, and how it's tested)

### 2.1 Account-creation leaf — `descender-account-stark`

`src/air.rs` — the full creation relation `C1 ∧ C2 ∧ C4 ∧ C6` + cross-component seams, as one
field-generic Plonky3 AIR, plus the public-output classes:
- **CLASS A** `role_root` — Poseidon2 fold of the role-handle commitments.
- **CLASS A-2** `nullifier` = `Poseidon2(DOMAIN ‖ pad_seed ‖ slot)` — the F2 nonce-free single-use key.
- **CLASS A-3** `checked_core_key_commitment` — the account's REGISTERED checked-core signing-key
  commitment (32-byte syndrome digest), transcript-bound + batch-digest-chain folded.
- **CLASS B** `relation_id` — pinned relation/profile id.
- **CLASS C** `syndrome[12] ‖ packed_witness_commitment[32]` — the reviewed-fq12 packed-witness
  state-18 frame the spend consumes (14-bit-LE packing, byte-identical to the deployed
  `pack_reviewed_fq12_packed_witness_state18_from_parts`).

Public API (`src/lib.rs`): `prove_account_creation{,_with_seed}`, `verify_account_creation`, batch
`prove_account_creation_batch` + `verify_account_creation_batch` (in-AIR Poseidon2 digest chain over
all rows' pubio). Tested via constraint-isolation + small-config prove/verify + integration tests
(`tests/{context_binding,redteam_attacks,roundtrip,hiding_mask_channel,soundness_context_binding}.rs`).

### 2.2 Recursion verifier-AIR (Phase-1) — `descender-recursion`

A hand-written verifier-as-AIR that re-verifies the creation leaf STARK in-circuit (Plonky3 0.6.1 has
no recursion primitive). `src/recursive_air.rs` is the monolithic verifier-AIR (index/challenge bus +
FRI/DEEP/quotient consumer + leaf C(zeta) mirror + public-IO), made PROVABLE (uniform via
`num_public_values`) and proven into a real O(1) recursive STARK proof. Gadgets:
`challenger`, `merkle`/`merkle_air`, `fri_fold`/`fri_fold_air`/`fri_chain_air`/`fri_layer_air`,
`deep`/`deep_air`, `leaf_constraint_air`, `public_io`/`public_io_air`, `index_bus`, `inlined_perm*`.

### 2.3 True single-proof O(1) block aggregator (Phase-3) — `block_recursive_air`

`src/block_recursive_air.rs` composes N member verifications + a chain-fold into ONE proof: each trace
row is a full `recursive_air` window (verifying that member in-circuit) folded into a running
`chain_state_root`. Proven at production q128 knobs, with a member-verification depth ladder
culminating in the FULL recursive verifier per member (all gadgets, FS-bound, openings at the
FS-derived index). So ONE O(1) proof attests N in-circuit-verified members chained to one root.

### 2.4 Native settlement verifier (the economically-viable deployable artifact) — `settlement`

`src/settlement.rs`: `prod_config()` (FRI knobs from the leaf `PROD_*` SoT), `prove_block` /
`verify_block` (the settlement-node entry point, O(1) verifier work per block at q128 knobs),
`serialize/deserialize_block_proof`. **Architecture position taken** (documented in
`research/path2_economic_deployment_architecture_v1.md`): native settlement (sovereign/L2 rollup whose
full nodes run this verifier) is the only deployment option both buildable in-repo *and* post-quantum
end-to-end; the precompile option needs an upstream EVM change, the SNARK-wrap option inserts a non-PQ
pairing boundary. `verifier_cost.rs` measures the per-block verifier Poseidon2-permutation count (the
budgeting basis: per-member work amortizes sharply as N grows).

### 2.5 F2 — single-use nullifier + reorg-safe registry

`src/pad_seed_registry.rs` (`PadSeedRegistry`): injective `admitted_for` (cross-account/persistent
guard) + `reorg_to` freeing of orphaned admissions, strictly-monotone heights. Keyed on the in-circuit
nonce-free `nullifier` (CLASS A-2), which — unlike the nonce-blinded `pad_source_commitment` — yields
the SAME value for a reused (pad_seed, slot), so reuse is detectable.

### 2.6 Phase-2 spend subsystem — `spend_air` / `checked_core` / `poseidon2_sponge`

The spend leaf AIR (`src/spend_air.rs`, `SpendAir`) proves, in-circuit at production config, the full
spend acceptance:
1. **Packed-witness consumption** — the public `packed_witness_commitment` is a well-formed 14-bit-LE
   packing of 18 canonical-`<q12289` residues (`syndrome[12] ‖ projection[6]`), byte-identical to the
   creation CLASS-C frame (fidelity-pinned).
2. **Checked-core frame digest** — `Poseidon2(DOMAIN ‖ message ‖ account ‖ slot ‖ state ‖ packed)`
   binds the statement to the witness (shared sponge primitives in `src/poseidon2_sponge.rs`).
3. **Checked-core SIS** — `syndrome == H·response (mod q)` for the fixed 12×22 matrix `H`
   (`src/checked_core.rs`, SHA256-derived byte-identical to the deployed `domain_matrix_entry`),
   `response` private and canonical.
4. **Sigma acceptance identity** — `syndrome == t + c⊛w (mod q)`, with `⊛` = Fq12 multiply mod
   `x^12+x+25` (`checked_core::fq12_mul`, fidelity-pinned vs the deployed `poly_mul_mod`).
5. **`w`/`t` bound to commitments** — `w == unpack(witness_commitment)`, `t == unpack(proof_commitment)`.
6. **Challenge derived in-circuit** — `c = Poseidon2(DOMAIN ‖ statement ‖ wc ‖ pc) mod q` (the
   SHA→Poseidon2 rehost; no SHA, no rejection sampler), constrained `== sigma_c`.

So the full sigma equation `H·r == t + c⊛w` is self-contained in-circuit with `c` bound (proper
Fiat-Shamir: `proof_commitment` in the challenge preimage). The witness flow is the real prover order
(fix key + nonce → derive `c` → solve `H·response = syndrome` via `checked_core::solve_response`; the
response is canonical not short-norm, so this is a solvable linear system needing no trapdoor).

Public API (`src/lib.rs`): `prove_spend_authorization{,_with_seed}`, `verify_spend_authorization`.

### 2.7 `{creation, spend}` co-verification

`src/lib.rs`: `verify_creation_and_spend(creation, creation_proof, spend, spend_proof)` verifies BOTH
proofs and enforces:
- **MED-2** — `creation_spend_packed_witness_binds`: spend consumes EXACTLY the creation's
  `packed_witness_commitment`.
- **public-key↔account** — `creation_spend_key_binds`: `spend.witness_commitment ==
  creation.checked_core_key_commitment` (the spend's key is the account's creation-registered key).

This is the deployable (settlement-layer) form of co-verification under the native-settlement
architecture.

## 3. Frozen KATs (drift protection)

Both pin a deterministic production-config proof's SHA-256 digest (single-thread PoW grind),
`#[ignore]` (slow; run explicitly):
- creation leaf — `frozen_q128_leaf_proof_kat`, `b79c7375484b78a5d00579e1d02b93eda1ea04cffc70923a0164887bbe82b4a0`
- spend leaf — `frozen_q128_spend_proof_kat`, `297df18647c3c9f3f2a286a4e2d33beb38bac3448031ccfdfc8635d3f0ce4e24`

## 4. Economics

Per block: ONE O(1) aggregate proof attests all members; per-member verifier work amortizes toward a
small constant as N grows (`verifier_cost`). Naive in-EVM verification exceeds the L1 gas limit
(measured), so on-chain L1 viability needs a precompile, a SNARK-wrap, or — the chosen, in-repo,
post-quantum path — native settlement where full nodes run `settlement::verify_block` off-EVM at
native speed. Under native settlement the measured per-block perm counts are throughput, not gas.

## 5. Remaining gates (precise, honest)

1. **Spend in the O(1) aggregator.** PARTIALLY DONE (2026-06-28): `descender_recursion::spend_block`
   aggregates N real spend witnesses (via `spend_air::spend_derive`) into one `chain_state_root`
   through the generic recursion — each member's `packed_witness_commitment` verified in-circuit at the
   SAME depth the creation aggregator runs at (`proves_spend_block_q128`, production q128). The generic
   machinery is not creation-coupled, so spends reuse it as-is. The full constraint-level spend
   verification — `descender_recursion::spend_constraint_air` re-deriving the ENTIRE spend C(zeta)
   (packing + frame/challenge Poseidon2 sponges + checked-core SIS + sigma identity + sigma commitments
   + relation-id) over opened-at-zeta cells — is now DONE (2026-06-28): every residual vanishes on a
   real lifted spend trace, and the full C(zeta) folds + proves+verifies at production q128 via the
   shared `alpha_fold` committed-accumulator primitive (`full_spend_czeta_folds_provably_q128`, ~57s).
   The SAME re-arithmetization closes BOTH the creation (`leaf_constraint_air`, in the recursion window)
   and spend (`spend_constraint_air`) full-inner-C(zeta) halves. REMAINING (the deeper rung): WIRING the
   spend C(zeta) mirror into a spend `recursive_air` window (as `eval_full_leaf_c` does for creation)
   and lifting the `{creation, spend}` cross-binding (MED-2 + key) into the aggregator.
2. **L1 settlement bridge** (decision + build). Position taken (native settlement) for the verifier;
   posting rollup state to L1 re-introduces the precompile-vs-SNARK-wrap PQ trade-off — a maintainer
   decision (future ADR).
3. **Consensus wiring** (deployment decisions). Wire `pad_seed_registry` into the live admission path;
   the lifecycle consume-once / admission registry into consensus; pin the reorg/free policy to the
   chosen finality profile (D2).
4. **External review + production gate** — gated by ADR-0008. This is the "external analysis" the
   objective explicitly EXCLUDES; the code has moved into the active repository, but the production
   gate remains closed.

## 6. Where to start reading

- Security floors: `research/path2_quantum128_parameterization_v1.md`
- Recursion/aggregation architecture: `research/path2_recursion_air_composition_architecture.md`
- Economics + settlement decision: `research/path2_economic_deployment_architecture_v1.md`
- Spend relation: `research/spend_authorization_relation_v1_spec.md`
- F2 boundary: `research/cryptanalysis/assumption_boundary_stress_test_20260627.md`
- Code entry points: `descender-account-stark/src/lib.rs` (leaf + spend + co-verification),
  `descender-recursion/src/{block_recursive_air,settlement}.rs` (aggregator + deployable verifier).
- Tracker row: `TRACKER.md` → `AGG-ROLLUP-001`.
