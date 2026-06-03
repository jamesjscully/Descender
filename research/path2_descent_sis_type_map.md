# Path 2 Descent/SIS Type Map

Status: first descent/SIS obligation row closed at interface level.

## Scope

This artifact closes `p2_sis_public_quotient_types` for the
`Path2DescentSisCommitmentV1` proof track. It does not prove SIS binding,
hiding, norm/range soundness, Fiat-Shamir security, extraction, or no-export
behavior.

Executable/formal artifact:

```text
Descender/Path2DescentSIS.lean
```

Build command:

```text
lake build Descender.Path2DescentSIS
```

## Type Map

| descent role | Lean name | Path 2 meaning |
|---|---|---|
| public quotient | `MaskedProjection` | six q12289 residues from packed witness lanes 12..17 |
| public object | `Public` | leaf digest, `creation_proof_binding_digest`, proof payload digest, relation transcript digest, and masked projection quotient |
| local representative | `Representative` | pad/projection/opening commitments, proof transcript commitment, and representative masked projection |
| hidden object | `HiddenOpening` | pad lanes, account-projection lanes, opening tuple witness, and blindings |
| account evidence | `AccountEvidence` | account id, public-key identity, slot id, verifier version, account state root, and lifecycle anchor |
| break event | `BreakEvent` | named descent/SIS payload break bucket |
| predicate package | `Predicates` | abstract hooks for representative opening, hidden representation, account backing, and break export |
| descent instance | `localQuotientDescent` | concrete `LocalQuotientDescent` over the Path 2 types |

## Quotient Rule

The public quotient is exactly the masked projection vector:

```text
quotientOfRepresentative(rep) = rep.maskedProjection
quotientOfPublic(pub) = pub.maskedProjection
```

Lean theorem:

```text
Path2DescentSIS.locallyValid_implies_maskedProjection_eq
```

states that local validity implies the representative and public object are in
the same masked-projection fiber.

## Wrapper Theorem

Lean theorem:

```text
Path2DescentSIS.accepts_glues_or_breaks
```

specializes the generic descent wrapper to the Path 2 type map:

```text
accepted public payload
  -> descent data
  -> hidden account-backed opening
     or named Path 2 descent/SIS break
```

This is the interface closure needed before the concrete SIS commitment is
specified.

## Break Buckets

`BreakEvent` currently names:

- `sisBindingBreak`;
- `samplerHidingBreak`;
- `publicProjectionBreak`;
- `normOrRangeBreak`;
- `fiatShamirOrDomainBreak`;
- `extractorSoundnessBreak`;
- `noExportBreak`.

These map to the remaining open rows in
`research/path2_descent_sis_obligation_matrix.md`.

## Remaining Work

The type map is intentionally abstract over `Predicates`. The next rows must
define:

- role-separated matrix domains and byte encodings;
- concrete representative opening predicate;
- SIS/module-SIS binding assumption;
- hiding/sampler theorem;
- non-public link-equation guard;
- norm/range controls;
- Fiat-Shamir/QROM frame;
- candidate-specific extractor;
- public serialization and reject-path no-export tests.
