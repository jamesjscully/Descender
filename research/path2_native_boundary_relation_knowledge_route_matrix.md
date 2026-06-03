# Path 2 Native Boundary Relation Knowledge Route Matrix

Status: route matrix pinned; no final proof family selected.

Purpose: compare candidate proof-family routes against the decomposed parent
trace obligation:

```text
ParentTraceCommitmentExtractionSoundness
ExtractedTraceRelationKnowledgeSoundness
```

Together these imply:

```text
ParentTraceExtractionSoundness
```

## Route Matrix

| route | commitment extraction | relation knowledge | no raw hidden openings | PQ fit | current disposition |
|---|---|---|---|---|---|
| `zk_masked_trace_iop_native_boundary` | plausible if parent trace commitment has knowledge extraction | selected profile requires full-witness relation knowledge for the extracted trace | yes by design, not proved | yes under hash/RO assumptions | active, but assumption-heavy |
| `transparent_stark_iop_with_witness_extraction` | plausible as a named STARK/IOP theorem | plausible if AIR/constraint theorem extracts satisfying trace | needs ZK masking layer | yes under hash/RO assumptions | viable broad fallback / active comparator |
| `mpc_in_the_head_hash_zk` | natural full witness extraction | natural relation knowledge for circuit witness | yes by design if instantiated correctly | yes under symmetric/hash assumptions | fallback if transparent trace ZK is too hard |
| `parked_lattice_sigma_private` | natural via special soundness | natural for linear/SIS relation | yes under simulator/rejection proof | yes under lattice assumptions | fallback floor, byte-heavy |
| `descent_sis_linear_zk_commitment` | plausible but needs commitment theorem | plausible for q-linear relation; harder for full native-boundary trace unless relation stays algebraic | yes if hiding/simulator works | yes under SIS/module-SIS | promising if trace relation can remain small |
| `vanilla_merkle_fri_trace` | no full hidden trace extractor for private columns | sampled/proximity only | no, raw query openings leak hidden cells | yes but irrelevant | dead |
| `public_linear_role_commitments` | binding maybe | relation can be checked but role vectors leak | no | yes mathematically but hiding fails | dead at current dimensions |
| `pedersen_ipa` | strong in classical setting | strong for linear relations | yes | no | prototype only |

## Interpretation

The native-boundary design has made one thing clear: the active transparent
route is not "just use FRI." It needs a ZK proof-family theorem with both:

```text
full trace extraction
full-witness relation knowledge for the extracted object
```

Vanilla proximity testing is dead unless the active profile is deliberately
changed to a witness-reconstruction route and the proof-system id changes.

## Most Promising Paths

### 1. Masked Trace IOP / STARK-Like Route

This remains the best blockchain-aligned route if we want:

- transparent setup;
- post-quantum assumptions;
- future aggregation compatibility;
- proof bytes likely below or comparable to generic circuit ZK.

But it must be treated as a serious proof-family assumption, not as a small
local lemma.

Required named assumptions:

```text
ParentTraceCommitmentExtractionSoundness
ExtractedTraceRelationKnowledgeSoundness
NoExportSoundness
NativeBoundaryTraceQromScheduleSoundness
NativeBoundaryTraceQromExtractorRewindSoundness
```

### 2. MPC-In-The-Head Route

This is conceptually safer for ZK and witness extraction:

- the proof naturally speaks about a full witness;
- simulator/no-export is native to the construction;
- relation knowledge is circuit-soundness.

The downside is proof size and weaker fit with the native parent-root/role
subcommitment API. It may be the right fallback if the masked trace IOP cannot
state a clean knowledge theorem.

### 3. Lattice/SIS Linear ZK Route

This is attractive if the relation can remain mostly q-linear and avoid generic
trace machinery. It aligns with the original Descender algebra. The risk is
that the full native-boundary trace now includes range, masking, role
restriction, and lifecycle checks; if those force many repetitions or large
ZK machinery, it may approach the parked sigma floor.

## Current Recommendation

Keep `zk_masked_trace_iop_native_boundary` active, but only as a named
proof-family assumption with the decomposed contract. The next proof task should
not be implementation or byte counting; it should be to write the precise
relation:

```text
NativeBoundaryTraceRelationV1
```

and classify every constraint as:

- algebraic q12289 row;
- base-128 range row;
- mask consistency row;
- role restriction row;
- lifecycle/context row;
- proof-system bookkeeping row.

Once that relation is explicit, we can decide whether a STARK-like assumption is
credible or whether the relation is better suited to MPC-in-the-head or
lattice/SIS ZK.

First-pass result:
`research/path2_native_boundary_trace_relation_v1_inventory.md` pins the active
relation inventory and
`research/path2_native_boundary_trace_relation_row_symbols.md` gives a symbolic
row model. The fixed lower bound is now 124 rows before open role/lifecycle/
account/checked-core/bookkeeping groups.

## Non-Claims

- This does not prove any route.
- This does not choose final bytes or parameters.
- This does not implement a verifier.
