# Descender Production Reference

> MIGRATION STATUS (2026-06-20): `LEGACY_FORMAL`.
> This crate and README describe the old account-carried / projection-carrier
> production-shaped API boundary. It is retained for executable baselines and
> negative controls, but it is not the active product specification. Current
> authority starts at `spec/README.md` and the Poseidon2/Plonky3 account-creation
> path recorded in
> `decisions/ADR-0004-poseidon2-plonky3-active-account-creation-path.md`.

This crate is the production-shaped API boundary for the account-carried
Descender reference implementation.

It re-exports the checked `production_reference` surface from
`descender-parameter-search` without exposing the research binaries.  It also
adds deployment-facing facade helpers:

- `deployment_sign_with_secret_key`
- `deployment_sign_reserved_and_consume`
- `deployment_verify_with_account_binding`

These helpers route through sanitized callback-error boundaries.  The actual
proof signer/verifier callback is still supplied by the caller until the
selected account-carried verifier has real compact hidden-source proof
semantics.

This is an intermediate split, not a deployment claim:

- the account-link layout is the reviewed `68 B` state boundary;
- the exact reviewed target transaction size is `256 B`;
- the archived aggressive branch is the strict `290 B` envelope:
  `258 B/Fq12` transaction plus a `32 B` hidden-source proof handle;
- the legacy-selected robust branch is the strict `314 B` projection-carrier envelope:
  `258 B/Fq12` transaction plus a `56 B` hidden-source fallback projection
  carrier;
- the selected account-state hidden-source commitment model is the
  state-assisted opening tuple; the direct expanded account-opening slot
  commitment path is archival/non-deployment;
- that `314 B` verifier-shaped entry point is present only as a fail-closed
  surface while the active-lane projection relation is promoted through
  preflight and the remaining account-witness opening, soundness, and
  zero-knowledge proof semantics are complete;
- `production_deployment_preflight()` intentionally fails closed until proof
  semantics, constant-time review, storage hardening, and external review land.

Run:

```bash
cargo test --manifest-path tools/production-reference/Cargo.toml
```
