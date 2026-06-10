# Descender Automated Research Loop

This directory holds the user-space research workflow that Armature invokes.
Armature only provides triggers, logs, events, and run inspection; these scripts
own the Descender-specific policy.

## Cryptanalysis-First Master Tracker

The single instrument for measuring progress toward internal
cryptanalysis-readiness (do all the cryptanalysis a third party would, before
external submission) is:

```text
research/cryptanalysis/cryptanalysis_first_master_tracker.md
```

It defines the recalibrated done-bar, the process change, the workstreams
(assumption ledger → reduce-to-standard → verified estimator → empirical
structural attacks → red-team dossier → aggregation decision → the real
reduction), and a scorecard. The packages below are inputs it consolidates.

## Review Readiness

The current deployable-cryptography preparation package for the q12289
implicit-commitment signature candidate is:

```text
research/cryptanalysis/review_readiness_20260519/README.md
```

That package freezes the target byte layout, maps the Lean proof endpoints,
records remaining assumptions, and lists the conformance, cryptanalysis, and
implementation-hardening gates required before third-party review.

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
`true` when it should enter the automated loop.

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
