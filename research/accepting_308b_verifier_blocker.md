# Accepting 308 B Verifier Blocker

This note records why the selected 308-byte verifier must remain fail-closed.
It also records the current disposition of the formerly blocking inactive
projection survivor.

## Current State

The selected target is the `308 B/Fq12` projection-carrier envelope:

```text
reviewed_fq12_account_carried_signature[252]
|| hidden_source_fallback_projection_carrier[56]
```

The executable boundary has the main parser/exporter components:

- carrier decoding;
- public carrier lane/check/transcript comparison;
- checked-core verification or break export;
- response-link kernel certificate check or export;
- hidden-source opening projection check or export;
- account commitment split check or export;
- pad PRF projection check or export.

Those are necessary verifier/exporter controls, but they do not prove that the
selected projection relation is strong enough to accept.

## Historical Blocking Survivor

The exact infeasibility replay for the instantiated generalized projection
system over the old 64-variable projection model exports a bounded survivor:

```text
space: hidden_source_projection_variables
space_column: 0
balanced_value: 1
linf_bound: 1
checked_core_response_difference_nonzero: false
carrier_lane_difference_nonzero: false
check_byte_difference_nonzero: false
```

This means the old rows allow an inactive hidden-source projection coordinate to
move without changing the checked-core response difference, serialized public
carrier lanes, or check bytes.

## Current Disposition

The selected path is now the twelve-lane active relation exposed by the
56-byte projection carrier, not the old 64-variable generalized projection
relation. The active-subspace replay classifies the old survivor as
`invalid_for_active_subspace_relation`:

```text
old space column: 0
in active subspace: false
active projection nullity: 0
open replay rows: 0
```

That removes this specific inactive-coordinate survivor from the selected
active-lane relation. It does not by itself make the deployment verifier
accepting.

## Consequence

An accepting verifier built on the old 64-variable rows would be
underconstrained. The deployment verifier must not be changed from fail-closed
to accepting until the active-lane relation is the only deployment-facing
relation and the remaining account-witness opening, soundness, and
zero-knowledge proof obligations close.

## Immediate Work Item

The next security-critical implementation task is not to flip the verifier to
accept. It is to promote the active-lane relation through production preflight,
the theorem surface, and Rust verifier naming, then continue with the remaining
hidden-source semantic checks. The executable statement, pad-seed, and
transcript-binding pieces are now credited in the partial acceptance contract;
they are not enough to make the proof deployable.

## Source Artifacts

- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.md`
