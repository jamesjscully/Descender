#!/usr/bin/env python3
"""Generate CT-013 challenge-hash long-message timing issue artifacts."""

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
OUT_JSON = READINESS_DIR / "ct_issue_challenge_hash_long_message_n32_q12289.json"
OUT_MD = READINESS_DIR / "ct_issue_challenge_hash_long_message_n32_q12289.md"

ISSUE_ID = "CT-013"
GROUP = "hash_frame_public_input"
ISSUE_CLASS = "challenge_hash_long_message"
BASELINE_CLASS = "challenge_hash_baseline"
OPERATION = "challenge_hash_frame + hash_frame_to_challenge_bytes"
EQUALIZED_GROUP = "equalized_hash_frame_diagnostic"
EQUALIZED_CLASS = "challenge_hash_long_message_equalized"
EQUALIZED_BASELINE_CLASS = "challenge_hash_baseline_equalized"
EQUALIZED_OPERATION = "equalized_challenge_hash_frame_1024"
POLICY_MIN = 0.80
POLICY_MAX = 1.25


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def find_row(rows: list[dict[str, Any]], group: str, class_name: str, operation: str) -> dict[str, Any]:
    for row in rows:
        if (
            row.get("group") == group
            and row.get("class") == class_name
            and row.get("operation") == operation
        ):
            return row
    raise KeyError(f"{group}/{class_name}/{operation}")


def source_checks() -> dict[str, bool]:
    source = SOURCE.read_text(encoding="utf-8")
    probe = PROBE.read_text(encoding="utf-8")
    return {
        "challenge_hash_frame_appends_full_message": (
            "append_frame(&mut frame, b\"message\", message);" in source
        ),
        "challenge_hash_frame_allocates_variable_vec": (
            "let mut frame = Vec::new();" in source
            and "let mut frame = challenge_hash_frame_prefix(public_key, message, slot_id, digest);"
            in source
        ),
        "hash_frame_to_challenge_bytes_hashes_full_frame": (
            "pub fn hash_frame_to_challenge_bytes(frame: &[u8]) -> [u8; CHALLENGE_BYTES]" in source
            and "let digest = Sha256::digest(frame);" in source
        ),
        "append_frame_copies_data_and_encodes_data_len": (
            "out.extend_from_slice(&(data.len() as u64).to_le_bytes());" in source
            and "out.extend_from_slice(data);" in source
        ),
        "probe_compares_7_byte_message_to_1024_byte_message": (
            'b"message"' in probe
            and "&[7_u8; 1024]" in probe
            and '"challenge_hash_long_message"' in probe
        ),
        "probe_has_fixed_1024_byte_equalized_control": (
            "EQUALIZED_DIAGNOSTIC_MESSAGE_BYTES: usize = 1024" in probe
            and "padded_message[..copy_len].copy_from_slice(&message[..copy_len]);" in probe
            and '"challenge_hash_long_message_equalized"' in probe
        ),
    }


def build_payload() -> dict[str, Any]:
    campaign = json.loads(CAMPAIGN_JSON.read_text(encoding="utf-8"))
    rows = campaign["summary"]["annotated_rows"]
    issue_row = find_row(rows, GROUP, ISSUE_CLASS, OPERATION)
    baseline_row = find_row(rows, GROUP, BASELINE_CLASS, OPERATION)
    equalized_row = find_row(rows, EQUALIZED_GROUP, EQUALIZED_CLASS, EQUALIZED_OPERATION)
    equalized_baseline = find_row(
        rows, EQUALIZED_GROUP, EQUALIZED_BASELINE_CLASS, EQUALIZED_OPERATION
    )
    observed_ratio = float(issue_row["ratio_to_group_baseline"])
    equalized_ratio = float(equalized_row["ratio_to_group_baseline"])
    baseline_message_len = len(b"message")
    long_message_len = 1024
    return {
        "schema": "Descender.ct_issue_challenge_hash_long_message.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "issue_id": ISSUE_ID,
        "scope": {
            "group": GROUP,
            "class": ISSUE_CLASS,
            "operation": OPERATION,
            "owned_artifacts_glob": "*ct_issue_challenge_hash_long_message*",
        },
        "policy": {
            "ratio_window": [POLICY_MIN, POLICY_MAX],
            "passes": POLICY_MIN <= observed_ratio <= POLICY_MAX,
            "excess_over_max_ratio": max(0.0, observed_ratio - POLICY_MAX),
        },
        "measurement": {
            "campaign_artifact": rel(CAMPAIGN_JSON),
            "baseline_class": BASELINE_CLASS,
            "baseline_message_bytes": baseline_message_len,
            "issue_message_bytes": long_message_len,
            "baseline_mean_avg_ns": baseline_row["mean_avg_ns"],
            "baseline_median_avg_ns": baseline_row["median_avg_ns"],
            "issue_mean_avg_ns": issue_row["mean_avg_ns"],
            "issue_median_avg_ns": issue_row["median_avg_ns"],
            "ratio_to_group_baseline": observed_ratio,
            "samples": issue_row["samples"],
            "observed_outcome": issue_row["observed_outcome"],
        },
        "equalized_control": {
            "group": EQUALIZED_GROUP,
            "operation": EQUALIZED_OPERATION,
            "baseline_class": EQUALIZED_BASELINE_CLASS,
            "issue_class": EQUALIZED_CLASS,
            "fixed_message_bytes": long_message_len,
            "baseline_mean_avg_ns": equalized_baseline["mean_avg_ns"],
            "issue_mean_avg_ns": equalized_row["mean_avg_ns"],
            "ratio_to_group_baseline": equalized_ratio,
            "passes_policy": POLICY_MIN <= equalized_ratio <= POLICY_MAX,
            "interpretation": (
                "Padding/copying messages to the same 1024-byte public length before frame "
                "construction removes the observed timing ratio in the diagnostic row, which "
                "isolates the non-equalized issue to public message length."
            ),
        },
        "source_evidence": {
            "production_reference": rel(SOURCE),
            "challenge_hash_frame_lines": "6268-6281",
            "hash_frame_to_challenge_bytes_lines": "6289-6295",
            "challenge_hash_frame_prefix_lines": "6313-6333",
            "append_frame_lines": "7293-7298",
            "probe": rel(PROBE),
            "probe_row_lines": "529-599",
            "equalized_helper_lines": "203-220",
            "source_checks": source_checks(),
        },
        "root_cause": {
            "summary": (
                "Challenge hashing is intentionally proportional to the public message length: "
                "the frame builder copies the full message into a Vec and the challenge derivation "
                "hashes the full frame."
            ),
            "details": [
                "challenge_hash_frame delegates to challenge_hash_frame_prefix, then appends proof_commitment.",
                "challenge_hash_frame_prefix serializes public parameters and calls append_frame with label message and the complete message slice.",
                "append_frame writes the data length and then extends the output Vec with every message byte, making allocation/copy work length-dependent.",
                "hash_frame_to_challenge_bytes calls Sha256::digest(frame), so compression work is also proportional to the serialized frame length.",
                "The probe compares b\"message\" (7 bytes) against a 1024-byte public message, so the long-message row does more public copy and SHA-256 work.",
                "The equalized diagnostic pads both inputs to 1024 bytes before constructing the frame and measures a ratio near 1.0, confirming the length dependency.",
            ],
        },
        "secret_dependence_and_exploitability": {
            "secret_dependent": False,
            "attacker_controlled_signal": "public message length",
            "public_input_channel": True,
            "assessment": (
                "This row is not evidence of secret-key, hidden-witness, sampler, pad, or proof "
                "material leakage. The measured difference follows from public message length, "
                "which is normally known to the caller and protocol transcript."
            ),
            "exploitability": (
                "An observer can estimate whether the API hashed a short or long public message "
                "by timing challenge derivation. In this harness that distinguishes 7-byte and "
                "1024-byte messages, but it does not reveal private signing state unless a caller "
                "places secret data directly in the public message field or protocol policy treats "
                "message length as confidential."
            ),
            "deployment_risk": (
                "Low for direct secret extraction under the current public-input model; medium "
                "for any blanket constant-time claim covering challenge_hash_frame because the "
                "operation is variable-time by construction unless inputs are length-normalized "
                "or the claim explicitly excludes public message length."
            ),
        },
        "proposed_remediation": {
            "shared_source_modified": False,
            "patch_status": "proposed_only_by_CT_013_worker",
            "primary_recommendation": (
                "Decide the policy boundary first: either document challenge hashing as "
                "constant-time only with respect to secrets and variable in public message length, "
                "or add a deployment wrapper that hashes fixed-size message digests/frames instead "
                "of variable-length messages."
            ),
            "concrete_options": [
                "Preferred protocol cleanup: make challenge_hash_frame accept a fixed [u8; DIGEST_BYTES] message digest or prehashed transcript digest, domain-separated from slot_digest, and include only fixed-size fields in the challenge frame.",
                "Compatibility hardening: introduce a deployment-only fixed_message_challenge_hash_frame helper that copies/truncates/pads public messages to a documented maximum before frame construction, mirroring the equalized diagnostic shape.",
                "Policy alternative: leave source unchanged, but remove hash_frame_public_input/message-length rows from strict CT closure and record that challenge hashing is variable-time in public transcript length.",
                "Avoid claiming constant-time behavior for challenge_hash_frame over arbitrary &[u8] messages unless one of the fixed-length wrappers is used.",
            ],
            "test_plan": [
                "Add a focused CT regression row named challenge_hash_long_message_fixed or challenge_hash_long_message_equalized for the production wrapper; gate ratio within 0.80..1.25.",
                "Add unit tests proving short and long messages map to distinct domain-separated fixed frames and that the fixed wrapper always processes the documented maximum or fixed digest length.",
                "Keep a negative documentation test or artifact showing raw challenge_hash_frame remains length-proportional and is not the deployment constant-time boundary.",
                "If using prehashing, add transcript tests that changing message bytes changes the challenge while the final challenge frame length remains constant.",
            ],
        },
        "status": "root_cause_identified_patch_proposed_not_applied",
    }


def render_md(payload: dict[str, Any]) -> str:
    measurement = payload["measurement"]
    policy = payload["policy"]
    equalized = payload["equalized_control"]
    checks = payload["source_evidence"]["source_checks"]
    status = "passes_policy" if policy["passes"] else "fails_policy"
    check_lines = "\n".join(
        f"- `{name}`: `{str(value).lower()}`" for name, value in sorted(checks.items())
    )
    details = "\n".join(f"- {item}" for item in payload["root_cause"]["details"])
    options = "\n".join(f"- {item}" for item in payload["proposed_remediation"]["concrete_options"])
    tests = "\n".join(f"- {item}" for item in payload["proposed_remediation"]["test_plan"])
    security = payload["secret_dependence_and_exploitability"]
    source = payload["source_evidence"]
    return f"""# CT-013 Challenge Hash Long Message

## Verdict

- issue: `{payload['issue_id']}`
- group: `{payload['scope']['group']}`
- class: `{payload['scope']['class']}`
- operation: `{payload['scope']['operation']}`
- policy window: `{policy['ratio_window'][0]:.2f}..{policy['ratio_window'][1]:.2f}`
- observed ratio: `{measurement['ratio_to_group_baseline']:.3f}`
- status: `{status}`

The long-message challenge-hash row is slower than the short-message baseline:
`{measurement['issue_mean_avg_ns']}` ns mean for a
`{measurement['issue_message_bytes']}`-byte message versus
`{measurement['baseline_mean_avg_ns']}` ns mean for a
`{measurement['baseline_message_bytes']}`-byte message, ratio
`{measurement['ratio_to_group_baseline']:.3f}`. This exceeds the configured
`{policy['ratio_window'][1]:.2f}` ceiling by `{policy['excess_over_max_ratio']:.3f}`.

## Root Cause

{payload['root_cause']['summary']}

{details}

The equalized diagnostic control pads/copies both messages to
`{equalized['fixed_message_bytes']}` bytes before hashing. Its ratio is
`{equalized['ratio_to_group_baseline']:.3f}` (`{equalized['issue_mean_avg_ns']}` ns
versus `{equalized['baseline_mean_avg_ns']}` ns), which passes the policy and
confirms the measured issue is public message-length work, not challenge-byte
content.

## Evidence

Relevant locations:
- `{source['production_reference']}` lines `{source['challenge_hash_frame_lines']}`, `{source['hash_frame_to_challenge_bytes_lines']}`, `{source['challenge_hash_frame_prefix_lines']}`, and `{source['append_frame_lines']}`
- `{source['probe']}` lines `{source['probe_row_lines']}` and `{source['equalized_helper_lines']}`
- campaign: `{measurement['campaign_artifact']}`

Source checks:
{check_lines}

## Secret Dependence And Exploitability

This row is not secret-dependent. The attacker-controlled signal is
`{security['attacker_controlled_signal']}` and the value is public transcript
input. {security['assessment']}

{security['exploitability']}

{security['deployment_risk']}

## Proposed Remediation

No shared source was edited by this scoped worker.

{payload['proposed_remediation']['primary_recommendation']}

Concrete options:
{options}

## Regression Test

{tests}

## Patch Boundary

This worker owns only files whose names contain
`ct_issue_challenge_hash_long_message`. Shared Rust source changes are reported
above as proposed remediation, not applied here.
"""


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    OUT_MD.write_text(render_md(payload), encoding="utf-8")
    print(rel(OUT_JSON))
    print(rel(OUT_MD))


if __name__ == "__main__":
    main()
