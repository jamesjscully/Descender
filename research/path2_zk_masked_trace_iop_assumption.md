# Path 2 ZK Masked-Trace IOP Assumption

> MIGRATION STATUS: `ACTIVE_RESEARCH`. This assumption note is superseded by the
> native-boundary refinement. Use the active native-boundary assumption notes
> and `spec/security_model.md` for current authority.

Status: superseded by native-boundary refinement; not proved.

Original assumption: `zkMaskedTraceIOPBindingAndHiding`

Active refined assumption:

```text
NativeBoundaryZkTraceBindingAndHiding
```

See:

```text
research/path2_native_boundary_zk_trace_binding_hiding_assumption.md
```

For the Path 2 transparent account-creation payload relation, the selected
proof family is now interpreted through the native boundary `b`: accepted
transcripts are knowledge-sound for a parent committed masked trace whose role
subcommitments are typed private restrictions of `p_section_root(b)`, and whose
public query openings are zero-knowledge for all hidden role, range, mask,
blinding, and account-binding columns.

## Clauses

| id | claim | needed for |
|---|---|---|
| `binding_extractability` | For fixed native boundary projections and transcript roots, an accepting proof has an extractor that outputs one parent trace, authenticated typed role subtraces, and `h in Fiber(B_c, b)`, or exports transparentProofSoundnessBreak. | turns payload acceptance into a native-boundary witness object rather than an after-the-fact satisfying tuple |
| `role_subcommitment_binding` | A native-boundary role handle cannot authenticate two different role vectors under the same parent trace root except as inner_commitment_binding_break / boundary_binding_break. | binds R_core_66 witness lanes to the native boundary |
| `masked_query_zero_knowledge` | The public transcript distribution is simulatable from public inputs, masked_projection, trace shape, and proof-size metadata without hidden pad/projection/opening/range/account witnesses. | prevents account-creation proof bytes from changing live-slot leakage |
| `mask_freshness` | Opened masked values use domain-separated one-time masks or a simulatable equivalent; reuse across rows, slots, accounts, roles, or query domains exports live_slot_leakage_or_keygen_break. | rules out linear recovery from multiple masked openings |
| `fiat_shamir_binding` | All query indices and proof challenges are derived after binding native_boundary_digest, leaf context projection, parent trace root, role handles, matrix-domain transcript, aggregation context, and proof system id. | prevents rebinding a valid proof across account/state/slot contexts |

## Consumed By

- `direct_replayable_relation_certificate_soundness_or_break`
- `transparent_relation_knowledge_soundness`
- `query_opening_zero_knowledge_no_export`
- `public_binding_and_leaf_context_composition`

## Current Byte Floor

- source: `research/path2_transparent_proof_byte_model.md`
- range/account lane floor: `194`
- 256-row floor: `6144` bytes
- 512-row floor: `8304` bytes
- 1024-row floor: `10720` bytes

## Fallback Threshold

If concrete masked-trace proof overhead approaches or exceeds the 16416-byte lattice-sigma repetition floor without materially better assumption quality, stop this route before verifier implementation.

## Failbacks

| condition | action |
|---|---|
| raw hidden trace leaves must be opened | reject this proof family; vanilla Merkle/FRI is a negative control |
| typed role subcommitments are unavailable | add explicit algebraic opening rows or move trace roots/role roots into the binding object |
| range-gadget ZK requires too many auxiliary openings | rerun byte model and compare against lattice-sigma or MPC-in-the-head fallback |
| concrete proof overhead exceeds the fallback threshold | stop verifier work and revisit the proof-family decision |

## Non-Claims

- this does not prove the assumption
- this does not identify a concrete proof-system implementation
- this does not finalize query counts
- this does not make Path 2 deployable
