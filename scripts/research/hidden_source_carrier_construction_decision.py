#!/usr/bin/env python3
"""Decide the next hidden-source carrier construction target."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.md"
RELATION_JSON = READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.json"
TARGET_JSON = READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    relation = load_json(RELATION_JSON)
    target = load_json(TARGET_JSON)
    candidates = [
        {
            "id": "opaque_32_byte_digest_only",
            "transaction_bytes": 284,
            "classification": "reject_as_standalone_proof",
            "soundness_posture": "a 32-byte digest can bind a transcript but does not by itself prove the hidden witness relation",
            "zero_knowledge_posture": "excellent byte size, but no knowledge semantics unless backed by precommitted state or an external proof system",
            "implementation_delta": "none for bytes; impossible to turn on acceptance without a new theorem",
            "decision": "do not promote as a standalone accepting verifier",
        },
        {
            "id": "state_assisted_32_byte_carrier",
            "transaction_bytes": 284,
            "classification": "preferred_research_track",
            "soundness_posture": "plausible only if account-state precommitments give the carrier force as a challenge-bound nullifier/opening handle",
            "zero_knowledge_posture": "keeps spend bytes small, but shifts leakage and trust analysis into account-state lifecycle",
            "implementation_delta": "define account-state creation semantics, precommitment digest frame, slot consumption, and restore-anchor rules",
            "decision": "pursue first, because it is the only route that can plausibly keep the selected 284 B envelope",
        },
        {
            "id": "small_algebraic_opening_carrier",
            "transaction_bytes": "292_to_348_estimate",
            "classification": "fallback_if_state_assisted_track_fails",
            "soundness_posture": "more plausible direct knowledge proof because the carrier can include q-ary response/opening material",
            "zero_knowledge_posture": "requires explicit leakage accounting for every revealed residue or compressed opening",
            "implementation_delta": "choose q-ary opening dimensions, pack residues, update transcript, and rerun byte/security budget",
            "decision": "keep as fallback and record the minimal byte increase if 32-byte state assistance cannot prove soundness",
        },
        {
            "id": "delayed_reveal_or_pay_to_hash",
            "transaction_bytes": "small_immediate_spend_plus_resolution_payload",
            "classification": "separate_protocol",
            "soundness_posture": "can be sound if finality rules require later reveal/challenge resolution",
            "zero_knowledge_posture": "depends on reveal payload; may disclose witness-adjacent material after spend",
            "implementation_delta": "requires chain protocol, availability, timeout, and fraud/dispute semantics",
            "decision": "do not mix into the selected immediate-signature theorem",
        },
    ]
    selected = candidates[1]
    required_modeling_work = [
        {
            "id": "state_creation_relation",
            "purpose": "define exactly what a user commits before any signing challenge is known",
            "output": "account-state precommitment relation and digest frame",
        },
        {
            "id": "carrier_as_handle",
            "purpose": "define how 32 carrier bytes select or authenticate the precommitted hidden-source opening without revealing it",
            "output": "carrier verification predicate over account state and canonical transcript",
        },
        {
            "id": "single_use_enforcement",
            "purpose": "make replay and rollback invalid in the same theorem as signature acceptance",
            "output": "slot nullifier/spent-ledger/restore-anchor rule",
        },
        {
            "id": "extractor_or_break",
            "purpose": "prove accepting state-assisted carrier yields hidden witness/pad source or a named commitment/hash/QROM/state break",
            "output": "knowledge-soundness theorem statement",
        },
        {
            "id": "simulator_or_leakage_bound",
            "purpose": "show carrier plus public residues do not leak unaccounted account witness, pad seed, or signer randomness",
            "output": "zero-knowledge or explicit leakage theorem",
        },
    ]
    decision_gates = [
        {
            "gate": "no_digest_only_acceptance",
            "condition": "32 bytes may bind context but must not be treated as a proof without state-assisted semantics",
            "status": "closed_rule",
        },
        {
            "gate": "state_assisted_theorem_required",
            "condition": "the selected 284 B branch needs a theorem coupling state creation, carrier verification, and spend consumption",
            "status": "open",
        },
        {
            "gate": "fallback_byte_increase",
            "condition": "if state-assisted semantics cannot produce extraction, move to a small algebraic carrier and update bytes",
            "status": "open",
        },
        {
            "gate": "separate_delayed_protocol",
            "condition": "delayed reveal/pay-to-hash remains a different protocol and cannot close the immediate-signature theorem",
            "status": "closed_rule",
        },
    ]
    next_steps = [
        "write the state-assisted 32-byte carrier relation as a byte-level account-state theorem target",
        "define the account-state precommitment digest frame and carrier-as-handle predicate",
        "add negative controls for digest-only acceptance, stale account state, replayed carrier, and rollback",
        "try to prove extractor-or-break for the state-assisted relation",
        "if extraction fails, instantiate the smallest algebraic carrier candidate and update byte budgets",
    ]
    return {
        "schema": "Descender.hidden_source_carrier_construction_decision.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "state_assisted_32_byte_carrier_selected_for_research",
        "deployment_ready": False,
        "source_artifacts": {
            "semantics_target": rel(TARGET_JSON),
            "accepting_relation": rel(RELATION_JSON),
            "semantics_target_schema": target.get("schema"),
            "accepting_relation_schema": relation.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "selected_carrier_bytes": 32,
            "selected_envelope_bytes": 284,
            "fallback_envelope_bytes_estimate": "292_to_348",
        },
        "selected_candidate": selected["id"],
        "candidate_evaluations": candidates,
        "required_modeling_work": required_modeling_work,
        "decision_gates": decision_gates,
        "next_steps": next_steps,
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source Carrier Construction Decision",
        "",
        "This artifact decides the next carrier construction target for the exact",
        "hidden-source accepting relation. It does not make the deployment verifier",
        "accept; it chooses the next proof direction and records fallback gates.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{payload['selected_candidate']}`",
        f"- n: `{target['n']}`",
        f"- q: `{target['q']}`",
        f"- checked core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- selected carrier bytes: `{target['selected_carrier_bytes']}`",
        f"- selected envelope bytes: `{target['selected_envelope_bytes']}`",
        f"- fallback envelope bytes estimate: `{target['fallback_envelope_bytes_estimate']}`",
        f"- accepting relation: `{payload['source_artifacts']['accepting_relation']}`",
        "",
        "## Candidate Evaluations",
        "",
        "| candidate | bytes | classification | decision | soundness posture |",
        "|---|---:|---|---|---|",
    ]
    for item in payload["candidate_evaluations"]:
        lines.append(
            f"| `{item['id']}` | `{item['transaction_bytes']}` | `{item['classification']}` | "
            f"{item['decision']} | {item['soundness_posture']} |"
        )
    lines.extend(
        [
            "",
            "## Required Modeling Work",
            "",
            "| item | purpose | output |",
            "|---|---|---|",
        ]
    )
    for item in payload["required_modeling_work"]:
        lines.append(f"| `{item['id']}` | {item['purpose']} | {item['output']} |")
    lines.extend(
        [
            "",
            "## Decision Gates",
            "",
            "| gate | status | condition |",
            "|---|---|---|",
        ]
    )
    for item in payload["decision_gates"]:
        lines.append(f"| `{item['gate']}` | `{item['status']}` | {item['condition']} |")
    lines.extend(["", "## Next Steps", ""])
    lines.extend(f"{index}. {step}" for index, step in enumerate(payload["next_steps"], 1))
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
