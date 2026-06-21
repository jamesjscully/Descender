# Descender Research Notes

Status: ACTIVE_RESEARCH_ENTRYPOINT

This directory contains active research notes, historical design records,
negative controls, generated reports, and automation state. It is not the
project's normative product specification.

Current authority starts here:

```text
../spec/README.md
../decisions/
../TRACKER.md
research/ARCHIVE_INDEX.md
../scripts/research/README.md
```

Use `ARCHIVE_INDEX.md` before citing older research as current status. Files not
listed as active support should be treated as evidence or history unless the
current spec spine explicitly imports them.

## Automated Research Loop

This directory holds the user-space research workflow that Armature invokes.
Armature only provides triggers, logs, events, and run inspection; these scripts
own the Descender-specific policy.

The director now has a pivot guard: by default it dispatches only tasks that are
explicitly aligned with the chain-native PQ authorization objective or whose
prompt references active Path 2/spec/tracker work. Legacy KEM/formalization
tasks and old projection-carrier deployment tasks can remain in state for
history, archive cleanup, negative controls, or regression work, but they are
not launched as current-product work unless `DESCENDER_ALLOW_LEGACY_TASKS=1` is
set.

Script-level routing rules live in `../scripts/research/README.md`.

## Cryptanalysis Routing

Current cryptanalysis package routing lives in:

```text
research/cryptanalysis/README.md
```

Root-level cryptanalysis notes are evidence unless the current spec spine or an
ADR imports a claim.

The old internal cryptanalysis-readiness master tracker has been retired. Useful
work is now either imported into `../TRACKER.md`, retained as specific
cryptanalysis evidence notes, or covered by `research/ARCHIVE_INDEX.md`.

## Historical Review Readiness

The older q12289 review-readiness package has been retired from the worktree.
Its binding decision content is superseded by the ADRs and current specs, and
the one surviving byte-conformance fixture now lives at
`../tools/parameter-search/testdata/q12289_conformance_vectors.json` for legacy
verifier drift tests.

## State Discipline

Implementation agents do not mark research tasks as accepted. They can only move
a task to `implementation_completed`, which triggers a separate quality check.

The quality check is a separate run with its own local gates and an optional
review agent. A task is marked `accepted` only when the quality layer returns an
explicit machine-readable accept verdict and required local checks pass.
Otherwise the task remains `quality_needs_human`, `quality_rejected`, or
`quality_blocked`.

## Typical Commands

Use the local Armature binary if it is not on `PATH`:

```sh
export ARMATURE_BIN=/home/jack/code/armature/target/debug/armature
```

Validate the configuration:

```sh
$ARMATURE_BIN config check
```

Run the director manually:

```sh
$ARMATURE_BIN run research-director
```

Run the Lean check manually:

```sh
$ARMATURE_BIN run lean-check
```

Inspect research state:

```sh
./scripts/research/status.py
```

## Queue Files

Queue files live in `research/queue/*.json`. The director imports enabled tasks
from these files into `research/state/tasks.json`.

A task with `"enabled": false` is documented but not launched. Flip it to
`true` when it should enter the automated loop. Enabled queue tasks still pass
through the pivot guard before import; pre-pivot work is skipped unless
`DESCENDER_ALLOW_LEGACY_TASKS=1` is set or the task is explicitly archive,
negative-control, fail-closed regression, or migration work.

Audit enabled queue files with:

```sh
python3 scripts/research/audit_task_pivot_alignment.py --strict-queue
```

Inspect local state backlog with:

```sh
python3 scripts/research/audit_task_pivot_alignment.py --include-state
```

Archive stale non-running local state tasks with:

```sh
python3 scripts/research/quarantine_non_pivot_state.py
```

## Agent Configuration

The scripts use these environment variables:

```text
PASEO_BIN                  default: paseo
DESCENDER_IMPL_PROVIDER    default: codex/gpt-5.5
DESCENDER_IMPL_THINKING    default: high
DESCENDER_QUALITY_PROVIDER default: codex/gpt-5.5
DESCENDER_QUALITY_THINKING default: high
DESCENDER_IMPL_MODE        default: full-access
DESCENDER_QUALITY_MODE     default: full-access
DESCENDER_DIRECTOR_PROVIDER default: codex/gpt-5.5
DESCENDER_DIRECTOR_THINKING default: high
DESCENDER_DIRECTOR_MODE     default: full-access
DESCENDER_AGENT_WORKTREE   if 1, launch agents in Paseo worktrees
DESCENDER_ALLOW_DIRTY_WORKTREE_BASE
                           if 1, allow worktree launch from dirty main checkout
DESCENDER_AGENT_DRY_RUN    if 1, do not launch Paseo agents
DESCENDER_ALLOW_LEGACY_TASKS
                           if 1, allow pre-pivot legacy tasks to dispatch
DESCENDER_MAX_PARALLEL_TASKS parallel research capacity, default 4
DESCENDER_MAX_LAUNCHES     director launch limit per run, default max parallel
DESCENDER_MAX_FOLLOWUPS    director follow-up import limit, default 8
```

The local `paseo` binary may need a working CLI installation. If it is not
usable in the current desktop environment, set `DESCENDER_AGENT_DRY_RUN=1` while
testing the Armature wiring.

By default, agents run in the live checkout because the early Lean scaffold may
be uncommitted while another agent is still setting it up. Once the baseline is
committed, set `DESCENDER_AGENT_WORKTREE=1` to isolate implementation work.

The launcher records `git status --short` before and after each agent run.
Worktree mode is refused when the main checkout is dirty unless
`DESCENDER_ALLOW_DIRTY_WORKTREE_BASE=1` is set explicitly. The quality gate
blocks acceptance if the independent reviewer changes tracked or untracked
source state.
