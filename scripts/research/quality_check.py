#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
import shutil
import subprocess

from common import (
    ROOT,
    append_jsonl,
    artifact_dir,
    emit,
    event_payload,
    git_status_short,
    load_task_or_die,
    run_capture,
    set_status,
    update_task,
    utc_now,
    write_text,
    RUNS_PATH,
)


VERDICT_RE = re.compile(r"^DESCENDER_QUALITY_VERDICT:\s*(accept|reject|needs_human)\s*$", re.I | re.M)
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


def archive_agent(paseo_bin: str, agent_id: str | None, reason: str, output_path) -> dict | None:
    if not agent_id or os.environ.get("DESCENDER_AGENT_DRY_RUN") == "1":
        return None
    cmd = [paseo_bin, "archive", "--json", agent_id]
    try:
        process = subprocess.run(
            cmd,
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )
        result = {
            "agent_id": agent_id,
            "reason": reason,
            "exit_code": 0 if process.returncode == 0 or "AGENT_ALREADY_ARCHIVED" in process.stdout else process.returncode,
            "raw_exit_code": process.returncode,
            "output": process.stdout[-4000:],
        }
    except FileNotFoundError:
        result = {
            "agent_id": agent_id,
            "reason": reason,
            "exit_code": 127,
            "output": f"paseo binary not found: {paseo_bin}\n",
        }
    with output_path.open("a", encoding="utf-8") as output:
        output.write(f"\n\n$ {' '.join(cmd)}\n\n")
        output.write(result["output"])
        output.write(f"\n[exit_code] {result['exit_code']}\n")
    return result


def build_review_prompt(task: dict, implementation_output: str, local_checks: list[dict]) -> str:
    checks = json.dumps(local_checks, indent=2, sort_keys=True)
    return f"""You are the independent Descender quality reviewer.

You are not the implementation agent. Do not edit files. Your job is to decide
whether the implementation output is good enough to update project state.
Work only in the checkout Paseo starts you in. Do not manually edit another checkout or worktree.

Task id: {task["id"]}
Title: {task.get("title", task["id"])}
Track: {task.get("track", "unknown")}
Kind: {task.get("kind", "unknown")}

Acceptance criteria:
{json.dumps(task.get("acceptance", []), indent=2)}

Local checks recorded by the workflow:
{checks}

Implementation output:
```text
{implementation_output[-20000:]}
```

Review requirements:
- Verify the result against the task, not just whether the agent exited cleanly.
- Check whether the result respects the current project authority chain:
  `spec/README.md`, `decisions/`, `TRACKER.md`, `research/ARCHIVE_INDEX.md`,
  and `scripts/research/README.md`.
- Treat work that advances legacy KEM/formalization goals or old
  308-byte/Fq12 projection-carrier deployment goals while ignoring the
  chain-native PQ authorization pivot as not accepted unless the task explicitly
  justified that legacy support path as archive, negative-control, fail-closed
  regression, or migration work.
- Treat missing tests, vague claims, or unreviewable output as not accepted.
- Failed required checks block acceptance. Failed non-required checks are context;
  mention them when relevant, but do not reject solely because they failed.
- If source files were edited, identify whether the edits are scoped and whether
  the relevant checks were run.
- Return exactly one machine-readable verdict line:
  DESCENDER_QUALITY_VERDICT: accept
  DESCENDER_QUALITY_VERDICT: reject
  DESCENDER_QUALITY_VERDICT: needs_human
- Then provide a short rationale.
"""


def build_revision_prompt(task: dict, review_output: str, local_checks: list[dict], revision_attempt: int) -> str:
    checks = json.dumps(local_checks, indent=2, sort_keys=True)
    acceptance = "\n".join(f"- {item}" for item in task.get("acceptance", []))
    return f"""The independent Descender quality reviewer did not accept your work.

You are the original implementation/research agent. Continue the same task in
the same checkout. Address the review directly, keep changes scoped, and do not
mark project state yourself; the separate quality workflow will review again.

Task id: {task["id"]}
Title: {task.get("title", task["id"])}
Revision attempt: {revision_attempt}

Original instructions:
{task.get("prompt", "")}

Acceptance criteria:
{acceptance or "- Produce a concrete, reviewable result."}

Local checks from the failed review:
{checks}

Reviewer output:
```text
{review_output[-20000:]}
```

In your final response, list changed files and tests/checks run.
"""


def parse_verdict(text: str) -> str:
    match = VERDICT_RE.search(text)
    if not match:
        return "needs_human"
    return match.group(1).lower()


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


def requires_no_source_edits(task: dict) -> bool:
    text = text_fields(task)
    return any(
        phrase in text
        for phrase in (
            "no source edits",
            "no source files are edited",
            "makes no source edits",
            "make no source edits",
            "do not edit files",
            "don't edit files",
            "without editing files",
            "no source files",
        )
    )


def requires_clean_lake_build(task: dict, implementation_git_changed: bool) -> bool:
    if implementation_git_changed:
        return True
    text = text_fields(task)
    return any(
        phrase in text
        for phrase in (
            "lean",
            "formalize",
            "prove",
            "proof",
            "theorem",
            "refactor",
            "implementation",
            "source edit",
        )
    ) and not requires_no_source_edits(task)


def send_revision_to_implementer(
    task_id: str,
    task: dict,
    review_output: str,
    local_checks: list[dict],
    paseo_bin: str,
    art,
) -> dict | None:
    max_revisions = int(os.environ.get("DESCENDER_MAX_REVISIONS", "2"))
    revision_attempt = int(task.get("revision_attempts", 0)) + 1
    implementer_agent_id = task.get("implementation", {}).get("agent_id")
    if not implementer_agent_id or revision_attempt > max_revisions:
        return None

    prompt = build_revision_prompt(task, review_output, local_checks, revision_attempt)
    prompt_path = art / f"revision_{revision_attempt}_prompt.md"
    output_path = art / f"revision_{revision_attempt}_output.txt"
    write_text(prompt_path, prompt)

    status_before = git_status_short()
    update_task(task_id, lambda task: (
        set_status(task, "revision_running", {
            "attempt": revision_attempt,
            "agent_id": implementer_agent_id,
            "prompt_path": str(prompt_path.relative_to(ROOT)),
        }),
        task.__setitem__("revision_attempts", revision_attempt),
    ))

    if os.environ.get("DESCENDER_AGENT_DRY_RUN") == "1":
        output_path.write_text(
            "DESCENDER_AGENT_DRY_RUN=1; revision not sent.\n",
            encoding="utf-8",
        )
        exit_code = 0
    else:
        cmd = [
            paseo_bin,
            "send",
            "--json",
            "--prompt-file",
            str(prompt_path),
            implementer_agent_id,
        ]
        with output_path.open("w", encoding="utf-8") as output:
            output.write(f"$ {' '.join(cmd)}\n\n")
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
            send_output = process.stdout
        except FileNotFoundError:
            send_output = f"paseo binary not found: {paseo_bin}\n"
            exit_code = 127
        with output_path.open("a", encoding="utf-8") as output:
            output.write(send_output)
            output.write(f"\n[exit_code] {exit_code}\n")
        append_agent_logs(paseo_bin, implementer_agent_id, output_path)

    status_after = git_status_short()
    git_status_changed = status_after != status_before
    run_record = {
        "at": utc_now(),
        "task_id": task_id,
        "role": "revision",
        "attempt": revision_attempt,
        "agent_id": implementer_agent_id,
        "exit_code": exit_code,
        "git_status_changed": git_status_changed,
        "git_status_before": status_before,
        "git_status_after": status_after,
        "prompt_path": str(prompt_path.relative_to(ROOT)),
        "output_path": str(output_path.relative_to(ROOT)),
    }
    append_jsonl(RUNS_PATH, run_record)

    def finish_revision(task: dict) -> None:
        task.setdefault("revisions", []).append(run_record)
        task.setdefault("implementation", {}).update({
            "finished_at": utc_now(),
            "exit_code": exit_code,
            "agent_id": implementer_agent_id,
            "output_path": str(output_path.relative_to(ROOT)),
            "git_status_after": status_after,
            "git_status_changed": git_status_changed,
            "revision_attempt": revision_attempt,
        })
        if exit_code == 0:
            set_status(task, "implementation_completed", {
                "source": "revision",
                "attempt": revision_attempt,
                "output_path": str(output_path.relative_to(ROOT)),
                "git_status_changed": git_status_changed,
            })
        else:
            set_status(task, "revision_failed", {
                "attempt": revision_attempt,
                "exit_code": exit_code,
                "output_path": str(output_path.relative_to(ROOT)),
            })

    update_task(task_id, finish_revision)
    if exit_code == 0:
        emit("descender.agent.completed", {
            "task_id": task_id,
            "role": "implementation",
            "output_path": str(output_path.relative_to(ROOT)),
            "revision_attempt": revision_attempt,
        })
    else:
        emit("descender.agent.failed", {
            "task_id": task_id,
            "role": "implementation",
            "exit_code": exit_code,
            "output_path": str(output_path.relative_to(ROOT)),
            "revision_attempt": revision_attempt,
        })
    return run_record


def main() -> int:
    payload = event_payload()
    if payload.get("role") != "implementation":
        print("quality_check only handles implementation completions")
        return 0
    task_id = payload.get("task_id")
    if not task_id:
        raise SystemExit("descender.agent.completed missing task_id")

    task = load_task_or_die(task_id)
    art = artifact_dir(task_id)
    update_task(task_id, lambda task: set_status(task, "quality_running", {"source": "quality_check"}))
    status_before = git_status_short()

    local_checks: list[dict] = []
    implementation_git_changed = bool(task.get("implementation", {}).get("git_status_changed"))
    no_source_edits_required = requires_no_source_edits(task)
    local_checks.append({
        "name": "implementation did not change git status",
        "exit_code": 1 if implementation_git_changed else 0,
        "required": no_source_edits_required,
    })

    lake_log = art / "quality_lake_build.log"
    lake_exit = run_capture(["lake", "build"], ROOT, lake_log)
    local_checks.append({
        "name": "lake build",
        "exit_code": lake_exit,
        "log_path": str(lake_log.relative_to(ROOT)),
        "required": requires_clean_lake_build(task, implementation_git_changed),
    })

    git_log = art / "quality_git_status.txt"
    git_exit = run_capture(["git", "status", "--short"], ROOT, git_log)
    local_checks.append({
        "name": "git status --short",
        "exit_code": git_exit,
        "log_path": str(git_log.relative_to(ROOT)),
        "required": False,
    })

    impl_output_path = ROOT / task.get("implementation", {}).get("output_path", payload.get("output_path", ""))
    implementation_output = ""
    if impl_output_path.exists():
        implementation_output = impl_output_path.read_text(encoding="utf-8", errors="replace")

    prompt = build_review_prompt(task, implementation_output, local_checks)
    prompt_path = art / "quality_prompt.md"
    output_path = art / "quality_output.txt"
    write_text(prompt_path, prompt)

    provider = os.environ.get("DESCENDER_QUALITY_PROVIDER", "codex/gpt-5.5")
    thinking = os.environ.get("DESCENDER_QUALITY_THINKING", "high")
    mode = os.environ.get("DESCENDER_QUALITY_MODE", "full-access")
    use_worktree = os.environ.get("DESCENDER_AGENT_WORKTREE") == "1"
    paseo_bin = os.environ.get("PASEO_BIN", shutil.which("paseo") or "paseo")
    exit_code = 0
    agent_id = None

    if os.environ.get("DESCENDER_AGENT_DRY_RUN") == "1":
        output_path.write_text(
            "DESCENDER_AGENT_DRY_RUN=1; quality agent not launched.\n"
            "DESCENDER_QUALITY_VERDICT: needs_human\n",
            encoding="utf-8",
        )
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
            f"Quality review: {task.get('title', task_id)}",
        ]
        if use_worktree:
            cmd.extend([
                "--worktree",
                f"descender-quality-{task_id}",
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
        if agent_id:
            append_agent_logs(paseo_bin, agent_id, output_path)
        else:
            with output_path.open("a", encoding="utf-8") as output:
                output.write("\n[warning] Could not parse Paseo agent id; logs not fetched.\n")

    review_output = output_path.read_text(encoding="utf-8", errors="replace")
    status_after = git_status_short()
    git_status_changed = status_after != status_before
    verdict = parse_verdict(review_output)
    required_checks_passed = all(
        (not check.get("required")) or check.get("exit_code") == 0
        for check in local_checks
    )
    revision_record = None
    should_request_revision = (
        exit_code == 0
        and not git_status_changed
        and verdict == "reject"
    )
    if should_request_revision:
        revision_record = send_revision_to_implementer(
            task_id,
            task,
            review_output,
            local_checks,
            paseo_bin,
            art,
        )

    if revision_record:
        final_status = "implementation_completed" if revision_record["exit_code"] == 0 else "revision_failed"
    elif exit_code != 0:
        final_status = "quality_blocked"
    elif git_status_changed:
        final_status = "quality_blocked"
    elif verdict == "accept" and required_checks_passed:
        final_status = "accepted"
    elif verdict == "reject":
        final_status = "quality_rejected"
    else:
        final_status = "quality_needs_human"

    cleanup: list[dict] = []
    quality_cleanup = archive_agent(paseo_bin, agent_id, "quality review completed", output_path)
    if quality_cleanup:
        cleanup.append(quality_cleanup)
    if not revision_record and final_status in {
        "accepted",
        "quality_rejected",
        "quality_needs_human",
        "quality_blocked",
        "revision_failed",
    }:
        impl_cleanup = archive_agent(
            paseo_bin,
            task.get("implementation", {}).get("agent_id"),
            f"task terminal status: {final_status}",
            output_path,
        )
        if impl_cleanup:
            cleanup.append(impl_cleanup)

    run_record = {
        "at": utc_now(),
        "task_id": task_id,
        "role": "quality",
        "provider": provider,
        "thinking": thinking,
        "mode": mode,
        "exit_code": exit_code,
        "agent_id": agent_id if os.environ.get("DESCENDER_AGENT_DRY_RUN") != "1" else None,
        "verdict": verdict,
        "required_checks_passed": required_checks_passed,
        "git_status_changed": git_status_changed,
        "git_status_before": status_before,
        "git_status_after": status_after,
        "prompt_path": str(prompt_path.relative_to(ROOT)),
        "output_path": str(output_path.relative_to(ROOT)),
        "local_checks": local_checks,
        "final_status": final_status,
        "revision_record": revision_record,
        "cleanup": cleanup,
    }
    append_jsonl(RUNS_PATH, run_record)

    def finish(task: dict) -> None:
        task.setdefault("quality", {}).update(run_record)
        if not revision_record:
            set_status(task, final_status, {
                "verdict": verdict,
                "required_checks_passed": required_checks_passed,
                "git_status_changed": git_status_changed,
                "output_path": str(output_path.relative_to(ROOT)),
            })

    update_task(task_id, finish)
    emit("descender.quality.completed", {
        "task_id": task_id,
        "status": final_status,
        "verdict": verdict,
        "required_checks_passed": required_checks_passed,
    })
    print(json.dumps(run_record, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
