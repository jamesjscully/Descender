#!/usr/bin/env python3
"""Emit CT-009 invalid challenge-mask equalized diagnostic artifacts."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
CAMPAIGN_JSON = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"
PROBE = ROOT / "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs"
OUT_JSON = READINESS_DIR / "ct_issue_invalid_challenge_mask_equalized_n32_q12289.json"
OUT_MD = READINESS_DIR / "ct_issue_invalid_challenge_mask_equalized_n32_q12289.md"

ISSUE_ID = "CT-009"
GROUP = "equalized_parse_diagnostic"
ISSUE_CLASS = "invalid_challenge_mask_equalized"
BASELINE_CLASS = "valid_linear_sigma_signature_equalized"
OPERATION = "equalized_decode_diagnostic"
POLICY_MIN = 0.80
POLICY_MAX = 1.25


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def row_by_class(rows: list[dict[str, Any]], class_name: str) -> dict[str, Any]:
    for row in rows:
        if row.get("group") == GROUP and row.get("class") == class_name:
            return row
    raise KeyError(class_name)


def source_checks() -> dict[str, bool]:
    probe = PROBE.read_text(encoding="utf-8")
    production_reference = PRODUCTION_REFERENCE.read_text(encoding="utf-8")
    return {
        "equalized_original_decode_runs_before_padding": (
            "let direct_ok = signature.decode().is_ok();" in probe
            and "let bytes = signature.as_bytes();" in probe
        ),
        "equalized_normalized_decode_is_after_original_decode": (
            "ProductionSignature::new(normalized.to_vec())" in probe
            and ".decode()" in probe
            and "direct_ok == expected_ok" in probe
        ),
        "equalized_canonical_decode_passes_are_fixed_post_work": (
            "EQUALIZED_DIAGNOSTIC_CANONICAL_DECODE_PASSES" in probe
            and "black_box(canonical.decode().is_ok());" in probe
        ),
        "invalid_challenge_mask_fixture_sets_last_algebraic_byte": (
            "invalid_challenge_bytes[SLOT_ID_BYTES + ALGEBRAIC_CHALLENGE_BYTES - 1] = 0xff;"
            in probe
        ),
        "production_decode_orders_challenge_before_responses_openings_proof": (
            "challenge: decode_challenge(&bytes[SLOT_ID_BYTES..response_start])?" in production_reference
            and "responses: decode_responses(&bytes[response_start..opening_start])?"
            in production_reference
            and "openings: decode_openings(&bytes[opening_start..proof_start])?"
            in production_reference
            and "proof: decode_linear_sigma_target_proof(&bytes[proof_start..])?"
            in production_reference
        ),
        "challenge_high_bits_early_return": (
            "ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK" in production_reference
            and 'return Err("challenge is not canonical: high bits are set".to_string());'
            in production_reference
        ),
    }


def build_payload() -> dict[str, Any]:
    campaign = json.loads(CAMPAIGN_JSON.read_text(encoding="utf-8"))
    rows = campaign["probe"]["rows"]
    issue_row = row_by_class(rows, ISSUE_CLASS)
    baseline_row = row_by_class(rows, BASELINE_CLASS)
    observed_ratio = float(issue_row["mean_avg_ns"]) / float(baseline_row["mean_avg_ns"])
    return {
        "schema": "Descender.ct_issue_invalid_challenge_mask_equalized.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "issue_id": ISSUE_ID,
        "scope": {
            "group": GROUP,
            "class": ISSUE_CLASS,
            "operation": OPERATION,
            "owned_artifact_prefix": "ct_issue_invalid_challenge_mask_equalized",
        },
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
        "source_evidence": {
            "probe": rel(PROBE),
            "equalized_decode_diagnostic_lines": "178-201",
            "probe_fixture_and_rows_lines": "323-431",
            "production_reference": rel(PRODUCTION_REFERENCE),
            "decode_order_lines": "7304-7321",
            "challenge_decode_lines": "7414-7441",
            "source_checks": source_checks(),
        },
        "why_equalization_is_insufficient": [
            (
                "equalized_decode_diagnostic is not a replacement decoder. It first calls "
                "signature.decode() on the attacker-provided bytes and only then performs "
                "normalized-buffer scanning, a normalized decode, and eight canonical decodes."
            ),
            (
                "The invalid challenge-mask fixture is full length, so the first decode enters "
                "decode_production_signature. That decoder validates challenge before responses, "
                "openings, and proof."
            ),
            (
                "decode_challenge returns immediately when the final algebraic challenge byte has "
                "bits outside ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK. The fixture sets that byte to "
                "0xff, so the original decode skips response, opening, and proof validation."
            ),
            (
                "The later fixed post-work is shared by the valid baseline and the malformed row. "
                "It reduces the relative timing gap, but it does not erase the missing original "
                "decode work. The measured ratio remains 0.660, below the 0.80 policy floor."
            ),
        ],
        "security_interpretation": {
            "secret_dependent": False,
            "public_input_channel": True,
            "assessment": (
                "This row is a public malformed-input parser oracle, not evidence of hidden "
                "witness or secret-key leakage by itself. The branch condition is controlled by "
                "signature bytes."
            ),
            "deployment_risk": (
                "It still blocks a constant-time parser-facade claim because reject classes are "
                "externally distinguishable after the equalized diagnostic wrapper."
            ),
        },
        "concrete_remediation": [
            (
                "Replace additive post-padding with a fixed-work decode path for the diagnostic "
                "and production parser facade: normalize/copy to canonical length once, visit "
                "slot, challenge, responses, openings, and proof ranges for every length-correct "
                "candidate, and accumulate public malformed flags before returning."
            ),
            (
                "Refactor decode_challenge into a no-early-return validation helper for hardened "
                "paths. It should scan all REPS, accumulate high-bit and active-block range "
                "failures, derive placeholder active_block/sign values under masks where needed, "
                "and report one malformed challenge flag at the end."
            ),
            (
                "Extend the same accumulated-error pattern to response, opening, and proof "
                "canonicality checks so an invalid challenge cannot skip the rest of the fixed "
                "parse surface."
            ),
            (
                "If deployment policy allows variable-time public rejects, explicitly document "
                "equalized_decode_diagnostic as a non-hardened benchmark control and exclude it "
                "from constant-time closure gates. Otherwise gate this row until it falls within "
                "0.80..1.25."
            ),
        ],
        "test_plan": [
            (
                "Add a structural regression test or instrumentation counter proving the "
                "hardened equalized path visits challenge, response, opening, and proof "
                "validation for invalid_challenge_mask_equalized."
            ),
            (
                "Add challenge-specific negative cases for high algebraic challenge bits and "
                "out-of-range active blocks; assert fail-closed generic rejection after the full "
                "scan, not first-error strings from early returns."
            ),
            (
                "Re-run the side-channel campaign and require invalid_challenge_mask_equalized / "
                "valid_linear_sigma_signature_equalized mean ratio in 0.80..1.25, with samples "
                "and batch_iterations recorded in the artifact."
            ),
        ],
        "patch_boundary": {
            "shared_source_modified": False,
            "reason": (
                "This worker owns only files whose names contain "
                "ct_issue_invalid_challenge_mask_equalized, so shared-source remediation is "
                "reported as a proposed patch plan rather than applied here."
            ),
            "candidate_shared_files": [
                rel(PROBE),
                rel(PRODUCTION_REFERENCE),
                "scripts/research/pre_third_party_side_channel_fault_campaign.py",
            ],
        },
        "status": "fails_policy_root_cause_identified_patch_proposed_not_applied",
    }


def render_md(payload: dict[str, Any]) -> str:
    measurement = payload["measurement"]
    policy = payload["policy"]
    checks = "\n".join(
        f"- `{name}`: `{value}`"
        for name, value in sorted(payload["source_evidence"]["source_checks"].items())
    )
    causes = "\n".join(f"- {item}" for item in payload["why_equalization_is_insufficient"])
    remediation = "\n".join(f"- {item}" for item in payload["concrete_remediation"])
    tests = "\n".join(f"- {item}" for item in payload["test_plan"])
    return f"""# CT-009 Invalid Challenge Mask Equalized Diagnostic

## Verdict

- issue: `{payload['issue_id']}`
- group: `{payload['scope']['group']}`
- class: `{payload['scope']['class']}`
- operation: `{payload['scope']['operation']}`
- status: `{payload['status']}`
- policy window: `{policy['ratio_window'][0]:.2f}..{policy['ratio_window'][1]:.2f}`
- observed ratio: `{measurement['ratio_to_equalized_valid_baseline']:.3f}`

The equalized invalid challenge-mask diagnostic still fails policy. It measured
`{measurement['issue_mean_avg_ns']}` ns mean versus `{measurement['baseline_mean_avg_ns']}` ns for
`{measurement['baseline_class']}`, ratio `{measurement['ratio_to_equalized_valid_baseline']:.3f}`.
That is `{policy['deficit_to_min_ratio']:.3f}` below the `{policy['ratio_window'][0]:.2f}` floor.

## Why Equalization Is Insufficient

{causes}

## Source Evidence

- `{payload['source_evidence']['probe']}` lines `{payload['source_evidence']['equalized_decode_diagnostic_lines']}`:
  `equalized_decode_diagnostic` runs the original `signature.decode()` before the normalized
  buffer decode and canonical decode passes.
- `{payload['source_evidence']['probe']}` lines `{payload['source_evidence']['probe_fixture_and_rows_lines']}`:
  the fixture sets the last algebraic challenge byte to `0xff` and records
  `invalid_challenge_mask_equalized`.
- `{payload['source_evidence']['production_reference']}` lines `{payload['source_evidence']['decode_order_lines']}`:
  production decoding validates challenge before responses, openings, and proof.
- `{payload['source_evidence']['production_reference']}` lines `{payload['source_evidence']['challenge_decode_lines']}`:
  `decode_challenge` returns on high challenge bits before scanning per-round challenge fields.

Source checks:
{checks}

## Security Interpretation

This row is not secret-dependent by itself. The branch condition is controlled by public signature
bytes, so the immediate leakage is a malformed-input parser oracle rather than hidden witness or
secret-key disclosure. It still blocks a hardened constant-time parser-facade claim because the
equalized wrapper leaves externally distinguishable reject classes.

## Concrete Remediation

{remediation}

## Regression Test

{tests}

## Patch Boundary

No shared source was edited. This worker owns only files whose names contain
`ct_issue_invalid_challenge_mask_equalized`, so the production/probe changes above are proposed
for the shared-source owner rather than applied here.
"""


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    OUT_MD.write_text(render_md(payload), encoding="utf-8")
    print(rel(OUT_JSON))
    print(rel(OUT_MD))


if __name__ == "__main__":
    main()
