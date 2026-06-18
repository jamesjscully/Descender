# Path 2 Native Boundary Relation Knowledge-Soundness Contract

Status: relation-knowledge contract pinned; proof not supplied.

Purpose: separate "the extractor obtains a full trace" from "the extracted
trace satisfies the native-boundary relation."

Both are required. A full trace that is not known to satisfy the relation is not
enough for account creation.

## Relation Knowledge Modes

The Lean profile now distinguishes:

```text
RelationKnowledgeMode =
  sampledRowChecksOnly
  | proximityWithoutWitness
  | proximityWithWitnessBridge
  | directRelationKnowledge
  | fullWitnessRelationKnowledge
```

The active theorem requires:

```text
UsesSelectedTraceKnowledgeModes(profile)
```

which pins the active profile to:

- `TraceExtractionMode.fullTraceExtractable`;
- `RelationKnowledgeMode.fullWitnessRelationKnowledge`;
- full `TraceRelationCoversNativeBoundary` coverage;
- `TraceRelationMeetsSelectedNativeBoundaryRowFloor`, with selected floor
  `176` rows;
- `TraceRelationMeetsSelectedNativeBoundaryClassFloors`, with conservative
  per-class floors for q-field, range, mask-consistency, role-restriction,
  opening-tuple, checked-core, lifecycle/context, and bookkeeping rows.

The broader helper predicate `SupportsRelationKnowledge(profile)` accepts:

- `proximityWithWitnessBridge`;
- `directRelationKnowledge`;
- `fullWitnessRelationKnowledge`.

It rejects:

- `sampledRowChecksOnly`;
- `proximityWithoutWitness`.

It also now requires the profile's `TraceRelationProfile` to cover every
mandatory native-boundary constraint class:

```text
qFieldRow
base128RangeRow
maskConsistencyRow
roleRestrictionRow
openingTupleAccountBindingRow
checkedCoreResidueCompatibilityRow
lifecycleContextRow
proofSystemBookkeepingRow
```

This prevents relation-knowledge soundness from being claimed for only the old
66-row semantic core or for a relation profile that omits masked-opening,
role-restriction, opening-tuple account-binding, checked-core compatibility,
lifecycle/context, or transcript/bookkeeping obligations.

## Contract

For an accepted proof with a full extracted trace:

```text
TransparentAccepts(b, view)
TransparentViewProjectsFrom(b, view)
ColumnProfileCompatibleWithBoundary(b, view, profile)
UsesSelectedTraceKnowledgeModes(profile)
```

the proof family must establish:

```text
TraceRelationValid(b, profile, trace)
```

for the extracted `trace`, not for a post-hoc witness and not only for sampled
rows.

The parent extraction theorem now also requires:

```text
MaskedOpeningsConsistent(b, view, profile, trace)
```

before relation validity closes. This is intentionally separate from
`TraceRelationValid`: the public masked/simulated openings must be tied to the
same hidden extracted cells and mask witnesses used by the relation, so a proof
cannot pair a simulator-safe public view with an unrelated relation witness.

2026-06-11 implementation note: the Rust transparent-opening verifier now
enforces the sampled-row part of this consistency condition per lane. The same
opened `account_share` cell must satisfy both
`account_share + mask_share = masked_projection` and
`account_share = lo_share + 128*hi_share`; the regression
`path2_transparent_zk_opened_row_relation_prevents_split_witness_cancellation`
constructs a row where the old fixed linear aggregate residual cancels to zero
and confirms the explicit digit check rejects it. This is necessary for
`MaskedOpeningsConsistent`, but it is not sufficient for the full contract above:
the remaining theorem still has to extract a full trace and prove
`TraceRelationValid` for that extracted trace, or export a named break.

2026-06-11 negative result: the current Rust opening core does not yet meet that
full contract. `redteam_transparent_zk_core_accepts_public_only_synthetic_trace`
constructs an accepting transcript from public `masked_projection` alone. This
shows the current relation does not force C1/C2/C4/C6 (pad-seed binding, PRF
mask derivation, packed-witness repack equality, and role-commitment
consistency). The next proof move is therefore a relation redesign or proof
family replacement, not hidden range extraction.

2026-06-12 strengthening: `research/path2_current_core_negative_theorem.md`
states the separation as a negative theorem for the current executable core.
The regression
`redteam_transparent_zk_core_language_exceeds_hidden_opening_language` constructs
an accepting core transcript for arbitrary public context digests and arbitrary
masked lanes without any pad seed, packed-witness commitment opening, PRF mask,
or role witness. This proves the current core accepted language is strictly
larger than the hidden-opening language; no extractor theorem can be sound for
this core without adding a full-relation subproof or replacing the proof family.
Lean records the minimal row-equation version as
`current_masked_share_core_acceptance_does_not_imply_full_hidden_opening`: the
current sampled row relation has a public-only satisfying assignment and does
not logically imply the missing full-hidden-opening predicate.
`research/path2_current_view_binding_negative_theorem.md` records the companion
composition failure: preserving `creation_proof_binding_digest` in the
transcript does not make the synthetic trace open that binding's role
commitments. Lean records the minimal co-location version as
`digest_colocation_does_not_imply_trace_role_binding` and packages both
current-route negatives as `current_transparent_route_blocker1_negative_witness`.

2026-06-12 public bridge progress: the verifier now checks the public
masked-projection consequences of the binding before the backend gate. The
packed witness commitment must unpack to the same masked projection carried by
the public view, and the pad-source role commitment must equal the canonical
commitment to that same public masked projection. This removes a concrete
rebinding surface between the public view and two binding commitments, but it is
only the public edge of `MaskedOpeningsConsistent`; the private pad seed, PRF
masks, account projection, packed-witness repack, and role openings still need a
full relation-knowledge backend.

The Rust proof-family metadata now reflects that ordering. A canonical
`Path2TransparentFullHiddenRelationProofContractV1` is bound into the production
obligations and proof-family capability; it lists the required hidden-relation
bindings and remains uninstantiated. That contract also binds
`Path2TransparentCurrentCoreNegativeTargetV1`, which names the Lean/Rust
counterexamples showing the current sampled core is insufficient, and
`Path2TransparentFullRelationBackendSufficiencyTargetV1`, which names the Lean
certificate/theorem pair whose instantiation would imply
`AccountAdmissionSoundness`. This prevents a future implementation from claiming
`full_trace_extractable` or full witness-relation knowledge unless the contract,
hidden-relation plan, negative target, positive theorem target, public
certificate proof, backend certificate, and extractor discharge move together.

The narrow replacement obligation is `Path2TransparentTraceToBindingRelationLayerV1`.
It is the verifier-visible relation that the current core lacks: the committed
trace roots must be proven to bind the same hidden opening as the accepted leaf,
creation binding, native role root, PRF masks, and public masked projection, and
the proof must remain zero-knowledge. Its digest is now carried directly by the
full-hidden-relation proof contract, public-input frame, production obligations,
and capability. The canonical layer separately marks the reference bridge
implemented only after local trace-binding check records, full-hidden suite
binding, the nonzero local-bridge digest check, the direct statement bridge, and
the reference certificate bridge are all checked. It still requires both the
hidden-relation constraint plan and the public full-relation certificate
proof-verifier bit before it can mark the full trace-to-binding relation
implemented, so the route stays fail-closed rather than claiming relation
knowledge from public root co-location.

The executable trace commitments now bind to the public statement context:
trace and constraint Merkle leaves include a `TraceBindingContext` digest over
the proof-system/relation identifiers, leaf context, creation-binding digest,
slot, hidden-source statement digest, matrix transcript, masked projection,
aggregation context, schedule, and query index. This prevents trace roots from
being anonymous commitments independent of the binding context. It deliberately
does not claim C1/C2/C4/C6 relation
knowledge; public-only synthetic traces remain possible for freshly chosen
public contexts, and that is why the full-hidden-relation subproof remains the
gate for account-admission soundness.

The public statement is now canonical too:
`Path2TransparentFullHiddenRelationPublicInputsV1` binds the full-relation
contract digest and trace-to-binding layer digest to the leaf context, creation
binding, slot, hidden-source statement digest, public masked lanes, role
commitments, trace roots, and query schedule. `Path2TransparentDescentPublicViewV1`
also carries the slot and hidden-source statement digest directly, and its
transcript/public-view digest hashes those anchors. The proof-bytes envelope
constructor and verifier compare the envelope slot and hidden-source statement
digest against the public view before the stricter full-hidden subproof shape
check, so an outer proof body cannot be rebound to a different source context.
The named full-hidden proof-suite binding verifier also compares the outer
proof-byte envelope's copied full-hidden subproof digest, local trace-binding
proof digest, hidden/trace witness-check digests, and direct statement-bridge
digest against the embedded full-hidden subproof envelope before reaching the
backend gate, so a caller cannot use that public entry point with stale copied
handles.
The proof-bytes verifier constructs this frame, runs the current incomplete
Merkle/FRI/row core, and
then calls `verify_path2_transparent_full_hidden_relation_subproof_v1`, so the
future backend has an exact statement digest to verify. The current proof suite
already carries a typed full-relation subproof envelope; because the backend is
uninstantiated, the envelope carries the typed statement bytes, requires the
local trace-binding proof digest field to be zero, and then reaches the
fail-closed sentinel. The disabled route can also wrap a typed full-hidden proof
payload that has been checked up to the backend-unimplemented gate; in that
case the envelope carries the payload's nonzero local trace-binding proof
digest and rejects any mismatch. That digest is derived from the backend
verifier result and includes the transparent transcript digest plus trace
schedule fields; the backend verifier receipt repeats those fields too, so the
local bridge identity is pinned to the sampled trace domain as well as to the
public view, binding context, and trace roots.
The full-hidden proof payload and subproof envelope now also repeat the
transparent transcript digest and trace schedule fields directly from the
statement/public inputs, so the final proof carrier no longer relies only on
`public_view_digest` to expose that schedule/transcript binding.
They also carry the slot and hidden-source statement digest from the same
statement/public-input context, matching the backend verifier result, verifier
receipt, and certificate payload.
The backend verifier receipt, full-hidden proof payload, and proof-carrying
outer subproof envelope also repeat the hidden-relation and trace-origin
witness-check receipt digests; bundle, proof-payload, and envelope validation
compare them against the embedded certificate/payload boundaries before the
backend acceptance gate.
The hidden-relation and trace-origin witness-check records now also repeat the
pre-proof `direct_payload_statement_bridge_digest`, full-hidden proof contract
digest, and hidden-relation constraint-plan digest, so local trace-root
recomputation is pinned to the same statement bridge and contract/plan context
before it can feed the local trace-binding proof.
The local trace-binding proof and backend verifier result also repeat and hash
that bridge digest and contract/plan pair directly, so the bridge digest cannot
be reused as a reduced handle under a neighboring direct-payload statement,
relation contract, or hidden-relation plan.
The standalone digest validators for the hidden witness-check record,
trace-origin witness-check record, full-hidden subproof statement, local
trace-binding proof, backend verifier result, backend verifier receipt,
certificate payload, and final proof payload now also reject the wrong
transparent proof-system id, rather than merely hashing a nonzero id. This
keeps a forged bridge from being re-digested under a different proof family.
The certificate payload, final full-hidden proof payload, and outer subproof
envelope now repeat and hash the same contract/plan pair too; validation
compares it against the statement, embedded certificate, receipt, and envelope
boundaries before any backend acceptance gate can be reached.
The local trace-binding proof digest is not evidence by itself: the backend
verifier result and certificate payload reject a nonzero local bridge digest
unless both trace-root-to-hidden-witness verification flags are set, and the
two trace/constraint-root binding verification flags are all-or-nothing rather
than a partial bridge state.
The certificate bundle rejects any verified witness-check claim unless the serialized
hidden-relation and trace-origin witness-check records are carried and decode
back to the same local bridge digest. Rust keeps the digest-only local proof
validator as a structural check, but bridge evidence must use the
check-record-backed local proof verifier, which reconstructs the proof from the
statement plus the hidden-relation and trace-origin check records. The helper
that derives a local proof from a backend verifier result is kept internal for
digest recomputation and is not a public bridge-evidence constructor.
The raw backend verifier-result constructor/digest helper and backend
verifier-receipt constructor/digest/byte decoder/encoder are likewise kept
crate-local; public bridge evidence enters through the check-carrying
certificate payload/bundle constructors or the proof-carrying acceptance
verifiers, not through digest-only backend receipt material.
The backend verifier receipt additionally repeats the backend statement digest,
public-input digest, verifier-key digest, backend proof-system id, backend
acceptance-verifier digest, backend obligation-set digest, full-hidden proof
contract digest, hidden-relation constraint-plan digest, and local trace-binding
proof digest, trace commitment root, and constraint commitment root, plus the
trace-to-binding layer digest and leaf/matrix/aggregation source anchors, so the
typed receipt cannot be rebound to a neighboring backend statement, bridge
context, or trace source. The top transparent proof-bytes envelope now also
repeats the pre-proof direct-payload statement bridge digest carried by the
full-hidden subproof envelope, with its wire version bumped for the added field,
so the direct-creation proof-bytes boundary checks the bridge before the
disabled backend gate is reached.
identity, committed trace root, relation contract, or trace-source context while
preserving only the opaque verifier-result digest.
The hidden-relation and trace-origin witness-check receipts now repeat those
same public source anchors and trace roots directly before certificate assembly,
so the local bridge is checked at the witness-check layer as well as at the
backend receipt layer.
The full-hidden certificate bundle carries the local trace-binding proof digest
in its header and carries those witness-check receipts as canonical bytes, not
merely as digest handles: checked bundle validation decodes the records,
compares their digests and local bridge digest against the certificate and
receipt, checks the trace-origin-to-hidden-check link, and recomputes the local
trace-binding proof digest. A certificate/receipt pair that claims verified
checks but omits the carried records is rejected. A forged outer subproof
envelope with a repaired
payload digest but tampered carried check bytes is rejected by the public-envelope
shape verifier before the disabled backend can be considered.
The final full-hidden proof payload repeats those same source anchors and roots
before shape validation accepts the embedded certificate bundle, closing the
last payload-level place where only reduced context fields were previously
visible.
The outer subproof envelope now repeats the same anchors too, so the verifier's
first decoded full-hidden carrier is no longer a reduced-context wrapper around
the stronger inner payload. The public envelope-shape check rejects a zero
hidden-source statement digest and compares the envelope slot/source pair
against a decoded statement-only or typed proof-payload carrier. The
proof-carrying transparent descent boundary uses
the stricter proof-suite envelope check, which requires the full-hidden subproof
segment, a nonzero local trace-binding proof digest, nonzero hidden/trace
witness-check digests, a top-level proof-bytes envelope digest of that exact
full-hidden subproof segment plus the same decoded local bridge/check digests,
and a top-level transparent-descent payload digest that also names the same
full-hidden subproof, local bridge, and hidden/trace check digests before
running the sampled masked-share core. A statement-only full-hidden
envelope is therefore not a proof-carrying bridge segment. The standalone public
opening verifier and named proof-suite binding verifier enforce the same nonzero
local bridge requirement. The trace-to-binding layer, production obligations,
and proof-family capability records also digest-bind an explicit
`reference_hidden_relation_checks_implemented` flag from the hidden-relation
constraint plan, an explicit `full_hidden_relation_local_bridge_digest_checked`
flag, an explicit `direct_payload_statement_bridge_checked` flag for the
pre-proof statement bridge, explicit endpoint-split flags requiring the
public-view-only verifier and backend certificate payload-only verifier to stay
rejection-only while requiring the proof-bytes account-creation boundary and
backend certificate proof-bundle endpoint, plus an explicit
`full_hidden_relation_reference_certificate_bridge_verified` flag for this
local-shape requirement. The hidden-relation plan separately records the
reference checker's pad-seed binding, PRF-mask derivation,
packed-witness-repack, and role-commitment consistency checks as true while
leaving the production/backend constraint bits false. They additionally digest-bind
`reference_trace_to_binding_bridge_implemented = true` as the narrow
local/reference bridge status. They also digest-bind
`full_hidden_relation_public_certificate_proof_verified = false`, so a checked
reference bundle cannot be confused with public proof verification.
The proof-carrying backend certificate path now validates this local bridge shape
only from the full certificate proof bundle, not from receipt bytes alone. It
requires the bundle's embedded certificate payload to match the caller payload,
then checks the carried hidden-relation and trace-origin records, matching
receipt, and recomputed nonzero local trace-binding proof digest. The public
certificate payload/proof-bundle constructors are the check-carrying
`*_from_checks_v1` APIs; raw digest-only constructors are crate-local test
fixtures and are not exported as bridge evidence. If the certificate payload
does not mark witness checks verified, the proof bundle must not carry witness
check bytes at all, so ignored payload bytes cannot become a no-export side
channel. The structural single-record
hidden-relation and trace-origin witness-check verifiers, raw serializers,
proof-shaped decoders, proof-shaped statement serializers, proof-shaped struct
digest validators, full-hidden public-input/subproof component/verifier-key/
backend-proof/proof-body digest helpers, and transparent direct-statement/
proof-byte/payload/proof-family binding digest helpers are crate-local too;
public bridge evidence must use the paired statement/check path that
reconstructs the local trace-binding proof. The public
surface is source-pinned by
`path2_transparent_bridge_public_surface_forbids_raw_proof_shape_helpers`,
which rejects re-exporting those raw helpers while requiring the proof-carrying
and fail-closed public bridge APIs to remain available.
The shape-only certificate payload validator is also crate-local; public
certificate checking is either the always-disabled payload-only verifier or the
proof-carrying verifier. The public certificate proof verifier and public
subproof-envelope verifier still reject the disabled-backend reference bundle
with the unimplemented-backend error. The backend-gated helper may inspect the
local bridge shape for construction and reference tests, but that does not claim
the full backend/extractor obligations are discharged.
The standalone public opening verifier remains
fail-closed unless the aggregate production-soundness obligations are discharged;
flipping only the full-hidden constraint-plan bit is not an acceptance condition.
The top-level transparent proof payload and typed proof-bytes envelope likewise
repeat the leaf/matrix/aggregation source anchors, slot, hidden-source statement
digest, trace and constraint commitment roots, transparent transcript digest,
trace schedule fields, full-hidden subproof digest, local trace-binding proof
digest, and hidden/trace witness-check digests next to the public-view/context
anchors; the transparent payload digest covers those bridge handles and the
proof-bytes boundary compares them against the decoded envelope.
Transparent descent now uses a two-stage direct-payload bridge. The
full-hidden public inputs carry a `direct_payload_statement_bridge_digest`
computed before proof bytes exist from the certificate relation transcript,
creation binding, public view, trace roots, transcript, schedule, and trace
binding context. The full-hidden public-input digest, subproof statement,
backend result/receipt, certificate payload, final proof payload, and subproof
envelope all carry and hash that pre-proof bridge. This intentionally does not
place the final replay payload digest or proof-family digest inside the
subproof, because those final digests are computed from proof bytes that contain
the subproof and would otherwise create a self-referential fixed point.
The direct creation proof payload also carries a proof-family public binding
digest; for transparent descent this post-proof digest hashes the transparent
public view, the pre-proof statement bridge, payload/proof-bytes digest,
full-hidden subproof digest, local trace-binding proof digest, and hidden/trace
witness-check digests, while non-transparent proof kinds must carry zero. The
selected-view bridge rejects stale statement-bridge and proof-family binding
digests before the backend gate.
The public
suite-shape verifier and named full-hidden binding verifier compare the
proof-bytes envelope slot/source pair against the decoded inner full-hidden
subproof envelope before the backend gate, so the outer trace wrapper cannot
target a neighboring account-opening context while preserving only the inner
full-hidden payload. The full-hidden public-input digest also
recomputes the native role-root commitment from the carried packed-witness,
randomizer, pad-source, opening-tuple, projection-relation, and gauge
commitments, so a decoded public-input object cannot preserve the public-view
role root while swapping one of the role commitments. The query-opening
manifest carries the selected public-view/context anchors, `query_count`, and
per-query records, so manifest validation remains tied to the same view even
though the manifest itself does not repeat the transcript digest.
The canonical full-hidden public-input constructor also checks the creation
binding's account, slot, verifier, state-root, lifecycle, opening-tuple, and
creation-statement fields against the leaf before it emits the public-input
digest, so the binding digest cannot be rebound to a neighboring creation
statement while the full-hidden inputs keep the old leaf statement digest.
The hidden-relation witness-check boundary repeats the slot and hidden-source
statement comparison against the creation binding, so decoded public inputs with
a rebound slot or statement digest fail before any trace-origin receipt can be
minted. The hidden-relation and trace-origin witness-check receipts also carry
and hash that slot and hidden-source statement digest, and local bridge/certificate
assembly rejects mixed receipts whose carried slot or statement context differs.
The full-hidden statement, backend verifier result, verifier receipt,
certificate payload, proof payload, and subproof envelope carry and hash the
same slot and hidden-source statement digest, so the bridge no longer hides
that account-opening context solely inside the public-input digest.
The hidden-relation and trace-origin witness-check receipts also repeat the
transparent transcript digest and trace schedule fields, and certificate
assembly compares those fields against the statement before setting the local
trace/constraint-root bridge verified bits.

Positive sufficiency is now formally named in Lean:
`research/path2_full_relation_backend_sufficiency_theorem.md` records
`SelectedTransparentFullRelationBackendCertificate` and the theorem that such a
certificate implies `AccountAdmissionSoundness`. The current Rust contract is
the byte-level placeholder for that certificate, not an implementation of it;
the sufficiency-target digest records `concrete_backend_certificate_instantiated
= false` and rejects premature positive claims.

## Required Relation Content

`TraceRelationValid(b, profile, trace)` must cover:

1. public context columns match `p_context(b)`;
2. parent root/profile/role handles match `p_section_root(b)`,
   `p_role_profile(b)`, and `p_role_handles(b)`;
3. base-128 hidden-role digits reconstruct canonical q12289 semantic values;
4. packed residue coordinates reconstruct the account witness state;
5. pad source derives pad lanes and mask domains;
6. projection relation links account projection, pad projection, and
   `p_public_projection(b)`;
7. opening tuple binds account witness, randomizer, lifecycle anchor, and
   spend-opening coordinates;
8. range witnesses satisfy `hybridBase128HiddenRoles`;
9. blinding/noise auxiliaries satisfy extractor-side bounds;
10. no relation column depends on post-admission spend challenge material.

## STARK-Like Theorem Shape

A STARK-like route is acceptable only with a theorem of this shape:

```text
AcceptSTARK(public_inputs, commitments, openings, fri_proof)
  =>
exists trace.
  Opens(parent_root, trace)
  LowDegreeOrValidEncoding(trace)
  ConstraintSystemSatisfied(trace, public_inputs)
  NativeBoundaryRelation(trace, b, profile)
```

or a named break.

The theorem must not stop at:

```text
sampled rows satisfy queried constraints
```

unless it also proves that sampled acceptance implies a reconstructed witness
trace satisfying the relation except with the claimed soundness error.

## Proximity Bridge

If a future profile uses proximity testing, the bridge must show:

```text
CommittedOracle close to low_degree_oracle
Constraint checks accept
  =>
exists trace.
  TraceRelationValid(b, profile, trace)
```

The bridge must also show that the reconstructed trace opens the same parent
root/role handles or exports `inner_commitment_binding_break` /
`transparentProofSoundnessBreak`.

Bare proximity to some low-degree object is insufficient if that object is not
bound to the role-complete native-boundary trace.

## Failure Modes

Reject or export a named break if:

- constraints are checked only on sampled rows;
- a reconstructed low-degree object is not tied to `p_section_root(b)`;
- role subtraces are not tied to `p_role_handles(b)`;
- the relation omits hidden range/digit reconstruction;
- masked public openings are not linked to hidden unmasked cells;
- the trace can satisfy public checks while omitting a mandatory private role;
- the proof family cannot state a concrete soundness error for relation
  knowledge.

## Current Assessment

This is the remaining nontrivial cryptographic assumption for the active
transparent route. It is plausible as a named STARK/IOP-style proof-family
assumption, but it is not something the native-boundary design itself proves.

The design work has reduced the problem to a clean yes/no:

```text
Can the chosen proof family provide relation knowledge soundness for the
role-complete native-boundary trace without raw hidden openings?
```

If yes, continue Path 2. If no, switch to MPC-in-the-head or the parked
lattice-sigma fallback.

Relation inventory:
`research/path2_native_boundary_trace_relation_v1_inventory.md` defines
`NativeBoundaryTraceRelationV1` as the active relation object. It shows the old
66-row core is now only a comparator; the masked/native-boundary lower bound is
at least 124 rows before role, lifecycle, account-binding, checked-core, and
bookkeeping rows.
`research/path2_native_boundary_trace_relation_row_symbols.md` pins symbolic
row variables for the remaining open groups.
`research/path2_native_boundary_open_row_estimates.md` gives a first
conservative range of 160-220 rows, with working estimate 176, assuming SHA
digest and transcript schedule checks remain outside the hidden relation.

## Lean Anchors

`Descender/Path2NativeBoundary.lean` now defines:

- `RelationKnowledgeMode`
- `SupportsRelationKnowledge`
- `TraceRelationProfile`
- `TraceRelationCoversNativeBoundary`
- `TraceRelationMeetsSelectedNativeBoundaryRowFloor`
- `TraceRelationMeetsSelectedNativeBoundaryClassFloors`

and `ParentTraceExtractionSoundness` requires both:

```text
UsesSelectedTraceKnowledgeModes(profile)
```

## Non-Claims

- This does not prove STARK/IOP soundness.
- This does not choose a concrete AIR/constraint system.
- This does not update byte counts.
