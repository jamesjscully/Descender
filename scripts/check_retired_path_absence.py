#!/usr/bin/env python3
"""Check that deliberately retired stale paths have not reappeared.

Each retired-path registry entry must be owned by a live consolidated-tracker
row so deleted stale paths cannot become a detached side list.
"""

from __future__ import annotations

from dataclasses import dataclass
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")

ACTIVE_TRACKER_STATUSES = {
    "proposed",
    "selected",
    "in_progress",
    "implemented",
    "verified",
    "blocked",
}


@dataclass(frozen=True)
class RetiredPath:
    path: str
    category: str
    owner: str


RETIREMENT_CATEGORIES = {
    "archived_deployable_target",
    "imported_background_conversation",
    "pre_adr_decision_note",
    "pre_pivot_root_plan",
    "retired_q12289_review_package",
    "retired_qrom_widening_probe",
    "superseded_tracker",
}

RETIRED_PATHS = (
    RetiredPath("audit/active_path_signature_tracker.md", "superseded_tracker", "PROC-ARCHIVE-008"),
    RetiredPath("chat.md", "imported_background_conversation", "PROC-ARCHIVE-008"),
    RetiredPath("formal/VERIFICATION_PLAN.md", "pre_pivot_root_plan", "PROC-ARCHIVE-008"),
    RetiredPath("lean_proof_plan.md", "pre_pivot_root_plan", "PROC-ARCHIVE-008"),
    RetiredPath(
        "research/cryptanalysis/cryptanalysis_first_master_tracker.md",
        "superseded_tracker",
        "PROC-ARCHIVE-008",
    ),
    RetiredPath(
        "research/cryptanalysis/review_readiness_20260519",
        "retired_q12289_review_package",
        "PROC-ARCHIVE-008",
    ),
    RetiredPath(
        "research/cryptanalysis/signature_readiness_20260516/qrom_wide_challenge_action_probe_n32_q12289.json",
        "retired_qrom_widening_probe",
        "PROC-ARCHIVE-008",
    ),
    RetiredPath(
        "research/cryptanalysis/signature_readiness_20260516/qrom_wide_challenge_action_probe_n32_q12289.md",
        "retired_qrom_widening_probe",
        "PROC-ARCHIVE-008",
    ),
    RetiredPath("research/deployable_signature_security_tracker.md", "superseded_tracker", "PROC-ARCHIVE-008"),
    RetiredPath("research/deployable_signature_target.md", "archived_deployable_target", "PROC-ARCHIVE-008"),
    RetiredPath("research/selected_hidden_source_path_decision.md", "pre_adr_decision_note", "PROC-ARCHIVE-008"),
    RetiredPath("research_program.md", "pre_pivot_root_plan", "PROC-ARCHIVE-008"),
    RetiredPath(
        "scripts/research/qrom_wide_challenge_action_probe.py",
        "retired_qrom_widening_probe",
        "PROC-ARCHIVE-008",
    ),
)


def tracker_statuses() -> dict[str, str]:
    statuses: dict[str, str] = {}
    for line in (ROOT / "TRACKER.md").read_text(encoding="utf-8").splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if len(cells) >= 2 and TRACKER_ITEM_ID.match(cells[0]):
            statuses[cells[0]] = cells[1]
    return statuses


def main() -> int:
    errors: list[str] = []
    paths = [entry.path for entry in RETIRED_PATHS]
    if len(paths) != len(set(paths)):
        errors.append("retired path registry has duplicate paths")
    if paths != sorted(paths):
        errors.append("retired path registry must be sorted by path")

    tracker_rows = tracker_statuses()
    for entry in RETIRED_PATHS:
        if entry.category not in RETIREMENT_CATEGORIES:
            errors.append(f"{entry.path}: unknown retirement category {entry.category!r}")
        owner_status = tracker_rows.get(entry.owner)
        if owner_status is None:
            errors.append(f"{entry.path}: owner tracker row does not exist: {entry.owner}")
        elif owner_status not in ACTIVE_TRACKER_STATUSES:
            errors.append(
                f"{entry.path}: owner tracker row is not active: "
                f"{entry.owner} has status {owner_status!r}"
            )
        if (ROOT / entry.path).exists():
            errors.append(f"retired path is present: {entry.path}")

    if errors:
        print("retired path absence check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print(
        "retired path absence check passed "
        f"({len(RETIRED_PATHS)} paths absent, "
        f"{len(RETIREMENT_CATEGORIES)} categories)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
