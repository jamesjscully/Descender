#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import signal
import shutil
import subprocess
import sys
import time
from pathlib import Path

from common import (
    ROOT,
    RUNS_PATH,
    TASKS_PATH,
    append_jsonl,
    emit,
    git_status_short,
    read_json,
    set_status,
    update_task,
    utc_now,
)


def inspect_agent(paseo_bin: str, agent_id: str) -> tuple[int, dict, str]:
    try:
        process = subprocess.run(
            [paseo_bin, "inspect", "--json", agent_id],
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )
    except FileNotFoundError:
        return 127, {}, f"paseo binary not found: {paseo_bin}\n"

    try:
        payload = json.loads(process.stdout)
    except json.JSONDecodeError:
        payload = {}
    return process.returncode, payload, process.stdout


def append_agent_logs(paseo_bin: str, agent_id: str, output_path: Path) -> None:
    output_path.parent.mkdir(parents=True, exist_ok=True)
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


def implementation_candidates() -> list[dict]:
    state = read_json(TASKS_PATH, {"tasks": []})
    candidates = []
    for task in state.get("tasks", []):
        implementation = task.get("implementation", {})
        if (
            task.get("status") == "implementation_running"
            and implementation.get("detached")
            and implementation.get("agent_id")
        ):
            candidates.append(json.loads(json.dumps(task)))
    return candidates


def complete_task(task: dict, inspect_payload: dict, inspect_output: str, paseo_bin: str) -> bool:
    task_id = task["id"]
    implementation = task.get("implementation", {})
    agent_id = implementation.get("agent_id")
    output_path = ROOT / implementation.get("output_path", f"research/artifacts/{task_id}/implementation_output.txt")
    status_before = implementation.get("git_status_before", "")
    status_after = git_status_short()
    git_status_changed = status_after != status_before

    with output_path.open("a", encoding="utf-8") as output:
        output.write("\n\n$ paseo inspect --json ")
        output.write(str(agent_id))
        output.write("\n\n")
        output.write(inspect_output)
        output.write("\n")
    append_agent_logs(paseo_bin, str(agent_id), output_path)

    run_record = {
        "at": utc_now(),
        "task_id": task_id,
        "role": "implementation",
        "monitor": True,
        "provider": implementation.get("provider"),
        "mode": implementation.get("mode"),
        "detached": True,
        "agent_id": agent_id,
        "paseo_status": inspect_payload.get("Status") or inspect_payload.get("status"),
        "exit_code": 0,
        "git_status_changed": git_status_changed,
        "git_status_before": status_before,
        "git_status_after": status_after,
        "output_path": str(output_path.relative_to(ROOT)),
    }
    append_jsonl(RUNS_PATH, run_record)

    def finish(current: dict) -> None:
        if current.get("status") != "implementation_running":
            return
        current.setdefault("implementation", {}).update({
            "finished_at": utc_now(),
            "exit_code": 0,
            "output_path": str(output_path.relative_to(ROOT)),
            "git_status_after": status_after,
            "git_status_changed": git_status_changed,
            "monitor": run_record,
        })
        set_status(current, "implementation_completed", {
            "source": "monitor",
            "agent_id": agent_id,
            "output_path": str(output_path.relative_to(ROOT)),
            "git_status_changed": git_status_changed,
        })

    update_task(task_id, finish)
    emit("descender.agent.completed", {
        "task_id": task_id,
        "role": "implementation",
        "agent_id": agent_id,
        "output_path": str(output_path.relative_to(ROOT)),
    })
    return True


def check_once() -> dict:
    paseo_bin = os.environ.get("PASEO_BIN", shutil.which("paseo") or "paseo")
    checked = 0
    completed = []
    blocked = []

    for task in implementation_candidates():
        checked += 1
        agent_id = str(task.get("implementation", {}).get("agent_id"))
        exit_code, payload, raw = inspect_agent(paseo_bin, agent_id)
        if exit_code != 0:
            blocked.append({
                "task_id": task.get("id"),
                "agent_id": agent_id,
                "exit_code": exit_code,
                "output": raw[-2000:],
            })
            continue
        status = payload.get("Status") or payload.get("status")
        if status == "idle":
            complete_task(task, payload, raw, paseo_bin)
            completed.append(task.get("id"))

    record = {
        "at": utc_now(),
        "role": "agent-monitor",
        "checked": checked,
        "completed": completed,
        "blocked": blocked,
    }
    append_jsonl(RUNS_PATH, record)
    print(json.dumps(record, indent=2, sort_keys=True), flush=True)
    return record


def run_loop() -> int:
    interval_seconds = int(os.environ.get("DESCENDER_AGENT_MONITOR_INTERVAL_SECONDS", "15"))
    stop = False

    def request_stop(signum, frame) -> None:
        nonlocal stop
        stop = True

    signal.signal(signal.SIGTERM, request_stop)
    signal.signal(signal.SIGINT, request_stop)
    print(json.dumps({
        "at": utc_now(),
        "role": "agent-monitor",
        "mode": "loop",
        "interval_seconds": interval_seconds,
    }, indent=2, sort_keys=True), flush=True)

    while not stop:
        check_once()
        for _ in range(max(interval_seconds, 1)):
            if stop:
                break
            time.sleep(1)

    print(json.dumps({
        "at": utc_now(),
        "role": "agent-monitor",
        "mode": "loop",
        "stopped": True,
    }, indent=2, sort_keys=True), flush=True)
    return 0


def main() -> int:
    if "--loop" in sys.argv[1:] or os.environ.get("DESCENDER_AGENT_MONITOR_LOOP") == "1":
        return run_loop()
    check_once()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
