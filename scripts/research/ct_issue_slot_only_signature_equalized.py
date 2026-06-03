#!/usr/bin/env python3
"""Emit CT-006 slot-only equalized diagnostic investigation artifacts."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
CAMPAIGN_JSON = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
OUT_JSON = READINESS_DIR / "ct_issue_slot_only_signature_equalized_n32_q12289.json"
OUT_MD = READINESS_DIR / "ct_issue_slot_only_signature_equalized_n32_q12289.md"

ISSUE_CLASS = "slot_only_signature_equalized"
BASELINE_CLASS = "valid_linear_sigma_signature_equalized"
POLICY_MIN = 0.80
POLICY_MAX = 1.25


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def row_by_class(rows: list[dict[str, Any]], class_name: str) -> dict[str, Any]:
    for row in rows:
        if row.get("class") == class_name:
            return row
    raise KeyError(class_name)


def ratio(row: dict[str, Any], baseline: dict[str, Any]) -> float:
    return float(row["mean_avg_ns"]) / float(baseline["mean_avg_ns"])


def build_payload() -> dict[str, Any]:
    campaign = json.loads(CAMPAIGN_JSON.read_text(encoding="utf-8"))
    rows = campaign["probe"]["rows"]
    issue_row = row_by_class(rows, ISSUE_CLASS)
    baseline_row = row_by_class(rows, BASELINE_CLASS)
    observed_ratio = ratio(issue_row, baseline_row)
    deficit_to_min = POLICY_MIN - observed_ratio
    return {
        "schema": "Descender.ct_issue_slot_only_signature_equalized.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "issue_id": "CT-006",
        "group": "equalized_parse_diagnostic",
        "class": ISSUE_CLASS,
        "operation": "equalized_decode_diagnostic",
        "policy": {
            "ratio_window": [POLICY_MIN, POLICY_MAX],
            "passes": POLICY_MIN <= observed_ratio <= POLICY_MAX,
            "deficit_to_min_ratio": deficit_to_min,
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
                    "equalized_decode_diagnostic first calls signature.decode() on the original "
                    "input before doing fixed normalized-buffer and canonical decode work."
                ),
            },
            {
                "source": "tools/parameter-search/src/production_reference.rs",
                "lines": "7304-7320",
                "finding": (
                    "decode_production_signature returns immediately when the byte length is not "
                    "LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES; the slot-only input is only "
                    "SLOT_ID_BYTES long, so it skips slot/challenge/response/opening/proof parsing."
                ),
            },
            {
                "source": "research/cryptanalysis/signature_readiness_20260516/pre_third_party_side_channel_fault_campaign_n32_q12289.md",
                "lines": "40-45, 63-70",
                "finding": (
                    "The equalized slot-only row measured 1910.8 ns against the equalized valid "
                    "baseline of 2562.0 ns, ratio 0.746, below the 0.80 policy floor."
                ),
            },
        ],
        "root_cause": (
            "The diagnostic equalization is additive rather than path-equivalent. It adds a "
            "normalized decode and eight canonical decodes after the original decode, but the "
            "first original decode still has input-shape-dependent work. For a slot-only "
            "signature, that first decode exits at the top-level length check, while the valid "
            "baseline traverses challenge, response, opening, and proof decoders before the same "
            "post-padding work runs. The fixed post-work narrows the cliff but does not erase it."
        ),
        "security_interpretation": (
            "This is a diagnostic reference timing cliff, not a proven secret leak by itself: "
            "signature length is public malformed-input state. It still blocks any hardened "
            "constant-time claim for this parser facade because externally distinguishable reject "
            "classes remain outside the configured timing-ratio policy."
        ),
        "proposed_remediation": [
            (
                "Move equalized diagnostic status derivation onto a fixed-width normalized buffer: "
                "copy/truncate the input to the canonical signature length, compute malformed flags "
                "separately with branchless public accumulators, run the full section decoders or "
                "constant-work validation over the normalized buffer, then combine the accumulated "
                "malformed flag with canonicality flags at the end."
            ),
            (
                "For a production parser, prefer a single fixed-work decode routine that always "
                "visits slot, challenge, response, opening, and proof ranges and accumulates errors "
                "instead of returning from the first length/canonicality failure."
            ),
            (
                "If the current function remains only a benchmark diagnostic, rename/comment it as "
                "non-hardened and keep it out of any deployment facade so it cannot be mistaken for "
                "a constant-time wrapper."
            ),
        ],
        "test_plan": [
            (
                "Add an executable timing closure row for slot_only_signature_equalized after the "
                "fixed-work decoder change, with the policy gate requiring 0.80 <= ratio <= 1.25 "
                "relative to valid_linear_sigma_signature_equalized."
            ),
            (
                "Add malformed-length regression cases for empty, slot-only, truncated-by-one, and "
                "extended-by-one inputs, asserting identical fail-closed outcomes and collecting "
                "per-row ratios in the side-channel campaign artifact."
            ),
            (
                "Add a structural unit test or instrumentation counter proving the equalized path "
                "visits challenge, response, opening, and proof validation for slot-only input even "
                "when the final result is reject_fail_closed."
            ),
        ],
        "proposed_patch_scope": {
            "shared_source_not_modified_by_this_worker": True,
            "candidate_files": [
                "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "tools/parameter-search/src/production_reference.rs",
                "scripts/research/pre_third_party_side_channel_fault_campaign.py",
            ],
            "note": (
                "This worker only reports the patch plan because CT-006 ownership is limited to "
                "ct_issue_slot_only_signature_equalized artifacts/scripts."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    m = payload["measurement"]
    p = payload["policy"]
    evidence_lines = "\n".join(
        f"- `{item['source']}:{item['lines']}` - {item['finding']}"
        for item in payload["evidence"]
    )
    remediation_lines = "\n".join(f"- {item}" for item in payload["proposed_remediation"])
    test_lines = "\n".join(f"- {item}" for item in payload["test_plan"])
    return f"""# CT-006 Slot-Only Signature Equalized Diagnostic

## Verdict

- issue: `{payload['issue_id']}`
- group: `{payload['group']}`
- class: `{payload['class']}`
- operation: `{payload['operation']}`
- policy window: `{p['ratio_window'][0]:.2f}..{p['ratio_window'][1]:.2f}`
- observed ratio: `{m['ratio_to_equalized_valid_baseline']:.3f}`
- status: `fails_policy`

The equalized slot-only diagnostic is still measurably faster than the equalized valid baseline:
`{m['issue_mean_avg_ns']}` ns mean versus `{m['baseline_mean_avg_ns']}` ns mean, ratio
`{m['ratio_to_equalized_valid_baseline']:.3f}`. That is below the configured `{p['ratio_window'][0]:.2f}`
floor by `{p['deficit_to_min_ratio']:.3f}`.

## Why Equalization Is Insufficient

{payload['root_cause']}

{payload['security_interpretation']}

## Evidence

{evidence_lines}

## Concrete Remediation

{remediation_lines}

## Regression Test

{test_lines}

## Patch Boundary

No shared source was edited. The proposed implementation patch belongs in the fixed-width parser or
diagnostic wrapper, but this CT-006 worker only owns files whose names contain
`ct_issue_slot_only_signature_equalized`.
"""


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    OUT_MD.write_text(markdown(payload), encoding="utf-8")
    print(rel(OUT_JSON))
    print(rel(OUT_MD))


if __name__ == "__main__":
    main()
