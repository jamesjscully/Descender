# Path 2 Proof-Family Route Clarity

Status: route-clarity note; no concrete proof family selected.

Purpose: decide whether the active proof-family blocker is merely an
implementation theorem to prove, or whether the current masked trace route is
hoping for a relationship that may not exist.

Parallel route synthesis:
`research/path2_parallel_proof_route_synthesis.md` compares the transparent
IOP, MPC-in-the-head, descent/SIS, and native image-membership probes against
the same active theorem target.

## Core Answer

We can construct a proof family to supply full committed-trace extraction and
relation knowledge, but only if the proof family is designed as an argument of
knowledge for the full native-boundary witness.

The relationship does not follow from a vanilla transparent trace proof with
Merkle commitments and sampled openings. That weaker object proves, at best,
that the opened positions and proximity checks are consistent. It does not
extract unqueried hidden role columns, masks, range witnesses, account-binding
witnesses, or the full relation trace.

So the route is viable, but conditional:

```text
argument-of-knowledge proof family => plausible theorem to prove
sampled/proximity-only proof family => relationship does not exist
```

## Exact Theorem Shape Needed

For the selected active profile, the concrete proof family must instantiate:

```text
ParentTraceCommitmentExtractionSoundness
ParentTraceDoubleBindingSoundness
MaskedOpeningConsistencySoundness
ExtractedTraceRelationKnowledgeSoundness
NativeBoundaryTraceQromExtractorRewindSoundness
NativeBoundaryTraceZeroKnowledge
```

These compose into:

```text
ParentTraceExtractionSoundness
```

and then into the active closure endpoint:

```text
transparent_view_full_interface_from_selected_profile_atomic_gates_or_break
```

The extractor output must include one full trace that:

- opens `p_section_root(b)`;
- is stable under the double-trace binding theorem;
- matches the verifier-visible masked openings;
- satisfies `NativeBoundaryTraceRelationV1`;
- assembles into `h in Fiber(B_c, b)`;
- covers every selected private role;
- keeps raw hidden cells out of public proof bytes.

## What A Transparent Route Must Actually Prove

A STARK-like or transparent IOP route is acceptable only with a theorem of this
shape:

```text
Accept(public_inputs, commitments, masked_openings, proof)
  =>
exists trace.
  Opens(parent_root, trace)
  MaskedOpeningsConsistent(trace, masked_openings)
  LowDegreeOrValidEncoding(trace)
  NativeBoundaryTraceRelationV1(trace, b, profile)
```

or a named break with a quantified soundness loss.

The theorem must also include a simulator/no-export statement:

```text
PublicTranscript(b, masked_projection)
  indistinguishable from
SimulatedTranscript(public boundary data)
```

for hidden role cells, range witnesses, masks, blinding/noise, and
account-binding witnesses.

## What Does Not Work

The following implication is invalid:

```text
Merkle root bound
+ sampled queried rows pass
+ proximity/FRI checks pass
  =>
extractor knows the full hidden native-boundary trace
```

It fails for two separate reasons:

1. **Knowledge failure.** Sampled openings do not determine unqueried hidden
   role columns. Without a knowledge extractor for the committed oracle, the
   proof does not supply `ExtractedRolesComplete`.
2. **Hiding tension.** If the verifier opens raw hidden trace cells to repair
   knowledge soundness, no-export fails. If the verifier sees only masked cells,
   the extractor still needs a theorem recovering the hidden cells and masks.

Bare proximity is also insufficient unless paired with a witness-reconstruction
theorem that binds the reconstructed witness to the same parent root and role
handles.

## Route Comparison

| route | theorem quality | byte risk | current read |
|---|---|---:|---|
| `zk_masked_trace_iop_native_boundary` | good if we can state/cite an IOP-of-knowledge theorem with ZK openings | medium | active baseline, but theorem-heavy |
| `transparent_stark_iop_with_witness_extraction` | plausible if using a known STARK/AIR knowledge theorem plus a real ZK masking layer | medium/high | best transparent fallback/comparator |
| `mpc_in_the_head_hash_zk` | cleanest full-witness extraction and ZK story | high/unknown | best theorem-quality fallback |
| `descent_sis_commitment_membership` | promising if hiding kernel and norm/range proof stay compact | high | thesis-aligned research path |
| `native_image_membership` | potentially dominant if `B_c` can be redesigned for compact image membership | unknown | high-upside redesign probe |
| `vanilla_merkle_fri_trace` | insufficient | low but irrelevant | negative control only |

## Best Current Route

The best near-term route is not to implement a verifier yet. It is to choose
one of two theorem-quality tracks:

1. **Transparent IOP of knowledge track.**
   Keep `zk_masked_trace_iop_native_boundary`, but require the proof family to
   provide an explicit committed-oracle argument of knowledge with masked
   openings. This preserves the active transparent/hash direction and likely
   best byte story, but the theorem is the hardest.

2. **Full-witness ZK fallback track.**
   Model the same `NativeBoundaryTraceRelationV1` as an MPC-in-the-head or
   related hash-ZK statement. This likely has a cleaner extraction/no-export
   theorem. If its byte estimate is tolerable, it may be the safer deployable
   route even if less elegant.

The native image-membership redesign remains worth probing, but it is not a
small proof-family substitution. It may require changing the Rust account
boundary object and should be treated as a redesign option, not the next proof
step.

## Concrete Next Test

For each candidate proof family, demand a one-page theorem statement answering:

1. What object is committed?
2. What does the extractor output?
3. Why does the output open the exact parent root in `b`?
4. Why does it include every selected private role?
5. Why do masked public openings bind to that same output?
6. Why does the output satisfy every selected relation class and row floor?
7. What is the QROM/forking loss?
8. What public transcript is simulatable without raw hidden openings?

If any candidate cannot answer these eight questions, it cannot instantiate the
active Path 2 theorem.

## Current Judgment

This is probably a proof-family selection problem, not a protocol impossibility.
But the proof family must be an argument of knowledge for the full hidden
native-boundary trace. If we choose a sampled/proximity-only construction, we
would indeed be hoping for a relationship that is not there.

The most likely deployable path is:

```text
try transparent masked IOP-of-knowledge first;
in parallel, estimate MPC-in-the-head for the same 176-row relation;
keep descent/SIS as a private-ZK transcript probe, not public C=A*v;
keep native image membership as a spec-level redesign probe;
pivot if the transparent theorem requires raw openings or generic STARK
overhead near the fallback floor.
```

## Non-Claims

- This does not prove the transparent IOP theorem.
- This does not select a concrete library or parameter set.
- This does not change the active Path 2 theorem target.
- This does not change account-creation bytes.
