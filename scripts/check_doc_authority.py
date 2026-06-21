#!/usr/bin/env python3
"""Check Descender documentation authority markers.

This is intentionally narrow. It does not try to classify every research note;
it enforces the process invariants introduced by the pivot:

- fixed active authority entry points carry explicit status markers;
- package entrypoints that prevent stale-path confusion carry explicit markers;
- direct tool packages are indexed by tools/README.md;
- spec spine files are discovered from spec/*.md and carry active normative markers;
- README.md preserves chain-native product framing and non-active path routing;
- spec/README.md and README.md share the checked active product direction and
  non-active path lists;
- README.md indexes the repo authority entrypoints exactly;
- README.md supporting entrypoints route back to spec/ADR/tracker authority;
- README.md local process checks mirror spec/process.md gate coverage;
- high-confusion historical files listed in ARCHIVE_INDEX have migration banners;
- archive labels have explicit retention/disposition policy rows;
- archive retention/disposition policy rows match the checked label contract;
- legacy archive labels route to their replacement authority;
- migration banner labels agree with ARCHIVE_INDEX classifications;
- Status: ACTIVE_* markers use the approved authority/routing vocabulary;
- active package README rows listed in ARCHIVE_INDEX have explicit routing banners;
- active README/package entrypoints route back to spec/ADR/tracker authority;
- active support files selected by the spec have ACTIVE_SUPPORT classifications
  and current spec/ADR/tracker anchors.
- spec/README.md indexes the normative spec spine exactly;
- active support spec notes outside spec/ defer to spec/README.md;
- active tracker/todo/status/checklist support files and active detail-status
  notes defer to TRACKER.md and are linked by TRACKER.md;
- tracker/todo/status/checklist/plan Markdown files covered only by package
  archive rows carry local package-context banners;
- active decision-note support files defer to decisions/ ADRs;
- ADR files have the expected append-only record shape;
- decisions/README.md states the append-only ADR process contract;
- ADR affected tracker rows reciprocally cite the ADR that names them;
- decisions/README.md indexes the ADR files exactly;
- TRACKER rows use the allowed status vocabulary.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

FIXED_REQUIRED_MARKERS = {
    "README.md": "Status: ACTIVE_REPO_ENTRYPOINT",
    "decisions/README.md": "Status: ACTIVE_NORMATIVE_PROCESS",
    "TRACKER.md": "Status: ACTIVE_TRACKER",
    "research/README.md": "Status: ACTIVE_RESEARCH_ENTRYPOINT",
    "research/ARCHIVE_INDEX.md": "Status: ACTIVE_ARCHIVE_INDEX",
    "audit/README.md": "Status: ACTIVE_AUDIT_ENTRYPOINT",
    "formal/README.md": "Status: ACTIVE_FORMAL_ENTRYPOINT",
    "legal/README.md": "Status: ACTIVE_LEGAL_ENTRYPOINT",
    "scripts/research/README.md": "Status: ACTIVE_RESEARCH_AUTOMATION_ENTRYPOINT",
    "tools/README.md": "Status: ACTIVE_TOOLS_ENTRYPOINT",
}

SUPPORTING_ENTRYPOINTS = {
    "research/README.md",
    "audit/README.md",
    "tools/README.md",
    "formal/README.md",
    "legal/README.md",
    "scripts/research/README.md",
}

SPEC_STATUS_MARKERS = (
    "Status: ACTIVE_NORMATIVE",
    "Status: ACTIVE_NORMATIVE_FOR_AUTHORITY_MAP",
)

LEGACY_LABELS = {
    "LEGACY_FORMAL",
    "ARCHIVED_TARGET",
    "SUPERSEDED_TRACKER",
    "SUPERSEDED_DECISION",
    "MIXED_STATUS",
    "NEGATIVE_CONTROL",
}

ACTIVE_LABELS = {
    "ACTIVE_SUPPORT",
    "ACTIVE_RESEARCH",
}

ACTIVE_STATUS_MARKERS = {
    "ACTIVE_AUDIT_ENTRYPOINT",
    "ACTIVE_ARCHIVE_INDEX",
    "ACTIVE_FORMAL_ENTRYPOINT",
    "ACTIVE_LEGAL_ENTRYPOINT",
    "ACTIVE_NORMATIVE",
    "ACTIVE_NORMATIVE_FOR_AUTHORITY_MAP",
    "ACTIVE_NORMATIVE_PROCESS",
    "ACTIVE_REPO_ENTRYPOINT",
    "ACTIVE_RESEARCH",
    "ACTIVE_RESEARCH_AUTOMATION_ENTRYPOINT",
    "ACTIVE_RESEARCH_ENTRYPOINT",
    "ACTIVE_SUPPORT",
    "ACTIVE_TOOLS_ENTRYPOINT",
    "ACTIVE_TRACKER",
}

ADR_STATUSES = {
    "Accepted",
    "Proposed",
    "Rejected",
    "Superseded",
}

ADR_REQUIRED_SECTIONS = (
    "## Context",
    "## Decision",
    "## Consequences",
    "## Supersedes",
    "## Superseded By",
    "## Affected Specs / Tracker Items",
)

ADR_PROCESS_README_PHRASES = (
    "Decision records are append-only.",
    "If a decision changes, add a new ADR and mark the old one `Superseded` with a pointer to the replacement.",
    "Each ADR must include:",
    "- `Status:` and `Date:` fields;",
    "- `## Context`, `## Decision`, and `## Consequences` sections;",
    "- `## Supersedes`, `## Superseded By`, and `## Affected Specs / Tracker Items` sections.",
    "Use `None.` for `Supersedes` or `Superseded By` when there is no prior or later decision.",
    "Put affected specs, ADRs, and tracker row IDs in backticks.",
    "If an ADR names a tracker row, that row must cite the ADR number as reciprocal decision coverage.",
    "For final ADRs (`Accepted`, `Superseded`, or `Rejected`), the `## Context`, `## Decision`, and `## Consequences` sections are treated as immutable decision text.",
    "Their hashes live in `ADR_IMMUTABLE_SECTIONS.sha256` and are checked by `scripts/check_adr_immutable_sections.py`.",
    "Accepted decision status values are `Accepted`, `Proposed`, `Rejected`, and `Superseded`.",
)

README_FRAMING_PHRASES = (
    "Descender is being pivoted into a chain-native post-quantum authorization stack.",
    "It is not currently presented as a standalone drop-in signature primitive or as a deployment-ready cryptographic system.",
    "Use these files as the current authority chain:",
    "When older research notes disagree with `spec/` or the ADRs, treat the older notes as history unless the current spec explicitly cites them as active support.",
    "These entrypoints route implementation, research, audit, and operating material back to the authority chain above.",
    "They are not product authority by themselves.",
)

ACTIVE_PRODUCT_DIRECTION = (
    "compact PQ spend authorization",
    "+ prevalidated one-use account slots",
    "+ consensus/state binding",
    "+ account policy expressiveness",
    "+ proof aggregation / rollup compatibility",
)

NON_ACTIVE_PRODUCT_PATHS = (
    "- the 260-byte legacy linear-sigma / KAT v0 path;",
    "- the 308/314-byte Fq12 projection-carrier envelope as the final product story;",
    "- spend-time hidden-witness extraction from the old projection-carrier transcript;",
    "- native BLS-style signature aggregation;",
    '- a raw "replace ECDSA with PQ CHECKSIG" framing.',
)

TRACKER_STATUSES = {
    "proposed",
    "selected",
    "in_progress",
    "implemented",
    "verified",
    "blocked",
    "archived",
    "superseded",
}

EXPECTED_ARCHIVE_RETENTION_POLICY = {
    "ACTIVE_SUPPORT": (
        "keep as cited support for the current spec spine",
        "remove only after the importing spec, ADR, or `TRACKER.md` row is updated to no longer depend on it",
    ),
    "ACTIVE_RESEARCH": (
        "keep as non-normative research evidence",
        "remove or relocate after no current spec, ADR, tracker row, selected support row, or active package entrypoint imports it",
    ),
    "NEGATIVE_CONTROL": (
        "keep as attack, failed-route, or regression evidence",
        "remove only after the negative case is captured in an active regression/KAT/audit artifact or explicitly retired by ADR or tracker row",
    ),
    "LEGACY_FORMAL": (
        "keep as legacy formal/KAT/conformance history",
        "remove or relocate after current formal authority and regression evidence replace the retained value, with tracker or ADR coverage",
    ),
    "ARCHIVED_TARGET": (
        "archive only as old product-target history",
        "remove or relocate after dependencies are gone and a tracker row or ADR records the retirement",
    ),
    "SUPERSEDED_TRACKER": (
        "archive only until all live work has been copied into `TRACKER.md`",
        "remove or relocate after `TRACKER.md` owns all remaining live items and no active support row imports the old tracker",
    ),
    "SUPERSEDED_DECISION": (
        "archive only as pre-ADR decision history",
        "remove or relocate after the relevant ADRs cover the history and no active support row imports the old decision note",
    ),
    "MIXED_STATUS": (
        "keep temporarily while useful evidence is split or routed",
        "split into active support, negative-control, or archived target material before removal, or retire by tracker row or ADR",
    ),
}

ARCHIVE_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|")
ARCHIVE_RETENTION_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+(.+?)\s+\|\s+(.+?)\s+\|")
MIGRATION_BANNER_LABEL = re.compile(r"MIGRATION STATUS[^`]*`([A-Z0-9_]+)`")
SELECTED_SUPPORT_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+(.+?)\s+\|")
CODE_SPAN = re.compile(r"`([^`]+)`")
ADR_REF = re.compile(r"^ADR-(\d{4})$")
ADR_FILE = re.compile(r"^ADR-(\d{4})-[a-z0-9][a-z0-9-]*\.md$")
ADR_INDEX_ROW = re.compile(
    r"^\|\s+`ADR-(\d{4})`\s+\|\s+`([^`]+)`\s+\|\s+`(\d{4}-\d{2}-\d{2})`\s+\|\s+(.+?)\s+\|\s+`([^`]+)`\s+\|"
)
TABLE_PATH_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|")
TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")
SUBORDINATE_DETAIL_DOC = re.compile(r"(tracker|todo|status|checklist)", re.I)
PACKAGE_DETAIL_DOC = re.compile(r"(tracker|todo|status|checklist|plan)", re.I)
SUPPORT_DECISION_NOTE = re.compile(r"decision", re.I)
SUPPORT_SPEC_NOTE = re.compile(r"spec", re.I)
ACTIVE_STATUS_LINE = re.compile(r"^Status:\s+(ACTIVE_[A-Z0-9_]+)\s*$", re.MULTILINE)


def read_rel(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def first_lines(text: str, count: int = 12) -> str:
    return "\n".join(text.splitlines()[:count])


def package_dir(rel: str) -> Path | None:
    if not rel.endswith("/**"):
        return None
    return ROOT / rel[:-3]


def archive_target_exists(rel: str) -> bool:
    pkg = package_dir(rel)
    if pkg is not None:
        return pkg.is_dir()
    return (ROOT / rel).exists()


def banner_path_for_archive_row(rel: str) -> Path:
    pkg = package_dir(rel)
    if pkg is not None:
        return pkg / "README.md"
    return ROOT / rel


def package_entrypoint_path(rel: str) -> Path | None:
    pkg = package_dir(rel)
    if pkg is None:
        return None
    markdown = pkg / "README.md"
    if markdown.exists():
        return markdown
    upstream_rst = pkg / "README.rst"
    if rel.startswith("tools/external/") and upstream_rst.exists():
        return upstream_rst
    return None


def tracker_item_ids() -> set[str]:
    ids: set[str] = set()
    for line in read_rel("TRACKER.md").splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if cells and TRACKER_ITEM_ID.match(cells[0]):
            ids.add(cells[0])
    return ids


def tracker_item_lines() -> dict[str, str]:
    lines: dict[str, str] = {}
    for line in read_rel("TRACKER.md").splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if cells and TRACKER_ITEM_ID.match(cells[0]):
            lines[cells[0]] = line
    return lines


def looks_repo_path(ref: str) -> bool:
    if ref in {"README.md", "TRACKER.md"}:
        return True
    return ref.startswith(
        (
            "audit/",
            "decisions/",
            "formal/",
            "legal/",
            "research/",
            "scripts/",
            "spec/",
            "tools/",
        )
    )


def check_required_markers(errors: list[str]) -> None:
    for rel, marker in FIXED_REQUIRED_MARKERS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing required authority file")
            continue
        head = first_lines(path.read_text(encoding="utf-8"), 8)
        if marker not in head:
            errors.append(f"{rel}: missing required marker {marker!r} in first 8 lines")


def check_active_status_marker_vocabulary(errors: list[str]) -> None:
    for path in ROOT.rglob("*.md"):
        if any(part in {".git", ".lake", "target", "node_modules"} for part in path.parts):
            continue
        rel = path.relative_to(ROOT).as_posix()
        text = path.read_text(encoding="utf-8", errors="replace")
        for match in ACTIVE_STATUS_LINE.finditer(text):
            marker = match.group(1)
            if marker not in ACTIVE_STATUS_MARKERS:
                line_no = text[:match.start()].count("\n") + 1
                errors.append(f"{rel}:{line_no}: unknown active Status marker {marker}")


def archive_rows(errors: list[str]) -> list[tuple[str, str]]:
    path = ROOT / "research/ARCHIVE_INDEX.md"
    if not path.exists():
        errors.append("research/ARCHIVE_INDEX.md: missing")
        return []

    rows: list[tuple[str, str]] = []
    seen: set[str] = set()
    for line_no, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        match = ARCHIVE_ROW.match(line)
        if not match:
            continue
        rel, label = match.groups()
        if rel in {"File", "Package"}:
            continue
        rows.append((rel, label))
        if rel in seen:
            errors.append(f"research/ARCHIVE_INDEX.md:{line_no}: duplicate archive row: {rel}")
        seen.add(rel)
        pkg = package_dir(rel)
        if pkg is not None and package_entrypoint_path(rel) is None:
            errors.append(
                f"research/ARCHIVE_INDEX.md:{line_no}: package row requires README.md"
                f" or allowed vendored README.rst: {rel}"
            )
        if not archive_target_exists(rel):
            errors.append(f"research/ARCHIVE_INDEX.md:{line_no}: listed file does not exist: {rel}")
        if label not in LEGACY_LABELS | ACTIVE_LABELS:
            errors.append(f"research/ARCHIVE_INDEX.md:{line_no}: unknown label {label!r}")
    return rows


def check_archive_retention_policy(errors: list[str]) -> None:
    path = ROOT / "research/ARCHIVE_INDEX.md"
    if not path.exists():
        errors.append("research/ARCHIVE_INDEX.md: missing")
        return
    section = markdown_section(
        path.read_text(encoding="utf-8"),
        "## Retention Policy",
    )
    if not section:
        errors.append("research/ARCHIVE_INDEX.md: missing ## Retention Policy section")
        return

    known_labels = LEGACY_LABELS | ACTIVE_LABELS
    if set(EXPECTED_ARCHIVE_RETENTION_POLICY) != known_labels:
        errors.append(
            "scripts/check_doc_authority.py: expected archive retention policy "
            "labels do not match recognized archive labels"
        )
    seen: set[str] = set()
    for line in section:
        match = ARCHIVE_RETENTION_ROW.match(line)
        if not match:
            continue
        label, disposition, removal_rule = match.groups()
        if label in seen:
            errors.append(f"research/ARCHIVE_INDEX.md: duplicate retention policy row: {label}")
        seen.add(label)
        if label not in known_labels:
            errors.append(f"research/ARCHIVE_INDEX.md: unknown retention policy label {label!r}")
        if not disposition.strip():
            errors.append(f"research/ARCHIVE_INDEX.md: retention policy row missing disposition: {label}")
        if not removal_rule.strip():
            errors.append(f"research/ARCHIVE_INDEX.md: retention policy row missing removal rule: {label}")
        expected = EXPECTED_ARCHIVE_RETENTION_POLICY.get(label)
        if expected is not None:
            expected_disposition, expected_removal_rule = expected
            if disposition.strip() != expected_disposition:
                errors.append(
                    "research/ARCHIVE_INDEX.md: retention policy disposition "
                    f"mismatch for {label}: {disposition.strip()!r} != "
                    f"{expected_disposition!r}"
                )
            if removal_rule.strip() != expected_removal_rule:
                errors.append(
                    "research/ARCHIVE_INDEX.md: retention policy removal rule "
                    f"mismatch for {label}: {removal_rule.strip()!r} != "
                    f"{expected_removal_rule!r}"
                )

    missing = sorted(known_labels - seen)
    if missing:
        errors.append(
            "research/ARCHIVE_INDEX.md: Retention Policy is missing labels: "
            + ", ".join(missing)
        )


def check_archive_banners(rows: list[tuple[str, str]], errors: list[str]) -> None:
    known_labels = LEGACY_LABELS | ACTIVE_LABELS
    replacement_routes = {
        "ARCHIVED_TARGET": ("spec/README.md",),
        "LEGACY_FORMAL": ("spec/README.md",),
        "MIXED_STATUS": ("spec/README.md", "spec/security_model.md", "TRACKER.md", "decisions/", "ADR"),
        "SUPERSEDED_TRACKER": ("TRACKER.md",),
        "SUPERSEDED_DECISION": ("decisions/", "ADR"),
    }
    for rel, label in rows:
        path = banner_path_for_archive_row(rel)
        if not path.exists():
            if label in LEGACY_LABELS:
                errors.append(f"{rel}: archive label {label} requires a README.md banner")
            continue
        head = first_lines(path.read_text(encoding="utf-8"), 10)
        banner_match = MIGRATION_BANNER_LABEL.search(head)
        if "MIGRATION STATUS" in head:
            if banner_match is None:
                errors.append(f"{rel}: migration banner must name an archive label")
            elif banner_match.group(1) not in known_labels:
                errors.append(
                    f"{rel}: migration banner label {banner_match.group(1)} "
                    "is not a recognized ARCHIVE_INDEX label"
                )
            elif banner_match.group(1) != label:
                errors.append(
                    f"{rel}: migration banner label {banner_match.group(1)} "
                    f"does not match ARCHIVE_INDEX label {label}"
                )
        if label in LEGACY_LABELS:
            if "MIGRATION STATUS" not in head:
                errors.append(f"{rel}: archive label {label} requires a MIGRATION STATUS banner")
            if f"`{label}`" not in head:
                errors.append(f"{rel}: migration banner does not name archive label {label}")
            routes = replacement_routes.get(label)
            if routes is not None and not any(route in head for route in routes):
                errors.append(
                    f"{rel}: archive label {label} must route to replacement authority "
                    f"near the top ({', '.join(routes)})"
                )


def check_active_detail_doc_banners(rows: list[tuple[str, str]], errors: list[str]) -> None:
    tracker_text = read_rel("TRACKER.md")
    for rel, label in rows:
        if label not in ACTIVE_LABELS:
            continue
        path = banner_path_for_archive_row(rel)
        if not path.exists():
            continue
        head = first_lines(path.read_text(encoding="utf-8"), 18)
        status_match = re.search(r"^Status:\s+(.+)$", head, re.IGNORECASE | re.MULTILINE)
        has_detail_status = (
            status_match is not None
            and SUBORDINATE_DETAIL_DOC.search(status_match.group(1)) is not None
        )
        if not SUBORDINATE_DETAIL_DOC.search(Path(rel).name) and not has_detail_status:
            continue
        if "TRACKER.md" not in head:
            errors.append(f"{rel}: active detail doc must defer to TRACKER.md near the top")
            continue
        lowered = head.lower()
        if "top-level" not in lowered and "subordinate" not in lowered:
            errors.append(
                f"{rel}: active detail doc must say it is subordinate or not top-level"
            )
        if rel not in tracker_text:
            errors.append(f"{rel}: active detail doc must be linked from TRACKER.md")


def check_package_detail_doc_banners(rows: list[tuple[str, str]], errors: list[str]) -> None:
    exact_rows = {rel for rel, _label in rows if not rel.endswith("/**")}
    for rel, _label in rows:
        pkg = package_dir(rel)
        if pkg is None or not pkg.exists():
            continue
        for path in sorted(pkg.rglob("*.md")):
            if path.name == "README.md":
                continue
            child_rel = path.relative_to(ROOT).as_posix()
            if child_rel in exact_rows:
                continue
            if not PACKAGE_DETAIL_DOC.search(path.name):
                continue
            head = first_lines(path.read_text(encoding="utf-8"), 14)
            lowered = head.lower()
            if "archive_index.md" not in lowered:
                errors.append(
                    f"{child_rel}: package detail doc must name research/ARCHIVE_INDEX.md "
                    "near the top"
                )
            if not any(
                phrase in lowered
                for phrase in (
                    "package-scoped",
                    "historical package",
                    "not current",
                    "not top-level",
                    "not product authority",
                )
            ):
                errors.append(
                    f"{child_rel}: package detail doc must state it is package-scoped, "
                    "historical, not current, not top-level, or not product authority"
                )
            if not any(
                route in head
                for route in ("spec/README.md", "TRACKER.md", "decisions/", "ADR")
            ):
                errors.append(
                    f"{child_rel}: package detail doc must route to spec/README.md, "
                    "TRACKER.md, or ADRs near the top"
                )


def check_active_decision_note_banners(rows: list[tuple[str, str]], errors: list[str]) -> None:
    for rel, label in rows:
        if label not in ACTIVE_LABELS:
            continue
        if not SUPPORT_DECISION_NOTE.search(Path(rel).name):
            continue
        path = banner_path_for_archive_row(rel)
        if not path.exists():
            continue
        head = first_lines(path.read_text(encoding="utf-8"), 18)
        if "decisions/" not in head and "ADR" not in head:
            errors.append(f"{rel}: active decision note must defer to decisions/ or ADRs")
            continue
        lowered = head.lower()
        if "not an adr" not in lowered and "not binding" not in lowered:
            errors.append(f"{rel}: active decision note must say it is not an ADR or not binding")


def check_active_spec_note_banners(rows: list[tuple[str, str]], errors: list[str]) -> None:
    for rel, label in rows:
        if label not in ACTIVE_LABELS:
            continue
        if rel.startswith("spec/") or not SUPPORT_SPEC_NOTE.search(Path(rel).name):
            continue
        path = banner_path_for_archive_row(rel)
        if not path.exists():
            continue
        head = first_lines(path.read_text(encoding="utf-8"), 18)
        if "spec/README.md" not in head:
            errors.append(f"{rel}: active spec note must defer to spec/README.md")
            continue
        lowered = head.lower()
        if "not a normative spec" not in lowered and "does not supersede" not in lowered:
            errors.append(
                f"{rel}: active spec note must say it is not normative or does not supersede"
            )


def check_active_readme_entrypoint_banners(rows: list[tuple[str, str]], errors: list[str]) -> None:
    for rel, label in rows:
        if label not in ACTIVE_LABELS:
            continue
        package_entrypoint = package_entrypoint_path(rel)
        if not rel.endswith("README.md") and package_entrypoint is None:
            continue
        if package_entrypoint is not None and package_entrypoint.name != "README.md":
            continue
        path = banner_path_for_archive_row(rel)
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        head = first_lines(text, 10)
        if "MIGRATION STATUS" in head:
            pass
        elif not re.search(r"^Status:\s+ACTIVE_", head, re.MULTILINE):
            errors.append(
                f"{rel}: active README entrypoint must have an ACTIVE status "
                "or MIGRATION STATUS banner near the top"
            )
        if "spec/README.md" not in text:
            errors.append(f"{rel}: active README entrypoint must route to spec/README.md")
        if "TRACKER.md" not in text and "decisions/" not in text and "ADR" not in text:
            errors.append(
                f"{rel}: active README entrypoint must route to TRACKER.md or ADRs"
            )


def check_spec_support_exists(rows: list[tuple[str, str]], errors: list[str]) -> None:
    spec = read_rel("spec/README.md")
    archive_labels = dict(rows)
    section = markdown_section(spec, "## Selected Supporting Active Material")
    if not section:
        errors.append("spec/README.md: missing ## Selected Supporting Active Material section")
        return

    tracker_ids = tracker_item_ids()

    selected_rows: list[tuple[str, str]] = []
    for line in section:
        match = SELECTED_SUPPORT_ROW.match(line)
        if match:
            selected_rows.append(match.groups())

    if not selected_rows:
        errors.append("spec/README.md: Selected Supporting Active Material needs anchored table rows")
        return

    for rel, anchor in selected_rows:
        if not (ROOT / rel).exists():
            errors.append(f"spec/README.md: selected support file missing: {rel}")
            continue
        label = archive_labels.get(rel)
        if label != "ACTIVE_SUPPORT":
            errors.append(
                "spec/README.md: selected support file must be in "
                f"ARCHIVE_INDEX.md as ACTIVE_SUPPORT: {rel}"
            )
        refs = CODE_SPAN.findall(anchor)
        if not refs:
            errors.append(f"spec/README.md: selected support row needs an anchor: {rel}")
            continue
        for ref in refs:
            adr_match = ADR_REF.match(ref)
            if adr_match:
                if not any((ROOT / "decisions").glob(f"ADR-{int(adr_match.group(1)):04d}-*.md")):
                    errors.append(f"spec/README.md: selected support row has missing ADR anchor {ref}: {rel}")
                continue
            if TRACKER_ITEM_ID.match(ref):
                if ref not in tracker_ids:
                    errors.append(f"spec/README.md: selected support row has missing tracker anchor {ref}: {rel}")
                continue
            if ref.startswith("spec/"):
                if not (ROOT / ref).exists():
                    errors.append(f"spec/README.md: selected support row has missing path anchor {ref}: {rel}")
                continue
            errors.append(
                "spec/README.md: selected support row anchor must be a spec path, "
                f"ADR, or tracker row: {ref}: {rel}"
            )


def markdown_section(text: str, heading: str) -> list[str]:
    lines = text.splitlines()
    start: int | None = None
    for i, line in enumerate(lines):
        if line.strip() == heading:
            start = i + 1
            break
    if start is None:
        return []

    section: list[str] = []
    for line in lines[start:]:
        if line.startswith("## "):
            break
        section.append(line)
    return section


def check_spec_spine_index(errors: list[str]) -> None:
    spec_readme = ROOT / "spec/README.md"
    if not spec_readme.exists():
        errors.append("spec/README.md: missing spec spine index")
        return

    spec_files = sorted(
        path.relative_to(ROOT).as_posix()
        for path in (ROOT / "spec").glob("*.md")
    )
    expected = set(spec_files) | {"TRACKER.md", "decisions/"}

    text = spec_readme.read_text(encoding="utf-8")
    section = markdown_section(text, "## Normative Documents")
    if not section:
        errors.append("spec/README.md: missing ## Normative Documents section")
        return

    listed: list[str] = []
    for line in section:
        match = TABLE_PATH_ROW.match(line)
        if match:
            listed.append(match.group(1))
    listed_set = set(listed)

    missing = sorted(expected - listed_set)
    extra = sorted(listed_set - expected)
    if missing:
        errors.append(
            "spec/README.md: Normative Documents table is missing: "
            + ", ".join(missing)
        )
    if extra:
        errors.append(
            "spec/README.md: Normative Documents table has unexpected entries: "
            + ", ".join(extra)
        )

    for rel in spec_files:
        head = first_lines((ROOT / rel).read_text(encoding="utf-8"), 8)
        if not any(marker in head for marker in SPEC_STATUS_MARKERS):
            errors.append(f"{rel}: spec spine file lacks an active normative status marker")

    for rel in listed:
        path = ROOT / rel
        if rel.endswith("/"):
            if not path.is_dir():
                errors.append(f"spec/README.md: indexed directory does not exist: {rel}")
        elif not path.exists():
            errors.append(f"spec/README.md: indexed file does not exist: {rel}")


def check_product_direction_block(
    rel: str,
    text: str,
    intro: str,
    errors: list[str],
) -> None:
    direction_match = re.search(
        rf"{re.escape(intro)}\n\n```text\n(.*?)\n```",
        text,
        re.DOTALL,
    )
    if direction_match is None:
        errors.append(f"{rel}: missing active product direction code block")
        return

    actual_direction = tuple(direction_match.group(1).splitlines())
    if actual_direction != ACTIVE_PRODUCT_DIRECTION:
        errors.append(
            f"{rel}: active product direction must be "
            f"{ACTIVE_PRODUCT_DIRECTION!r}; got {actual_direction!r}"
        )


def check_non_active_product_paths(
    rel: str,
    text: str,
    heading: str,
    errors: list[str],
) -> None:
    non_active = markdown_section(text, heading)
    bullets = tuple(line.strip() for line in non_active if line.strip().startswith("- "))
    if bullets != NON_ACTIVE_PRODUCT_PATHS:
        errors.append(
            f"{rel}: non-active product path list must be "
            f"{NON_ACTIVE_PRODUCT_PATHS!r}; got {bullets!r}"
        )


def check_repo_entrypoint_index(errors: list[str]) -> None:
    readme = ROOT / "README.md"
    if not readme.exists():
        errors.append("README.md: missing repo entrypoint")
        return

    spec_files = sorted(
        path.relative_to(ROOT).as_posix()
        for path in (ROOT / "spec").glob("*.md")
    )
    expected = set(spec_files) | {"decisions/", "TRACKER.md", "research/ARCHIVE_INDEX.md"}

    text = readme.read_text(encoding="utf-8")
    section = markdown_section(text, "## Start Here")
    if not section:
        errors.append("README.md: missing ## Start Here section")
        return

    listed: list[str] = []
    for line in section:
        match = TABLE_PATH_ROW.match(line)
        if match:
            listed.append(match.group(1))
    listed_set = set(listed)

    missing = sorted(expected - listed_set)
    extra = sorted(listed_set - expected)
    if missing:
        errors.append("README.md: Start Here table is missing: " + ", ".join(missing))
    if extra:
        errors.append("README.md: Start Here table has unexpected entries: " + ", ".join(extra))

    for rel in listed:
        path = ROOT / rel
        if rel.endswith("/"):
            if not path.is_dir():
                errors.append(f"README.md: indexed directory does not exist: {rel}")
        elif not path.exists():
            errors.append(f"README.md: indexed file does not exist: {rel}")

    support_expected = SUPPORTING_ENTRYPOINTS
    support_section = markdown_section(text, "## Supporting Entrypoints")
    if not support_section:
        errors.append("README.md: missing ## Supporting Entrypoints section")
        return

    support_listed: list[str] = []
    for line in support_section:
        match = TABLE_PATH_ROW.match(line)
        if match:
            support_listed.append(match.group(1))
    support_set = set(support_listed)

    support_missing = sorted(support_expected - support_set)
    support_extra = sorted(support_set - support_expected)
    if support_missing:
        errors.append(
            "README.md: Supporting Entrypoints table is missing: "
            + ", ".join(support_missing)
        )
    if support_extra:
        errors.append(
            "README.md: Supporting Entrypoints table has unexpected entries: "
            + ", ".join(support_extra)
        )

    for rel in support_listed:
        path = ROOT / rel
        if not path.exists():
            errors.append(f"README.md: indexed supporting entrypoint does not exist: {rel}")
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        if "spec/README.md" not in text:
            errors.append(
                f"{rel}: supporting entrypoint must route readers to spec/README.md"
            )
        if "TRACKER.md" not in text and "decisions/" not in text and "ADR" not in text:
            errors.append(
                f"{rel}: supporting entrypoint must route readers to TRACKER.md or ADRs"
            )


def check_readme_framing_contract(errors: list[str]) -> None:
    readme = ROOT / "README.md"
    if not readme.exists():
        errors.append("README.md: missing repo entrypoint")
        return

    text = readme.read_text(encoding="utf-8")
    normalized = re.sub(r"\s+", " ", text)
    for phrase in README_FRAMING_PHRASES:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"README.md: missing repo framing phrase {phrase!r}")

    check_product_direction_block(
        "README.md",
        text,
        "The active product direction is:",
        errors,
    )
    check_non_active_product_paths(
        "README.md",
        text,
        "## Non-Active Product Paths",
        errors,
    )


def check_spec_readme_framing_contract(errors: list[str]) -> None:
    spec_readme = ROOT / "spec" / "README.md"
    if not spec_readme.exists():
        errors.append("spec/README.md: missing spec spine index")
        return

    text = spec_readme.read_text(encoding="utf-8")
    normalized = re.sub(r"\s+", " ", text)
    required_phrases = (
        "This directory is the current single source of truth for Descender's product direction and active protocol claims.",
        "Descender's target is a chain-native post-quantum authorization stack, not a standalone drop-in signature primitive.",
        "Those paths may remain in the repo as archived history, regression tests, negative controls, or formal scaffolding.",
    )
    for phrase in required_phrases:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"spec/README.md: missing spec framing phrase {phrase!r}")

    check_product_direction_block(
        "spec/README.md",
        text,
        "The active goal is:",
        errors,
    )
    check_non_active_product_paths(
        "spec/README.md",
        text,
        "## Current Non-Active Paths",
        errors,
    )


def check_readme_local_process_checks(errors: list[str]) -> None:
    readme = ROOT / "README.md"
    process = ROOT / "spec" / "process.md"
    if not readme.exists() or not process.exists():
        return

    readme_section = markdown_section(
        readme.read_text(encoding="utf-8"),
        "## Local Checks",
    )
    if not readme_section:
        errors.append("README.md: missing ## Local Checks section")
        return

    process_section = markdown_section(
        process.read_text(encoding="utf-8"),
        "## Process Gate Coverage",
    )
    if not process_section:
        errors.append("spec/process.md: missing ## Process Gate Coverage section")
        return

    process_gates = {
        command
        for line in process_section
        for command in CODE_SPAN.findall(line)
        if command.startswith("python3 scripts/")
    }
    readme_text = "\n".join(readme_section)
    for command in sorted(process_gates):
        if command not in readme_text:
            errors.append(
                "README.md: Local Checks must mention process coverage gate: "
                + command
            )
    required_phrases = [
        "The full local build gate is:",
        "./scripts/check_build.sh",
        "Active Path 2 Rust tests are split by runtime:",
        "./scripts/check_active_rust_fast_tests.sh",
        "./scripts/check_active_rust_slow_tests.sh",
        "The fast script is part of the full local/CI gate.",
        "The slow script is explicit manual coverage for empirical hiding batteries, deployed verifier integration, and the larger recursion verifier-AIR suite.",
    ]
    normalized = re.sub(r"\s+", " ", readme_text)
    for phrase in required_phrases:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"README.md: Local Checks missing phrase {phrase!r}")


def check_tools_entrypoint_index(errors: list[str]) -> None:
    readme = ROOT / "tools/README.md"
    if not readme.exists():
        errors.append("tools/README.md: missing tools entrypoint")
        return

    expected: set[str] = set()
    for path in sorted((ROOT / "tools").iterdir()):
        if not path.is_dir():
            continue
        if (path / "Cargo.toml").exists() or (path / "README.md").exists():
            expected.add(path.relative_to(ROOT).as_posix() + "/")

    text = readme.read_text(encoding="utf-8")
    section = markdown_section(text, "## Tool Packages")
    if not section:
        errors.append("tools/README.md: missing ## Tool Packages section")
        return

    listed: list[str] = []
    for line in section:
        match = TABLE_PATH_ROW.match(line)
        if match:
            listed.append(match.group(1))
    listed_set = set(listed)

    missing = sorted(expected - listed_set)
    extra = sorted(listed_set - expected)
    if missing:
        errors.append("tools/README.md: Tool Packages table is missing: " + ", ".join(missing))
    if extra:
        errors.append("tools/README.md: Tool Packages table has unexpected entries: " + ", ".join(extra))

    for rel in listed:
        path = ROOT / rel
        if not rel.endswith("/"):
            errors.append(f"tools/README.md: tool package entry must end with '/': {rel}")
        elif not path.is_dir():
            errors.append(f"tools/README.md: indexed tool package does not exist: {rel}")


def check_adr_lifecycle_references(
    rel: str,
    text: str,
    status: str,
    existing_adr_numbers: set[int],
    known_tracker_ids: set[str],
    errors: list[str],
) -> None:
    for section_name in ("## Supersedes", "## Superseded By"):
        section = "\n".join(markdown_section(text, section_name))
        for match in re.finditer(r"\bADR-(\d{4})\b", section):
            number = int(match.group(1))
            if number not in existing_adr_numbers:
                errors.append(f"{rel}: {section_name} references missing ADR-{number:04d}")

    superseded_by = "\n".join(markdown_section(text, "## Superseded By"))
    if status == "Superseded" and not re.search(r"\bADR-\d{4}\b", superseded_by):
        errors.append(
            f"{rel}: Superseded ADR must name a replacement ADR in "
            "## Superseded By"
        )

    affected = "\n".join(markdown_section(text, "## Affected Specs / Tracker Items"))
    refs = CODE_SPAN.findall(affected)
    if not refs:
        errors.append(
            f"{rel}: ## Affected Specs / Tracker Items must cite paths or tracker IDs"
        )
        return
    for ref in refs:
        if TRACKER_ITEM_ID.match(ref):
            if ref not in known_tracker_ids:
                errors.append(f"{rel}: affected tracker item does not exist: {ref}")
            continue
        if ADR_REF.match(ref):
            number = int(ADR_REF.match(ref).group(1))
            if number not in existing_adr_numbers:
                errors.append(f"{rel}: affected ADR does not exist: {ref}")
            continue
        if looks_repo_path(ref):
            if not (ROOT / ref).exists():
                errors.append(f"{rel}: affected path does not exist: {ref}")
            continue
        errors.append(
            f"{rel}: affected reference is not a path, ADR, or tracker ID: {ref}"
        )


def check_adrs(errors: list[str]) -> list[tuple[int, str, str, str, str]]:
    paths = sorted((ROOT / "decisions").glob("ADR-*.md"))
    if not paths:
        errors.append("decisions/: no ADR files found")
        return []

    numbers: list[int] = []
    existing_adr_numbers = {
        int(match.group(1))
        for path in paths
        if (match := ADR_FILE.match(path.name))
    }
    known_tracker_ids = tracker_item_ids()
    metadata: list[tuple[int, str, str, str, str]] = []
    for path in paths:
        rel = path.relative_to(ROOT).as_posix()
        match = ADR_FILE.match(path.name)
        if not match:
            errors.append(f"{rel}: ADR filename must match ADR-NNNN-short-name.md")
            continue
        number = int(match.group(1))
        numbers.append(number)
        text = path.read_text(encoding="utf-8")
        first_line = text.splitlines()[0] if text.splitlines() else ""
        heading_match = re.match(rf"^# ADR-{number:04d}: (.+)$", first_line)
        if not heading_match:
            errors.append(f"{rel}: first heading must start with '# ADR-{number:04d}: '")
            title = ""
        else:
            title = heading_match.group(1).strip()
        status_match = re.search(r"^Status:\s+(.+)$", text, re.MULTILINE)
        if not status_match:
            errors.append(f"{rel}: missing Status field")
            status = ""
        elif status_match.group(1).strip() not in ADR_STATUSES:
            status = status_match.group(1).strip()
            errors.append(f"{rel}: unknown ADR status {status!r}")
        else:
            status = status_match.group(1).strip()
        date_match = re.search(r"^Date:\s+(\d{4}-\d{2}-\d{2})$", text, re.MULTILINE)
        if not date_match:
            errors.append(f"{rel}: missing Date field in YYYY-MM-DD form")
            date = ""
        else:
            date = date_match.group(1)
        for section in ADR_REQUIRED_SECTIONS:
            if section not in text:
                errors.append(f"{rel}: missing required section {section!r}")
            elif not any(line.strip() for line in markdown_section(text, section)):
                errors.append(f"{rel}: required section {section!r} is empty")
        check_adr_lifecycle_references(
            rel,
            text,
            status,
            existing_adr_numbers,
            known_tracker_ids,
            errors,
        )
        metadata.append((number, status, date, title, path.name))

    expected = list(range(1, len(numbers) + 1))
    if sorted(numbers) != expected:
        errors.append(
            "decisions/: ADR numbers must be contiguous from ADR-0001; "
            f"found {sorted(numbers)}"
        )
    return sorted(metadata)


def check_adr_index(
    expected: list[tuple[int, str, str, str, str]],
    errors: list[str],
) -> None:
    readme = ROOT / "decisions/README.md"
    if not readme.exists():
        errors.append("decisions/README.md: missing ADR index host")
        return
    text = readme.read_text(encoding="utf-8")
    if "## ADR Index" not in text:
        errors.append("decisions/README.md: missing ## ADR Index section")
        return

    actual: list[tuple[int, str, str, str, str]] = []
    for line_no, line in enumerate(text.splitlines(), start=1):
        match = ADR_INDEX_ROW.match(line)
        if not match:
            continue
        number, status, date, title, filename = match.groups()
        actual.append((int(number), status, date, title.strip(), filename))
        if not (ROOT / "decisions" / filename).exists():
            errors.append(
                f"decisions/README.md:{line_no}: indexed ADR file does not exist: {filename}"
            )

    if actual != expected:
        errors.append(
            "decisions/README.md: ADR index does not match ADR files; "
            "update number/status/date/title/file rows"
        )


def check_adr_process_readme_contract(errors: list[str]) -> None:
    readme = ROOT / "decisions/README.md"
    if not readme.exists():
        errors.append("decisions/README.md: missing ADR process document")
        return

    normalized = re.sub(r"\s+", " ", readme.read_text(encoding="utf-8"))
    for phrase in ADR_PROCESS_README_PHRASES:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"decisions/README.md: missing ADR process phrase {phrase!r}")


def check_adr_tracker_reciprocity(errors: list[str]) -> None:
    tracker_rows = tracker_item_lines()
    for path in sorted((ROOT / "decisions").glob("ADR-*.md")):
        match = ADR_FILE.match(path.name)
        if not match:
            continue
        number = int(match.group(1))
        adr_id = f"ADR-{number:04d}"
        rel = path.relative_to(ROOT).as_posix()
        affected = "\n".join(
            markdown_section(
                path.read_text(encoding="utf-8"),
                "## Affected Specs / Tracker Items",
            )
        )
        for ref in CODE_SPAN.findall(affected):
            if not TRACKER_ITEM_ID.match(ref):
                continue
            tracker_row = tracker_rows.get(ref)
            if tracker_row is None:
                # Existence is reported by check_adr_lifecycle_references.
                continue
            if adr_id not in tracker_row:
                errors.append(
                    f"{rel}: affected tracker row {ref} must cite {adr_id}"
                )


def check_tracker_statuses(errors: list[str]) -> None:
    text = read_rel("TRACKER.md")
    for line_no, line in enumerate(text.splitlines(), start=1):
        if not line.startswith("|"):
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if len(cells) < 4 or not TRACKER_ITEM_ID.match(cells[0]):
            continue
        status = cells[1]
        if status not in TRACKER_STATUSES:
            errors.append(f"TRACKER.md:{line_no}: unknown tracker status {status!r}")


def check_entrypoint_stale_claims(errors: list[str]) -> None:
    stale_claims = {
        "formal/README.md": [
            "spec/descender-signature-spec.md`):",
            "There is exactly one **canonical specification**",
        ],
        "research/README.md": [
            "The current deployable-cryptography preparation package",
            "The single instrument for measuring progress",
        ],
        "scripts/research/director.py": [
            "`research_program.md` defines the mathematical and cryptographic target.",
            "`lean_proof_plan.md` describes the Lean proof plan.",
        ],
        "scripts/research/launch_agent.py": [
            "Read research_program.md and lean_proof_plan.md when relevant.",
        ],
        "research/hidden_source_projection_design_fork.md": [
            "The selected 308-byte target exposes",
        ],
        "research/path2_native_boundary_b_construction_tracker.md": [
            "Status: active proof-design tracker.",
        ],
        "research/path2_native_solution_search_todo_tracker.md": [
            "Status: active todo tracker.",
        ],
    }
    for rel, needles in stale_claims.items():
        if not (ROOT / rel).exists():
            continue
        text = read_rel(rel)
        for needle in needles:
            if needle in text:
                errors.append(f"{rel}: stale authority phrase remains: {needle!r}")


def main() -> int:
    errors: list[str] = []
    check_required_markers(errors)
    check_active_status_marker_vocabulary(errors)
    rows = archive_rows(errors)
    check_archive_retention_policy(errors)
    check_archive_banners(rows, errors)
    check_active_detail_doc_banners(rows, errors)
    check_package_detail_doc_banners(rows, errors)
    check_active_decision_note_banners(rows, errors)
    check_active_spec_note_banners(rows, errors)
    check_active_readme_entrypoint_banners(rows, errors)
    check_spec_support_exists(rows, errors)
    check_spec_spine_index(errors)
    check_spec_readme_framing_contract(errors)
    check_readme_framing_contract(errors)
    check_repo_entrypoint_index(errors)
    check_readme_local_process_checks(errors)
    check_tools_entrypoint_index(errors)
    adr_metadata = check_adrs(errors)
    check_adr_process_readme_contract(errors)
    check_adr_index(adr_metadata, errors)
    check_adr_tracker_reciprocity(errors)
    check_tracker_statuses(errors)
    check_entrypoint_stale_claims(errors)

    if errors:
        print("documentation authority check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("documentation authority check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
