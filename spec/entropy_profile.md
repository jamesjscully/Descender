# Entropy And Domain-Separation Profile

Status: ACTIVE_NORMATIVE

This file defines the entropy-source and domain-separation requirements for
Descender signer, wallet, HSM, custody, and proof-generation deployments. It
turns the signer-profile entropy paragraph into a registry-style contract:
which streams exist, what each stream may derive, how deterministic test mode is
separated, and what evidence is required before a deployment claim.

The profile exists because a chain-native authorization stack cannot rely on
implicit randomness plumbing. A duplicated pad seed, reused slot stream,
cross-domain transcript collision, or diagnostic test seed leaking into
deployment can become direct asset-loss or privacy-loss behavior.

## Scope

The entropy profile covers:

- deployment entropy-source classes;
- domain-separated streams for keygen, slot refresh, pad derivation,
  randomizer derivation, proof blinding, and deterministic tests;
- binding requirements for account id, slot id, policy, profile, chain domain,
  and lifecycle anchor;
- reproducible KAT mode and deployment-mode separation;
- evidence needed for software wallets, secure elements, HSMs, custody
  services, and proof-generation hosts.

It does not define constant-time behavior, leakage tests, or fault-state
recovery. Those remain governed by `spec/signer_profile.md`,
`spec/leakage_profile.md`, `spec/signer_api.md`, and
`spec/lifecycle_model.md`.

## Entropy Source Classes

Every deployment profile must name one entropy source class for each
secret-bearing command.

| Class | Requirement |
|---|---|
| `software_os_csprng` | uses a reviewed OS CSPRNG API with documented failure behavior |
| `secure_element_rng` | uses the secure element RNG or approved host/SE mixing profile |
| `hsm_rng` | uses an HSM-approved DRBG/RNG with health-test and failure semantics |
| `custody_service_rng` | uses a custody-service DRBG/RNG with audit logging and quorum policy where applicable |
| `deterministic_test_rng` | reproducible test-only source, compile-time or profile separated from deployment |

A deployment build must fail closed if its configured entropy source is
unavailable, reports failed health checks, or cannot satisfy the requested
stream. It must not silently fall back to deterministic test mode.

## Stream Registry

Every stream must have a versioned domain label and a fixed input schema before
deployment readiness. These are the required direct-mode streams:

| Stream | Secret/public | Purpose | Required context binding |
|---|---|---|---|
| `keygen_seed` | secret | root secret material or keygen seed | profile id, account namespace, keygen version, entropy-source class |
| `slot_refresh_seed` | secret | slot creation or refresh randomness | profile id, account id, slot id, policy digest, lifecycle anchor |
| `pad_seed` | secret | projection-mask source material | profile id, account id, slot id, creation-evidence version |
| `projection_mask_prf` | secret input, public masked output | derive q12289 projection masks from `pad_seed` and slot id | profile id or frozen C2 stream id, slot id, mask-PRF version |
| `spend_randomizer` | secret | spend-time randomizers if a selected relation needs them | profile id, account id, slot digest, message digest, state context |
| `proof_blinding` | secret | prover blinding and trace-opening randomness | profile id, proof family, public statement digest, transcript domain |
| `diagnostic_test_seed` | secret test-only | deterministic KAT and regression reproduction | test profile id, vector id, generator version |

The stream registry is append-only after a profile is frozen. Changing a domain
label, context field, byte order, or stream purpose requires a new profile or
corpus version.

## Active Mask-PRF Evidence

The current active implementation has partial evidence for the
`projection_mask_prf` stream:

- `tools/descender-mask-prf` implements the signer-side no-Plonky3
  Poseidon2/KoalaBear mask derivation;
- `tools/descender-account-stark/tests/mask_prf_equivalence.rs` checks that the
  standalone signer-side derivation is byte-identical to the account-STARK
  in-proof reference over deterministic samples and edge inputs;
- `tools/descender-mask-prf` tests cover determinism, input sensitivity,
  q12289 range, a Plonky3 Poseidon2 KAT, and a structural wide-reduce bias
  margin.

This evidence prevents signer/proof PRF drift. It does not by itself prove a
deployment entropy-source profile, secure-element RNG integration, stateful
slot uniqueness, or secret-side leakage resistance.

Before `projection_mask_prf` can be frozen for deployment, the profile must
either:

1. freeze the existing C2 byte stream as the versioned profile stream with KATs;
   or
2. add an explicit domain-labeled stream and migrate the proof/signing path with
   positive and negative KATs.

The choice must be recorded in `TRACKER.md` or a later ADR before the entropy
surface can be marked `verified`.

## Context Binding Rules

For deployment streams:

1. Stream derivation must bind `profile_id` and stream version.
2. Account-specific streams must bind account id or account namespace.
3. Slot-specific streams must bind slot id and, where available,
   `slot_authorization_digest`.
4. Spend-specific streams must bind message digest and state context digest.
5. Proof-blinding streams must bind proof-family id and public statement digest.
6. Diagnostic/test streams must bind vector id and must not use the deployment
   domain.

If a stream intentionally omits one of these fields, the omission must be named
in the security model or deployment profile and charged to a specific residual
assumption.

## Deterministic Test Mode

Deterministic test mode is required for KAT reproduction but forbidden as a
deployment fallback.

Rules:

- deterministic test mode must use a distinct `profile_id` or compile-time
  feature from deployment builds;
- generated KATs must state the deterministic seed source and vector id;
- deployment builds must reject or fail to compile any diagnostic/test command
  that can emit deployment-accepted authorizations;
- logs, panic messages, and API errors must not print derived seeds or stream
  state;
- conformance tests must include a negative check that deployment profiles do
  not accept test-profile outputs unless the profile explicitly says they are
  test-only.

## Reuse And Collision Policy

A signer or custody service must treat stream reuse as fail-closed unless a
reviewed theorem or profile permits it.

The profile must define:

- whether entropy is generated per account, per slot, per spend, or per proof;
- how duplicate `slot_refresh_seed`, `pad_seed`, and `proof_blinding` state is
  detected in local storage;
- what happens after a crash before persistent state is updated;
- whether host-provided entropy is mixed, rejected, or used only as public
  auxiliary input;
- the public result class for entropy-source failure.

Replaying the same entropy input must not silently create two live slots with
the same authorization identity.

## Required Evidence

Tracker ownership: `ADOPT-HW-ENT-001`, `SPEC-SIGNER-001`, `DIR-KAT-001`,
`SPEC-KAT-001`, `ADOPT-HW-API-001`, `ADOPT-HW-STATE-001`,
`ADOPT-HW-CT-001`, `DIR-CT-001`, `SPEC-DEPLOY-001`, `DIR-LIFE-001`, and
`DIR-REJECT-001`.

Before the entropy/domain-separation surface can be marked `verified`, the repo
needs:

- a frozen stream registry for the target deployment profile;
- canonical byte encodings and domain labels for every stream in scope;
- KATs for `projection_mask_prf`, proof blinding, deterministic test mode, and
  any spend randomizer stream in the selected relation;
- negative KATs proving wrong profile, wrong slot, wrong account, wrong message,
  wrong state context, and wrong test/deployment domain reject or change public
  outputs as specified;
- entropy-source requirements for software wallet, secure element, HSM, and
  custody-service profiles that claim support;
- deployment tests proving deterministic test mode cannot be used as a fallback;
- crash/retry/restore tests showing duplicate stream material cannot silently
  create two live slots;
- logging/API tests proving derived seed material is not exposed;
- external review for any hardware wallet, HSM, or custody entropy claim.

Until those artifacts exist, the entropy profile is partial adoption evidence:
the stream model is specified, and the current mask-PRF drift evidence is useful,
but deployment entropy readiness remains unproved.
