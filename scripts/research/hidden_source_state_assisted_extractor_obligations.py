#!/usr/bin/env python3
"""Generate extractor obligations for the state-assisted hidden-source carrier."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.md"
)
STATE_RELATION_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    relation = load_json(STATE_RELATION_JSON)
    extracted_witness = [
        {
            "name": "account_witness_syndrome_opening",
            "required_property": "opens the account witness commitment and is the witness used by the checked Fq12 equation",
        },
        {
            "name": "pad_seed_opening",
            "required_property": "derives the public pad-opening statement bitmap/count and residues",
        },
        {
            "name": "carrier_handle_secret",
            "required_property": "links the prechallenge hidden-source opening commitment to the 32-byte carrier handle",
        },
        {
            "name": "opening_commitment_preimage",
            "required_property": "binds witness opening, pad seed opening, and carrier handle secret before the challenge",
        },
    ]
    obligation_rows = [
        {
            "id": "extract_hidden_source",
            "kind": "success",
            "trigger": "accepted carrier, valid prechallenge opening commitment, valid slot transition, and no binding/lifecycle/core break",
            "must_return": [item["name"] for item in extracted_witness],
            "named_break": None,
            "fallback_if_open": "carrier cannot be accepted until this extractor is proved",
        },
        {
            "id": "break_opening_commitment",
            "kind": "named_break",
            "trigger": "two accepting witnesses or carrier handles bind to one hidden-source opening commitment",
            "must_return": [
                "opening_commitment",
                "opening_preimage_0",
                "opening_preimage_1",
                "distinguishing_field",
            ],
            "named_break": "hidden_source_opening_commitment_binding_break",
            "fallback_if_open": "use a larger algebraic carrier or strengthen account-state commitment semantics",
        },
        {
            "id": "break_carrier_binding",
            "kind": "named_break",
            "trigger": "one 32-byte carrier verifies under two distinct statement/transcript contexts",
            "must_return": [
                "carrier_bytes",
                "context_0",
                "context_1",
                "shared_opening_commitment",
            ],
            "named_break": "carrier_transcript_binding_break",
            "fallback_if_open": "expand carrier or include more algebraic transcript material",
        },
        {
            "id": "break_state_lifecycle",
            "kind": "named_break",
            "trigger": "accepted spend uses a consumed, rolled-back, stale, or unanchored slot",
            "must_return": [
                "public_key_identity",
                "slot_id",
                "spent_or_anchor_record",
                "accepted_transition",
            ],
            "named_break": "state_lifecycle_or_restore_anchor_break",
            "fallback_if_open": "deployment remains blocked regardless of carrier bytes",
        },
        {
            "id": "break_underlying_checked_core",
            "kind": "named_break",
            "trigger": "carrier relation accepts while checked Fq12 core or its assumptions are violated",
            "must_return": [
                "core_signature_context",
                "violated_component",
                "counterexample_or_reduction_witness",
            ],
            "named_break": "checked_core_or_assumption_break",
            "fallback_if_open": "do not promote state-assisted acceptance; revisit checked-core theorem assumptions",
        },
    ]
    named_assumptions = [
        {
            "id": "hidden_source_opening_commitment_binding",
            "scope": "prechallenge account-state hidden-source opening commitment",
            "status": "open",
            "local_evidence_needed": "byte frame, collision/binding argument, and negative controls for two distinct openings",
        },
        {
            "id": "carrier_transcript_binding",
            "scope": "32-byte carrier handle over opening commitment and canonical proof transcript",
            "status": "open",
            "local_evidence_needed": "domain-separated frame, replay/cross-context negative controls, and reduction to SHA-256 binding/RO assumption",
        },
        {
            "id": "state_lifecycle_integrity",
            "scope": "spent-slot ledger, restore anchor, reservation, and rollback policy",
            "status": "open",
            "local_evidence_needed": "deployment-specific state-root/anchor placement plus executable replay/rollback tests",
        },
        {
            "id": "checked_core_soundness",
            "scope": "reviewed Fq12 core, response-link certificate, pad PRF, QROM, and matrix/keygen assumptions",
            "status": "open",
            "local_evidence_needed": "existing review packet evidence plus final composition theorem",
        },
    ]
    proof_order = [
        "prove carrier transcript binding from byte frame and hash/RO assumption",
        "prove opening commitment binding or record required account-state strengthening",
        "prove extraction from accepted state-assisted carrier under no-break hypotheses",
        "compose with checked Fq12 hidden-source accepting relation",
        "compose with state lifecycle integrity and replay/rollback rules",
        "if extraction fails at any step, activate the small algebraic carrier fallback",
    ]
    fallback_triggers = [
        {
            "trigger": "opening commitment cannot bind witness opening and pad seed without revealing too much",
            "fallback": "small_algebraic_opening_carrier",
        },
        {
            "trigger": "32-byte handle cannot be shown to authenticate the hidden-source opening with extraction",
            "fallback": "small_algebraic_opening_carrier",
        },
        {
            "trigger": "state lifecycle assumptions become chain-specific and nonportable",
            "fallback": "treat as separate chain protocol or increase signature bytes",
        },
        {
            "trigger": "negative controls require accepting verifier access to hidden account data",
            "fallback": "larger public algebraic carrier",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_extractor_obligations.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "extractor_obligations_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "state_assisted_relation": rel(STATE_RELATION_JSON),
            "state_assisted_relation_schema": relation.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "extracted_witness": extracted_witness,
        "obligation_rows": obligation_rows,
        "named_assumptions": named_assumptions,
        "proof_order": proof_order,
        "fallback_triggers": fallback_triggers,
        "decision": {
            "can_accept_now": False,
            "reason": "extractor and named-break obligations are specified but not discharged",
            "next_artifact": "state_assisted_carrier_negative_controls",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source State-Assisted Extractor Obligations",
        "",
        "This artifact converts the state-assisted carrier theorem shape into",
        "concrete extractor obligations. It remains fail-closed: it defines what",
        "must be returned or broken, but does not prove those branches.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- n: `{target['n']}`",
        f"- q: `{target['q']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- source relation: `{payload['source_artifacts']['state_assisted_relation']}`",
        "",
        "## Extracted Witness",
        "",
        "| witness item | required property |",
        "|---|---|",
    ]
    for row in payload["extracted_witness"]:
        lines.append(f"| `{row['name']}` | {row['required_property']} |")
    lines.extend(
        [
            "",
            "## Obligation Rows",
            "",
            "| id | kind | trigger | named break | fallback if open |",
            "|---|---|---|---|---|",
        ]
    )
    for row in payload["obligation_rows"]:
        lines.append(
            f"| `{row['id']}` | `{row['kind']}` | {row['trigger']} | "
            f"`{row['named_break'] or '-'}` | {row['fallback_if_open']} |"
        )
    lines.extend(
        [
            "",
            "## Named Assumptions",
            "",
            "| assumption | status | scope | local evidence needed |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["named_assumptions"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | {row['scope']} | "
            f"{row['local_evidence_needed']} |"
        )
    lines.extend(["", "## Proof Order", ""])
    lines.extend(f"{index}. {step}" for index, step in enumerate(payload["proof_order"], 1))
    lines.extend(["", "## Fallback Triggers", "", "| trigger | fallback |", "|---|---|"])
    for row in payload["fallback_triggers"]:
        lines.append(f"| {row['trigger']} | `{row['fallback']}` |")
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
