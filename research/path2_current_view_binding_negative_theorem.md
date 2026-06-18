# Path 2 Current View-Binding Negative Theorem

Status: negative theorem for the current executable transparent-view binding
shape; does not rule out a redesigned typed-subcommitment API.

Purpose: push blocker 1 one level past the sampled-core counterexample. Even if
the public view transcript binds `creation_proof_binding_digest` together with
`trace_commitment_root`, the current core does not prove that the committed
trace opens the role commitments named by `Path2CreationProofBindingV1`.

## Current Binding Shape

`Path2CreationProofBindingV1` binds:

- `packed_witness_commitment`;
- `randomizer_commitment`;
- `pad_source_commitment`;
- `opening_tuple_commitment`;
- `projection_relation_commitment`;
- `gauge_commitment`;
- `native_role_root_commitment`;
- account/slot/lifecycle/context fields.

The role root is:

```text
native_role_root_commitment =
  H(packed_witness_commitment,
    randomizer_commitment,
    pad_source_commitment,
    opening_tuple_commitment,
    projection_relation_commitment,
    gauge_commitment)
```

The transparent public view binds only the digest of this binding object:

```text
creation_proof_binding_digest
```

and separately binds:

```text
trace_commitment_root
constraint_commitment_root
transparent_transcript_digest
```

The current proof core authenticates opened masked-share leaves to
`trace_commitment_root`; it does not contain a relation row, subproof, or
extractor theorem saying those leaves open the role commitments inside
`creation_proof_binding_digest`.

## Negative Theorem

For the current executable core:

```text
CoreAccept(view, manifest, proof)
and view.creation_proof_binding_digest = digest(binding)
```

does not imply:

```text
trace_commitment_root opens role vectors authenticated by binding
```

Reason: `creation_proof_binding_digest` is only transcript context for the
core. The verifier never opens or checks the binding's role commitments inside
the masked-share relation. An adversary can keep the binding digest fixed while
choosing a different synthetic trace root whose opened leaves satisfy the
masked-share equations.

The executable witness is:

```text
redteam_transparent_zk_core_accepts_public_only_synthetic_trace
```

It starts from an honest template view, preserves:

```text
synthetic_view.creation_proof_binding_digest
  = honest_view.creation_proof_binding_digest
```

but changes:

```text
synthetic_view.trace_commitment_root != honest_view.trace_commitment_root
synthetic_view.transparent_transcript_digest != honest_view.transparent_transcript_digest
```

The debug core accepts the synthetic proof. Therefore transcript co-location of
`creation_proof_binding_digest` and `trace_commitment_root` is not a semantic
opening link.

## Formal Lean Anchor

`Descender/Path2NativeBoundary.lean` now includes a minimal digest co-location
model:

```text
CurrentTraceBindingView
SameCreationBindingDigest(honest, synthetic)
```

and proves:

```text
same_creation_binding_digest_does_not_determine_trace_root
digest_colocation_does_not_imply_trace_role_binding
```

The first theorem shows that preserving a creation binding digest does not
determine the trace root. The second shows there is a model of the missing
`TraceRoleBinding` predicate in which digest co-location holds but semantic
trace-to-role binding is false. This is a minimal logical model of the
composition gap, not a collision attack on the hash or a full byte-level
formalization of the Rust transcript.

Lean also packages this theorem with the current row-core negative as:

```text
current_transparent_route_blocker1_negative_witness
```

The executable proof-family metadata binds the same negative status through
`Path2TransparentCurrentCoreNegativeTargetV1`, whose digest is carried by the
full-hidden-relation proof contract, production obligations, and proof-family
capability.

## Consequence

The selected two-hop payload binding remains viable only with an additional
full-relation proof obligation:

```text
TraceRoleBinding(trace, binding)
```

or a named break. This obligation must prove at least:

- pad-source trace columns open `binding.pad_source_commitment`;
- projection trace columns open `binding.projection_relation_commitment`;
- opening-tuple trace columns open `binding.opening_tuple_commitment`;
- packed-witness projection lanes agree with
  `binding.packed_witness_commitment`;
- all role openings are authenticated under the same parent trace/root context.

In the current fail-closed implementation, that missing obligation is exactly
part of the uninstantiated full-hidden-relation subproof contract
(`role_commitment_consistency_required = true`,
`role_commitment_consistency_implemented = false`).

## Go / No-Go Rule

The current binding shape is a no-go for production unless one of these happens:

1. instantiate a full-relation subproof backend proving `TraceRoleBinding`;
2. replace the role commitments with typed extractable subcommitments under the
   transparent parent trace root;
3. add trace roots to the creation binding and still prove that the trace opens
   the role commitments;
4. switch to a full-witness ZK route such as MITH where the extracted witness
   natively contains all role openings.

Without one of these, blocker 1 is negative: accepted masked-share transcripts
cannot imply a valid hidden opening for the accepted account-creation binding.

## Non-Claims

- This does not say two-hop payload binding is impossible.
- This does not say SHA role digests are collision-weak.
- This does not rule out typed transparent trace subcommitments.
- This does not replace the current-core negative theorem; it strengthens the
  reason the current core cannot be composed into account-admission soundness.
