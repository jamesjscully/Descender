#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import signal
import sys
import time
from datetime import datetime, timedelta, timezone

from common import EVENTS_PATH, RUNS_PATH, TASKS_PATH, append_jsonl, emit, read_json, utc_now
from pivot_guard import task_pivot_aligned


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

ACTIVITY_EVENTS = {
    "descender.agent.requested",
    "descender.agent.completed",
    "descender.agent.failed",
    "descender.quality.completed",
}

ACTIVITY_ROLES = {
    "director",
    "implementation",
    "quality",
    "revision",
}


def parse_time(value: str | None) -> datetime | None:
    if not value:
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None


def jsonl_records(path) -> list[dict]:
    if not path.exists():
        return []
    records = []
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(record, dict):
            records.append(record)
    return records


def text_fields(task: dict) -> str:
    parts = [
        str(task.get("track", "")),
        str(task.get("kind", "")),
        str(task.get("title", "")),
        str(task.get("summary", "")),
        str(task.get("prompt", "")),
    ]
    parts.extend(str(item) for item in task.get("acceptance", []))
    return "\n".join(parts).lower()


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


def has_schedulable_queued_task(active: list[dict], queued: list[dict]) -> bool:
    unsafe_active = [task for task in active if not task_parallel_safe(task)]
    if unsafe_active:
        return False
    max_parallel = max(1, int(os.environ.get("DESCENDER_MAX_PARALLEL_TASKS", "4")))
    if len(active) >= max_parallel:
        return False
    if not active:
        return bool(queued)
    return any(task_parallel_safe(task) for task in queued)


def latest_activity_at(state: dict) -> datetime | None:
    times: list[datetime] = []
    for task in state.get("tasks", []):
        for key in ("updated_at", "created_at"):
            parsed = parse_time(task.get(key))
            if parsed:
                times.append(parsed)
        for entry in task.get("history", []):
            parsed = parse_time(entry.get("at"))
            if parsed:
                times.append(parsed)

    for record in jsonl_records(EVENTS_PATH):
        if record.get("event_type") in ACTIVITY_EVENTS:
            parsed = parse_time(record.get("at"))
            if parsed:
                times.append(parsed)

    for record in jsonl_records(RUNS_PATH):
        if record.get("role") in ACTIVITY_ROLES:
            parsed = parse_time(record.get("at"))
            if parsed:
                times.append(parsed)

    return max(times) if times else None


def check_once() -> dict:
    state = read_json(TASKS_PATH, {"tasks": []})
    active = [
        task for task in state.get("tasks", [])
        if task.get("status") in ACTIVE_STATUSES
    ]
    queued = [
        task for task in state.get("tasks", [])
        if task.get("status") == "queued" and task_pivot_aligned(task)
    ]
    threshold_minutes = int(os.environ.get("DESCENDER_WATCHDOG_IDLE_MINUTES", "15"))
    now = datetime.now(timezone.utc)
    last_activity = latest_activity_at(state)
    idle_for = None if not last_activity else now - last_activity

    should_kick = (
        has_schedulable_queued_task(active, queued)
        and (last_activity is None or idle_for >= timedelta(minutes=threshold_minutes))
    )
    record = {
        "at": utc_now(),
        "role": "watchdog",
        "active_count": len(active),
        "queued_count": len(queued),
        "schedulable_queued": has_schedulable_queued_task(active, queued),
        "idle_seconds": None if idle_for is None else int(idle_for.total_seconds()),
        "threshold_seconds": threshold_minutes * 60,
        "kicked": False,
    }

    if should_kick:
        kicked = emit("descender.director.kick", {
            "reason": "watchdog_idle_timeout",
            "idle_seconds": record["idle_seconds"],
            "queued_count": len(queued),
        })
        record["kicked"] = kicked

    append_jsonl(RUNS_PATH, record)
    print(json.dumps(record, indent=2, sort_keys=True))
    return record


def run_loop() -> int:
    interval_seconds = int(os.environ.get("DESCENDER_WATCHDOG_INTERVAL_SECONDS", "60"))
    stop = False

    def request_stop(signum, frame) -> None:
        nonlocal stop
        stop = True

    signal.signal(signal.SIGTERM, request_stop)
    signal.signal(signal.SIGINT, request_stop)
    print(json.dumps({
        "at": utc_now(),
        "role": "watchdog",
        "mode": "loop",
        "interval_seconds": interval_seconds,
    }, indent=2, sort_keys=True), flush=True)

    while not stop:
        check_once()
        sys.stdout.flush()
        for _ in range(max(interval_seconds, 1)):
            if stop:
                break
            time.sleep(1)

    print(json.dumps({
        "at": utc_now(),
        "role": "watchdog",
        "mode": "loop",
        "stopped": True,
    }, indent=2, sort_keys=True), flush=True)
    return 0


def main() -> int:
    if "--loop" in sys.argv[1:] or os.environ.get("DESCENDER_WATCHDOG_LOOP") == "1":
        return run_loop()
    check_once()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
