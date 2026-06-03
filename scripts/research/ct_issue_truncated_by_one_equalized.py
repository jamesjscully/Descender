#!/usr/bin/env python3
"""Emit CT-007 truncated-by-one equalized diagnostic timing artifacts."""

from __future__ import annotations

import argparse
import json
import platform
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
CAMPAIGN_JSON = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "ct_issue_truncated_by_one_equalized_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "ct_issue_truncated_by_one_equalized_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_campaign(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def find_row(rows: list[dict[str, Any]], group: str, class_name: str) -> dict[str, Any]:
    for row in rows:
        if row.get("group") == group and row.get("class") == class_name:
            return row
    raise KeyError(f"{group}/{class_name}")


def build_payload(campaign: dict[str, Any]) -> dict[str, Any]:
    rows = campaign["summary"]["annotated_rows"]
    raw_valid = find_row(rows, "parse_malformed_input", "valid_linear_sigma_signature")
    raw_truncated = find_row(rows, "parse_malformed_input", "truncated_by_one")
    eq_valid = find_row(
        rows,
        "equalized_parse_diagnostic",
        "valid_linear_sigma_signature_equalized",
    )
    eq_truncated = find_row(
        rows,
        "equalized_parse_diagnostic",
        "truncated_by_one_equalized",
    )
    policy = campaign["summary"]["timing_ratio_policy"]
    signature_bytes = campaign["probe"]["signature_bytes"]

    return {
        "schema": "Descender.constant_time_issue.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "worker": "CT-007",
        "issue_id": "CT-007",
        "issue_name": "ct_issue_truncated_by_one_equalized",
        "scope": {
            "group": "equalized_parse_diagnostic",
            "class": "truncated_by_one_equalized",
            "operation": "equalized_decode_diagnostic",
            "owned_artifacts_glob": "*ct_issue_truncated_by_one_equalized*",
        },
        "environment": {
            "platform": platform.platform(),
            "python_version": platform.python_version(),
        },
        "inputs": {
            "campaign_json": rel(CAMPAIGN_JSON),
            "campaign_probe": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
            "decode_source": "tools/parameter-search/src/production_reference.rs",
        },
        "measurements": {
            "signature_bytes": signature_bytes,
            "truncated_bytes": signature_bytes - 1,
            "equalized_valid_baseline": eq_valid,
            "equalized_truncated_by_one": eq_truncated,
            "raw_valid_baseline": raw_valid,
            "raw_truncated_by_one": raw_truncated,
            "ratio_policy": policy,
            "policy_pass": (
                policy["min_ratio"]
                <= eq_truncated["ratio_to_group_baseline"]
                <= policy["max_ratio"]
            ),
            "raw_to_equalized_ratio_lift": (
                eq_truncated["ratio_to_group_baseline"]
                - raw_truncated["ratio_to_group_baseline"]
            ),
            "equalized_mean_gap_ns": (
                eq_valid["mean_avg_ns"] - eq_truncated["mean_avg_ns"]
            ),
        },
        "code_references": [
            {
                "path": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "178-201",
                "observation": (
                    "equalized_decode_diagnostic measures a raw signature.decode() before "
                    "normalizing into a fixed-size buffer or running canonical decode passes."
                ),
            },
            {
                "path": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "181-194",
                "observation": (
                    "The diagnostic copies min(input.len(), 254) bytes into a zero-filled "
                    "buffer and decodes that normalized buffer, but this happens after the "
                    "unequal raw direct decode."
                ),
            },
            {
                "path": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "196-199",
                "observation": (
                    "Additional canonical decode passes add common work, but they are a fixed "
                    "suffix and do not compensate for the unequal prefix."
                ),
            },
            {
                "path": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "312-314,420-426",
                "observation": (
                    "The issue input is a canonical signature with one byte popped, then timed "
                    "through equalized_decode_diagnostic."
                ),
            },
            {
                "path": "tools/parameter-search/src/production_reference.rs",
                "lines": "7304-7320",
                "observation": (
                    "decode_production_signature returns immediately on length mismatch; exact "
                    "length inputs continue into slot, challenge, response, opening, and proof decoding."
                ),
            },
        ],
        "root_cause": {
            "summary": (
                "The equalized diagnostic has an unequal prefix: it calls the ordinary decoder "
                "on the original input before doing the fixed-buffer diagnostic work. Valid input "
                "therefore pays a complete raw decode in that prefix, while the 253-byte input "
                "takes the raw decoder's immediate public length rejection."
            ),
            "why_equalization_is_insufficient": (
                "The later normalized-buffer decode and canonical decode passes are common "
                "suffix work. They lift the truncated ratio from the raw parser's 0.336 to 0.743, "
                "but the missing full raw decode in the prefix leaves a 659.4 ns mean gap against "
                "the equalized valid baseline, still below the 0.80 minimum."
            ),
            "skipped_or_unequal_work": [
                "one full original-input raw decode on the valid equalized row",
                "slot_id little-endian parse in the raw prefix",
                "challenge canonical mask and per-repetition challenge decode in the raw prefix",
                "active response and local opening decoders in the raw prefix",
                "linear sigma proof decode in the raw prefix",
            ],
            "not_caused_by": [
                "secret-dependent witness access",
                "account-state lookup",
                "callback verifier behavior",
                "hardware leakage modeling",
            ],
        },
        "secret_dependence_and_exploitability": {
            "secret_dependent": False,
            "attacker_controlled_signal": "public signature byte length",
            "leaked_information": (
                "Whether the submitted signature has exact wire length before the diagnostic "
                "normalization suffix runs."
            ),
            "secret_extraction_risk": "low",
            "policy_risk": (
                "The diagnostic row itself fails the configured malformed-input timing ratio "
                "policy, so it should not be cited as evidence that parsing is equalized."
            ),
            "exploitability": (
                "A caller who can time this diagnostic-shaped path can distinguish 253-byte "
                "signatures from exact-length signatures. The discriminant is public and "
                "attacker supplied, but it remains a parser timing cliff and an implementation "
                "fingerprint."
            ),
        },
        "remediation": {
            "proposed_patch_scope": (
                "No shared source is edited by this worker. The change below is a proposed patch "
                "for the probe or a future hardened verifier-facing parse helper."
            ),
            "recommended_diagnostic_change": (
                "Remove the unequal raw direct decode from the start of equalized_decode_diagnostic. "
                "Compute semantic acceptance from a public exact-length flag plus a fixed-work "
                "normalized decode result, or add a deliberately matched raw-decode compensation "
                "pass so valid and malformed classes execute the same decode schedule."
            ),
            "recommended_production_change": (
                "For deployment-facing constant-time parsing, introduce a fixed-work decode helper "
                "that pads/truncates into a 254-byte scratch buffer, runs all field decoders over "
                "fixed slices, accumulates length and canonicality validity bits, and returns a "
                "single rejection only after equivalent parse work has completed."
            ),
            "patch_shape": [
                "Copy min(input.len(), 254) bytes into a zero-filled scratch buffer.",
                "Record exact_length = input.len() == 254 without returning.",
                "Run slot/challenge/response/opening/proof decode work over the scratch buffer.",
                "Avoid formatting length-specific error strings on the timed hardened path.",
                "Combine exact_length and nested canonicality flags into one public accept/reject result.",
                "If keeping the diagnostic helper, ensure any raw decode used to check expected_ok occurs outside the timed closure or is matched by equivalent work for all classes.",
            ],
            "caveats": [
                "The existing equalized rows are diagnostic controls only; they are not a production hardening proof.",
                "Nested field decoders can have their own canonicality exits, so final hardening must test adjacent malformed rows too.",
            ],
        },
        "test_plan": [
            "Add a CT regression artifact/probe lane named ct_issue_truncated_by_one_equalized for equalized_decode_diagnostic or its replacement.",
            "Require valid_linear_sigma_signature_equalized and truncated_by_one_equalized ratio within the existing 0.80..1.25 policy in release builds.",
            "Assert the 253-byte input remains fail-closed after any diagnostic or production hardening change.",
            "Instrument a schedule check or counter in tests showing the original-input raw decode no longer creates a class-dependent prefix.",
            "Run empty, slot-only, extended-by-one, and invalid-challenge equalized controls so this one-byte fix does not hide adjacent timing cliffs.",
        ],
        "verdict": {
            "fail_closed": eq_truncated["observed_outcome"] == "all_batches_matched",
            "constant_time_policy": "fail",
            "deployment_blocker": True,
            "rationale": (
                "The row rejects safely but the equalized wrapper remains materially faster for "
                "the truncated class because its first decode is still the ordinary early-return parser."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    measurements = payload["measurements"]
    eq_valid = measurements["equalized_valid_baseline"]
    eq_truncated = measurements["equalized_truncated_by_one"]
    raw_truncated = measurements["raw_truncated_by_one"]
    policy = measurements["ratio_policy"]
    refs = payload["code_references"]

    return "\n".join(
        [
            "# CT-007: truncated_by_one_equalized",
            "",
            "## Scope",
            "",
            "- group: `equalized_parse_diagnostic`",
            "- class: `truncated_by_one_equalized`",
            "- operation: `equalized_decode_diagnostic`",
            "- owned lane: files containing `ct_issue_truncated_by_one_equalized`",
            "",
            "## Measurement",
            "",
            f"- equalized valid baseline mean: `{eq_valid['mean_avg_ns']:.1f} ns`",
            f"- equalized truncated-by-one mean: `{eq_truncated['mean_avg_ns']:.1f} ns`",
            f"- ratio to equalized valid baseline: `{eq_truncated['ratio_to_group_baseline']:.3f}`",
            f"- policy window: `{policy['min_ratio']:.2f}..{policy['max_ratio']:.2f}`",
            f"- fail-closed outcome: `{eq_truncated['observed_outcome']}`",
            f"- raw truncated-by-one ratio for comparison: `{raw_truncated['ratio_to_group_baseline']:.3f}`",
            f"- mean gap after diagnostic equalization: `{measurements['equalized_mean_gap_ns']:.1f} ns`",
            "",
            "## Root Cause",
            "",
            payload["root_cause"]["summary"],
            "",
            payload["root_cause"]["why_equalization_is_insufficient"],
            "",
            "Unequal work still present:",
            "",
            *[f"- {item}" for item in payload["root_cause"]["skipped_or_unequal_work"]],
            "",
            "Relevant source anchors:",
            "",
            *[
                f"- `{ref['path']}:{ref['lines']}` - {ref['observation']}"
                for ref in refs
            ],
            "",
            "## Secret Dependence And Exploitability",
            "",
            f"- secret-dependent: `{payload['secret_dependence_and_exploitability']['secret_dependent']}`",
            f"- attacker-controlled signal: `{payload['secret_dependence_and_exploitability']['attacker_controlled_signal']}`",
            f"- leaked information: {payload['secret_dependence_and_exploitability']['leaked_information']}",
            f"- secret extraction risk: `{payload['secret_dependence_and_exploitability']['secret_extraction_risk']}`",
            "",
            payload["secret_dependence_and_exploitability"]["exploitability"],
            "",
            "## Remediation And Tests",
            "",
            payload["remediation"]["recommended_diagnostic_change"],
            "",
            payload["remediation"]["recommended_production_change"],
            "",
            "Concrete patch shape:",
            "",
            *[f"- {item}" for item in payload["remediation"]["patch_shape"]],
            "",
            "Regression test plan:",
            "",
            *[f"- {item}" for item in payload["test_plan"]],
            "",
            payload["remediation"]["proposed_patch_scope"],
            "",
            "## Verdict",
            "",
            f"- fail closed: `{payload['verdict']['fail_closed']}`",
            f"- constant-time policy: `{payload['verdict']['constant_time_policy']}`",
            f"- deployment blocker: `{payload['verdict']['deployment_blocker']}`",
            f"- rationale: {payload['verdict']['rationale']}",
            "",
        ]
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-json", type=Path, default=CAMPAIGN_JSON)
    parser.add_argument("--output-json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--output-md", type=Path, default=DEFAULT_MD)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    campaign = load_campaign(args.input_json)
    payload = build_payload(campaign)
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    args.output_md.write_text(markdown(payload), encoding="utf-8")
    print(rel(args.output_json))
    print(rel(args.output_md))


if __name__ == "__main__":
    main()
