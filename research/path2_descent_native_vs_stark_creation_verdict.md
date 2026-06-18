# Descent-native ZK vs STARK for the account-creation relation — VERDICT

Date: 2026-06-18. Triggered by James's pushback: don't accept the 1.36 MB
off-the-shelf Plonky3 STARK reflexively — explore whether KoalaBear/Poseidon2 can
fix/redesign the hand-rolled descent verifier (the spirit of the project). Two
analysis agents ran a design-first feasibility red-team (analysis only, no impl).

## TL;DR

**Keep the STARK for the creation relation.** The descent-native route is real and
elegant but only wins in a deployment model Descender does not have (direct-mode,
no aggregation). Under the *established, adversarially-verified* aggregation
requirement, the STARK is correct — and the "1.36 MB" is a single-proof artifact:
batched it is **~1.6 KB/account (N=1024)**, ~0.49 KB (N=4096), recursion-native.

## Why (the decisive pivot = aggregation)

`path2_proof_family_route_clarity.md` (2026-06-17, 16-agent adversarial workflow)
already SELECTED transparent STARK/FRI PRIMARY **because it is the only PQ candidate
with production recursion/aggregation/IVC — "decisive for a blockchain"** — over the
descent-native lattice family. The next high-leverage move it names: a recursive
circuit co-verifying {creation proof + checked-core spend} into one O(1) chain-state
proof (see `path2_proof_family_aggregation_recursion_analysis.md`).

This exploration re-stress-tested the strongest descent-native alternative
specifically and CONFIRMS that decision.

## The two descent-native avenues evaluated

**Avenue 1 — make the mask a lattice/Module-LWE object (full lattice-sigma
unification). REJECTED.** It does not discharge C2 (mask = PRF(pad_seed,slot)) — it
*severs* it: the mask becomes bound to a fresh lattice secret, not to pad_seed,
breaking the C1↔C2 link that account uniqueness + duplicate-detection depend on.
Adds a new Module-LWE assumption. Aggregation hostile: q12289 splits → unfavorable
for lattice-folding soundness (no integrated ZK; ~2.07× norm-relaxed soundness).

**Avenue 2 — keep Poseidon2 PRF; sigma for the linear/SIS part + small ZK-sumcheck/
GKR for the few Poseidon2 evals. BEST descent-native option, but direct-mode-only.**
~20–40 KB, FRI-free, cheap verify, NO new foundational assumption, discharges C2
honestly (proves the PRF as the small circuit it is — *enabled by the just-landed
KoalaBear-Poseidon2 mask swap*; SHA-in-circuit was the original blocker). BUT no
native recursion → under aggregation it "collapses to wrapping in a STARK =
re-selecting the STARK." Single biggest risk: the GKR layer is not ZK by default and
its per-evaluation openings are the SAME surface HIGH-2 leaked through — must be
verified by attempted arithmetic recovery, never substring scan.

## Size/soundness physics (make-or-break number)

A compact descent-native single proof (~1.5–3 KB, 128-bit) IS achievable — but only
via Dilithium-style sparse ±1 challenges over a FRESH n=256 cyclotomic Module-SIS
ring (NOT native q12289/Fq12, and NOT helped by KoalaBear-as-a-field; sparse-challenge
sigmas live over a polynomial ring). A field-wide challenge is illegal (breaks
‖c·witness‖ shortness). Additionally the descent SIS binding does not currently close
(extracted kernel dominated by blinding deltas up to ~4096 —
`path2_descent_sis_hardness_sanity.md`); fixing it is a binding-theorem redesign.
Net: reaching ~1.5–3 KB means building a second PQ cryptosystem to roughly MATCH the
STARK's *batched* per-account size, while losing recursion.

## What survives of the descent framework

The descent/SIS structure remains the core of the SPEND signature (73-byte
linear-sigma, coded-SIS binding). The STARK is specifically the creation-relation
prover. KoalaBear/Poseidon2 (the field work done this cycle) directly serve the
recursion endgame.

## Decision status

Recommended: ACCEPT (STARK for creation; descent-native documented as the
direct-mode-only alternative — revisit only if the deployment model ever drops
aggregation). Open alternative on the table: challenge the aggregation premise and
prototype Avenue 2 head-to-head if creation is ever made direct-mode. Next
high-leverage target if accepted: the aggregation/recursion circuit.

Agents: analysis only, no code/builds. Cited: route_clarity, hiding_redesign_options,
lattice_sigma_toy_model, norm_range_surface, hardness_sanity, feasibility,
proof_friendly_inner_commitment_sketch, geometric_descent_lens.
