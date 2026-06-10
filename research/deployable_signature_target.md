# Deployable Signature Target

This file pins the current selected target for internal deployment-readiness work.
It is a specification target, not a deployment claim.

> Sizes updated 2026-06-09 to the post-AP2-004 challenge width (27-byte
> challenge). The authoritative computed/tested value in
> `production_reference.rs` is `REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES == 314`.
> The earlier `252 B` base / `308 B` envelope figures were pre-widening. RELABEL
> DONE 2026-06-09: the stale "308" Rust strings and the source-gate-inspected
> fixture `selected_projection_valid_308_byte_fixture_accepts` were renamed to
> `314` in coordinated lockstep (the `.find()` marker, the marker-name string, and
> the `fn` definition all updated together; `PRODUCTION_DEPLOYMENT_BLOCKER_*` and
> the timing-fixture message too). Verified: the renamed fixture test passes and
> the source-gate still resolves the marker.

## Selected Target

- Profile: `Descender.Signature.q12289.local_delta.stateful.v1`
- Selected envelope: `314 B/Fq12` projection-carrier hidden-source envelope
- Status: fail-closed selected research track
- Verifier state: deployment-shaped entrypoint exists, but must reject until the active-lane relation is promoted through preflight and the remaining account-witness opening, soundness, and zero-knowledge proof semantics close
- Base checked-core transaction: `258 B`
- Projection carrier: `56 B`
- Total selected envelope: `314 B`
- Account-state hidden-source commitment model: state-assisted opening tuple.
  The direct expanded account-opening slot commitment path is archived and not
  a deployment verifier surface.

The selected envelope is:

```text
reviewed_fq12_account_carried_signature[258]
|| hidden_source_fallback_projection_carrier[56]
```

The 56-byte carrier is:

```text
projection_lanes[24]
|| check_bytes[3]
|| transcript_binding[29]
```

where `projection_lanes` are twelve canonical little-endian `q = 12289`
residues. The selected lane map is the one implemented by
`HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS` and
`HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_COEFFICIENTS` in
`tools/parameter-search/src/production_reference.rs`.

The selected hidden-source projection statement is the twelve-lane active
relation exposed by the carrier. The earlier 64-variable generalized projection
model is not the selected accepting relation for this byte target; with only a
56-byte carrier it leaves inactive hidden-source coordinates unbound.

## Archived Routes

- `256 B` reviewed account-carried route: codec and account/challenge binding
  surface only. It is not the selected deployment verifier.
- `260 B` full-response public quotient route: rejected as a deployable route
  because the public representative relation admitted a public forgery class.
- `290 B/Fq12` RO-PoK handle route: archived aggressive research option. It uses
  a 32-byte hidden-source proof handle and was superseded by the 56-byte
  projection carrier because the projection route gives concrete public
  algebraic material for exact certificate attempts.
- direct expanded account-opening slot commitment route: archived
  projection-wiring option. The selected 314-byte route must commit the
  state-assisted opening tuple instead, with the projection opening carried as
  `witness_opening_digest`.

## Required Acceptance Semantics

Before this target can become deployable, the final verifier must accept only if
all of the following hold:

- the 258-byte Fq12 checked core parses canonically;
- account slot, witness commitment, randomizer commitment, and slot freshness
  checks pass;
- the Fiat-Shamir challenge frame binds public key, message, slot id, witness
  commitment, and proof commitment;
- the Fq12 sigma equation holds;
- public local fiber consistency holds;
- public pad-opening consistency holds;
- the state-assisted opening tuple is fixed in account state before the
  challenge and binds slot id, witness commitment, randomizer commitment,
  restore anchor, projection-opening digest, pad-seed digest, and carrier-handle
  secret digest;
- the 56-byte projection carrier parses canonically;
- carrier check bytes bind the selected projection map and profile;
- transcript binding prevents carrier replay across public keys, messages,
  checked-core signature bodies, and account slots;
- the selected tuple verifier composes the state slot opening, projection
  opening, pad seed, hidden-material lanes, and 56-byte carrier binding in one
  executable relation;
- projection lanes are linked to the checked-core response relation or export a
  named break;
- projection lanes are linked to the hidden-source account-witness and pad-source
  openings or export a named break;
- leakage from all 56 carrier bytes is either charged as public projection
  leakage or classified as public context binding;
- no deployment-facing verifier or theorem path reasons over the rejected
  64-variable projection model as the accepted hidden-source relation;
- all rejection behavior is public-data dependent and fail-closed.

## Current Blocking Fact

The historical exact bridge replay over the 64-variable projection model found
a bounded inactive-coordinate survivor. The active-lane replay excludes that
specific survivor for the twelve verifier-visible lanes, and the pad-opening
witness binding is now executable; the hidden-source acceptance contract also
credits executable statement, pad-seed, and transcript binding. The
state-assisted path now binds the selected account-witness projection-opening
frame into the opening tuple and hidden-source statement, but that is not yet a
full opening of the account witness commitment. The regression test
`projection_carrier_tuple_does_not_prove_full_witness_opening` demonstrates the
gap directly: two different packed Fq12 account-witness commitments can accept
against the same six-value projection-opening frame after rebinding the allowed
tuple/carrier transcript. The selected verifier still remains fail-closed until
the active-lane relation is wired into the final theorem surface and either a
full account-witness opening proof is added, or the deployment claim is changed
to a proved projection-only relation with soundness and zero-knowledge semantics.

## Source Of Truth

The selected target is consistent with:

- `research/cryptanalysis/signature_readiness_20260516/hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.md`
- `research/cryptanalysis/signature_readiness_20260516/hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.md`
- `tools/parameter-search/src/production_reference.rs`
- `tools/production-reference/src/lib.rs`
