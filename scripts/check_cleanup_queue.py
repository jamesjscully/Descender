#!/usr/bin/env python3
"""Check that every non-active archive row has an explicit cleanup queue entry."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ARCHIVE = ROOT / "research" / "ARCHIVE_INDEX.md"
QUEUE = ROOT / "research" / "CLEANUP_QUEUE.md"
TRACKER = ROOT / "TRACKER.md"
DECISIONS = ROOT / "decisions"

NON_ACTIVE_LABELS = {
    "ARCHIVED_TARGET",
    "LEGACY_FORMAL",
    "MIXED_STATUS",
    "NEGATIVE_CONTROL",
    "SUPERSEDED_DECISION",
    "SUPERSEDED_TRACKER",
}

ALLOWED_DISPOSITIONS = {
    "remove_after_import",
    "relocate_after_import",
    "retain_history",
    "retain_legacy_formal_spine",
    "retain_legacy_conformance",
    "retain_negative_control",
}

LABEL_ALLOWED_DISPOSITIONS = {
    "ARCHIVED_TARGET": {
        "remove_after_import",
        "relocate_after_import",
        "retain_history",
    },
    "LEGACY_FORMAL": {
        "remove_after_import",
        "relocate_after_import",
        "retain_legacy_formal_spine",
        "retain_legacy_conformance",
    },
    "MIXED_STATUS": {
        "remove_after_import",
        "relocate_after_import",
    },
    "NEGATIVE_CONTROL": {
        "remove_after_import",
        "relocate_after_import",
        "retain_negative_control",
    },
    "SUPERSEDED_DECISION": {
        "remove_after_import",
        "relocate_after_import",
        "retain_history",
    },
    "SUPERSEDED_TRACKER": {
        "remove_after_import",
        "relocate_after_import",
    },
}

RETAIN_DISPOSITIONS = {
    "retain_history",
    "retain_legacy_formal_spine",
    "retain_legacy_conformance",
    "retain_negative_control",
}

REMOVE_OR_RELOCATE_DISPOSITIONS = {
    "remove_after_import",
    "relocate_after_import",
}

ACTIVE_TRACKER_STATUSES = {
    "proposed",
    "selected",
    "in_progress",
    "implemented",
    "verified",
    "blocked",
}

OWNER_RULE_PHRASES = (
    "`Owner` must include at least one active `TRACKER.md` row.",
    "ADR references may supplement owner context, but they do not replace live tracker ownership.",
)

ARCHIVE_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|")
DISPOSITION_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+[^|]+\|\s*$")
COMPATIBILITY_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+(.+?)\s+\|\s*$")
QUEUE_ROW = re.compile(
    r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|\s+(.+?)\s+\|\s+(.+?)\s+\|"
)
TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")
ADR_REF = re.compile(r"^ADR-(\d{4})$")
CODE_SPAN = re.compile(r"`([^`]+)`")


def has_word(text: str, word: str) -> bool:
    return re.search(rf"\b{re.escape(word)}\b", text, flags=re.IGNORECASE) is not None


def section_lines(text: str, heading: str) -> list[str]:
    lines = text.splitlines()
    result: list[str] = []
    in_section = False
    for line in lines:
        if line == heading:
            in_section = True
            continue
        if in_section and line.startswith("## "):
            break
        if in_section:
            result.append(line)
    return result


def archive_non_active_rows() -> dict[str, str]:
    rows: dict[str, str] = {}
    for line in ARCHIVE.read_text(encoding="utf-8").splitlines():
        match = ARCHIVE_ROW.match(line)
        if not match:
            continue
        target, label = match.groups()
        if label in NON_ACTIVE_LABELS:
            rows[target] = label
    return rows


def tracker_statuses() -> dict[str, str]:
    statuses: dict[str, str] = {}
    for line in TRACKER.read_text(encoding="utf-8").splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if len(cells) >= 2 and TRACKER_ITEM_ID.match(cells[0]):
            statuses[cells[0]] = cells[1]
    return statuses


def adr_exists(ref: str) -> bool:
    match = ADR_REF.match(ref)
    return bool(match and any(DECISIONS.glob(f"ADR-{int(match.group(1)):04d}-*.md")))


def queue_rows(errors: list[str]) -> dict[str, tuple[str, str, str, list[str]]]:
    if not QUEUE.exists():
        errors.append("research/CLEANUP_QUEUE.md: missing cleanup queue")
        return {}

    text = QUEUE.read_text(encoding="utf-8")
    if "Status: ACTIVE_SUPPORT" not in "\n".join(text.splitlines()[:8]):
        errors.append("research/CLEANUP_QUEUE.md: missing active support status marker")
    if "## Disposition Vocabulary" not in text:
        errors.append("research/CLEANUP_QUEUE.md: missing ## Disposition Vocabulary section")
    if "## Cleanup Queue" not in text:
        errors.append("research/CLEANUP_QUEUE.md: missing ## Cleanup Queue section")

    disposition_section = section_lines(text, "## Disposition Vocabulary")
    declared_dispositions = {
        match.group(1)
        for line in disposition_section
        if (match := DISPOSITION_ROW.match(line))
    }
    missing_dispositions = sorted(ALLOWED_DISPOSITIONS - declared_dispositions)
    extra_dispositions = sorted(declared_dispositions - ALLOWED_DISPOSITIONS)
    if missing_dispositions:
        errors.append(
            "research/CLEANUP_QUEUE.md: disposition vocabulary missing: "
            + ", ".join(missing_dispositions)
        )
    if extra_dispositions:
        errors.append(
            "research/CLEANUP_QUEUE.md: unknown disposition vocabulary rows: "
            + ", ".join(extra_dispositions)
        )

    if "## Label Compatibility" not in text:
        errors.append("research/CLEANUP_QUEUE.md: missing ## Label Compatibility section")
    if "## Exit Condition Rule" not in text:
        errors.append("research/CLEANUP_QUEUE.md: missing ## Exit Condition Rule section")
    if "## Owner Rule" not in text:
        errors.append("research/CLEANUP_QUEUE.md: missing ## Owner Rule section")
    normalized = re.sub(r"\s+", " ", text)
    for phrase in OWNER_RULE_PHRASES:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"research/CLEANUP_QUEUE.md: missing owner rule phrase {phrase!r}")
    compatibility_section = section_lines(text, "## Label Compatibility")
    declared_compatibility: dict[str, set[str]] = {}
    for line_no, line in enumerate(compatibility_section, start=1):
        match = COMPATIBILITY_ROW.match(line)
        if not match:
            continue
        label, dispositions_cell = match.groups()
        dispositions = set(CODE_SPAN.findall(dispositions_cell))
        if not dispositions:
            errors.append(
                f"research/CLEANUP_QUEUE.md: label compatibility row has no dispositions: {label}"
            )
        declared_compatibility[label] = dispositions

    expected_labels = set(LABEL_ALLOWED_DISPOSITIONS)
    declared_labels = set(declared_compatibility)
    missing_labels = sorted(expected_labels - declared_labels)
    extra_labels = sorted(declared_labels - expected_labels)
    if missing_labels:
        errors.append(
            "research/CLEANUP_QUEUE.md: label compatibility missing labels: "
            + ", ".join(missing_labels)
        )
    if extra_labels:
        errors.append(
            "research/CLEANUP_QUEUE.md: unknown label compatibility rows: "
            + ", ".join(extra_labels)
        )
    for label, expected in sorted(LABEL_ALLOWED_DISPOSITIONS.items()):
        declared = declared_compatibility.get(label)
        if declared is not None and declared != expected:
            errors.append(
                "research/CLEANUP_QUEUE.md: label compatibility mismatch for "
                f"{label}: declared {sorted(declared)} != expected {sorted(expected)}"
            )

    rows: dict[str, tuple[str, str, str, list[str]]] = {}
    for line_no, line in enumerate(text.splitlines(), start=1):
        match = QUEUE_ROW.match(line)
        if not match:
            continue
        target, label, disposition, evidence, owner = match.groups()
        if target in {"Target", "---"}:
            continue
        if target in rows:
            errors.append(f"research/CLEANUP_QUEUE.md:{line_no}: duplicate cleanup target {target}")
        refs = CODE_SPAN.findall(owner)
        rows[target] = (label, disposition, evidence.strip(), refs)
    return rows


def main() -> int:
    errors: list[str] = []
    archive_rows = archive_non_active_rows()
    cleanup_rows = queue_rows(errors)
    known_tracker_statuses = tracker_statuses()

    missing = sorted(set(archive_rows) - set(cleanup_rows))
    extra = sorted(set(cleanup_rows) - set(archive_rows))
    if missing:
        errors.append("research/CLEANUP_QUEUE.md: missing targets: " + ", ".join(missing))
    if extra:
        errors.append("research/CLEANUP_QUEUE.md: unexpected targets: " + ", ".join(extra))

    for target, (label, disposition, evidence, refs) in cleanup_rows.items():
        expected_label = archive_rows.get(target)
        if expected_label is not None and label != expected_label:
            errors.append(
                f"research/CLEANUP_QUEUE.md: label mismatch for {target}: "
                f"{label} != {expected_label}"
            )
        if disposition not in ALLOWED_DISPOSITIONS:
            errors.append(
                f"research/CLEANUP_QUEUE.md: invalid disposition for {target}: {disposition}"
            )
        allowed_for_label = LABEL_ALLOWED_DISPOSITIONS.get(label, set())
        if disposition in ALLOWED_DISPOSITIONS and disposition not in allowed_for_label:
            errors.append(
                f"research/CLEANUP_QUEUE.md: disposition {disposition} is not valid "
                f"for {label} target {target}"
            )
        if not evidence:
            errors.append(f"research/CLEANUP_QUEUE.md: empty evidence/blocker for {target}")
        else:
            if evidence[-1] not in {".", "!", "?"}:
                errors.append(
                    f"research/CLEANUP_QUEUE.md: evidence/blocker must be a complete sentence "
                    f"for {target}"
                )
            if "tbd" in evidence.lower() or "todo" in evidence.lower():
                errors.append(
                    f"research/CLEANUP_QUEUE.md: evidence/blocker is not an exit condition "
                    f"for {target}"
                )
            if disposition in RETAIN_DISPOSITIONS and not (
                has_word(evidence, "until") or has_word(evidence, "while")
            ):
                errors.append(
                    f"research/CLEANUP_QUEUE.md: retained target must state an until/while "
                    f"exit condition for {target}"
                )
            if disposition in REMOVE_OR_RELOCATE_DISPOSITIONS and not (
                has_word(evidence, "after") or has_word(evidence, "once")
            ):
                errors.append(
                    f"research/CLEANUP_QUEUE.md: remove/relocate target must state an "
                    f"after/once exit condition for {target}"
                )
        if not refs:
            errors.append(f"research/CLEANUP_QUEUE.md: missing owner refs for {target}")
        has_active_tracker_owner = False
        for ref in refs:
            if TRACKER_ITEM_ID.match(ref):
                status = known_tracker_statuses.get(ref)
                if status is None:
                    errors.append(
                        f"research/CLEANUP_QUEUE.md: owner tracker row does not exist for {target}: {ref}"
                    )
                elif status not in ACTIVE_TRACKER_STATUSES:
                    errors.append(
                        f"research/CLEANUP_QUEUE.md: owner tracker row is not active "
                        f"for {target}: {ref} has status {status!r}"
                    )
                else:
                    has_active_tracker_owner = True
            elif ADR_REF.match(ref):
                if not adr_exists(ref):
                    errors.append(
                        f"research/CLEANUP_QUEUE.md: owner ADR does not exist for {target}: {ref}"
                    )
            else:
                errors.append(
                    f"research/CLEANUP_QUEUE.md: owner ref is not a tracker ID or ADR for {target}: {ref}"
                )
        if refs and not has_active_tracker_owner:
            errors.append(
                f"research/CLEANUP_QUEUE.md: target needs at least one active tracker owner row: {target}"
            )

    if errors:
        print("cleanup queue check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print(f"cleanup queue check passed ({len(cleanup_rows)} non-active archive targets)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
