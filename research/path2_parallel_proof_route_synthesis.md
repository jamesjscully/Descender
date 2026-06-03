# Path 2 Parallel Proof Route Synthesis

Status: synthesis of four parallel route probes; no final proof family selected.

Purpose: combine the transparent IOP, MPC-in-the-head, descent/SIS, and native
image-membership probes into one route decision frame for the active Path 2
native-boundary account-creation proof.

## Common Target

Every route is judged against the same active theorem target:

```text
accepted account-creation boundary b
  =>
exists h.
  h in Fiber(B_c, b)
  LocalValid_c(h)
  DescentConsistent_c(h)
```

with no public export of the hidden representative, pad source, opening tuple,
masks, range witnesses, blinding/noise, or raw hidden role columns.

The current selected transparent endpoint is:

```text
transparent_view_full_interface_from_selected_profile_atomic_gates_or_break
```

The current hidden relation planning object is `NativeBoundaryTraceRelationV1`,
with:

- `176` selected working relation rows;
- selected per-class row floors;
- `194` range/account witness lane floor;
- `16416` bytes as the parked lattice-sigma comparator.

## Route Verdicts

| route | classification | main win | main risk | current verdict |
|---|---|---|---|---|
| transparent masked trace IOP / STARK-like IOP of knowledge | proof-family instantiation of current endpoint | preserves transparent/hash direction and current Lean/Rust interface | must provide a real committed-oracle argument of knowledge with ZK masked openings | active baseline, theorem-heavy |
| MPC-in-the-head / full-witness hash ZK | parallel proof-family endpoint or selected-profile mode | full-witness extraction and ZK are native theorem features | proof bytes, verifier work, and aggregation may be worse | best theorem-quality fallback |
| descent/SIS commitment membership | replacement theorem or protocol redesign, depending on public object | aligns with q12289/descent math and PQ commitment thesis | public `C=A*v` leaks; private ZK/range proof may be large | research path, not a drop-in trace instantiation |
| native image-membership redesign | protocol redesign probe | may remove full trace extraction as the account-admission proof | requires redesigning `B_c`/boundary semantics and proving image/lift theorem | high-upside redesign, not immediate proof-family work |

## Transparent IOP Probe

This route is viable only if the proof family supplies a full committed-trace
argument of knowledge:

```text
Accept(public_inputs, commitments, masked_openings, proof)
  =>
exists trace.
  TraceOpensBoundary(b, profile, trace)
  MaskedOpeningsConsistent(b, view, profile, trace)
  TraceRelationValid(b, profile, trace)
```

or a named break with quantified loss.

What is already pinned:

- Lean selected profile requires `fullTraceExtractable` and
  `fullWitnessRelationKnowledge`.
- Lean requires relation class coverage, a `176` row floor, and per-class
  floors.
- Rust digest-binds the proof-family interface and negative controls for
  missing extraction, relation knowledge, QROM rewind, no-export, row floors,
  and selected endpoint flags.

Main false assumption to avoid:

```text
Merkle root + sampled openings + FRI/proximity
  =>
full hidden native-boundary trace extraction
```

That implication is invalid unless a separate witness-reconstruction /
IOP-of-knowledge theorem binds the reconstructed object to the same parent
root, role handles, masks, range witnesses, and account-binding rows.

Go if:

- there is a concrete proof-family theorem or acceptable named assumption for
  full committed-trace extraction;
- masked public openings are simulator-safe and tied to the extracted trace;
- typed role subcommitments are extractable and no-export-safe;
- QROM/forking loss is quantified;
- concrete overhead stays materially below the fallback comparator.

No-go if:

- hidden cells or range witnesses must be opened raw;
- the theorem is only sampled local consistency or bare proximity;
- role handles are digest labels without typed extractable semantics;
- overhead approaches the fallback floor without better theorem quality.

## MPC-In-The-Head Probe

This route should be stated as a full-witness argument of knowledge for the
same active relation:

```text
MITHAccept(b, view, proof)
  =>
exists trace h.
  TraceOpensBoundary(b, profile, trace)
  MaskedOpeningsConsistent(b, view, profile, trace)
  TraceRelationValid(b, profile, trace)
  TraceAssemblesRepresentative(trace, h)
  ExtractedRolesComplete(b, profile, h)
  InFiber(P, b.context, b, h)
  LocalValid(b.context, h)
  DescentConsistent(b.context, h)
```

or a named binding/QROM/soundness/no-export break.

Theorem advantage:

- extraction of the full witness is native to the construction;
- simulator/no-export is easier to state because opened views are shares;
- it avoids relying on sampled Merkle/FRI openings to imply hidden trace
  extraction.

Needed Lean/Rust shape:

- either add a second selected profile/mode using `fullWitnessExtractable`, or
  add a parallel endpoint consuming `MITHFullWitnessExtractionSoundness`;
- map the MITH witness into the existing `ExtractedTrace` and
  `HiddenRepresentative` surfaces so assembly/range/blinding/no-export lemmas
  can be reused;
- keep role handles algebraic/proof-friendly, not SHA preimage semantics inside
  the hidden relation.

Go if:

- a 176-row / 194-lane MITH proof is plausibly below or near `16416` bytes;
- opened MPC views are simulatable and do not leak raw hidden columns;
- extractor maps cleanly into the active `ExtractedTrace`;
- QROM rewind/forking loss is quantified.

No-go if:

- proof bytes or verifier work are substantially worse than transparent stress
  rows without better deployability;
- role restriction becomes SHA-heavy inside the witness circuit;
- aggregation/verifier cost is unacceptable for account creation.

## Descent/SIS Probe

This is not a drop-in instantiation of the current trace extractor if the route
publishes SIS commitments as the admission object. The current public
`C = A*v mod q` shape remains dead because the role matrices are full column
rank and reveal openings.

It can become a proof-family replacement only if SIS commitments are
proof-internal or otherwise hiding, while the public boundary remains the
selected Path 2 account object.

Replacement theorem shape:

```text
AcceptCreateSis(pub, payload)
  =>
exists hidden_opening representative account_evidence.
  masked_projection_i = pad_i + account_projection_i mod q
  binding digest agrees with leaf/context
  norm/range/digit constraints hold
  hidden opening is account-backed
  local quotient descent glues to Fiber(B_c, b)
```

or a named SIS binding, sampler hiding, norm/range, QROM, extractor, or
no-export break.

Main risks:

- public full-rank leakage is fatal;
- broad randomness helps hiding but weakens short-SIS binding;
- bounded randomness helps binding but needs a hiding/sampler theorem;
- raw q12289 residues are too wide for clean shortness, so base-128 digit/range
  proof remains necessary;
- verifier must never see pad and account-projection lanes separately.

Next test:

Build a toy `lattice_sigma_rejection_zk` transcript model for the exact private
payload surface, including repetitions, rejection sampling, range proof cost,
and simulator transcript, then compare against the transparent and MITH
fallbacks.

## Native Image-Membership Probe

This is the high-upside redesign route. It asks whether `B_c` can be chosen so
that valid-boundary image membership has compact public obstruction equations:

```text
q : Q_c
obs_c(q) = 0
  =>
exists h.
  boundary_c(h) = q
  LocalValid_c(h)
```

with a safety-saturated kernel:

```text
B_c(h) = B_c(h') => h ~safe h'
```

Possible simplification:

- demote full committed-trace extraction as the admission proof;
- reduce role-restriction and two-view masked-opening consistency burden;
- make account admission more like compact image membership than a generic
  hidden trace proof.

Non-removable constraints:

- valid private fiber existence;
- projection completeness;
- checked-core frame binding;
- lifecycle consume-once/storage semantics;
- no-export/fiber privacy;
- same spend/replay/lifecycle behavior.

Scope warning:

This is not purely local if it changes what consensus calls the native
boundary. It can touch projection bundles, boundary digests, consensus carrier,
admission registry, checked-core frame binding, and spend authorization.

Smallest safe experiment:

Write a spec-only `Path2NativeBoundaryImageMembershipV0` table over the current
projection bundle:

1. public obstruction equations;
2. mapping to current `assemble_path2_native_boundary_v1` checks;
3. the single remaining lift theorem;
4. no-export invariants.

If this table is only digest self-consistency, the route is no-go.

## Integrated Recommendation

Run two concrete next probes in parallel:

1. **Transparent theorem probe.**
   Write a one-page concrete theorem for a masked trace IOP of knowledge:
   commitment object, extractor, extracted trace, relation theorem,
   masked-opening consistency, simulator theorem, and QROM loss.

2. **MITH byte/theorem probe.**
   Estimate an MPC-in-the-head proof for the same `176`-row / `194`-lane
   relation and state its full-witness extraction theorem.

Keep descent/SIS as a separate private-ZK transcript experiment, not as a
public SIS commitment route. Keep native image membership as a spec-level
redesign probe only until it shows an obstruction theorem stronger than digest
self-consistency.

## Current Judgment

The best deployable path is still unresolved, but the structure is clearer:

- The transparent route is not impossible; it needs a real IOP-of-knowledge
  theorem, not sampled/proximity soundness.
- MITH is the cleanest theorem fallback and should be compared on bytes.
- SIS/descent is attractive only if commitments are private/ZK and avoid the
  full-rank leakage dead path.
- Native image membership could dominate, but only as a genuine boundary-map
  redesign.

## Non-Claims

- This does not select a concrete proof family.
- This does not prove any route.
- This does not modify the active theorem target.
- This does not update production bytes.
