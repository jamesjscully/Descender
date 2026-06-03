#!/usr/bin/env python3
"""Generate CT-002 slot-only signature timing issue artifacts."""

from __future__ import annotations

import argparse
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
OUT_DIR = ROOT / "research" / "cryptanalysis" / "signature_readiness_20260516"
JSON_OUT = OUT_DIR / "ct_issue_slot_only_signature.json"
MD_OUT = OUT_DIR / "ct_issue_slot_only_signature.md"
PROBE_MANIFEST = ROOT / "tools" / "production-reference" / "Cargo.toml"
PROBE_SOURCE = (
    ROOT
    / "tools"
    / "production-reference"
    / "src"
    / "bin"
    / "ct_issue_slot_only_signature_probe.rs"
)
DECODE_SOURCE = ROOT / "tools" / "parameter-search" / "src" / "production_reference.rs"


def run_probe(samples: int, batch_iterations: int) -> dict:
    result = subprocess.run(
        [
            "cargo",
            "run",
            "--manifest-path",
            str(PROBE_MANIFEST),
            "--release",
            "--bin",
            "ct_issue_slot_only_signature_probe",
            "--",
            "--samples",
            str(samples),
            "--batch-iterations",
            str(batch_iterations),
        ],
        cwd=ROOT,
        check=True,
        text=True,
        capture_output=True,
    )
    return json.loads(result.stdout)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def row_by_class(probe: dict, name: str) -> dict:
    return next(row for row in probe["rows"] if row["class"] == name)


def ratio_by_num(probe: dict, name: str) -> dict:
    return next(row for row in probe["ratios"] if row["numerator_class"] == name)


def build_report(probe: dict) -> dict:
    direct_ratio = ratio_by_num(probe, "slot_only_signature")
    equalized_ratio = ratio_by_num(probe, "slot_only_signature_equalized")
    valid = row_by_class(probe, "valid_linear_sigma_signature")
    slot_only = row_by_class(probe, "slot_only_signature")

    return {
        "schema": "Descender.ct_issue_slot_only_signature.v1",
        "issue": {
            "id": "CT-002",
            "group": "parse_malformed_input",
            "class": "slot_only_signature",
            "operation": "ProductionSignature::decode",
            "reported_timing_ratio": 0.351,
            "observed_direct_mean_ratio": direct_ratio["mean_ratio"],
            "observed_direct_median_ratio": direct_ratio["median_ratio"],
            "observed_equalized_mean_ratio": equalized_ratio["mean_ratio"],
            "observed_equalized_median_ratio": equalized_ratio["median_ratio"],
            "status": "confirmed_public_malformed_input_timing_cliff",
        },
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "inputs": {
            "probe_source": rel(PROBE_SOURCE),
            "decode_source": rel(DECODE_SOURCE),
            "probe_manifest": rel(PROBE_MANIFEST),
            "probe_command": (
                "cargo run --manifest-path tools/production-reference/Cargo.toml "
                "--release --bin ct_issue_slot_only_signature_probe -- "
                f"--samples {probe['samples']} --batch-iterations {probe['batch_iterations']}"
            ),
        },
        "root_cause": {
            "summary": (
                "The slot-only signature reaches only the top-level length guard in "
                "decode_production_signature. Because len == SLOT_ID_BYTES and the expected "
                "linear-sigma transaction length is 254 bytes, the routine returns before "
                "reading slot_id or decoding challenge, responses, openings, or proof."
            ),
            "decode_path": [
                "ProductionSignature::decode calls decode_production_signature(self.as_bytes()).",
                "decode_production_signature first checks bytes.len() != LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES.",
                "For a 4-byte slot-only signature the function immediately returns an Err with the observed byte length.",
                "The valid baseline continues through slot_id extraction, decode_challenge, decode_responses, decode_openings, and decode_linear_sigma_target_proof.",
            ],
            "primary_source_location": "tools/parameter-search/src/production_reference.rs:3155",
            "length_guard_location": "tools/parameter-search/src/production_reference.rs:7305",
            "probe_interpretation": (
                "The direct ratio is expected to be far below 1.0 because the malformed input "
                "does less public parsing work. Equalized diagnostic rows intentionally pad "
                "to a fixed-size buffer and perform a canonical decode-shaped pass, showing "
                "the class of mitigation needed if uniform parser timing is required."
            ),
        },
        "secret_dependence": {
            "classification": "public_input_length_dependent_not_secret_dependent_under_current_boundary",
            "reasoning": [
                "The branch predicate is signature byte length, which is attacker-controlled public input.",
                "No secret key, hidden witness, pad material, sampler state, or account randomizer preimage is read before this rejection.",
                "The error string includes public length metadata only.",
            ],
            "residual_risk": (
                "The timing cliff can fingerprint parser classes and can act as a remote "
                "rejection oracle if exposed over a low-noise service. It is not, by itself, "
                "evidence of hidden-witness or signing-key leakage."
            ),
        },
        "exploitability": {
            "secret_extraction": "low_not_indicated",
            "remote_fingerprint_or_rejection_oracle": "possible_for_public_malformed_inputs",
            "deployment_relevance": (
                "Treat as a hardening and policy issue for public parsers. It should block a "
                "blanket constant-time parser claim, but it need not block a narrower claim "
                "that public malformed-input parsing is outside the secret-dependent leakage model."
            ),
        },
        "remediation": {
            "recommended_policy": (
                "Decide explicitly whether public malformed-input parser timing is in scope. "
                "If it is out of scope, encode that exception in the CT policy and keep fail-closed tests. "
                "If it is in scope, replace the early-return parser shape with a constant-shape decode."
            ),
            "concrete_patch_plan": [
                "Add a non-allocating, fixed-shape decode wrapper for deployment-facing parsing.",
                "Copy input into a fixed 254-byte buffer with a public copy length and accumulate a length_ok mask.",
                "Run challenge, response, opening, and proof syntactic decoders or equivalent constant-work checks over the normalized buffer.",
                "Combine length_ok and field canonicality masks and return one sanitized malformed-signature error after all public parse checks complete.",
                "Avoid formatting variable-length detailed errors on timing-sensitive deployment paths; keep detailed errors behind diagnostics.",
            ],
            "test_plan": [
                "Keep the fail-closed unit test for slot-only signatures.",
                "Add a CT regression row comparing slot_only_signature against valid_linear_sigma_signature for the deployment parser if public parser timing is in scope.",
                "Run a leakage harness such as dudect or ctgrind on the deployment parser and separately on secret-side signer/sampler code before making a production constant-time claim.",
                "If parser timing is policy-exempt, add a test that asserts the row is classified public_input_length_dependent rather than secret_dependent.",
            ],
        },
        "probe": probe,
        "probe_summary": {
            "valid_mean_ns": valid["mean_avg_ns"],
            "slot_only_mean_ns": slot_only["mean_avg_ns"],
            "direct_mean_ratio": direct_ratio["mean_ratio"],
            "equalized_mean_ratio": equalized_ratio["mean_ratio"],
        },
    }


def write_markdown(report: dict) -> None:
    issue = report["issue"]
    probe = report["probe_summary"]
    root = report["root_cause"]
    secret = report["secret_dependence"]
    exploit = report["exploitability"]
    remediation = report["remediation"]

    lines = [
        "# CT-002 Slot-Only Signature Decode Timing",
        "",
        f"- group: `{issue['group']}`",
        f"- class: `{issue['class']}`",
        f"- operation: `{issue['operation']}`",
        f"- reported ratio: `{issue['reported_timing_ratio']:.3f}`",
        f"- observed direct mean ratio: `{issue['observed_direct_mean_ratio']:.3f}`",
        f"- observed equalized diagnostic mean ratio: `{issue['observed_equalized_mean_ratio']:.3f}`",
        f"- status: `{issue['status']}`",
        "",
        "## Finding",
        "",
        root["summary"],
        "",
        "| row | mean ns | ratio |",
        "|---|---:|---:|",
        f"| valid baseline | `{probe['valid_mean_ns']:.1f}` | `1.000` |",
        f"| slot only | `{probe['slot_only_mean_ns']:.1f}` | `{probe['direct_mean_ratio']:.3f}` |",
        f"| slot only, equalized diagnostic | n/a | `{probe['equalized_mean_ratio']:.3f}` |",
        "",
        "## Decode Path",
        "",
    ]
    lines.extend(f"- {item}" for item in root["decode_path"])
    lines.extend(
        [
            "",
            f"Primary source: `{root['primary_source_location']}`.",
            f"Length guard: `{root['length_guard_location']}`.",
            "",
            "## Secret Dependence And Exploitability",
            "",
            f"Classification: `{secret['classification']}`.",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in secret["reasoning"])
    lines.extend(
        [
            "",
            f"Residual risk: {secret['residual_risk']}",
            "",
            f"Secret extraction: `{exploit['secret_extraction']}`.",
            f"Remote fingerprint/rejection oracle: `{exploit['remote_fingerprint_or_rejection_oracle']}`.",
            "",
            "## Remediation",
            "",
            remediation["recommended_policy"],
            "",
        ]
    )
    lines.extend(f"- {item}" for item in remediation["concrete_patch_plan"])
    lines.extend(["", "## Test Plan", ""])
    lines.extend(f"- {item}" for item in remediation["test_plan"])
    lines.extend(
        [
            "",
            "## Reproduction",
            "",
            "```bash",
            report["inputs"]["probe_command"],
            f"python3 {rel(Path(__file__).resolve())}",
            "```",
            "",
            f"JSON artifact: `{rel(JSON_OUT)}`.",
        ]
    )
    MD_OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--samples", type=int, default=25)
    parser.add_argument("--batch-iterations", type=int, default=10_000)
    args = parser.parse_args()

    probe = run_probe(args.samples, args.batch_iterations)
    report = build_report(probe)
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    JSON_OUT.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    write_markdown(report)
    print(JSON_OUT)
    print(MD_OUT)


if __name__ == "__main__":
    main()
