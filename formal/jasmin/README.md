# Jasmin — Deployable Implementation Layer

**Owns:** the production verifier/signer core: constant-time, functionally
correct vs the EasyCrypt/spec model, compiled to verified assembly. Closes seam 2
(spec↔code) by proof, not assumption.

**Status:** not started — scoped after **EC-SPIKE** (Jasmin's value is realized
through EasyCrypt's correctness proof).

**First deliverable (JASMIN-SCOPE):** identify the smallest deployable core to
implement first — the checked-core parse + Fq12 sigma-equation check (`spec` §2,
§4 Verify steps 1–3) — and estimate effort. No spike code until EC-SPIKE returns.

Notes:
- Jasmin + EasyCrypt is the battle-tested deployable PQ stack (Formosa / ML-KEM).
- Must reproduce `../kat/` byte-for-byte.
- Constant-time is a first-class obligation (D2 conservative): the verifier must
  be CT on all public-data-dependent rejection paths.
