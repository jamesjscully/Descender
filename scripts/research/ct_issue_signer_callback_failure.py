#!/usr/bin/env python3
"""Generate CT-012 signer callback failure timing issue artifacts."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_DIR = ROOT / "research" / "cryptanalysis" / "signature_readiness_20260516"
JSON_OUT = OUT_DIR / "ct_issue_signer_callback_failure.json"
MD_OUT = OUT_DIR / "ct_issue_signer_callback_failure.md"
CAMPAIGN = OUT_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
CLOSURE = OUT_DIR / "side_channel_timing_closure_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def row_by_class(rows: list[dict[str, Any]], class_name: str) -> dict[str, Any]:
    return next(row for row in rows if row.get("class") == class_name)


def campaign_annotated_rows(campaign: dict[str, Any]) -> list[dict[str, Any]]:
    for key in ("annotated_rows", "rows"):
        rows = campaign.get(key)
        if isinstance(rows, list) and rows:
            return rows
    analysis = campaign.get("analysis", {})
    rows = analysis.get("annotated_rows")
    if isinstance(rows, list) and rows:
        return rows
    summary = campaign.get("summary", {})
    rows = summary.get("annotated_rows")
    if isinstance(rows, list) and rows:
        return rows
    raise KeyError("could not find campaign annotated rows")


def build_report() -> dict[str, Any]:
    campaign = load_json(CAMPAIGN)
    closure = load_json(CLOSURE)
    campaign_rows = campaign_annotated_rows(campaign)
    campaign_row = row_by_class(campaign_rows, "signer_callback_failure")
    closure_row = row_by_class(closure["timing_policy_issue_rows"], "signer_callback_failure")

    return {
        "schema": "Descender.ct_issue_signer_callback_failure.v1",
        "issue": {
            "id": "CT-012",
            "group": "fault_injection_callback",
            "class": "signer_callback_failure",
            "operation": "deployment_sign_with_secret_key",
            "status": "confirmed_callback_failure_timing_cliff",
            "policy_window": [0.8, 1.25],
            "reported_timing_ratio_range": [0.610, 0.637],
        },
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "inputs": {
            "campaign_artifact": rel(CAMPAIGN),
            "closure_artifact": rel(CLOSURE),
            "deployment_facade_source": "tools/production-reference/src/lib.rs",
            "signer_boundary_source": "tools/parameter-search/src/production_reference.rs",
            "probe_source": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
        },
        "measurements": {
            "campaign_row": campaign_row,
            "closure_row": closure_row,
            "baseline_interpretation": (
                "The campaign script assigns fault_injection_callback rows to the "
                "rejection-class baseline verifier_callback_failure. The signer callback "
                "failure row is therefore a shorter signing-side failure path relative to "
                "a verification-side sanitized callback failure baseline."
            ),
        },
        "root_cause": {
            "summary": (
                "deployment_sign_with_secret_key delegates to sign_with_secret_key_sanitized, "
                "which delegates to sign_with_account_binding_sanitized. The sanitized wrapper "
                "does hide the callback's error string, but it still returns immediately when "
                "the signer callback returns Err. That skips the success-path signature decode, "
                "slot-id check, proof-commitment check, and all signing algorithm work hidden "
                "inside the callback."
            ),
            "evidence": [
                {
                    "source": "tools/production-reference/src/lib.rs",
                    "lines": "14-23",
                    "finding": (
                        "The deployment facade is only a thin wrapper around "
                        "sign_with_secret_key_sanitized."
                    ),
                },
                {
                    "source": "tools/parameter-search/src/production_reference.rs",
                    "lines": "4577-4599",
                    "finding": (
                        "sign_with_account_binding validates the public account commitment, "
                        "checks the public unused slot flag, invokes the callback, then only "
                        "after a successful callback decodes and checks the returned signature."
                    ),
                },
                {
                    "source": "tools/parameter-search/src/production_reference.rs",
                    "lines": "4705-4728",
                    "finding": (
                        "The sanitized account/secret-key wrappers map any callback Err to "
                        "PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON but preserve the early "
                        "return shape."
                    ),
                },
                {
                    "source": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                    "lines": "512-525",
                    "finding": (
                        "The probe injects an Err-returning signer callback and asserts only "
                        "that the public sanitized reason is returned."
                    ),
                },
                {
                    "source": "tools/production-reference/src/lib.rs",
                    "lines": "2001-2007",
                    "finding": (
                        "The facade unit test confirms the sanitized public reason but does "
                        "not assert equalized failure timing."
                    ),
                },
            ],
            "mechanism": [
                "The failure callback receives the reference keypair, public slot id, and witness commitment, then returns Err.",
                "map_err replaces the callback detail with a fixed public string.",
                "The question mark on the callback result unwinds before signature.decode() and account-binding checks.",
                "A normal successful signing path must build a signature, decode it, and check slot and proof commitments.",
            ],
        },
        "secret_dependence": {
            "classification": "not_demonstrated_as_secret_dependent_by_this_row",
            "secret_material_crosses_boundary": True,
            "secret_material_touched_before_divergence": (
                "The wrapper passes secret_key.reference_keypair() into the caller-supplied "
                "callback before the observed Err, so the boundary is secret-adjacent. The "
                "probe callback itself ignores that reference and always fails."
            ),
            "current_probe_control": (
                "In the measured row, divergence is controlled by a deterministic injected "
                "callback failure, not by a secret-key predicate, sampler rejection count, "
                "hidden witness value, or signature byte content."
            ),
            "risk": (
                "A real signer callback whose error probability, retry count, or failure "
                "location depends on secret material would expose that dependence through "
                "this facade because the facade does not equalize failure and success work."
            ),
        },
        "exploitability": {
            "secret_extraction": "not_shown_for_the_current_deterministic_fault_probe",
            "remote_oracle": (
                "Possible as a callback-failure-vs-success timing oracle if an attacker can "
                "trigger or observe signing attempts across a low-noise service boundary."
            ),
            "most_relevant_attack_surface": [
                "secret-dependent rejection or retry behavior inside the real signing callback",
                "faulted signer integrations that fail after touching different amounts of secret-side state",
                "service endpoints that expose signing failure timing or distinct retry behavior",
            ],
            "bounded_by_current_evidence": [
                "The public error string is sanitized.",
                "The deterministic probe callback ignores the secret reference.",
                "No key-recovery or witness-recovery distinguisher is demonstrated by the measured row alone.",
            ],
        },
        "remediation": {
            "recommended_change": (
                "Move the deployment signer facade to a constant-shape result path: have the "
                "callback return a fixed-size candidate signature plus a success mask or a "
                "fixed public failure code, always run candidate decode and account-binding "
                "checks on a normalized signature buffer, perform fixed cleanup/zeroization, "
                "and select the final sanitized result only after that fixed work."
            ),
            "implementation_notes": [
                "Do not expose raw callback errors at deployment-facing signer boundaries.",
                "Avoid callback-controlled early returns before signature decode and account-binding checks.",
                "Bound and equalize signer retry counts; report exhaustion with the same public failure shape.",
                "Keep detailed signer diagnostics in non-production debug paths outside the constant-time claim.",
                "Audit the real signing callback and sampler separately for secret-dependent branches and memory access.",
            ],
            "proposed_shared_source_patch": {
                "status": "not_applied_by_this_worker",
                "reason": (
                    "Ownership is limited to files whose names contain "
                    "ct_issue_signer_callback_failure. A shared-source patch should be "
                    "reviewed centrally with the other CT workers."
                ),
                "candidate_files": [
                    "tools/parameter-search/src/production_reference.rs",
                    "tools/production-reference/src/lib.rs",
                    "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
                ],
            },
        },
        "test_plan": [
            "Add a ct_issue_signer_callback_failure timing regression that compares callback failure, callback success, and verifier callback failure under a signing-specific baseline, with the policy gate 0.80 <= ratio <= 1.25.",
            "Add structural instrumentation proving the sanitized signer facade always runs candidate signature decode and slot/proof commitment checks for both callback Err and callback Ok.",
            "Add a negative test using a sentinel secret in the callback error detail and assert the public result remains PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON.",
            "Add a signer callback harness with fixed retry budgets and deterministic secret-independent failure injection points.",
            "Run dudect/ctgrind or an equivalent leakage harness over the real secret-side signer/sampler boundary before making a production constant-time claim.",
        ],
        "commands": [
            "rg -n \"deployment_sign_with_secret_key|fault_injection_callback|signer_callback_failure|sign_with_secret|secret_key|signer\" .",
            "nl -ba tools/parameter-search/src/production_reference.rs | sed -n '4570,4625p;4688,4738p;9700,9718p'",
            "nl -ba tools/production-reference/src/lib.rs | sed -n '10,25p;1996,2012p'",
            "nl -ba tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs | sed -n '440,525p;610,660p'",
            "cargo test --manifest-path tools/production-reference/Cargo.toml facade_sign_uses_sanitized_secret_key_boundary -- --nocapture",
            "cargo run --release --manifest-path tools/production-reference/Cargo.toml --bin pre_third_party_side_channel_fault_probe -- --samples 2 --batch-iterations 100",
            "python3 scripts/research/ct_issue_signer_callback_failure.py",
        ],
        "verdict": (
            "CT-012 is a real deployment-facing timing-policy failure at a secret-adjacent "
            "signing facade. The measured row is not itself a demonstrated secret leak, "
            "because the injected callback ignores the secret and always fails, but the "
            "facade's early-return shape would amplify any secret-dependent failure or retry "
            "behavior inside the real signer callback."
        ),
    }


def write_md(report: dict[str, Any]) -> str:
    closure_row = report["measurements"]["closure_row"]
    campaign_row = report["measurements"]["campaign_row"]
    md = f"""# CT-012 Signer Callback Failure Timing

## Summary

`deployment_sign_with_secret_key` has a confirmed signer-callback failure timing cliff in the `fault_injection_callback / signer_callback_failure` row. The checked campaign records ratio `{campaign_row['ratio_to_group_baseline']:.3f}`; the later closure artifact records ratio `{closure_row['ratio_to_group_baseline']:.3f}` with mean `{closure_row['mean_avg_ns']} ns`, below the `0.80..1.25` timing-policy floor.

This is a real deployment-facing constant-time readiness issue. The measured fault row is secret-adjacent rather than proven secret-dependent: the facade passes the reference keypair to the callback, but the probe callback ignores it and always returns `Err`.

## Root Cause

The deployment facade is a thin wrapper over `sign_with_secret_key_sanitized`. That wrapper sanitizes the callback error string to `PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON`, but it preserves the callback-controlled early return. When the callback fails, the path exits before the success-path signature decode, slot-id check, and proof-commitment check.

Evidence:

- `tools/production-reference/src/lib.rs:14-23`: `deployment_sign_with_secret_key` delegates to `sign_with_secret_key_sanitized`.
- `tools/parameter-search/src/production_reference.rs:4577-4599`: `sign_with_account_binding` invokes the callback and only decodes/checks the signature after callback success.
- `tools/parameter-search/src/production_reference.rs:4705-4728`: the sanitized wrappers replace callback details with the fixed public signer callback failure reason.
- `tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs:512-525`: the timing row injects an always-failing signer callback.

## Secret Dependence

Classification: `not demonstrated as secret-dependent by this row`.

The observed branch is controlled by a deterministic injected callback failure. The callback receives secret-adjacent material through `secret_key.reference_keypair()`, but the probe callback ignores it. No sampler rejection count, hidden witness value, or secret-key predicate controls the measured failure.

The risk is still meaningful: if the real signing callback can fail after secret-dependent work, this facade exposes the failure location and retry shape because failure returns before the rest of the fixed signing-boundary work.

## Exploitability

Current evidence does not show key recovery or witness recovery. It does show a timing oracle distinguishing an immediate sanitized signer failure from the longer callback-success/signature-check path. A remote attacker would need to trigger or observe signer callback failures over a low-noise boundary for direct exploitation.

Most relevant exploit surface:

- secret-dependent rejection or retry behavior inside the real signer callback;
- faulted signer integrations that fail after different amounts of secret-side work;
- service endpoints that expose signing failure timing or retry behavior.

## Remediation

Make the deployment signer facade constant-shape:

1. Have the callback return a fixed-size candidate signature plus a success mask or fixed public failure code.
2. Always run candidate decode and account-binding checks on a normalized signature buffer.
3. Bound and equalize signer retry counts.
4. Perform fixed cleanup/zeroization before selecting the final sanitized result.
5. Keep raw callback diagnostics out of deployment-facing signer APIs.

No shared source patch was applied by this worker because the ownership rule limits edits to `ct_issue_signer_callback_failure` artifacts/scripts. Candidate shared files for a central patch are `tools/parameter-search/src/production_reference.rs`, `tools/production-reference/src/lib.rs`, and the side-channel probe.

## Test Plan

Add a focused timing regression named `ct_issue_signer_callback_failure` that compares callback failure, callback success, and verifier callback failure under a signing-specific baseline, requiring `0.80 <= ratio <= 1.25`. Add structural instrumentation proving the sanitized signer facade runs candidate signature decode and account-binding checks for both callback `Err` and callback `Ok`. Keep the existing sentinel-secret error sanitization test, and add a signer callback harness with fixed retry budgets.

## Commands Run

```text
rg -n "deployment_sign_with_secret_key|fault_injection_callback|signer_callback_failure|sign_with_secret|secret_key|signer" .
nl -ba tools/parameter-search/src/production_reference.rs | sed -n '4570,4625p;4688,4738p;9700,9718p'
nl -ba tools/production-reference/src/lib.rs | sed -n '10,25p;1996,2012p'
nl -ba tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs | sed -n '440,525p;610,660p'
cargo test --manifest-path tools/production-reference/Cargo.toml facade_sign_uses_sanitized_secret_key_boundary -- --nocapture
cargo run --release --manifest-path tools/production-reference/Cargo.toml --bin pre_third_party_side_channel_fault_probe -- --samples 2 --batch-iterations 100
python3 scripts/research/ct_issue_signer_callback_failure.py
```

The focused facade test passed. The quick two-sample probe confirmed the sanitized fail-closed outcome; the timing verdict above is anchored to the existing five-sample campaign and closure artifacts.
"""
    return md


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    report = build_report()
    JSON_OUT.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    MD_OUT.write_text(write_md(report))
    print(rel(JSON_OUT))
    print(rel(MD_OUT))


if __name__ == "__main__":
    main()
