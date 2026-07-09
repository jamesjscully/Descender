# Constant-Time And Leakage Evidence Profile

Status: ACTIVE_NORMATIVE

This file defines the constant-time, side-channel, fixed-shape failure, and
secret-leakage evidence model for Descender signer, wallet, HSM, secure-element,
custody, and proof-generation deployments.

The profile exists because Descender's active path is intended to avoid the
signer-side fragility that makes floating-point post-quantum signatures awkward
in hostile wallet hardware. Avoiding floating point is only the first rule. A
deployment claim also needs auditable secret/public classification, platform
profiles, static review, leakage tests or reviewed substitutes, serialization
negative tests, and fixed-shape failure behavior.

## Scope

The leakage profile covers:

- secret/public classification for signer-side and prover-side code;
- forbidden secret-dependent behavior;
- constant-time and fixed-shape failure review requirements;
- leakage-harness requirements for keygen, sampler, pad expansion, witness
  handling, proof generation, and spend authorization;
- public serialization negative tests;
- compiler, target, dependency, and build-mode profiles;
- residual-risk and external-review requirements for deployment claims.

It does not define public consensus verification semantics. Public verifier
parse/reject behavior is governed by `spec/consensus_verifier.md`. Signer API
commands are governed by `spec/signer_api.md`; entropy-source and stream rules
are governed by `spec/entropy_profile.md`.

## Secret/Public Classification

Every reviewed implementation surface must classify values before claiming
constant-time or leakage evidence.

| Class | Examples | Rule |
|---|---|---|
| `secret_key_material` | keygen seed, private key bytes, custody shares | never public; zeroize or secure-store |
| `secret_witness` | hidden account witness, private projection residue, proof witness columns | never public unless a theorem explicitly marks a field public |
| `secret_entropy` | pad seed, sampler randomness, proof blinding, spend randomizer | never logged or serialized; domain-separated by `spec/entropy_profile.md` |
| `local_state_secret` | unreleased slot state, pending-release metadata, rollback-sensitive custody state | not consensus-visible; API exposure requires an explicit public class |
| `public_context` | account id, slot id, message digest, state root, profile id | may drive public branches and resource accounting |
| `public_result` | fixed public result class, public slot record, accepted public proof bytes | may be serialized under the public API and KAT schema |

Unknown values default to secret until classified. A deployment profile must not
inherit a research wrapper's classification unless the wrapper is part of the
reviewed build.

## Forbidden Behaviors

Deployment builds must not use any of the following with secret inputs:

- floating-point arithmetic;
- secret-dependent branches or early exits;
- secret-dependent table indices, memory addresses, or allocation shapes;
- secret-dependent loop counts visible to the host, transport, or power/timing
  profile;
- secret-dependent error strings, panic messages, callback diagnostics, or log
  lines;
- host-endian or implementation-defined arithmetic for secret-bearing values;
- unreviewed compiler intrinsics, hardware acceleration, or dependency code on
  secret-bearing paths;
- debug or diagnostic callbacks that can run in a deployment profile.

Public verifier parsing may branch on public input length or public version.
That does not establish signer-side safety.

## Evidence Matrix

Every deployment claim must fill an evidence matrix for each secret-bearing
surface.

| Surface | Minimum evidence before deployment claim |
|---|---|
| keygen | static secret-flow review, entropy-source review, zeroization/secure-storage review, leakage or reviewed substitute |
| sampler/rejection loop | bounded public shape, secret-independent externally visible retry behavior, fixed public failure class |
| pad expansion and mask PRF | domain/stream review, CT arithmetic review, signer/proof drift KATs, leakage or reviewed substitute |
| witness handling | no-export serialization tests, memory lifetime review, proof-public-boundary review |
| proof generation | proof-family leakage review, blinding review, deterministic-test separation, callback/log audit |
| spend authorization | message/context binding review, slot-state fixed-shape failure tests, no hidden diagnostics |
| signer API | fixed public errors, diagnostic-build separation, malformed-request fixed-shape behavior |
| storage and rollback | secure state update review, crash/restore tests, no secret leakage through recovery diagnostics |

`reviewed substitute` means a documented review argument accepted by the review
package for a surface where ordinary leakage testing is not meaningful or not
yet instrumented. It must name the target, compiler, dependency versions, and
residual risk.

## Leakage Harness Requirements

A leakage harness or reviewed substitute must define:

- target platform and build profile;
- binary, crate, function, or command under test;
- secret classes under variation;
- public context held fixed;
- trace source or observation model, such as timing, instruction count, memory
  addresses, cache observations, power/EM traces, or reviewed constant-shape
  instrumentation;
- statistical test, threshold, and failure interpretation;
- sample count or review rationale;
- negative controls that the harness can detect when a known leak is introduced;
- artifact format and reproducibility command.

A harness that only exercises public verifier rejection does not satisfy
secret-side leakage evidence. A desktop harness does not imply hardware wallet,
secure-element, HSM, or custody readiness unless the target profile says why the
observation model transfers.

## Fixed-Shape Failure Requirements

Secret-bearing operations must collapse externally visible failures into fixed
public classes.

Required failure cases:

- malformed public request;
- unsupported version or profile;
- consumed, pending, burned, absent, or restore-uncertain slot;
- entropy-source failure;
- callback or dependency failure;
- storage write failure before releasing an authorization;
- proof-generation failure;
- resource-limit failure.

For each case, tests or review must show that deployment builds do not expose
secret-dependent retry counts, branch-specific diagnostics, witness material,
raw dependency errors, or platform exception text.

## Serialization And Logging Negative Tests

Public bytes, API responses, logs, panic messages, and generated artifacts must
be checked for forbidden material before deployment readiness.

Required negative tests:

- public slot records do not contain secret key material, pad seeds, projection
  masks, private openings, sampler randomness, or hidden witnesses;
- spend authorizations do not contain local signer diagnostics or unreleased
  state metadata;
- diagnostic builds use a separate profile and cannot emit deployment-accepted
  authorizations unless explicitly reviewed;
- panic/log paths in deployment builds do not print secrets or raw callbacks;
- generated conformance vectors include only public fields or explicitly
  test-only deterministic seeds.

## Target And Build Profiles

Every leakage claim must name the exact build profile:

| Field | Requirement |
|---|---|
| `target_id` | platform, architecture, OS/firmware, secure-element/HSM model if any |
| `compiler_profile` | compiler, flags, LTO/codegen settings, sanitizer settings if any |
| `dependency_profile` | pinned dependency versions and reviewed secret-bearing dependencies |
| `feature_profile` | enabled features, diagnostic flags, test/deployment mode |
| `resource_profile` | stack, heap, storage, APDU/message, timeout, and retry limits |
| `evidence_artifacts` | review notes, harness outputs, KATs, negative tests, residual risks |

Changing any target/build field invalidates deployment-specific CT/leakage
claims until evidence is refreshed or a review says the change is irrelevant.

## Relationship To Current Evidence

Current active evidence is partial:

- `spec/signer_profile.md` states the non-negotiable signer-side rules;
- `spec/signer_api.md` defines fixed public API result classes and diagnostic
  separation requirements;
- `spec/entropy_profile.md` defines entropy streams and records current
  mask-PRF drift evidence;
- active Rust gates compile the signer-side mask-PRF crate and run mask-PRF
  equivalence tests.
- `scripts/check_active_rust_slow_tests.sh` names a manual desktop-reference
  empirical hiding mask-channel battery for account-STARK creation, and the
  leakage battery checker pins that slow-test command, source markers, sample
  count, threshold, and test functions as fail-closed metadata rather than a
  deployment claim.
- `scripts/check_active_secret_float_surface.py` is wired into local and CI
  gates to reject floating-point types, casts, literals, and math calls in
  first-party active signer, prover, verifier, and recursion source outside
  test-only code and explicit non-deployment estimator support. The same gate
  requires the account-STARK byte counter to reject serde float serialization.

This is not enough for a CT/leakage deployment claim. The missing pieces are the
static secret-flow audit, target/build profiles, leakage harness or reviewed
substitute, serialization negative tests, fixed-shape failure tests, and
external review for hardware/custody claims.

## Required Evidence

Tracker ownership: `SPEC-LEAKAGE-001`, `DIR-CT-001`, `ADOPT-HW-CT-001`,
`SPEC-SIGNER-001`, `ADOPT-HW-API-001`, `ADOPT-HW-ENT-001`,
`ADOPT-HW-STATE-001`, `DIR-KAT-001`, `SPEC-KAT-001`, `SPEC-DEPLOY-001`,
`DIR-LIFE-001`, `DIR-REJECT-001`, and `DIR-REVIEW-001`.

Before the constant-time/leakage surface can be marked `verified`, the repo
needs:

- secret/public classification for every signer/prover deployment surface;
- static review of secret-dependent branches, memory access, arithmetic,
  allocation, logging, callbacks, and dependencies;
- target/build profiles for each claimed deployment class;
- leakage harnesses or reviewed substitutes for keygen, sampler, pad expansion,
  witness handling, proof generation, and spend authorization;
- fixed-shape failure tests for malformed input, consumed/pending/burned slots,
  entropy failure, callback failure, storage failure, proof failure, and
  resource-limit failure;
- serialization/logging negative tests for public API outputs, public proof
  bytes, conformance vectors, logs, and panic paths;
- zeroization or secure-storage evidence for secret material lifetimes;
- CI or manual cadence for running expensive leakage tests;
- external review for any hardware wallet, secure-element, HSM, or custody
  side-channel claim.

Until those artifacts exist, `DIR-CT-001` and `ADOPT-HW-CT-001` remain below
`verified`, and hardware/custody adoption surfaces remain below `ready`.
