# Path 2 Native Boundary Proof-Family Extractor Classification

Status: preliminary classification; no concrete library selected.

Purpose: test candidate proof-family shapes against the newly pinned hidden
extractor requirement:

```text
accepted proof => full role-complete h in Fiber(B_c, b)
```

The key distinction is whether the proof gives full committed-trace extraction
or only consistency for sampled openings.

## Classification

| candidate | extraction class | standing | reason |
|---|---|---|---|
| `vanilla_merkle_fri_trace` | sampled openings / proximity only | dead negative control | Public verifier opens trace cells. If hidden roles are in those cells, no-export fails. If hidden roles are not in those cells, the extractor does not recover a complete `h`. |
| `plain_merkle_commitment_with_random_queries` | sampled openings only | dead | Binding to opened leaves is not enough to recover unqueried private role columns. It cannot prove `ExtractedRolesComplete`. |
| `transparent_stark_iop_with_witness_extraction` | full trace, if the theorem is available | conditionally viable | A STARK-style IOP can be usable only if its knowledge-soundness theorem extracts an entire low-degree/trace witness from an accepting transcript, and the ZK layer makes hidden openings simulatable. |
| `zk_masked_trace_iop_native_boundary` | intended full trace plus simulator | active | This is the current path: one parent root, typed private role restrictions, extractor-only hidden columns, masked public query view, and native-boundary transcript schedule. |
| `mpc_in_the_head_hash_zk` | full witness from accepting views, usually native ZK | fallback | ZK is conceptually cleaner because verifier views reveal shares rather than raw hidden columns, but payload size and role-restriction aggregation may be worse. |
| `parked_lattice_sigma_private` | full witness via sigma extractor | fallback floor | It has a clear extractor story but currently carries the large account-creation floor that the native-boundary route was designed to avoid. |
| `public_linear_role_commitments_current_dimensions` | public full-rank image, no hiding | dead for private roles | The rank/leakage probe already showed this exposes the role vector under current dimensions. |
| `dual_commitment_hiding_plus_binding` | depends on commitment theorem | fallback research path | Could work if it gives both hiding and extractable binding for private role columns, but it adds assumptions and likely extra bytes. |

## Active Candidate Requirements

`zk_masked_trace_iop_native_boundary` remains viable only if it supplies all of:

1. Full parent-trace extraction from accepted transcripts.
2. Typed role extraction under `p_section_root(b)` and `p_role_profile(b)`.
3. Simulator-safe public openings for masked/query-visible cells.
4. Extractor-only access to raw hidden role cells.
5. Hybrid base-128 range proof for hidden role values.
6. Extractor-side bounds for blinding/noise auxiliaries.
7. Fiat-Shamir schedule matching the native-boundary QROM ledger.

## The Crucial Theorem Shape

The proof-family theorem must be closer to:

```text
Accept(public_view, proof)
  =>
exists trace.
  ParentRootOpens(parent_section_root, trace)
  AllRoleRestrictionsOpen(trace, profile, role_handles)
  RelationHolds(trace, b)
```

than to:

```text
Accept(public_view, proof)
  =>
all sampled query openings were locally consistent
```

The second shape is insufficient unless paired with a separate proximity and
knowledge-extraction theorem that reconstructs the full committed trace.

## Extraction Mode Gate

The Lean profile now carries an extraction capability:

```text
TraceExtractionMode =
  sampledOpeningOnly
  | proximityOnly
  | fullTraceExtractable
  | fullWitnessExtractable
```

The active theorem requires:

```text
UsesSelectedTraceKnowledgeModes(profile)
```

so `sampledOpeningOnly` and bare `proximityOnly` profiles are rejected for the
native-boundary route. The selected profile is `fullTraceExtractable` plus
`fullWitnessRelationKnowledge`. A proximity proof can become viable only by
changing the active profile/proof-system id to a witness-reconstruction mode.

## Practical Implication

This does not require committing to a concrete STARK implementation yet. It
does require rejecting any proof-system description whose soundness theorem is
only local-query consistency. For this project, "transparent" is not enough;
we need transparent plus knowledge soundness for the whole native-boundary
trace.

## Next Proof Move

Define the minimum theorem interface for the active candidate:

```text
NativeBoundaryTraceKnowledgeSoundness
NativeBoundaryTraceZeroKnowledge
NativeBoundaryTraceQromSchedule
```

Then decide whether those three assumptions are acceptable as one named
proof-family assumption or whether any one of them needs to be decomposed
further before we can call the proof side closed.

## Non-Claims

- This does not select a concrete STARK, IOP, or MPC-in-the-head construction.
- This does not estimate concrete proof bytes.
- This does not prove simulator security or knowledge soundness.
