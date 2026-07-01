# Path 2 Fiber Privacy / No-Export V1

Status: first no-export theorem target for the active native account model.

Purpose: prove or specify that the private fiber:

```text
H_c(q) =
  LiftCore_c(q)
  × AuthorizationLocal_c(q)
  × Gauge_c(q)
  × Randomizer_c(q)
```

is not exported by public account creation, spend verification, checked-core
frame construction, helper APIs, or reject/error paths.

## Target Claim

For the compact native path:

```text
PublicView_c(q,sig,msg,state)
```

is simulatable from:

```text
q
p_context(q)
p_account(q)
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
msg
state
signature/proof bytes
allowed public metadata
```

without access to:

```text
authorizationLocal
gauge
randomizer
raw liftCore representative coordinates
```

or the adversary exports:

```text
fiber_privacy_break
authorization_local_hiding_break
gauge_invariance_break
checked_core_frame_binding_break
byte_data_availability_break
```

## Allowed Public Exports

Public account creation may export:

- canonical `q` bytes or digest/data-availability handle for `q`;
- projection digests/handles for `p_context`, `p_account`, `p_spend`,
  `p_lifecycle`, `p_transcript`, and `p_state`;
- obstruction flags or public edge-check certificates;
- public lifecycle/state markers;
- public spend signature bytes;
- checked-core proof commitment and responses, if their verifier frame is
  bound to `q`;
- rejection code names that do not include private witness values.

## Forbidden Exports

Never export:

- private authorization-local opening/signing witness;
- private trapdoor material;
- raw pad seed or pad-source coordinates;
- opening tuple witness scalars;
- unmasked projection lanes;
- gauge/trivialization coordinates;
- randomizer/blinding/noise values;
- raw lift-core representative coordinates unless explicitly public under
  `q`;
- reject-path diagnostics containing any of the above;
- helper API outputs returning any of the above.

## Component-Level Privacy

### AuthorizationLocal

Publicly allowed:

```text
AuthClass_c(auth)=p_spend(q)
```

Forbidden:

```text
auth
opening/auth witness coordinates
trapdoor material
```

The spend verifier may learn only that a signature/proof accepted under:

```text
p_spend(q)
```

not which private representative produced it.

### Gauge

Gauge may affect representation but not meaning.

Required:

```text
gauge_valid_c(q,gauge)
  =>
meaning_c(gauge.h)=meaning_of_boundary_c(q)
```

No public byte should identify the gauge choice unless the gauge is already a
public canonical convention.

If canonicalization selects one representative and that representative reveals
private structure, canonicalization is not safe; add randomized gauge or move
only the public quotient into `q`.

### Randomizer

Randomizer is hiding material.

Required:

```text
randomizer_valid_c(q,randomizer)
  =>
public semantics unchanged
```

and:

```text
PublicView(real randomizer)
  indistinguishable from
PublicView(simulated randomizer)
```

or a named sampler/hiding break.

### LiftCore

LiftCore is the only ambiguous component. If it is fully determined by `q`, it
is not private. If it contains local representative choices, those choices must
be treated as gauge/private and not exported.

Rule:

```text
public liftCore fields = projections of q
private liftCore fields = forbidden exports
```

## Checked-Core Interaction

Checked-core frame construction must not include private fiber coordinates.

Allowed frame:

```text
frame(
  p_spend(q),
  p_lifecycle(q),
  p_transcript(q),
  p_state(q),
  msg,
  state,
  p_context(q),
  proof_commitment
)
```

Forbidden frame:

```text
frame(..., opening_commitment, pad_seed_digest, raw projection lanes, ...)
```

unless each handle is only a byte binding for a projection of `q`.

Checked-core response bytes may be public signature/proof evidence, but they
must be covered by the checked-core leakage theorem or sampler theorem. They
must not be reused as account-creation evidence for hidden-source opening.

## Error Path Rule

Reject paths must be fail-closed:

```text
Reject(q,payload) returns only:
  reject_code
  public field name
  public byte offset
  named break class
```

and never:

```text
expected private value
decoded private lane
partial opening
raw mask
seed material
```

This applies equally to debugging helpers, tests, JSON serializers, and
negative-control fixtures.

## Account-Creation Boundary Report

The active account-creation proof boundary imports this no-export target
through:

```text
Path2AccountCreationNoExportBoundaryReportV1
formal/kat/path2_account_creation_no_export_boundary_v1.json
```

The report pins `zero_knowledge_no_export_semantics` for
`Path2AccountStarkCreationPayloadV1` and names
`account_stark_simulator_or_no_export_break` if the selected payload simulator
or no-export theorem fails.

The current forbidden private-material field denylist is:

```text
pad_source
projection
syndrome
opening_tuple
witness
mask
private_trace_material
```

The current selected-payload simulator theorem targets are:

```text
selected_account_stark_public_bundle_simulator_v1
selected_account_stark_no_export_v1
```

The accepted public bundle simulator may consume only this
`simulator_public_input_fields` public-input allowlist:

```text
leaf_context_digest
creation_proof_binding_digest
native_role_root_commitment
aggregation_context_digest
slot_id
masked_projection
account_stark_bundle_bytes_or_digest
public_replay_payload_digest
deployed_account_id
deployed_account_state_root
deployed_lifecycle_anchor
deployed_verifier_version
deployed_packed_witness_commitment
public_rejection_code
```

This report is a boundary and fail-closed regression anchor, not simulator
closure. It records that the public payload byte model exports no private
material and that current public mutation coverage is wired. The open blocker is
simulator semantics, no-export theorem closure, external review, and deployment
readiness; the no-export audit vector corpus is frozen by the digest-pinned
current vectors. The denylist term
`private_trace_material` covers private trace material.

The algebra-layer simulator/no-export theorem for the selected payload is now
stated and proven in
`research/path2_account_stark_simulator_no_export_theorem_v1.md` (`S(pub) ≈_c
Real` from the allowlist alone, reducing no-export to {zk-FRI HVZK, Poseidon2
mask-PRF, Poseidon2 commitment hiding}). It does NOT close the boundary report:
the implementation no-export scan, pad-seed single-use lifecycle term
(`DIR-LIFE-001`), the Lean public-view predicate, and external review remain the
open blockers. The frozen no-export vector flag is closed; simulator,
no-export, accepted-bundle simulability, review, and deployment flags stay
false.

## Helper API Rule

Public helper APIs may expose only:

```text
PublicView_c(q)
ProjectionDigest_i(q)
CanonicalFrame_c(q,msg,state,commitment)
NamedBreak
```

Private helper APIs may expose signer state only behind explicit signer-local
interfaces and must not be used by verifier or serializer code.

Any public helper that returns authorization-local, gauge, randomizer, or raw
hidden-lift material is a deployability blocker.

## Theorem Shape

```text
fiber_privacy_no_export_v1:
  obs_c(q)=0
  ∧ LocalValid_c(q,h)
  ∧ PublicViewProduced(c,q,h,sig,msg,state,view)
    =>
  SimulatableFromPublic(q,sig,msg,state,view)
```

or:

```text
fiber_privacy_break
authorization_local_hiding_break
gauge_invariance_break
checked_core_frame_binding_break
```

## Practical Test Surface

Before deployability, add or preserve tests that scan all public outputs for:

- raw seed bytes;
- raw opening tuple bytes;
- raw projection lane bytes;
- raw pad/mask bytes;
- raw randomizer bytes;
- private helper return values;
- reject diagnostics with private expected/actual values.

This is an implementation verification task, but the proof model should name it
now so we do not accidentally declare the theorem closed from algebra alone.

## Current Assessment

The active native path has a good privacy shape because public account creation
uses `q` and public obstruction checks, not a transparent hidden-witness proof.

The remaining danger is accidental export:

- old helper APIs from the 308b/fallback path;
- reject diagnostics;
- checked-core frame drift;
- public query openings if any fallback proof route is reintroduced.

These are controllable, but they must remain first-class obligations.

## Next Concrete Move

Update the Lean interface with a public-view/no-export predicate for the active
native account design, parallel to the older native-boundary no-export interface.

Then update the tracker so the remaining proof blockers are:

1. public-view no-export/simulation;
2. spend closure final composition;
3. byte/state model refresh after the added `p_spend` fields.
