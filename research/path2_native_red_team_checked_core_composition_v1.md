# Path 2 Native Red-Team: Checked-Core Composition V1

Status: closed for the current active-native Fq12 row-equation boundary.

## Attack Question

Do checked-core equation soundness, message binding, proof-commitment timing,
and native boundary binding compose without a gap or stale legacy-frame
dependency?

## What Holds

The native checked-core frame is strongly bound at the byte/frame layer:

- the frame includes native boundary version, parameter set, verifier family,
  account/spend/chain/fork domain digests, aggregation mode, native boundary
  digest, child projection digests, transcript digest, state digest, message
  digest, current state digest, and proof commitment;
- mutation KATs show every frame field changes the challenge;
- assembly derives the checked-core frame from the projection bundle rather than
  accepting independent frame bytes;
- legacy hidden-source/fallback handles are excluded from the native frame;
- the checked-core proof byte length is now fixed by
  `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES`.

## Red-Team Finding Before Wiring

The active-native verifier did not verify checked-core equations. It assembled
the native public boundary, computed/digested the checked-core frame, and
digested fixed-length proof bytes into the public spend view. A fixed-length
arbitrary proof byte string could therefore pass public-boundary preflight if
the carrier and bundle were otherwise internally consistent.

This was acceptable only because deployment remained fail-closed, but it was not
enough to close the deployable proof theorem.

## Blue-Team Action Taken

Added `path2_native_checked_core_accept_v1(frame, verifier_handle, proof)` and
wired it into `assemble_path2_native_boundary_v1`. The predicate:

- validates the native checked-core frame;
- requires `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES`;
- decodes the reviewed Fq12 account-carried proof;
- checks `proof.proof_commitment == frame.proof_commitment`;
- derives the checked-core challenge from `Path2NativeCheckedCoreFrameV1`;
- computes the actual Fq12 response syndrome from the proof response bytes;
- computes the expected response syndrome from the spend
  `checked_core_verifier_handle_digest`, frame challenge, and proof commitment;
- rejects unless the Fq12 sigma row equation matches.

The implementation-facing verifier path now calls this predicate before carrier
preflight can accept, because preflight calls boundary assembly.

The deployment preflight now sets:

```text
checked_core_composition_review_complete = true
```

The remaining deployment blockers are signer/verifier pipeline productization,
constant-time audit, and storage/lifecycle hardening.

## Tests

Commands run:

- `cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native_deployment_preflight --lib`
- `cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native --lib`
- `cargo test --manifest-path tools/production-reference/Cargo.toml --lib`

## Residual Risk

This closes the active-native Fq12 row-equation composition gap. It does not
claim that all signer/verifier plumbing is productized, constant-time, or
storage-hardened; those are tracked by separate deployment blockers.
