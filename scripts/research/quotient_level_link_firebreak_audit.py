#!/usr/bin/env python3
"""Audit that quotient-level link sidecar modules stay out of deployment imports."""

from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_firebreak_audit.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_firebreak_audit.md"

IMPORT_RE = re.compile(r"^import\s+([A-Za-z0-9_'.]+(?:\.[A-Za-z0-9_'.]+)*)\s*$")
SIDECAR_MODULES = {
    "Descender.DescentInterface": "generic descent sidecar interface",
    "Descender.HiddenCommittedQuotientLink": "hidden quotient-link theorem-shape prototype",
}
ALLOWED_IMPORTS = {
    "Descender.DescentInterface": {"Descender/HiddenCommittedQuotientLink.lean"},
    "Descender.HiddenCommittedQuotientLink": set(),
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def lean_files() -> list[Path]:
    files = [ROOT / "Descender.lean"]
    files.extend(sorted((ROOT / "Descender").rglob("*.lean")))
    return [path for path in files if path.exists()]


def module_imports(path: Path) -> list[str]:
    imports: list[str] = []
    for line in path.read_text().splitlines():
        match = IMPORT_RE.match(line)
        if match:
            imports.append(match.group(1))
    return imports


def build_payload() -> dict[str, Any]:
    import_records: list[dict[str, Any]] = []
    violations: list[dict[str, Any]] = []
    for path in lean_files():
        path_rel = rel(path)
        imports = module_imports(path)
        sidecar_imports = [module for module in imports if module in SIDECAR_MODULES]
        if sidecar_imports:
            import_records.append(
                {
                    "path": path_rel,
                    "sidecar_imports": sidecar_imports,
                }
            )
        for module in sidecar_imports:
            if path_rel not in ALLOWED_IMPORTS[module]:
                violations.append(
                    {
                        "path": path_rel,
                        "module": module,
                        "reason": (
                            f"{module} is sidecar-only and may only be imported by "
                            f"{sorted(ALLOWED_IMPORTS[module])}"
                        ),
                    }
                )
    root_imports = module_imports(ROOT / "Descender.lean")
    return {
        "schema": "Descender.quotient_level_link.firebreak_audit.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "sidecar_modules": SIDECAR_MODULES,
        "allowed_imports": {key: sorted(value) for key, value in ALLOWED_IMPORTS.items()},
        "root_sidecar_imports": [module for module in root_imports if module in SIDECAR_MODULES],
        "sidecar_import_records": import_records,
        "violations": violations,
        "passed": not violations,
        "decision": (
            "firebreak_passed"
            if not violations
            else "firebreak_failed_fix_before_implementation_work"
        ),
        "implementation_instruction": (
            "Keep Descender.HiddenCommittedQuotientLink and Descender.DescentInterface "
            "out of the deployable import aggregate unless a later promotion gate "
            "explicitly recommends migration."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Quotient-Level Link Firebreak Audit",
        "",
        f"Status: `{payload['decision']}`.",
        "",
        "This audit checks that sidecar quotient-link modules are not imported by the deployable Lean aggregate.",
        "",
        "## Summary",
        "",
        "| item | value |",
        "|---|---|",
        f"| passed | `{payload['passed']}` |",
        f"| root sidecar imports | `{payload['root_sidecar_imports']}` |",
        f"| violations | `{len(payload['violations'])}` |",
        "",
        payload["implementation_instruction"],
        "",
        "## Allowed Imports",
        "",
        "| module | allowed importers |",
        "|---|---|",
    ]
    for module, allowed in payload["allowed_imports"].items():
        lines.append(f"| `{module}` | `{allowed}` |")
    lines.extend(["", "## Observed Sidecar Imports", ""])
    if payload["sidecar_import_records"]:
        lines.extend(["| path | sidecar imports |", "|---|---|"])
        for record in payload["sidecar_import_records"]:
            lines.append(
                f"| `{record['path']}` | `{record['sidecar_imports']}` |"
            )
    else:
        lines.append("No sidecar imports found.")
    lines.extend(["", "## Violations", ""])
    if payload["violations"]:
        lines.extend(["| path | module | reason |", "|---|---|---|"])
        for violation in payload["violations"]:
            lines.append(
                f"| `{violation['path']}` | `{violation['module']}` | {violation['reason']} |"
            )
    else:
        lines.append("No violations.")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "passed": payload["passed"],
                "violations": len(payload["violations"]),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
