#!/usr/bin/env python3
"""Generate CT-011 slot-already-consumed timing issue artifacts."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
CAMPAIGN_JSON = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
CLOSURE_JSON = READINESS_DIR / "side_channel_timing_closure_n32_q12289.json"
FACADE_SOURCE = ROOT / "tools/production-reference/src/lib.rs"
REFERENCE_SOURCE = ROOT / "tools/parameter-search/src/production_reference.rs"
PROBE_SOURCE = ROOT / "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs"
OUT_JSON = READINESS_DIR / "ct_issue_slot_already_consumed_n32_q12289.json"
OUT_MD = READINESS_DIR / "ct_issue_slot_already_consumed_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def find_row(rows: list[dict[str, Any]]) -> dict[str, Any]:
    for row in rows:
        if (
            row.get("group") == "rejection_class"
            and row.get("class") == "slot_already_consumed"
            and row.get("operation") == "deployment_verify_with_account_binding"
        ):
            return row
    raise SystemExit("slot_already_consumed timing row not found")


def source_checks() -> dict[str, bool]:
    facade = FACADE_SOURCE.read_text()
    reference = REFERENCE_SOURCE.read_text()
    probe = PROBE_SOURCE.read_text()
    return {
        "deployment_facade_uses_sanitized_binding_then_completion": (
            "verify_with_account_binding_sanitized(" in facade
            and "complete_verified_signature(account, checked, report, is_accepted)" in facade
        ),
        "verify_account_binding_decodes_then_checks_slot_occupancy": (
            "let parts = signature" in reference
            and ".decode()" in reference
            and "let slot = account.get_slot(parts.slot_id)" in reference
            and "if !slot.is_unused()" in reference
            and '"slot is already consumed"' in reference
        ),
        "consumed_slot_return_precedes_commitment_validation": (
            'return Err(ProductionAccountBindingError::new(\n            Some(parts.slot_id),\n            "slot is already consumed",'
            in reference
            and "slot.commitment()\n        .validate()" in reference
        ),
        "sanitized_callback_is_after_account_binding": (
            "let checked = verify_account_binding(account, signature)?" in reference
            and "PRODUCTION_VERIFIER_CALLBACK_FAILURE_REASON" in reference
        ),
        "accepted_completion_consumes_only_after_callback_accepts": (
            "let next_account = if accepted" in reference
            and "consume_verified_signature(account, &checked)" in reference
        ),
        "probe_constructs_consumed_account": (
            "let consumed_account = test_account(false);" in probe
            and '"slot_already_consumed"' in probe
            and '"slot is already consumed"' in probe
        ),
    }


def build_payload() -> dict[str, Any]:
    campaign = load_json(CAMPAIGN_JSON)
    closure = load_json(CLOSURE_JSON)
    campaign_row = find_row(campaign["summary"]["annotated_rows"])
    closure_row = find_row(closure["timing_policy_issue_rows"])
    policy = campaign["summary"]["timing_ratio_policy"]
    return {
        "schema": "Descender.ct_issue_slot_already_consumed.v1",
        "issue_id": "CT-011",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "scope": {
            "group": "rejection_class",
            "class": "slot_already_consumed",
            "operation": "deployment_verify_with_account_binding",
            "owned_artifact_prefix": "ct_issue_slot_already_consumed",
        },
        "timing_evidence": {
            "source_campaign": rel(CAMPAIGN_JSON),
            "source_closure": rel(CLOSURE_JSON),
            "baseline_class": "verifier_callback_failure",
            "campaign_row": campaign_row,
            "closure_row": closure_row,
            "reported_ratio_range": [
                campaign_row["ratio_to_group_baseline"],
                closure_row["ratio_to_group_baseline"],
            ],
            "policy_window": {
                "min": policy["min_ratio"],
                "max": policy["max_ratio"],
            },
            "violates_policy": not (
                policy["min_ratio"]
                <= campaign_row["ratio_to_group_baseline"]
                <= policy["max_ratio"]
            ),
        },
        "source_evidence": {
            "facade_source": rel(FACADE_SOURCE),
            "facade_lines": "38-65",
            "reference_source": rel(REFERENCE_SOURCE),
            "verify_account_binding_lines": "4731-4756",
            "sanitized_callback_lines": "4943-4968",
            "consume_completion_lines": "4971-4995",
            "probe_source": rel(PROBE_SOURCE),
            "probe_setup_lines": "303-308",
            "probe_row_lines": "480-494",
            "source_checks": source_checks(),
        },
        "root_cause": {
            "summary": (
                "The deployment facade rejects a consumed slot inside "
                "verify_account_binding before it reaches the sanitized verifier "
                "callback or accepted-completion path. The valid rejection-class "
                "baseline dispatches the dummy verifier callback, while the consumed "
                "slot row stops after decode, slot lookup, and the public unused flag."
            ),
            "control_flow": [
                "deployment_verify_with_account_binding calls verify_with_account_binding_sanitized and only then evaluates the accepted callback and complete_verified_signature.",
                "verify_with_account_binding_sanitized first calls verify_account_binding(account, signature).",
                "verify_account_binding decodes the signature and performs account.get_slot(parts.slot_id).",
                "For a consumed slot, !slot.is_unused() immediately returns ProductionAccountBindingError(\"slot is already consumed\").",
                "That early return skips slot commitment validation, proof commitment/randomizer comparison, sanitized verifier callback dispatch, accepted(report), and consume_verified_signature.",
                "The probe row uses the same canonical signature as the accepted baseline but passes test_account(false), so the timing difference is dominated by this public account-state branch.",
            ],
        },
        "secret_dependence_and_exploitability": {
            "secret_dependent": False,
            "public_input_or_state_channel": True,
            "classification": "public_account_state_dependent_rejection_class_timing",
            "assessment": (
                "This row is not evidence of hidden-witness, signing-key, pad-seed, "
                "or sampler leakage: the branch predicate is the public account slot "
                "occupancy bit exposed to the verifier state machine. It is still "
                "externally distinguishable when the same API surface is timed."
            ),
            "exploitability": (
                "An adversary who can submit candidate signatures against observable "
                "account state can distinguish already-consumed slot rejection from "
                "deeper account-binding failures and callback-driven verifier failure. "
                "That supports replay/oracle fingerprinting, but not direct secret extraction "
                "under the current boundary."
            ),
            "deployment_risk": (
                "Low for direct secret extraction; medium for protocol hardening if the "
                "deployment claims uniform timing across public rejection classes. If slot "
                "occupancy is already public chain state, the security policy may explicitly "
                "exempt this branch while still avoiding detailed rejection oracles."
            ),
        },
        "proposed_remediation": {
            "shared_source_patch_required": True,
            "patch_status": "not_applied_by_CT_011_worker",
            "recommendation": (
                "Choose and encode the deployment policy. If public account-state rejection "
                "timing is in scope, make the account-binding facade evaluate a fixed-shape "
                "set of public checks and defer the consumed-slot error until after the same "
                "commitment/check/callback-equivalent work as other rejection rows. If it is "
                "out of scope, classify consumed-slot timing as public state dependent and "
                "keep the reason sanitized at the network boundary."
            ),
            "concrete_steps": [
                "Split account binding into a checked-context builder that accumulates slot_present, slot_unused, commitment_valid, and proof_commitment_match flags without early return for any in-range slot.",
                "For consumed in-range slots, continue through commitment validation and proof commitment comparison using the public slot commitment, but mask the final result to reject.",
                "Do not dispatch a secret-bearing final verifier callback with an invalid checked context; instead run an explicit fixed-work public dummy/equalization hook or require the final verifier facade to expose a safe reject-work path.",
                "Collapse externally visible deployment errors for account-binding rejection classes to a generic sanitized rejection unless diagnostics are explicitly requested off the timing-sensitive path.",
                "Keep consume_verified_signature unchanged as a state-transition guard, but do not count it as the equalizer for pre-callback rejection classes because it is only reached after accepted reports.",
            ],
            "test_plan": [
                "Add a focused CT regression row comparing slot_already_consumed, slot_out_of_range, proof_commitment_mismatch, verifier_callback_failure, and valid_accept_dummy_callback under the chosen deployment policy.",
                "If timing is in scope, require slot_already_consumed ratio_to_group_baseline to fall inside 0.80..1.25 on the final verifier facade rather than only the diagnostic reference probe.",
                "If timing is policy-exempt, add a machine-readable policy exception that marks the row public_account_state_dependent_not_secret_dependent and removes it from secret-dependent CT gates.",
                "Add unit tests that consumed-slot replay remains fail-closed and that accepted verification still marks the next account slot consumed exactly once.",
            ],
        },
        "status": "root_cause_identified_patch_proposed_not_applied",
    }


def render_markdown(payload: dict[str, Any]) -> str:
    timing = payload["timing_evidence"]
    campaign_row = timing["campaign_row"]
    closure_row = timing["closure_row"]
    source = payload["source_evidence"]
    checks = "\n".join(
        f"- `{name}`: `{value}`"
        for name, value in sorted(source["source_checks"].items())
    )
    flow = "\n".join(f"- {item}" for item in payload["root_cause"]["control_flow"])
    steps = "\n".join(
        f"- {item}" for item in payload["proposed_remediation"]["concrete_steps"]
    )
    tests = "\n".join(f"- {item}" for item in payload["proposed_remediation"]["test_plan"])
    return f"""# CT-011 Slot Already Consumed

- group: `{payload['scope']['group']}`
- class: `{payload['scope']['class']}`
- operation: `{payload['scope']['operation']}`
- status: `{payload['status']}`

## Timing Evidence

| source | mean ns | median ns | ratio |
|---|---:|---:|---:|
| campaign | `{campaign_row['mean_avg_ns']}` | `{campaign_row['median_avg_ns']}` | `{campaign_row['ratio_to_group_baseline']:.3f}` |
| closure replay | `{closure_row['mean_avg_ns']}` | `{closure_row['median_avg_ns']}` | `{closure_row['ratio_to_group_baseline']:.3f}` |

- source campaign: `{timing['source_campaign']}`
- source closure: `{timing['source_closure']}`
- baseline class: `{timing['baseline_class']}`
- policy window: `{timing['policy_window']['min']:.2f}..{timing['policy_window']['max']:.2f}`
- violates policy: `{timing['violates_policy']}`

## Root Cause

{payload['root_cause']['summary']}

Control flow:
{flow}

Relevant locations:
- `{source['facade_source']}` lines `{source['facade_lines']}`
- `{source['reference_source']}` lines `{source['verify_account_binding_lines']}`, `{source['sanitized_callback_lines']}`, and `{source['consume_completion_lines']}`
- `{source['probe_source']}` lines `{source['probe_setup_lines']}` and `{source['probe_row_lines']}`

## Source Checks

{checks}

## Secret Dependence And Exploitability

Classification: `{payload['secret_dependence_and_exploitability']['classification']}`.

{payload['secret_dependence_and_exploitability']['assessment']}

Exploitability: {payload['secret_dependence_and_exploitability']['exploitability']}

Deployment risk: {payload['secret_dependence_and_exploitability']['deployment_risk']}

## Proposed Remediation

Shared source patch needed, but not applied by this scoped worker.

{payload['proposed_remediation']['recommendation']}

Concrete steps:
{steps}

## Regression Test

{tests}

## Reproduction

```bash
python3 scripts/research/ct_issue_slot_already_consumed.py
```

To refresh the underlying timing campaign:

```bash
cargo run --manifest-path tools/production-reference/Cargo.toml --release --bin pre_third_party_side_channel_fault_probe -- --samples 5 --batch-iterations 100
python3 scripts/research/side_channel_timing_closure.py
```

JSON artifact: `{rel(OUT_JSON)}`.
"""


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(render_markdown(payload))


if __name__ == "__main__":
    main()
