#!/usr/bin/env python3
"""Generate the compact hidden-relation proof instantiation decision artifact."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.md"
)
EXTRACTOR_THEOREM_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    theorem = load_json(EXTRACTOR_THEOREM_JSON)
    candidate_instantiations = [
        {
            "id": "digest_only_32_byte_handle",
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "classification": "rejected",
            "reason": "binds context/opening bytes but has no knowledge semantics for witness and pad openings",
            "soundness_gap": "acceptance would not imply extracted hidden-source witness or a named break",
            "zk_leakage_note": "small, but irrelevant because soundness fails",
        },
        {
            "id": "state_assisted_32_byte_random_oracle_proof_handle",
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "classification": "research_track_fail_closed",
            "reason": "could be viable only if the account-state precommitment plus random-oracle proof-handle assumption is stated as a compact proof-of-knowledge oracle",
            "soundness_gap": "requires a new explicit assumption: accepting handle knowledge-extracts witness/pad openings for the committed hidden relation",
            "zk_leakage_note": "best byte profile; leakage is only the handle if the extractor/zero-knowledge oracle assumption is accepted",
        },
        {
            "id": "small_algebraic_opening_carrier",
            "carrier_bytes": "40_to_64",
            "envelope_bytes": "292_to_316",
            "classification": "fallback_candidate",
            "reason": "adds enough public algebraic/check bytes to make witness/pad opening semantics testable without treating a digest handle as a proof",
            "soundness_gap": "still needs exact relation, but no longer relies solely on opaque 32-byte handle semantics",
            "zk_leakage_note": "must account for every added algebraic byte against witness/pad leakage",
        },
        {
            "id": "full_public_opening_carrier",
            "carrier_bytes": "96_plus",
            "envelope_bytes": "348_plus",
            "classification": "control_not_target",
            "reason": "straightforward proof/debug architecture but too large for the compact blockchain target unless all smaller routes fail",
            "soundness_gap": "lowest semantic ambiguity, highest byte cost",
            "zk_leakage_note": "requires strongest leakage audit because more relation material is public",
        },
    ]
    required_assumption_if_32b = {
        "id": "state_assisted_hidden_relation_ro_pok",
        "status": "not_yet_accepted",
        "statement": (
            "For the domain-separated state-assisted hidden-relation frame, any "
            "adversary producing an accepting 32-byte carrier handle for a "
            "prechallenge opening commitment either yields extractor access to "
            "the committed account-witness and pad-seed openings satisfying the "
            "canonical hidden-source relation, or breaks SHA-256/random-oracle "
            "binding, account commitment binding, lifecycle precommitment, or "
            "the checked-core assumptions."
        ),
        "why_nonstandard": "a 32-byte digest-sized object is being used as a proof handle, not as a conventional proof transcript",
        "required_evidence": [
            "byte-level verifier contract",
            "honest acceptance test vector",
            "semantic negative-control rejection tests",
            "extractor-or-break theorem",
            "zero-knowledge/leakage theorem for the opaque handle",
            "external review of the compact proof-handle assumption",
        ],
    }
    decision_gates = [
        {
            "gate": "can_state_assisted_handle_be_a_named_assumption",
            "current": "open",
            "pass_condition": "the RO proof-handle assumption is explicit, minimal, falsifiable by negative controls, and reviewable by a third party",
            "fail_action": "activate small_algebraic_opening_carrier",
        },
        {
            "gate": "can_negative_controls_be_executable",
            "current": "open",
            "pass_condition": "witness substitution, pad substitution, context replay, and false-statement controls become byte-level tests",
            "fail_action": "increase carrier bytes until controls are executable",
        },
        {
            "gate": "can_zero_knowledge_budget_remain_small",
            "current": "open",
            "pass_condition": "carrier leaks no account witness, pad seed, lifecycle secret, or signer randomness beyond the named 32-byte handle",
            "fail_action": "choose the smallest algebraic carrier whose leakage theorem closes",
        },
        {
            "gate": "can_global_theorem_compose",
            "current": "open",
            "pass_condition": "the proof-handle theorem composes with checked-core, response-link, QROM, pad, keygen, and state lifecycle assumptions",
            "fail_action": "do not promote accepting verifier",
        },
    ]
    implementation_tasks = [
        {
            "task": "define_ro_pok_verifier_api",
            "output": "Rust trait/callback that takes only canonical public statement bytes plus 32 carrier bytes and returns fail-closed acceptance",
        },
        {
            "task": "add_byte_level_negative_tests",
            "output": "tests for each semantic control using serialized frames, not only structured objects",
        },
        {
            "task": "write_leakage_budget",
            "output": "explicit accounting for carrier bytes, statement fields, pad-opening residues, slot id, anchor, and transcript digest",
        },
        {
            "task": "fallback_byte_model",
            "output": "byte-cost table for 40, 48, 56, and 64 byte algebraic carriers if the 32-byte assumption is too strong",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_compact_proof_instantiation.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "compact_proof_instantiation_decision_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "extractor_theorem": rel(EXTRACTOR_THEOREM_JSON),
            "extractor_theorem_schema": theorem.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "current_carrier_bytes": 32,
            "current_envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "candidate_instantiations": candidate_instantiations,
        "required_assumption_if_32b": required_assumption_if_32b,
        "decision_gates": decision_gates,
        "implementation_tasks": implementation_tasks,
        "decision": {
            "can_accept_now": False,
            "selected_next_track": "state_assisted_32_byte_random_oracle_proof_handle",
            "fallback_track": "small_algebraic_opening_carrier",
            "reason": "the 32-byte path remains the byte-minimal research target, but it requires an explicit proof-handle assumption and executable semantic controls before any accepting verifier can be promoted",
            "next_artifact": "state_assisted_ro_pok_assumption_and_verifier_api",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    assumption = payload["required_assumption_if_32b"]
    lines = [
        "# Hidden-Source State-Assisted Compact Proof Instantiation",
        "",
        "This artifact decides what the compact hidden-relation proof object is",
        "allowed to mean.  It remains fail-closed: the 32-byte route stays as the",
        "byte-minimal research track only if its proof-handle assumption is made",
        "explicit and survives executable negative controls and review.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- current carrier bytes: `{target['current_carrier_bytes']}`",
        f"- current envelope bytes: `{target['current_envelope_bytes']}`",
        f"- extractor theorem: `{payload['source_artifacts']['extractor_theorem']}`",
        "",
        "## Candidate Instantiations",
        "",
        "| id | carrier bytes | envelope bytes | classification | reason |",
        "|---|---:|---:|---|---|",
    ]
    for row in payload["candidate_instantiations"]:
        lines.append(
            f"| `{row['id']}` | `{row['carrier_bytes']}` | `{row['envelope_bytes']}` | "
            f"`{row['classification']}` | {row['reason']} |"
        )
    lines.extend(
        [
            "",
            "## Required 32-Byte Assumption",
            "",
            f"- id: `{assumption['id']}`",
            f"- status: `{assumption['status']}`",
            f"- why nonstandard: {assumption['why_nonstandard']}",
            "",
            assumption["statement"],
            "",
            "Required evidence:",
        ]
    )
    lines.extend(f"- {item}" for item in assumption["required_evidence"])
    lines.extend(["", "## Decision Gates", "", "| gate | current | pass condition | fail action |", "|---|---|---|---|"])
    for row in payload["decision_gates"]:
        lines.append(
            f"| `{row['gate']}` | `{row['current']}` | {row['pass_condition']} | {row['fail_action']} |"
        )
    lines.extend(["", "## Implementation Tasks", "", "| task | output |", "|---|---|"])
    for row in payload["implementation_tasks"]:
        lines.append(f"| `{row['task']}` | {row['output']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- selected next track: `{payload['decision']['selected_next_track']}`",
            f"- fallback track: `{payload['decision']['fallback_track']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
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
