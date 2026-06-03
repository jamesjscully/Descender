#!/usr/bin/env python3
"""Emit CT-010 slot-out-of-range verifier timing issue artifacts."""

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
DEFAULT_JSON = READINESS_DIR / "ct_issue_slot_out_of_range_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "ct_issue_slot_out_of_range_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def find_row(rows: list[dict[str, Any]], group: str, class_name: str) -> dict[str, Any]:
    for row in rows:
        if row.get("group") == group and row.get("class") == class_name:
            return row
    raise KeyError(f"{group}/{class_name}")


def build_payload(campaign: dict[str, Any]) -> dict[str, Any]:
    rows = campaign["summary"]["annotated_rows"]
    raw_rows = campaign["probe"]["rows"]
    policy = campaign["summary"]["timing_ratio_policy"]
    baseline = find_row(rows, "rejection_class", "verifier_callback_failure")
    valid = find_row(rows, "rejection_class", "valid_accept_dummy_callback")
    issue = find_row(rows, "rejection_class", "slot_out_of_range")
    consumed = find_row(rows, "rejection_class", "slot_already_consumed")
    proof_mismatch = find_row(raw_rows, "rejection_class", "proof_commitment_mismatch")

    policy_pass = policy["min_ratio"] <= issue["ratio_to_group_baseline"] <= policy["max_ratio"]
    return {
        "schema": "Descender.constant_time_issue.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "worker": "CT-010",
        "issue_id": "CT-010",
        "issue_name": "ct_issue_slot_out_of_range",
        "scope": {
            "group": "rejection_class",
            "class": "slot_out_of_range",
            "operation": "deployment_verify_with_account_binding",
            "owned_artifacts_glob": "*ct_issue_slot_out_of_range*",
        },
        "environment": {
            "platform": platform.platform(),
            "python_version": platform.python_version(),
        },
        "inputs": {
            "campaign_json": rel(CAMPAIGN_JSON),
            "campaign_probe": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
            "facade_source": "tools/production-reference/src/lib.rs",
            "verifier_source": "tools/parameter-search/src/production_reference.rs",
        },
        "measurements": {
            "valid_accept_dummy_callback": valid,
            "callback_failure_baseline": baseline,
            "slot_out_of_range": issue,
            "adjacent_slot_already_consumed": consumed,
            "later_proof_commitment_mismatch_control": proof_mismatch,
            "reported_ratio_current_campaign": issue["ratio_to_group_baseline"],
            "reported_ratio_prior_closure": 0.427,
            "ratio_policy": policy,
            "policy_pass": policy_pass,
        },
        "code_references": [
            {
                "path": "tools/production-reference/src/lib.rs",
                "lines": "56-64",
                "observation": (
                    "The deployment facade calls verify_with_account_binding_sanitized and "
                    "maps account-binding errors directly to verifier rejections before the "
                    "accepted callback or consume path runs."
                ),
            },
            {
                "path": "tools/parameter-search/src/production_reference.rs",
                "lines": "4735-4740",
                "observation": (
                    "verify_account_binding fully decodes the signature, then uses "
                    "account.get_slot(parts.slot_id); a missing slot returns "
                    "'slot id is out of range'."
                ),
            },
            {
                "path": "tools/parameter-search/src/production_reference.rs",
                "lines": "4959-4968",
                "observation": (
                    "verify_with_account_binding_sanitized invokes the verifier callback only "
                    "after verify_account_binding succeeds, so this class never enters the "
                    "callback boundary."
                ),
            },
            {
                "path": "tools/parameter-search/src/production_reference.rs",
                "lines": "4971-4994",
                "observation": (
                    "The account consumption path is also after the callback acceptance decision; "
                    "slot_out_of_range rejects before clone/consume work."
                ),
            },
            {
                "path": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                "lines": "321-323,464-477",
                "observation": (
                    "The campaign mutates only the public slot id to 7 in a one-slot account and "
                    "checks for the sanitized 'slot id is out of range' rejection."
                ),
            },
        ],
        "root_cause": {
            "summary": (
                "The slot_out_of_range row is a public account-state control-flow cliff in the "
                "deployment verifier facade. A canonical signature with slot_id 7 is decoded, "
                "then verify_account_binding exits immediately when the one-slot account has no "
                "matching slot."
            ),
            "timing_mechanism": (
                "The rejection avoids the sanitized verifier callback, avoids callback error "
                "sanitization work, avoids proof-commitment comparison against a slot, and avoids "
                "the accepted-path account clone/consume transition. The campaign's rejection "
                "baseline is verifier_callback_failure, which performs successful account binding "
                "and then invokes the callback, so slot_out_of_range measures at about 0.380x of "
                "that baseline in the current campaign."
            ),
            "not_caused_by": [
                "secret-key access",
                "hidden witness decoding",
                "secret-dependent callback behavior",
                "account randomizer preimage access",
                "cryptographic proof verification",
            ],
        },
        "secret_dependence_and_exploitability": {
            "secret_dependent": False,
            "attacker_controlled_signal": "public signature slot_id and public account slot count/state",
            "leaked_information": (
                "Whether the claimed public slot id exists in the supplied account state, and "
                "coarse placement of the rejection before or after account-binding success."
            ),
            "secret_extraction_risk": "low_not_indicated",
            "remote_fingerprint_or_rejection_oracle": "possible_for_public_account_state_classes",
            "deployment_relevance": (
                "This should block a blanket constant-time verifier-facade claim across rejection "
                "classes. It does not by itself indicate leakage of signing keys or hidden-source "
                "witnesses because the branch predicate is public and the verifier callback is not "
                "entered."
            ),
        },
        "remediation": {
            "recommended_change": (
                "For any deployment path that claims constant-time rejection handling, split the "
                "diagnostic facade from a hardened facade. The hardened facade should normalize "
                "account-binding checks into a fixed-work precheck over public account records, "
                "run a constant-shape verifier callback boundary or documented dummy callback path, "
                "and return a single sanitized rejection after all public rejection classes have "
                "performed comparable work."
            ),
            "patch_shape": [
                "Decode the signature into a fixed local shape before branching on slot membership.",
                "Scan account slots with fixed bounds for the deployment account profile, accumulating slot_exists, slot_unused, and commitment_match masks.",
                "For missing slots, feed a zero/dummy slot record into the same commitment-validation and callback-boundary shape used by present slots.",
                "Defer conversion to 'slot id is out of range' or a generic sanitized rejection until after callback/consume-equivalent work has completed.",
                "Keep detailed slot diagnostics behind non-constant-time tooling paths if public rejection timing is policy-exempt.",
            ],
            "test_plan": [
                "Add a CT regression row named ct_issue_slot_out_of_range comparing slot_out_of_range, verifier_callback_failure, slot_already_consumed, and proof_commitment_mismatch.",
                "Require slot_out_of_range/verifier_callback_failure to fall inside the configured 0.80..1.25 timing window on the hardened deployment facade.",
                "Add a callback-count unit test proving the current diagnostic facade does not invoke the real verifier callback for missing slots, and a hardened-path test proving the dummy/equalized callback path is taken.",
                "Retain fail-closed tests for slot id 7 against a one-slot account.",
            ],
        },
        "verdict": {
            "fail_closed": issue["observed_outcome"] == "all_batches_matched",
            "constant_time_policy": "fail" if not policy_pass else "pass",
            "deployment_blocker_for_blanket_ct_claim": not policy_pass,
            "source_patch_included": False,
            "rationale": (
                "The verifier rejects safely, but rejection happens before the sanitized callback "
                "and consume path. That public branch creates a material timing distinction across "
                "deployment verifier rejection classes."
            ),
        },
    }


def render_markdown(payload: dict[str, Any]) -> str:
    m = payload["measurements"]
    issue = m["slot_out_of_range"]
    baseline = m["callback_failure_baseline"]
    valid = m["valid_accept_dummy_callback"]
    consumed = m["adjacent_slot_already_consumed"]
    proof = m["later_proof_commitment_mismatch_control"]
    policy = m["ratio_policy"]

    return f"""# CT-010: slot_out_of_range

## Scope

- group: `rejection_class`
- class: `slot_out_of_range`
- operation: `deployment_verify_with_account_binding`
- owned lane: files containing `ct_issue_slot_out_of_range`

## Measurement

- callback-failure baseline mean: `{baseline["mean_avg_ns"]:.1f} ns`
- valid accept mean: `{valid["mean_avg_ns"]:.1f} ns`
- slot-out-of-range mean: `{issue["mean_avg_ns"]:.1f} ns`
- ratio to rejection baseline: `{issue["ratio_to_group_baseline"]:.3f}`
- prior closure ratio: `{m["reported_ratio_prior_closure"]:.3f}`
- adjacent consumed-slot ratio: `{consumed["ratio_to_group_baseline"]:.3f}`
- later proof-mismatch mean: `{proof["mean_avg_ns"]:.1f} ns`
- policy window: `{policy["min_ratio"]:.2f}..{policy["max_ratio"]:.2f}`
- fail-closed outcome: `{issue["observed_outcome"]}`

## Root Cause

The facade calls `verify_with_account_binding_sanitized` before it invokes the caller-supplied verifier callback or the accepted-path account transition. The sanitized verifier first calls `verify_account_binding`, which decodes the public signature and then performs `account.get_slot(parts.slot_id)`.

The probe mutates only the public slot id to `7` while using a one-slot account. That lookup fails and returns `slot id is out of range`, so the flow exits before callback dispatch, callback-error sanitization, proof-commitment comparison against a real slot, account cloning, and slot consumption.

Relevant source references:

- `tools/production-reference/src/lib.rs:56-64` - deployment facade maps account-binding errors to verifier rejections before the accepted callback and transition.
- `tools/parameter-search/src/production_reference.rs:4735-4740` - `verify_account_binding` decodes, then rejects a missing account slot.
- `tools/parameter-search/src/production_reference.rs:4959-4968` - the sanitized callback runs only after account binding succeeds.
- `tools/parameter-search/src/production_reference.rs:4971-4994` - consume work runs only after callback report acceptance.
- `tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs:321-323,464-477` - the campaign constructs and times the slot-out-of-range case.

## Secret Dependence And Exploitability

- secret-dependent: `False`
- attacker-controlled signal: `public signature slot_id and public account slot count/state`
- leaked information: whether the claimed public slot exists and whether rejection occurred before account-binding success.
- secret extraction risk: `low_not_indicated`

This is a real timing-policy failure for a blanket verifier-facade constant-time claim, but it is not evidence of signing-key or hidden-witness leakage. The branch predicate is public and the verifier callback is not entered for this class.

Remote callers may still use the timing gap as a public rejection-class oracle or implementation fingerprint if the facade is exposed over a low-noise service.

## Remediation

For a hardened deployment facade, keep diagnostic APIs separate and add a fixed-work account-binding path:

- Decode the signature into a fixed local shape before branching on slot membership.
- Scan account slots with fixed bounds for the deployment account profile, accumulating `slot_exists`, `slot_unused`, and `commitment_match` masks.
- For missing slots, feed a zero/dummy slot record into commitment-validation and callback-boundary work with the same shape as present slots.
- Defer conversion to `slot id is out of range` or a generic sanitized rejection until after callback/consume-equivalent work completes.
- Keep detailed slot diagnostics behind tooling paths if public rejection timing is explicitly policy-exempt.

## Regression Test

- Add a CT regression row named `ct_issue_slot_out_of_range` comparing `slot_out_of_range`, `verifier_callback_failure`, `slot_already_consumed`, and `proof_commitment_mismatch`.
- Require `slot_out_of_range / verifier_callback_failure` to stay inside `{policy["min_ratio"]:.2f}..{policy["max_ratio"]:.2f}` on the hardened deployment facade.
- Add callback-count tests for both paths: current diagnostic facade does not invoke the real verifier callback for missing slots; hardened path performs dummy/equalized callback-boundary work.
- Retain fail-closed tests for slot id `7` against a one-slot account.

## Verdict

- fail closed: `True`
- constant-time policy: `fail`
- source patch included: `False`
- deployment blocker for blanket CT claim: `True`
"""


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--campaign-json", type=Path, default=CAMPAIGN_JSON)
    parser.add_argument("--json-out", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md-out", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    campaign = load_json(args.campaign_json)
    payload = build_payload(campaign)
    args.json_out.parent.mkdir(parents=True, exist_ok=True)
    args.json_out.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    args.md_out.write_text(render_markdown(payload), encoding="utf-8")
    print(f"wrote {rel(args.json_out)}")
    print(f"wrote {rel(args.md_out)}")


if __name__ == "__main__":
    main()
