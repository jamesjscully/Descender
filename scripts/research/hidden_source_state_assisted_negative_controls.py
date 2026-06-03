#!/usr/bin/env python3
"""Generate negative controls for the state-assisted hidden-source carrier."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.md"
)
EXTRACTOR_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.json"
)
STATE_RELATION_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    extractor = load_json(EXTRACTOR_JSON)
    relation = load_json(STATE_RELATION_JSON)
    controls = [
        {
            "id": "digest_only_acceptance",
            "class": "carrier_semantics",
            "attack": "treat the 32-byte carrier as only H(statement, transcript) with no hidden-source opening commitment",
            "required_rejection": "reject before proof acceptance; digest-only binding has no knowledge semantics",
            "current_evidence": "specified in relation; executable carrier verifier not implemented",
            "status": "needs_executable_test",
        },
        {
            "id": "postchallenge_opening_commitment",
            "class": "state_creation",
            "attack": "create or switch hidden_source_opening_commitment after the Fiat-Shamir challenge is known",
            "required_rejection": "reject because opening commitment is not prechallenge account state",
            "current_evidence": "specified in relation; account-state schema hook not implemented",
            "status": "needs_executable_test",
        },
        {
            "id": "wrong_opening_commitment",
            "class": "carrier_binding",
            "attack": "bind carrier to an opening commitment different from the slot's committed hidden-source opening",
            "required_rejection": "reject by carrier-as-handle predicate",
            "current_evidence": "specified in relation; carrier profile not implemented",
            "status": "needs_executable_test",
        },
        {
            "id": "message_replay",
            "class": "transcript_binding",
            "attack": "reuse carrier/opening commitment under a different message digest",
            "required_rejection": "reject because carrier frame binds message digest and proof transcript",
            "current_evidence": "specified in extractor obligations; carrier transcript binding proof open",
            "status": "needs_executable_test",
        },
        {
            "id": "public_key_replay",
            "class": "transcript_binding",
            "attack": "reuse carrier/opening commitment under a different public key digest",
            "required_rejection": "reject because carrier frame binds public-key digest and account commitments",
            "current_evidence": "covered by accepting-relation transcript requirements; carrier profile not implemented",
            "status": "needs_executable_test",
        },
        {
            "id": "slot_replay",
            "class": "state_lifecycle",
            "attack": "reuse the same carrier/opening commitment for a consumed slot",
            "required_rejection": "reject by spent-slot ledger and single-use carrier predicate",
            "current_evidence": "reference lifecycle tests already reject consumed/spent slots; carrier-specific nullifier test still needed",
            "status": "partially_covered",
        },
        {
            "id": "restore_anchor_rollback",
            "class": "state_lifecycle",
            "attack": "restore an old account snapshot with an unspent opening commitment after a later anchor",
            "required_rejection": "reject by restore-anchor or monotonic state-root rule",
            "current_evidence": "reference lifecycle has restore-anchor rollback rejection; deployment anchor placement remains open",
            "status": "partially_covered",
        },
        {
            "id": "witness_pad_mismatch",
            "class": "hidden_source_relation",
            "attack": "opening commitment binds a witness opening or pad seed different from the checked accepting relation",
            "required_rejection": "reject by opening-commitment binding or extract a named binding break",
            "current_evidence": "extractor obligation specifies named break; proof and executable carrier test open",
            "status": "needs_executable_test",
        },
        {
            "id": "cached_statement_digest",
            "class": "transcript_binding",
            "attack": "verify carrier against a cached hidden-source statement digest instead of recomputing statement bytes",
            "required_rejection": "reject unless recomputed statement, transcript, carrier, and account state all agree",
            "current_evidence": "accepting relation requires recomputation; carrier profile not implemented",
            "status": "needs_executable_test",
        },
    ]
    coverage_summary: dict[str, int] = {}
    for row in controls:
        coverage_summary[row["status"]] = coverage_summary.get(row["status"], 0) + 1
    implementation_plan = [
        {
            "step": "add_state_schema_test_vectors",
            "purpose": "construct account states with and without hidden_source_opening_commitment rows",
        },
        {
            "step": "add_carrier_frame_mutation_tests",
            "purpose": "mutate message, key, slot, signature digest, and opening commitment while holding carrier bytes fixed",
        },
        {
            "step": "add_lifecycle_carrier_tests",
            "purpose": "extend existing spent-slot and restore-anchor tests to nullify hidden-source opening commitments",
        },
        {
            "step": "add_witness_pad_mismatch_tests",
            "purpose": "prove the verifier rejects commitments whose witness/pad preimages do not match the checked relation",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_negative_controls.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "negative_controls_specified_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "extractor_obligations": rel(EXTRACTOR_JSON),
            "state_assisted_relation": rel(STATE_RELATION_JSON),
            "extractor_schema": extractor.get("schema"),
            "relation_schema": relation.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "negative_controls": controls,
        "coverage_summary": coverage_summary,
        "implementation_plan": implementation_plan,
        "decision": {
            "can_accept_now": False,
            "reason": "negative controls are specified, but most carrier-specific executable tests remain open",
            "next_artifact": "state_assisted_opening_commitment_byte_frame",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source State-Assisted Negative Controls",
        "",
        "This artifact records the adversarial controls that must fail for the",
        "state-assisted 32-byte carrier before acceptance can be considered. It",
        "keeps existing lifecycle coverage separate from carrier-specific gaps.",
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
        f"- extractor obligations: `{payload['source_artifacts']['extractor_obligations']}`",
        f"- coverage summary: `{payload['coverage_summary']}`",
        "",
        "## Negative Controls",
        "",
        "| control | class | required rejection | status |",
        "|---|---|---|---|",
    ]
    for row in payload["negative_controls"]:
        lines.append(
            f"| `{row['id']}` | `{row['class']}` | {row['required_rejection']} | `{row['status']}` |"
        )
    lines.extend(["", "## Implementation Plan", "", "| step | purpose |", "|---|---|"])
    for row in payload["implementation_plan"]:
        lines.append(f"| `{row['step']}` | {row['purpose']} |")
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
