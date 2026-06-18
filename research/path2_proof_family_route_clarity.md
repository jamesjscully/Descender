# Path 2 Proof-Family Route Clarity

Status: route-clarity note. 2026-06-17 UPDATE — strategy-level family SELECTED
(see decision block below); concrete proof-system instantiation still open.

## 2026-06-17 Decision (adversarially verified, 16-agent workflow)

**SELECTED: Transparent STARK/FRI IOP-of-knowledge (Family A) PRIMARY.** MITH/
VOLEitH (B) = theorem-clean fallback. LaBRADOR/LatticeFold-class lattice folding
(C) = strategic PQ-native long-term bet. Native image-membership (D) = statement-
shrinking probe only. A is the only candidate that is BOTH post-quantum AND has
production-deployed recursion/aggregation/IVC, which is decisive for a blockchain.

Flip-condition verdicts (each survived two independent adversarial skeptics):

- **Flip A (wide-challenge MITH rises to primary): does NOT fire.** Few-round
  MITH is real/deployed, but MITH has no native recursion; aggregating it
  collapses to wrapping it in an external STARK = re-selecting A. Repo's ~7.3 KB
  MITH floor RETRACTED (scales with gate count; honest band ~5-30 KB).
- **Flip B (lattice folding makes C primary now): does NOT fire.** Research-grade,
  no integrated ZK (open problem), only ~2.07x norm-relaxed knowledge soundness
  (can break C4 repack injectivity), and a documented NTT-friendly-ring soundness-
  FAILURE class. CRITICAL: **q=12289 is NTT-friendly because it splits, so it is
  UNFAVORABLE for lattice-folding soundness** — promoting C needs a non-splitting
  modulus (loses the q=12289 speed advantage).
- **Flip C (STARK zk-masking fails): does NOT fire — STARK survives.** zk-STARKs
  hide all 194 lanes without raw openings (2024/1037); FRI is a genuine AoK with
  a quantified extractor (2023/1071). Caveats: byte band `[4096,12288] B` is a
  repo planning floor (real STARK proofs are tens-to-hundreds of KB) so the
  "beats MITH on bytes" claim is RETRACTED (bytes are the weakest constraint);
  the late-2025 proximity-gap disproof forces conservative Johnson-bound FRI
  params (~147 queries); QROM is a named seam.

8-question AoK test on STARK = **CONDITIONAL-PASS** (Q1/Q3/Q6 pass; Q2/Q4/Q5/Q7/
Q8 are instantiation debt). Highest-leverage move: a Family-A recursive circuit
co-verifying {creation proof + checked-core spend} into one O(1) chain-state
proof. Full aggregation/recursion analysis:
`research/path2_proof_family_aggregation_recursion_analysis.md`. Highest-risk
active-route obstruction now being attacked:
`research/path2_stark_route_c1c2_sha_binding_bridge_risk.md`.

---

Original note (pre-decision):

Purpose: decide whether the active proof-family blocker is merely an
implementation theorem to prove, or whether the current masked trace route is
hoping for a relationship that may not exist.

Parallel route synthesis:
`research/path2_parallel_proof_route_synthesis.md` compares the transparent
IOP, MPC-in-the-head, descent/SIS, and native image-membership probes against
the same active theorem target.
Concrete next probes:
`research/path2_transparent_iop_knowledge_theorem_probe.md` states the
transparent IOP-of-knowledge theorem surface, and
`research/path2_mith_full_witness_probe.md` states the MITH full-witness
fallback theorem, optimistic wide-challenge byte floor, and conservative
64-round extrapolation.

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

The current Rust masked-share core is now a concrete negative example of this
failure mode. `research/path2_current_core_negative_theorem.md` proves that
the executable core accepts a synthetic public-only language point with
arbitrary public context digests and masked lanes, so it cannot imply knowledge
of C1/C2/C4/C6. This does not kill the transparent route, but it does force the
route to become a real argument of knowledge for the full relation rather than
a sampled masked-share proximity proof.

`research/path2_current_view_binding_negative_theorem.md` adds the companion
composition no-go: the current transcript can bind a
`creation_proof_binding_digest` next to a synthetic trace root without proving
that the trace opens the binding's role commitments. The transparent route must
therefore supply typed extractable role subcommitments or a full-relation
subproof; two-hop digest co-location is not enough.
Lean now also checks the minimal current-row-core countermodel as
`current_masked_share_core_acceptance_does_not_imply_full_hidden_opening` and
the minimal digest co-location countermodel as
`digest_colocation_does_not_imply_trace_role_binding`; the combined current-route
verdict is `current_transparent_route_blocker1_negative_witness`.

The positive route is also now pinned:
`research/path2_full_relation_backend_sufficiency_theorem.md` adds the Lean
certificate
`SelectedTransparentFullRelationBackendCertificate`. If a concrete backend
instantiates that certificate, Lean proves it implies `AccountAdmissionSoundness`.
Rust pins the same target with
`Path2TransparentFullRelationBackendSufficiencyTargetV1`; its digest is carried
by the full-hidden-relation contract, production obligations, and proof-family
capability while the concrete certificate remains uninstantiated.

## Route Comparison

| route | theorem quality | byte risk | current read |
|---|---|---:|---|
| `zk_masked_trace_iop_native_boundary` | good if we can state/cite an IOP-of-knowledge theorem with ZK openings | medium | active baseline, but theorem-heavy |
| `transparent_stark_iop_with_witness_extraction` | plausible if using a known STARK/AIR knowledge theorem plus a real ZK masking layer | medium/high | best transparent fallback/comparator |
| `mpc_in_the_head_hash_zk` | cleanest full-witness extraction and ZK story | high; conservative active extrapolation is about 36.3 KB | theorem-quality fallback unless wide challenges make bytes competitive |
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
   theorem. The current 64-round extrapolation is not byte-competitive unless
   wide challenge action cuts repetitions with quantified QROM loss.

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
