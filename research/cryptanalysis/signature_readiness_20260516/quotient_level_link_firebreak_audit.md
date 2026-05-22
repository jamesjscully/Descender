# Quotient-Level Link Firebreak Audit

Status: `firebreak_passed`.

This audit checks that sidecar quotient-link modules are not imported by the deployable Lean aggregate.

## Summary

| item | value |
|---|---|
| passed | `True` |
| root sidecar imports | `[]` |
| violations | `0` |

Keep Descender.HiddenCommittedQuotientLink and Descender.DescentInterface out of the deployable import aggregate unless a later promotion gate explicitly recommends migration.

## Allowed Imports

| module | allowed importers |
|---|---|
| `Descender.DescentInterface` | `['Descender/HiddenCommittedQuotientLink.lean']` |
| `Descender.HiddenCommittedQuotientLink` | `[]` |

## Observed Sidecar Imports

| path | sidecar imports |
|---|---|
| `Descender/HiddenCommittedQuotientLink.lean` | `['Descender.DescentInterface']` |

## Violations

No violations.
