#!/usr/bin/env python3
"""List high-risk Markdown docs that are not yet classified.

In normal mode this is a cleanup helper. In `--strict` mode it is a gate. It
finds Markdown files whose names or contents look like trackers, todos,
decisions, specs, status reports, plans, readiness packages, active status
notes, or legacy deployment claims, then removes files already covered by the
active authority spine or `research/ARCHIVE_INDEX.md`.

Use `--include-status-lines` as an advisory migration view for files with any
explicit `Status:` line. Use `--include-path2-research` to require
classification for every `research/path2*.md` note, including notes without an
explicit status marker. Use `--include-cryptanalysis-root` to require
classification for every root-level `research/cryptanalysis/*.md` security note.
Use `--include-all-markdown` to require classification for every Markdown file
outside the active authority set and ignored generated/state directories.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

ARCHIVE_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|")
HIGH_RISK_NAME = re.compile(
    r"(tracker|todo|status|decision|target|readiness|spec|plan|roadmap|checklist|verdict)",
    re.I,
)
HIGH_RISK_CONTENT = {
    "active_status": re.compile(r"^Status:\s+active\b", re.I | re.M),
    "current_status": re.compile(r"^Status:\s+current\b", re.I | re.M),
    "selected_308": re.compile(r"selected\s+308[- ]byte|selected\s+308\s*B", re.I),
    "deployable_target": re.compile(
        r"active deployable|current deployable|deployable-signature target|deployable signature target",
        re.I,
    ),
    "authority_language": re.compile(
        r"single instrument for measuring|canonical specification",
        re.I,
    ),
}

EXCLUDED_PARTS = {
    ".git",
    ".lake",
    "target",
    "node_modules",
}

EXCLUDED_PREFIXES = (
    "research/state/",
    "research/tasks/",
    "research/agents/",
    "research/artifacts/",
)

CURRENT_AUTHORITY_BASE = {
    "README.md",
    "TRACKER.md",
    "decisions/README.md",
    "research/README.md",
    "research/ARCHIVE_INDEX.md",
    "scripts/research/README.md",
}


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def archive_index_classifications() -> tuple[set[str], list[str]]:
    index = ROOT / "research/ARCHIVE_INDEX.md"
    if not index.exists():
        return set(), []
    paths: set[str] = set()
    prefixes: list[str] = []
    for line in index.read_text(encoding="utf-8").splitlines():
        match = ARCHIVE_ROW.match(line)
        if not match or match.group(1) in {"File", "Package"}:
            continue
        path = match.group(1)
        if path.endswith("/**"):
            prefixes.append(path[:-2])
        else:
            paths.add(path)
    return paths, prefixes


def current_authority_paths() -> set[str]:
    paths = set(CURRENT_AUTHORITY_BASE)
    paths.update(
        path.relative_to(ROOT).as_posix()
        for path in (ROOT / "spec").glob("*.md")
    )
    return paths


def excluded(path: Path) -> bool:
    path_rel = rel(path)
    if any(part in EXCLUDED_PARTS for part in path.parts):
        return True
    return any(path_rel.startswith(prefix) for prefix in EXCLUDED_PREFIXES)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="List high-risk Markdown docs not covered by current authority or ARCHIVE_INDEX.",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="exit nonzero if any high-risk unclassified Markdown files are found",
    )
    parser.add_argument(
        "--include-status-lines",
        action="store_true",
        help=(
            "also list unclassified Markdown files with any explicit Status: "
            "line; useful for staged migration, not enabled in the default gate"
        ),
    )
    parser.add_argument(
        "--include-path2-research",
        action="store_true",
        help=(
            "also list unclassified research/path2*.md notes, even when they "
            "lack an explicit Status: line"
        ),
    )
    parser.add_argument(
        "--include-cryptanalysis-root",
        action="store_true",
        help=(
            "also list unclassified root-level research/cryptanalysis/*.md "
            "security notes"
        ),
    )
    parser.add_argument(
        "--include-all-markdown",
        action="store_true",
        help=(
            "also list every unclassified Markdown file outside the active "
            "authority set and ignored generated/state directories"
        ),
    )
    args = parser.parse_args()

    archive_paths, archive_prefixes = archive_index_classifications()
    classified = archive_paths | current_authority_paths()
    candidates: list[tuple[str, list[str]]] = []
    for path in ROOT.rglob("*.md"):
        if excluded(path):
            continue
        path_rel = rel(path)
        if path_rel in classified or path_rel.startswith("decisions/ADR-"):
            continue
        if any(path_rel.startswith(prefix) for prefix in archive_prefixes):
            continue
        reasons: list[str] = []
        if HIGH_RISK_NAME.search(path.name):
            reasons.append("filename")
        text = path.read_text(encoding="utf-8", errors="replace")
        reasons.extend(
            name for name, pattern in HIGH_RISK_CONTENT.items()
            if pattern.search(text)
        )
        if args.include_status_lines and re.search(r"^Status:\s+.+$", text, re.M):
            reasons.append("status_line")
        if args.include_path2_research and path_rel.startswith("research/path2"):
            reasons.append("path2_research")
        if (
            args.include_cryptanalysis_root
            and path.parent == ROOT / "research" / "cryptanalysis"
        ):
            reasons.append("cryptanalysis_root")
        if args.include_all_markdown:
            reasons.append("all_markdown")
        if reasons:
            candidates.append((path_rel, reasons))

    mode = "strict gate" if args.strict else "advisory mode"
    print(
        "High-risk Markdown classification candidates "
        f"({len(candidates)} files, {mode}):"
    )
    for candidate, reasons in sorted(candidates):
        print(f"{candidate} ({', '.join(reasons)})")
    if args.strict and candidates:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
