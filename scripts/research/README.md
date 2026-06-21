# Descender Research Automation Scripts

Status: ACTIVE_RESEARCH_AUTOMATION_ENTRYPOINT

This directory contains local automation for research task scheduling, quality
review, and artifact generation. The scripts are not normative product
specifications. Current product authority starts in:

```text
../../spec/README.md
../../decisions/
../../TRACKER.md
../../research/ARCHIVE_INDEX.md
```

## Active Control Scripts

These scripts are part of the current research process:

| Script | Role |
|---|---|
| `director.py` | imports and dispatches pivot-aligned tasks |
| `launch_agent.py` | launches implementation/research agents with current authority context |
| `quality_check.py` | reviews completed agent work against current authority |
| `common.py` | shared state, artifact, and event helpers |
| `pivot_guard.py` | shared pivot-alignment classifier for queue import and dispatch |
| `audit_task_pivot_alignment.py` | checks queue/state tasks against the pivot guard |
| `quarantine_non_pivot_state.py` | moves stale local state tasks to `pivot_archived` |
| `check_research_automation_routing.py` | checks that active automation routes through current authority and pivot guards |
| `qrom_challenge_budget_memo.py` | regenerates the active Poseidon2 byte/mask QROM budget artifact |
| `qrom_security_loss_review.py` | regenerates arithmetic checks over the active QROM budget |
| `pre_third_party_qrom_frame_probe.py` | regenerates QROM frame-shape and target-horizon checks |
| `qrom_multitarget_margin.py` | regenerates the current multi-target fail-closed cap |
| `check_qrom_budget_sync.py` | checks regenerated QROM artifacts against the current Rust reference |
| `check_qrom_generated_artifact_drift.py` | fails if regeneration leaves tracked QROM JSON/Markdown artifacts dirty |
| `monitor_agents.py` | run-state inspection helper |
| `cleanup_paseo_agents.py` | local agent cleanup helper |

The active-control table is exact and mechanically checked. Adding a script to
this table means it is current automation and must be covered by
`scripts/research/check_research_automation_routing.py`. Scripts outside this
table are treated as legacy helpers, artifact reproducers, negative-control
generators, or migration tools. Their current baseline is capped by the same
check and should ratchet downward as old generators are removed or reclassified.

The director must reject pre-pivot work by default. Set
`DESCENDER_ALLOW_LEGACY_TASKS=1` only for deliberate archive, negative-control,
or regression work.

Queue import uses the same pivot guard as dispatch. Enabled tasks in
`research/queue/*.json` are imported only when they are pivot-aligned or when the
legacy override is set. Skipped imports are recorded as
`descender.queue.skipped` events.

CI and `scripts/check_build.sh` run:

```sh
python3 scripts/research/check_research_automation_routing.py
python3 scripts/research/audit_task_pivot_alignment.py --strict-queue
python3 scripts/research/qrom_challenge_budget_memo.py
python3 scripts/research/qrom_security_loss_review.py
python3 scripts/research/pre_third_party_qrom_frame_probe.py
python3 scripts/research/qrom_multitarget_margin.py
python3 scripts/research/check_qrom_budget_sync.py
python3 scripts/research/check_qrom_generated_artifact_drift.py
```

The QROM reports are generated under ignored `research/` report paths. The
tracked authority is the generator/checker chain and the build gate, not the
ignored report files themselves.

To inspect existing local state without making it a CI gate:

```sh
python3 scripts/research/audit_task_pivot_alignment.py --include-state
```

To remove stale non-running state tasks from the local scheduler:

```sh
python3 scripts/research/quarantine_non_pivot_state.py
```

Running tasks are not archived by default. Use `--include-running` only when the
corresponding agents have been stopped or intentionally abandoned.

## Legacy Artifact Generators

Many scripts in this directory generated reports for older campaigns such as
the 308-byte/Fq12 projection-carrier path, pre-signature projection route, or
May 2026 signature-readiness package. Those scripts are retained as historical
reproducers and negative-control tools.

They must not be treated as current product automation unless the task is
explicitly framed as:

- archive classification;
- stale-path cleanup;
- negative-control preservation;
- fail-closed regression evidence;
- migration of useful evidence into `spec/`, ADRs, or `TRACKER.md`.

New task generators should cite the current authority chain and should avoid
phrases that present legacy targets as selected deployment paths.

## Adding New Scripts

New scripts should state which current artifact they serve:

- a `spec/` requirement;
- an ADR;
- a `TRACKER.md` row;
- an active-support research note listed in `research/ARCHIVE_INDEX.md`;
- a negative-control or archive cleanup task.

If a script emits Markdown that could be mistaken for current status, the output
must include a status banner or be listed in `research/ARCHIVE_INDEX.md`.
