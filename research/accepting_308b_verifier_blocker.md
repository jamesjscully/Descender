# Legacy 308 B Verifier Blocker Negative Control

> MIGRATION STATUS (2026-06-20): `NEGATIVE_CONTROL`.
> This file records why the old 308-byte projection-carrier verifier could not
> become the product acceptance path. Keep it as attack/negative-control history;
> current authority is `../spec/README.md`, `../decisions/ADR-0005-archive-legacy-projection-carrier-product-target.md`,
> and `../TRACKER.md`.

This note records why the historical 308-byte projection-carrier verifier had to
remain fail-closed. It also records the disposition of the formerly blocking
inactive projection survivor. It is retained as negative-control evidence, not as
a live implementation plan or current product target.

## Historical Verifier Shape

The old reviewed verifier shape was the `308 B/Fq12` projection-carrier envelope:

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

Those were necessary verifier/exporter controls, but they did not prove that the
historical projection relation was strong enough to accept.

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

## Current Disposition Of This Evidence

The active product path is the Poseidon2/Plonky3 Path 2 authority chain in
`../spec/README.md` and `../TRACKER.md`, not this projection-carrier envelope.
This note remains only to explain the fail-closed negative control and the old
survivor.

In the old projection-carrier remediation line, the twelve-lane relation exposed
by the 56-byte projection carrier replaced the old 64-variable generalized
projection relation. The active-subspace replay classifies the old survivor as
`invalid_for_active_subspace_relation`:

```text
old space column: 0
in active subspace: false
active projection nullity: 0
open replay rows: 0
```

That removed this specific inactive-coordinate survivor from the active-lane
relation in that historical line. It did not make the deployment verifier
accepting.

## Consequence

An accepting verifier built on the old 64-variable rows would have been
underconstrained. The deployment verifier must not be changed from fail-closed
to accepting unless the current spec spine and `TRACKER.md` record the active
relation, account-witness opening, soundness, and zero-knowledge proof
obligations as closed.

## Retained Evidence Role

This file does not own live work. The retained value is the negative-control
lesson: projection-carrier verifier code must remain fail-closed unless the
current authority chain has a complete proof relation and regression evidence.
The owning tracker rows are `PROC-ARCHIVE-007` and `PROC-LEGACY-GATE-001`.

## Source Artifacts

- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.md`
