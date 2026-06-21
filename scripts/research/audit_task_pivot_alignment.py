#!/usr/bin/env python3
from __future__ import annotations

import argparse

from common import QUEUE_DIR, ROOT, TASKS_PATH, read_json, slug
from pivot_guard import task_pivot_alignment_reason


ACTIVE_STATUSES = {
    "queued",
    "implementation_requested",
    "implementation_running",
    "implementation_completed",
    "quality_requested",
    "quality_running",
    "revision_running",
}


def queue_issues() -> list[dict]:
    issues: list[dict] = []
    for path in sorted(QUEUE_DIR.glob("*.json")):
        items = read_json(path, [])
        if isinstance(items, dict):
            items = items.get("tasks", [])
        for item in items:
            if not isinstance(item, dict) or not item.get("enabled", True):
                continue
            aligned, reason = task_pivot_alignment_reason(item)
            if aligned:
                continue
            issues.append({
                "task_id": slug(item.get("id") or item.get("title") or path.stem),
                "queue_file": path.relative_to(ROOT).as_posix(),
                "reason": reason,
                "title": item.get("title"),
            })
    return issues


def state_issues() -> list[dict]:
    state = read_json(TASKS_PATH, {"tasks": []})
    issues: list[dict] = []
    for task in state.get("tasks", []):
        if not isinstance(task, dict) or task.get("status") not in ACTIVE_STATUSES:
            continue
        aligned, reason = task_pivot_alignment_reason(task)
        if aligned:
            continue
        issues.append({
            "task_id": task.get("id"),
            "status": task.get("status"),
            "reason": reason,
            "title": task.get("title"),
        })
    return issues


def print_issues(title: str, issues: list[dict]) -> None:
    print(f"{title}: {len(issues)}")
    for issue in issues:
        print(
            "- {task_id}: {reason} [{context}] {title}".format(
                task_id=issue.get("task_id"),
                reason=issue.get("reason"),
                context=issue.get("queue_file") or issue.get("status"),
                title=issue.get("title") or "",
            )
        )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Audit research queue/state tasks against the Descender pivot guard.",
    )
    parser.add_argument(
        "--strict-queue",
        action="store_true",
        help="exit nonzero if enabled queue tasks fail the pivot guard",
    )
    parser.add_argument(
        "--include-state",
        action="store_true",
        help="also report active tasks already present in research/state/tasks.json",
    )
    parser.add_argument(
        "--strict-state",
        action="store_true",
        help="exit nonzero if active state tasks fail the pivot guard",
    )
    args = parser.parse_args()

    queue = queue_issues()
    print_issues("Non-pivot enabled queue tasks", queue)

    state: list[dict] = []
    if args.include_state or args.strict_state:
        state = state_issues()
        print_issues("Non-pivot active state tasks", state)

    if args.strict_queue and queue:
        return 1
    if args.strict_state and state:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
