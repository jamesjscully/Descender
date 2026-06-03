#!/usr/bin/env python3
"""Emit CT-003 truncated-by-one decode timing issue artifacts."""

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
DEFAULT_JSON = READINESS_DIR / "ct_issue_truncated_by_one_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "ct_issue_truncated_by_one_n32_q12289.md"


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
    valid = find_row(rows, "parse_malformed_input", "valid_linear_sigma_signature")
    issue = find_row(rows, "parse_malformed_input", "truncated_by_one")
    equalized = find_row(rows, "equalized_parse_diagnostic", "truncated_by_one_equalized")
    policy = campaign["summary"]["timing_ratio_policy"]
    signature_bytes = campaign["probe"]["signature_bytes"]

    return {
        "schema": "Descender.constant_time_issue.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "worker": "CT-003",
        "issue_id": "CT-003",
        "issue_name": "ct_issue_truncated_by_one",
        "scope": {
            "group": "parse_malformed_input",
            "class": "truncated_by_one",
            "operation": "ProductionSignature::decode",
            "owned_artifacts_glob": "*ct_issue_truncated_by_one*",
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
            "valid_baseline": valid,
            "truncated_by_one": issue,
            "truncated_by_one_equalized_control": equalized,
            "ratio_policy": policy,
            "policy_pass": (
                policy["min_ratio"]
                <= issue["ratio_to_group_baseline"]
                <= policy["max_ratio"]
            ),
        },
        "code_references": [
            {
                "path": "tools/parameter-search/src/production_reference.rs",
                "lines": "7304-7310",
                "observation": (
                    "decode_production_signature returns immediately when input length is not "
                    "LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES."
                ),
            },
            {
                "path": "tools/parameter-search/src/production_reference.rs",
                "lines": "7311-7320",
                "observation": (
                    "Only exact-length inputs continue into fixed field slicing and nested "
                    "challenge/response/opening/proof decoders."
                ),
            },
            {
                "path": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "312-314,365-372",
                "observation": (
                    "The probe constructs the issue input by popping one byte from a canonical "
                    "signature and timing ProductionSignature::decode rejection."
                ),
            },
        ],
        "root_cause": {
            "summary": (
                "The 253-byte truncated signature hits the top-level public length check and "
                "returns Err before the valid path decodes challenge bits, response bytes, "
                "opening coordinates, and proof bytes."
            ),
            "timing_mechanism": (
                "Valid canonical input performs all field decoders; truncated-by-one input does "
                "one length comparison plus error formatting. That explains the observed mean "
                "ratio of about 0.336 against the valid parse baseline."
            ),
            "not_caused_by": [
                "secret-dependent witness access",
                "secret-dependent table lookup",
                "slot-account state lookup",
                "cryptographic verification",
            ],
        },
        "secret_dependence_and_exploitability": {
            "secret_dependent": False,
            "attacker_controlled_signal": "input byte length",
            "leaked_information": (
                "Whether the submitted signature length equals the public 254-byte linear-sigma "
                "wire format length."
            ),
            "secret_extraction_risk": "low",
            "policy_risk": "non-compliant with the current malformed-input timing ratio policy",
            "exploitability": (
                "A remote caller who can time decode can distinguish truncated/extended public "
                "length classes from exact-length parse work. Because the caller chooses the "
                "bytes and length, this is not a practical key or witness oracle by itself, but "
                "it is a parser timing cliff and can fingerprint implementation behavior."
            ),
        },
        "remediation": {
            "recommended_change": (
                "For any hardened verifier-facing parse path, normalize the input into a fixed "
                "254-byte buffer, decode all fields from that buffer with fixed loop bounds, "
                "accumulate public validity errors, and defer returning length/canonicality "
                "failure until after equivalent parse work has completed."
            ),
            "patch_shape": [
                "Introduce a hardened decode helper rather than silently changing diagnostic APIs.",
                "Copy min(input.len(), 254) bytes into a zero-filled fixed buffer.",
                "Record a length_mismatch flag without returning.",
                "Run challenge, response, opening, and proof decode work over the fixed buffer.",
                "Combine length_mismatch and nested canonicality failures into a single rejection.",
                "Use the hardened helper from deployment verifier entrypoints that claim constant-time parsing.",
            ],
            "caveats": [
                "Rust Result and String construction are not constant-time primitives; hardened code should avoid data-dependent error formatting on the measured path.",
                "Nested decoders currently have their own early canonicality exits, so full parse hardening must address those rows too, not only truncated_by_one.",
            ],
        },
        "test_plan": [
            "Add a CT regression probe named ct_issue_truncated_by_one that compares valid exact-length decode and 253-byte decode on the hardened path.",
            "Require truncated_by_one/valid ratio within the existing 0.80..1.25 timing policy over release builds.",
            "Keep a fail-closed unit test asserting the 253-byte signature is rejected.",
            "Include empty, slot-only, extended-by-one, and invalid-challenge controls so the one-byte fix does not leave adjacent malformed parser cliffs.",
        ],
        "verdict": {
            "fail_closed": issue["observed_outcome"] == "all_batches_matched",
            "constant_time_policy": "fail",
            "deployment_blocker": True,
            "rationale": (
                "The row rejects safely but is materially faster than valid parsing and below "
                "the configured timing-ratio minimum."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    measurements = payload["measurements"]
    issue = measurements["truncated_by_one"]
    valid = measurements["valid_baseline"]
    equalized = measurements["truncated_by_one_equalized_control"]
    policy = measurements["ratio_policy"]
    refs = payload["code_references"]

    return "\n".join(
        [
            "# CT-003: truncated_by_one",
            "",
            "## Scope",
            "",
            "- group: `parse_malformed_input`",
            "- class: `truncated_by_one`",
            "- operation: `ProductionSignature::decode`",
            "- owned lane: files containing `ct_issue_truncated_by_one`",
            "",
            "## Measurement",
            "",
            f"- valid baseline mean: `{valid['mean_avg_ns']:.1f} ns`",
            f"- truncated-by-one mean: `{issue['mean_avg_ns']:.1f} ns`",
            f"- ratio to valid baseline: `{issue['ratio_to_group_baseline']:.3f}`",
            f"- policy window: `{policy['min_ratio']:.2f}..{policy['max_ratio']:.2f}`",
            f"- fail-closed outcome: `{issue['observed_outcome']}`",
            f"- equalized diagnostic control ratio: `{equalized['ratio_to_group_baseline']:.3f}`",
            "",
            "## Root Cause",
            "",
            payload["root_cause"]["summary"],
            "",
            payload["root_cause"]["timing_mechanism"],
            "",
            "Relevant source references:",
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
            "## Remediation",
            "",
            payload["remediation"]["recommended_change"],
            "",
            "Concrete patch shape:",
            "",
            *[f"- {item}" for item in payload["remediation"]["patch_shape"]],
            "",
            "Caveats:",
            "",
            *[f"- {item}" for item in payload["remediation"]["caveats"]],
            "",
            "## Regression Test",
            "",
            *[f"- {item}" for item in payload["test_plan"]],
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
    args.output_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    args.output_md.write_text(markdown(payload), encoding="utf-8")
    print(rel(args.output_json))
    print(rel(args.output_md))


if __name__ == "__main__":
    main()
