#!/usr/bin/env python3
"""Emit CT-008 extended-by-one equalized diagnostic investigation artifacts."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
CAMPAIGN_JSON = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
OUT_JSON = READINESS_DIR / "ct_issue_extended_by_one_equalized_n32_q12289.json"
OUT_MD = READINESS_DIR / "ct_issue_extended_by_one_equalized_n32_q12289.md"

ISSUE_ID = "CT-008"
GROUP = "equalized_parse_diagnostic"
ISSUE_CLASS = "extended_by_one_equalized"
BASELINE_CLASS = "valid_linear_sigma_signature_equalized"
OPERATION = "equalized_decode_diagnostic"
POLICY_MIN = 0.80
POLICY_MAX = 1.25


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def row_by_class(rows: list[dict[str, Any]], class_name: str) -> dict[str, Any]:
    for row in rows:
        if (
            row.get("group") == GROUP
            and row.get("class") == class_name
            and row.get("operation") == OPERATION
        ):
            return row
    raise KeyError(f"{GROUP}/{class_name}/{OPERATION}")


def ratio(row: dict[str, Any], baseline: dict[str, Any]) -> float:
    return float(row["mean_avg_ns"]) / float(baseline["mean_avg_ns"])


def build_payload() -> dict[str, Any]:
    campaign = json.loads(CAMPAIGN_JSON.read_text(encoding="utf-8"))
    rows = campaign["probe"]["rows"]
    issue_row = row_by_class(rows, ISSUE_CLASS)
    baseline_row = row_by_class(rows, BASELINE_CLASS)
    observed_ratio = ratio(issue_row, baseline_row)
    return {
        "schema": "Descender.ct_issue_extended_by_one_equalized.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "issue_id": ISSUE_ID,
        "group": GROUP,
        "class": ISSUE_CLASS,
        "operation": OPERATION,
        "policy": {
            "ratio_window": [POLICY_MIN, POLICY_MAX],
            "passes": POLICY_MIN <= observed_ratio <= POLICY_MAX,
            "deficit_to_min_ratio": POLICY_MIN - observed_ratio,
        },
        "measurement": {
            "campaign_artifact": rel(CAMPAIGN_JSON),
            "baseline_class": BASELINE_CLASS,
            "baseline_mean_avg_ns": baseline_row["mean_avg_ns"],
            "baseline_median_avg_ns": baseline_row["median_avg_ns"],
            "issue_mean_avg_ns": issue_row["mean_avg_ns"],
            "issue_median_avg_ns": issue_row["median_avg_ns"],
            "ratio_to_equalized_valid_baseline": observed_ratio,
            "samples": issue_row["samples"],
            "batch_iterations": issue_row["batch_iterations"],
            "avg_ns_samples": issue_row["avg_ns_samples"],
            "observed_outcome": issue_row["observed_outcome"],
        },
        "evidence": [
            {
                "source": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "178-201",
                "finding": (
                    "equalized_decode_diagnostic begins with signature.decode() on the original "
                    "input, so malformed public length still controls the first decode path before "
                    "the normalized-buffer and canonical decode padding runs."
                ),
            },
            {
                "source": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "315-317,429-435",
                "finding": (
                    "The extended-by-one test appends one byte to a canonical signature and then "
                    "times equalized_decode_diagnostic(&extended_signature, false)."
                ),
            },
            {
                "source": "tools/parameter-search/src/production_reference.rs",
                "lines": "7304-7320",
                "finding": (
                    "decode_production_signature returns immediately when bytes.len() differs "
                    "from LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES; exact-length inputs "
                    "continue into slot, challenge, response, opening, and proof decoders."
                ),
            },
            {
                "source": "research/cryptanalysis/signature_readiness_20260516/pre_third_party_side_channel_fault_campaign_n32_q12289.md",
                "lines": "40-45, 70-72",
                "finding": (
                    "The equalized extended-by-one row measured 1922.0 ns against the equalized "
                    "valid baseline of 2562.0 ns, ratio 0.750, below the 0.80 policy floor."
                ),
            },
        ],
        "root_cause": (
            "The diagnostic is additive rather than path-equivalent. It measures the original "
            "decode result first, then performs fixed normalized-buffer scanning and canonical "
            "decode passes. For an extended-by-one signature, the first decode exits at the "
            "top-level public length check. The valid equalized baseline performs the full first "
            "decode over challenge, response, opening, and proof bytes before both rows execute "
            "the same added diagnostic padding. That fixed post-work narrows the timing gap but "
            "cannot remove work already skipped by the initial decode branch."
        ),
        "security_interpretation": {
            "secret_dependent": False,
            "attacker_controlled_signal": "serialized signature byte length",
            "impact": (
                "This leaks public malformed-length class and parser progress, not key or witness "
                "material. It still fails the configured constant-time policy for a hardened parse "
                "facade because externally observable reject classes remain distinguishable."
            ),
        },
        "concrete_remediation": [
            (
                "Derive the decode status from a fixed-width normalized buffer instead of first "
                "calling signature.decode() on attacker-shaped input. Copy/truncate/pad to the "
                "canonical signature length, accumulate an exact-length flag separately, run the "
                "full section validation over the normalized buffer, and combine flags only after "
                "fixed work completes."
            ),
            (
                "For production hardening, introduce a fixed-work decode routine that always "
                "visits slot, challenge, response, opening, and proof ranges with fixed loop "
                "bounds and accumulates public malformed/canonicality errors instead of returning "
                "at the first length failure."
            ),
            (
                "Avoid data-dependent error formatting on the measured path; return compact error "
                "codes after fixed-work validation, with rich strings constructed only outside the "
                "timing-sensitive boundary."
            ),
        ],
        "test_plan": [
            (
                "Add a CT regression row for extended_by_one_equalized on the fixed-work path and "
                "gate it with 0.80 <= ratio <= 1.25 against valid_linear_sigma_signature_equalized."
            ),
            (
                "Keep semantic unit tests asserting the 255-byte signature rejects fail-closed "
                "while the canonical 254-byte signature accepts."
            ),
            (
                "Add a structural test or instrumentation counter showing the extended-by-one "
                "path still visits challenge, response, opening, and proof validation before "
                "returning reject_fail_closed."
            ),
            (
                "Run adjacent malformed-length controls, including empty, slot-only, and "
                "truncated-by-one, so the extended-by-one fix does not leave sibling timing cliffs."
            ),
        ],
        "patch_boundary": {
            "shared_source_modified": False,
            "owned_lane": "Only files whose names contain ct_issue_extended_by_one_equalized.",
            "proposed_patch_only": (
                "This worker did not edit shared Rust source. The remediation above is the "
                "concrete patch plan for the production-reference parse boundary."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    measurement = payload["measurement"]
    policy = payload["policy"]
    security = payload["security_interpretation"]
    evidence_lines = "\n".join(
        f"- `{item['source']}:{item['lines']}` - {item['finding']}"
        for item in payload["evidence"]
    )
    remediation_lines = "\n".join(f"- {item}" for item in payload["concrete_remediation"])
    test_lines = "\n".join(f"- {item}" for item in payload["test_plan"])
    status = "passes_policy" if policy["passes"] else "fails_policy"
    return f"""# CT-008 Extended-By-One Equalized Diagnostic

## Verdict

- issue: `{payload['issue_id']}`
- group: `{payload['group']}`
- class: `{payload['class']}`
- operation: `{payload['operation']}`
- policy window: `{policy['ratio_window'][0]:.2f}..{policy['ratio_window'][1]:.2f}`
- observed ratio: `{measurement['ratio_to_equalized_valid_baseline']:.3f}`
- status: `{status}`

The equalized extended-by-one diagnostic is still materially faster than the equalized valid
baseline: `{measurement['issue_mean_avg_ns']}` ns mean versus
`{measurement['baseline_mean_avg_ns']}` ns mean, ratio
`{measurement['ratio_to_equalized_valid_baseline']:.3f}`. That is below the configured
`{policy['ratio_window'][0]:.2f}` floor by `{policy['deficit_to_min_ratio']:.3f}`.

## Why Equalization Is Insufficient

{payload['root_cause']}

This row is not evidence of secret-dependent leakage by itself. The attacker-controlled signal is
`{security['attacker_controlled_signal']}`, and `secret_dependent` is `{str(security['secret_dependent']).lower()}`.
{security['impact']}

## Evidence

{evidence_lines}

## Concrete Remediation

{remediation_lines}

## Regression Test

{test_lines}

## Patch Boundary

No shared source was edited. The proposed implementation patch belongs in the fixed-work parser or
diagnostic wrapper, but this CT-008 worker only owns files whose names contain
`ct_issue_extended_by_one_equalized`.
"""


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    OUT_MD.write_text(markdown(payload), encoding="utf-8")
    print(rel(OUT_JSON))
    print(rel(OUT_MD))


if __name__ == "__main__":
    main()
