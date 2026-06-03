# Path 2 Native Red-Team: DA Digest / Projection-Preimage Ambiguity V1

Status: closed for the current executable boundary.

## Attack Question

Can a carrier commit to one 330-byte public semantic surface while the
projection-preimage data-availability material names a different public
interpretation?

This would matter because validators see only the compact carrier plus the
available projection preimage material. If the carrier accepts a caller-chosen
DA digest, an attacker can make the carrier internally consistent with an
arbitrary digest while serving different projection preimages off-carrier.

## Finding

The pre-red-team API treated `projection_preimage_da_digest` as an external
input to both carrier construction and verifier preflight. Preflight checked
that the carrier matched the same externally supplied digest, but it did not
force the digest to equal a canonical hash of the actual
`Path2NativeProjectionBundleV1`.

That left the DA digest free-floating. It was not a direct hidden-witness leak
or same-`q` private-fiber collision, but it was a consensus-binding gap: the
carrier could bind to digest `d` while the verifier caller separately selected
what `d` meant.

## Blue-Team Fix

Added a first-class canonical projection-preimage DA frame:

- `PATH2_NATIVE_PROJECTION_PREIMAGE_DA_DOMAIN`
- `path2_native_projection_preimage_da_v1_bytes`
- `path2_native_projection_preimage_da_digest_v1`

The frame is domain-separated and length/label framed. It binds:

- context projection bytes
- account projection bytes
- spend projection bytes
- lifecycle projection bytes
- state projection bytes
- obstruction flags digest
- checked-core public input digests: message, current state, proof commitment

Verifier preflight and deployment pipeline no longer accept a caller-supplied
DA digest. They recompute the digest from the bundle and reject carriers whose
`projection_preimage_da_digest` differs.

Signer public output also derives the digest from the bundle before constructing
the carrier.

## Tests

Covered by:

- `path2_native_consensus_carrier_preflight_recomputes_bundle`
- `path2_native_consensus_carrier_preflight_rejects_mismatches`
- `path2_native_consensus_carrier_preflight_reports_do_not_export_private_material`
- `path2_native_signer_pipeline_public_output_is_public_and_verifier_compatible`
- `path2_active_native_domains_and_public_schemas_are_unique`
- `production_domain_schema_registry_is_globally_disjoint`

Commands run:

- `cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native --lib`
- `cargo test --manifest-path tools/parameter-search/Cargo.toml domain --lib`
- `cargo test --manifest-path tools/production-reference/Cargo.toml --lib`

## Residual Risk

This closes the executable free-floating digest gap. It does not by itself prove
that the external DA layer stores and serves the bytes under that digest with
availability/finality guarantees; that belongs to the target-chain storage and
DA hardening pass.
