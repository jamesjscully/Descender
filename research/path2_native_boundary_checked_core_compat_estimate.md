# Path 2 Native Boundary Checked-Core Compatibility Estimate

Status: first-pass estimate; semantic row class required, exact checked-core row mapping pending.

Purpose: stress-test `r_checked_core_residue_compatibility`, one of the
highest-risk open row groups in `NativeBoundaryTraceRelationV1`.

## Rule

Account creation must not re-run the spend verifier. It only needs to bind the
creation-time hidden representative to the checked-core residue interface that
the spend proof will later consume.

So this group should be residue compatibility, not checked-core verifier logic.

Lean now requires `checkedCoreResidueCompatibilityRow` coverage through
`TraceRelationCoversNativeBoundary`, so relation knowledge cannot be discharged
by a profile that leaves the checked-core handoff implicit.

## Required Semantics

The trace must expose, extractor-side, the residues that make the account
creation opening compatible with checked-core spend semantics:

| item | relation role |
|---|---|
| 12 checked-core carried residues | equality to packed/opening trace coordinates |
| active-lane source | same account witness / projection relation used by creation |
| spend handle independence | no dependence on spend message or challenge |
| authorization boundary | compatibility with `p_slot_leaf(b)` / `p_slot_authorization(b)` as public byte binding, not digest preimage |

## Candidate Row Decomposition

| subgroup | estimate | rationale |
|---|---:|---|
| `checked_core_residue_links` | 12 | one q12289 equality/link row per carried checked-core residue |
| `checked_core_domain_independence` | 0 | enforced by transcript/domain separation and absence of spend challenge columns |
| `authorization_digest_binding` | 0 | outer byte binding; do not compute SHA inside hidden relation |

Total:

```text
r_checked_core_residue_compatibility = 12
```

## Why This Is Plausible

The checked-core signature remains a downstream spend object. Account creation
only precommits the residue/opening boundary. If the spend digest or challenge
must be recomputed inside account creation, the design has crossed into a
different theorem and the row estimate is invalid.

## Failure Modes

This estimate fails if:

- account creation must verify checked-core challenge equations;
- `slot_authorization_digest` must be recomputed as SHA inside the trace;
- checked-core compatibility depends on the future spend message;
- the 12 carried residues are not sufficient to link account creation to spend
  handoff;
- compatibility requires public raw opening of hidden account/projection lanes.

Any failure here is serious because it means the native-boundary account object
does not cleanly separate admission from spend.

## Decision

Keep:

```text
r_checked_core_residue_compatibility = 12
```

for the current algebraic estimate.

Mark it `conditional_on_residue_boundary_only`: valid only if checked-core
handoff consumes the admitted residue boundary and all digest/challenge binding
stays in the spend theorem and QROM ledger.

## Next Move

Stress-test role restriction rows:

```text
r_role_restriction_*
```

The question is whether role handles are pure proof-family metadata /
subcommitment openings, or whether each role handle needs extra algebraic link
rows in the trace.

## Non-Claims

- This does not prove spend handoff.
- This does not prove the 12-residue interface is sufficient.
- This does not implement checked-core compatibility.
