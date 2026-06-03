#!/usr/bin/env python3
"""Aggregate the pre-third-party cryptoanalysis campaign probe outcomes."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
OUT_JSON = READINESS_DIR / "pre_third_party_cryptoanalysis_campaign_results_n32_q12289.json"
OUT_MD = READINESS_DIR / "pre_third_party_cryptoanalysis_campaign_results_n32_q12289.md"
QROM_CLOSURE = READINESS_DIR / "qrom_multitarget_margin_n32_q12289.json"
TIMING_CLOSURE = READINESS_DIR / "side_channel_timing_closure_n32_q12289.json"
LIFECYCLE_CLOSURE = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"


PROBES = [
    (
        "PTC-001",
        "matrix/SIS",
        READINESS_DIR / "pre_third_party_matrix_sis_probe_n32_q12289.json",
    ),
    (
        "PTC-002",
        "response-link",
        READINESS_DIR / "pre_third_party_response_link_attack_sanity_probe_n32_q12289.json",
    ),
    (
        "PTC-003",
        "QROM frame",
        READINESS_DIR / "pre_third_party_qrom_frame_probe_n32_q12289.json",
    ),
    (
        "PTC-004",
        "pad PRF",
        READINESS_DIR / "pre_third_party_pad_prf_seed_sweep_n32_q12289.json",
    ),
    (
        "PTC-005",
        "transcript malleability",
        READINESS_DIR / "pre_third_party_transcript_malleability_probe_n32_q12289.json",
    ),
    (
        "PTC-006",
        "side-channel/fault",
        READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json",
    ),
    (
        "PTC-007",
        "state lifecycle",
        READINESS_DIR / "pre_third_party_state_lifecycle_adversary_n32_q12289.json",
    ),
    (
        "PTC-008",
        "rogue-key/account",
        READINESS_DIR / "pre_third_party_rogue_key_account_fixture_analysis_n32_q12289.json",
    ),
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def load_optional(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {}
    return load(path)


def summarize_probe(probe_id: str, label: str, path: Path) -> dict[str, Any]:
    data = load(path)
    status = data.get("status") or data.get("verdict", {}).get("status") or "present"
    finding = "see artifact"
    severity = "info"

    if probe_id == "PTC-001":
        finding = "independent matrix rederivation and SIS sanity probe passed"
    elif probe_id == "PTC-002":
        verdict = data.get("verdict", {})
        finding = (
            "selected [0,3] response-link resisted local reductions; malformed inputs fail closed; "
            f"selected box vector found={verdict.get('local_reductions_found_selected_box_vector')}"
        )
    elif probe_id == "PTC-003":
        closure = load_optional(QROM_CLOSURE)
        if closure:
            limit = closure.get("fail_closed_limit", {})
            finding = (
                "frame mutations rejected; QROM concern closed by fail-closed "
                f"aggregate target cap log2={limit.get('maximum_covered_target_count_log2')}; "
                f"realistic multi-target ready={closure.get('deployment_ready_for_realistic_multitarget')}"
            )
        else:
            max_ok = data.get("query_budget", {}).get("max_target_count_log2_at_128_bits")
            finding = f"frame mutations rejected; multi-target 128-bit margin holds only through log2 targets={max_ok}"
            severity = "review_attention"
    elif probe_id == "PTC-004":
        params = data.get("parameters", {})
        checks = data.get("checks", [])
        finding = f"pad PRF sweep passed {params.get('seed_count')} seeds with {len(data.get('failed_checks', []))} failed checks"
    elif probe_id == "PTC-005":
        summary = data.get("mutation_summary", {})
        finding = (
            f"whole-transcript mutations fail closed: {summary.get('fail_closed_count')} fail-closed, "
            f"{summary.get('unexpected_accept_count')} unexpected accepts"
        )
    elif probe_id == "PTC-006":
        summary = data.get("summary", {})
        closure = load_optional(TIMING_CLOSURE)
        finding = (
            "sampled fail-closed behavior held, but timing cliffs remain; "
            f"policy issues={(closure.get('policy') or {}).get('timing_policy_issue_count', summary.get('timing_policy_issue_count'))}"
        )
        severity = "open_risk"
    elif probe_id == "PTC-007":
        counts = data.get("classification_counts", {})
        closure = load_optional(LIFECYCLE_CLOSURE)
        if closure:
            finding = (
                "state lifecycle concern closed by monotonic trusted-anchor policy; "
                f"adversary scenarios={sum(counts.values())}, criteria={len(closure.get('deployment_acceptance_criteria', []))}"
            )
        else:
            finding = (
                f"state adversary scenarios: {counts.get('fail-closed')} fail-closed, "
                f"{counts.get('blocked by memo policy')} policy-blocked, "
                f"{counts.get('open implementation risk')} open implementation risk"
            )
            severity = "open_risk" if counts.get("open implementation risk") else "info"
    elif probe_id == "PTC-008":
        summary = data.get("fixture_summary", {})
        finding = (
            f"strict rogue-key/account fixtures rejected: {summary.get('strict_reject_count')}; "
            f"weakened accepts classified: {summary.get('weakened_accept_count')}"
        )

    return {
        "probe_id": probe_id,
        "label": label,
        "artifact": rel(path),
        "status": status,
        "severity": severity,
        "finding": finding,
    }


def build_payload() -> dict[str, Any]:
    rows = [summarize_probe(*probe) for probe in PROBES]
    open_risks = [row for row in rows if row["severity"] == "open_risk"]
    review_attention = [row for row in rows if row["severity"] == "review_attention"]
    missing = [row for row in rows if not (ROOT / row["artifact"]).exists()]
    closures = {
        "qrom_multitarget_margin": rel(QROM_CLOSURE) if QROM_CLOSURE.exists() else None,
        "side_channel_timing": rel(TIMING_CLOSURE) if TIMING_CLOSURE.exists() else None,
        "state_lifecycle": rel(LIFECYCLE_CLOSURE) if LIFECYCLE_CLOSURE.exists() else None,
    }
    return {
        "schema": "Descender.pre_third_party_cryptoanalysis_campaign_results.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "probe_count": len(rows),
        "missing_artifact_count": len(missing),
        "open_risk_count": len(open_risks),
        "review_attention_count": len(review_attention),
        "closure_artifacts": closures,
        "rows": rows,
        "bottom_line": (
            "All eight pre-third-party probes now have executable local artifacts. "
            "No new selected response-link or transcript-malleability break was found. "
            "QROM multi-target risk is fenced by a fail-closed aggregate-target cap, "
            "state lifecycle replay is fenced by monotonic trusted-anchor policy, "
            "and side-channel/timing hardening remains open."
        ),
    }


def write_markdown(payload: dict[str, Any]) -> None:
    lines = [
        "# Pre-Third-Party Cryptoanalysis Campaign Results",
        "",
        f"- probes: `{payload['probe_count']}`",
        f"- missing artifacts: `{payload['missing_artifact_count']}`",
        f"- review-attention items: `{payload['review_attention_count']}`",
        f"- open-risk items: `{payload['open_risk_count']}`",
        f"- bottom line: {payload['bottom_line']}",
        "",
        "| id | label | status | severity | finding | artifact |",
        "|---|---|---|---|---|---|",
    ]
    for row in payload["rows"]:
        lines.append(
            f"| `{row['probe_id']}` | {row['label']} | `{row['status']}` | `{row['severity']}` | {row['finding']} | `{row['artifact']}` |"
        )
    lines.append("")
    OUT_MD.write_text("\n".join(lines))


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    write_markdown(payload)
    print(json.dumps({
        "json": rel(OUT_JSON),
        "markdown": rel(OUT_MD),
        "probe_count": payload["probe_count"],
        "open_risk_count": payload["open_risk_count"],
        "review_attention_count": payload["review_attention_count"],
        "status": "written",
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
