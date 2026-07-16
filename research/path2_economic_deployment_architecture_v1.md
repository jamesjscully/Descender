# Path2EconomicDeploymentArchitectureV1

> Classification: `ACTIVE_SUPPORT`. Decision analysis for the economically-viable
> quantum-128 Ethereum deployment architecture, supporting `AGG-ROLLUP-001` and
> `ADOPT-ETH-ROLLUP-001`. Recommends a direction for a future ADR; not itself
> normative until that ADR is accepted.

Status: economic deployment-architecture options + recommendation.

## The forcing constraint (remeasured 2026-07-10)

The active 120-query account-creation STARK bundle is **12,144,273 bytes
(~12.14 MB)** in the frozen production KAT; the historical 56-query baseline was
~4.75 MB. On Ethereum, nonzero calldata encoding would cost up to ~183M gas —
**>6× a ~30M block gas limit** — and the bytes occupy roughly 88-90 raw 128-KiB
blob payloads. **Per-account on-chain verification of the creation proof is categorically
impossible**, at any security level. The compact spend (~262–318 B) is the only
artifact that can live on the on-chain hot path.

Therefore the economic question is not "tune parameters" — it is *where does the
12.14 MB creation proof get verified, trustlessly, with the whole pipeline
post-quantum?* That choice is the deployable architecture.

## Options

| Option | How creation proof is handled | Trustless? | PQ end-to-end? | Ethereum-viable? | Build cost |
|---|---|---|---|---|---|
| 1. PQ validity rollup | verified off-chain by a prover; a recursive **STARK** validity proof over the batch is posted to L1; L1 verifies one small proof | yes | yes (inner + outer both STARK/Poseidon2) | yes (one succinct proof per batch) | high (needs STARK recursion) |
| 2. Native STARK recursion | a recursion AIR verifies N inner account-STARK proofs → one proof | yes | yes | only if the aggregate is small enough for L1 (it is, if recursion compresses) | high (the "multi-month" recursion AIR) |
| 3. Committee prevalidator | an off-chain committee verifies creation proofs and attests to the slot leaf; L1 trusts the attestation | **no** (trust the committee) | yes (committee can be PQ-signed) | yes (only the leaf + attestation on-chain) | low–medium |
| 4. Descender appchain / L1 | Descender's own validators verify the 12.14 MB proofs off Ethereum; Ethereum used only for bridging | yes (own consensus) | yes | n/a (not "on Ethereum") | medium (own chain) + bridge |

Key observations:
- **Option 1 contains Option 2.** A PQ validity rollup *needs* PQ recursion to
  produce its L1 validity proof — most rollup recursion today uses non-PQ SNARKs
  (pairing-based Groth16/PLONK), which would make the on-chain proof the
  quantum-weak link and defeat the entire premise. So a genuinely PQ rollup must
  use STARK-over-STARK recursion (Plonky3/ethSTARK-style). The native recursion AIR
  (Option 2) is the load-bearing component of Option 1.
- **Option 3 is the only low-cost path but is not trustless** — it reintroduces a
  committee trust assumption at admission. Acceptable for custody/enterprise or as
  an interim bootstrap; not acceptable as the decentralized end-state.
- **Spend path is separate and lighter.** Spends (~262–318 B) are size-viable
  on-chain regardless; their EVM verification gas (Poseidon2 FS + separated Fq12
  action) still needs measurement and may itself motivate spend aggregation.

## Quantum-128 placement under this architecture

Because creation proofs are verified off the on-chain hot path, the q128
parameterization (`ADR-0008`: ~120 FRI queries, Poseidon2 capacity c=9) is a
**prover/recursion-layer** cost, not an on-chain per-tx cost. The end-to-end PQ
requirement is: the **inner** account-STARK is q128 AND the **outer** recursive
aggregate is q128. A non-PQ outer proof voids the PQ claim no matter how strong
the inner proof is. This is the single most important design constraint for the
rollup option.

## Recommendation

**Deploy Descender as a PQ validity rollup whose creation-proof aggregation uses
Descender-native STARK recursion (Option 1 built on Option 2):**

1. the inner account-STARK is re-parameterized to q128 (`ADR-0008`);
2. a Poseidon2/Plonky3 STARK recursion AIR verifies batches of inner proofs and
   the consume-once state transitions, itself q128;
3. L1 verifies one recursive STARK validity proof per batch; only slot-leaf
   records and compact spends touch L1;
4. a **committee prevalidator (Option 3)** is acceptable as a *non-trustless
   interim* for early/enterprise deployment, explicitly labelled as such, while
   the recursion layer is built.

Rationale: this is the only path that is simultaneously trustless,
post-quantum end-to-end, and Ethereum-viable. Its cost and timeline are dominated
by the STARK recursion AIR — which is exactly the unbuilt `AGG-ROLLUP-001`
component and the binding gate on the whole economic goal.

## Honest residual / what this does NOT resolve

- The STARK recursion AIR is large and hand-written (Plonky3 0.6.1 ships no
  recursion primitive), but it is NOT unstarted: per `AGG-ROLLUP-001`, Phase-1
  single-proof recursion is substantially built (~70 passing gadget tests across
  the in-AIR challenger, inlined Poseidon2 perm, Merkle, FRI fold/layer/chain,
  DEEP/constraint-eval, index-bus, public-IO), with the monolithic `recursive_air`
  integration the open Phase-1 milestone and Phase-2 (creation+spend) / Phase-3
  (block aggregation → O(1)/block, the economic payoff) remaining. Until Phase-3
  lands there is no trustless economically-viable Ethereum deployment, but the gate
  is under active construction, not greenfield.
- Outer-recursion q128 must be budgeted separately and inherits the same
  FS-grind/sponge constraints as the inner proof (`ADR-0008`); recursion can
  compound the field/sponge floors at the outer layer.
- Spend-path EVM verification gas is unmeasured.
- This is an internal decision analysis; final selection belongs in a future ADR once the maintainer picks the fork.

## Measured aggregate-proof amortization (2026-06-27, q128 worktree)

The block accumulator (`BlockAccumulatorAir`) folds N per-account commitments into one O(1)
`chain_state_root` proof, proved at PRODUCTION quantum-128 FRI knobs (120 queries / log_blowup 4 /
query-PoW 16). Serializing the real proof (`block_accumulator::tests::aggregate_proof_size_amortizes_per_account`)
gives:

| block size N | aggregate proof | per-account footprint |
|---|---|---|
| 4  | 531,009 B | 132,752 B |
| 16 | 648,977 B | 40,561 B |
| 64 | 791,905 B | 12,373 B |

The aggregate proof grows only ~logarithmically with N (×16 the block ⇒ ×1.49 the proof), so it is
effectively O(1) per block: ONE proof verified once on-chain attests the whole block, and the
per-account share falls (≈133 KB → ≈12 KB and shrinking), already ~91×–1012× below the ~12.14 MB
per-account creation proof that made per-account on-chain verification categorically impossible.
This is the measured economic-viability basis: aggregate, verify once per block, amortize. (The raw
per-account creation/recursive proof can live in DA / off the hot path; only the single aggregate is
verified on chain.) The on-chain EVM verifier gas model for that single aggregate verification is the
remaining economic-measurement item.

## Measured aggregate-proof VERIFICATION cost (2026-06-27, q128 worktree)

Beyond proof size, the on-chain gas is dominated by the verifier's Poseidon2 permutation count.
Measured (not modeled) by counting permutation invocations during `verify` of the production-knob
aggregate block proof (`verifier_cost::tests::measure_aggregate_verify_perm_count`):

| block N | verify Poseidon2 perms | per-member perms |
|---|---|---|
| 4  | 24,327 | 6,081 |
| 16 | 33,211 | 2,075 |
| 64 | 44,015 | 687 |

Two findings:
1. **Per-member verification work amortizes sharply** (6,081 → 687 perms/member as N grows 4→64),
   confirming the aggregation economics: one block verification covers the whole block.
2. **Absolute per-block gas exceeds the L1 block limit under a naive verifier.** At an illustrative
   EVM cost of 3k–30k gas per width-16 Poseidon2 permutation (precompile-ish .. in-Solidity), the
   per-block verification is ~72M–1.3B gas — far above Ethereum's ~30M block gas limit. So a NAIVE
   in-EVM native-STARK verifier is NOT economically viable on L1 regardless of aggregation.

**Deployment implication (sharpens the recommendation):** on-chain L1 viability requires one of
(a) a KoalaBear-Poseidon2 EVM **precompile** (bringing gas/perm to the low hundreds, ⇒ ~5–15M gas/block —
viable), (b) a succinct **SNARK wrap** of the aggregate STARK verified by existing EVM precompiles
(~250k gas constant, fully viable, but adds a non-PQ wrap at the settlement boundary), or (c) settling
on a **PQ validity rollup** whose own verifier runs off-EVM (the architecture already recommended
here). The aggregation makes per-member cost negligible; the remaining economic lever is the
per-block verifier cost, which these measured perm counts now quantify for budgeting.

## Decision + built artifact: native settlement verifier (2026-06-27, q128 worktree)

The deployment lever above (per-block verifier cost) is resolved by **choosing native settlement**:
Descender settles as a sovereign / L2 validity rollup whose full nodes run the aggregate-proof
verifier off-EVM. Of the three viable options, this is the only one that is both buildable inside this
repository (no EVM-client fork, no EIP) and post-quantum end-to-end (the precompile option needs an
upstream EVM change; the SNARK-wrap option inserts a non-PQ pairing assumption at the settlement
boundary, defeating the quantum-128 goal). Native settlement runs verification at native speed with no
EVM gas, so the measured per-block perm counts above are throughput numbers, not gas costs — the path
is economically viable.

That verifier is now **built and shipping** as `descender_recursion::settlement`:
- `prod_config()` — the deployed aggregate-STARK config, FRI knobs read from the leaf STARK's
  `PROD_*` constants (single source of truth; the aggregate verifier cannot silently desync).
- `prove_block(config, cfg, member_windows) -> (BlockProof, chain_state_root)` — sequencer side.
- `verify_block(config, cfg, proof, claimed_root) -> Result<(), _>` — the deployable entry point a
  settlement-layer full node / sequencer / DA verifier calls to accept or reject a block. `Ok(())`
  means every member was verified in-circuit by the full recursive verifier AND chains to
  `claimed_root`, in O(1) verifier work per block at production quantum-128 knobs.
- `serialize_block_proof` / `deserialize_block_proof` — transport to the settlement layer.

End-to-end test `settlement::tests::settlement_roundtrip_accepts_and_rejects_q128` proves a block,
serializes it for transport, deserializes it on the verifier side, ACCEPTS the honest block, and
REJECTS a tampered `chain_state_root` — all at production q128 FRI knobs. This is the economic
deployment artifact for the chosen native-settlement architecture, exercised end to end.

What remains for L1 finality (not for the rollup itself): a settlement bridge posting the rollup's
state to L1, which is where the precompile-vs-SNARK-wrap choice (and its PQ trade-off) would re-enter
— a maintainer/architecture decision, out of scope for the native verifier built here.
