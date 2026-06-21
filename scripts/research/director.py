#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
import shutil
import subprocess

from common import (
    ROOT,
    RUNS_PATH,
    append_jsonl,
    artifact_dir,
    emit,
    EVENTS_PATH,
    git_status_short,
    import_queue_files,
    locked_state,
    set_status,
    slug,
    utc_now,
    write_text,
)
from pivot_guard import task_pivot_aligned, task_pivot_alignment_reason, text_fields


ACTIVE_STATUSES = {
    "implementation_requested",
    "implementation_running",
    "implementation_completed",
    "quality_requested",
    "quality_running",
    "revision_running",
}

NO_EDIT_PHRASES = (
    "no source edits",
    "no source files are edited",
    "makes no source edits",
    "make no source edits",
    "do not edit files",
    "don't edit files",
    "without editing files",
    "no source files",
)

SOURCE_EDIT_PHRASES = (
    "edit files",
    "source edit",
    "source files are edited",
    "remove the `sorry`",
    "remove the sorry",
    "prove the core",
    "formalize",
    "refactor",
    "add the missing",
    "create the combined",
    "implementation",
)

READ_ONLY_KINDS = {
    "research",
    "review",
    "critique",
    "experiment",
    "risk-analysis",
}

FOLLOWUPS_RE = re.compile(
    r"DESCENDER_FOLLOWUPS_JSON_START\s*(.*?)\s*DESCENDER_FOLLOWUPS_JSON_END",
    re.S,
)
AGENT_ID_RE = re.compile(r"\b[0-9a-f]{7,8}\b|[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")


def parse_agent_id(text: str) -> str | None:
    try:
        payload = json.loads(text)
        if isinstance(payload, dict) and payload.get("agentId"):
            return str(payload["agentId"])
    except json.JSONDecodeError:
        pass
    match = AGENT_ID_RE.search(text)
    return match.group(0) if match else None


def append_agent_logs(paseo_bin: str, agent_id: str, output_path) -> None:
    with output_path.open("a", encoding="utf-8") as output:
        output.write(f"\n\n$ {paseo_bin} logs {agent_id}\n\n")
        output.flush()
        subprocess.run(
            [paseo_bin, "logs", agent_id],
            cwd=ROOT,
            stdout=output,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )


def latest_director_agent_id_from_runs() -> str | None:
    if not RUNS_PATH.exists():
        return None
    for line in reversed(RUNS_PATH.read_text(encoding="utf-8", errors="replace").splitlines()):
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        if record.get("role") == "director" and record.get("agent_id"):
            return str(record["agent_id"])
    return None


def director_agent_id() -> str | None:
    with locked_state() as state:
        configured = state.get("director", {}).get("agent_id")
    return configured or latest_director_agent_id_from_runs()


def save_director_agent_id(agent_id: str, source: str) -> None:
    with locked_state() as state:
        state.setdefault("director", {}).update({
            "agent_id": agent_id,
            "source": source,
            "updated_at": utc_now(),
        })


def read_artifact_excerpt(relative_path: str | None, limit: int) -> str:
    if not relative_path:
        return ""
    path = ROOT / relative_path
    if not path.exists():
        return ""
    return path.read_text(encoding="utf-8", errors="replace")[-limit:]


def task_brief(task: dict) -> dict:
    return {
        "id": task.get("id"),
        "status": task.get("status"),
        "track": task.get("track"),
        "kind": task.get("kind"),
        "priority": task.get("priority"),
        "title": task.get("title"),
        "summary": task.get("summary"),
        "quality": {
            "verdict": task.get("quality", {}).get("verdict"),
            "final_status": task.get("quality", {}).get("final_status"),
            "output_path": task.get("quality", {}).get("output_path"),
        },
    }


def task_parallel_safe(task: dict) -> bool:
    if task.get("parallel_safe") is True:
        return True
    if task.get("parallel_safe") is False:
        return False

    text = text_fields(task)
    if any(phrase in text for phrase in NO_EDIT_PHRASES):
        return True
    if any(phrase in text for phrase in SOURCE_EDIT_PHRASES):
        return False

    kinds = {
        part.strip().lower()
        for part in str(task.get("kind", "")).replace(",", "|").split("|")
        if part.strip()
    }
    return bool(kinds) and kinds.issubset(READ_ONLY_KINDS)


def active_tasks(state: dict) -> list[dict]:
    return [
        task for task in state.get("tasks", [])
        if task.get("status") in ACTIVE_STATUSES
    ]


def capacity_snapshot(state: dict) -> dict:
    active = active_tasks(state)
    unsafe_active = [task for task in active if not task_parallel_safe(task)]
    max_parallel = max(1, int(os.environ.get("DESCENDER_MAX_PARALLEL_TASKS", "4")))
    available = 0 if unsafe_active else max(0, max_parallel - len(active))
    return {
        "active": active,
        "unsafe_active": unsafe_active,
        "max_parallel": max_parallel,
        "available": available,
    }


def build_followup_prompt(state: dict, max_followups: int) -> str:
    tasks = state.get("tasks", [])
    accepted = [task for task in tasks if task.get("status") == "accepted"]
    queued = [task for task in tasks if task.get("status") == "queued"]
    terminal = [
        task for task in tasks
        if task.get("status") in {
            "accepted",
            "quality_rejected",
            "quality_needs_human",
            "quality_blocked",
            "implementation_failed",
            "revision_failed",
            "pivot_archived",
        }
    ]
    status_counts: dict[str, int] = {}
    for task in tasks:
        status = str(task.get("status", "unknown"))
        status_counts[status] = status_counts.get(status, 0) + 1
    capacity = capacity_snapshot(state)

    accepted_context = []
    for task in accepted[-3:]:
        accepted_context.append({
            "id": task.get("id"),
            "title": task.get("title"),
            "implementation_excerpt": read_artifact_excerpt(
                task.get("implementation", {}).get("output_path"),
                4000,
            ),
            "quality_excerpt": read_artifact_excerpt(
                task.get("quality", {}).get("output_path"),
                3000,
            ),
        })

    return f"""You are the autonomous research director for Descender.

Your job is to keep the project moving under its current pivot: Descender is now
organized as a chain-native post-quantum authorization stack. Future tasks must
advance the active authority spine, Path 2 prevalidated slots, Poseidon2/Plonky3
account creation, checked-core spend binding, policy/aggregation/rollup support,
or the migration/process work needed to keep those claims coherent.

Task status counts:
```json
{json.dumps(status_counts, indent=2, sort_keys=True)}
```

Parallelism policy:
```json
{json.dumps({
    "active_count": len(capacity["active"]),
    "unsafe_active_count": len(capacity["unsafe_active"]),
    "max_parallel_tasks": capacity["max_parallel"],
    "available_parallel_slots": capacity["available"],
}, indent=2, sort_keys=True)}
```

Highest priority queued work:
```json
{json.dumps([task_brief(task) for task in queued[:30]], indent=2, sort_keys=True)}
```

Recent terminal work:
```json
{json.dumps([task_brief(task) for task in terminal[-20:]], indent=2, sort_keys=True)}
```

Recent accepted outputs:
```json
{json.dumps(accepted_context, indent=2, sort_keys=True)}
```

Repository context:
- Current authority starts at `spec/README.md`, append-only ADRs in
  `decisions/`, and `TRACKER.md`.
- `research/ARCHIVE_INDEX.md` classifies old research, old trackers, negative
  controls, and active support docs.
- `scripts/research/README.md` classifies research automation. Legacy artifact
  generators are only for archive, negative-control, regression, or migration
  work unless `DESCENDER_ALLOW_LEGACY_TASKS=1` is set.
- Pre-pivot KEM/formalization seed plans have been retired from the worktree.
  Do not generate new work against that lineage unless the task explicitly
  explains how it supports the current Path 2 chain-native authorization stack.
- The workflow has a separate quality gate; make acceptance criteria concrete.
- The scheduler may run clearly read-only/research/review tasks in parallel.
- Source-editing Lean or implementation tasks are treated as exclusive unless a
  task explicitly declares `parallel_safe: true`.
- Do not edit files. Only propose follow-up tasks.
- Avoid duplicate tasks already present in current task state.
- Prefer tasks that compound useful information or unblock implementation.
- Every proposed task must include `"pivot_aligned": true` and should cite the
  current spec, ADR, or tracker item it advances.
- Do not propose work that treats the old 308-byte/Fq12 projection-carrier path
  as the selected deployment path. Such work is valid only as archive cleanup,
  negative-control preservation, fail-closed regression evidence, or migration
  into the current authority chain.

Return at most {max_followups} follow-up tasks as a JSON array between the
markers `DESCENDER_FOLLOWUPS_JSON_START` and `DESCENDER_FOLLOWUPS_JSON_END`.
Each task object must include `id`, `enabled`, `priority`, `track`, `kind`,
`title`, `summary`, `prompt`, `acceptance`, and `pivot_aligned`.
"""


def parse_followups(text: str, max_followups: int) -> list[dict]:
    matches = list(FOLLOWUPS_RE.finditer(text))
    if not matches:
        return []
    match = matches[-1]
    try:
        payload = json.loads(match.group(1))
    except json.JSONDecodeError:
        return []
    if not isinstance(payload, list):
        return []

    followups: list[dict] = []
    for item in payload[:max_followups]:
        if not isinstance(item, dict):
            continue
        if not item.get("title") or not item.get("prompt"):
            continue
        now = utc_now()
        task_id = slug(item.get("id") or item["title"])
        if task_id == "stable-kebab-case-id" or item.get("title") == "Short title":
            continue
        if "Precise instructions for the implementation/research agent" in str(item.get("prompt")):
            continue
        task = {
            **item,
            "id": task_id,
            "enabled": True,
            "status": "queued",
            "created_at": now,
            "updated_at": now,
            "history": [{
                "at": now,
                "event": "generated_by_director",
                "details": {"source": "followup_director"},
            }],
        }
        if not task_pivot_aligned(task):
            continue
        followups.append(task)
    return followups


def generate_followups(state: dict) -> list[str]:
    max_followups = int(os.environ.get("DESCENDER_MAX_FOLLOWUPS", "8"))
    if max_followups <= 0:
        return []

    art = artifact_dir("director-followups")
    timestamp = utc_now().replace(":", "").replace("+", "Z")
    prompt_path = art / f"prompt-{timestamp}.md"
    output_path = art / f"output-{timestamp}.txt"
    prompt = build_followup_prompt(state, max_followups)
    write_text(prompt_path, prompt)

    provider = os.environ.get("DESCENDER_DIRECTOR_PROVIDER", "codex/gpt-5.5")
    thinking = os.environ.get("DESCENDER_DIRECTOR_THINKING", "high")
    mode = os.environ.get("DESCENDER_DIRECTOR_MODE", "full-access")
    paseo_bin = os.environ.get("PASEO_BIN", shutil.which("paseo") or "paseo")
    status_before = git_status_short()
    agent_id = None
    invocation = "dry-run"

    if os.environ.get("DESCENDER_AGENT_DRY_RUN") == "1":
        output_path.write_text(
            "DESCENDER_AGENT_DRY_RUN=1; director agent not launched.\n"
            "DESCENDER_FOLLOWUPS_JSON_START\n[]\nDESCENDER_FOLLOWUPS_JSON_END\n",
            encoding="utf-8",
        )
        exit_code = 0
    else:
        existing_agent_id = director_agent_id()
        exit_code = 1
        with output_path.open("w", encoding="utf-8") as output:
            if existing_agent_id:
                send_cmd = [
                    paseo_bin,
                    "send",
                    "--json",
                    "--prompt-file",
                    str(prompt_path),
                    existing_agent_id,
                ]
                output.write(f"$ {' '.join(send_cmd)}\n\n")
                output.flush()
                try:
                    process = subprocess.run(
                        send_cmd,
                        cwd=ROOT,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.STDOUT,
                        text=True,
                        check=False,
                    )
                    exit_code = process.returncode
                    run_output = process.stdout
                except FileNotFoundError:
                    run_output = f"paseo binary not found: {paseo_bin}\n"
                    exit_code = 127
                output.write(run_output)
                output.write(f"\n[exit_code] {exit_code}\n")
                if exit_code == 0:
                    agent_id = existing_agent_id
                    invocation = "send"
            if exit_code != 0:
                run_cmd = [
                    paseo_bin,
                    "run",
                    "--json",
                    "--provider",
                    provider,
                    "--thinking",
                    thinking,
                    "--mode",
                    mode,
                    "--cwd",
                    str(ROOT),
                    "--title",
                    "Descender research director followups",
                    prompt,
                ]
                if existing_agent_id:
                    output.write(
                        "\n[director] stored director thread was unavailable; "
                        "creating a replacement director agent.\n\n"
                    )
                output.write(f"$ {' '.join(run_cmd[:-1])} <prompt>\n\n")
                output.flush()
                try:
                    process = subprocess.run(
                        run_cmd,
                        cwd=ROOT,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.STDOUT,
                        text=True,
                        check=False,
                    )
                    exit_code = process.returncode
                    run_output = process.stdout
                except FileNotFoundError:
                    run_output = f"paseo binary not found: {paseo_bin}\n"
                    exit_code = 127
                output.write(run_output)
                output.write(f"\n[exit_code] {exit_code}\n")
                agent_id = parse_agent_id(run_output)
                invocation = "run"
        if agent_id:
            save_director_agent_id(agent_id, invocation)
            append_agent_logs(paseo_bin, agent_id, output_path)

    status_after = git_status_short()
    transcript = output_path.read_text(encoding="utf-8", errors="replace")
    proposals = [] if status_after != status_before else parse_followups(transcript, max_followups)
    imported: list[str] = []

    with locked_state() as state:
        existing = {task.get("id") for task in state.get("tasks", [])}
        for task in proposals:
            if task["id"] in existing:
                continue
            state.setdefault("tasks", []).append(task)
            existing.add(task["id"])
            imported.append(task["id"])

    append_jsonl(RUNS_PATH, {
        "at": utc_now(),
        "role": "director",
        "provider": provider,
        "thinking": thinking,
        "mode": mode,
        "exit_code": exit_code,
        "agent_id": agent_id,
        "invocation": invocation,
        "git_status_changed": status_after != status_before,
        "prompt_path": str(prompt_path.relative_to(ROOT)),
        "output_path": str(output_path.relative_to(ROOT)),
        "imported_followups": imported,
    })
    return imported


def dispatch_requested(launched: int) -> None:
    with locked_state() as state:
        requested = [
            task for task in state.get("tasks", [])
            if task.get("status") == "implementation_requested"
        ][:launched]

    for task in requested:
        task_id = task["id"]
        emitted = emit("descender.agent.requested", {
            "task_id": task_id,
            "role": "implementation",
            "track": task.get("track"),
            "kind": task.get("kind"),
        })
        if emitted:
            print(f"requested implementation agent for {task_id}")
        else:
            with locked_state() as state:
                for current in state.get("tasks", []):
                    if current.get("id") == task_id and current.get("status") == "implementation_requested":
                        set_status(current, "dispatch_blocked", {
                            "reason": "armature event emit failed",
                        })
            print(f"failed to emit agent request for {task_id}; marked dispatch_blocked")


def request_queued_work(max_launches: int) -> int:
    launched = 0
    with locked_state() as state:
        capacity = capacity_snapshot(state)
        available = min(max_launches, capacity["available"])
        if available <= 0:
            active_ids = [task.get("id") for task in capacity["active"]]
            unsafe_ids = [task.get("id") for task in capacity["unsafe_active"]]
            print(json.dumps({
                "message": "parallel capacity is full; director will not launch more work",
                "active": active_ids,
                "unsafe_active": unsafe_ids,
                "max_parallel_tasks": capacity["max_parallel"],
            }, indent=2, sort_keys=True))
            return 0

        queued = [
            task for task in state.get("tasks", [])
            if task.get("status") == "queued"
        ]
        queued.sort(key=lambda task: (task.get("priority", 100), task.get("created_at", "")))
        skipped_for_pivot = []

        for task in queued:
            if launched >= available:
                break
            if not task_pivot_aligned(task):
                _aligned, reason = task_pivot_alignment_reason(task)
                skipped_for_pivot.append({
                    "id": task.get("id"),
                    "reason": reason,
                })
                append_jsonl(EVENTS_PATH, {
                    "at": utc_now(),
                    "event_type": "descender.task.skipped",
                    "payload": {
                        "task_id": task.get("id"),
                        "reason": reason,
                        "source": "director",
                    },
                })
                continue
            if not task_parallel_safe(task) and capacity["active"]:
                continue
            set_status(task, "implementation_requested", {
                "source": "director",
                "pivot_guard": "passed",
            })
            launched += 1
            capacity["active"].append(task)
            if not task_parallel_safe(task):
                capacity["unsafe_active"].append(task)
                break

        if skipped_for_pivot and launched == 0:
            print(json.dumps({
                "message": "queued work exists, but the pivot guard skipped legacy/non-aligned tasks",
                "skipped_count": len(skipped_for_pivot),
                "sample": skipped_for_pivot[:20],
                "override": "set DESCENDER_ALLOW_LEGACY_TASKS=1 to dispatch legacy tasks",
            }, indent=2, sort_keys=True))

    if launched:
        dispatch_requested(launched)
    return launched


def main() -> int:
    imported = import_queue_files()
    if imported:
        print(f"imported tasks: {', '.join(imported)}")

    max_launches = int(os.environ.get("DESCENDER_MAX_LAUNCHES", os.environ.get("DESCENDER_MAX_PARALLEL_TASKS", "4")))
    with locked_state() as state:
        capacity = capacity_snapshot(state)
        if capacity["available"] <= 0:
            print(json.dumps({
                "message": "parallel capacity is full; director will not launch more work",
                "active": [task.get("id") for task in capacity["active"]],
                "unsafe_active": [task.get("id") for task in capacity["unsafe_active"]],
                "max_parallel_tasks": capacity["max_parallel"],
            }, indent=2, sort_keys=True))
            return 0
        snapshot = json.loads(json.dumps(state))
        queued_count = sum(1 for task in state.get("tasks", []) if task.get("status") == "queued")

    if queued_count:
        launched = request_queued_work(max_launches)
        if launched:
            print(f"launched {launched} queued task(s); skipping follow-up generation this cycle")
            return 0

    generated = generate_followups(snapshot)
    if generated:
        print(f"generated follow-up tasks: {', '.join(generated)}")

    launched = request_queued_work(max_launches)
    if launched == 0:
        print("no queued tasks ready")
        return 0

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
