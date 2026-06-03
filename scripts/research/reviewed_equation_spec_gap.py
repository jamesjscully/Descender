#!/usr/bin/env python3
"""Review the concrete equation spec gap for the 250-byte account-carried path."""

from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "reviewed_equation_spec_gap_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "reviewed_equation_spec_gap_n32_q12289.md"
PRODREF = ROOT / "tools/parameter-search/src/production_reference.rs"
STATEFUL = ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs"
README = READINESS_DIR / "README.md"
TODOS = READINESS_DIR / "compact_signature_todos.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read(path: Path) -> str:
    return path.read_text()


def const_usize(source: str, name: str) -> int | None:
    match = re.search(rf"pub const {re.escape(name)}:\s*usize\s*=\s*([0-9_]+)", source)
    if not match:
        match = re.search(rf"const {re.escape(name)}:\s*usize\s*=\s*([0-9_]+)", source)
    if not match:
        return None
    return int(match.group(1).replace("_", ""))


def has_all(source: str, needles: list[str]) -> bool:
    return all(needle in source for needle in needles)


def build_payload() -> dict[str, Any]:
    prodref = read(PRODREF)
    stateful = read(STATEFUL)
    readme = read(README)
    todos = read(TODOS)

    reviewed_rows = const_usize(prodref, "REVIEWED_ACCOUNT_CARRIED_PUBLIC_ROWS")
    reviewed_response_residues = const_usize(
        prodref, "REVIEWED_ACCOUNT_CARRIED_RESPONSE_RESIDUES"
    )
    reviewed_response_bytes = const_usize(
        prodref, "REVIEWED_ACCOUNT_CARRIED_MASKED_RESPONSE_BYTES"
    )
    legacy_public_rows = const_usize(stateful, "SIS_FULL_RESPONSE_PUBLIC_ROWS")

    legacy_fq12_pinned = has_all(
        stateful,
        [
            "CHALLENGE_ACTION_RESIDUES",
            "CHALLENGE_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH",
            "challenge_action.fq12.v1",
            "X^12 + X + 25",
        ],
    )
    reviewed_codec_pinned = has_all(
        prodref,
        [
            "REVIEWED_ACCOUNT_CARRIED_PUBLIC_ROWS",
            "REVIEWED_ACCOUNT_CARRIED_RESPONSE_RESIDUES",
            "reviewed_account_carried_response_syndrome",
            "verify_reviewed_account_carried_transcript_binding",
        ],
    )
    reviewed_equation_action_pinned = (
        "REVIEWED_ACCOUNT_CARRIED_CHALLENGE_ACTION" in prodref
        or "reviewed_account_carried_challenge_action" in prodref
        or "challenge_action.fq10" in prodref
    )
    docs_name_reviewed_10x21 = "10 x 21" in readme and "10 x 21" in todos
    docs_name_legacy_fq12 = "Fq12" in readme or "F_12289[X]/(X^12 + X + 25)" in readme

    dimension_mismatch = (
        reviewed_rows is not None
        and legacy_public_rows is not None
        and reviewed_rows != legacy_public_rows
        and legacy_fq12_pinned
        and not reviewed_equation_action_pinned
    )
    blockers = []
    if not reviewed_equation_action_pinned:
        blockers.append(
            "reviewed 10-row account-carried equation has no pinned challenge-action domain/algebra"
        )
    if dimension_mismatch:
        blockers.append(
            "legacy executable equation is pinned to 12 rows/Fq12 while reviewed byte target is 10 rows/10x21"
        )
    if not reviewed_codec_pinned:
        blockers.append("reviewed 250-byte codec/quotient-map surface is not fully pinned")

    status = "blocked" if blockers else "ready_for_equation_implementation"
    return {
        "schema": "Descender.reviewed_equation_spec_gap.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": {
            "n": 32,
            "q": 12289,
            "transaction_bytes": 250,
            "account_link_bytes": 68,
        },
        "sources": {
            "production_reference": rel(PRODREF),
            "legacy_stateful_reference": rel(STATEFUL),
            "readme": rel(README),
            "todos": rel(TODOS),
        },
        "reviewed_target": {
            "public_rows": reviewed_rows,
            "response_residues": reviewed_response_residues,
            "masked_response_bytes": reviewed_response_bytes,
            "codec_and_transcript_binding_pinned": reviewed_codec_pinned,
            "equation_challenge_action_pinned": reviewed_equation_action_pinned,
        },
        "legacy_executable_equation": {
            "public_rows": legacy_public_rows,
            "fq12_action_pinned": legacy_fq12_pinned,
            "domain": "Descender.Signature.q12289.local_delta.challenge_action.fq12.v1"
            if legacy_fq12_pinned
            else None,
            "polynomial": "X^12 + X + 25" if legacy_fq12_pinned else None,
        },
        "documentation_crosscheck": {
            "docs_name_reviewed_10x21": docs_name_reviewed_10x21,
            "docs_name_legacy_fq12": docs_name_legacy_fq12,
        },
        "findings": [
            {
                "severity": "high",
                "status": "deployment_blocker",
                "finding": blocker,
            }
            for blocker in blockers
        ],
        "decision": {
            "status": status,
            "deployment_ready": False,
            "safe_implementation_path": "Do not implement a guessed equation. First pin a reviewed challenge-action algebra/domain for the 10-row quotient, update the Lean/QROM budget to that algebra, then implement the Rust verifier equation against the pinned object.",
            "viable_options": [
                "Define a reviewed Fq10-style challenge action for the 10-residue digest and regenerate QROM/action-collision evidence.",
                "Change the reviewed target back to a 12-row quotient/action, accepting the byte and proof-model consequences.",
                "Add an explicit adapter proving how a 12-residue action soundly acts on the 10-residue account-carried quotient without public-projection leakage.",
            ],
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    reviewed = payload["reviewed_target"]
    legacy = payload["legacy_executable_equation"]
    decision = payload["decision"]
    lines = [
        "# Reviewed Equation Spec Gap",
        "",
        "This artifact checks whether the reviewed 250-byte account-carried path",
        "has enough pinned algebra to implement its final verifier equation in",
        "Rust without inventing a new proof assumption.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- target: `n={target['n']}`, `q={target['q']}`",
        f"- transaction bytes: `{target['transaction_bytes']}`",
        f"- account-link bytes: `{target['account_link_bytes']}`",
        f"- reviewed rows: `{reviewed['public_rows']}`",
        f"- reviewed response residues: `{reviewed['response_residues']}`",
        f"- reviewed masked response bytes: `{reviewed['masked_response_bytes']}`",
        f"- reviewed equation challenge action pinned: `{reviewed['equation_challenge_action_pinned']}`",
        f"- legacy executable rows: `{legacy['public_rows']}`",
        f"- legacy Fq12 action pinned: `{legacy['fq12_action_pinned']}`",
        f"- status: `{decision['status']}`",
        f"- deployment ready: `{decision['deployment_ready']}`",
        "",
        "## Findings",
        "",
        "| severity | status | finding |",
        "|---|---|---|",
    ]
    for finding in payload["findings"]:
        lines.append(
            f"| {finding['severity']} | {finding['status']} | {finding['finding']} |"
        )
    if not payload["findings"]:
        lines.append("| info | clear | no equation-spec blocker detected |")
    lines.extend(
        [
            "",
            "## Safe Implementation Path",
            "",
            decision["safe_implementation_path"],
            "",
            "## Viable Options",
            "",
        ]
    )
    for option in decision["viable_options"]:
        lines.append(f"- {option}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD), "status": payload["decision"]["status"]}, indent=2))


if __name__ == "__main__":
    main()
