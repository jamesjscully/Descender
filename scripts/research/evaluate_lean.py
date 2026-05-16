#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess

from common import ROOT, STATE_DIR, artifact_dir, emit, run_capture, utc_now, write_json


def main() -> int:
    art = artifact_dir("lean-check")
    build_log = art / "lake_build.log"
    exit_code = run_capture(["lake", "build"], ROOT, build_log)

    sorry_result = subprocess.run(
        ["rg", "-n", r"\bsorry\b|\badmit\b", "Descender.lean", "Descender"],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        check=False,
    )
    sorry_log = art / "sorry_inventory.txt"
    sorry_log.write_text(sorry_result.stdout, encoding="utf-8")
    sorry_count = 0 if sorry_result.returncode == 1 else len([line for line in sorry_result.stdout.splitlines() if line.strip()])

    status = {
        "checked_at": utc_now(),
        "lake_build_exit_code": exit_code,
        "lake_build_log": str(build_log.relative_to(ROOT)),
        "sorry_inventory": str(sorry_log.relative_to(ROOT)),
        "sorry_count": sorry_count,
    }
    write_json(STATE_DIR / "lean_status.json", status)
    emit("descender.lean.checked", status)
    print(json.dumps(status, indent=2, sort_keys=True))
    return 0 if exit_code == 0 else exit_code


if __name__ == "__main__":
    raise SystemExit(main())
