#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
from pathlib import Path

from common import (
    ROOT,
    append_jsonl,
    artifact_dir,
    emit,
    event_payload,
    git_status_short,
    load_task_or_die,
    set_status,
    update_task,
    utc_now,
    write_text,
    RUNS_PATH,
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


def append_agent_logs(paseo_bin: str, agent_id: str, output_path: Path) -> None:
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


def build_prompt(task: dict) -> str:
    acceptance = "\n".join(f"- {item}" for item in task.get("acceptance", []))
    return f"""You are an implementation/research agent working on Descender.

Task id: {task["id"]}
Title: {task.get("title", task["id"])}
Track: {task.get("track", "unknown")}
Kind: {task.get("kind", "unknown")}

Summary:
{task.get("summary", "")}

Instructions:
{task.get("prompt", "")}

Acceptance criteria:
{acceptance or "- Produce a concrete, reviewable result."}

Repository notes:
- Current project authority starts at spec/README.md, decisions/, and TRACKER.md.
- Use research/ARCHIVE_INDEX.md before treating older research notes as current.
- Use scripts/research/README.md before treating older artifact generators as
  current automation.
- Pre-pivot KEM/formalization seed plans have been retired from the worktree.
  Use current specs, ADRs, tracker rows, and classified research notes instead;
  do not let old lineage override the chain-native PQ authorization objective.
- Old 308-byte/Fq12 projection-carrier scripts and notes are archive,
  negative-control, fail-closed regression, or migration inputs unless the task
  explicitly justifies another use under current authority.
- Keep edits tightly scoped to this task.
- Work only in the checkout Paseo starts you in. Do not manually edit another checkout or worktree.
- Do not mark the task complete in project state.
- The separate Descender quality workflow decides whether this work is accepted.
- In your final response, list changed files and tests/checks run.
"""


def main() -> int:
    payload = event_payload()
    task_id = payload.get("task_id")
    role = payload.get("role", "implementation")
    if role != "implementation":
        print(f"launch_agent ignoring role={role}")
        return 0
    if not task_id:
        raise SystemExit("descender.agent.requested missing task_id")

    task = load_task_or_die(task_id)
    art = artifact_dir(task_id)
    prompt = build_prompt(task)
    prompt_path = art / "implementation_prompt.md"
    output_path = art / "implementation_output.txt"
    write_text(prompt_path, prompt)

    provider = os.environ.get("DESCENDER_IMPL_PROVIDER", "codex/gpt-5.5")
    thinking = os.environ.get("DESCENDER_IMPL_THINKING", "high")
    mode = os.environ.get("DESCENDER_IMPL_MODE", "full-access")
    paseo_bin = os.environ.get("PASEO_BIN", shutil.which("paseo") or "paseo")
    detached = os.environ.get("DESCENDER_IMPL_DETACH", "1") != "0"
    use_worktree = os.environ.get("DESCENDER_AGENT_WORKTREE") == "1"
    worktree_name = f"descender-{task_id}"
    agent_id = None
    status_before = git_status_short()

    if use_worktree and status_before.strip() and os.environ.get("DESCENDER_ALLOW_DIRTY_WORKTREE_BASE") != "1":
        output_path.write_text(
            "Refusing to launch in Paseo worktree mode because the main checkout "
            "has uncommitted or untracked state. Commit the baseline or unset "
            "DESCENDER_AGENT_WORKTREE.\n\n"
            f"{status_before}",
            encoding="utf-8",
        )
        update_task(task_id, lambda task: set_status(task, "implementation_failed", {
            "reason": "dirty checkout cannot be used as coherent worktree base",
            "output_path": str(output_path.relative_to(ROOT)),
        }))
        return 1

    update_task(task_id, lambda task: (
        set_status(task, "implementation_running", {
            "provider": provider,
            "thinking": thinking,
            "mode": mode,
            "worktree": worktree_name if use_worktree else None,
            "prompt_path": str(prompt_path.relative_to(ROOT)),
            "git_status_before": status_before,
        }),
        task.setdefault("implementation", {}).update({
            "provider": provider,
            "mode": mode,
            "detached": detached,
            "worktree": worktree_name if use_worktree else None,
            "prompt_path": str(prompt_path.relative_to(ROOT)),
            "started_at": utc_now(),
            "git_status_before": status_before,
        }),
    ))

    if os.environ.get("DESCENDER_AGENT_DRY_RUN") == "1":
        output_path.write_text("DESCENDER_AGENT_DRY_RUN=1; implementation agent not launched.\n", encoding="utf-8")
        exit_code = 0
    else:
        cmd = [
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
            task.get("title", task_id),
        ]
        if detached:
            cmd.append("--detach")
        cmd.extend([
            "--label",
            "role=implementation",
            "--label",
            f"task_id={task_id}",
            "--label",
            "project=descender",
        ])
        if use_worktree:
            cmd.extend([
                "--worktree",
                worktree_name,
            ])
        cmd.append(
            prompt,
        )
        with output_path.open("w", encoding="utf-8") as output:
            output.write(f"$ {' '.join(cmd[:-1])} <prompt>\n\n")
        try:
            process = subprocess.run(
                cmd,
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
        with output_path.open("a", encoding="utf-8") as output:
            output.write(run_output)
            output.write(f"\n[exit_code] {exit_code}\n")
        agent_id = parse_agent_id(run_output)
        if agent_id and not detached:
            append_agent_logs(paseo_bin, agent_id, output_path)
        else:
            with output_path.open("a", encoding="utf-8") as output:
                if agent_id:
                    output.write("\n[detached] Final logs will be collected by monitor_agent.py.\n")
                else:
                    output.write("\n[warning] Could not parse Paseo agent id; logs not fetched.\n")
    status_after = git_status_short()
    git_status_changed = status_after != status_before

    run_record = {
        "at": utc_now(),
        "task_id": task_id,
        "role": "implementation",
        "provider": provider,
        "thinking": thinking,
        "mode": mode,
        "detached": detached,
        "worktree": worktree_name if use_worktree else None,
        "exit_code": exit_code,
        "agent_id": agent_id if os.environ.get("DESCENDER_AGENT_DRY_RUN") != "1" else None,
        "git_status_changed": git_status_changed,
        "git_status_before": status_before,
        "git_status_after": status_after,
        "prompt_path": str(prompt_path.relative_to(ROOT)),
        "output_path": str(output_path.relative_to(ROOT)),
    }
    append_jsonl(RUNS_PATH, run_record)

    def finish(task: dict) -> None:
        task.setdefault("implementation", {}).update({
            "exit_code": exit_code,
            "agent_id": agent_id if os.environ.get("DESCENDER_AGENT_DRY_RUN") != "1" else None,
            "output_path": str(output_path.relative_to(ROOT)),
        })
        if exit_code == 0 and detached:
            task.setdefault("implementation", {}).update({
                "detached": True,
                "launch_finished_at": utc_now(),
            })
            set_status(task, "implementation_running", {
                "source": "detached_launch",
                "agent_id": agent_id,
                "output_path": str(output_path.relative_to(ROOT)),
            })
        elif exit_code == 0:
            task.setdefault("implementation", {}).update({
                "finished_at": utc_now(),
                "git_status_after": status_after,
                "git_status_changed": git_status_changed,
            })
            set_status(task, "implementation_completed", {
                "output_path": str(output_path.relative_to(ROOT)),
                "git_status_changed": git_status_changed,
            })
        else:
            set_status(task, "implementation_failed", {
                "exit_code": exit_code,
                "output_path": str(output_path.relative_to(ROOT)),
            })

    update_task(task_id, finish)

    if exit_code == 0 and detached:
        emit("descender.agent.started", {
            "task_id": task_id,
            "role": "implementation",
            "agent_id": agent_id,
            "output_path": str(output_path.relative_to(ROOT)),
        })
    elif exit_code == 0:
        emit("descender.agent.completed", {
            "task_id": task_id,
            "role": "implementation",
            "output_path": str(output_path.relative_to(ROOT)),
        })
    else:
        emit("descender.agent.failed", {
            "task_id": task_id,
            "role": "implementation",
            "exit_code": exit_code,
            "output_path": str(output_path.relative_to(ROOT)),
        })
    print(json.dumps(run_record, indent=2, sort_keys=True))
    return 0 if exit_code == 0 else exit_code


if __name__ == "__main__":
    raise SystemExit(main())
