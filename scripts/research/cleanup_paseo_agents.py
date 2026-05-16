#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import shutil
import subprocess

from common import ROOT, RUNS_PATH, TASKS_PATH, append_jsonl, read_json, utc_now


TERMINAL_STATUSES = {
    "accepted",
    "quality_rejected",
    "quality_needs_human",
    "quality_blocked",
    "implementation_failed",
    "revision_failed",
}

ACTIVE_STATUSES = {
    "implementation_requested",
    "implementation_running",
    "implementation_completed",
    "quality_requested",
    "quality_running",
    "revision_running",
}


def run_json(cmd: list[str]) -> tuple[int, object | None, str]:
    process = subprocess.run(
        cmd,
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        check=False,
    )
    try:
        payload = json.loads(process.stdout)
    except json.JSONDecodeError:
        payload = None
    return process.returncode, payload, process.stdout


def archive_exit_code(code: int, output: str) -> int:
    if code == 0:
        return 0
    if "AGENT_ALREADY_ARCHIVED" in output:
        return 0
    return code


def collect_agent_ids() -> tuple[set[str], set[str]]:
    state = read_json(TASKS_PATH, {"tasks": []})
    protected: set[str] = set()
    candidates: set[str] = set()

    director_id = state.get("director", {}).get("agent_id")
    if director_id:
        protected.add(str(director_id))

    for task in state.get("tasks", []):
        status = task.get("status")
        impl_id = task.get("implementation", {}).get("agent_id")
        quality_id = task.get("quality", {}).get("agent_id")

        if status in ACTIVE_STATUSES and impl_id:
            protected.add(str(impl_id))
        if status in TERMINAL_STATUSES:
            if impl_id:
                candidates.add(str(impl_id))
            if quality_id:
                candidates.add(str(quality_id))

    if RUNS_PATH.exists():
        for line in RUNS_PATH.read_text(encoding="utf-8", errors="replace").splitlines():
            try:
                record = json.loads(line)
            except json.JSONDecodeError:
                continue
            agent_id = record.get("agent_id")
            if not agent_id:
                continue
            role = record.get("role")
            if role == "director":
                protected.add(str(agent_id))
            elif role in {"quality", "implementation", "revision"}:
                candidates.add(str(agent_id))

    return candidates - protected, protected


def main() -> int:
    paseo_bin = os.environ.get("PASEO_BIN", shutil.which("paseo") or "paseo")
    candidates, protected = collect_agent_ids()
    archived = []
    skipped = []

    for agent_id in sorted(candidates):
        if agent_id in protected:
            skipped.append({"agent_id": agent_id, "reason": "protected"})
            continue
        code, payload, output = run_json([paseo_bin, "archive", "--json", agent_id])
        record = {
            "agent_id": agent_id,
            "exit_code": archive_exit_code(code, output),
            "raw_exit_code": code,
            "output": output[-2000:],
        }
        archived.append(record)

    result = {
        "at": utc_now(),
        "role": "cleanup",
        "archived": archived,
        "skipped": skipped,
        "protected": sorted(protected),
    }
    append_jsonl(RUNS_PATH, result)
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if all(item["exit_code"] == 0 for item in archived) else 1


if __name__ == "__main__":
    raise SystemExit(main())
