# Jasmin — Deployable Implementation Layer

Status: ACTIVE_RESEARCH

This package is a deployable-implementation research lane, not product
authority by itself. Current verifier, signer, wire-format, and adoption status
starts in `../../spec/README.md`, ADRs, and `../../TRACKER.md`.

**Owns:** the production verifier/signer core: constant-time, functionally
correct vs the EasyCrypt/spec model, compiled to verified assembly. Closes seam 2
(spec↔code) by proof, not assumption.

**Status:** no Jasmin implementation exists yet. EC-SPIKE returned GO
(qualified), so the implementation lane is gated by a current EasyCrypt VERIFY
model, current wire-format/KAT targets, and Jasmin toolchain bring-up.

**First deliverable (JASMIN-SCOPE):** identify the smallest deployable core to
implement first — the checked-core parse + Fq12 sigma-equation check (`spec` §2,
§4 Verify steps 1–3) — and estimate effort. The surviving value is the
constant-time/proof-engineering decomposition; byte targets must come from the
current spec spine.

Notes:
- Jasmin + EasyCrypt is the battle-tested deployable PQ stack (Formosa / ML-KEM).
- Must reproduce `../kat/` byte-for-byte.
- Constant-time is a first-class obligation (D2 conservative): the verifier must
  be CT on all public-data-dependent rejection paths.
