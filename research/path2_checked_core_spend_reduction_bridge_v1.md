# Path 2 Checked-Core Spend Reduction Bridge V1

Status: first concrete bridge for `CheckedCoreSpendReduction`.

Purpose: instantiate the Lean checked-core spend-reduction hook with the active
Rust native checked-core verifier relation, and name exactly what remains a
cryptographic assumption.

## Concrete Relation

The concrete candidate for:

```text
CheckedCoreSpendReduction.checkedCoreAccepts(q,msg,state,evidence)
```

is:

```text
path2_native_checked_core_accept_v1(
  checked_core_frame =
    assemble_path2_native_boundary_v1(bundle, proof_bytes)
      .checked_core_frame,
  checked_core_verifier_handle_digest =
    bundle.spend.checked_core_verifier_handle_digest,
  checked_core_proof_bytes = proof_bytes
)
```

where `bundle` is the concrete representation of `q`, `msg` is represented by
`bundle.message_digest`, and `state` is represented by
`bundle.current_state_digest`.

The concrete evidence object is the decoded:

```text
ProductionReviewedFq12AccountCarriedProof
```

plus the assembled:

```text
Path2NativeCheckedCoreFrameV1
```

Lean currently abstracts this as:

```text
CheckedCoreSpendEvidence.proofCommitment
CheckedCoreSpendEvidence.checkedCoreFrame
```

## Field Binding

The Rust frame assembled by `assemble_path2_native_boundary_v1` contains:

```text
native_boundary_version
parameter_set_id
verifier_family_id
account_creation_domain_digest
spend_verifier_domain_digest
chain_or_application_domain_digest
fork_or_epoch_scope_digest
aggregation_mode
native_boundary_digest
p_account_digest
p_spend_digest
p_lifecycle_digest
p_transcript_digest
p_state_digest
message_digest
current_state_digest
proof_commitment
```

This instantiates the Lean ordered frame:

```text
OrderedFrameEncoder.encode [
  Proj.pSpend q,
  Proj.pLifecycle q,
  Proj.pTranscript q,
  Proj.pState q,
  msg,
  state,
  evidence.proofCommitment
]
```

with two implementation-level refinements:

1. context and account digests are also included in the Rust frame;
2. `msg` and `state` are digest-sized public fields, not raw unbounded values.

These refinements are strengthening, not weakening, provided the projection
completeness bridge identifies the context/account fields as projections of
the same concrete `q`.

## Acceptance Checks

`path2_native_checked_core_accept_v1` performs these checks:

1. `validate_path2_native_checked_core_frame_v1(checked_core_frame)`;
2. checked-core proof byte length equals
   `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES`;
3. proof bytes decode as `ProductionReviewedFq12AccountCarriedProof`;
4. decoded `proof.proof_commitment` equals
   `checked_core_frame.proof_commitment`;
5. checked-core challenge is derived from
   `Path2NativeCheckedCoreFrameV1`;
6. actual response syndrome is computed from
   `proof.masked_quotient_response`;
7. expected response syndrome is computed from:

```text
challenge,
checked_core_verifier_handle_digest,
proof.proof_commitment
```

8. actual and expected syndromes must be equal.

Therefore a successful call exports one of:

```text
valid native checked-core evidence over q,msg,state,commitment
```

or:

```text
checked_core_equation_soundness_break
checked_core_frame_binding_break
domain_binding_or_qrom_break
byte_parser_canonicality_break
```

depending on which lower-level assumption fails.

## Why This Is Not Still Opaque

Before the red-team patch, `SpendClosure` could be supplied as an uninterpreted
field. Now `SpendClosure` is derived from `CheckedCoreSpendReduction`, whose
evidence must bind the checked-core frame to projections of `q`.

The Rust bridge is concrete because:

- the frame is assembled inside `assemble_path2_native_boundary_v1`;
- callers do not provide `p_*` digest fields independently after assembly;
- proof commitment equality is checked before syndrome acceptance;
- frame mutation KATs show checked-core challenge sensitivity;
- boundary mutation KATs show public projection changes cascade into the frame
  and spend view.

## Current Evidence

Rust implementation anchors:

- `Path2NativeCheckedCoreFrameV1`
- `encode_path2_native_checked_core_frame_v1`
- `path2_native_checked_core_frame_v1_challenge`
- `path2_native_checked_core_accept_v1`
- `assemble_path2_native_boundary_v1`

Rust KAT anchors:

- `path2_native_checked_core_frame_v1_golden_vector_and_roundtrip`
- `path2_native_checked_core_frame_v1_mutations_change_challenge`
- `path2_native_checked_core_frame_v1_rejects_invalid_context`
- `path2_native_checked_core_frame_v1_excludes_legacy_private_handles`
- `path2_native_boundary_assembly_derives_one_consistent_public_surface`
- `path2_native_boundary_assembly_rejects_independent_digest_inputs`

Lean anchors:

- `CheckedCoreSpendEvidence`
- `CheckedCoreSpendReduction`
- `checked_core_reduction_implies_spend_closure`
- `public_admission_spend_action_or_break`

## Remaining Obligations

- [ ] Define the concrete `q` representation used by the bridge.
  Candidate: `Path2NativeProjectionBundleV1` plus derived
  `Path2NativeBoundaryAssemblyV1`.

- [ ] State the Rust/Lean frame correspondence:

```text
encode_path2_native_checked_core_frame_v1(assembly.checked_core_frame)
  corresponds to
CheckedCoreSpendReduction.frameEncoder.encode [...]
```

- [ ] Export the downstream checked-core cryptographic theorem:

```text
if syndrome equality accepts an invalid spend relation,
then checked_core_equation_soundness_break
```

- [ ] Export the hash/parser theorem:

```text
if two distinct frames have the same checked-core challenge,
then domain_binding_or_qrom_break or byte_parser_canonicality_break
```

- [ ] Add a one-line reference to this bridge in the residual assumption
  ledger under downstream checked-core assumptions.

## Interim Conclusion

This bridge is achievable with the current design. No new public bytes are
required, and no return to hidden-witness extraction is needed.

The only material proof burden left in this item is to make the Rust relation
definition and the Lean `CheckedCoreSpendReduction` field correspondence
explicit enough that future code changes cannot silently reintroduce an
independent checked-core handle.
