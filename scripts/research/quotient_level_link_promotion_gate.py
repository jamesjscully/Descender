#!/usr/bin/env python3
"""Generate the promotion gate for quotient-level account links."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_promotion_gate.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_promotion_gate.md"

ARTIFACTS = [
    "Descender.lean",
    "Descender/DescentInterface.lean",
    "Descender/HiddenCommittedQuotientLink.lean",
    "research/cryptanalysis/signature_readiness_20260516/README.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_research_program.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_kernel_search.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_kernel_search.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_coordinate_pair_sweep.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_coordinate_pair_sweep.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-1-2_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_sparse-difference-link-0m3-1m2_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-1-14_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-8-20_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-14-15_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_current-coordinate-link-0-3_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-1-2_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_sparse-difference-link-0m3-1m2_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-1-14_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-8-20_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-14-15_b96.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hiding_review.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hiding_review.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_account_semantics.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_account_semantics.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_lean_interface_comparison.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hidden_commitment_byte_state_review.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hidden_commitment_byte_state_review.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_existing_commitment_reuse_review.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_existing_commitment_reuse_review.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_implementation_handoff.md",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_firebreak_audit.json",
    "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_firebreak_audit.md",
]

SCRIPTS = [
    "scripts/research/quotient_level_link_kernel_search.py",
    "scripts/research/quotient_level_link_coordinate_pair_sweep.py",
    "scripts/research/quotient_level_link_exact_enum.py",
    "scripts/research/quotient_level_link_sampler_replay.py",
    "scripts/research/quotient_level_link_hiding_review.py",
    "scripts/research/quotient_level_link_account_semantics.py",
    "scripts/research/quotient_level_link_hidden_commitment_byte_state_review.py",
    "scripts/research/quotient_level_link_existing_commitment_reuse_review.py",
    "scripts/research/quotient_level_link_promotion_gate.py",
    "scripts/research/quotient_level_link_firebreak_audit.py",
    "scripts/research/verify_quotient_level_link_closure.py",
    "scripts/research/review_gate_replay.py",
]


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def file_record(rel_path: str) -> dict[str, Any]:
    path = ROOT / rel_path
    return {
        "path": rel_path,
        "exists": path.exists(),
        "sha256": sha256(path) if path.exists() else None,
        "bytes": path.stat().st_size if path.exists() else None,
    }


def load_json(rel_path: str) -> dict[str, Any]:
    return json.loads((ROOT / rel_path).read_text())


def build_payload() -> dict[str, Any]:
    hiding = load_json(
        "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hiding_review.json"
    )
    byte_state = load_json(
        "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hidden_commitment_byte_state_review.json"
    )
    reuse = load_json(
        "research/cryptanalysis/signature_readiness_20260516/quotient_level_link_existing_commitment_reuse_review.json"
    )
    artifact_records = [file_record(path) for path in ARTIFACTS]
    script_records = [file_record(path) for path in SCRIPTS]
    missing = [item["path"] for item in artifact_records + script_records if not item["exists"]]
    return {
        "schema": "Descender.quotient_level_link.promotion_gate.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "final_decision": {
            "classification": "no_replacement",
            "decision": "keep_current_path",
            "migration_plan_required": False,
            "implementation_instruction": (
                "Do not migrate the deployable q12289 verifier, proof path, "
                "byte target, or account-link implementation based on this "
                "research program.  Continue using the current reviewed split "
                "account-carried bounded-kernel design unless a later promotion "
                "gate with independent review supersedes this one."
            ),
            "reason": (
                "Public quotient links fail sampler hiding under public-link "
                "conditioning.  Hidden-commitment links preserve the desired "
                "sampler surface only by adding proof obligations or assumptions "
                "that are not cheaper or simpler than the current 68-byte split "
                "account object.  Existing account commitments cannot supply "
                "the hidden linear link equation for free."
            ),
        },
        "baseline": {
            "current_spend_bytes": 250,
            "current_account_hidden_link_bytes": 68,
            "current_online_plus_account_bytes": 318,
            "current_control_candidate": hiding["summary"]["control_candidate"],
            "current_control_kernel_margin": hiding["summary"]["control_margin"],
        },
        "promotion_matrix": [
            {
                "candidate": "current-reviewed-split-account-link",
                "evidence_complete": True,
                "bytes_saved_vs_current_account": 0,
                "state_changed": False,
                "assumptions_changed": "none",
                "lean_support": "CommittedSigma reviewed account-link path",
                "external_review_status": "current control",
                "recommendation": "keep",
            },
            {
                "candidate": "public-two-row-quotient-link-shortlist",
                "evidence_complete": True,
                "bytes_saved_vs_current_account": 64,
                "state_changed": True,
                "assumptions_changed": "adds public link conditioning; hiding target fails",
                "lean_support": "not migrated",
                "external_review_status": "rejected before review",
                "recommendation": "reject: sampler support floor falls to 106-108 bits",
            },
            {
                "candidate": "hidden-commitment-quotient-link",
                "evidence_complete": True,
                "bytes_saved_vs_current_account": None,
                "state_changed": "depends_on_commitment_choice",
                "assumptions_changed": "adds commitment hiding, binding/extraction, and non-public link-equation proof obligations",
                "lean_support": "Descender.HiddenCommittedQuotientLink sidecar only",
                "external_review_status": "not ready",
                "recommendation": "reject for deployment: no surviving byte or proof simplification",
            },
            {
                "candidate": "reuse-existing-account-commitments-for-link",
                "evidence_complete": True,
                "bytes_saved_vs_current_account": 0,
                "state_changed": False,
                "assumptions_changed": "would require new ZK/hash-circuit proof if link remains hidden",
                "lean_support": "source hooks checked; no free homomorphic equation",
                "external_review_status": "not needed for rejection",
                "recommendation": "reject for deployment",
            },
            {
                "candidate": "pedersen-style-homomorphic-hidden-link",
                "evidence_complete": False,
                "bytes_saved_vs_current_account": "at_most_4_before_proof_bytes",
                "state_changed": True,
                "assumptions_changed": "adds new algebraic commitment assumption and likely proof/opening bytes",
                "lean_support": "interface-compatible in principle only",
                "external_review_status": "would require independent cryptanalytic review",
                "recommendation": "research-only, not promotion-ready",
            },
        ],
        "assumption_deltas": [
            "No cryptanalytic assumption is removed from the deployable path.",
            "Public quotient links add a public-link conditioning surface and fail the 128-bit hiding target.",
            "Hidden commitments add commitment hiding and binding/extraction obligations.",
            "A hidden link equation must be non-public; a public projection of the sigma equation adds no security constraint.",
            "Reuse of byte-hash account commitments would require a new ZK/hash-circuit soundness argument to avoid opening link values.",
        ],
        "replay_commands": [
            "python3 scripts/research/quotient_level_link_kernel_search.py",
            "python3 scripts/research/quotient_level_link_coordinate_pair_sweep.py --top-k 20 --exact-margin-floor 43 --bound 96",
            "python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-1-2 --bound 96",
            "python3 scripts/research/quotient_level_link_exact_enum.py sparse-difference-link-0m3-1m2 --bound 96",
            "python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-1-14 --bound 96",
            "python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-8-20 --bound 96",
            "python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-14-15 --bound 96",
            "python3 scripts/research/quotient_level_link_sampler_replay.py current-coordinate-link-0-3 --bound 96",
            "python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-1-2 --bound 96",
            "python3 scripts/research/quotient_level_link_sampler_replay.py sparse-difference-link-0m3-1m2 --bound 96",
            "python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-1-14 --bound 96",
            "python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-8-20 --bound 96",
            "python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-14-15 --bound 96",
            "python3 scripts/research/quotient_level_link_hiding_review.py",
            "python3 scripts/research/quotient_level_link_account_semantics.py",
            "python3 scripts/research/quotient_level_link_hidden_commitment_byte_state_review.py",
            "python3 scripts/research/quotient_level_link_existing_commitment_reuse_review.py",
            "python3 scripts/research/quotient_level_link_firebreak_audit.py",
            "python3 scripts/research/quotient_level_link_promotion_gate.py",
            "python3 scripts/research/verify_quotient_level_link_closure.py",
            "python3 scripts/research/review_gate_replay.py --dry-run --skip-timing --json /tmp/quotient_review_gate_dry_run.json --md /tmp/quotient_review_gate_dry_run.md",
        ],
        "lean_build_commands": [
            "lake build Descender.DescentInterface",
            "lake build Descender.HiddenCommittedQuotientLink",
        ],
        "lean_theorem_names": [
            "Descender.DescentInterface.EffectiveDescentCertificate.accepts_glues_or_breaks",
            "Descender.CommittedSigma.AccountLinkProofSystem.accepts_accountBacked_of_noBreak",
            "Descender.CommittedSigma.HomomorphicAccountLinkInterface.linkProofAccepts_of_publicProjection",
            "Descender.CommittedSigma.reviewedAccountHiddenLinkBytes_eq",
            "Descender.HiddenCommittedQuotientLink.Interface.toAccountLinkProofSystem",
            "Descender.HiddenCommittedQuotientLink.Interface.accepts_accountBacked_of_noBreaks",
            "Descender.HiddenCommittedQuotientLink.Interface.accepts_glues_or_breaks_of_effectiveDescent",
        ],
        "independent_review_requirements_for_future_reopening": [
            "Concrete commitment/proof system with byte and verifier-cost model.",
            "Sampler replay showing no public-link conditioning below the hiding target.",
            "Exact bounded-kernel certificate for the concrete selected link.",
            "Lean q12289-shaped instantiation of the sidecar theorem shape.",
            "Independent cryptanalytic review of any new commitment or ZK/hash-circuit assumption.",
            "Passing import firebreak audit before any deployable import change.",
            "Review-gate replay dry run includes quotient_level_link_closure_verify.",
        ],
        "upstream_summaries": {
            "hiding_review": hiding["summary"],
            "hidden_commitment_byte_state_review": byte_state["summary"],
            "existing_commitment_reuse_review": reuse["summary"],
        },
        "artifact_hashes": artifact_records,
        "script_hashes": script_records,
        "missing_records": missing,
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["final_decision"]
    baseline = payload["baseline"]
    lines = [
        "# Quotient-Level Link Promotion Gate",
        "",
        "Status: no replacement.  Keep the current reviewed split design.",
        "",
        "This gate does not change the q12289 verifier, proof path, byte target, or implementation.",
        "",
        "## Decision",
        "",
        "| item | value |",
        "|---|---|",
        f"| classification | `{decision['classification']}` |",
        f"| decision | `{decision['decision']}` |",
        f"| migration plan required | `{decision['migration_plan_required']}` |",
        f"| current spend bytes | `{baseline['current_spend_bytes']}` |",
        f"| current account bytes | `{baseline['current_account_hidden_link_bytes']}` |",
        f"| current online plus account bytes | `{baseline['current_online_plus_account_bytes']}` |",
        "",
        decision["reason"],
        "",
        "Implementation instruction: " + decision["implementation_instruction"],
        "",
        "## Promotion Matrix",
        "",
        "| candidate | evidence complete | bytes saved | state changed | assumptions changed | Lean support | review status | recommendation |",
        "|---|---:|---:|---|---|---|---|---|",
    ]
    for row in payload["promotion_matrix"]:
        lines.append(
            f"| {row['candidate']} | `{row['evidence_complete']}` | "
            f"`{row['bytes_saved_vs_current_account']}` | `{row['state_changed']}` | "
            f"{row['assumptions_changed']} | {row['lean_support']} | "
            f"{row['external_review_status']} | {row['recommendation']} |"
        )
    lines.extend(["", "## Assumption Deltas", ""])
    for item in payload["assumption_deltas"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Replay Commands", ""])
    for command in payload["replay_commands"]:
        lines.append(f"- `{command}`")
    lines.extend(["", "## Lean Support", ""])
    lines.append("Build commands:")
    for command in payload["lean_build_commands"]:
        lines.append(f"- `{command}`")
    lines.append("")
    lines.append("Theorem/interface names:")
    for name in payload["lean_theorem_names"]:
        lines.append(f"- `{name}`")
    lines.extend(["", "## Future Reopening Requirements", ""])
    for item in payload["independent_review_requirements_for_future_reopening"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Artifact Hashes", ""])
    lines.extend(["| path | sha256 | bytes |", "|---|---|---:|"])
    for item in payload["artifact_hashes"]:
        lines.append(f"| `{item['path']}` | `{item['sha256']}` | `{item['bytes']}` |")
    lines.extend(["", "## Script Hashes", ""])
    lines.extend(["| path | sha256 | bytes |", "|---|---|---:|"])
    for item in payload["script_hashes"]:
        lines.append(f"| `{item['path']}` | `{item['sha256']}` | `{item['bytes']}` |")
    if payload["missing_records"]:
        lines.extend(["", "## Missing Records", ""])
        for item in payload["missing_records"]:
            lines.append(f"- `{item}`")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "classification": payload["final_decision"]["classification"],
                "decision": payload["final_decision"]["decision"],
                "missing_records": payload["missing_records"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
