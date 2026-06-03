#!/usr/bin/env python3
"""Generate the state-assisted RO proof-handle assumption and verifier API."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.md"
)
COMPACT_PROOF_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    compact = load_json(COMPACT_PROOF_JSON)
    assumption = {
        "id": "state_assisted_hidden_relation_ro_pok",
        "status": "explicit_unaccepted_assumption",
        "model": "random_oracle_proof_of_knowledge_handle",
        "statement": (
            "Given a canonical state-assisted hidden-relation public statement, "
            "a prechallenge hidden_source_opening_commitment stored in account "
            "state, and a 32-byte carrier handle, any accepting verifier result "
            "either has an extractor that obtains the committed account-witness "
            "opening, pad-seed opening, carrier-handle opening, and lifecycle "
            "witness satisfying the canonical hidden-source relation, or yields "
            "one of the named binding, lifecycle, pad, checked-core, QROM, or "
            "keygen breaks recorded by the extractor theorem."
        ),
        "explicit_nonclaims": [
            "SHA-256 alone is not claimed to prove knowledge",
            "digest equality alone is not an accepting proof",
            "the current Rust deployment verifier remains fail-closed",
            "this assumption is not deployable without executable controls and external review",
        ],
    }
    verifier_api = {
        "rust_trait_name": "StateAssistedHiddenRelationRoPokVerifier",
        "function": "verify_state_assisted_hidden_relation_ro_pok",
        "inputs": [
            {
                "name": "statement_frame",
                "type": "&[u8]",
                "source": "canonical verifier-owned hidden-relation public statement frame",
                "secret": False,
            },
            {
                "name": "carrier",
                "type": "&[u8; 32]",
                "source": "strict 32-byte proof handle from the 284-byte envelope",
                "secret": False,
            },
            {
                "name": "opening_commitment",
                "type": "&[u8; 32]",
                "source": "prechallenge hidden_source_opening_commitment from account state",
                "secret": False,
            },
            {
                "name": "transcript_digest",
                "type": "&[u8; 32]",
                "source": "canonical hidden-source proof transcript digest",
                "secret": False,
            },
        ],
        "outputs": [
            {
                "name": "accept",
                "type": "bool",
                "meaning": "true only under the explicit RO proof-handle assumption",
            },
            {
                "name": "public_error",
                "type": "fixed enum/string",
                "meaning": "fixed public failure reason with no secret-dependent diagnostics",
            },
        ],
        "forbidden_inputs": [
            "account witness syndrome",
            "pad seed",
            "carrier handle secret",
            "opening preimage",
            "secret-key material",
            "debug witness traces",
        ],
    }
    transcript_frame = [
        {
            "field": "domain",
            "encoding": "ASCII domain separator",
            "value": "Descender.hidden_source.state_assisted.ro_pok.v1",
        },
        {
            "field": "profile",
            "encoding": "ASCII profile",
            "value": "n32.q12289.fq12.state.ro_pok.v1",
        },
        {
            "field": "statement_frame_len",
            "encoding": "u32 little-endian",
            "value": "canonical statement frame length",
        },
        {
            "field": "statement_frame",
            "encoding": "raw bytes",
            "value": "canonical verifier-owned public statement",
        },
        {
            "field": "opening_commitment",
            "encoding": "32 bytes",
            "value": "prechallenge account-state opening commitment",
        },
        {
            "field": "transcript_digest",
            "encoding": "32 bytes",
            "value": "reviewed hidden-source proof transcript digest",
        },
        {
            "field": "carrier",
            "encoding": "32 bytes",
            "value": "proof handle",
        },
    ]
    acceptance_rules = [
        {
            "rule": "strict_public_input_only",
            "requirement": "the verifier API receives no hidden witness or pad material",
        },
        {
            "rule": "canonical_frame_recompute",
            "requirement": "statement_frame and transcript_digest must be recomputed by the caller from canonical bytes, not cached attacker input",
        },
        {
            "rule": "fixed_error_surface",
            "requirement": "all rejection paths map to one public failure reason",
        },
        {
            "rule": "fail_closed_default",
            "requirement": "default implementation returns rejection until an audited verifier is supplied",
        },
        {
            "rule": "no_digest_only_acceptance",
            "requirement": "matching SHA-256 frames is necessary for binding checks but not sufficient for accepting proof semantics",
        },
    ]
    review_questions = [
        {
            "question": "Is the RO proof-handle assumption materially stronger than standard hash/RO assumptions?",
            "expected_answer": "yes; it must be treated as a new, explicit assumption unless instantiated by a real proof system",
            "deployment_impact": "blocks deployment without third-party review or fallback",
        },
        {
            "question": "Can semantic negative controls be executed without witness access at verify time?",
            "expected_answer": "only if a concrete verifier or oracle model is provided",
            "deployment_impact": "failure activates the larger algebraic carrier fallback",
        },
        {
            "question": "Does the API leak secret-dependent verifier diagnostics?",
            "expected_answer": "no; fixed public errors are mandatory",
            "deployment_impact": "any diagnostic leakage keeps verifier fail-closed",
        },
        {
            "question": "Can the assumption compose with checked-core and QROM budgets?",
            "expected_answer": "open until the global accepting-verifier theorem is written",
            "deployment_impact": "blocks promotion",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_ro_pok_assumption_api.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "ro_pok_assumption_api_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "compact_proof_instantiation": rel(COMPACT_PROOF_JSON),
            "compact_proof_instantiation_schema": compact.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_random_oracle_proof_handle",
        },
        "assumption": assumption,
        "verifier_api": verifier_api,
        "transcript_frame": transcript_frame,
        "acceptance_rules": acceptance_rules,
        "review_questions": review_questions,
        "decision": {
            "can_accept_now": False,
            "reason": "the 32-byte RO proof-handle assumption and verifier API are explicit, but the assumption remains unaccepted and lacks executable semantic controls",
            "next_artifact": "state_assisted_ro_pok_byte_level_negative_tests",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    assumption = payload["assumption"]
    api = payload["verifier_api"]
    lines = [
        "# Hidden-Source State-Assisted RO-PoK Assumption API",
        "",
        "This artifact makes the 32-byte random-oracle proof-handle route explicit.",
        "It is not a deployment claim.  It defines the assumption and the verifier",
        "API shape that must be reviewed before the fail-closed verifier can become",
        "accepting.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- compact proof instantiation: `{payload['source_artifacts']['compact_proof_instantiation']}`",
        "",
        "## Assumption",
        "",
        f"- id: `{assumption['id']}`",
        f"- status: `{assumption['status']}`",
        f"- model: `{assumption['model']}`",
        "",
        assumption["statement"],
        "",
        "Explicit nonclaims:",
    ]
    lines.extend(f"- {item}" for item in assumption["explicit_nonclaims"])
    lines.extend(
        [
            "",
            "## Verifier API",
            "",
            f"- Rust trait name: `{api['rust_trait_name']}`",
            f"- function: `{api['function']}`",
            "",
            "| input | type | source | secret |",
            "|---|---|---|:---:|",
        ]
    )
    for row in api["inputs"]:
        lines.append(
            f"| `{row['name']}` | `{row['type']}` | {row['source']} | `{row['secret']}` |"
        )
    lines.extend(["", "Outputs:", "", "| output | type | meaning |", "|---|---|---|"])
    for row in api["outputs"]:
        lines.append(f"| `{row['name']}` | `{row['type']}` | {row['meaning']} |")
    lines.extend(["", "Forbidden inputs:"])
    lines.extend(f"- {item}" for item in api["forbidden_inputs"])
    lines.extend(["", "## Transcript Frame", "", "| field | encoding | value |", "|---|---|---|"])
    for row in payload["transcript_frame"]:
        lines.append(f"| `{row['field']}` | {row['encoding']} | {row['value']} |")
    lines.extend(["", "## Acceptance Rules", "", "| rule | requirement |", "|---|---|"])
    for row in payload["acceptance_rules"]:
        lines.append(f"| `{row['rule']}` | {row['requirement']} |")
    lines.extend(["", "## Review Questions", "", "| question | expected answer | deployment impact |", "|---|---|---|"])
    for row in payload["review_questions"]:
        lines.append(
            f"| {row['question']} | {row['expected_answer']} | {row['deployment_impact']} |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
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
