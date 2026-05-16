#!/usr/bin/env python3
from __future__ import annotations

from collections import Counter

from common import STATE_DIR, TASKS_PATH, read_json


def main() -> int:
    state = read_json(TASKS_PATH, {"tasks": []})
    tasks = state.get("tasks", [])
    counts = Counter(task.get("status", "unknown") for task in tasks)
    print("Descender research status")
    print("=========================")
    if not tasks:
        print("No imported tasks. Enable tasks in research/queue/*.json and run research-director.")
    else:
        for status, count in sorted(counts.items()):
            print(f"{status}: {count}")
        print()
        for task in tasks:
            print(f"{task.get('id')}: {task.get('status')} - {task.get('title', '')}")

    lean_status = read_json(STATE_DIR / "lean_status.json", None)
    if lean_status:
        print()
        print("Lean")
        print("----")
        print(f"lake build exit: {lean_status.get('lake_build_exit_code')}")
        print(f"sorry/admit count: {lean_status.get('sorry_count')}")
        print(f"log: {lean_status.get('lake_build_log')}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
