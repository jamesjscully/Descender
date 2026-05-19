# Implementation Audit Note

Date: 2026-05-19

Scope:

- `tools/parameter-search/src/bin/signature_q12289_implicit_verifier.rs`
- implementation-gate closure status for A6/A7/A8 in
  [assumption_review.md](assumption_review.md)

This note is intentionally narrow.  The current Rust artifact is a verifier
prototype plus vector generator.  It is not a signer, not a key generator, and
not a deployable cryptographic implementation.

## What Was Checked

Executable checks now cover:

- canonical public-key length reject;
- canonical signature length reject;
- challenge high-bit reject;
- public-key q-coordinate canonicality reject;
- response bound reject;
- pinned hash-frame length and SHA-256 digest;
- positive verification of the pinned zero-public-key fixture;
- single-byte mutation reject across every public-key byte and every signature
  byte of the positive fixture;
- conformance-bundle regeneration/diff through `--verify-conformance-vector`;
- independent parsing of the generated hash frame and payload bitstreams inside
  the verifier harness.

The generated conformance bundle now includes twelve verifier-negative cases
plus two hash-frame-negative cases, adding explicit short/long public-key,
short/long signature, negative response bound, message mutation, public-key
mutation, reconstructed-commitment length, and reconstructed-commitment
canonicality checks.

## A6: Signer Distribution

Status: open.

Reason:

- no reference signing algorithm exists in the tree;
- no concrete mask sampler or acceptance predicate exists in code;
- no retry policy or rejection-timing behavior exists to audit;
- the Lean `RejectionSamplingCertificate` endpoint is not instantiated by a
  concrete signer.

Conclusion:

The signer relation audit cannot be closed yet.  Any claim about secret
independence of accepted transcripts would be premature.

## A7: Byte-Level Verifier Conformance

Status: partially strengthened, still open.

What improved:

- parser rejection coverage is broader and directly executable;
- the conformance vector bundle now records more canonical-length failures;
- byte-mutation checks reduce the risk of unnoticed accept-on-corruption bugs
  in the current fixture path.

What still blocks closure:

- no fuzz target harness for arbitrary-input exploration beyond unit tests.
- no production implementation independent from the Rust prototype.

Conclusion:

The review checklist's conformance gate is closed for the current Rust
prototype/vector bundle.  A production implementation should still be checked
against the same vectors before deployment.

## A8: Randomness, Constant Time, And Secret Handling

Status: open.

Current code reality:

- the verifier prototype consumes only public inputs;
- it does not generate randomness;
- it does not hold signing secrets or secret keys in memory;
- it does not implement keygen or signing, so there is no secret-dependent
  branch/index audit to perform on production-critical paths.

Implications:

- randomness handling review cannot be closed because there is no RNG API,
  failure policy, or deterministic-test override for keygen/signing;
- zeroization review cannot be closed because there is no signer/keygen secret
  state or zeroization mechanism to inspect;
- constant-time review cannot be closed because the sensitive operations do not
  exist yet in code.

Required next steps:

1. introduce a reference keygen/signing skeleton with explicit RNG interfaces
   and hard-failure behavior;
2. isolate secret-bearing types and add zeroization hooks where supported;
3. review signer/keygen control flow for secret-dependent branches, indexing,
   and observable retry behavior;
4. add dedicated fuzz targets once parser/verifier helpers are moved behind a
   reusable library surface.

## Review Gate Effect

Closed by this note:

- Review Bundle Gate: `One implementation audit note.`

Still open after this note:

- Reference keygen.
- Reference signing.
- Constant-time review.
- Fuzz targets.
- Randomness handling review.
- Secret zeroization review.
