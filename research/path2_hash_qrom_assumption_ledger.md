# Path 2 Hash And QROM Assumption Ledger

Status: direct-mode assumption ledger for account-creation / prevalidated-slot
authorization. Direct-mode query-budget symbols, including the active
native-boundary transparent proof symbols, numeric caps, and public schema
separation are executable as a first-class summary object.

## Scope

Path 2 has two phases:

- account creation or slot refresh admits a live prevalidated slot;
- spend consumes one admitted slot and binds it to the checked-core message.

The proof must not reuse one random-oracle or PRF assumption ambiguously across
those phases. Every digest below is a separate role even when the implementation
uses SHA-256 for several roles.

## Domain Inventory

| domain | phase | role | status |
|---|---|---|---|
| `DESCENDER_SLOT_LEAF_V1` | creation/state | canonical leaf preimage domain | registered as Path 2 planning domain |
| `DESCENDER_SLOT_AUTHORIZATION_V1` | creation/spend | checked-core authorization digest domain | registered as Path 2 planning domain |
| `DESCENDER_SLOT_PREVALIDATE_SINGLE_V1` | creation | direct single-slot evidence domain | registered as Path 2 planning domain |
| `DESCENDER_SLOT_PREVALIDATE_BATCH_V1` | creation | optional batch evidence domain | registered as Path 2 planning domain |
| `DESCENDER_SLOT_ROOT_UPDATE_V1` | creation/state | optional account-root update domain | registered as Path 2 planning domain |
| `DESCENDER_SLOT_CONSUME_SINGLE_V1` | spend | direct consume domain | registered as Path 2 planning domain |
| `DESCENDER_SLOT_CONSUME_BATCH_V1` | spend | optional batch consume domain | registered as Path 2 planning domain |
| `reviewed_fq12_packed_witness_state18_projection_mask` | creation relation | projection-mask PRF input domain | existing reviewed Fq12 domain |
| checked-core challenge domain | spend | Fiat-Shamir challenge | existing checked-core domain |
| pad-source PRF/hash domains | creation relation | pad opening and pad-source projections | existing reviewed Fq12 domains |

Direct single-slot deployment consumes only the direct domains. Batch domains
must reject in direct mode unless the optional aggregation theorem is enabled.

## Digest Roles

| object | definition | assumption charged | break if violated |
|---|---|---|---|
| `prevalidated_slot_leaf_digest` | `SHA256(PrevalidatedSlotLeafV1_bytes)` | collision/binding for canonical leaf frame | `prevalidation_leaf_binding_break` |
| `slot_authorization_digest` | `SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || PrevalidatedSlotLeafV1_bytes)` | collision/binding between spend frame and admitted leaf | `prevalidation_leaf_binding_break` or `checked_core_message_binding_break` |
| `prevalidation_transcript_digest` | domain-separated evidence envelope, duplicated leaf binding fields, and payload digest | transcript binding and QROM programmability separation | `qrom_or_domain_separation_break` |
| `evidence_payload_digest` | digest of replayable certificate/proof payload | data availability binding, not proof semantics | `prevalidation_data_availability_break` or `qrom_or_domain_separation_break` |
| `hidden_source_statement_digest` | digest of active-lane public relation statement | statement binding | `creation_time_hidden_source_soundness_break` or `prevalidation_leaf_binding_break` |
| `opening_commitment` | commitment/digest over state-assisted opening tuple | binding to selected opening tuple | opening binding or hidden-source soundness break |
| `pad_seed_digest` | digest of hidden pad seed | preimage resistance and duplicate detection | `live_slot_leakage_or_keygen_break` |
| `lifecycle_anchor` | digest over restore/rollback context | lifecycle binding | `slot_replay_or_rollback_break` |
| checked-core challenge digest | challenge frame over message/key/slot/auth/signature body | Fiat-Shamir soundness and message binding | `checked_core_message_binding_break` |
| optional batch root | root over canonical slot leaves | accumulator/root collision binding | `optional_aggregation_soundness_break` |
| `native_boundary_digest` | digest of `Path2NativeBoundaryV1` public boundary | native-boundary account admission, two-view extraction, and spend handoff binding | `boundary_binding_break`, `descent_soundness_break`, or `qrom_or_domain_separation_break` |
| `parent_section_root` | root/commitment for the descent section from which private roles restrict | parent-root binding for private role handles | `commitment_opening_break` or `descent_soundness_break` |
| `role_profile_digest` / `column_profile` | typed role and column-profile policy bound before queries | role restriction soundness and mask-domain separation | `transparent_proof_soundness_break` or `qrom_or_domain_separation_break` |
| `role_handle_table` | public handles for private/public typed restrictions of `parent_section_root` | private role authentication under the native boundary | `commitment_opening_break`, `transparent_proof_soundness_break`, or `descent_soundness_break` |
| `role_authentication_mode_table` | allowed authentication mode for each role handle | prevents hidden-role downgrade to public raw openings or public full-rank linear commitments | `transparent_proof_soundness_break` or `qrom_or_domain_separation_break` |
| `masked_query_commitment_roots` | commitments to masked query openings in the transparent view | query-opening binding without exporting hidden role vectors | `transparent_proof_soundness_break` or `no_export_zk_break` |
| `no_export_policy_id` | policy digest for simulator-only/extractor-only hidden openings | no-export/hiding separation | `no_export_zk_break` or `qrom_or_domain_separation_break` |
| `norm_range_mode` | selected range representation, currently `hybridBase128HiddenRoles` | range soundness for hidden role openings | `norm_range_break` or `transparent_proof_soundness_break` |
| `blinding_noise_policy_id` | selected extractor-side masking/noise policy | prevents extractor proof from relying on unbounded hidden cancellations | `blinding_noise_bound_break` or `transparent_proof_soundness_break` |

## PRF And Hiding Roles

| primitive role | phase | hidden input | public input | assumption |
|---|---|---|---|---|
| projection-mask PRF | creation/live-slot | pad seed | slot id, lane id, profile/domain | six masked projection residues are uniform-looking before opening |
| pad-source derivation | creation relation | pad seed | profile/domain/slot context | public pad-opening statement and pad lanes bind to the same pad source |
| checked-core Fiat-Shamir | spend | none, modeled as RO | message, key, slot, authorization digest, signature body | adversary cannot reprogram challenge after slot admission |

If a creation certificate exposes raw pad seed, projection masks, or unmasked
selected projection residues, the projection-mask PRF hiding assumption no
longer applies to that public view. That case must be modeled as
`public_creation_opening_model`, not as the standard Path 2 live-slot view.

## Symbolic Query Budget

The final theorem should quantify over:

| symbol | meaning |
|---|---|
| `q_leaf` | leaf digest queries |
| `q_auth` | slot authorization digest queries |
| `q_evidence` | evidence transcript/payload digest queries |
| `q_stmt` | hidden-source statement digest queries |
| `q_pad` | pad digest/PRF queries |
| `q_mask` | projection-mask PRF queries |
| `q_chal` | checked-core challenge queries |
| `q_life` | lifecycle anchor/root queries |
| `q_transparent_trace_commit` | selected transparent payload trace commitment queries |
| `q_transparent_query_index` | selected transparent payload query-index derivation queries |
| `q_transparent_merkle_open` | selected transparent payload Merkle/opening queries |
| `q_transparent_consistency` | selected transparent payload consistency/FRI queries |
| `q_transparent_extractor` | selected transparent payload extractor/forking queries |
| `q_transparent_simulator` | selected transparent payload simulator/no-export queries |
| `q_trace_commitment_binding` | parent trace commitment and double-opening binding queries |
| `q_trace_relation_knowledge` | proof-family relation knowledge-soundness queries |
| `q_trace_proximity_bridge` | proximity/list-decoding-to-witness bridge queries, if used |
| `q_trace_extractor_rewind` | extractor rewinding/forking queries for full trace extraction |
| `q_native_boundary` | native-boundary transcript-binding queries |
| `q_native_role_handle` | role-handle table and parent-root authentication queries |
| `q_native_role_auth_mode` | role authentication mode table queries |
| `q_native_mask_domain` | per-query native-boundary mask-domain derivation queries |
| `q_native_no_export_policy` | no-export policy challenge/programming queries |
| `q_native_norm_range_policy` | hybrid base-128 range-check challenge queries |
| `q_native_blinding_policy` | blinding/noise bound challenge queries |
| `q_batch` | optional accumulator/batch-root queries |

The direct-mode advantage should have no `q_batch` term unless aggregation or
batch inclusion is enabled.

## Executable Direct-Mode Query Contract

`tools/parameter-search/src/production_reference.rs` now exposes
`path2_direct_query_budget_v1`, with schema
`DESCENDER_PATH2_DIRECT_QUERY_BUDGET_V1`.

The direct-mode numeric caps are inherited from the current QROM challenge
budget memo:

| quantity | value |
|---|---:|
| target security | 128 bits |
| deployment quantum hash-query cap | `2^16` |
| deployment classical hash-query cap | `2^33` |
| QROM quadratic loss at deployment cap | 32 bits |
| transcript term after QROM loss | 130 bits |
| minimum deployment margin | 1 bit |

For direct single-slot mode it marks these symbols active:

```text
q_leaf, q_auth, q_evidence, q_stmt, q_pad, q_mask, q_chal, q_life
q_transparent_trace_commit, q_transparent_query_index
q_transparent_merkle_open, q_transparent_consistency
q_transparent_extractor, q_transparent_simulator
q_trace_commitment_binding, q_trace_relation_knowledge
q_trace_proximity_bridge, q_trace_extractor_rewind
q_native_boundary, q_native_role_handle, q_native_role_auth_mode
q_native_mask_domain, q_native_no_export_policy
q_native_norm_range_policy, q_native_blinding_policy
```

and marks this symbol inactive:

```text
q_batch
```

The executable direct advantage terms are:

```text
Adv_native_boundary_zk_trace_binding_hiding(
  q_transparent_trace_commit,q_transparent_query_index,
  q_transparent_merkle_open,q_transparent_consistency,
  q_transparent_extractor,q_transparent_simulator,
  q_trace_commitment_binding,q_trace_relation_knowledge,
  q_trace_proximity_bridge,q_trace_extractor_rewind,
  q_native_boundary,q_native_role_handle,q_native_role_auth_mode,
  q_native_mask_domain,q_native_no_export_policy,
  q_native_norm_range_policy,q_native_blinding_policy)
Adv_prevalidation_leaf_binding(q_leaf,q_auth,q_evidence,q_stmt)
Adv_prevalidation_registry_binding(q_life,q_leaf)
Adv_checked_core_message_binding(q_chal,q_auth)
Adv_lifecycle_consume_once(q_life)
Adv_live_slot_leakage(q_pad,q_mask,q_leaf)
Adv_qrom_domain_separation(q_leaf,q_auth,q_evidence,q_chal,q_pad,q_mask,
  q_transparent_trace_commit,q_transparent_query_index,
  q_transparent_merkle_open,q_transparent_consistency,
  q_trace_commitment_binding,q_trace_relation_knowledge,
  q_trace_proximity_bridge,q_trace_extractor_rewind,
  q_native_boundary,q_native_role_handle,q_native_role_auth_mode,
  q_native_mask_domain,q_native_no_export_policy,
  q_native_norm_range_policy,q_native_blinding_policy)
```

Optional aggregation terms remain recorded in the object but all contain
`q_batch`, and direct-mode tests require that no direct term contains
`optional_aggregation`.

KAT `path2_direct_query_budget_keeps_batch_inactive` pins this split, including
the selected transparent payload query symbols.
KAT `path2_domain_constants_are_disjoint_and_mode_separated` pins the current
Path 2 domain/schema constants as duplicate-free, prefix-disjoint, and
direct/batch/consume separated, including the public admission-view schema and
direct query-budget schema plus selected transparent proof domains/schemas.

## Active Native Hash/QROM Review

The active native admitted carrier is now treated as a public transcript object,
not as a hidden-witness proof theorem. The executable review object is:

```text
Path2NativeHashQromReviewV1
path2_native_hash_qrom_review_v1
```

It imports the current deployment query caps from `path2_direct_query_budget_v1`
and pins:

| quantity | value |
|---|---:|
| hash function model | SHA-256 as domain-separated public RO/QROM surface |
| digest size | 256 bits |
| target security | 128 bits |
| deployment quantum hash-query cap | `2^16` |
| deployment classical hash-query cap | `2^33` |
| QROM quadratic loss at deployment cap | 32 bits |
| transcript term after QROM loss | 130 bits |
| minimum deployment margin | 1 bit |
| admitted consensus carrier size | 330-byte body, domain-prefixed canonical bytes |

The active native query symbols are:

```text
q_native_context
q_native_account
q_native_spend
q_native_lifecycle
q_native_state
q_native_transcript
q_native_boundary
q_native_checked_core_frame
q_native_consensus_carrier
q_native_preflight_report
q_native_projection_preimage_da
```

The ordinary hash/QROM assumptions are limited to public byte binding and
domain separation:

```text
public canonical byte frames are SHA-256 bound
registered domain/schema strings separate public random-oracle queries
native consensus carrier digest binds admitted q bytes
native boundary digest binds context/account/spend/lifecycle/state projection digests
native transcript digest binds the boundary digest and checked-core frame domain
projection-preimage data-availability digest binds expanded public preimage bytes
```

These do not discharge the hidden-witness proof obligations: account-witness
opening soundness, checked-core equation soundness, fiber privacy/no-export,
lifecycle state-transition semantics, and fiber-coordinate irrelevance remain
separate theorem obligations.

KAT `path2_native_hash_qrom_review_tracks_public_transcript_assumptions` pins
the review schema, deployment caps, production registry entry count, consensus
carrier byte counts, active native query symbols, and the separation between
ordinary public hash assumptions and hidden-witness obligations.

## Native-Boundary Transparent Schedule

The active account-creation proof-family assumption is:

```text
NativeBoundaryZkTraceBindingAndHiding
```

It replaces the older monolithic hidden-source term for the active path. The
older `Adv_creation_hidden_source_soundness` name remains a compatibility label
for pre-native-boundary documents, but the proof obligation now factors through
one admitted public boundary:

```text
b = B_c(h)
```

The schedule is pinned in
`research/path2_native_boundary_zk_trace_qrom_schedule.md`. Its core rule is
that no query index, challenge, mask-opening request, or consistency selector is
derived before these public values are bound:

```text
native_boundary_digest
leaf_context_digest
parent_section_root
role_profile_digest / column_profile
role_handle_table
role_authentication_mode_table
matrix_domain_transcript_digest
aggregation_context_digest
trace_shape
constraint_commitment_root
masked_query_commitment_roots
no_export_policy_id
norm_range_mode
blinding_noise_policy_id
```

The native-boundary mask-domain derivation must include:

```text
DESCENDER_PATH2_NATIVE_BOUNDARY_MASK_V1
native_boundary_digest
proof_system_id
relation_id
role_profile_digest
role_id
column_set_id
row_index
query_index
transparent_transcript_digest
```

If the full tuple is not bound before challenge derivation, the active theorem
must export `qrom_or_domain_separation_break`, `no_export_zk_break`, or
`live_slot_leakage_or_keygen_break` rather than claim hiding.

## Parent-Trace Extraction Terms

The native-boundary proof-family term must internally account for:

```text
Adv_trace_commitment_binding(q_trace_commitment_binding,
                             q_native_role_handle)
Adv_trace_relation_knowledge_soundness(q_trace_relation_knowledge,
                                       q_transparent_consistency)
Adv_trace_proximity_to_witness(q_trace_proximity_bridge)
Adv_trace_extractor_rewind(q_trace_extractor_rewind,
                           q_transparent_extractor)
```

`Adv_trace_proximity_to_witness` is present only for a proof family that uses a
proximity/list-decoding bridge. A proof family with direct full-witness
extraction may set that term inactive, but it must still account for relation
knowledge soundness and extractor rewinding/forking loss.

These terms are the internal structure of
`Adv_native_boundary_zk_trace_binding_hiding`; they are listed here so the
final theorem does not hide parent-trace extraction inside a generic
"transparent proof soundness" phrase.

## Direct-Mode Bound Shape

For direct single-slot deployment, the target proof should reduce accepted
forgery/unauthorized spend to:

```text
Adv_Path2_direct(q_*) <=
    Adv_native_boundary_zk_trace_binding_hiding(
        q_transparent_trace_commit,
        q_transparent_query_index,
        q_transparent_merkle_open,
        q_transparent_consistency,
        q_transparent_extractor,
        q_transparent_simulator,
        q_trace_commitment_binding,
        q_trace_relation_knowledge,
        q_trace_proximity_bridge,
        q_trace_extractor_rewind,
        q_native_boundary,
        q_native_role_handle,
        q_native_role_auth_mode,
        q_native_mask_domain,
        q_native_no_export_policy,
        q_native_norm_range_policy,
        q_native_blinding_policy)
  + Adv_prevalidation_leaf_binding(q_leaf, q_auth, q_evidence, q_stmt)
  + Adv_prevalidation_registry_binding(q_life, q_leaf)
  + Adv_checked_core_message_binding(q_chal, q_auth)
  + Adv_lifecycle_consume_once(q_life)
  + Adv_live_slot_leakage(q_pad, q_mask, q_leaf)
  + Adv_qrom_domain_separation(q_leaf, q_auth, q_evidence, q_chal, q_pad, q_mask,
                               q_transparent_trace_commit,
                               q_transparent_query_index,
                               q_transparent_merkle_open,
                               q_transparent_consistency,
                               q_trace_commitment_binding,
                               q_trace_relation_knowledge,
                               q_trace_proximity_bridge,
                               q_trace_extractor_rewind,
                               q_native_boundary,
                               q_native_role_handle,
                               q_native_role_auth_mode,
                               q_native_mask_domain,
                               q_native_no_export_policy,
                               q_native_norm_range_policy,
                               q_native_blinding_policy)
```

`Adv_optional_aggregation_soundness` is absent in direct mode.

## Optional Aggregation Bound Extension

When aggregation is enabled, the direct bound is extended with:

```text
  + Adv_optional_aggregation_soundness(q_batch)
  + Adv_accumulator_or_root_binding(q_batch, q_leaf)
  + Adv_batch_data_availability(q_batch)
```

The batch theorem must additionally prove that every batch-admitted leaf has the
same canonical meaning as direct admission. Otherwise a batch proof could become
a second, weaker admission verifier.

## Required Negative Controls

The KAT suite must include:

- wrong leaf domain rejects;
- wrong authorization domain rejects;
- single-slot evidence under batch domain rejects in direct mode;
- batch evidence under single-slot domain rejects in batch mode;
- leaf digest mutation rejects;
- authorization digest mutation rejects;
- transcript digest mutation rejects;
- payload digest mutation rejects;
- evidence transcript digest construction does not include `slot_leaf_digest`,
  avoiding circular leaf/transcript hashing;
- evidence transcript digest construction includes `slot_id`, avoiding
  cross-slot replay of otherwise matching evidence;
- checked-core challenge rebinding to another `slot_authorization_digest`
  rejects;
- native-boundary digest mutation rejects;
- parent-section root mutation rejects;
- role-profile / column-profile mutation rejects;
- role-handle table mutation rejects;
- role-authentication mode table mutation rejects;
- query indices derived before role handles or authentication modes are fixed
  reject;
- mask-domain derivation omitting native boundary, proof-system id, relation
  id, role profile, role id, column set, row, query, or transcript digest
  rejects;
- no-export policy id mutation rejects;
- norm-range mode mutation rejects;
- blinding/noise policy id mutation rejects;
- public full-rank linear commitments for private roles reject in the active
  native-boundary mode;
- duplicate or prefix-related Path 2 domains remain absent from the global
  domain registry.

## Current Evidence

| artifact | role |
|---|---|
| `research/prevalidated_slot_leaf_v1_frame.md` | canonical leaf and authorization digest definitions |
| `research/account_creation_evidence_v1_spec.md` | evidence transcript and payload digest roles |
| `research/path2_live_slot_leakage_ledger.md` | leakage boundary for digest-only live slots |
| `research/account_creation_aggregation_readiness_contract.md` | direct vs optional batch domain separation |
| `research/path2_native_boundary_zk_trace_qrom_schedule.md` | native-boundary transcript ordering and mask-domain rule |
| `research/path2_native_boundary_zk_trace_checklist.md` | active native-boundary proof-family gates |
| `research/path2_native_boundary_role_authentication_mode_map.md` | private/public role authentication modes and forbidden downgrade modes |
| `scripts/research/pre_signature_projection_global_domain_registry.py` | registered Path 2 planning domains and duplicate/prefix checks |
| `research/cryptanalysis/signature_readiness_20260516/pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.md` | existing mask-PRF live-slot leakage composition to reinterpret for creation time |

## Current Conclusion

Path 2 can keep a clean direct-mode theorem only if:

- `slot_authorization_digest` is the sole checked-core spend binding to the
  prevalidated slot;
- creation evidence digests are never accepted without replayable payload or the
  active `NativeBoundaryZkTraceBindingAndHiding` proof-family assumption;
- public creation payloads stay within the leakage ledger;
- private role vectors are never exported through standalone public full-rank
  linear commitments in the active mode;
- every transparent query, mask domain, role handle, role authentication mode,
  range mode, no-export policy, and blinding policy is bound before challenge
  derivation;
- direct and batch domains are rejected across modes;
- aggregation assumptions are inactive unless aggregation is enabled.

For internal direct-mode proof work, this ledger discharges the Path 2
hash/QROM accounting boundary: all active symbols, inactive optional aggregation
symbols, numeric caps, and public schema/domain separation are named and pinned.
External QROM review and any future batch-mode byte-frame KATs remain outside
the direct-mode internal closure.
