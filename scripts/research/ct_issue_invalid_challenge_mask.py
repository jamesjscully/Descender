#!/usr/bin/env python3
"""Generate CT-005 invalid challenge-mask timing issue artifacts."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
CAMPAIGN_JSON = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
SOURCE = ROOT / "tools/parameter-search/src/production_reference.rs"
PROBE = ROOT / "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs"
OUT_JSON = READINESS_DIR / "ct_issue_invalid_challenge_mask_n32_q12289.json"
OUT_MD = READINESS_DIR / "ct_issue_invalid_challenge_mask_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_campaign_row() -> dict[str, Any]:
    payload = json.loads(CAMPAIGN_JSON.read_text())
    for row in payload["summary"]["annotated_rows"]:
        if (
            row["group"] == "parse_malformed_input"
            and row["class"] == "invalid_challenge_mask"
            and row["operation"] == "ProductionSignature::decode"
        ):
            return row
    raise SystemExit("invalid_challenge_mask row not found")


def source_contains_all() -> dict[str, bool]:
    source = SOURCE.read_text()
    probe = PROBE.read_text()
    checks = {
        "decode_order_challenge_before_responses": (
            "challenge: decode_challenge(&bytes[SLOT_ID_BYTES..response_start])?" in source
            and "responses: decode_responses(&bytes[response_start..opening_start])?" in source
        ),
        "challenge_high_bits_early_return": (
            "ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK" in source
            and 'return Err("challenge is not canonical: high bits are set".to_string());'
            in source
        ),
        "challenge_active_block_early_return": (
            "if active_block[rep] >= GROUPS" in source
            and '"challenge active block in round {rep} is out of range: {}"' in source
        ),
        "probe_mutates_last_algebraic_challenge_byte": (
            "invalid_challenge_bytes[SLOT_ID_BYTES + ALGEBRAIC_CHALLENGE_BYTES - 1] = 0xff;"
            in probe
        ),
        "probe_has_equalized_control": (
            '"invalid_challenge_mask_equalized"' in probe
            and "equalized_decode_diagnostic(&invalid_challenge_signature, false)" in probe
        ),
    }
    return checks


def build_payload() -> dict[str, Any]:
    row = load_campaign_row()
    checks = source_contains_all()
    return {
        "schema": "Descender.ct_issue_invalid_challenge_mask.v1",
        "issue_id": "CT-005",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "scope": {
            "group": "parse_malformed_input",
            "class": "invalid_challenge_mask",
            "operation": "ProductionSignature::decode",
            "owned_artifact_prefix": "ct_issue_invalid_challenge_mask",
        },
        "source_evidence": {
            "production_reference": rel(SOURCE),
            "decode_order_lines": "7304-7321",
            "challenge_decode_lines": "7414-7441",
            "probe": rel(PROBE),
            "probe_row_lines": "383-390",
            "source_checks": checks,
        },
        "timing_evidence": {
            "source_campaign": rel(CAMPAIGN_JSON),
            "baseline_class": "valid_linear_sigma_signature",
            "row": row,
            "policy_window": {"min": 0.80, "max": 1.25},
            "violates_policy": not (0.80 <= row["ratio_to_group_baseline"] <= 1.25),
        },
        "root_cause": {
            "summary": (
                "The malformed challenge byte is rejected inside decode_challenge "
                "before ProductionSignature::decode decodes responses, openings, "
                "and the proof."
            ),
            "details": [
                "decode_production_signature first checks total length, then constructs ProductionSignatureParts with challenge decoded before response/opening/proof fields.",
                "decode_challenge immediately returns when high bits outside ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK are set in the final algebraic challenge byte.",
                "decode_challenge also has per-round early returns for out-of-range active_block values, so challenge-format rejection depth depends on the public malformed byte pattern.",
                "The probe mutates SLOT_ID_BYTES + ALGEBRAIC_CHALLENGE_BYTES - 1 to 0xff, taking the high-bit early-return path.",
            ],
        },
        "secret_dependence_and_exploitability": {
            "secret_dependent": False,
            "public_input_channel": True,
            "assessment": (
                "This specific row is not a hidden-witness timing leak: the branch "
                "condition is derived from attacker-provided/public signature bytes "
                "during parsing. It is nevertheless externally distinguishable and "
                "violates the current timing-ratio policy for malformed public inputs."
            ),
            "exploitability": (
                "An adversary can classify malformed challenge-mask rejection versus "
                "deeper parse/valid decode work by timing decode calls, but this does "
                "not by itself expose the secret key, hidden account witness, sampler "
                "randomness, or pad material."
            ),
            "deployment_risk": (
                "Low for secret extraction in this isolated decode path; medium for "
                "protocol hardening because externally visible reject classes can aid "
                "fingerprinting, oracle composition, and regression drift unless the "
                "public parser policy explicitly permits variable-time rejects."
            ),
        },
        "proposed_remediation": {
            "shared_source_patch_required": True,
            "patch_status": "not_applied_by_CT_005_worker",
            "recommendation": (
                "Refactor production signature parsing into a full-scan decoder that "
                "accumulates public canonicality failures while decoding all fixed-size "
                "fields, then returns a generic malformed-signature error after the "
                "same parse work for all length-correct signatures."
            ),
            "concrete_steps": [
                "Keep the top-level length check as a public fast-fail only if the policy accepts length-class timing; otherwise normalize/copy to a fixed buffer before parsing.",
                "Replace decode_challenge early returns with accumulated error flags for high bits and active_block range over all REPS.",
                "Decode responses, openings, and proof unconditionally for length-correct signatures, accumulating their canonicality flags before constructing a result.",
                "Return one public malformed-signature error after the full scan when any accumulated flag is set.",
                "Apply the same full-scan strategy to reviewed account-carried and reviewed Fq12 decoders that share decode_challenge.",
            ],
            "test_plan": [
                "Add unit tests asserting invalid high challenge bits, out-of-range active blocks, bad responses, bad openings, and bad proof encodings all reject with the chosen generic error.",
                "Add a focused timing regression row for invalid_challenge_mask after the full-scan decoder; require ratio_to_group_baseline inside 0.80..1.25 or record an explicit public-fast-fail exception.",
                "Keep equalized_parse_diagnostic as a diagnostic control only; do not treat it as the production remediation.",
            ],
        },
        "status": "root_cause_identified_patch_proposed_not_applied",
    }


def render_md(payload: dict[str, Any]) -> str:
    row = payload["timing_evidence"]["row"]
    checks = payload["source_evidence"]["source_checks"]
    check_lines = "\n".join(
        f"- `{name}`: `{value}`" for name, value in sorted(checks.items())
    )
    steps = "\n".join(f"- {item}" for item in payload["proposed_remediation"]["concrete_steps"])
    tests = "\n".join(f"- {item}" for item in payload["proposed_remediation"]["test_plan"])
    return f"""# CT-005 Invalid Challenge Mask

- group: `{payload['scope']['group']}`
- class: `{payload['scope']['class']}`
- operation: `{payload['scope']['operation']}`
- status: `{payload['status']}`

## Timing Evidence

- source campaign: `{payload['timing_evidence']['source_campaign']}`
- baseline class: `{payload['timing_evidence']['baseline_class']}`
- mean ns: `{row['mean_avg_ns']}`
- median ns: `{row['median_avg_ns']}`
- ratio to group baseline: `{row['ratio_to_group_baseline']:.3f}`
- policy window: `0.80..1.25`
- violates policy: `{payload['timing_evidence']['violates_policy']}`

## Root Cause

`decode_production_signature` checks the total length, then decodes the challenge
before responses, openings, and proof. `decode_challenge` returns immediately
when the final algebraic challenge byte has bits outside
`ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK`; it also returns inside the per-round loop
when an active block is out of range. The probe sets that final algebraic
challenge byte to `0xff`, so the decoder takes the shallow high-bit rejection
path and skips the remaining fixed-size parse work.

## Source Checks

{check_lines}

Relevant locations:
- `{payload['source_evidence']['production_reference']}` lines `{payload['source_evidence']['decode_order_lines']}` and `{payload['source_evidence']['challenge_decode_lines']}`
- `{payload['source_evidence']['probe']}` lines `{payload['source_evidence']['probe_row_lines']}`

## Secret Dependence And Exploitability

This row is not secret-dependent. The branch condition is derived from public,
attacker-provided signature bytes, not from the secret key, hidden witness,
sampler randomness, or pad material. It is still an externally visible public
parse oracle: an adversary can distinguish challenge-mask rejection from deeper
parse or valid decode work by timing the API.

Deployment risk is low for direct secret extraction in this isolated decode
path, but medium for hardening because visible reject-class timing can support
fingerprinting, oracle composition, and regression drift unless the parser
explicitly allows variable-time public malformed-input handling.

## Proposed Remediation

Shared source patch needed, but not applied by this scoped worker:
{payload['proposed_remediation']['recommendation']}

Concrete steps:
{steps}

## Regression Test

{tests}
"""


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(render_md(payload))
    print(rel(OUT_JSON))
    print(rel(OUT_MD))


if __name__ == "__main__":
    main()
