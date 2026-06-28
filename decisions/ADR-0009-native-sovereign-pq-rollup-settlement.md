# ADR-0009: Native Sovereign Post-Quantum Validity-Rollup Settlement

Status: Accepted
Date: 2026-06-28

## Context

`ADR-0008` committed to a hard quantum-128 target and noted that the higher FRI
parameters raise on-chain verification cost, moving the Mode B aggregation path onto
the deployable critical path. That aggregation now exists: a true single-proof O(1)
block aggregator (`descender_recursion::block_recursive_air`) attests, in ONE proof,
that every block member was verified in-circuit and chains to one `chain_state_root`,
at production q128 knobs. The remaining deployment question is WHERE that one proof is
verified.

`descender_recursion::verifier_cost` measured the per-block verifier cost: naive
in-EVM verification of a KoalaBear/Poseidon2 STARK at 120 FRI queries is far above
Ethereum's block gas limit (~72M–1.3B gas/block at illustrative per-permutation
costs), so a naive in-EVM native-STARK verifier is not economically viable on L1
regardless of aggregation. Three options can carry the aggregate proof, but they
preserve quantum-128 differently:

- **Poseidon2 EVM precompile** — brings gas to the low millions/block and is
  post-quantum-preserving, but requires an EVM-client fork plus an EIP, which is
  outside this repository's control.
- **SNARK-wrap** — re-prove the aggregate STARK in a pairing SNARK verified by the
  existing EVM precompiles (~250k gas, viable on today's L1), but a pairing SNARK is
  NOT post-quantum: it inserts a non-PQ trust boundary, defeating the stated premise
  for anything that relies on it for validity.
- **Native settlement** — Descender settles as a sovereign / L2 post-quantum
  validity rollup whose full nodes, sequencer, and DA verifiers run the native STARK
  verifier off-EVM. Verification is O(1) per block at native speed with no EVM gas,
  it is post-quantum end-to-end (Poseidon2/FRI throughout), and it is buildable
  inside this repository with no external dependency. That verifier is already built
  and tested at production q128 knobs (`descender_recursion::settlement`:
  `prod_config` / `prove_block` / `verify_block` / proof transport).

## Decision

Adopt **native sovereign post-quantum validity-rollup settlement** as the canonical
deployment architecture for the quantum-128 standard. The chain's validity and
economic viability are established by the O(1) aggregate proof verified by its own
full nodes via `descender_recursion::settlement::verify_block`; there is no EVM-gas
cost on the critical path, and the trust base is post-quantum end-to-end.

Settlement to, or anchoring on, an external L1 (e.g. Ethereum) is OPTIONAL and is NOT
part of the quantum-128 trust base. If an L1 anchor is later required, the anchor
proof must be supplied either by a Poseidon2 precompile (post-quantum-preserving,
pending upstream EVM support) or by a SNARK-wrap (which introduces a non-post-quantum
assumption FOR THE ANCHOR ONLY, never for the chain's own validity). That choice, and
any L1 bridge, is deferred to its own future ADR and does not gate the quantum-128
deployable standard.

This decision resolves the settlement-architecture deployment question that was left
open in `research/path2_economic_deployment_architecture_v1.md`; it does not decide
the optional L1-bridge sub-question.

## Consequences

- Economic viability rests on O(1) aggregation plus native verification, not on EVM
  gas; the measured per-block permutation counts are throughput numbers, not gas.
- The deployable verifier is `descender_recursion::settlement`; it reads the leaf
  `PROD_*` FRI knobs as the single source of truth, so the chain verifier cannot
  silently desync from the prover.
- The precompile-vs-SNARK-wrap question is demoted to an optional, separately-gated
  L1-anchor add-on and no longer blocks the quantum-128 deployable standard.
- A sovereign rollup means the project owns consensus, data availability, and
  finality; the pad-seed registry and spend lifecycle are wired into that consensus
  with an explicit finality profile (`ADR-0010`).
- External third-party cryptanalysis remains required before production and is out of
  scope for this ADR (`ADR-0008`); the code remains on the `q128-reparam` worktree.

## Supersedes

None. Builds on `ADR-0008` (quantum-128 target) and refines, without superseding,
`ADR-0003` (direct mode before aggregation): direct mode remains a valid mode; the
aggregated path is the deployable settlement path.

## Superseded By

None.

## Affected Specs / Tracker Items

- `research/path2_economic_deployment_architecture_v1.md`
- `research/path2_q128_build_status_and_architecture_v1.md`
- `TRACKER.md` (`AGG-ROLLUP-001`)
