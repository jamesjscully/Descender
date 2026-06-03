#!/usr/bin/env python3
"""Generate CT-004 extended-by-one decode timing artifacts."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TIMING_REPORT = ROOT / "research/cryptanalysis/signature_readiness_20260516/pre_third_party_side_channel_fault_campaign_n32_q12289.json"
JSON_OUT = ROOT / "research/cryptanalysis/signature_readiness_20260516/ct_issue_extended_by_one.json"
MD_OUT = ROOT / "research/cryptanalysis/signature_readiness_20260516/ct_issue_extended_by_one.md"


def load_report() -> dict:
    with TIMING_REPORT.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def find_row(rows: list[dict], group: str, klass: str, operation: str) -> dict:
    for row in rows:
        if row["group"] == group and row["class"] == klass and row["operation"] == operation:
            return row
    raise KeyError(f"missing timing row {group}/{klass}/{operation}")


def artifact() -> dict:
    report = load_report()
    annotated = report["summary"]["annotated_rows"]
    valid = find_row(
        annotated,
        "parse_malformed_input",
        "valid_linear_sigma_signature",
        "ProductionSignature::decode",
    )
    issue = find_row(
        annotated,
        "parse_malformed_input",
        "extended_by_one",
        "ProductionSignature::decode",
    )
    equalized_valid = find_row(
        annotated,
        "equalized_parse_diagnostic",
        "valid_linear_sigma_signature_equalized",
        "equalized_decode_diagnostic",
    )
    equalized_issue = find_row(
        annotated,
        "equalized_parse_diagnostic",
        "extended_by_one_equalized",
        "equalized_decode_diagnostic",
    )

    return {
        "schema": "Descender.ct_issue_extended_by_one.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "issue": {
            "id": "CT-004",
            "group": "parse_malformed_input",
            "class": "extended_by_one",
            "operation": "ProductionSignature::decode",
            "reported_ratio_to_valid_baseline": issue["ratio_to_group_baseline"],
            "status": "confirmed_public_length_timing_cliff",
        },
        "evidence": {
            "timing_report": str(TIMING_REPORT.relative_to(ROOT)),
            "valid_baseline": valid,
            "extended_by_one": issue,
            "equalized_valid_baseline": equalized_valid,
            "equalized_extended_by_one": equalized_issue,
            "ratio_delta_after_equalization": (
                equalized_issue["ratio_to_group_baseline"] - issue["ratio_to_group_baseline"]
            ),
            "probe_source": {
                "path": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "extended_input_lines": [315, 317],
                "extended_probe_lines": [375, 381],
                "equalized_diagnostic_lines": [178, 199],
            },
            "decode_source": {
                "path": "tools/parameter-search/src/production_reference.rs",
                "signature_length_constant_lines": [69, 70],
                "decode_length_check_lines": [7304, 7310],
                "full_decode_work_lines": [7311, 7320],
            },
        },
        "root_cause": {
            "summary": (
                "decode_production_signature performs an exact public byte-length check "
                "before any slot parsing, challenge decoding, response/opening scans, or proof copy. "
                "The extended-by-one input is a canonical signature with one extra byte, so it "
                "rejects immediately on length and skips the full valid decode path."
            ),
            "skipped_work": [
                "slot_id little-endian parse",
                "challenge canonical mask and per-repetition block/sign decode",
                "52 active response byte canonicality checks",
                "26 local opening pair decodes and q-canonicality checks",
                "linear sigma proof commitment/response copy",
            ],
            "why_equalized_probe_matters": (
                "The equalized diagnostic normalizes/copies a public fixed-size buffer and adds "
                "canonical decode passes; the extended-by-one ratio rises from 0.3253 to 0.7502, "
                "supporting that the cliff is skipped parser work at a public malformed-input branch."
            ),
        },
        "secret_dependence": {
            "classification": "public-input timing difference, not observed secret-dependent leakage",
            "reasoning": [
                "The branch condition is bytes.len(), controlled by the serialized signature length.",
                "The malformed input rejects before account state, secret key material, callback verifier semantics, or hidden-source proof state are consulted.",
                "The observed ratio leaks malformed length class and parser progress, not a witness, key, slot secret, or signature-derived secret.",
            ],
            "exploitability": (
                "Remote callers can distinguish malformed length classes from valid-length parser work "
                "if timing is observable. This may be useful for endpoint fingerprinting or DoS shaping, "
                "but by itself does not expose cryptographic secrets because the discriminant is public "
                "attacker-supplied length."
            ),
        },
        "remediation": {
            "recommended_code_change": (
                "For a hardened production verifier, replace immediate malformed-length returns in "
                "ProductionSignature::decode with a fixed-work decode wrapper: copy/truncate/pad into "
                "a 254-byte scratch buffer, run the challenge/response/opening/proof decode work, "
                "accumulate a public validity flag that includes exact length, and return the stored "
                "error only after the fixed work completes."
            ),
            "compatibility_note": (
                "The semantic result should remain fail-closed for all non-254-byte inputs; only "
                "the amount of parser work before returning changes."
            ),
            "test_plan": [
                "Add a microbenchmark/policy row for ProductionSignature::decode_fixed_work or the final verifier parse boundary.",
                "Assert valid, truncated_by_one, extended_by_one, empty, and invalid_challenge_mask remain fail-closed/accepted as before.",
                "Run the timing policy with ratio bounds 0.8..1.25 against the final production verifier path, not only this diagnostic reference binary.",
                "Add regression coverage that extended_by_one does not return before the fixed-work challenge/response/opening/proof passes execute.",
            ],
            "proposed_patch_scope": (
                "No shared source was edited in this worker lane. The above remediation is a proposed "
                "patch for the production-reference decode boundary."
            ),
        },
    }


def write_markdown(data: dict) -> None:
    issue = data["issue"]
    evidence = data["evidence"]
    root = data["root_cause"]
    secret = data["secret_dependence"]
    remediation = data["remediation"]
    valid = evidence["valid_baseline"]
    row = evidence["extended_by_one"]
    eq_row = evidence["equalized_extended_by_one"]
    eq_valid = evidence["equalized_valid_baseline"]

    lines = [
        "# CT-004: extended_by_one ProductionSignature::decode",
        "",
        "## Finding",
        "",
        (
            f"`{issue['group']}/{issue['class']}` is confirmed as a public malformed-length "
            f"timing cliff in `{issue['operation']}`. The row rejects fail-closed, but averages "
            f"{row['mean_avg_ns']} ns versus {valid['mean_avg_ns']} ns for the valid baseline "
            f"(ratio {row['ratio_to_group_baseline']:.4f})."
        ),
        "",
        "## Root Cause",
        "",
        root["summary"],
        "",
        "Relevant source anchors:",
        "",
        "- `tools/parameter-search/src/production_reference.rs:7304` starts `decode_production_signature`.",
        "- `tools/parameter-search/src/production_reference.rs:7305` returns immediately when `bytes.len() != LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES`.",
        "- `tools/parameter-search/src/production_reference.rs:7311` through `:7320` is the valid-path work skipped by the extended-by-one input.",
        "- `tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs:315` through `:317` constructs the extra-byte input.",
        "- `tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs:375` through `:381` times `extended_signature.decode().is_err()`.",
        "",
        "Skipped valid-path work:",
        "",
    ]
    lines.extend(f"- {item}" for item in root["skipped_work"])
    lines.extend(
        [
            "",
            "## Probe Interpretation",
            "",
            (
                f"The equalized diagnostic valid row averages {eq_valid['mean_avg_ns']} ns; "
                f"the equalized extended-by-one row averages {eq_row['mean_avg_ns']} ns "
                f"(ratio {eq_row['ratio_to_group_baseline']:.4f}). "
                f"{root['why_equalized_probe_matters']}"
            ),
            "",
            "## Secret Dependence And Exploitability",
            "",
            f"Classification: {secret['classification']}.",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in secret["reasoning"])
    lines.extend(
        [
            "",
            secret["exploitability"],
            "",
            "## Remediation And Tests",
            "",
            remediation["recommended_code_change"],
            "",
            remediation["compatibility_note"],
            "",
        ]
    )
    lines.extend(f"- {item}" for item in remediation["test_plan"])
    lines.extend(["", remediation["proposed_patch_scope"], ""])
    MD_OUT.write_text("\n".join(lines), encoding="utf-8")


def main() -> None:
    data = artifact()
    JSON_OUT.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    write_markdown(data)
    print(JSON_OUT.relative_to(ROOT))
    print(MD_OUT.relative_to(ROOT))


if __name__ == "__main__":
    main()
