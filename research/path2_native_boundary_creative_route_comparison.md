# Path 2 Native Boundary Creative Route Comparison

Status: first-pass route comparison.

Purpose: compare alternatives after reframing the problem as:

```text
zero-knowledge image-membership proof for B_c
```

instead of as a fixed masked trace proof.

## Evaluation Criteria

A route is attractive if it improves at least one of:

- proof theorem simplicity;
- proof bytes;
- verifier cost;
- aggregation compatibility;
- post-quantum assumption fit;
- alignment with descent/native-boundary mathematics;
- risk of hidden leakage/replay bugs.

It is rejected if it cannot establish:

```text
exists h. B_c(h)=b and LocalValid_c(h) and DescentConsistent_c(h)
```

without exporting private representative data.

## Route 1: Native Image Membership

Idea:

Redesign `B_c` so that membership in the valid image has a compact algebraic
certificate:

```text
b in Im(B_c | valid representatives)
```

instead of proving a large operational trace.

Potential win:

- could eliminate full trace extraction;
- aligns best with the descent framing;
- may make role consistency native rather than proven;
- could improve aggregation because image-membership certificates may compose.

Risk:

- unclear whether such a compact image test exists;
- may require redesigning the account boundary object;
- hiding still needs a ZK layer unless the image test is naturally hiding.

Best next question:

```text
Can we construct B_c so that its image is characterized by low-rank syndrome,
descent-gluing, or quotient equations with compact witnesses?
```

Disposition: most mathematically promising, highest uncertainty.

## Route 2: Descent/SIS Commitment Membership

Idea:

Use SIS/module-SIS commitments as the native role restriction mechanism, with
ZK proof of opening to a valid descent representative.

Potential win:

- PQ-native;
- q12289 arithmetic aligns with current relation;
- avoids generic STARK trace machinery;
- may connect directly to the original Descender commitment thesis.

Risk:

- hiding vs binding/norm bounds are delicate;
- previous public full-rank commitments leaked;
- compact ZK may approach lattice-sigma repetition floor;
- range/blinding policy still hard.

Best next question:

```text
Can we make the commitment hiding kernel native, rather than using public
full-rank C=A*v or a large sigma repetition?
```

Disposition: strong second path; likely worth parallel investigation.

## Route 3: Full-Witness Hash ZK / MPC-In-The-Head

Idea:

Stop trying to retrofit ZK onto a transparent trace. Prove the image-membership
relation as a full witness statement using MPC-in-the-head or related
hash/symmetric ZK.

Potential win:

- full witness extraction is natural;
- no-export is native;
- relation knowledge is easier to state;
- PQ under symmetric/hash assumptions.

Risk:

- proof sizes may exceed transparent route;
- aggregation story is weaker;
- relation must be circuitized;
- if SHA-heavy fields re-enter, cost rises quickly.

Best next question:

```text
For a 176-row algebraic relation with no SHA preimages, is MPC-in-the-head
actually smaller or cleaner than masked trace/STARK?
```

Disposition: best fallback if theorem quality, not row count, blocks STARK.

## Route 4: Keep Masked Trace / STARK-Like Route

Idea:

Continue current path:

```text
full trace extraction + relation knowledge + no-export
```

for `NativeBoundaryTraceRelationV1`.

Potential win:

- transparent setup;
- blockchain-friendly verification model;
- future aggregation/rollup compatibility;
- current row estimate is not fatal.

Risk:

- requires a strong proof-family knowledge theorem;
- ZK query openings are nontrivial;
- 1024-row stress has little byte headroom;
- could become generic STARK engineering.

Best next question:

```text
Can a concrete transparent proof family satisfy full trace extraction and
relation knowledge for hidden columns without large overhead?
```

Disposition: active baseline, but not obviously dominant anymore.

## Route 5: Split Admission Layers

Idea:

Let account creation prove a smaller boundary certificate and defer some checks
to spend/state transition.

Potential win:

- smaller account-creation proof;
- lifecycle and checked-core checks may stay outside the hidden relation;
- could better match blockchain execution.

Risk:

- dangerous if a bad live slot can be admitted;
- spend may not be able to repair an invalid boundary safely;
- deferred checks must not change replay/leakage surface.

Best next question:

```text
Which relation rows are strictly admission-critical, and which are safe
spend/state obligations?
```

Disposition: worth auditing, but must be conservative.

## Ranking For Next Investigation

| rank | route | reason |
|---:|---|---|
| 1 | `native_image_membership` | could remove the proof-family bottleneck by changing the mathematical object |
| 2 | `descent_sis_commitment_membership` | best alignment with original Descender thesis and PQ q12289 arithmetic |
| 3 | `masked_trace_stark_like` | current active baseline; row count plausible but theorem overhead uncertain |
| 4 | `full_witness_hash_zk` | clean theorem fallback if transparent knowledge/ZK is too heavy |
| 5 | `split_admission_layers` | useful optimization/audit, but risky if it weakens admission soundness |

## Immediate Recommendation

Investigate `native_image_membership` first, but narrowly:

```text
Can B_c be chosen so that valid image membership is checked by compact
descent/gluing equations rather than full trace extraction?
```

If yes, it may dominate all proof-family engineering routes.

If no, investigate `descent_sis_commitment_membership` next, because it is the
most thesis-aligned way to avoid generic STARK overhead.

Keep the masked trace route as the measured baseline, not the only path.

## Non-Claims

- This does not prove any alternative route.
- This does not kill the active masked trace route.
- This does not update byte estimates.
