# ADR-0007: Research Automation Follows Current Authority

Status: Accepted
Date: 2026-06-20

## Context

The automated research loop still had queued and prompt-level assumptions from
the pre-pivot KEM/descent program. Leaving those defaults in place risks
creating new work that looks current while ignoring the active Path 2
chain-native PQ authorization objective.

## Decision

Research automation must default to the current authority chain:

```text
spec/README.md
decisions/
TRACKER.md
research/ARCHIVE_INDEX.md
```

The director may dispatch only tasks that are explicitly marked
`pivot_aligned: true` or whose prompt clearly advances active Path 2/spec/tracker
work. Legacy KEM/formalization tasks remain in state for history, but dispatch
requires an explicit override.

## Consequences

- `scripts/research/director.py` enforces a pivot-alignment guard.
- Agent and reviewer prompts cite the current authority chain before legacy
  planning documents.
- `DESCENDER_ALLOW_LEGACY_TASKS=1` is the escape hatch for deliberately running
  legacy tasks.
- New generated follow-up tasks must cite the current spec, ADR, archive index,
  or tracker item they advance.

## Supersedes

This ADR supersedes the implicit automation behavior that treated
`research_program.md` and `lean_proof_plan.md` as current project authority.

## Superseded By

None.

## Affected Specs / Tracker Items

- `research/README.md`
- `scripts/research/README.md`
- `TRACKER.md`: `PROC-AUTO-001`, `PROC-AUTO-002`
