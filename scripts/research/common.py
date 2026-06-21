#!/usr/bin/env python3
from __future__ import annotations

import contextlib
import fcntl
import json
import os
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable

from pivot_guard import task_pivot_alignment_reason


ROOT = Path(os.environ.get("ARMATURE_WORKSPACE_ROOT", Path(__file__).resolve().parents[2]))
RESEARCH = ROOT / "research"
STATE_DIR = RESEARCH / "state"
QUEUE_DIR = RESEARCH / "queue"
ARTIFACTS_DIR = RESEARCH / "artifacts"
TASKS_PATH = STATE_DIR / "tasks.json"
EVENTS_PATH = STATE_DIR / "events.jsonl"
RUNS_PATH = STATE_DIR / "agent-runs.jsonl"
LOCK_PATH = STATE_DIR / ".state.lock"


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def ensure_dirs() -> None:
    for path in (STATE_DIR, QUEUE_DIR, ARTIFACTS_DIR):
        path.mkdir(parents=True, exist_ok=True)


def slug(value: str) -> str:
    cleaned = re.sub(r"[^A-Za-z0-9._-]+", "-", value.strip()).strip("-")
    return cleaned.lower() or "task"


def read_json(path: Path, default: Any) -> Any:
    if not path.exists():
        return default
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    with tmp.open("w", encoding="utf-8") as handle:
        json.dump(value, handle, indent=2, sort_keys=True)
        handle.write("\n")
    tmp.replace(path)


def append_jsonl(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(value, sort_keys=True) + "\n")


@contextlib.contextmanager
def locked_state() -> Any:
    ensure_dirs()
    with LOCK_PATH.open("a+", encoding="utf-8") as lock:
        fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
        state = read_json(TASKS_PATH, {"tasks": [], "updated_at": utc_now()})
        try:
            yield state
        finally:
            state["updated_at"] = utc_now()
            write_json(TASKS_PATH, state)
            fcntl.flock(lock.fileno(), fcntl.LOCK_UN)


def find_task(state: dict[str, Any], task_id: str) -> dict[str, Any] | None:
    for task in state.get("tasks", []):
        if task.get("id") == task_id:
            return task
    return None


def history(task: dict[str, Any], event: str, details: dict[str, Any] | None = None) -> None:
    task.setdefault("history", []).append({
        "at": utc_now(),
        "event": event,
        "details": details or {},
    })
    task["updated_at"] = utc_now()


def set_status(task: dict[str, Any], status: str, details: dict[str, Any] | None = None) -> None:
    previous = task.get("status")
    task["status"] = status
    history(task, f"status:{previous}->{status}", details)


def import_queue_files() -> list[str]:
    ensure_dirs()
    imported: list[str] = []
    with locked_state() as state:
        existing = {task.get("id") for task in state.get("tasks", [])}
        for path in sorted(QUEUE_DIR.glob("*.json")):
            items = read_json(path, [])
            if isinstance(items, dict):
                items = items.get("tasks", [])
            for item in items:
                if not item.get("enabled", True):
                    continue
                task_id = slug(item.get("id") or item.get("title") or path.stem)
                if task_id in existing:
                    continue
                aligned, reason = task_pivot_alignment_reason(item)
                if not aligned:
                    append_jsonl(EVENTS_PATH, {
                        "at": utc_now(),
                        "event_type": "descender.queue.skipped",
                        "payload": {
                            "task_id": task_id,
                            "queue_file": str(path.relative_to(ROOT)),
                            "reason": reason,
                        },
                    })
                    continue
                now = utc_now()
                task = {
                    **item,
                    "id": task_id,
                    "status": "queued",
                    "created_at": now,
                    "updated_at": now,
                    "history": [{
                        "at": now,
                        "event": "imported",
                        "details": {"queue_file": str(path.relative_to(ROOT))},
                    }],
                }
                state.setdefault("tasks", []).append(task)
                existing.add(task_id)
                imported.append(task_id)
    return imported


def event_payload() -> dict[str, Any]:
    if os.environ.get("ARMATURE_PAYLOAD_JSON"):
        return json.loads(os.environ["ARMATURE_PAYLOAD_JSON"])
    raw_event = os.environ.get("ARMATURE_EVENT_JSON")
    if raw_event:
        event = json.loads(raw_event)
        if isinstance(event, dict) and isinstance(event.get("payload"), dict):
            return event["payload"]
        return event if isinstance(event, dict) else {}
    event_path = os.environ.get("ARMATURE_EVENT_PATH")
    if event_path:
        event = read_json(Path(event_path), {})
        if isinstance(event, dict) and isinstance(event.get("payload"), dict):
            return event["payload"]
        return event if isinstance(event, dict) else {}
    return {}


def armature_bin() -> str | None:
    configured = os.environ.get("ARMATURE_BIN")
    if configured:
        return configured
    local = Path("/home/jack/code/armature/target/debug/armature")
    if local.exists():
        return str(local)
    return "armature"


def emit(event_type: str, payload: dict[str, Any]) -> bool:
    record = {
        "at": utc_now(),
        "event_type": event_type,
        "payload": payload,
    }
    append_jsonl(EVENTS_PATH, record)
    binary = armature_bin()
    if not binary:
        print(json.dumps(record), file=sys.stderr)
        return False
    cmd = [binary, "emit", event_type]
    if os.environ.get("ARMATURE_CORRELATION_ID"):
        cmd.extend(["--correlation", os.environ["ARMATURE_CORRELATION_ID"]])
    cmd.extend(["--json", json.dumps(payload)])
    try:
        process = subprocess.run(
            cmd,
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        return process.returncode == 0
    except FileNotFoundError:
        print(f"armature binary not found for event {event_type}", file=sys.stderr)
        return False


def run_capture(cmd: list[str], cwd: Path, log_path: Path, timeout: int | None = None) -> int:
    log_path.parent.mkdir(parents=True, exist_ok=True)
    with log_path.open("w", encoding="utf-8") as log:
        log.write(f"$ {' '.join(cmd)}\n\n")
        log.flush()
        process = subprocess.run(
            cmd,
            cwd=cwd,
            stdout=log,
            stderr=subprocess.STDOUT,
            text=True,
            timeout=timeout,
            check=False,
        )
        log.write(f"\n[exit_code] {process.returncode}\n")
        return process.returncode


def git_status_short(cwd: Path = ROOT) -> str:
    process = subprocess.run(
        ["git", "status", "--short"],
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        check=False,
    )
    return process.stdout


def artifact_dir(task_id: str) -> Path:
    path = ARTIFACTS_DIR / slug(task_id)
    path.mkdir(parents=True, exist_ok=True)
    return path


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def load_task_or_die(task_id: str) -> dict[str, Any]:
    with locked_state() as state:
        task = find_task(state, task_id)
        if task is None:
            raise SystemExit(f"unknown task id: {task_id}")
        return json.loads(json.dumps(task))


def update_task(task_id: str, mutator: Callable[[dict[str, Any]], None]) -> dict[str, Any]:
    with locked_state() as state:
        task = find_task(state, task_id)
        if task is None:
            raise SystemExit(f"unknown task id: {task_id}")
        mutator(task)
        return json.loads(json.dumps(task))
