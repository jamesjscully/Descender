# Path 2 Native Boundary `H_c` / `b` Factorization Ledger

Status: first-pass boundary component ledger.

Purpose: make the hidden representative space `H_c`, safety equivalence
`~safe`, and public projection audit concrete enough to decide whether the
current Path 2 account-creation objects can factor through one native boundary
object `b = B_c(h)`.

## 1. Context Object

Let `c` be the admission context:

```text
c = (
  profile_version_digest,
  verifier_version_digest,
  account_id,
  public_key_identity,
  slot_id,
  account_state_root_before,
  lifecycle_anchor,
  aggregation_context_digest
)
```

`c` is not secret. It is the base over which hidden representatives are
compared. A representative that changes any non-aggregation field above is not
another representative in the same fiber; it belongs to another context.

Direct mode fixes `aggregation_context_digest` to the direct-mode value, but it
stays in `c` so the proof model does not block future aggregation.

## 2. Hidden Representative Space

Define `H_c` as the space of role-labeled descent representatives:

```text
H_c =
  Section_c
  x PackedResidues_c
  x PadSource_c
  x OpeningTuple_c
  x ProjectionRelation_c
  x LifecycleSlot_c
  x TraceWitness_c
  x Gauge_c
```

The components have these intended meanings:

| component | role | hidden/public status | safety status |
|---|---|---|---|
| `Section_c` | local descent section whose restrictions give the role vectors | hidden | structural |
| `PackedResidues_c` | 18 q12289 residues: 12 checked-core residues plus six masked projection residues | hidden opening, committed publicly | structural |
| `PadSource_c` | pad seed/source, pad projection lanes, mask derivation source | hidden except digest/commitment | structural |
| `OpeningTuple_c` | state-assisted opening tuple binding account witness, randomizer, restore/lifecycle anchor, pad seed, carrier handle | hidden opening, committed publicly | structural |
| `ProjectionRelation_c` | account projection lanes, pad projection lanes, carrier/check relation | hidden relation with public masked quotient | structural |
| `LifecycleSlot_c` | live marker, uniqueness namespace, consume-once authorization handle | public effect, hidden only through representative choice | safety-critical |
| `TraceWitness_c` | hidden extractor view for the transparent/two-view proof: role vectors, masks, range witnesses, account-binding witnesses | extractor-private | proof witness |
| `Gauge_c` | encodings, blinding, mask labels, and representative choices that should not affect safety behavior | hidden | quotient/gauge |

The important design choice is that `Path2CreationProofBindingV1` should not be
viewed as five independent commitments. It should be viewed as a public handle
to restrictions of `Section_c`:

```text
packed_witness_commitment        = p_packed(B_c(h))
randomizer_commitment            = p_randomizer(B_c(h))
pad_source_commitment            = p_pad(B_c(h))
opening_tuple_commitment         = p_opening(B_c(h))
projection_relation_commitment   = p_projection_relation(B_c(h))
```

If these handles remain independent, the proof pays for cross-role equality.
If they are authenticated restrictions of one descent section, consistency is
native.

## 3. Local Validity

`LocalValid_c(h)` should be stated before choosing a proof system. It includes:

1. canonical q12289 decoding for all semantic residues;
2. packed-residue reconstruction for the 32-byte account witness state;
3. pad-source derivation for masks and pad projection lanes;
4. active-lane carrier equations for account projection lanes and pad lanes;
5. checked-core residue compatibility for the 12 carried rows;
6. range/canonicality predicates for whatever digit representation is selected;
7. no postchallenge dependence on spend message, signature digest, proof
   commitment, or `slot_authorization_digest`.

This keeps validity out of `B_c`. The boundary commits to or projects a valid
object; it does not define validity by fiat.

## 4. Descent / Gluing Consistency

`DescentConsistent_c(h)` is the native consistency predicate:

```text
all role components are restrictions of one Section_c
```

Concretely:

- `PackedResidues_c` and `OpeningTuple_c` name the same account witness and
  randomizer commitments.
- `PadSource_c` opens the same pad seed named by `OpeningTuple_c`.
- `ProjectionRelation_c` uses pad lanes derived from `PadSource_c`.
- `ProjectionRelation_c` uses account projection lanes obtained by unmasking
  `PackedResidues_c` with masks derived from `PadSource_c`.
- `TraceWitness_c` opens the same role vectors that the public proof handles
  name.
- `LifecycleSlot_c` uses the same `slot_id`, account namespace, lifecycle
  anchor, and admission state root as `c`.

This is exactly the condition that should become native to the boundary
construction. The public proof should not need to establish that unrelated
commitments happened to agree.

## 5. Safety Equivalence

Define:

```text
h ~safe h'
```

iff both representatives are in the same context `c`, both are locally valid and
descent-consistent when used for admission, and they induce identical
safety-relevant public behavior:

| invariant | required equality |
|---|---|
| account namespace | same `account_id` and `public_key_identity` |
| slot identity | same `slot_id` |
| admission namespace | same `account_state_root_before` and `lifecycle_anchor` |
| opening boundary | same opening commitment / opening handle exported through `b` |
| pad boundary | same pad-source boundary exported through `b` |
| projection quotient | same public masked projection / active-lane quotient |
| checked-core authorization | same spend-facing authorization handle |
| lifecycle behavior | same live marker, uniqueness domain, and consume-once transition |
| transcript behavior | same creation statement, transparent public view, and challenge-binding projections |

Allowed differences under `~safe`:

- blinding randomness;
- one-time masks that are hidden and freshly domain-separated;
- internal encodings that decode to the same canonical residues;
- proof-system authentication paths;
- range-witness representations that prove the same canonical values;
- aggregation metadata that is explicitly outside direct-mode safety or fixed
  in `c`.

Forbidden differences under `~safe`:

- any difference that changes spend acceptance;
- any difference that changes replay/rollback/nullifier behavior;
- any difference that changes the admitted leaf authorization digest;
- any difference that lets one representative pass checked-core binding and
  another fail;
- any difference that changes public account ownership or slot identity.

The target kernel condition is:

```text
B_c(h) = B_c(h')  =>  h ~safe h'
```

or else the adversary exports `boundary_binding_break` or
`descent_soundness_break`.

## 6. Candidate Boundary Map

The minimal mathematical candidate is:

```text
Boundary_c := H_c / ~safe
B_c(h) := [h]_safe
```

The practical encoded candidate should be a canonical representative of the
same quotient:

```text
b = (
  context_projection,
  section_root_or_parent_trace_root,
  role_restriction_handles,
  public_projection_quotient,
  lifecycle_authorization_handle,
  transparent_public_view_handle
)
```

where:

```text
role_restriction_handles =
  restrictions(section_root_or_parent_trace_root, role_profile)
```

not independently sampled commitments.

## 7. Public Projection Audit

### Natural projections from `b`

These fields are natural projections if `b` is the quotient above:

| public field | source today | projection from `b` |
|---|---|---|
| `account_id` | leaf/evidence/certificate/binding | `p_context(b)` |
| `public_key_identity` | leaf/evidence/certificate/binding | `p_context(b)` |
| `slot_id` | leaf/evidence/certificate/binding | `p_context(b)` |
| `verifier_version_digest` | leaf/certificate/binding | `p_context(b)` |
| `account_state_root_before` | leaf/certificate/binding | `p_context(b)` |
| `lifecycle_anchor` | leaf/certificate/binding | `p_context(b)` |
| `live_status_marker` | leaf | `p_lifecycle(b)` |
| `aggregation_context_digest` | statement/binding/public view | `p_context(b)` |
| `masked_projection[6]` | transparent public view | `p_public_projection(b)` |
| `trace_commitment_root` | transparent public view | `p_section_root(b)` |
| `column_profile` | transparent public view | `p_role_profile(b)` |

### Projections that require a semantic decision

These fields can factor through `b`, but only if we define their meaning as
projections/restrictions of one section boundary:

| public field | current risk | required native-boundary interpretation |
|---|---|---|
| `opening_commitment` | currently a digest/handle repeated across leaf, statement, evidence, certificate, and binding | projection of `OpeningTuple_c` restriction of `Section_c` |
| `pad_seed_digest` | digest of hidden pad seed, repeated independently | projection of `PadSource_c` restriction, not independent admission evidence |
| `hidden_source_statement_digest` | digest of a statement assembled from many public/hidden handles | digest of canonical statement projected from `b` and `c` |
| `prevalidation_transcript_digest` | may bind evidence bytes rather than semantic quotient | byte-binding projection over `b` plus proof data availability |
| `creation_proof_binding_digest` | currently extra V2 leaf digest | digest of canonical encoded `B_c` role-restriction boundary |
| `packed_witness_commitment` | role handle in `Path2CreationProofBindingV1` | authenticated restriction of `section_root_or_parent_trace_root` |
| `randomizer_commitment` | role handle | authenticated restriction of `section_root_or_parent_trace_root` |
| `pad_source_commitment` | role handle | authenticated restriction of `section_root_or_parent_trace_root` |
| `opening_tuple_commitment` | role handle, currently equal to leaf opening commitment | authenticated restriction of `section_root_or_parent_trace_root`; may replace standalone opening digest semantics |
| `projection_relation_commitment` | role handle | authenticated restriction of `section_root_or_parent_trace_root` |
| `relation_transcript_digest` | certificate/evidence field | transcript projection from admitted proof over `b` |
| `transparent_transcript_digest` | transparent public view | transcript projection from public proof over `b` |

### Byte-binding fields, not semantic roots

These should remain outer consensus/data-availability binding, but should not be
treated as independent proof semantics:

| field | allowed role |
|---|---|
| `slot_leaf_digest` | hash of canonical projection frame from `b` |
| `slot_authorization_digest` | hash of admitted leaf projection; spend-facing handle |
| `evidence_payload_digest` | data-availability handle for proof payload |
| `public_replay_payload_digest` | data-availability handle for replay/proof payload |
| `payload_digest` | payload byte binding |
| `proof_bytes_digest` | proof byte binding |
| `query_openings_digest` | query-opening byte binding under no-export policy |
| `merkle_paths_digest` | authentication byte binding |
| `consistency_messages_digest` | proof-system byte binding |

If any of these are used as a substitute for `exists h. B_c(h)=b`, that path is
invalid for native consistency.

## 8. First Factorization Result

The current Path 2 design is close to factorable, but not yet native.

What already fits:

- Context fields are consistently repeated and can become projections from
  `b`.
- `Path2CreationProofBindingV1` is the right place to express role handles.
- `Path2TransparentDescentPublicViewV1` already has the correct shape for a
  public projection: public context digest, creation proof binding digest,
  masked projection, trace root, and transcript digest.
- The selected role-commitment direction,
  `role_commitments_are_trace_subcommitments`, is exactly the native-boundary
  interpretation.

What does not yet fit:

- `opening_commitment`, `pad_seed_digest`, `hidden_source_statement_digest`,
  `creation_proof_binding_digest`, and the role commitments are still modeled
  as repeated digest handles rather than as projections of one quotient object.
- The current proof story still asks the two-view theorem to prove that the
  trace root, role handles, masked projection, and leaf context are mutually
  consistent. Native `b` should make most of this consistency definitional,
  leaving the proof family to prove only valid opening of the one boundary.
- `Path2PrevalidatedSlotLeafV2` adds `creation_proof_binding_digest`, which is
  probably the right 32-byte state cost, but the theorem must say this digest is
  the boundary digest for `B_c`, not merely another independent handle.

## 9. Redesign Direction

The next design target is:

```text
Path2NativeBoundaryV1Digest := H(
  context_projection,
  parent_section_root,
  typed_role_restriction_profile,
  public_masked_projection,
  lifecycle_authorization_projection,
  aggregation_context_digest
)
```

Then:

- `creation_proof_binding_digest` should become or derive from
  `Path2NativeBoundaryV1Digest`.
- `slot_authorization_digest` should authorize a leaf that names this native
  boundary digest.
- `Path2TransparentDescentPublicViewV1.creation_proof_binding_digest` should
  bind the same native boundary digest.
- role commitments should be typed subroots/restrictions under
  `parent_section_root`.
- the extractor target becomes:

  ```text
  exists h in Fiber(B_c, b).
    LocalValid_c(h)
    DescentConsistent_c(h)
  ```

  rather than a list of cross-commitment equality obligations.

## 10. Immediate Next Proof Move

Write the `Path2NativeBoundaryV1` theorem interface:

1. exact public fields in the boundary digest;
2. exact role projections out of the boundary;
3. statement that role handles are restrictions of one parent section root;
4. kernel condition `B_c(h)=B_c(h') => h ~safe h' or boundary_binding_break`;
5. update the two-view theorem so `same_parent_context` refers to this native
   boundary instead of separately naming leaf context, creation proof binding,
   trace root, and role handles.

## Non-Claims

- This ledger does not prove the boundary binding theorem.
- This ledger does not instantiate a transparent proof family.
- This ledger does not change byte counts yet.
- This ledger does not remove the need for no-export/ZK proof of query
  openings.
- This ledger does not make `Path2CreationProofBindingV1` deployable as-is; it
  identifies the semantic reinterpretation needed for it to be deployable.
