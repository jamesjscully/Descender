# Descent Interface TODOs

This is the owner checklist for preserving the original descent/stacks idea in
the current q12289 account-carried signature track without turning the proof
surface into a category-theory project.

## Position

The active construction is not a literal stack or sheaf-complex KEM anymore. It
is a concrete committed-sigma/account-carried q-ary quotient signature target.
The descent connection that remains worth keeping is the proof architecture:

```text
public quotient/account data
  + local verifier equations
  + account-carried hidden-link evidence
  => existence of a compatible hidden representative, or a named break
```

That is the finite descent condition for this scheme. Public representatives
alone are not semantic witnesses; they are local data over a public quotient.
The verifier is sound only when the local data glues back to hidden account
state.

## Non-Goals

- Do not make the deployment theorem depend on mathlib category theory.
- Do not replace the current SIS/committed-sigma review gates with stack
  terminology.
- Do not claim the scheme is deployable because the descent interface exists.
- Do not hide concrete blockers behind abstract vocabulary.

## Active TODOs

- [x] Add this note as the owner checklist for the descent-interface work.
- [x] Add a small Lean module for quotient/fiber/effective-descent vocabulary.
- [ ] Instantiate the Lean interface for the current
  `FullResponseDeploymentReviewCertificate` endpoint.
- [ ] Prove that the current stateful public-opening verifier acceptance path
  is an `accepts_glues_or_breaks` instance.
- [ ] Connect the rejected bare-public-quotient path to the generic
  singleton-fiber obstruction theorem.
- [ ] Add a short paragraph to the third-party review packet explaining that
  the descent interface is explanatory proof hygiene, not an additional
  security assumption.
- [ ] Once the selected account-carried verifier has accepting hidden-source
  proof semantics, update this note with the concrete accepted-public-data type
  and break-event names.

## Thin Lean Interface

The desired Lean abstraction has four layers:

1. `InPublicFiber`: a representative maps to the same quotient as the public
   data.
2. `LocallyValid`: the representative is in the public fiber and satisfies the
   local verifier equations.
3. `EffectiveDescentCertificate`: any locally valid representative either
   glues to account-backed hidden state or yields a named break.
4. `accepts_glues_or_breaks`: accepted public data therefore has an
   account-backed hidden witness or a named break.

This is enough to make the stacks/descent analogy precise for this repository:
local representatives are permitted, but semantic validity is the existence of
a compatible hidden object over the public quotient.

## Instantiation Plan

The first concrete target should use the existing q12289 quotient objects:

- public data: the stateful account-carried verifier relation;
- representative: the masked full-response byte vector;
- quotient: packed q-syndrome digest/account state;
- local validity: byte-local-delta representative semantics plus canonical
  q-syndrome digest match;
- hidden object: account-backed witness/randomizer state;
- account evidence: the reviewed 68-byte hidden-link account data;
- breaks: matrix sampler mismatch, extraction break, witness-hiding break,
  pad-PRF break, Fiat-Shamir frame mismatch, or bounded response-link failure.

The near-term proof obligation is not new cryptography. It is a repackaging of
the already named conditional-soundness endpoint into a reusable finite-descent
shape.
