# Path 2 Native Red-Team: No-Export Leakage V1

Status: closed for the current executable public boundary; proof-system hiding
remains charged to the checked-core proof assumptions.

## Attack Question

Do carrier fields, preflight reports, lifecycle markers, obstruction flags, or
projection digests distinguish private representative fibers beyond the intended
public class?

## Result

No raw private field export was found in the active native carrier, account
view, spend view, preflight report, verifier verdict, or signer public output.
Existing KATs already scan serialized/debug public outputs for private fiber,
pad, opening, gauge, raw proof, fallback-carrier, and decoded private-lane
markers.

The pass did find one executable leakage-shape issue: the public spend view
exported `checked_core_proof_byte_len` while accepting arbitrary proof byte
lengths. The proof bytes are public, but a deployable signature should have a
parameter-set-fixed checked-core proof length. If that length were allowed to
vary with signer/prover choices, the length field could become an avoidable
side channel and aggregation malleability surface.

## Blue-Team Fix

Added:

- `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES =
  REVIEWED_FQ12_ACCOUNT_CARRIED_PROOF_BYTES`

`public_native_spend_view_v1` now rejects any checked-core proof whose length is
not exactly `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES`. This also forces
`assemble_path2_native_boundary_v1`, carrier preflight, verifier pipeline, and
signer public output through the same fixed-length boundary because they all
construct the spend view.

The production facade exports `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES`.

## Tests

Covered by:

- `public_native_spend_view_v1_serializes_digest_only_proof_surface`
- `path2_native_consensus_carrier_preflight_reports_do_not_export_private_material`
- `path2_native_verifier_pipeline_is_public_only_and_fail_closed_until_productized`
- `path2_native_signer_pipeline_public_output_is_public_and_verifier_compatible`

Commands run:

- `cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native --lib`
- `cargo test --manifest-path tools/production-reference/Cargo.toml --lib`

## Residual Risk

This pass rules out direct export and variable-length proof leakage in the
current executable public boundary. It does not prove zero-knowledge for the
checked-core proof bytes themselves; that remains the proof-family hiding
obligation and should be reviewed in the checked-core composition pass.
