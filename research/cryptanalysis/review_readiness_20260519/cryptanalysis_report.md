# Cryptanalysis Report

Date: 2026-05-19

Status: open review artifact. This file exists to close the Review Bundle Gate
navigation gap. It does not yet satisfy the Cryptanalysis Gate.

Related sources:

- [cryptanalysis_plan.md](cryptanalysis_plan.md): required attack campaign.
- [assumption_review.md](assumption_review.md): A4 and A9 risk statements.
- [target_spec.md](target_spec.md): exact frozen attack surface.
- [review_gate_checklist.md](review_gate_checklist.md): gate status dashboard.

## Scope

This report must eventually archive the substantive evidence behind the
cryptanalysis claims for the frozen
`validated-q12289-n48-m1-32-single-carrier` target. Until the sections below
contain actual attack scripts, parameters, outputs, and concrete estimates,
this report is incomplete and the Review Bundle Gate item "One cryptanalysis
report" remains open in substance even though the navigation artifact exists.

## Frozen Target Under Attack

- Profile: `validated-q12289-n48-m1-32-single-carrier`
- Public key: `172 B`
- Signature: `269 B`
- Modulus: `q = 12289`
- Repetitions: `7`
- Response bound: `224`
- Hash domain: `Descender.Signature.q12289.implicit_commitment.v1`
- Hash frame:
  `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`

Normative target definition: [target_spec.md](target_spec.md).

## Executive Status

| area | required evidence | current status |
|---|---|---|
| bad-vector/keygen-break rerun | archived rerun against frozen target with parameters, scripts, outputs, and negative result summary | open |
| lattice-reduction attacks | archived scripts, reduction configuration, hardware/runtime notes, and outputs | open |
| meet-in-the-middle/search attacks | documented attack model, search bounds, and results | open |
| transcript malleability attacks | documented parser/hash-frame/challenge mutation attempts and outcomes | open |
| concrete security table | one table with attack class, cost, memory, data, and confidence | open |
| known limitations | plain-language statement of what is not modeled or not claimed | open |

## Required Evidence By Gate Item

### 1. Bad-Vector / Keygen-Break Search

Gate item:
[review_gate_checklist.md](review_gate_checklist.md) "Bad-vector/keygen-break
search rerun against frozen target."

Required evidence:

- exact script or command entrypoints used;
- frozen target/profile identifier and any seeds or sampling bounds;
- witness-search parameters and stopping condition;
- output logs or summarized results with references to archived outputs;
- explicit statement whether any bad public view or narrow dual witness was
  found.

Current status:

- No archived rerun is attached here yet.
- Planning source: [cryptanalysis_plan.md](cryptanalysis_plan.md).
- Risk owner statement: A4 in [assumption_review.md](assumption_review.md).

### 2. Lattice-Reduction Attack Archive

Gate item:
[review_gate_checklist.md](review_gate_checklist.md) "Lattice-reduction attack
scripts and outputs archived."

Required evidence:

- attack model and target lattice statement;
- reduction tooling and version information;
- parameterization and reproducibility notes;
- output artifacts or summarized outcomes with archive references;
- interpretation of what the results do and do not show.

Current status:

- No archived scripts or outputs are linked from this bundle yet.
- Required by the concrete security section of
  [cryptanalysis_plan.md](cryptanalysis_plan.md).

### 3. Meet-in-the-Middle / Search Attacks

Gate item:
[review_gate_checklist.md](review_gate_checklist.md)
"Meet-in-the-middle/search attacks documented."

Required evidence:

- attack strategy and intended target property;
- search bounds, pruning assumptions, and memory model;
- final cost estimate or negative result;
- statement of confidence and caveats.

Current status:

- No substantive attack note is archived yet.

### 4. Transcript Malleability Attacks

Gate item:
[review_gate_checklist.md](review_gate_checklist.md)
"Transcript malleability attacks documented."

Required evidence:

- attempted mutations across challenge, response, public key, message, domain,
  profile, and reconstructed commitments;
- whether acceptance changed under any noncanonical or semantically distinct
  encoding;
- links to reject vectors or parser notes where relevant;
- mitigation notes if any mutation class succeeds.

Current status:

- The bundle has partial negative-vector context in
  [conformance_plan.md](conformance_plan.md), but this report does not yet
  archive a cryptanalysis-oriented malleability write-up.

### 5. Concrete Security Table

Gate item:
[review_gate_checklist.md](review_gate_checklist.md) "Concrete security table
written."

Required evidence:

- one consolidated table covering each attack class;
- best known cost, memory, data, and confidence;
- note of estimator or heuristic limitations;
- explicit distinction between formal bound and attack-based estimate.

Current status:

- Not written.

Placeholder table shape:

| attack class | best known cost | memory | data | confidence | notes |
|---|---:|---:|---:|---|---|
| keygen / bad witness search | pending | pending | pending | pending | pending |
| lattice reduction | pending | pending | pending | pending | pending |
| MITM / structured search | pending | pending | pending | pending | pending |
| transcript malleability | pending | pending | pending | pending | pending |

### 6. Known Limitations

Gate item:
[review_gate_checklist.md](review_gate_checklist.md) "Known limitations written
plainly."

Required evidence:

- explicit list of what this report does not claim;
- separation between mathematical attacks, implementation attacks, and proof
  modeling gaps;
- consistent wording with [target_theorem.md](target_theorem.md) and
  [assumption_review.md](assumption_review.md).

Current status:

- Not yet written as a dedicated section with final wording.

## Known Limitations Placeholder

This section is intentionally incomplete. At minimum, the final report must say
plainly that:

- the current Lean core proves the `2^-133` blind-challenge term, not full
  concrete deployment security by itself;
- A4 and A9 remain security-critical until frozen-target attack evidence is
  archived;
- side channels and implementation bugs are tracked separately in
  [implementation_audit.md](implementation_audit.md).

## Completion Rule

Do not mark this report complete until it contains substantive attack evidence,
not just links to plans. When completed, its conclusions must still match the
frozen target in [target_spec.md](target_spec.md) and the deployment claim
boundary in [target_theorem.md](target_theorem.md).
