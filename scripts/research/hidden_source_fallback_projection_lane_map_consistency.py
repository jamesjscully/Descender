#!/usr/bin/env python3
"""Verify Rust projection lane constants against the selected map certificate."""

from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.md"
)
MAP_SPEC_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rust_array_body(source: str, name: str) -> str:
    pattern = re.compile(rf"{name}:[^=]+=\s*\[(.*?)\];", re.DOTALL)
    match = pattern.search(source)
    if match is None:
        raise RuntimeError(f"missing Rust constant {name}")
    return match.group(1)


def rust_string_array(source: str, name: str) -> list[str]:
    return re.findall(r'"([^"]+)"', rust_array_body(source, name))


def rust_u16_array(source: str, name: str) -> list[int]:
    return [int(item) for item in re.findall(r"\b\d+\b", rust_array_body(source, name))]


def build_payload() -> dict[str, Any]:
    spec = load_json(MAP_SPEC_JSON)
    source = PARAMETER_REFERENCE.read_text()
    rust_symbols = rust_string_array(source, "HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_SYMBOLS")
    rust_coefficients = rust_u16_array(
        source, "HIDDEN_SOURCE_FALLBACK_PROJECTION_LANE_COEFFICIENTS"
    )
    selected_rows = sorted(spec["selected_rows"], key=lambda row: int(row["lane"]))
    selected_symbols = [
        f"{row['kind']}[{int(row['block'])},{int(row['coord'])}]"
        for row in selected_rows
    ]
    selected_coefficients = [int(row["coefficient_mod_q"]) for row in selected_rows]
    rows = []
    for lane in range(max(len(selected_rows), len(rust_symbols), len(rust_coefficients))):
        rows.append(
            {
                "lane": lane,
                "selected_symbol": selected_symbols[lane]
                if lane < len(selected_symbols)
                else None,
                "rust_symbol": rust_symbols[lane] if lane < len(rust_symbols) else None,
                "selected_coefficient": selected_coefficients[lane]
                if lane < len(selected_coefficients)
                else None,
                "rust_coefficient": rust_coefficients[lane]
                if lane < len(rust_coefficients)
                else None,
                "status": "pass"
                if lane < len(selected_symbols)
                and lane < len(rust_symbols)
                and lane < len(selected_coefficients)
                and lane < len(rust_coefficients)
                and selected_symbols[lane] == rust_symbols[lane]
                and selected_coefficients[lane] == rust_coefficients[lane]
                else "fail",
            }
        )
    failures = [row for row in rows if row["status"] != "pass"]
    return {
        "schema": "Descender.hidden_source_fallback_projection_lane_map_consistency.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pass" if not failures else "fail",
        "deployment_ready": False,
        "source_artifacts": {
            "projection_map_exact_certificate_spec": rel(MAP_SPEC_JSON),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "projection_map_exact_certificate_spec_schema": spec.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "carrier_bytes": 56,
            "envelope_bytes": 308,
            "row_digest": spec.get("target", {}).get("row_digest"),
        },
        "lane_rows": rows,
        "coverage": {
            "lane_count": len(rows),
            "passing_lane_count": len(rows) - len(failures),
            "failing_lane_count": len(failures),
        },
        "decision": {
            "can_accept_now": False,
            "reason": "lane constants are consistent with the selected projection map, but deployment remains fail-closed until byte-level account-witness derivation and accepting theorem are complete"
            if not failures
            else "Rust lane constants diverge from the selected projection map certificate",
            "next_artifact": "hidden_source_fallback_projection_account_witness_opening_frame_expansion",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    coverage = payload["coverage"]
    lines = [
        "# Hidden-Source Projection Lane-Map Consistency",
        "",
        "This artifact verifies that the Rust lane-symbol and coefficient constants",
        "match the selected projection-map certificate row order.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- lanes passing: `{coverage['passing_lane_count']}` / `{coverage['lane_count']}`",
        f"- failing lanes: `{coverage['failing_lane_count']}`",
        f"- row digest: `{payload['target']['row_digest']}`",
        "",
        "## Lanes",
        "",
        "| lane | selected symbol | rust symbol | selected coeff | rust coeff | status |",
        "|---:|---|---|---:|---:|---|",
    ]
    for row in payload["lane_rows"]:
        lines.append(
            f"| `{row['lane']}` | `{row['selected_symbol']}` | `{row['rust_symbol']}` | "
            f"`{row['selected_coefficient']}` | `{row['rust_coefficient']}` | `{row['status']}` |"
        )
    lines.extend(["", "## Source Artifacts", ""])
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(f"wrote {rel(DEFAULT_JSON)}")
    print(f"wrote {rel(DEFAULT_MD)}")
    if payload["status"] != "pass":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
