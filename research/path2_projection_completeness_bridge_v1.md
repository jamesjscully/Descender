# Path 2 Projection Completeness Bridge V1

Status: concrete bridge target for `ProjectionCompleteness`.

Purpose: instantiate the Lean projection-completeness obligation with the
active Rust native projection bundle and derived boundary assembly.

## Lean Interface

Lean states:

```text
ProjectionCompleteness(M,Proj)
```

through equalities:

```text
Proj.pSpend(q)      = M.meaningOfBoundary(q).spendAuthorizationClass
Proj.pLifecycle(q)  = M.meaningOfBoundary(q).lifecycleIdentity
Proj.pTranscript(q) = M.meaningOfBoundary(q).transcriptBindingSurface
Proj.pState(q)      = M.meaningOfBoundary(q).stateTransitionBoundary
```

This bridge gives concrete content to `q`, `Proj`, and
`meaningOfBoundary`.

## Concrete `q`

The concrete native boundary object is two-layered:

```text
q_preimage  = Path2NativeProjectionBundleV1
q_assembled = Path2NativeBoundaryAssemblyV1
```

linked by:

```text
q_assembled = assemble_path2_native_boundary_v1(q_preimage, proof_bytes)
```

The public consensus carrier is a compact view of `q_assembled` plus:

```text
projection_preimage_da_digest =
  path2_native_projection_preimage_da_digest_v1(q_preimage)
```

The expanded preimage must be available through the target-chain data
availability path when a verifier needs to recompute the compact carrier.

## Concrete Projections

### `Proj.pSpend(q)`

Concrete representation:

```text
Path2NativeSpendProjectionV1
```

Canonical byte preimage:

```text
path2_native_spend_projection_v1_bytes(bundle.spend)
```

Digest:

```text
path2_native_spend_projection_v1_digest(bundle.spend)
```

Semantic coordinates:

```text
spend_verifier_family_id
spend_authorization_class_digest
checked_core_verifier_handle_digest
public_key_identity
allowed_authorization_local_mode
allowed_state_transition_class_digest
aggregation_mode
spend_domain_separator_digest
```

Internal derivation checks:

```text
spend_authorization_class_digest
  == path2_native_spend_authorization_class_digest_v1(...)
allowed_authorization_local_mode
  == PATH2_NATIVE_AUTHORIZATION_LOCAL_MODE_DIRECT
```

Cross-projection checks:

```text
context.verifier_family_id == spend.spend_verifier_family_id
context.aggregation_mode == spend.aggregation_mode
context.spend_verifier_domain_digest == spend.spend_domain_separator_digest
account.public_key_identity == spend.public_key_identity
spend.allowed_state_transition_class_digest
  == state.consumed_successor_state_class
```

### `Proj.pLifecycle(q)`

Concrete representation:

```text
Path2NativeLifecycleProjectionV1
```

Canonical byte preimage:

```text
path2_native_lifecycle_projection_v1_bytes(bundle.lifecycle)
```

Semantic coordinates:

```text
slot_id
live_status_marker
lifecycle_anchor
nullifier_namespace_digest
consume_once_rule_digest
replay_domain_digest
restore_or_reorg_scope_digest
```

Cross-projection checks:

```text
lifecycle.live_status_marker == state.live_state_requirement
```

Lifecycle consume-once and restore/reorg enforcement are not fully discharged
by compact boundary assembly. They remain the storage/lifecycle gate.

### `Proj.pTranscript(q)`

Concrete representation:

```text
Path2NativeTranscriptProjectionV1
```

It is not caller-supplied. It is constructed during assembly from:

```text
native_boundary_digest
p_context_digest
p_account_digest
p_spend_digest
p_lifecycle_digest
p_state_digest
checked_core_frame_domain_digest
aggregation_mode
```

Canonical byte preimage:

```text
path2_native_transcript_projection_v1_bytes(assembly.transcript_projection)
```

Digest:

```text
path2_native_transcript_projection_v1_digest(assembly.transcript_projection)
```

Key bridge fact:

```text
p_transcript(q)
```

is derived only after the acyclic boundary digest is computed. It binds the
boundary digest; the boundary digest does not recursively bind transcript.

### `Proj.pState(q)`

Concrete representation:

```text
Path2NativeStateProjectionV1
```

Canonical byte preimage:

```text
path2_native_state_projection_v1_bytes(bundle.state)
```

Semantic coordinates:

```text
account_state_root_before
admitted_boundary_id
live_state_requirement
consumed_successor_state_class
data_availability_requirement
aggregation_state_boundary
```

Cross-projection checks:

```text
spend.allowed_state_transition_class_digest
  == state.consumed_successor_state_class
lifecycle.live_status_marker
  == state.live_state_requirement
```

## `meaningOfBoundary(q)`

For the active native route:

```text
meaningOfBoundary(q)
```

is the semantic tuple reconstructed from:

```text
context projection
account projection
spend projection
lifecycle projection
transcript projection
state projection
```

and the edge equalities enforced by `assemble_path2_native_boundary_v1`.

It is not an additional byte string, witness, or hidden opening.

## Public Output Derivation

The bridge requires these public outputs to be views of the same assembled
`q`:

```text
PublicNativeAccountViewV1
PublicNativeSpendViewV1
Path2NativeConsensusCarrierV1
Path2NativeConsensusCarrierPreflightReportV1
```

Current Rust anchors:

- `public_native_account_view_v1`
- `public_native_spend_view_v1`
- `path2_native_consensus_carrier_v1_from_assembly`
- `path2_native_consensus_carrier_v1_preflight`
- `path2_native_verifier_pipeline_v1`
- `path2_native_signer_public_output_v1`

Bridge rule:

```text
no validator-facing public output may supply p_spend, p_lifecycle,
p_transcript, p_state, native_boundary_digest, checked_core_frame_digest,
or projection_preimage_da_digest independently of assembly/recomputation
```

## Hash And Data Availability Conditions

Projection completeness at the byte level needs two assumptions:

1. canonical projection encoders have one parse/meaning per byte string;
2. SHA-256 digest equality binds those bytes under the registered domains, or
   exports:

```text
domain_binding_or_qrom_break
hash_binding_break
byte_data_availability_break
```

The compact carrier does not contain every projection preimage. It contains:

```text
projection_preimage_da_digest
```

so byte-level completeness also requires the DA path to provide the expanded
projection bundle to any verifier that must recompute the carrier.

## Current Evidence

Rust implementation anchors:

- `Path2NativeProjectionBundleV1`
- `Path2NativeBoundaryAssemblyV1`
- `assemble_path2_native_boundary_v1`
- all `path2_native_*_projection_v1_bytes` encoders
- all `path2_native_*_projection_v1_digest` functions
- `path2_native_projection_preimage_da_digest_v1`

Rust KAT anchors:

- `path2_native_projection_preimage_encoders_have_stable_layouts`
- `path2_native_projection_mutations_cascade_to_transcript_and_boundary`
- `path2_native_projection_preimages_exclude_private_sentinels`
- `path2_native_boundary_assembly_derives_one_consistent_public_surface`
- `path2_native_boundary_assembly_rejects_independent_digest_inputs`
- carrier/preflight recomputation KATs

Lean anchors:

- `ProjectionCompleteness`
- `SpendActionMatchesMeaning`
- `spend_action_matches_meaning_of_projection_complete`
- `public_admission_spend_action_or_break`

## Remaining Obligations

- [ ] Add a short formal note that `q` in Lean corresponds to the pair:

```text
(Path2NativeProjectionBundleV1, Path2NativeBoundaryAssemblyV1)
```

subject to successful assembly.

- [ ] Explicitly classify `message_digest`, `current_state_digest`, and
  `proof_commitment` as spend-instance inputs, not account meaning fixed at
  account creation.

- [ ] Link the storage/lifecycle hardening gate to the `p_lifecycle` and
  `p_state` coordinates that compact assembly only checks syntactically.

- [ ] Link ordered encoder/hash-binding assumptions to each digest equality.

## Interim Conclusion

Projection completeness is plausible with the current design and no extra
public bytes.

The key move is treating the concrete boundary as:

```text
projection preimages + deterministic assembled public surface
```

rather than as the 330-byte compact carrier alone. The carrier is a consensus
view of `q`, not the whole proof object needed for recomputation.
