# HAX-SPIKE — target function + obligation (paper-spike)

> PACKAGE CONTEXT (2026-06-20): This is the historical spike input for
> `SPIKE_VERDICT.md`. The extraction was later run on the isolated
> `spike_core/` crate, so use `SPIKE_VERDICT.md` for measured status. Current
> authority starts at `../../spec/README.md`, ADRs, and `../../TRACKER.md`.

Run `hax` on ONE self-contained Rust
verifier function, extract to F\*, judge whether the obligation is tractable.
This file names the function and the exact obligation. It is no longer the
measured status log.

## The named target

**`reviewed_fq12_account_carried_expected_response_syndrome`**
in `tools/parameter-search/src/production_reference.rs:37008`.

```rust
pub fn reviewed_fq12_account_carried_expected_response_syndrome(
    challenge: &[u8; CHALLENGE_BYTES],
    witness_commitment: &[u8; DIGEST_BYTES],
    proof_commitment: &[u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES],
) -> Result<[u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS], String> {
    let witness    = unpack_reviewed_fq12_account_carried_syndrome_digest(witness_commitment)?;
    let randomizer = unpack_reviewed_fq12_account_carried_syndrome_digest(proof_commitment)?;
    Ok(fq12_syndrome_add(
        &randomizer,
        &fq12_syndrome_mul(&reviewed_fq12_challenge_action_vector(challenge), &witness),
    ))
}
```

This is the literal RHS of the verification equation `z = a + action(c)·w` over
Fq12 (`randomizer` = `a`, `challenge_action_vector` = `action(c)`, `witness` =
`w`), checked for equality against the decoded response syndrome inside
`verify_reviewed_fq12_account_carried_sigma_equation`
(`production_reference.rs:38153`).

## Why it is a clean FIRST target

- **Self-contained arithmetic core.** Its transitive callees are all pure,
  small, fixed-size, panic-light integer code — no I/O, no trait objects, no
  generics, no `unsafe`:
  - `fq12_syndrome_add` (:46268) — 12-lane elementwise `add_mod`.
  - `fq12_syndrome_mul` (:46279) → `poly_mul_mod` (:46221) — schoolbook poly
    multiply then reduce mod the monic action polynomial (degree-12,
    `X^12+X+25`), then `fq12_syndrome_normalize` (:46259).
  - `add_mod`/`sub_mod`/`negate_mod` (:46168) — `% Q` modular ops, `Q=12289`.
  - `reviewed_fq12_challenge_action_vector` (:36992) → `uniform_mod_from_hash`
    (SHA-256 hash-to-field). **This is the one heavyweight callee** — see
    go/no-go below; the spike can stub `action(c)` as an abstract
    `c -> [u16;12]` oracle (it is a random oracle in spec §3, not algebraic).
  - `unpack_..._syndrome_digest` (:45993) — fixed-width bit unpacking of a
    32-byte digest into 12 canonical residues in `[0,Q)`.
- **Matches the spec equation 1:1.** Maps directly onto spec §3's
  `action : challenge → Fq12` and the relation `z = a + action(c)·w`.
- **Retires a real marker gate.** It is exactly the source-inspected
  "sigma-equation" path the marker gates assert by string search
  (`production_reference.rs:8023` `verify_reviewed_fq12_account_carried_sigma_equation`
  marker); a clean F\* extraction lets that gate be deleted (the stated *Go*
  outcome in `README.md`).

## The exact obligation an F\* extraction must prove (vs spec §3)

Extract `reviewed_fq12_account_carried_expected_response_syndrome` (with
`action(c)` abstracted as an opaque `action : challenge -> Fq12`) and prove the
F\* model satisfies, for all inputs that decode (i.e. the `Ok` branch):

> **OBLIGATION (spec §3 correspondence).**
> Let `a = decode(proof_commitment)`, `w = decode(witness_commitment)`,
> `act = action(challenge)`, all in `Fq12 ≅ (F_q)^12`, `q = 12289`, with field
> multiplication taken modulo `X^12 + X + 25`. Then the extracted function
> returns `a + act * w` computed in `Fq12`; equivalently, the `[u16;12]` result
> equals the coefficient vector of the polynomial product-plus-add reduced mod
> `(X^12+X+25, q)`, with every lane canonical in `[0, q)`.

Concretely the F\* proof must discharge:
1. **`add_mod`/`sub_mod` = field `+`/`−` mod q**, result `< q` (no `u16`/`u32`
   overflow: intermediates fit `u32` since `q < 2^14`, products `< 2^28`).
2. **`poly_mul_mod` = multiplication in `F_q[X]/(X^12+X+25)`** — the schoolbook
   convolution + monic reduction loop computes the quotient-ring product
   (the `assert_eq!(modulus.last()==Some(1))` monic precondition holds for the
   `..._ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH` constant).
3. **`unpack_..._syndrome_digest`** maps a canonical 32-byte digest to a residue
   vector with each lane `< q` (rejects non-canonical high bits / out-of-range
   residues — the `Err` paths).
4. **Composition** = `a + act*w` in `Fq12` (the spec §3 RHS), with `action`
   left abstract (random-oracle, not proven algebraic — that is the EC layer's
   job, not hax's).

## Go / No-go criteria

- **GO** — hax extracts the function + its pure callees to F\* with `action`
  abstracted, and obligations (1)–(4) are stated as F\* lemmas that `fstar.exe`
  (already installed) discharges (SMT/Z3) with reasonable effort. → adopt hax,
  delete the corresponding sigma-equation marker gate.
- **NO-GO** — hax chokes on:
  - the `Result<_, String>` / `?` error plumbing (string-typed errors),
  - `Vec<u16>` growth in `poly_mul_mod` (dynamic length, `pop`/`push`),
  - `Sha256` inside `uniform_mod_from_hash` / `chunks_exact` iterators,
  - the unbounded `for counter in 0_u32..` rejection-sampler loop
    (`unreachable!()`),
  → fall back to Cryptol+SAW or a Lean executable model anchored by `../kat/`.
  Mitigation to try first: extract only `fq12_syndrome_add` + `fq12_syndrome_mul`
  + `add_mod`/`sub_mod` over fixed `[u16;12]` arrays (no `Vec`, no SHA, no
  `Result`) as the minimal core; that is the highest-probability clean GO and is
  the recommended first extraction.

## Status

**COMPLETED AS A SPIKE INPUT.** Target named, obligation written, callees mapped.
The later measured result is `SPIKE_VERDICT.md`: extraction succeeded on the
isolated `spike_core/` crate and remains blocked only on proof-library/toolchain
cleanup before real F\* lemmas can replace source-inspection gates.
