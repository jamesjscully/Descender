# Path 2 Native Boundary Parent Trace Extraction Open Obligations

Status: active blocker checklist.

Purpose: list exactly what remains before `ParentTraceExtractionSoundness` can
be treated as an acceptable internal proof-family assumption.

## Required Obligations

| id | status | requirement | why it matters |
|---|---|---|---|
| `commitment_extractability` | interface pinned / proof open | Accepted proof yields an extractor output for the full parent trace committed by `p_section_root(b)`. | Without this, unqueried hidden roles are not known. |
| `role_restriction_extractability` | interface pinned | Extracted trace includes every role named by `p_role_handles(b)` and `ProfileRoleComplete(profile)`. | Prevents partial witnesses from satisfying account creation. |
| `relation_knowledge_soundness` | interface pinned / proof open | Extracted trace satisfies the native-boundary relation, not just sampled row checks. | Needed for `TraceRelationValid`. |
| `proximity_to_witness_bridge` | open | If using proximity/low-degree testing, closeness implies an extractable witness trace for the committed relation. | Bare proximity is not enough. |
| `forking_or_rewinding_bound` | interface pinned / proof open | QROM/FS extraction loss is quantified for the proof-family extractor. | Needed for the final advantage bound. |
| `masked_opening_consistency` | theorem_gate_pinned | Public masked openings are consistent with hidden extracted cells and masks. | Needed to connect verifier view to extractor view. |
| `no_raw_hidden_public_openings` | partly pinned | Hidden role cells, range witnesses, masks, and blinding/noise are never verifier-visible raw. | Needed for no-export/ZK. |
| `double_trace_binding_break` | interface pinned / proof open | Two safety-distinct full traces under one parent root export a named binding break. | Needed for boundary binding and role uniqueness. |
| `domain_ordering` | ledger pinned | Root/profile/role handles/auth modes/mask domains are bound before query derivation. | Needed to avoid adaptive trace/profile choice. |

## Minimal Acceptable Theorem

A concrete proof family must supply:

```text
For all adversaries A:
  if A produces an accepting proof for (b, view, profile)
  and UsesSelectedTraceKnowledgeModes(profile),
  then an extractor E^A outputs trace such that:
    TransparentExtractsTrace(b, view, trace)
    TraceOpensBoundary(b, profile, trace)
    TracesSafetyEquivalent(b, profile, trace, trace)
    MaskedOpeningsConsistent(b, view, profile, trace)
    TraceRelationValid(b, profile, trace)
  except with probability bounded by:
    Adv_trace_commitment_binding
  + Adv_double_trace_binding
  + Adv_masked_opening_consistency
  + Adv_relation_knowledge_soundness
  + Adv_qrom_extraction_loss
  + Adv_domain_separation
```

The exact term names can change, but the categories cannot disappear.

Ledger update:
`research/path2_hash_qrom_assumption_ledger.md` now names symbolic rows for:

- `q_trace_commitment_binding`;
- `q_trace_double_binding`;
- `q_trace_masked_opening_consistency`;
- `q_trace_relation_knowledge`;
- `q_trace_proximity_bridge`;
- `q_trace_extractor_rewind`.

These feed the internal parent-trace extraction structure of
`Adv_native_boundary_zk_trace_binding_hiding`.

Relation-mode update:
`research/path2_native_boundary_relation_knowledge_soundness_contract.md` pins
`RelationKnowledgeMode` and `SupportsRelationKnowledge`. Lean now rejects
`sampledRowChecksOnly` and `proximityWithoutWitness` for
`ParentTraceExtractionSoundness`; the active theorem now further requires
`UsesSelectedTraceKnowledgeModes`, which selects `fullTraceExtractable` and
`fullWitnessRelationKnowledge` for the current profile.

Two-view consistency update:
`Descender/Path2NativeBoundary.lean` now exposes
`MaskedOpeningConsistencySoundness`; `ParentTraceExtractionSoundness` requires
the extracted trace to both open the native boundary and satisfy
`maskedOpeningsConsistent` before relation validity can close. This prevents
closing the active theorem with disjoint public masked openings and an unrelated
hidden extractor trace.

Interface hardening update:
`NativeBoundaryTraceKnowledgeSoundness` now lists
`ParentTraceCommitmentExtractionSoundness`,
`ParentTraceDoubleBindingSoundness`,
`ExtractedTraceRelationKnowledgeSoundness`,
`MaskedOpeningConsistencySoundness`, and `ParentTraceExtractionSoundness`
separately. The Rust proof-family interface digest binds matching requirement
bits, plus `qrom_extractor_rewind_bound_required` and
`double_trace_binding_break_exported`. Lean now has theorem-level
`NativeBoundaryTraceQromExtractorRewindSoundness` and
`ParentTraceDoubleBindingSoundness` obligations matching those Rust flags. This
is a fail-closed interface gate, not a proof of those obligations. The active
parent-trace extraction combiner now consumes the double-binding theorem before
returning the composed extraction contract.

## Likely Hardest Subclaim

`relation_knowledge_soundness` is probably the hardest subclaim. It must turn
acceptance of a compact transparent proof into a full relation witness. In a
STARK-like route, this means the low-degree/proximity proof, constraint checks,
and query schedule together imply a valid trace witness, not merely successful
answers at sampled positions.

## Redesign Triggers

Abandon or fall back if the chosen proof family requires any of:

- raw hidden cells opened to the verifier;
- treating Merkle roots as extractable commitments without a knowledge theorem;
- accepting proximity without a witness reconstruction theorem;
- role handles that are not tied to the extracted parent trace;
- post-query choice of profile, role handles, or native boundary digest;
- proof-size overhead close to the parked lattice-sigma floor without a better
  theorem story.

## Current Recommendation

Do not implement a verifier yet. The next useful proof step is to choose one
candidate theorem shape:

1. STARK-style full trace knowledge extraction;
2. MPC-in-the-head full witness extraction;
3. keep `ParentTraceExtractionSoundness` as a named assumption and document the
   deployability dependency.

Path 1 remains the active preference only if we can state a credible
knowledge-soundness theorem with no raw hidden openings.

## Non-Claims

- This does not prove any listed obligation.
- This does not choose final parameters or byte counts.
