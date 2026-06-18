# Path-2 Proof-Family Aggregation / Recursion Compatibility Analysis

Status: RESEARCH-GRADE ANALYSIS (synthesized + adversarially verified, 2026-06). Covers a currently-MISSING repo dimension: proof RECURSION / FOLDING / AGGREGATION for a future Descender blockchain. The existing repo work covers batch slot-admission only, not proof-of-proof composition. This document does NOT change the active proof-family recommendation; it grounds the aggregation axis of that decision. All per-family conclusions below survived two independent adversarial-skeptic passes; confidence labels and corrections reflect the adversarially-survived position, not the original optimistic finding.

## Scope and ground rules

- Object under decision: the CREATION-time proof for `NativeBoundaryTraceRelationV1` (176 relation rows, 194 private witness lanes, Fq12 over q12289). Produced once per account at admission, verified by validators.
- The SPEND signature (262/318 B) is a SEPARATE object, referenced in the checked-core frame only by a hash digest. It is not the subject of this analysis, but it is the lever for the highest-value recursion opportunity (see Blockchain Implications).
- Byte size is the WEAKEST constraint for a creation-only proof. Verifier compute and recursion/aggregation matter more for a blockchain. This reweighting is the reason aggregation is analyzed at all.
- DEPLOYED vs RESEARCH-GRADE is tracked per family. Repo byte/compute numbers are PLANNING FLOORS, not measured.

## Five aggregation/recursion criteria

1. BATCH VERIFICATION: can N proofs be verified together cheaper than N separate verifications?
2. AGGREGATION: can N proofs be combined into ONE proof, size sublinear in N?
3. RECURSION / IVC / PCD: can a proof verify other proofs (proof-of-proof / accumulation), enabling rollup-style succinct chain state? At what concrete cost?
4. INTERACTION WITH SPEND SIGNATURE: does enabling creation-proof recursion open the door to compressing creation + spend into succinct chain state?
5. PQ-SAFETY of the aggregation/recursion MECHANISM ITSELF (distinct from the base proof's PQ safety).

---

## Family A — Transparent STARK/FRI IOP-of-knowledge

Maturity: DEPLOYED. Confidence: high on the mechanism; medium on Descender instantiation.

1. BATCH VERIFICATION — YES, deployed. Batch-FRI / STARKPack-style amortization folds N AIR instances into a shared FRI query phase with shared Merkle leaves. CORRECTION (adversarially flagged): STARKPack itself is a Nethermind RESEARCH PROTOTYPE delivering CONSTANT-FACTOR amortization (~3x proof-size, ~2x verifier-time), NOT "one FRI verification independent of N." True O(1)-in-N comes from the recursive wrapper (criterion 3), not batch-FRI alone. The production-deployed batching is the recursive composition inside Boojum/Starknet/Miden.
2. AGGREGATION — YES. A recursive STARK wrapper AIR verifies N inner FRI proofs and emits ONE proof whose size is independent of N. Deployed in zkSync Boojum, Starknet, Polygon Miden.
3. RECURSION / IVC — DEPLOYED. Plonky2/Plonky3 do STARK-verifies-STARK natively. Concrete anchors: Plonky2 compresses to ~43 KB, prove ~11.6 s, Ethereum verify ~1M gas (keccak-256 only); small recursion circuits prove <0.2 s / verify <50 ms on 2025 CPUs. CAVEATS: (a) the cited Plonky2 ~43 KB config targets ~100-bit CONJECTURED security, below Descender's 128-bit bar — at 128 bits with conservative FRI parameters the proof and query count are larger; (b) the recursion-circuit cost is dominated by in-circuit hashing of the FRI/Merkle verifier, so it REWARDS an arithmetization-friendly hash (Poseidon/Rescue/Monolith) and PENALIZES SHA-256 — which conflicts with the SHA-256 checked-core/role bindings and is the dominant practical engineering risk.
4. SPEND-SIGNATURE INTERACTION — STRONGLY POSITIVE, the highest-leverage point. Because the checked-core frame references the creation proof only by a hash digest, a recursive STARK can verify {creation proof + the SHA-256 checked-core spend relation} in ONE circuit and emit a single O(1) succinct chain-state proof — rollup-style compression of creation+spend, the canonical L2 architecture.
5. PQ-SAFETY of mechanism — SAFE if you terminate in a STARK: recursion is hash/RO-only (FRI + Merkle), same symmetric assumption as the base. CAVEAT: a final Groth16/PLONK wrap for minimal on-chain bytes reintroduces a NON-PQ pairing assumption. To stay PQ, terminate recursion in a STARK and accept a larger (tens-of-KB) final proof; no audited PQ-STARK rollup exists at those byte sizes yet.

## Family B — MITH / VOLEitH hash-ZK

Maturity: base DEPLOYED (FAEST is a NIST additional-signatures round-2/3 candidate); recursion ABSENT/research-grade. Confidence: high.

1. BATCH VERIFICATION — WEAK / linear. Each proof is independent GGM-tree / all-but-one vector-commitment openings; no shared low-degree oracle to batch. N proofs cost ~N verifications; only minor hash-call batching savings, no asymptotic win.
2. AGGREGATION — effectively NO native mechanism. Proof size is LINEAR in circuit size (QuickSilver ~1 field element per multiplication gate; survey: VOLE-ZK 5-200 KB, MPCitH 10-1000 KB, both O(n), non-succinct). No folding/accumulation primitive; combining N proofs requires a SEPARATE SNARK over the N verifiers. Repo model: ~36.3 KB per single 176-row/194-lane proof at 64 rounds; N simply add.
3. RECURSION / IVC — HOSTILE. To recurse you must express the MITH verifier (thousands of symmetric-primitive/hash evaluations + GGM expansions) as a circuit and prove it with an OUTER succinct system — almost always a STARK/SNARK. So MITH does not recurse on its own; it becomes the INNER leaf of a Family-A recursion. The VC-then-MPCitH / Virgo++ line exists precisely because MPCitH does not compose; it is research-grade, not turnkey IVC.
4. SPEND-SIGNATURE INTERACTION — NEUTRAL-to-NEGATIVE. Cannot natively co-compress with spend; any joint succinct state needs an outer STARK wrapper, at which point the MITH layer is just an expensive inner witness.
5. PQ-SAFETY of mechanism — base is cleanly PQ (symmetric/CR-hash only, the family's main selling point), but there is NO PQ-safe NATIVE recursion; the borrowed wrapper governs PQ status (STARK = PQ but you paid double; pairing-SNARK = not PQ).

## Family C — Descent/SIS lattice folding (LaBRADOR / LatticeFold class)

Maturity: RESEARCH-GRADE, no production blockchain deployment. Confidence: medium.

1. BATCH VERIFICATION — strong in theory. Ajtai/M-SIS commitments are additively homomorphic; N instances fold via random linear combination into one accumulated instance — batch verification IS the folding step, sublinear in N.
2. AGGREGATION — native and elegant. LatticeFold (eprint 2024/257, Boneh-Chen) folds N R1CS/CCS instances into ONE accumulated instance + one final SNARK; LatticeFold+ (eprint 2025/247, CRYPTO 2025) is ~5-10x faster prover with a purely-algebraic range proof and 2-vector decomposition; Neo (2025/294) / Neo+SuperNeo (2026/242) add pay-per-bit costs over small fields. CAVEAT: integrated zero-knowledge and proof-size compression for these folding schemes are OPEN PROBLEMS in the 2025-2026 literature — directly material because the Descender relation MUST hide.
3. RECURSION / IVC / PCD — the headline feature: folding "naturally leads to an efficient recursive lattice-based SNARK and an efficient PCD scheme," performance ~Hypernova-class over a 64-bit field. CONCRETE VERIFIER COST is NOT pinned in deployable terms — papers report prover speedups and asymptotics, not gas/byte/round numbers; treat as unquantified for Descender.
4. SPEND-SIGNATURE INTERACTION — potentially the best long-term fit because Descender already lives over q12289/Fq12 module math, so creation proofs and a lattice-native spend relation could share one folding accumulator. SPECULATIVE for the current SHA-256-framed spend signature, which is not lattice-native.
5. PQ-SAFETY of mechanism — BEST: the recursion mechanism itself is PQ (M-SIS based), unlike Family A's optional non-PQ wrap and unlike Nova/ProtoStar (DLOG/curve, NOT PQ).

RISKS specific to C (adversarially surfaced, decision-relevant):
- DOCUMENTED SOUNDNESS-FAILURE CLASS from NTT-friendly rings: three LaBRADOR implementations collapsed to 1, ~31, and ~6 bits of soundness (target 128) because the ring split into many CRT/NTT factors, blowing up the random-linear-combination cheating probability. The repo's preferred q=12289 is NTT-friendly precisely because X^n+1 splits into many factors — so "q=12289 is favorable/lattice-native" has the SIGN BACKWARDS for soundness; speed and soundness are in direct tension. condor-rs (a nascent Rust port) is one of the buggy implementations.
- KNOWLEDGE SOUNDNESS only for a RELAXED relation R' with ~2.07x norm slack (= sqrt(128/30)*beta, Johnson-Lindenstrauss NormCheck). The base-128 range/digit and C4 account-projection-repack constraints must absorb this without losing injectivity — a 2x relaxation could let a digit exceed 127 / admit two distinct digit vectors, breaking the C4 repack-to-`packed_witness_commitment` binding. Tightening M-SIS parameters to kill the slack enlarges norms/dimensions and thus the proof.
- Adds a SECOND lattice-assumption family (M-SIS) on top of the dossier's existing EXTERNAL coded-SIS lattice-core assumption.
- A live history of soundness subtleties (LatticeFold norm-growth break motivating LatticeFold+; Fiat-Shamir attacks on folded/GKR arguments, KRS CRYPTO 2025). Route C bets account-admission soundness on the freshest cryptography in the candidate set.

## Family D — Native image-membership redesign

Maturity: CONJECTURAL. Confidence: medium. No obstruction theorem stronger than digest self-consistency exists yet.

1. BATCH VERIFICATION — only if the obstruction equations obs_c(q)=0 are cheap public algebraic checks; then N admissions reduce to N small equation evaluations (potentially batchable by random linear combination). This is a property of the equations, not a proof system.
2. AGGREGATION — none natively; there is no proof object to aggregate. Any required sub-proof's family (A/B/C) governs aggregation.
3. RECURSION — INHERITED. If image-membership shrinks the statement to compact public equations + a small lift witness, it REDUCES the recursion-circuit size for whatever outer system wraps it (smaller verifier-as-circuit = cheaper Family-A recursion) — a genuine but indirect benefit.
4. SPEND-SIGNATURE INTERACTION — high-upside but speculative; redesigning B_c could make admission a compact membership predicate that composes more naturally with spend, but this touches consensus carrier, admission registry, checked-core frame binding, and spend authorization — a protocol redesign, not a recursion feature.
5. PQ-SAFETY — inherited from the chosen field/commitment and backend; the membership map adds no recursion assumption.

NET: Family D is a STATEMENT-SHRINKING optimization that makes A or C recursion cheaper. It is NOT a recursion family and is orthogonal to this decision (it changes WHAT is proved, not HOW proofs compose).

---

## Ranking table (aggregation/recursion for a blockchain)

| Rank | Family | Batch verify | Aggregation | Recursion/IVC | Spend interaction | PQ-safe mechanism | Maturity |
|------|--------|--------------|-------------|---------------|-------------------|-------------------|----------|
| 1 | A STARK/FRI | Yes (const-factor batch-FRI; O(1)-in-N via wrapper) | Yes, deployed | Deployed (Plonky2/3, Boojum, Starknet, Miden) | Strongly positive (co-compress creation+spend → O(1) chain state) | Yes IF terminate in STARK (Groth16 wrap breaks PQ) | DEPLOYED |
| 2 | C lattice folding | Strong (folding = batch) | Native, elegant, sublinear | Headline feature (recursive SNARK + PCD), but no concrete costs | Best long-term (shared module math), speculative now | BEST (M-SIS, mechanism itself PQ) | RESEARCH-GRADE |
| 3 | D image-membership | Only if equations cheap | None native | Inherited (shrinks inner statement) | High-upside, speculative protocol redesign | Inherited | CONJECTURAL |
| 4 | B MITH/VOLEitH | Weak / linear | No native mechanism | Hostile (needs outer STARK wrap) | Neutral-to-negative | PQ base, no PQ native recursion | base DEPLOYED, recursion ABSENT |

## Blockchain implications

- Deploy-NOW recursion layer: Family A is the only candidate that is BOTH post-quantum AND has production-deployed batch-FRI aggregation, recursive composition, and IVC. Stay PQ by terminating recursion in a STARK; do not add a Groth16/PLONK wrap unless a non-PQ outermost proof is explicitly acceptable.
- Highest-leverage opportunity is NOT shrinking creation-proof bytes (creation is one-time, byte cost is the weakest constraint). It is a Family-A recursive circuit verifying {creation proof + hash-digest-referenced checked-core SPEND relation} together, emitting ONE O(1) succinct chain-state proof — rollup-style compression of the whole system, dominant over any per-proof byte tuning.
- Recursion-friendliness rewards an arithmetization-friendly hash; the SHA-256 checked-core/role bindings are a real conflict. Plan a recursion-only re-hashing layer or a SHA-friendly STARK, and keep SHA/PRF digest checks OUTSIDE the hidden AIR to avoid prover-cost blowup. Note the aggregation advantage and the in-circuit-hash prover cost are COUPLED, not independent.
- Long-term PQ-native endgame is Family C. Promote it only when (a) a knowledge-soundness proof for the TIGHTENED relation closes the ~2.07x R' slack without breaking C4 repack injectivity, (b) a non-splitting modulus is chosen that survives the NTT-friendly-ring soundness-failure class (costing the q=12289 speed advantage), (c) integrated ZK exists (currently open), and (d) a MEASURED Fq12 proof-size + verifier benchmark exists.
- Family D is a cost optimizer on top of A or C, not a standalone recursion route.

## Non-claims

- This document does NOT claim any family currently delivers a sound + compact + aggregatable + ZK argument of knowledge for the EXACT Fq12/q12289 176-row/194-lane relation. All four require additional instantiation work; the base full-witness AoK + ZK hiding (the open Path-2 blocker, including the reopened HIGH-2) is UNRESOLVED, and if the base AoK fails, recursion compatibility is moot.
- It does NOT claim measured numbers. The repo byte/compute figures ([4096,12288] B STARK band, ~36256 B MITH, 16416 B sigma, ~9344/~23680 verifier q-ops) are PLANNING FLOORS, not measured; the STARK band in particular is a repo planning floor, NOT literature-grounded (concrete FRI/STIR/WHIR proofs at 128-bit run 114-306 KiB at large degree — the Descender relation is far smaller, but the "beats MITH on bytes" claim is RETRACTED as unverified).
- It does NOT claim Nova / Hypernova / ProtoStar / Bulletproofs are post-quantum — they are DLOG/curve-based and used here only as performance comparators, never as PQ candidates.
- It does NOT claim STARKPack or the Plonky3-recursion crate are production-deployed — STARKPack is a research prototype giving constant-factor batching; the cited recursion crate is unaudited/dev-stage. The DEPLOYED recursion is the system-level composition inside Plonky2/Boojum/Starknet/Miden.
- It does NOT claim above-Johnson-bound / up-to-capacity FRI soundness is settled — the up-to-capacity proximity-gap conjecture was DISPROVEN in late 2025, so any deployment MUST pin conservative Johnson-bound parameters (delta <= 1 - sqrt(rho)), accepting higher query counts and larger proofs.
- It does NOT claim LatticeFold-class folding has a turnkey, soundness-audited IVC/PCD implementation today — the "folding leads to PCD" result is paper-level, and the eprint-to-audited-implementation gap is exactly where the repo's existing Lean<->EC paper-composition seams (GAP-B/D) would recur for a new backend.
- It does NOT claim the QROM/post-quantum Fiat-Shamir loss for FRI is tightly quantified — there is no tight FRI-specific QROM reduction; it is carried as a named-standard future-work seam.
