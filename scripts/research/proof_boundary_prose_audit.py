#!/usr/bin/env python3
"""Audit review prose for proof-boundary wording drift."""

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
LEDGER = READINESS_DIR / "proof_boundary_ledger_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "proof_boundary_prose_audit_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "proof_boundary_prose_audit_n32_q12289.md"

AUDITED_FILES = [
    READINESS_DIR / "third_party_review_packet.md",
    READINESS_DIR / "bucket1_deterministic_proof_tracker_n32_q12289.md",
    READINESS_DIR / "bucket2_universal_pad_prf_tracker_n32_q12289.md",
    READINESS_DIR / "compact_signature_todos.md",
]

TERMS = [
    "assumption",
    "assumptions",
    "assume",
    "expected",
    "should",
    "informal",
    "informally",
    "believed",
    "external",
    "externally",
    "certified",
    "deployable",
    "deployment-ready",
]


@dataclass(frozen=True)
class Classification:
    boundary_class: str
    status: str


def classify(line: str) -> Classification | None:
    lowered = line.lower()
    if "internally_certified" in lowered or "machine-checked" in lowered:
        return Classification("PBL-001", "internally_certified")
    if "certified" in lowered and ("sampler" in lowered or "compact sampler" in lowered):
        return Classification("PBL-005", "external_review_required")
    if "response-link" in lowered or "selected certificate" in lowered or "pinned hash" in lowered:
        return Classification("PBL-002", "internally_certified_for_selected_certificate")
    if "rust" in lowered or "production-reference" in lowered or "cross-language" in lowered:
        return Classification("PBL-003", "cross_language_tested")
    if "qrom" in lowered or "soundness" in lowered or "query" in lowered or "fork" in lowered:
        return Classification("PBL-005", "external_review_required")
    if "sampler" in lowered or "pad/prf" in lowered or "pad/sampler" in lowered:
        return Classification("PBL-005", "external_review_required")
    if (
        "hardness" in lowered
        or "sis" in lowered
        or "lattice" in lowered
        or "cryptographic" in lowered
        or "keygen" in lowered
        or "estimator" in lowered
        or "assumption" in lowered
        or "assumptions" in lowered
    ):
        return Classification("PBL-004", "external_review_required")
    if (
        "side-channel" in lowered
        or "constant-time" in lowered
        or "storage" in lowered
        or "lifecycle" in lowered
        or "integration" in lowered
        or "deployment" in lowered
        or "deployable" in lowered
        or "deployment-ready" in lowered
    ):
        return Classification("PBL-006", "external_review_required")
    if "external" in lowered or "review" in lowered or "externally" in lowered:
        return Classification("PBL-007", "not_internally_certified")
    if "expected" in lowered or "should" in lowered or "informal" in lowered or "informally" in lowered:
        return Classification("PBL-007", "review_instruction_or_pinned_expectation")
    return None


def main() -> int:
    ledger = json.loads(LEDGER.read_text())
    class_ids = {row["id"] for row in ledger["classes"]}
    observations = []
    failures = []

    for path in AUDITED_FILES:
        for line_no, line in enumerate(path.read_text().splitlines(), start=1):
            lowered = line.lower()
            matched_terms = [term for term in TERMS if term in lowered]
            if not matched_terms:
                continue
            classification = classify(line)
            if classification is None:
                failures.append(
                    {
                        "path": str(path.relative_to(ROOT)),
                        "line": line_no,
                        "terms": matched_terms,
                        "line_text": line.strip(),
                        "failure": "uncategorized_boundary_language",
                    }
                )
                continue
            if classification.boundary_class not in class_ids:
                failures.append(
                    {
                        "path": str(path.relative_to(ROOT)),
                        "line": line_no,
                        "terms": matched_terms,
                        "line_text": line.strip(),
                        "failure": "unknown_boundary_class",
                    }
                )
                continue
            observations.append(
                {
                    "path": str(path.relative_to(ROOT)),
                    "line": line_no,
                    "terms": matched_terms,
                    "boundary_class": classification.boundary_class,
                    "status": classification.status,
                    "line_text": line.strip(),
                }
            )

    report = {
        "schema": "Descender.proof_boundary_prose_audit.v1",
        "ledger": str(LEDGER.relative_to(ROOT)),
        "audited_files": [str(path.relative_to(ROOT)) for path in AUDITED_FILES],
        "terms": TERMS,
        "observation_count": len(observations),
        "uncategorized_count": len(failures),
        "failures": failures,
        "status": "pass" if not failures else "fail",
    }
    DEFAULT_JSON.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")

    lines = [
        "# Proof Boundary Prose Audit",
        "",
        f"- status: `{report['status']}`",
        f"- observations: `{len(observations)}`",
        f"- uncategorized: `{len(failures)}`",
        f"- ledger: `{report['ledger']}`",
        "",
        "## Boundary Counts",
        "",
    ]
    counts: dict[str, int] = {}
    for observation in observations:
        key = f"{observation['boundary_class']}:{observation['status']}"
        counts[key] = counts.get(key, 0) + 1
    for key in sorted(counts):
        lines.append(f"- `{key}`: `{counts[key]}`")
    if failures:
        lines.extend(["", "## Failures", ""])
        for failure in failures:
            lines.append(
                f"- `{failure['path']}:{failure['line']}` {failure['failure']}: "
                f"{failure['line_text']}"
            )
    DEFAULT_MD.write_text("\n".join(lines) + "\n")
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
