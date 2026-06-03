#!/usr/bin/env python3
"""Generate the conditional leakage/global-composition theorem for the RO-PoK route."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.md"
)
EXTERNAL_REVIEW_JSON = READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.json"
RO_POK_API_JSON = READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
RO_POK_NEGATIVE_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
)
EXTRACTOR_THEOREM_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
)
QROM_REVIEW_JSON = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
PAD_REVIEW_JSON = READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
KEYGEN_SURFACE_JSON = READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json"
RESPONSE_LINK_JSON = (
    READINESS_DIR / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    external_review = load_json(EXTERNAL_REVIEW_JSON)
    api = load_json(RO_POK_API_JSON)
    negative = load_json(RO_POK_NEGATIVE_JSON)
    extractor = load_json(EXTRACTOR_THEOREM_JSON)
    qrom = load_json(QROM_REVIEW_JSON)
    pad = load_json(PAD_REVIEW_JSON)
    keygen = load_json(KEYGEN_SURFACE_JSON)
    response_link = load_json(RESPONSE_LINK_JSON)

    theorem = {
        "name": "ro_pok_leakage_global_composition_or_break",
        "informal_statement": (
            "Conditioned on external acceptance of the state-assisted 32-byte RO-PoK "
            "assumption, any accepting transcript for the fail-closed 284 B/Fq12 "
            "account-carried hidden-source envelope is either explainable by an "
            "extracted account-witness and pad-source opening that satisfies the "
            "canonical checked statement, or yields one named break in transcript "
            "binding, RO-PoK extraction, leakage independence, checked-core "
            "response-link soundness, QROM Fiat-Shamir, pad source, keygen, or "
            "state lifecycle assumptions."
        ),
        "scope": "conditional theorem target for the selected 32-byte RO-PoK research route",
        "non_claim": "does not assert that the current verifier is accepting or that the nonstandard RO-PoK assumption has been accepted",
    }

    leakage_channels = [
        {
            "id": "carrier_handle",
            "bytes": 32,
            "public": True,
            "source": "signature envelope",
            "allowed_meaning": "domain-separated proof handle bound to the public statement and prechallenge opening commitment",
            "must_not_reveal": ["account witness", "pad seed", "signer randomness"],
            "closure_condition": "external RO-PoK assumption includes zero-knowledge or leakage-bounded simulation for the 32-byte handle",
        },
        {
            "id": "public_statement_frame",
            "bytes": "variable_hash_inputs",
            "public": True,
            "source": "verifier-recomputed transcript",
            "allowed_meaning": "public key digest, message digest, slot id, state digest, signature digest, and pad-opening statement digest",
            "must_not_reveal": ["private account opening", "unused slots", "pad source beyond public pad statement"],
            "closure_condition": "statement frame is a deterministic function of public bytes and committed account state",
        },
        {
            "id": "negative_control_rejection_behavior",
            "bytes": 0,
            "public": True,
            "source": "verifier control flow",
            "allowed_meaning": "fail-closed rejection phase labels in tests and diagnostics only",
            "must_not_reveal": ["witness-correlated branch timing", "pad-correlated branch timing"],
            "closure_condition": "production verifier returns a uniform reject result and constant-time audit covers secret-dependent paths",
        },
        {
            "id": "account_state_reads",
            "bytes": "public_account_state",
            "public": True,
            "source": "blockchain account/storage layer",
            "allowed_meaning": "current account commitment, slot occupancy, restore anchor, and consumed transition",
            "must_not_reveal": ["secret opening values", "future pad seeds", "unused private witness material"],
            "closure_condition": "state schema theorem proves only committed public state is read by verifier",
        },
        {
            "id": "checked_core_responses",
            "bytes": 252,
            "public": True,
            "source": "checked-core Fq12 signature body",
            "allowed_meaning": "existing checked-core response material and challenge binding",
            "must_not_reveal": ["hidden-source openings not already modeled by checked-core leakage"],
            "closure_condition": "checked-core leakage theorem composes with hidden-source extraction without duplicate challenge programming",
        },
    ]

    composition_edges = [
        {
            "from": "strict_byte_parse",
            "to": "canonical_statement",
            "claim": "all later proof obligations consume one unique byte-level public statement",
            "failure_break": "transcript_binding_break",
        },
        {
            "from": "canonical_statement",
            "to": "ro_pok_extractor",
            "claim": "accepted 32-byte handle extracts witness/pad openings or breaks the RO-PoK assumption",
            "failure_break": "state_assisted_hidden_relation_ro_pok_break",
        },
        {
            "from": "ro_pok_extractor",
            "to": "checked_core_response_link",
            "claim": "extracted openings are consistent with the checked Fq12 core and bounded-kernel response-link certificate",
            "failure_break": "checked_core_or_response_link_break",
        },
        {
            "from": "checked_core_response_link",
            "to": "qrom_fiat_shamir",
            "claim": "challenge derivation and multi-query loss are charged once for the composed transcript",
            "failure_break": "qrom_fiat_shamir_break",
        },
        {
            "from": "ro_pok_extractor",
            "to": "pad_and_keygen",
            "claim": "pad source and matrix/keygen distribution assumptions are only used after canonical extraction",
            "failure_break": "pad_source_or_keygen_break",
        },
        {
            "from": "account_state_reads",
            "to": "state_lifecycle",
            "claim": "precommitment, slot consumption, and restore anchor prevent replay and rollback",
            "failure_break": "state_lifecycle_break",
        },
    ]

    proof_obligations = [
        {
            "id": "external_ro_pok_acceptance",
            "status": "open",
            "required_result": "external reviewers accept the RO-PoK assumption as separable from whole-scheme unforgeability",
            "evidence": rel(EXTERNAL_REVIEW_JSON),
        },
        {
            "id": "leakage_simulator",
            "status": "open",
            "required_result": "construct simulator for carrier handle, public statement, account-state reads, and rejection behavior",
            "evidence": rel(RO_POK_API_JSON),
        },
        {
            "id": "byte_negative_controls_against_accepting_model",
            "status": negative.get("executable_status", {}).get("current"),
            "required_result": "all semantic substitutions reject under an accepting verifier/oracle model",
            "evidence": rel(RO_POK_NEGATIVE_JSON),
        },
        {
            "id": "extractor_or_break_theorem",
            "status": extractor.get("status"),
            "required_result": "accepted handle yields witness/pad openings or a named break",
            "evidence": rel(EXTRACTOR_THEOREM_JSON),
        },
        {
            "id": "checked_core_qrom_pad_keygen_composition",
            "status": "open",
            "required_result": "compose response-link, QROM, pad-source, and keygen assumptions with no double counting or hidden inputs",
            "evidence": [
                rel(RESPONSE_LINK_JSON),
                rel(QROM_REVIEW_JSON),
                rel(PAD_REVIEW_JSON),
                rel(KEYGEN_SURFACE_JSON),
            ],
        },
        {
            "id": "constant_time_uniform_reject",
            "status": "open",
            "required_result": "production verifier exposes no secret-dependent timing or diagnostic leakage",
            "evidence": "production constant-time audit artifact",
        },
    ]

    named_breaks = [
        "transcript_binding_break",
        "state_assisted_hidden_relation_ro_pok_break",
        "leakage_simulation_break",
        "checked_core_or_response_link_break",
        "qrom_fiat_shamir_break",
        "pad_source_or_keygen_break",
        "state_lifecycle_break",
        "constant_time_uniform_reject_break",
    ]

    return {
        "schema": "Descender.hidden_source_ro_pok_leakage_global_composition_theorem.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "ro_pok_leakage_global_composition_theorem_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "external_review_packet": rel(EXTERNAL_REVIEW_JSON),
            "external_review_packet_schema": external_review.get("schema"),
            "ro_pok_api": rel(RO_POK_API_JSON),
            "ro_pok_api_schema": api.get("schema"),
            "ro_pok_negative_controls": rel(RO_POK_NEGATIVE_JSON),
            "ro_pok_negative_controls_schema": negative.get("schema"),
            "extractor_theorem": rel(EXTRACTOR_THEOREM_JSON),
            "extractor_theorem_schema": extractor.get("schema"),
            "qrom_review_schema": qrom.get("schema"),
            "pad_review_schema": pad.get("schema"),
            "keygen_surface_schema": keygen.get("schema"),
            "response_link_schema": response_link.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `284 B/Fq12` account-carried hidden-source envelope",
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
        },
        "theorem": theorem,
        "leakage_channels": leakage_channels,
        "composition_edges": composition_edges,
        "proof_obligations": proof_obligations,
        "named_breaks": named_breaks,
        "decision": {
            "can_accept_now": False,
            "selected_now": "32_byte_ro_pok_fail_closed",
            "fallback_selected_now": False,
            "reason": "the leakage and composition theorem is stated conditionally, but external RO-PoK review, leakage simulator, accepting-model controls, and constant-time uniform reject remain open",
            "next_artifact": "hidden_source_ro_pok_leakage_simulator_or_fallback_projection_certificate",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Hidden-Source RO-PoK Leakage And Global Composition Theorem",
        "",
        "This artifact states the conditional theorem for the selected RO-PoK route.",
        "It remains fail-closed: the theorem is only usable if external review accepts",
        "the nonstandard 32-byte proof-handle assumption and the leakage/composition",
        "obligations below are discharged.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {payload['target']['selected_branch']}",
        f"- envelope bytes: `{payload['target']['envelope_bytes']}`",
        f"- carrier bytes: `{payload['target']['carrier_bytes']}`",
        "",
        "## Theorem",
        "",
        f"- name: `{payload['theorem']['name']}`",
        f"- scope: {payload['theorem']['scope']}",
        f"- non-claim: {payload['theorem']['non_claim']}",
        "",
        payload["theorem"]["informal_statement"],
        "",
        "## Leakage Channels",
        "",
        "| id | bytes | source | allowed meaning | closure condition |",
        "|---|---:|---|---|---|",
    ]
    for row in payload["leakage_channels"]:
        lines.append(
            f"| `{row['id']}` | `{row['bytes']}` | {row['source']} | "
            f"{row['allowed_meaning']} | {row['closure_condition']} |"
        )
    lines.extend(
        [
            "",
            "## Composition Edges",
            "",
            "| from | to | claim | failure break |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["composition_edges"]:
        lines.append(
            f"| `{row['from']}` | `{row['to']}` | {row['claim']} | `{row['failure_break']}` |"
        )
    lines.extend(
        [
            "",
            "## Proof Obligations",
            "",
            "| id | status | required result | evidence |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["proof_obligations"]:
        evidence = row["evidence"]
        if isinstance(evidence, list):
            evidence = ", ".join(f"`{item}`" for item in evidence)
        else:
            evidence = f"`{evidence}`"
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | {row['required_result']} | {evidence} |"
        )
    lines.extend(["", "## Named Breaks", ""])
    lines.extend(f"- `{item}`" for item in payload["named_breaks"])
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- selected now: `{payload['decision']['selected_now']}`",
            f"- fallback selected now: `{payload['decision']['fallback_selected_now']}`",
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
