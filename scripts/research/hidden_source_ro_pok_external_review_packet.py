#!/usr/bin/env python3
"""Generate the external-review packet for the 32-byte hidden-source RO-PoK route."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.md"
RO_POK_API_JSON = READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
RO_POK_NEGATIVE_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
)
DECISION_GATE_JSON = READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    api = load_json(RO_POK_API_JSON)
    negative = load_json(RO_POK_NEGATIVE_JSON)
    gate = load_json(DECISION_GATE_JSON)
    review_questions = [
        {
            "id": "assumption_plausibility",
            "question": "Is the state-assisted hidden-relation RO-PoK assumption independently meaningful, falsifiable, and no stronger than the compact signature claim it supports?",
            "acceptance_standard": "Reviewer can state a precise adversarial game whose winning condition implies extraction or a named primitive break.",
            "reject_if": "The 32-byte handle is merely a restatement that invalid signatures are hard to find.",
        },
        {
            "id": "extractor_interface",
            "question": "Does an accepting transcript expose enough public structure for a black-box or algebraic extractor to recover a hidden witness/pad source under the named assumption?",
            "acceptance_standard": "Extractor inputs are exactly byte-level public transcript fields and account state; no secret verifier advice is introduced.",
            "reject_if": "Extraction requires signer-only data, private verifier state, or an unmodeled distributional promise.",
        },
        {
            "id": "byte_binding",
            "question": "Do the domain separators, transcript frame, and 32-byte proof handle bind every source of replay, malleability, and cross-protocol ambiguity?",
            "acceptance_standard": "All mutable bytes are charged to an explicit hash input and covered by negative tests.",
            "reject_if": "Any acceptance-relevant field can be swapped without changing the RO-PoK digest.",
        },
        {
            "id": "leakage_budget",
            "question": "Can the proof handle be treated as a bounded public leakage channel without revealing the account witness, pad seed, per-slot state, or signer randomness?",
            "acceptance_standard": "Leakage theorem accounts for the handle, account-state reads, diagnostics, rejection behavior, and repeated use.",
            "reject_if": "The handle or rejection path leaks witness-correlated predicates not present in the public statement.",
        },
        {
            "id": "fallback_trigger",
            "question": "If the 32-byte route is rejected, is the rejection reason actionable by moving to the 56-byte public projection fallback?",
            "acceptance_standard": "Reviewer classifies failures as assumption-only, control-only, leakage-only, or global-composition failures.",
            "reject_if": "The failure also invalidates the fallback relation or the checked-core response-link theorem.",
        },
    ]
    required_evidence = [
        {
            "id": "api_contract",
            "artifact": rel(RO_POK_API_JSON),
            "status": api.get("status"),
            "review_use": "Defines the exact assumption boundary, verifier inputs, forbidden inputs, acceptance rules, and review questions.",
        },
        {
            "id": "negative_controls",
            "artifact": rel(RO_POK_NEGATIVE_JSON),
            "status": negative.get("status"),
            "review_use": "Shows which byte mutations must fail and records the current fail-closed Rust boundary.",
        },
        {
            "id": "route_decision_gate",
            "artifact": rel(DECISION_GATE_JSON),
            "status": gate.get("status"),
            "review_use": "Fixes when to keep the 284-byte route, switch to 308 bytes, or reject both.",
        },
    ]
    red_flags = [
        "assumption cannot be separated from unforgeability of the whole scheme",
        "extractor needs a secret-independent but unproved accepting-density claim",
        "proof handle verification cannot be made executable without hidden verifier advice",
        "byte-level transcript binding omits account state, slot id, public-key digest, message digest, or pad-opening statement digest",
        "leakage proof requires treating rejection behavior as secret-independent without a constant-time implementation argument",
    ]
    return {
        "schema": "Descender.hidden_source_ro_pok_external_review_packet.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "ro_pok_external_review_packet_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `284 B/Fq12` account-carried hidden-source envelope",
            "carrier_bytes": 32,
            "fallback_branch": "fail-closed `308 B/Fq12` algebraic projection fallback",
        },
        "review_position": {
            "claim": "The 32-byte RO-PoK handle is acceptable only as an explicit, externally reviewed hidden-source proof-of-knowledge assumption.",
            "not_claimed": [
                "The current verifier is accepting.",
                "The assumption is standard.",
                "The proof package is deployable.",
                "The fallback is selected.",
            ],
            "default_action": "keep_verifier_fail_closed",
        },
        "required_evidence": required_evidence,
        "review_questions": review_questions,
        "red_flags": red_flags,
        "reviewer_outputs": [
            "accept_284b_assumption_with_conditions",
            "reject_284b_assumption_activate_308b_projection_certificate",
            "reject_both_routes",
        ],
        "decision": {
            "can_accept_now": False,
            "external_review_required": True,
            "selected_now": "32_byte_ro_pok_fail_closed",
            "fallback_selected_now": False,
            "next_artifact_if_rejected": "hidden_source_fallback_projection_exact_certificate",
            "next_artifact_if_accepted": "hidden_source_ro_pok_leakage_and_global_composition_theorem",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Hidden-Source RO-PoK External Review Packet",
        "",
        "This packet isolates the review question for the 32-byte hidden-source",
        "RO-PoK route.  It does not make the verifier accepting; it records what",
        "external review must accept before the fail-closed `284 B/Fq12`",
        "account-carried hidden-source envelope can remain the lead route.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {payload['target']['selected_branch']}",
        f"- fallback branch: {payload['target']['fallback_branch']}",
        f"- default action: `{payload['review_position']['default_action']}`",
        "",
        "## Required Evidence",
        "",
        "| id | status | artifact | review use |",
        "|---|---|---|---|",
    ]
    for row in payload["required_evidence"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['artifact']}` | {row['review_use']} |"
        )
    lines.extend(
        [
            "",
            "## Review Questions",
            "",
            "| id | question | acceptance standard | reject if |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["review_questions"]:
        lines.append(
            f"| `{row['id']}` | {row['question']} | {row['acceptance_standard']} | {row['reject_if']} |"
        )
    lines.extend(["", "## Red Flags", ""])
    lines.extend(f"- {item}" for item in payload["red_flags"])
    lines.extend(["", "## Reviewer Outputs", ""])
    lines.extend(f"- `{item}`" for item in payload["reviewer_outputs"])
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- external review required: `{payload['decision']['external_review_required']}`",
            f"- selected now: `{payload['decision']['selected_now']}`",
            f"- fallback selected now: `{payload['decision']['fallback_selected_now']}`",
            f"- next if rejected: `{payload['decision']['next_artifact_if_rejected']}`",
            f"- next if accepted: `{payload['decision']['next_artifact_if_accepted']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
