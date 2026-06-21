#!/usr/bin/env python3
from __future__ import annotations

import argparse

from common import append_jsonl, locked_state, set_status, utc_now, EVENTS_PATH
from pivot_guard import task_pivot_alignment_reason


DEFAULT_QUARANTINE_STATUSES = {
    "queued",
    "implementation_requested",
    "implementation_completed",
    "quality_requested",
}

RUNNING_STATUSES = {
    "implementation_running",
    "quality_running",
    "revision_running",
}


def quarantine(include_running: bool, dry_run: bool) -> list[dict]:
    changed: list[dict] = []
    eligible = set(DEFAULT_QUARANTINE_STATUSES)
    if include_running:
        eligible |= RUNNING_STATUSES

    with locked_state() as state:
        for task in state.get("tasks", []):
            status = task.get("status")
            if status not in eligible:
                continue
            aligned, reason = task_pivot_alignment_reason(task)
            if aligned:
                continue

            record = {
                "task_id": task.get("id"),
                "previous_status": status,
                "reason": reason,
                "title": task.get("title"),
            }
            changed.append(record)
            if dry_run:
                continue

            task.setdefault("pivot_quarantine", {}).update({
                "at": utc_now(),
                "previous_status": status,
                "reason": reason,
            })
            set_status(task, "pivot_archived", {
                "reason": reason,
                "previous_status": status,
                "source": "quarantine_non_pivot_state.py",
            })

    if changed and not dry_run:
        append_jsonl(EVENTS_PATH, {
            "at": utc_now(),
            "event_type": "descender.state.pivot_archived",
            "payload": {
                "count": len(changed),
                "include_running": include_running,
                "tasks": changed,
            },
        })
    return changed


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Move non-pivot research state tasks out of active scheduling.",
    )
    parser.add_argument(
        "--include-running",
        action="store_true",
        help="also archive implementation_running, quality_running, and revision_running tasks",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="show tasks that would be archived without modifying state",
    )
    args = parser.parse_args()

    changed = quarantine(args.include_running, args.dry_run)
    action = "Would archive" if args.dry_run else "Archived"
    print(f"{action} {len(changed)} non-pivot state tasks")
    for item in changed:
        print(
            "- {task_id}: {previous_status} -> pivot_archived ({reason}) {title}".format(
                task_id=item.get("task_id"),
                previous_status=item.get("previous_status"),
                reason=item.get("reason"),
                title=item.get("title") or "",
            )
        )
    if changed and args.dry_run:
        print("Run without --dry-run to update research/state/tasks.json.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
